extern crate gl;
extern crate glutin;
extern crate nanovg;

use glutin::GlContext;
use nanovg::{Color, ColoringStyle, Font, Alignment, TextOptions, Scissor, FillStyle, Frame, Transform, PathOptions, StrokeStyle};
use std::time::Instant;
use std::f32::consts::PI;

const INIT_WINDOW_SIZE: (u32, u32) = (300, 300);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("NanoVG Text")
        .with_dimensions(INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1);
    let context = glutin::ContextBuilder::new()
        .with_vsync(false)
        .with_multisampling(4)
        .with_srgb(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let context = nanovg::ContextBuilder::new()
        .stencil_strokes()
        .build()
        .expect("Initialization of NanoVG failed!");

    let font = Font::from_file(&context, "Roboto-Regular", "resources/Roboto-Regular.ttf")
            .expect("Failed to load font 'Roboto-Regular.ttf'");

    let emoji = Font::from_file(&context, "NotoEmoji", "resources/NotoEmoji-Regular.ttf")
            .expect("Failed to load font 'NotoEmoji-Regular.ttf'");

    font.add_fallback(emoji);

    let mut running = true;
    let mut mouse = (0.0f32, 0.0f32);
    let start_time = Instant::now();

    loop {
        let elapsed = get_elapsed(&start_time);
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                glutin::WindowEvent::CursorMoved { position, .. } => mouse = (position.0 as f32, position.1 as f32),
                _ => {}
            },
            _ => {}
        });

        if !running {
            break;
        }

        let (width, height) = gl_window.get_inner_size().unwrap();
        let (width, height) = (width as i32, height as i32);

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.3, 0.3, 0.32, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        context.frame((width, height), gl_window.hidpi_factor(), |frame| {
            let (width, height) = (width as f32, height as f32);

            let margin = 50.0;
            let clip = (margin, margin, width - margin * 2.0, height - margin * 2.0);
            let transform = Transform::new().with_translation(mouse.0, mouse.1).rotate(elapsed * 4.0);
            render_text(&frame, font, "text", clip, transform);
            let transform = Transform::new().with_translation(150.0, 100.0).rotate(-PI / 6.0);
            draw_paragraph(&frame, font, -150.0 / 2.0, -50.0, 150.0, 100.0, mouse, transform);
        });

        gl_window.swap_buffers().unwrap();
    }
}

fn get_elapsed(instant: &Instant) -> f32 {
    let elapsed = instant.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    elapsed as f32
}

fn render_text(frame: &Frame, font: Font, text: &str, clip: (f32, f32, f32, f32), transform: Transform) {
    let (cx, cy, cw, ch) = clip;
    let scissor = Scissor::Rect {
        x: cx,
        y: cy,
        width: cw,
        height: ch,
    };

    // draw clipping area
    frame.path(
        |path| {
            path.rect((cx, cy), (cw, ch));
            path.stroke(StrokeStyle {
                coloring_style: ColoringStyle::Color(Color::from_rgb(0, 0, 0)),
                ..Default::default()
            });
        }, 
        PathOptions::default()
    );

    // draw small rectangle that is translated with transform
    // and gets clipped into clipping area
    frame.path(
        |path| {
            path.rect((-50.0, -50.0), (50.0, 50.0));
            path.fill(FillStyle {
                coloring_style: ColoringStyle::Color(Color::from_rgb(50, 50, 50)),
                ..Default::default()
            });
        },
        PathOptions {
            scissor: Some(scissor),
            transform: Some(transform),
            ..Default::default()
        }
    );

    frame.text(font, (0.0, 0.0), text, 
        TextOptions {
            size: 28.0,
            color: Color::from_rgb(255, 255, 255),
            align: Alignment::new().bottom().right(),
            scissor: Some(scissor),
            transform: Some(transform),
            ..Default::default()
        }
    );
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn draw_paragraph(frame: &Frame, font: Font, x: f32, y: f32, width: f32, _height: f32, mouse: (f32, f32), transform: Transform) {
    let text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.🎉";
    let text_options = TextOptions {
        color: Color::from_rgba(255, 255, 255, 255),
        size: 18.0,
        align: Alignment::new().left().top(),
        transform: Some(transform),
        ..Default::default()
    };
    let metrics = frame.text_metrics(font, text_options);

    let mut y = y;
    let (mx, my) = if let Some(inv) = transform.try_inverse() {
        inv.transform_point(mouse)
    } else {
        mouse
    };

    for row in frame.text_break_lines(text, width) {
        let hit = mx > x && mx < (x + width) && my >= y && my < (y + metrics.line_height);

        // draw line background
        frame.path(
            |path| {
                path.rect((x, y), (row.width, metrics.line_height));
                path.fill(FillStyle {
                    coloring_style: ColoringStyle::Color(Color::from_rgba(255, 255, 255, if hit { 64 } else { 16 })),
                    ..Default::default()
                });
            },
            PathOptions {
                transform: Some(transform),
                ..Default::default()
            }
        );

        // draw line text
        frame.text(
            font,
            (x, y),
            row.text,
            text_options,
        );

        if hit {
            let mut caretx = if mx < x + row.width / 2.0 { x } else { x + row.width };
            let mut px = x;

            // calculate mouse caret position
            for glyph in frame.text_glyph_positions((x, y), row.text) {
                let x0 = glyph.x;
                let x1 = if let Some(next) = *glyph.next { next.x } else { x + row.width };
                let gx = x0 * 0.3 + x1 * 0.7;

                if mx >= px && mx < gx {
                    caretx = x0;
                    break;
                }

                px = gx;
            }

            // draw mouse caret
            frame.path(
                |path| {
                    path.rect((caretx, y), (1.0, metrics.line_height));
                    path.fill(FillStyle {
                        coloring_style: ColoringStyle::Color(Color::from_rgba(255, 192, 0, 255)),
                        ..Default::default()
                    });
                },
                PathOptions {
                    transform: Some(transform),
                    ..Default::default()
                }
            );
        }

        y += metrics.line_height;
    }

    draw_tooltip(frame, (x, y + 20.0), mouse, font, transform.clone());
}

fn draw_tooltip(frame: &Frame, (x, y): (f32, f32), mouse: (f32, f32), font: Font, transform: Transform) {
    let tooltip_text = "Hover your mouse over the text to see calculated caret position.";
    let tooltip_opts = TextOptions {
        color: Color::from_rgba(0, 0, 0, 220),
        size: 13.0,
        align: Alignment::new().left().top(),
        line_height: 1.2,
        line_max_width: 150.0,
        transform: Some(transform),
        ..Default::default()
    };
    // draw tooltip
    let bounds = frame.text_box_bounds(font, (x, y), tooltip_text, tooltip_opts);
    let (mx, my) = if let Some(inv) = transform.try_inverse() {
        inv.transform_point(mouse)
    }
    else {
        mouse
    };

    let gx = f32::abs((mx - (bounds.min_x + bounds.max_x) * 0.5) / (bounds.min_x - bounds.max_x));
    let gy = f32::abs((my - (bounds.min_y + bounds.max_y) * 0.5) / (bounds.min_y - bounds.max_y));
    let alpha = f32::max(gx, gy) - 0.5;
    let alpha = clamp(alpha, 0.0, 1.0);

    frame.path(
        |path| {
            path.rounded_rect(
                (bounds.min_x - 2.0, bounds.min_y - 2.0),
                (bounds.max_x - bounds.min_x + 4.0, bounds.max_y - bounds.min_y + 4.0),
                3.0
            );
            let px = (bounds.max_x + bounds.min_x) / 2.0;
            let py = bounds.min_y;
            path.move_to((px, py - 10.0));
            path.line_to((px + 7.0, py + 1.0));
            path.line_to((px - 7.0, py + 1.0));
            path.fill(FillStyle {
                coloring_style: ColoringStyle::Color(Color::from_rgba(220, 220, 220, 255)),
                ..Default::default()
            });
        },
        PathOptions {
            alpha,
            transform: Some(transform),
            ..Default::default()
        }
    );

    frame.text_box(font, (x, y), tooltip_text, tooltip_opts);
}
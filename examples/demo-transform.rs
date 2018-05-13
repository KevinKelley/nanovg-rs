extern crate gl;
extern crate glutin;
extern crate nanovg;

use glutin::GlContext;
use nanovg::{Color, Font, Alignment, TextOptions, Gradient,
             Frame, Transform, PathOptions, StrokeOptions, Scissor, Clip};
use std::time::Instant;

const INIT_WINDOW_SIZE: (u32, u32) = (300, 300);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("NanoVG Transform")
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

        context.frame((width, height), gl_window.hidpi_factor(), |mut frame| {
            let (width, height) = (width as f32, height as f32);

            // position red rect that drawn rotated at offset with transform
            frame.transformed(Transform::new().translate(width / 2.0, height + 100.0),
                |frame| {
                    rotating_red_rect(&frame, 50.0, 50.0, elapsed);
                }
            );

            // position button with x and y
            draw_button(&frame, font, "Button", 20.0, 150.0, 100.0, 40.0, Color::from_rgb(64, 64, 64));

            // position multiple buttons with transform
            frame.transformed(Transform::new().translate(width - 125.0, 20.0),
                |frame| {
                    button_container(&frame, font, 125.0);
                }
            );

            let translate = Transform::new().translate(60.0, 60.0);
            let rotate = Transform::new().rotate(elapsed);

            // here frame gets translated with transform,
            // green rectangle gets drawn, and then it gets rotated in PathOptions
            frame.transformed(translate, |frame| {
                frame.path(
                    |path| {
                        path.rect((0.0, 0.0), (50.0, 50.0));
                        path.fill(Color::from_rgb(0, 255, 0), Default::default());
                    },
                    PathOptions {
                        transform: Some(rotate),
                        ..Default::default()
                    }
                );
            });

            let transform = rotate * translate; // we can multiply or premultiply Transforms

            // this has same effect as the one above
            // only draws white stroke on the green rectangle drawn above
            frame.path(
                |path| {
                    path.rect((0.0, 0.0), (50.0, 50.0));
                    path.stroke(
                        Color::from_rgb(255, 255, 255),
                        StrokeOptions {
                            width: 5.0,
                            ..Default::default()
                        }
                    );
                },
                PathOptions {
                    transform: Some(transform),
                    ..Default::default()
                }
            );


            // this example renders rotated rectangular
            // area where rectangle gets drawn and rotated
            // at mouse position, when rectangle gets
            // out of this area it gets clipped
            let margin = 50.0;
            let clip = (margin, margin, width - margin * 2.0, height - margin * 2.0);
            let mouse_transform = Transform::new().with_translation(mouse.0, mouse.1).rotate(elapsed * 4.0);
            frame.transformed(Transform::new().rotate(10.0f32.to_radians()),
                |frame| {
                    render_area(&frame, font, clip, mouse_transform.absolute()); // the absolute is here because we do not want
                                                                                 // our mouse to be translated in frame's local coordinate space,
                                                                                 // we want to use it as it is
                                                                                 // if you we to remove it, the rectangle inside area that is
                                                                                 // dragged by mouse would get invalid coordinates
                }
            );
        });

        gl_window.swap_buffers().unwrap();
    }
}

fn get_elapsed(instant: &Instant) -> f32 {
    let elapsed = instant.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    elapsed as f32
}

fn rotating_red_rect(frame: &Frame, w: f32, h: f32, t: f32) {
    frame.path(
        |path| {
            path.rect((-w / 2.0, -h / 2.0), (w, h));
            path.fill(Color::from_rgb(255, 0, 0), Default::default());
        },
        PathOptions {
            transform: Some(Transform::new().rotate(t * 2.0)),
            ..Default::default()
        }
    );
}

fn is_black(color: Color) -> bool {
    color.red() == 0.0 && color.green() == 0.0 && color.blue() == 0.0 && color.alpha() == 0.0
}

fn button_container(frame: &Frame, font: Font, w: f32) {
    draw_button(frame, font, "Button 1", 0.0, 0.0, w, 35.0, Color::from_rgb(123, 0, 0));
    draw_button(frame, font, "Button 2", 0.0, 50.0, w, 35.0, Color::from_rgb(0, 123, 0));
    draw_button(frame, font, "Button 3", 0.0, 100.0, w, 35.0, Color::from_rgb(0, 0, 123));
}

fn draw_button(frame: &Frame, font: Font, text: &str, x: f32, y: f32, w: f32, h: f32, color: Color) {
    let corner_radius = 4.0;
    let color_is_black = is_black(color);

    // button background
    frame.path(
        |path| {
            path.rounded_rect((x + 1.0, y + 1.0), (w - 2.0, h - 2.0), corner_radius - 0.5);
            if !color_is_black {
                path.fill(color, Default::default());
            }

            path.fill(
                Gradient::Linear {
                    start: (x, y),
                    end: (w, h),
                    start_color: Color::from_rgba(255, 255, 255, if color_is_black { 16 } else { 32 }),
                    end_color: Color::from_rgba(0, 0, 0, if color_is_black { 16 } else { 32 }),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // button border
    frame.path(
        |path| {
            path.rounded_rect((x + 0.5, y + 0.5), (w - 1.0, h - 1.0), corner_radius - 0.5);
            path.stroke(Color::from_rgba(0, 0, 0, 48), Default::default());
        },
        Default::default(),
    );

    let mut options = TextOptions {
        size: 20.0,
        align: Alignment::new().center().middle(),
        ..Default::default()
    };

    options.color = Color::from_rgba(0, 0, 0, 160);

    frame.text(
        font,
        (x + w / 2.0, y + h / 2.0 - 1.0),
        text,
        options,
    );

    options.color = Color::from_rgba(255, 255, 255, 160);

    frame.text(
        font,
        (x + w / 2.0 + 0.25, y + h / 2.0),
        text,
        options,
    );
}

fn render_area(frame: &Frame, font: Font, clip: (f32, f32, f32, f32), transform: Transform) {
    let (cx, cy, cw, ch) = clip;
    let scissor = Scissor {
        x: cx,
        y: cy,
        width: cw,
        height: ch,
        transform: None,
    };

    // draw clipping area
    frame.path(
        |path| {
            path.rect((cx, cy), (cw, ch));
            path.fill(Color::from_rgba(255, 255, 255, 20), Default::default());
            path.stroke(Color::from_rgb(0, 0, 0), Default::default());
        },
        PathOptions::default()
    );

    // draw small rectangle that is translated with transform
    // and gets clipped into clipping area
    frame.path(
        |path| {
            path.rect((-50.0, -50.0), (50.0, 50.0));
            path.fill(Color::from_rgb(50, 50, 50), Default::default());
        },
        PathOptions {
            clip: Clip::Scissor(scissor),
            transform: Some(transform),
            ..Default::default()
        }
    );

    frame.text(font, (0.0, 0.0), "text",
        TextOptions {
            size: 28.0,
            color: Color::from_rgb(255, 255, 255),
            align: Alignment::new().bottom().right(),
            clip: Clip::Scissor(scissor),
            transform: Some(transform),
            ..Default::default()
        }
    );
}
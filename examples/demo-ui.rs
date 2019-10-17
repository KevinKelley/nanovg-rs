extern crate gl;
extern crate glutin;
extern crate nanovg;
extern crate rand;

use std::time::Instant;
use std::f32::consts;
use rand::Rng;
use glutin::GlContext;
use nanovg::{Direction, Alignment, Color, Font, Frame, Gradient, ImagePattern,
             LineCap, LineJoin, PathOptions, Scissor, Solidity, StrokeOptions,
             TextOptions, Transform, Winding, Image, Context, Clip, Intersect};

const INIT_WINDOW_SIZE: (u32, u32) = (1000, 600);

const ICON_SEARCH: &str = "\u{1F50D}";
const ICON_CIRCLED_CROSS: &str = "\u{2716}";
const ICON_CHEVRON_RIGHT: &str = "\u{E75E}";
const ICON_CHECK: &str = "\u{2713}";
const ICON_LOGIN: &str = "\u{E740}";
const ICON_TRASH: &str = "\u{E729}";

const GRAPH_HISTORY_COUNT: usize = 100;

struct DemoData<'a> {
    fonts: DemoFonts<'a>,
    images: Vec<Image<'a>>,
}

struct DemoFonts<'a> {
    icons: Font<'a>,
    sans: Font<'a>,
    sans_bold: Font<'a>,
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("NanoVG UI")
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

    let start_time = Instant::now();
    let mut running = true;

    let mut mx = 0.0f32;
    let mut my = 0.0f32;

    let demo_data = load_demo_data(&context);

    let mut fps_graph = PerformanceGraph::new(GraphRenderStyle::Fps, "Frame Time");
    let mut cpu_graph = PerformanceGraph::new(GraphRenderStyle::Ms, "CPU Time");
    let mut rng_graph = PerformanceGraph::new(GraphRenderStyle::Percent, "Random");

    let mut percent = 0.0f32;
    let mut rng = rand::thread_rng();
    let mut prev_time = 0.0;

    while running {
        let elapsed = get_elapsed(&start_time);
        let delta_time = elapsed - prev_time;
        prev_time = elapsed;

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    mx = position.0 as f32;
                    my = position.1 as f32;
                }
                _ => {}
            },
            _ => {}
        });

        let (width, height) = gl_window.get_inner_size().unwrap();
        let (width, height) = (width as i32, height as i32);

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.3, 0.3, 0.32, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        let (width, height) = (width as f32, height as f32);
        context.frame((width, height), gl_window.hidpi_factor(), |frame| {
            render_demo(
                &frame,
                mx,
                my,
                width as f32,
                height as f32,
                elapsed,
                &demo_data,
            );

            fps_graph.draw(&frame, demo_data.fonts.sans, 5.0, 5.0);
            cpu_graph.draw(&frame, demo_data.fonts.sans, 5.0 + 200.0 + 5.0, 5.0);
            rng_graph.draw(&frame, demo_data.fonts.sans, 5.0 + 200.0 + 5.0 + 200.0 + 5.0, 5.0);
        });

        fps_graph.update(delta_time);

        percent = if rng.gen() { percent + 1.0 } else { percent - 1.0 };
        percent = clamp(percent, 0.0, 100.0);
        rng_graph.update(percent);

        let cpu_time = get_elapsed(&start_time) - elapsed;
        cpu_graph.update(cpu_time);

        gl_window.swap_buffers().unwrap();
    }

    println!("Average Frame Time: {:.2} ms", fps_graph.average() * 1000.0);
    println!("          CPU Time: {:.2} ms", cpu_graph.average() * 1000.0);
    println!("       RNG Percent: {:.2}%  ", rng_graph.average());
}

fn load_demo_data(context: &Context) -> DemoData {
    let demo_fonts = DemoFonts {
        icons: Font::from_file(context, "Entypo", "resources/entypo.ttf")
            .expect("Failed to load font 'entypo.ttf'"),

        sans: Font::from_file(context, "Roboto-Regular", "resources/Roboto-Regular.ttf")
            .expect("Failed to load font 'Roboto-Regular.ttf'"),

        sans_bold: Font::from_file(context, "Roboto-Bold", "resources/Roboto-Bold.ttf")
            .expect("Failed to load font 'Roboto-Bold.ttf'"),
    };

    let emoji = Font::from_file(context, "NotoEmoji", "resources/NotoEmoji-Regular.ttf")
            .expect("Failed to load font 'NotoEmoji-Regular.ttf'");

    let mut images = Vec::new();
    for i in 0..12 {
        let file_name = format!("resources/images/image{}.jpg", i + 1);
        let image = Image::new(context)
                        .build_from_file(&file_name)
                        .expect(&format!("Failed to load image {}", &file_name));
        images.push(image);
    }


    demo_fonts.sans.add_fallback(emoji);
    demo_fonts.sans_bold.add_fallback(emoji);

    DemoData {
        fonts: demo_fonts,
        images: images
    }
}

fn get_elapsed(instant: &Instant) -> f32 {
    let elapsed = instant.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    elapsed as f32
}

fn is_black(color: Color) -> bool {
    color.red() == 0.0 && color.green() == 0.0 && color.blue() == 0.0 && color.alpha() == 0.0
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

fn render_demo(frame: &Frame, mx: f32, my: f32, width: f32, height: f32, t: f32, data: &DemoData) {
    draw_eyes(frame, width - 250.0, 50.0, 150.0, 100.0, mx, my, t);
    draw_paragraph(frame, &data.fonts, width - 450.0, 50.0, 150.0, 100.0, mx, my);
    draw_graph(frame, 0.0, height / 2.0, width, height / 2.0, t);
    draw_color_wheel(frame, width - 300.0, height - 300.0, 250.0, 250.0, t);
    draw_lines(frame, 120.0, height - 50.0, 600.0, 50.0, t);
    draw_widths(frame, 10.0, 50.0, 30.0);
    draw_caps(frame, 10.0, 300.0, 30.0);

    draw_scissor(&frame, 50.0, height - 80.0, t);

    let mut x = 50.0;
    let mut y = 50.0;
    // widgets
    draw_window(frame, &data.fonts, "Widgets `n stuff", x, y, 300.0, 400.0);
    x += 10.0;
    y += 45.0;

    draw_search_box(frame, &data.fonts, "Search", x, y, 280.0, 25.0);
    y += 40.0;

    draw_drop_down(frame, &data.fonts, "Effects", x, y, 280.0,28.0);
    let popy = y + 14.0;
    y += 45.0;

    // form
    draw_label(frame, &data.fonts, "Login", x, y, 280.0, 20.0);
    y += 25.0;

    draw_edit_box(frame, &data.fonts, "Email", x, y, 280.0, 28.0);
    y += 35.0;

    draw_edit_box(frame, &data.fonts, "Password", x, y, 280.0, 28.0);
    y += 38.0;

    draw_check_box(frame, &data.fonts, "Remember me", x, y, 140.0, 28.0);
    draw_button(frame, &data.fonts, Some(ICON_LOGIN), "Sign in", x + 138.0, y, 140.0, 28.0, Color::from_rgba(0, 96, 128, 255));
    y += 45.0;

    // slider
    draw_label(frame, &data.fonts, "Diameter", x, y, 280.0, 20.0);
    y += 25.0;

    draw_edit_box_num(frame, &data.fonts, "128.00", "px", x + 180.0, y, 100.0, 28.0, );

    draw_slider(frame, 0.4, x, y, 170.0, 28.0);
    y += 55.0;

    draw_button(frame, &data.fonts, Some(ICON_TRASH), "Delete", x, y, 160.0, 28.0, Color::from_rgba(128, 16, 8, 255));

    draw_button(frame, &data.fonts, None, "Cancel", x + 170.0, y, 110.0, 28.0, Color::from_rgba(0, 0, 0, 0));

    draw_thumbnails(frame, &data.images, 365.0, popy - 30.0, 160.0, 300.0, t);
}

fn draw_eyes(frame: &Frame, x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32, t: f32) {
    let ex = w * 0.23;
    let ey = h * 0.5;
    let lx = x + ex;
    let ly = y + ey;
    let rx = x + w - ex;
    let ry = y + ey;
    let br = (if ex < ey { ex } else { ey }) * 0.5;
    let blink = 1.0 - ((t * 0.5).sin()).powf(200.0) * 0.8;

    // eye shades
    frame.path(
        |path| {
            path.ellipse((lx + 3.0, ly + 16.0), ex, ey);
            path.ellipse((rx + 3.0, ry + 16.0), ex, ey);

            path.fill(
                Gradient::Linear {
                    start: (x, y + h * 0.5),
                    end: (x + w * 0.1, y + h),
                    start_color: Color::from_rgba(0, 0, 0, 32),
                    end_color: Color::from_rgba(0, 0, 0, 16),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // eye whites
    frame.path(
        |path| {
            path.ellipse((lx, ly), ex, ey);
            path.ellipse((rx, ry), ex, ey);

            path.fill(
                Gradient::Linear {
                    start: (x, y + h * 0.25),
                    end: (x + w * 0.1, y + h),
                    start_color: Color::from_rgba(220, 220, 220, 255),
                    end_color: Color::from_rgba(128, 128, 128, 255),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // eye pupils
    frame.path(
        |path| {
            let mut dx = (mx - rx) / (ex * 10.0);
            let mut dy = (my - ry) / (ey * 10.0);
            let d = (dx * dx + dy * dy).sqrt();

            if d > 1.0 {
                dx /= d;
                dy /= d;
            }

            dx *= ex * 0.4;
            dy *= ey * 0.5;

            path.ellipse(
                (lx + dx, ly + dy + ey * 0.25 * (1.0 - blink)),
                br,
                br * blink,
            );
            path.ellipse(
                (rx + dx, ry + dy + ey * 0.25 * (1.0 - blink)),
                br,
                br * blink,
            );
            path.fill(Color::from_rgba(32, 32, 32, 255), Default::default());
        },
        Default::default(),
    );

    // left eye gloss
    frame.path(
        |path| {
            path.ellipse((lx, ly), ex, ey);
            path.fill(
                Gradient::Radial {
                    center: (lx - ex * 0.25, ly - ey * 0.5),
                    inner_radius: ex * 0.1,
                    outer_radius: ex * 0.75,
                    start_color: Color::from_rgba(255, 255, 255, 128),
                    end_color: Color::from_rgba(255, 255, 255, 0),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // right eye gloss
    frame.path(
        |path| {
            path.ellipse((rx, ry), ex, ey);
            path.fill(
                Gradient::Radial {
                    center: (rx - ex * 0.25, ry - ey * 0.5),
                    inner_radius: ex * 0.1,
                    outer_radius: ex * 0.75,
                    start_color: Color::from_rgba(255, 255, 255, 128),
                    end_color: Color::from_rgba(255, 255, 255, 0),
                },
                Default::default()
            );
        },
        Default::default(),
    );
}

fn draw_paragraph(frame: &Frame, fonts: &DemoFonts, x: f32, y: f32, width: f32, _height: f32, mx: f32, my: f32) {
    let text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.🎉";
    let text_options = TextOptions {
        color: Color::from_rgba(255, 255, 255, 255),
        size: 18.0,
        align: Alignment::new().left().top(),
        ..Default::default()
    };
    let metrics = frame.text_metrics(fonts.sans, text_options);

    let mut gutter_line = 0;
    let mut gutter_x = 0.0f32;
    let mut gutter_y = 0.0f32;

    let mut y = y;
    let mut line_number = 0;
    for row in frame.text_break_lines(fonts.sans, text, width, text_options) {
        line_number += 1;
        let hit = mx > x && mx < (x + width) && my >= y && my < (y + metrics.line_height);

        // draw line background
        frame.path(
            |path| {
                path.rect((x, y), (row.width, metrics.line_height));
                path.fill(Color::from_rgba(255, 255, 255, if hit { 64 } else { 16 }), Default::default());
            },
            Default::default()
        );

        // draw line text
        frame.text(
            fonts.sans,
            (x, y),
            row.text,
            text_options,
        );

        if hit {
            let mut caretx = if mx < x + row.width / 2.0 { x } else { x + row.width };
            let mut px = x;

            // calculate mouse caret position
            let mut glyph_positions = frame.text_glyph_positions((x, y), row.text).peekable();
            while let Some(glyph) = glyph_positions.next() {
                let x0 = glyph.x;
                let x1 = if let Some(next) = glyph_positions.peek() { next.x } else { x + row.width };
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
                    path.fill(Color::from_rgba(255, 192, 0, 255), Default::default());
                },
                Default::default()
            );

            gutter_x = x - 10.0;
            gutter_y = y + metrics.line_height / 2.0;
            gutter_line = line_number;
        }

        y += metrics.line_height;
    }


    // draw gutter
    if gutter_line != 0 {
        let gx = gutter_x;
        let gy = gutter_y;
        let gutter_text = format!("{}", gutter_line);
        let gutter_text_options = TextOptions {
            color: Color::from_rgba(32, 32, 32, 255),
            size: 13.0,
            align: Alignment::new().right().middle(),
            ..Default::default()
        };
        let (_, bounds) = frame.text_bounds(fonts.sans, (gx, gy), &gutter_text, gutter_text_options);
        frame.path(
            |path| {
                path.rounded_rect(
                    (bounds.min_x - 4.0, bounds.min_y - 2.0),
                    (bounds.max_x - bounds.min_x + 8.0, bounds.max_y - bounds.min_y + 4.0),
                    (bounds.max_y - bounds.min_y + 4.0) / 2.0 - 1.0
                );

                path.fill(Color::from_rgba(255, 192, 0, 255), Default::default());
            },
            Default::default()
        );
        frame.text(fonts.sans, (gx, gy), &gutter_text, gutter_text_options);
    }

    y += 20.0;

    let tooltip_text = "Hover your mouse over the text to see calculated caret position.";
    let tooltip_opts = TextOptions {
        color: Color::from_rgba(0, 0, 0, 220),
        size: 13.0,
        align: Alignment::new().left().top(),
        line_height: 1.2,
        line_max_width: 150.0,
        ..Default::default()
    };
    // draw tooltip
    let bounds = frame.text_box_bounds(fonts.sans, (x, y), tooltip_text, tooltip_opts);
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
            path.fill(Color::from_rgba(220, 220, 220, 255), Default::default());
        },
        PathOptions {
            alpha,
            ..Default::default()
        }
    );

    frame.text_box(fonts.sans, (x, y), tooltip_text, tooltip_opts);
}

fn draw_graph(frame: &Frame, x: f32, y: f32, w: f32, h: f32, t: f32) {
    let mut samples = [0.0f32; 6];
    let mut sx = [0.0f32; 6];
    let mut sy = [0.0f32; 6];
    let dx = w / 5.0;

    samples[0] = (1.0 + f32::sin(t * 1.2345 + f32::cos(t * 0.33457) * 0.44)) * 0.5;
    samples[1] = (1.0 + f32::sin(t * 0.68363 + f32::cos(t * 1.3) * 1.55)) * 0.5;
    samples[2] = (1.0 + f32::sin(t * 1.1642 + f32::cos(t * 0.33457) * 1.24)) * 0.5;
    samples[3] = (1.0 + f32::sin(t * 0.56345 + f32::cos(t * 1.63) * 0.14)) * 0.5;
    samples[4] = (1.0 + f32::sin(t * 1.6245 + f32::cos(t * 0.254) * 0.3)) * 0.5;
    samples[5] = (1.0 + f32::sin(t * 0.345 + f32::cos(t * 0.03) * 0.6)) * 0.5;

    for i in 0..6 {
        sx[i] = x + i as f32 * dx;
        sy[i] = y + h * samples[i] * 0.8;
    }

    // graph background
    frame.path(
        |path| {
            path.move_to((sx[0], sy[0]));
            for i in 1..6 {
                path.cubic_bezier_to(
                    (sx[i], sy[i]),
                    (sx[i - 1] + dx * 0.5, sy[i - 1]),
                    (sx[i] - dx * 0.5, sy[i]),
                );
            }

            path.line_to((x + w, y + h));
            path.line_to((x, y + h));

            path.fill(
                Gradient::Linear {
                    start: (x, y),
                    end: (x, y + h),
                    start_color: Color::from_rgba(0, 160, 192, 0),
                    end_color: Color::from_rgba(0, 160, 192, 64),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // graph line (darker)
    frame.path(
        |path| {
            path.move_to((sx[0], sy[0] + 2.0));
            for i in 1..6 {
                path.cubic_bezier_to(
                    (sx[i], sy[i] + 2.0),
                    (sx[i - 1] + dx * 0.5, sy[i - 1] + 2.0),
                    (sx[i] - dx * 0.5, sy[i] + 2.0),
                );
            }

            path.stroke(
                Color::from_rgba(0, 0, 0, 32),
                StrokeOptions {
                    width: 3.0,
                    ..Default::default()
                }
            );
        },
        Default::default(),
    );

    // graph line (lighter)
    frame.path(
        |path| {
            path.move_to((sx[0], sy[0]));
            for i in 1..6 {
                path.cubic_bezier_to(
                    (sx[i], sy[i]),
                    (sx[i - 1] + dx * 0.5, sy[i - 1]),
                    (sx[i] - dx * 0.5, sy[i]),
                );
            }

            path.stroke(
                Color::from_rgba(0, 160, 192, 255),
                StrokeOptions {
                    width: 3.0,
                    ..Default::default()
                }
            );
        },
        Default::default(),
    );

    // graph sample points (background shades)
    for i in 0..6 {
        frame.path(
            |path| {
                path.rect((sx[i] - 10.0, sy[i] - 10.0 + 2.0), (20.0, 20.0));
                path.fill(
                    Gradient::Radial {
                        center: (sx[i], sy[i] + 2.0),
                        inner_radius: 3.0,
                        outer_radius: 8.0,
                        start_color: Color::from_rgba(0, 0, 0, 32),
                        end_color: Color::from_rgba(0, 0, 0, 0),
                    },
                    Default::default()
                );
            },
            Default::default(),
        );
    }

    // graph sample points (main dots)
    frame.path(
        |path| {
            for i in 0..6 {
                path.circle((sx[i], sy[i]), 4.0);
            }

            path.fill(Color::from_rgba(0, 160, 192, 255), Default::default());
        },
        Default::default(),
    );

    // graph sample points (small white dots)
    frame.path(
        |path| {
            for i in 0..6 {
                path.circle((sx[i], sy[i]), 2.0);
            }

            path.fill(Color::from_rgba(220, 220, 220, 255), Default::default());
        },
        Default::default(),
    );
}

fn draw_color_wheel(frame: &Frame, x: f32, y: f32, w: f32, h: f32, t: f32) {
    let cx = x + w * 0.5;
    let cy = y + h * 0.5;
    let r1 = if w < h { w } else { h } * 0.5 - 5.0;
    let r0 = r1 - 20.0;
    let aeps = 0.5 / r1; // half a pixel arc length in radians (2pi cancels out).
    let hue = f32::sin(t * 0.12);

    for i in 0..6 {
        let a0 = i as f32 / 6.0 * consts::PI * 2.0 - aeps;
        let a1 = (i as f32 + 1.0) / 6.0 * consts::PI * 2.0 + aeps;

        // draw color segment gradient
        frame.path(
            |path| {
                path.arc((cx, cy), r0, a0, a1, Winding::Direction(Direction::Clockwise));
                path.arc((cx, cy), r1, a1, a0, Winding::Direction(Direction::CounterClockwise));
                let ax = cx + f32::cos(a0) * (r0 + r1) * 0.5;
                let ay = cy + f32::sin(a0) * (r0 + r1) * 0.5;
                let bx = cx + f32::cos(a1) * (r0 + r1) * 0.5;
                let by = cy + f32::sin(a1) * (r0 + r1) * 0.5;
                path.fill(
                    Gradient::Linear {
                        start: (ax, ay),
                        end: (bx, by),
                        start_color: Color::from_hsla(a0 / (consts::PI * 2.0), 1.0, 0.55, 255),
                        end_color: Color::from_hsla(a1 / (consts::PI * 2.0), 1.0, 0.55, 255),
                    },
                    Default::default()
                );
            },
            Default::default()
        );
    }

    // draw circle outlines
    frame.path(
        |path| {
            path.circle((cx, cy), r0 - 0.5);
            path.circle((cx, cy), r1 + 0.5);
            path.stroke(
                Color::from_rgba(0, 0, 0, 64),
                StrokeOptions {
                    width: 1.0,
                    ..Default::default()
                }
            );
        },
        Default::default()
    );

    let transform = Transform::new().translate(cx, cy).rotate(hue * consts::PI * 2.0);

    // color selector
    frame.path(
        |path| {
            path.rect((r0 - 1.0, -3.0), (r1 - r0 + 2.0, 6.0));
            path.stroke(
                Color::from_rgba(255, 255, 255, 192),
                StrokeOptions {
                    width: 2.0,
                    ..Default::default()
                }
            );
        },
        PathOptions {
            transform: Some(transform),
            ..Default::default()
        }
    );

    // color marker inside selector
    frame.path(
        |path| {
            path.rect((r0 - 2.0 - 10.0, -4.0 - 10.0), (r1 - r0 + 4.0 + 20.0, 8.0 + 20.0));
            path.move_to((0.0, 0.0));
            path.rect((r0 - 2.0, -4.0), (r1 - r0 + 4.0, 8.0));
            path.winding(Winding::Solidity(Solidity::Hole));
            path.fill(
                Gradient::Box {
                    position: (r0 - 3.0, -5.0),
                    size: (r1 - r0 + 6.0, 10.0),
                    radius: 2.0,
                    feather: 4.0,
                    start_color: Color::from_rgba(0, 0, 0, 128),
                    end_color: Color::from_rgba(0, 0, 0, 0)
                },
                Default::default()
            );
        },
        PathOptions {
            transform: Some(transform),
            ..Default::default()
        }
    );

    let r = r0 - 6.0;

    // center triangle
    frame.path(
        |path| {
            let ax = f32::cos(120.0 / 180.0 * consts::PI) * r;
            let ay = f32::sin(120.0 / 180.0 * consts::PI) * r;
            let bx = f32::cos(-120.0 / 180.0 * consts::PI) * r;
            let by = f32::sin(-120.0 / 180.0 * consts::PI) * r;

            path.move_to((r, 0.0));
            path.line_to((ax, ay));
            path.line_to((bx, by));
            path.close();
            path.fill(
                Gradient::Linear {
                    start: (r, 0.0),
                    end: (ax, ay),
                    start_color: Color::from_hsla(hue, 1.0, 0.5, 255),
                    end_color: Color::from_rgba(255, 255, 255, 255),
                },
                Default::default()
            );
            path.fill(
                Gradient::Linear {
                    start: ((r + ax) * 0.5, ((0.0 + ay) * 0.5)),
                    end: (bx, by),
                    start_color: Color::from_rgba(0, 0, 0, 0),
                    end_color: Color::from_rgba(0, 0, 0, 255),
                },
                Default::default()
            );
            path.stroke(
                Color::from_rgba(0, 0, 0, 64),
                StrokeOptions {
                    width: 2.0,
                    ..Default::default()
                }
            );
        },
        PathOptions {
            transform: Some(transform),
            ..Default::default()
        }
    );


    let ax = f32::cos(120.0 / 180.0 * consts::PI) * r * 0.3;
    let ay = f32::sin(120.0 / 180.0 * consts::PI) * r * 0.4;

    // select circle on triangle
    frame.path(
        |path| {
            path.circle((ax, ay), 5.0);
            path.stroke(
                Color::from_rgba(255, 255, 255, 192),
                StrokeOptions {
                    width: 2.0,
                    ..Default::default()
                }
            );
        },
        PathOptions {
            transform: Some(transform),
            ..Default::default()
        }
    );

    // select circle outline
    frame.path(
        |path| {
            path.rect((ax - 20.0, ay - 20.0), (40.0, 40.0));
            path.move_to((0.0, 0.0));
            path.circle((ax, ay), 7.0);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.fill(
                Gradient::Radial {
                    center: (ax, ay),
                    inner_radius: 7.0,
                    outer_radius: 9.0,
                    start_color: Color::from_rgba(0, 0, 0, 64),
                    end_color: Color::from_rgba(0, 0, 0, 0),
                },
                Default::default()
            );
        },
        PathOptions {
            transform: Some(transform),
            ..Default::default()
        }
    );
}

fn draw_lines(frame: &Frame, x: f32, y: f32, w: f32, _h: f32, t: f32) {
    let pad = 5.0;
    let s = w / 9.0 - pad * 2.0;
    let mut pts = [0.0f32; 4 * 2];
    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];
    let joins = [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel];

    pts[0] = -s * 0.25 + f32::cos(t * 0.3) * s * 0.5;
    pts[1] = f32::sin(t * 0.3) * s * 0.5;
    pts[2] = -s * 0.25;
    pts[3] = 0.0;
    pts[4] = s * 0.25;
    pts[5] = 0.0;
    pts[6] = s * 0.25 + f32::cos(-t * 0.3) * s * 0.5;
    pts[7] = f32::sin(-t * 0.3) * s * 0.5;

    for i in 0..caps.len() {
        for j in 0..joins.len() {
            let fx = x + s * 0.5 + (i as f32 * 3.0 + j as f32) / 9.0 * w + pad;
            let fy = y - s * 0.5 + pad;

            let cap = caps[i];
            let join = joins[j];

            frame.path(
                |path| {
                    path.move_to((fx + pts[0], fy + pts[1]));
                    path.line_to((fx + pts[2], fy + pts[3]));
                    path.line_to((fx + pts[4], fy + pts[5]));
                    path.line_to((fx + pts[6], fy + pts[7]));

                    path.stroke(
                        Color::from_rgba(0, 0, 0, 160),
                        StrokeOptions {
                            width: s * 0.3,
                            line_cap: cap,
                            line_join: join,
                            ..Default::default()
                        }
                    );
                },
                Default::default(),
            );

            frame.path(
                |path| {
                    path.move_to((fx + pts[0], fy + pts[1]));
                    path.line_to((fx + pts[2], fy + pts[3]));
                    path.line_to((fx + pts[4], fy + pts[5]));
                    path.line_to((fx + pts[6], fy + pts[7]));

                    path.stroke(
                        Color::from_rgba(0, 192, 255, 255),
                        StrokeOptions {
                            width: 1.0,
                            line_cap: cap,
                            line_join: join,
                            ..Default::default()
                        }
                    );
                },
                Default::default(),
            );
        }
    }
}

fn draw_widths(frame: &Frame, x: f32, y: f32, width: f32) {
    let mut y = y;

    for i in 0..20 {
        let w = (i as f32 + 0.5) * 0.1;
        frame.path(
            |path| {
                path.move_to((x, y));
                path.line_to((x + width, y + width * 0.3));

                path.stroke(
                    Color::from_rgba(0, 0, 0, 255),
                    StrokeOptions {
                        width: w,
                        ..Default::default()
                    }
                );

                y += 10.0;
            },
            Default::default(),
        )
    }
}

fn draw_caps(frame: &Frame, x: f32, y: f32, width: f32) {
    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];
    let line_width = 8.0;

    frame.path(
        |path| {
            path.rect((x - line_width / 2.0, y), (width + line_width, 40.0));
            path.fill(Color::from_rgba(255, 255, 255, 32), Default::default());
        },
        Default::default(),
    );

    frame.path(
        |path| {
            path.rect((x, y), (width, 40.0));
            path.fill(Color::from_rgba(255, 255, 255, 32), Default::default());
        },
        Default::default(),
    );

    for i in 0..caps.len() {
        frame.path(
            |path| {
                path.move_to((x, y + i as f32 * 10.0 + 5.0));
                path.line_to((x + width, y + i as f32 * 10.0 + 5.0));

                path.stroke(
                    Color::from_rgba(0, 0, 0, 255),
                    StrokeOptions {
                        width: line_width,
                        line_cap: caps[i],
                        ..Default::default()
                    }
                );
            },
            Default::default(),
        );
    }
}

fn draw_scissor(frame: &Frame, x: f32, y: f32, t: f32) {
    let first_transform = Transform::new()
        .translate(x, y)
        .rotate(5.0f32.to_radians());

    frame.path(
        |path| {
            path.rect((-20.0, -20.0), (60.0, 40.0));
            path.fill(Color::from_rgba(255, 0, 0, 255), Default::default());
        },
        PathOptions {
            transform: Some(first_transform),
            ..Default::default()
        }
    );

    let second_transform = first_transform
        .translate(40.0, 0.0)
        .rotate(t);

    frame.path(
        |path| {
            path.rect((-20.0, -10.0), (60.0, 30.0));
            path.fill(Color::from_rgba(255, 128, 0, 64), Default::default());
        },
        PathOptions {
            transform: Some(second_transform),
            ..Default::default()
        }
    );

    frame.path(
        |path| {
            path.rect((-20.0, -10.0), (60.0, 30.0));
            path.fill(Color::from_rgba(255, 128, 0, 255), Default::default());
        },
        PathOptions {
            clip: Clip::Intersect(
                Intersect {
                    x: -20.0,
                    y: -10.0,
                    width: 60.0,
                    height: 30.0,
                    with: Scissor {
                        x: -20.0,
                        y: -20.0,
                        width: 60.0,
                        height: 40.0,
                        transform: Some(first_transform)
                    },
                    transform: Some(second_transform),
                }
            ),
            transform: Some(second_transform),
            ..Default::default()
        }
    );
}

fn draw_window(frame: &Frame, fonts: &DemoFonts, title: &str, x: f32, y: f32, w: f32, h: f32) {
    let corner_radius = 3.0;

    // window background
    frame.path(
        |path| {
            path.rounded_rect((x, y), (w, h), corner_radius);
            path.fill(Color::from_rgba(28, 30, 34, 192), Default::default());
        },
        Default::default(),
    );

    // drop shadow
    frame.path(
        |path| {
            path.rect((x - 10.0, y - 10.0), (w + 20.0, h + 30.0));
            path.move_to((x, y));
            path.rounded_rect((x, y), (w, h), corner_radius);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.fill(
                Gradient::Box {
                    position: (x, y + 2.0),
                    size: (w, h),
                    radius: corner_radius * 2.0,
                    feather: 10.0,
                    start_color: Color::from_rgba(0, 0, 0, 128),
                    end_color: Color::from_rgba(0, 0, 0, 0),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // header
    frame.path(
        |path| {
            path.rounded_rect((x + 1.0, y + 1.0), (w - 2.0, 30.0), corner_radius - 1.0);
            path.fill(
                Gradient::Linear {
                    start: (x, y),
                    end: (x, y + 15.0),
                    start_color: Color::from_rgba(255, 255, 255, 8),
                    end_color: Color::from_rgba(0, 0, 0, 16),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // header separator
    frame.path(
        |path| {
            path.move_to((x + 0.5, y + 0.5 + 30.0));
            path.line_to((x + 0.5 + w - 1.0, y + 0.5 + 30.0));

            path.stroke(Color::from_rgba(0, 0, 0, 32), Default::default());
        },
        Default::default(),
    );

    // header text
    frame.text(
        fonts.sans_bold,
        (x + w / 2.0, y + 16.0),
        title,
        TextOptions {
            size: 18.0,
            align: Alignment::new().center().middle(),
            color: Color::from_rgba(0, 0, 0, 128),
            blur: 2.0,
            ..Default::default()
        },
    );

    frame.text(
        fonts.sans_bold,
        (x + w / 2.0, y + 16.0),
        title,
        TextOptions {
            size: 18.0,
            align: Alignment::new().center().middle(),
            color: Color::from_rgba(220, 220, 220, 160),
            blur: 0.0,
            ..Default::default()
        },
    );
}

fn draw_search_box(frame: &Frame, fonts: &DemoFonts, text: &str, x: f32, y: f32, w: f32, h: f32) {
    let corner_radius = h / 2.0 - 1.0;

    // background rounded rectangle
    frame.path(
        |path| {
            path.rounded_rect((x, y), (w, h), corner_radius);
            path.fill(
                Gradient::Box {
                    position: (x, y + 1.5),
                    size: (w, h),
                    radius: h / 2.0,
                    feather: 5.0,
                    start_color: Color::from_rgba(0, 0, 0, 16),
                    end_color: Color::from_rgba(0, 0, 0, 92),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    frame.text(
        fonts.icons,
        (x + h * 0.55, y + h * 0.55),
        ICON_SEARCH,
        TextOptions {
            color: Color::from_rgba(255, 255, 255, 64),
            size: h * 1.3,
            align: Alignment::new().center().middle(),
            ..Default::default()
        },
    );

    frame.text(
        fonts.sans,
        (x + h * 1.05, y + h * 0.5),
        text,
        TextOptions {
            color: Color::from_rgba(255, 255, 255, 32),
            size: 20.0,
            align: Alignment::new().left().middle(),
            ..Default::default()
        },
    );

    frame.text(
        fonts.icons,
        (x + w - h * 0.55, y + h * 0.55),
        ICON_CIRCLED_CROSS,
        TextOptions {
            color: Color::from_rgba(255, 255, 255, 32),
            size: h * 1.3,
            align: Alignment::new().center().middle(),
            ..Default::default()
        },
    );
}

fn draw_drop_down(frame: &Frame, fonts: &DemoFonts, text: &str, x: f32, y: f32, w: f32, h: f32) {
    let corner_radius = 4.0;

    // drop down button with linear gradient
    frame.path(
        |path| {
            path.rounded_rect((x + 1.0, y + 1.0), (w - 2.0, h - 2.0), corner_radius - 1.0);
            path.fill(
                Gradient::Linear {
                    start: (x, y),
                    end: (x, y + h),
                    start_color: Color::from_rgba(255, 255, 255, 16),
                    end_color: Color::from_rgba(0, 0, 0, 16),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // border arond drop down
    frame.path(
        |path| {
            path.rounded_rect((x + 0.5, y + 0.5), (w - 1.0, h - 1.0), corner_radius - 0.5);
            path.stroke(Color::from_rgba(0, 0, 0, 48), Default::default());
        },
        Default::default(),
    );

    // main drop down text
    frame.text(
        fonts.sans,
        (x + h * 0.3, y + h * 0.5),
        text,
        TextOptions {
            color: Color::from_rgba(255, 255, 255, 160),
            size: 20.0,
            align: Alignment::new().left().middle(),
            ..Default::default()
        },
    );

    // chevron on right
    frame.text(
        fonts.icons,
        (x + w - h * 0.5, y + h * 0.5),
        ICON_CHEVRON_RIGHT,
        TextOptions {
            color: Color::from_rgba(255, 255, 255, 64),
            size: h * 1.3,
            align: Alignment::new().center().middle(),
            ..Default::default()
        },
    );
}

fn draw_label(frame: &Frame, fonts: &DemoFonts, text: &str, x: f32, y: f32, _w: f32, h: f32) {
    frame.text(
        fonts.sans,
        (x, y + h * 0.5),
        text,
        TextOptions {
            size: 18.0,
            color: Color::from_rgba(255, 255, 255, 128),
            align: Alignment::new().left().middle(),
            ..Default::default()
        },
    );
}

fn draw_edit_box_base(frame: &Frame, x: f32, y: f32, w: f32, h: f32) {
    let corner_radius = 4.0;

    // base background
    frame.path(
        |path| {
            path.rounded_rect((x + 1.0, y + 1.0), (w - 2.0, h - 2.0), corner_radius - 1.0);
            path.fill(
                Gradient::Box {
                    position: (x + 1.0, y + 1.0 + 1.5),
                    size: (w - 2.0, h - 2.0),
                    radius: 3.0,
                    feather: 4.0,
                    start_color: Color::from_rgba(255, 255, 255, 32),
                    end_color: Color::from_rgba(32, 32, 32, 32),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // base border
    frame.path(
        |path| {
            path.rounded_rect((x + 0.5, y + 0.5), (w - 1.0, h - 1.0), corner_radius - 0.5);
            path.stroke(Color::from_rgba(0, 0, 0, 48), Default::default());
        },
        Default::default(),
    );
}

fn draw_edit_box(frame: &Frame, fonts: &DemoFonts, text: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_edit_box_base(frame, x, y, w, h);

    frame.text(
        fonts.sans,
        (x + h * 0.3, y + h * 0.5),
        text,
        TextOptions {
            size: 20.0,
            color: Color::from_rgba(255, 255, 255, 64),
            align: Alignment::new().left().middle(),
            ..Default::default()
        },
    );
}

fn draw_edit_box_num(frame: &Frame, fonts: &DemoFonts, text: &str, units: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_edit_box_base(frame, x, y, w, h);

    let units_options = TextOptions {
        size: 18.0,
        color: Color::from_rgba(255, 255, 255, 64),
        align: Alignment::new().right().middle(),
        ..Default::default()
    };

    let (uw, _) = frame.text_bounds(fonts.sans, (0.0, 0.0), units, units_options);

    frame.text(fonts.sans, (x + w - h * 0.3, y + h * 0.5), units, units_options);

    frame.text(
        fonts.sans,
        (x + w - uw - h * 0.5, y + h * 0.5),
        text,
        TextOptions {
            size: 20.0,
            color: Color::from_rgba(255, 255, 255, 128),
            align: Alignment::new().right().middle(),
            ..Default::default()
        },
    );
}

fn draw_check_box(frame: &Frame, fonts: &DemoFonts, text: &str, x: f32, y: f32, _w: f32, h: f32) {
    // checkbox text
    frame.text(
        fonts.sans,
        (x + 28.0, y + h * 0.5),
        text,
        TextOptions {
            size: 18.0,
            color: Color::from_rgba(255, 255, 255, 160),
            align: Alignment::new().left().middle(),
            ..Default::default()
        },
    );

    // tick box
    frame.path(
        |path| {
            path.rounded_rect((x + 1.0, y + h * 0.5 - 9.0), (18.0, 18.0), 3.0);
            path.fill(
                Gradient::Box {
                    position: (x + 1.0, y + h * 0.5 - 9.0 + 1.0),
                    size: (18.0, 18.0),
                    radius: 3.0,
                    feather: 3.0,
                    start_color: Color::from_rgba(0, 0, 0, 32),
                    end_color: Color::from_rgba(0, 0, 0, 92),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // tick icon
    frame.text(
        fonts.icons,
        (x + 9.0 + 2.0, y + h * 0.5),
        ICON_CHECK,
        TextOptions {
            size: 40.0,
            color: Color::from_rgba(255, 255, 255, 128),
            align: Alignment::new().center().middle(),
            ..Default::default()
        },
    );
}

fn draw_button(frame: &Frame, fonts: &DemoFonts, preicon: Option<&str>, text: &str, x: f32, y: f32, w: f32, h: f32, color: Color) {
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
                    end: (x, y + h),
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

    let (tw, _) = frame.text_bounds(
        fonts.sans_bold,
        (0.0, 0.0),
        text,
        TextOptions {
            size: 20.0,
            ..Default::default()
        },
    );

    let mut iw = 0.0;

    if let Some(icon) = preicon {
        let icon_options = TextOptions {
            size: h * 1.3,
            color: Color::from_rgba(255, 255, 255, 96),
            align: Alignment::new().left().middle(),
            ..Default::default()
        };

        iw = frame.text_bounds(fonts.icons, (0.0, 0.0), icon, icon_options).0;
        iw += h * 0.15;

        frame.text(
            fonts.icons,
            (x + w * 0.5 - tw * 0.5 - iw * 0.75, y + h * 0.5),
            icon,
            icon_options,
        );
    }

    let mut options = TextOptions {
        size: 20.0,
        align: Alignment::new().left().middle(),
        ..Default::default()
    };

    options.color = Color::from_rgba(0, 0, 0, 160);

    frame.text(
        fonts.sans_bold,
        (x + w * 0.5 - tw * 0.5 + iw * 0.25, y + h * 0.5 - 1.0),
        text,
        options,
    );

    options.color = Color::from_rgba(255, 255, 255, 160);

    frame.text(
        fonts.sans_bold,
        (x + w * 0.5 - tw * 0.5 + iw * 0.25, y + h * 0.5),
        text,
        options,
    );
}

fn draw_slider(frame: &Frame, value: f32, x: f32, y: f32, w: f32, h: f32) {
    let cy = y + h * 0.5;
    let kr = h * 0.25;
    let corner_radius = 2.0;

    // slot bar
    frame.path(
        |path| {
            path.rounded_rect((x, cy - 2.0), (w, 4.0), corner_radius);
            path.fill(
                Gradient::Box {
                    position: (x, cy - 2.0 + 1.0),
                    size: (w, 4.0),
                    radius: 2.0,
                    feather: 2.0,
                    start_color: Color::from_rgba(0, 0, 0, 32),
                    end_color: Color::from_rgba(0, 0, 0, 128),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // knob shadow
    frame.path(
        |path| {
            path.rect(
                (x + (value * w) - kr - 5.0, cy - kr - 5.0),
                (kr * 2.0 + 5.0 + 5.0, kr * 2.0 + 5.0 + 5.0 + 3.0),
            );
            path.move_to((x, y));
            path.circle((x + value * w, cy), kr);
            path.winding(Winding::Solidity(Solidity::Hole));

            path.fill(
                Gradient::Radial {
                    center: (x + value * w, cy + 1.0),
                    inner_radius: kr - 3.0,
                    outer_radius: kr + 3.0,
                    start_color: Color::from_rgba(0, 0, 0, 64),
                    end_color: Color::from_rgba(0, 0, 0, 0),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // knob
    frame.path(
        |path| {
            path.circle((x + (value * w), cy), kr - 1.0);
            path.fill(Color::from_rgba(40, 43, 48, 255), Default::default());

            path.fill(
                Gradient::Linear {
                    start: (x, cy - kr),
                    end: (x, cy + kr),
                    start_color: Color::from_rgba(255, 255, 255, 16),
                    end_color: Color::from_rgba(0, 0, 0, 16),
                },
                Default::default()
            );
        },
        Default::default(),
    );

    // knob outline
    frame.path(
        |path| {
            path.circle((x + value * w, cy), kr - 0.5);
            path.stroke(Color::from_rgba(0, 0, 0, 92), Default::default());
        },
        Default::default(),
    );
}

fn draw_thumbnails(frame: &Frame, images: &Vec<Image>, x: f32, y: f32, w: f32, h: f32, t: f32) {
    let corner_radius = 3.0;
    let thumb = 60.0;
    let stackh = (images.len() / 2) as f32 * (thumb + 10.0) + 10.0;

    frame.path(
        |path| {
            path.rect((x - 10.0, y - 10.0), (w + 20.0, h + 30.0));
            path.move_to((x, y));
            path.rounded_rect((x, y), (w, h), corner_radius);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.fill(
                Gradient::Box {
                    position: (x, y + 4.0),
                    size: (w, h),
                    radius: corner_radius * 2.0,
                    feather: 20.0,
                    start_color: Color::from_rgba(0, 0, 0, 128),
                    end_color: Color::from_rgba(0, 0, 0, 0),
                },
                Default::default()
            );
        },
        Default::default()
    );

    // left arrow
    frame.path(
        |path| {
            let arry = 30.5;
            path.rounded_rect((x, y), (w, h), corner_radius);
            path.move_to((x - 10.0, y + arry));
            path.line_to((x + 1.0, y + arry - 11.0));
            path.line_to((x + 1.0, y + arry + 11.0));
            path.fill(Color::from_rgba(200, 200, 200, 255), Default::default());
        },
        Default::default()
    );

    let dv = 1.0 / (images.len() - 1) as f32;
    let u = (1.0 + f32::cos(t * 0.5)) * 0.5;
    let u2 = (1.0 - f32::cos(t * 0.2)) * 0.5;
    for (i, image) in images.iter().enumerate() {
        let tx = x + 10.0 + (i % 2) as f32 * (thumb + 10.0);
        let ty = y + 10.0 + (i / 2) as f32 * (thumb + 10.0);
        let (imgw, imgh) = image.size();
        let iw;
        let ih;
        let ix;
        let iy;

        if imgw < imgh {
            iw = thumb;
            ih = iw * imgh as f32 / imgw as f32;
            ix = 0.0;
            iy = -(ih - thumb) * 0.5;
        } else {
            ih = thumb;
            iw = ih * imgw as f32 / imgh as f32;
            ix = -(iw - thumb) * 0.5;
            iy = 0.0;
        }

        let v = i as f32 * dv;
        let a = clamp((u2 - v) / dv, 0.0, 1.0);

        let path_opts = PathOptions {
            clip: Clip::Scissor(
                Scissor{
                    x,
                    y,
                    width: w,
                    height: h,
                    transform: None,
                }
            ),
            transform: Some(Transform::new().translate(0.0, -(stackh - h) * u)),
            ..Default::default()
        };

        if a < 1.0 {
            draw_spinner(frame, path_opts, tx + thumb / 2.0, ty + thumb / 2.0, thumb * 0.25, t);
        }

        // draw image
        frame.path(
            |path| {
                path.rounded_rect((tx, ty), (thumb, thumb), 5.0);
                path.fill(
                    ImagePattern {
                        image: image,
                        origin: (tx + ix, ty + iy),
                        size: (iw, ih),
                        angle: 0.0 / 180.0 * consts::PI,
                        alpha: a
                    },
                    Default::default()
                );
            },
            path_opts
        );

        // draw image background shade
        frame.path(
            |path| {
                path.rect((tx - 5.0, ty - 5.0), (thumb + 10.0, thumb + 10.0));
                path.move_to((tx, ty));
                path.rounded_rect((tx, ty), (thumb, thumb), 6.0);
                path.winding(Winding::Solidity(Solidity::Hole));
                path.fill(
                    Gradient::Box {
                        position: (tx - 1.0, ty),
                        size: (thumb + 2.0, thumb + 2.0),
                        radius: 5.0,
                        feather: 3.0,
                        start_color: Color::from_rgba(0, 0, 0, 128),
                        end_color: Color::from_rgba(0, 0, 0, 0),
                    },
                    Default::default()
                );
            },
            path_opts
        );

        // draw image border
        frame.path(
            |path| {
                path.rounded_rect((tx + 0.5, ty + 0.5), (thumb - 1.0, thumb - 1.0), 4.0 - 0.5);
                path.stroke(
                    Color::from_rgba(255, 255, 255, 192),
                    StrokeOptions {
                        width: 1.0,
                        ..Default::default()
                    }
                );
            },
            path_opts
        );
    }

    // white fade border (top)
    frame.path(
        |path| {
            path.rect((x + 4.0, y), (w - 8.0, 6.0));
            path.fill(
                Gradient::Linear {
                    start: (x, y),
                    end: (x, y + 6.0),
                    start_color: Color::from_rgba(200, 200, 200, 255),
                    end_color: Color::from_rgba(200, 200, 200, 0),
                },
                Default::default()
            );
        },
        Default::default()
    );

    // white fade border (bottom)
    frame.path(
        |path| {
            path.rect((x + 4.0, y + h - 6.0), (w - 8.0, 6.0));
            path.fill(
                Gradient::Linear {
                    start: (x, y + h),
                    end: (x, y + h - 6.0),
                    start_color: Color::from_rgba(200, 200, 200, 255),
                    end_color: Color::from_rgba(200, 200, 200, 0),
                },
                Default::default()
            );
        },
        Default::default()
    );

    // scrollbar socket
    frame.path(
        |path| {
            path.rounded_rect((x + w - 12.0, y + 4.0), (8.0, h - 8.0), 3.0);
            path.fill(
                Gradient::Box {
                    position: (x + w - 12.0 + 1.0, y + 4.0 + 1.0),
                    size: (8.0, h - 8.0),
                    radius: 3.0,
                    feather: 4.0,
                    start_color: Color::from_rgba(0, 0, 0, 32),
                    end_color: Color::from_rgba(0, 0, 0, 92),
                },
                Default::default()
            );
        },
        Default::default()
    );

    let scrollh = (h / stackh) * (h - 8.0);
    // scrollbar thumb
    frame.path(
        |path| {
            path.rounded_rect(
                (x + w - 12.0 + 1.0, y + 4.0 + 1.0 + (h - 8.0 - scrollh) * u),
                (8.0 - 2.0, scrollh - 2.0),
                2.0
            );
            path.fill(
                Gradient::Box {
                    position: (x + w - 12.0 - 1.0, y + 4.0 + (h - 8.0 - scrollh) * u - 1.0),
                    size: (8.0, scrollh),
                    radius: 3.0,
                    feather: 4.0,
                    start_color: Color::from_rgba(220, 220, 220, 255),
                    end_color: Color::from_rgba(128, 128, 128, 255),
                },
                Default::default()
            );
        },
        Default::default()
    );
}

fn draw_spinner(frame: &Frame, options: PathOptions, cx: f32, cy: f32, r: f32, t: f32) {
    let a0 = 0.0 + t * 6.0;
    let a1 = consts::PI + t * 6.0;
    let r0 = r;
    let r1 = r * 0.75;

    frame.path(
        |path| {
            let ax = cx + f32::cos(a0) * (r0 + r1) * 0.5;
            let ay = cy + f32::sin(a0) * (r0 + r1) * 0.5;
            let bx = cx + f32::cos(a1) * (r0 + r1) * 0.5;
            let by = cy + f32::sin(a1) * (r0 + r1) * 0.5;
            path.arc((cx, cy), r0, a0, a1, Winding::Direction(Direction::Clockwise));
            path.arc((cx, cy), r1, a1, a0, Winding::Direction(Direction::CounterClockwise));
            path.fill(
                Gradient::Linear {
                    start: (ax, ay),
                    end: (bx, by),
                    start_color: Color::from_rgba(0, 0, 0, 0),
                    end_color: Color::from_rgba(0, 0, 0, 128)
                },
                Default::default()
            );
        },
        options
    );
}

enum GraphRenderStyle {
    Fps,
    Ms,
    Percent
}

struct PerformanceGraph {
    style: GraphRenderStyle,
    name: String,
    values: [f32; GRAPH_HISTORY_COUNT],
    head: usize
}

impl PerformanceGraph {
    fn new(style: GraphRenderStyle, name: &str) -> PerformanceGraph {
        PerformanceGraph {
            style,
            name: String::from(name),
            values: [0.0; GRAPH_HISTORY_COUNT],
            head: 0,
        }
    }

    fn update(&mut self, frame_time: f32) {
        self.head = (self.head + 1) % GRAPH_HISTORY_COUNT;
        self.values[self.head] = frame_time;
    }

    fn draw(&self, frame: &Frame, font: Font, x: f32, y: f32) {
        let w = 200.0;
        let h = 35.0;
        let average = self.average();

        frame.path(
            |path| {
                path.rect((x, y), (w, h));
                path.fill(Color::from_rgba(0, 0, 0, 128), Default::default());
            },
            Default::default()
        );

        frame.path(
            |path| {
                path.move_to((x, y + h));
                match self.style {
                    GraphRenderStyle::Fps => {
                        for i in 0..self.values.len() {
                            let v = 1.0 / (0.00001 + self.values[(self.head + i) % self.values.len()]);
                            let v = clamp(v, 0.0, 80.0);
                            let vx = x + (i as f32 / (self.values.len() - 1) as f32) * w;
                            let vy = y + h - ((v / 80.0) * h);
                            path.line_to((vx, vy));
                        }
                    },
                    GraphRenderStyle::Ms => {
                        for i in 0..self.values.len() {
                            let v = self.values[(self.head + i) % self.values.len()] * 1000.0;
                            let v = clamp(v, 0.0, 20.0);
                            let vx = x + (i as f32 / (self.values.len() - 1) as f32) * w;
                            let vy = y + h - ((v / 20.0) * h);
                            path.line_to((vx, vy));
                        }
                    },
                    GraphRenderStyle::Percent => {
                        for i in 0..self.values.len() {
                            let v = self.values[(self.head + i) % self.values.len()] * 1.0;
                            let v = clamp(v, 0.0, 100.0);
                            let vx = x + (i as f32 / (self.values.len() - 1) as f32) * w;
                            let vy = y + h - ((v / 100.0) * h);
                            path.line_to((vx, vy));
                        }
                    }
                }

                path.line_to((x + w, y + h));

                path.fill(Color::from_rgba(255, 192, 0, 128), Default::default());
            },
            Default::default()
        );

        frame.text(font, (x + 3.0, y + 1.0), &self.name, TextOptions {
            color: Color::from_rgba(240, 240, 240, 192),
            align: Alignment::new().left().top(),
            size: 14.0,
            ..Default::default()
        });

        match self.style {
            GraphRenderStyle::Fps => {
                frame.text(
                    font,
                    (x + w - 3.0, y + 1.0),
                    format!("{:.2} FPS", 1.0 / average),
                    TextOptions {
                        size: 18.0,
                        color: Color::from_rgba(240, 240, 240, 255),
                        align: Alignment::new().right().top(),
                        ..Default::default()
                    }
                );

                frame.text(
                    font,
                    (x + w - 3.0, y + h - 1.0),
                    format!("{:.2} ms", average * 1000.0),
                    TextOptions {
                        size: 15.0,
                        color: Color::from_rgba(240, 240, 240, 160),
                        align: Alignment::new().right().bottom(),
                        ..Default::default()
                    }
                );
            },
            GraphRenderStyle::Ms => {
                frame.text(
                    font,
                    (x + w - 3.0, y + 1.0),
                    format!("{:.2} ms", average * 1000.0),
                    TextOptions {
                        size: 18.0,
                        color: Color::from_rgba(240, 240, 240, 255),
                        align: Alignment::new().right().top(),
                        ..Default::default()
                    }
                );
            },
            GraphRenderStyle::Percent => {
                frame.text(
                    font,
                    (x + w - 3.0, y + 1.0),
                    format!("{:.1} %", average * 1.0),
                    TextOptions {
                        size: 18.0,
                        color: Color::from_rgba(240, 240, 240, 255),
                        align: Alignment::new().right().top(),
                        ..Default::default()
                    }
                )
            }
        }
    }

    fn average(&self) -> f32 {
        self.values.iter().sum::<f32>() / self.values.len() as f32
    }
}

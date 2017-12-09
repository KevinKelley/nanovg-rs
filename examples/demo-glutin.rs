extern crate glutin;
extern crate gl;
extern crate nanovg;

use std::time::Instant;
use std::f32::consts::PI;
use glutin::GlContext;
use nanovg::{FillStyle, StrokeStyle, ColoringStyle, Color, Paint, CompositeOperation,
             BasicCompositeOperation, PathOptions, Scissor, TextOptions, Alignment, Image, Font, Transform};

const INIT_WINDOW_SIZE: (u32, u32) = (1024, 720);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Glutin NanoVG")
        .with_dimensions(INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4)
        .with_srgb(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let context = nanovg::ContextBuilder::new().stencil_strokes().build().expect("Initialization of NanoVG failed!");
    let img = Image::new(&context)
        .repeat_x()
        .repeat_y()
        .build_from_file("resources/lenna.png")
        .expect("Couldn't load image");
    let mechanic_font =
        Font::from_file(&context, "Mechanic", "resources/Mechanic of the Heart.ttf")
            .expect("Failed to load font 'Mechanic of the Heart.ttf'");

    let start_time = Instant::now();
    let mut running = true;

    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => {}
                }
            }
            _ => {}
        });

        let (width, height) = gl_window.get_inner_size_pixels().unwrap();
        let (width, height) = (width as i32, height as i32);

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::Clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
            );
        }

        let elapsed = {
            let elapsed = start_time.elapsed();
            elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
        } as f32;

        // Let's draw a frame!

        context.frame((width, height), width as f32 / height as f32, |frame| {
            // Draw red-filled rectangle.
            frame.path(
                |path| {
                    path.rect((100.0, 100.0), (300.0, 300.0));
                    path.fill(FillStyle {
                        coloring_style: ColoringStyle::Paint(Paint::with_linear_gradient(
                            &context,
                            (100.0, 100.0),
                            (400.0, 400.0),
                            Color::from_rgb(0xAA, 0x6C, 0x39),
                            Color::from_rgb(0x88, 0x2D, 0x60),
                        )),
                        ..Default::default()
                    });
                },
                Default::default(),
            );

            // Draw custom yellow/blue shape.
            frame.path(
                |path| {
                    let origin = (150.0, 140.0);
                    path.circle(origin, 64.0);
                    path.sub_path(origin, |sp| {
                        sp.line_to((origin.0 + 300.0, origin.1 - 50.0));
                        sp.quad_bezier_to((origin.0 + 500.0, origin.1 + 100.0), (300.0, 100.0));
                        sp.close();
                    });
                    path.stroke(StrokeStyle {
                        coloring_style: ColoringStyle::Color(Color::new(1.0, 1.0, 0.0, 1.0)),
                        width: 3.0,
                        ..Default::default()
                    });
                    path.fill(FillStyle {
                        coloring_style: ColoringStyle::Color(Color::new(0.2, 0.0, 0.8, 1.0)),
                        ..Default::default()
                    });
                },
                PathOptions {
                    composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Lighter),
                    alpha: elapsed.cos() * 0.5 + 0.5,
                    transform: Some(Transform::new().scale(0.2, 1.0).translate(100.0, 50.0)),
                    ..Default::default()
                },
            );

            // Draw rolling image (with scissors)
            frame.path(
                |path| {
                    let radius = 100.0;
                    let distance = 500.0; // Distance to roll
                    let rolled = ((elapsed / 5.0).sin() * 0.5 + 0.5) * distance; // Distance currently rolled
                    let origin = (rolled + 100.0, 600.0);
                    let paint = Paint::with_image_pattern(
                        &context,
                        &img,
                        origin,
                        (100.0, 100.0),
                        rolled / (2.0 * PI * radius) * 2.0 * PI,
                        1.0,
                    );
                    path.circle(origin, radius);
                    path.fill(FillStyle {
                        coloring_style: ColoringStyle::Paint(paint),
                        ..Default::default()
                    })
                },
                PathOptions {
                    scissor: Some(Scissor::Rect {
                        x: 150.0,
                        y: 600.0,
                        width: 1000.0,
                        height: 200.0,
                    }),
                    ..Default::default()
                },
            );

            // Draw stroked rectangle.
            frame.path(
                |path| {
                    path.rect((300.0, 310.0), (300.0, 300.0));
                    let color = Color::lerp(
                        Color::from_rgb(0x2e, 0x50, 0x77),
                        Color::from_rgb(0xff, 0xca, 0x77),
                        elapsed.sin() * 0.5 + 0.5,
                    );
                    path.fill(FillStyle {
                        coloring_style: ColoringStyle::Color(Color::new(0.2, 0.2, 0.2, 0.7)),
                        ..Default::default()
                    });
                    path.stroke(StrokeStyle {
                        coloring_style: ColoringStyle::Color(color),
                        width: 5.0,
                        ..Default::default()
                    });
                },
                Default::default(),
            );
        });

        // Draw some strings!

        context.text(
            mechanic_font,
            (50.0, 50.0),
            "Hello world",
            TextOptions {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                size: 24.0,
                letter_spacing: (elapsed.sin() * 0.5 + 0.5) * 30.0,
                ..Default::default()
            },
        );

        context.text_box(
            mechanic_font,
            (50.0, 74.0),
            "Multi-\nline",
            TextOptions {
                color: Color::new(1.0, 0.6, 1.0, 1.0),
                size: 24.0,
                ..Default::default()
            },
        );

        context.text_box(
            mechanic_font,
            (800.0, 50.0),
            "This text is automatically wrapped.\nResize the window and try it out!",
            TextOptions {
                color: Color::new(0.6, 1.0, 1.0, 1.0),
                size: 24.0,
                align: Alignment::new().right().baseline(),
                line_height: 1.2,
                line_max_width: gl_window.get_inner_size().unwrap_or(INIT_WINDOW_SIZE).0 as f32 -
                    800.0,
                ..Default::default()
            },
        );

        gl_window.swap_buffers().unwrap();
    }
}

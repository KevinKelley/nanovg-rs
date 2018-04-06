extern crate gl;
extern crate glutin;
extern crate nanovg;

use glutin::GlContext;
use nanovg::{Color, Style, Font, Alignment, TextOptions,
             Frame, Transform, PathOptions, StrokeOptions, Paint, Scissor, Clip};
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

        context.frame((width, height), gl_window.hidpi_factor(), |frame| {
            let (width, height) = (width as f32, height as f32);

            frame.path(
                |path| {
                    path.rect((0.0, 0.0), (50.0, 50.0));
                    let color = Color::from_rgb(255,0,0);
                    path.fill(color, Default::default());
                },
                Default::default()
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

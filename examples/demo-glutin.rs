extern crate glutin;
extern crate gl;
extern crate nanovg;

use glutin::GlContext;

use nanovg::{StrokeStyle, ColoringStyle, Color};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("nanovg Test")
        .with_dimensions(1024, 720);
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
    
    let context = nanovg::Context::with_gl3(nanovg::CreateFlags::new().stencil_strokes()).unwrap();

    let mut running = true;

    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => {},
            },
            _ => {},
        });

        let (width, height) = gl_window.get_inner_size_pixels().unwrap();
        let (width, height) = (width as i32, height as i32);

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        let ratio = width as f32 / height as f32;
        context.frame((width, height), ratio, |frame| {
            // Draw red-filled rectangle.
            frame.path(|path| {
                path.rect((100.0, 100.0), (120.0, 30.0));
                path.fill(ColoringStyle::Color(Color::new(1.0, 0.0, 0.0, 1.0)));
            });

            // Draw blue-stroked rectangle.
            frame.path(|path| {
                path.rect((100.0, 140.0), (120.0, 30.0));
                path.stroke(StrokeStyle {
                    coloring_style: ColoringStyle::Color(Color::new(0.0, 0.6, 0.8, 1.0)),
                    width: 5.0,
                    .. Default::default()
                });
            });

            context.global_alpha(0.4);

            // Draw custom yellow shape.
            frame.path(|path| {
                path.circle((300.0, 300.0), 64.0);
                path.sub_path((300.0, 300.0), |sp| {
                    sp.line_to((600.0, 300.0));
                    sp.quad_bezier_to((800.0, 600.0), (100.0, 100.0));
                });
                path.stroke(StrokeStyle {
                    coloring_style: ColoringStyle::Color(Color::new(1.0, 1.0, 0.0, 1.0)),
                    width: 3.0,
                    .. Default::default()
                });
                path.fill(ColoringStyle::Color(Color::new(0.2, 0.0, 0.8, 1.0)));
            });
        });

        gl_window.swap_buffers().unwrap();
    }
}

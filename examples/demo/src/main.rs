
#![feature(start)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unstable)]

extern crate num;
//extern crate native;
extern crate libc;

extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
use std::cell::Cell; // for glfw error count
use nanovg::Ctx;


/// evaluate the expression, then check for GL error.
macro_rules! glcheck {
    ($e: expr) => (
        {
            $e;
            assert_eq!(unsafe {gl::GetError()}, 0);
        }
    )
}


mod perf;
mod demo;

/*#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}*/



/// give GLFW a way to report errors, and count them.
fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    glcheck!(unsafe {gl::FrontFace(gl::CCW)});
    glcheck!(unsafe {gl::Enable(gl::DEPTH_TEST)});
    glcheck!(unsafe {gl::Enable(gl::SCISSOR_TEST)});
    glcheck!(unsafe {gl::DepthFunc(gl::LEQUAL)});
    glcheck!(unsafe {gl::FrontFace(gl::CCW)});
    glcheck!(unsafe {gl::Enable(gl::CULL_FACE)});
    glcheck!(unsafe {gl::CullFace(gl::BACK)});
}


static mut blowup: bool = false;
static mut screenshot: bool = false;
static mut premult: bool = false;

fn main()
{
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // TODO: Get this working? No idea what's wrong with it, but the example of usage in glfw-rs is out of date
	/*// set up GLFW error callback, with an error-counter
	glfw.set_error_callback(Some(
	    glfw::Callback {
	        f: error_callback,
	        data: Cell::new(0),
	    }
	));*/


	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
 	glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
 	glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));
 	glfw.window_hint(glfw::WindowHint::OpenglDebugContext(true));

    let (mut window, events) = glfw.create_window(1100, 800, "NanoVG GL3 Rust demo", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

	// window.set_key_callback(key);
    window.set_key_polling(true);

    window.make_current();

    // use glfw to load GL function pointers
    glcheck!(gl::load_with(|name| window.get_proc_address(name)));
    init_gl();

   	let vg: nanovg::Ctx = nanovg::Ctx::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
   	//assert!(!vg.ptr.is_null());

    let data = demo::DemoData::load(&vg, "../../res");

//    return test_linebreaks(vg);

	glfw.set_swap_interval(0);

	glfw.set_time(0.0);
	let mut prevt = glfw.get_time();

	let mut fps = perf::PerfGraph::init(perf::Style::FPS, "Frame Time");

    while !window.should_close()
    {
    	let t: f64 = glfw.get_time();
    	let dt: f64 = t - prevt;
    	prevt = t;
    	fps.update(dt);

        let (mx, my) = window.get_cursor_pos(); // (f64,f64)
        let (winWidth, winHeight) = window.get_size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.get_framebuffer_size();
        // Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f32 / winWidth as f32;

        // Update and render
        glcheck!(unsafe {gl::Viewport(0, 0, fbWidth, fbHeight)});
        if unsafe {premult} {
            glcheck!(unsafe {gl::ClearColor(0.0, 0.0, 0.0, 0.0)});
        } else {
            glcheck!(unsafe {gl::ClearColor(0.3, 0.3, 0.32, 1.0)});
        }
        glcheck!(unsafe {gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT)});

        glcheck!(unsafe {gl::Enable(gl::BLEND)});
        glcheck!(unsafe {gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)});
        glcheck!(unsafe {gl::Enable(gl::CULL_FACE)});
        glcheck!(unsafe {gl::Disable(gl::DEPTH_TEST)});

        vg.begin_frame(winWidth, winHeight, pxRatio as f32);

        unsafe { demo::render_demo(&vg, mx as f32,my as f32, winWidth as f32,winHeight as f32, t as f32, blowup, &data); }
        fps.render(&vg, 5.0, 5.0);

        vg.end_frame();


        unsafe {gl::Enable(gl::DEPTH_TEST);}
        unsafe {
	        if screenshot {
	        	screenshot = false;
	        	demo::save_screenshot(fbWidth as u32, fbHeight as u32, premult, "dump.png");
	        }
        }

    	window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
            unsafe {blowup = !blowup};
        }
        glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press, _) => {
            unsafe {screenshot = true};
        }
        glfw::WindowEvent::Key(glfw::Key::P, _, glfw::Action::Press, _) => {
            unsafe {premult = !premult};
        }
        _ => {}
    }
}

//// think linebreaks api needs some love
//fn test_linebreaks(vg:Ctx) {
//    let x=0.0; let y=0.0;
//    let width = 120.0;
//
//    let text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.";
//    //let text = "01234 6789 1234 6789 1234 6789 123456789012345678901234567890123456789012345678901234567890123456789";
//
//
//    // The text break API can be used to fill a large buffer of rows,
//    // or to iterate over the text just few lines (or just one) at a time.
//    // The "next" variable of the last returned item tells where to continue.
//    let mut start: uint = 0;    // byte pos in utf8 'text' str
//    let end: uint = text.len(); // exclusive
//    'chunks: loop {
//        let text = text.slice(start, end);
//
//println!("{}", text);
//
//        let rows = vg.text_break_lines(text, width, 3);
//        let nrows = rows.len();
//        if nrows == 0 { break 'chunks; }
//        for i in range(0, nrows) {
//            let row = &rows[i];
//            let line = text.slice(row.start_index(), row.end_index());
//
//println!("i: {}  st: {}, en: {}  \t {} \tnext: {}", i, row.start_index(), row.end_index(), line, row.next_index());
//
//        }
//        // Keep going...
//        start += rows[nrows-1].next_index();
//    }
//}

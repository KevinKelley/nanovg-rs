//#include <stdio.h>
//#define GLFW_INCLUDE_ES3
//#include <GLFW/glfw3.h>
//#include "nanovg.h"
//#define NANOVG_GLES3_IMPLEMENTATION
//#include "nanovg_gl.h"
//#include "nanovg_gl_utils.h"
//#include "demo.h"
//#include "perf.h"

#![feature(globs)]
#![feature(macro_rules)]
//#![macro_escape]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variable)]

extern crate num;
extern crate native;
extern crate libc;
extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
use std::cell::Cell; // for glfw error count
use nanovg::Ctx;


/// evaluate the expression, then check for GL error.
macro_rules! glcheck(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)


mod perf;
mod demo;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}



/// give GLFW a way to report errors, and count them.
fn error_callback(_: glfw::Error, description: String, error_count: &Cell<uint>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::DEPTH_TEST));
    glcheck!(gl::Enable(gl::SCISSOR_TEST));
    glcheck!(gl::DepthFunc(gl::LEQUAL));
    glcheck!(gl::FrontFace(gl::CCW));
    glcheck!(gl::Enable(gl::CULL_FACE));
    glcheck!(gl::CullFace(gl::BACK));
}


static mut blowup: bool = false;
static mut screenshot: bool = false;
static mut premult: bool = false;

fn main()
{
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	// set up GLFW error callback, with an error-counter
	glfw.set_error_callback(Some(
	    glfw::Callback {
	        f: error_callback,
	        data: Cell::new(0),
	    }
	));


	glfw.window_hint(glfw::ContextVersion(3, 2));
 	glfw.window_hint(glfw::OpenglForwardCompat(true));
 	glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
 	glfw.window_hint(glfw::OpenglDebugContext(true));

    let (window, events) = glfw.create_window(1100, 800, "NanoVG GL3 exmaple", glfw::Windowed)
        .expect("Failed to create GLFW window.");

	// window.set_key_callback(key);
    window.set_key_polling(true);

    window.make_current();

    // use glfw to load GL function pointers
    glcheck!(gl::load_with(|name| glfw.get_proc_address(name)));
    init_gl();

   	let vg: nanovg::Ctx = nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
   	assert!(!vg.ptr.is_null());
    //println!("created nanovg Ctx: {}", vg);

    let mut data = demo::DemoData::load(&vg);

	//if (vg == NULL) {
	//  printf("Could not init nanovg.\n");
	//  return -1;
	//}

//  if (loadDemoData(vg, &data) == -1)
//    return -1;

	glfw.set_swap_interval(0);

	glfw.set_time(0.0);
	let mut prevt = glfw.get_time();

	let mut fps = perf::PerfGraph::init(perf::FPS, "Frame Time");

    while !window.should_close()
    {
    	let premult = unsafe { premult };
    	let blowup  = unsafe { blowup  };

    	let t: f64 = glfw.get_time();
    	let dt: f64 = t - prevt;
    	prevt = t;
    	fps.update(dt);

        let (mx, my) = window.get_cursor_pos(); // (f64,f64)
        let (winWidth, winHeight) = window.get_size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.get_framebuffer_size();
        // Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f64 / winWidth as f64;

        // Update and render
        glcheck!(gl::Viewport(0, 0, fbWidth, fbHeight));
        if premult {
        	glcheck!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
        } else {
        	glcheck!(gl::ClearColor(0.3, 0.3, 0.32, 1.0));
        }
        glcheck!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

        glcheck!(gl::Enable(gl::BLEND));
        glcheck!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        glcheck!(gl::Enable(gl::CULL_FACE));
        glcheck!(gl::Disable(gl::DEPTH_TEST));


        vg.begin_frame(winWidth, winHeight, pxRatio as f32);

        demo::render_demo(&vg, mx as f32,my as f32, winWidth as f32,winHeight as f32, t as f32, blowup, &data);
        fps.render(&vg, 5.0, 5.0);

        vg.end_frame();


        gl::Enable(gl::DEPTH_TEST);

        unsafe {
	        if screenshot {
	        	screenshot = false;
	        	demo::save_screenshot(fbWidth, fbHeight, premult, "dump.png");
	        }
        }

    	window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }

// cleanup should be handled by Drop trait
//  freeDemoData(vg, &data);
//  nvgDeleteGLES3(vg);
//  glfwTerminate();
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
            unsafe {blowup = !blowup};
        }
        glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) => {
            unsafe {screenshot = true};
        }
        glfw::KeyEvent(glfw::KeyP, _, glfw::Press, _) => {
            unsafe {premult = !premult};
        }
        _ => {}
    }
}

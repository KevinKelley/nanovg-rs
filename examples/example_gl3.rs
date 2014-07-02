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

extern crate native;
extern crate libc;
extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
//use gl::*;
use gl::types::*;
use std::cell::Cell;

use nanovg::NVGcolor;
use nanovg::NVGcontext;



macro_rules! verify(
    ($e: expr) => (
        {
            $e;
            assert_eq!(gl::GetError(), 0);
        }
    )
)


mod perf;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}



fn error_callback(_: glfw::Error, description: String, error_count: &Cell<uint>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn init_gl() {
    verify!(gl::FrontFace(gl::CCW));
    verify!(gl::Enable(gl::DEPTH_TEST));
    verify!(gl::Enable(gl::SCISSOR_TEST));
    verify!(gl::DepthFunc(gl::LEQUAL));
    verify!(gl::FrontFace(gl::CCW));
    verify!(gl::Enable(gl::CULL_FACE));
    verify!(gl::CullFace(gl::BACK));
}


static blowup: bool = false;
static screenshot: bool = false;
static premult: bool = false;

fn main()
{
//  struct DemoData data;

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	//glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
	glfw.set_error_callback(Some(
	    glfw::Callback {
	        f: error_callback,
	        data: Cell::new(0),
	    }
	));

	//#ifndef _WIN32 // don't require this on win32, and works with more cards
	//	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
	//	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
	//	glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
	//	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
	//#endif
	//	glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, 1);

	glfw.window_hint(glfw::ContextVersion(3, 2));
 	glfw.window_hint(glfw::OpenglForwardCompat(true));
 	glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
 	glfw.window_hint(glfw::OpenglDebugContext(true));

    let (window, events) = glfw.create_window(300, 300, "NanoVG GL3 exmaple", glfw::Windowed)
        .expect("Failed to create GLFW window.");

	// window.set_key_callback(key);
    window.set_key_polling(true);

    window.make_current();

    // use glfw to load GL function pointers
    verify!(gl::load_with(|name| glfw.get_proc_address(name)));
    init_gl();

    let vg: *mut nanovg::NVGcontext;
    unsafe {
     	vg = nanovg::nvgCreateGL3(nanovg::NVG_ANTIALIAS | nanovg::NVG_STENCIL_STROKES);
     	assert!(!vg.is_null());
    }
    println!("created NVGcontext: {}", vg);

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
    	let mut t: f64 = glfw.get_time();
    	let mut dt: f64 = t - prevt;
    	prevt = t;
    	fps.update(dt);

        let (mx, my) = window.get_cursor_pos(); // (f64,f64)
        let (winWidth, winHeight) = window.get_size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.get_framebuffer_size();
        // Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f64 / winWidth as f64;

        // Update and render
        verify!(gl::Viewport(0, 0, fbWidth, fbHeight));
        if premult {
        	verify!(gl::ClearColor(0.0, 0.0, 0.0, 0.0));
        } else {
        	verify!(gl::ClearColor(0.3, 0.3, 0.32, 1.0));
        }
        verify!(gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT));

        verify!(gl::Enable(gl::BLEND));
        verify!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        verify!(gl::Enable(gl::CULL_FACE));
        verify!(gl::Disable(gl::DEPTH_TEST));


        unsafe { nanovg::nvgBeginFrame(vg, winWidth, winHeight, pxRatio as f32); }

        //renderDemo(vg, mx,my, winWidth,winHeight, t, blowup, &data);
        fps.render(vg, 5.0, 5.0);

        unsafe { nanovg::nvgEndFrame(vg); }


        gl::Enable(gl::DEPTH_TEST);

        //if screenshot {
        //  screenshot = false;
        //  saveScreenShot(fbWidth, fbHeight, premult, "dump.png");
        //}

    	window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }
    }

//  freeDemoData(vg, &data);
//
//  nvgDeleteGLES3(vg);
//
//  glfwTerminate();
//  return 0;
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
//static void key(GLFWwindow* window, int key, int scancode, int action, int mods)
//{
//  NVG_NOTUSED(scancode);
//  NVG_NOTUSED(mods);
//  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
//    glfwSetWindowShouldClose(window, GL_TRUE);
//  if (key == GLFW_KEY_SPACE && action == GLFW_PRESS)
//    blowup = !blowup;
//  if (key == GLFW_KEY_S && action == GLFW_PRESS)
//    screenshot = 1;
//  if (key == GLFW_KEY_P && action == GLFW_PRESS)
//    premult = !premult;
//}
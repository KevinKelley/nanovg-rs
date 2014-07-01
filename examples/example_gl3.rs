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

extern crate native;
extern crate libc;
extern crate glfw;
extern crate gl;
extern crate nanovg;

use glfw::Context;
use std::cell::Cell;

use nanovg::NVGcolor;
use nanovg::NVGcontext;

//mod perf;
pub mod perf {
	use nanovg::NVGcontext;

	pub enum Style {
	    FPS,

	    MS
	}

	static CAP:int = 100;

	pub struct PerfGraph {
		pub style: Style,
		pub name: String,
		pub values: [f64, ..CAP],
		pub head: int,
		pub count: int,
	}

	//void initGraph(struct PerfGraph* fps, int style, const char* name);
	pub fn init(grf: &PerfGraph, style: Style, name: &str)
	{}
	//void updateGraph(struct PerfGraph* fps, float frameTime);
	pub fn update(grf: &mut PerfGraph, frameTime: f64)
	{
		if grf.count == CAP { grf.head = (grf.head + 1) % CAP }
		grf.count = if grf.count < CAP { grf.count + 1 } else { CAP } ;
		grf.values[((grf.head+grf.count) % CAP) as uint] = frameTime;
	}
	//void renderGraph(struct NVGcontext* vg, float x, float y, struct PerfGraph* fps);
	pub fn render(grf: &PerfGraph, vg: &NVGcontext, x: f64, y: f64)
	{}
	//float getGraphAverage(struct PerfGraph* fps);
	pub fn getGraphAverage(grf: &PerfGraph) -> f64
	{
		let mut sum: f64 = 0.0;
		let mut i = grf.head;
		while i < grf.head + grf.count {
			let ix: uint = (i % CAP) as uint;
			sum += grf.values[ix];
			i = i+1;
		}
		sum / grf.count as f64
	}
} // mod perf

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}



fn error_callback(_: glfw::Error, description: String, error_count: &Cell<uint>) {
    println!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}


static blowup: bool = false;
static screenshot: bool = false;
static premult: bool = false;

fn main() {
//  GLFWwindow* window;
//  struct DemoData data;
//  struct NVGcontext* vg = NULL;
//  struct PerfGraph fps;
//  double prevt = 0;

    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	//glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
	glfw.set_error_callback(Some(
	    glfw::Callback {
	        f: error_callback,
	        data: Cell::new(0),
	    }
	));

 	glfw.window_hint(glfw::ClientApi(glfw::OpenGlEsApi));
	glfw.window_hint(glfw::ContextVersion(3, 0));

    let (window, events) = glfw.create_window(300, 300, "NanoVG GL3 exmaple", glfw::Windowed)
        .expect("Failed to create GLFW window.");

//  glfwSetKeyCallback(window, key);
    window.set_key_polling(true);

    window.make_current();

//  vg = nvgCreateGLES3(NVG_ANTIALIAS | NVG_STENCIL_STROKES);
//  if (vg == NULL) {
//    printf("Could not init nanovg.\n");
//    return -1;
//  }
//
//  if (loadDemoData(vg, &data) == -1)
//    return -1;

	glfw.set_swap_interval(0);

	glfw.set_time(0.0);
	let mut prevt = glfw.get_time();

	let mut fps: perf::PerfGraph = perf::PerfGraph {
		style: perf::FPS,
		name: String::from_str("Frame Time"),
		values: [0.0, ..100],
		head: 0,
		count: 0,
	};
//  initGraph(&fps, GRAPH_RENDER_FPS, "Frame Time");

    while !window.should_close()
    {
		// double mx, my, t, dt;
		// int winWidth, winHeight;
		// int fbWidth, fbHeight;
		// float pxRatio;

    	let mut t: f64 = glfw.get_time();
    	let mut dt: f64 = t - prevt;
    	prevt = t;
    	perf::update(&mut fps, dt);

        let (mx, my) = window.get_cursor_pos(); // (f64,f64)
        let (winWidth, winHeight) = window.get_size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.get_framebuffer_size();
        //// Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f64 / winWidth as f64;

        // Update and render
        gl::Viewport(0, 0, fbWidth, fbHeight);
        if premult {
        	gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        } else {
        	gl::ClearColor(0.3, 0.3, 0.32, 1.0);
        }
        gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::CULL_FACE);
        gl::Disable(gl::DEPTH_TEST);

        //nvgBeginFrame(vg, winWidth, winHeight, pxRatio);
        //
        //renderDemo(vg, mx,my, winWidth,winHeight, t, blowup, &data);
        //renderGraph(vg, 5,5, &fps);

        //nvgEndFrame(vg);

        gl::Enable(gl::DEPTH_TEST);

        //if (screenshot) {
        //  screenshot = 0;
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

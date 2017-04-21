#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

extern crate gl;
extern crate sdl2;
extern crate sdl2_sys;
extern crate nanovg;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

mod perf;
mod demo;

/// evaluate the expression, then check for GL error.
macro_rules! glcheck {
    ($e: expr) => (
        {
            $e;
            assert_eq!(unsafe {gl::GetError()}, 0);
        }
    )
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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_flags().debug().forward_compatible().set();
    gl_attr.set_context_version(3, 2);
    gl_attr.set_stencil_size(8);

    let window = video_subsystem.window("NanoVG GL3 Rust-sdl2 Demo", 1100, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();


    let gl_context = window.gl_create_context().unwrap();
    match window.gl_make_current(&gl_context) {
        Err(val) => {
            println!("make_current error: {}", val);
            return;
        },
        _ => {}
    }

    glcheck!(gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _));
    init_gl();


   	let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
    let data = demo::DemoData::load(&vg, "res");

    let mut timer = sdl_context.timer().unwrap();

    let mut prevt: f64 = (timer.ticks() as f64)/1000.0;
    let mut fps = perf::PerfGraph::init(perf::Style::FPS, "Frame Time");
    let mut mx = 0;
    let mut my = 0;

    'running: loop {
        let t: f64 = (timer.ticks() as f64)/1000.0;
        let dt: f64 = t - prevt;
        prevt = t;
        fps.update(dt);

        let (winWidth, winHeight) = window.size();  // (i32,i32)
        let (fbWidth, fbHeight) = window.drawable_size();
        // Calculate pixel ration for hi-dpi devices.
        let pxRatio = fbWidth as f32 / winWidth as f32;

        // Update and render
        glcheck!(unsafe {gl::Viewport(0, 0, fbWidth as i32, fbHeight as i32)});
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

        vg.begin_frame(winWidth as u32, winHeight as u32, pxRatio as f32);

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

    	window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    unsafe {screenshot = true;}
                },
                Event::KeyDown { keycode: Some(Keycode::P), ..} => {
                    unsafe {premult = !premult;}
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    unsafe {blowup = !blowup;}
                },
                Event::MouseMotion {x, y, ..}  => {
                    mx = x;
                    my = y;
                }
                _ => {}
            }
        }
    }
}

extern crate glutin;
extern crate gl;
extern crate nanovg;
extern crate chrono;

use std::f32::consts::PI;
use glutin::GlContext;
use nanovg::{StrokeOptions, Style, Color, Paint, CompositeOperation,
             BasicCompositeOperation, PathOptions, TextOptions, Font, Transform, Alignment };
use chrono::prelude::*;

const INIT_WINDOW_SIZE: (u32, u32) = (480, 480);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("NanoVG Clock")
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
    let roboto_font =
        Font::from_file(&context, "Roboto", "resources/Roboto-Regular.ttf")
            .expect("Failed to load font 'Roboto-Regular.ttf'");

    let mut running = true;

    let mut prev_second = -1.0;

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

        let dt: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
        let hour = dt.hour();
        let am = hour < 12;
        let hour: f32 = f64::from(hour % 12) as f32;
        let minute: f32 = f64::from(dt.minute()) as f32;
        let second: f32 = f64::from(dt.second()) as f32;
        let year: i32 = dt.year();
        let month: u32 = dt.month();
        let day: u32 = dt.day();

        // don't bother re-draw unless time has changed
        if second == prev_second {
            continue;
        }
        else {
            prev_second = second;
        }

        let (width, height) = gl_window.get_inner_size().unwrap();
        let (width, height) = (width as i32, height as i32);

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::Clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
            );
        }


        // round clock size is minimum of height and width
        let clock_size = width.min(height) - 2;

        let font_size = 24.0;

        let origin = (0.0, 0.0);  // upper-left corner
        let dial_center = (f64::from(width) as f32 / 2.0, f64::from(height) as f32 / 2.0);
        let dial_radius: f32 = f64::from(clock_size / 2) as f32;
        let second_hand_len = dial_radius * 0.9;
        let minute_hand_len = dial_radius * 0.8;
        let hour_hand_len   = dial_radius * 0.6;

        let two_pi = 2.0 * PI;
        let radians_per_sec = two_pi / 60.0;
        let radians_per_hour = two_pi / 12.0;


        let white:     Color = Color::new(1.0, 1.0, 1.0, 1.0);
        let silver:    Color = Color::from_rgb(196,199,206);
        let darksilver:Color = Color::from_rgb(148,152,161);
        let darkgray:  Color = Color::from_rgb(169,169,169);
        let dial_color = Color::new(0.2, 0.0, 0.8, 1.0);



        context.frame((width, height), gl_window.hidpi_factor(), |frame| {

            // hour/minute markers

            //let sigils = ["XII", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII"];
            let sigils: Vec<String> = (0..13).map(|n| { format!("{}", n) }).collect();
            for h in 1..13 {
                let j = f64::from(h) as f32;
                let x = dial_center.0 + (second_hand_len * (j*radians_per_hour).sin());
                let y = dial_center.1 - (second_hand_len * (j*radians_per_hour).cos());
                frame.text(
                    roboto_font,
                    (x, y),
                    &sigils[h as usize],
                    TextOptions {
                        color: silver,
                        size: font_size,
                        align: Alignment::new().center().middle(),
                        ..Default::default()
                    },
                );
            }
            'ticks: for m in 1..61 {
                if m % 5 == 0 { continue 'ticks; }
                let m = f64::from(m) as f32;
                let ticks_radius = dial_radius * 0.925;
                let tick_len = 3.0;
                let tick_width = 1.0;
                frame.path(
                    |path| {
                        path.move_to((0.0, -ticks_radius));
                        path.line_to((0.0, -ticks_radius-tick_len));
                        path.close();
                        path.stroke(
                            Style::Color(white),
                            StrokeOptions {
                                width: tick_width,
                                ..Default::default()
                            }
                        );
                        path.fill(Style::Color(white), Default::default());
                    },
                    PathOptions {
                        composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Lighter),
                        alpha: 1.0,
                        transform: Some(Transform::new().translate(dial_center.0, dial_center.1).rotate(m * radians_per_sec)),
                        ..Default::default()
                    },
                );
            }

            // time-string
            let show_time_string = false;
            if show_time_string {
                frame.text(
                    roboto_font,
                    (dial_center.0, dial_center.1 + dial_radius * 0.7 - font_size),
                    format!("{}:{:02}:{:02} {}", hour, minute, second, if am { "AM"}  else { "PM" }),
                        TextOptions {
                            color: silver,
                            size: font_size,
                            align: Alignment::new().center().baseline(),
                            ..Default::default()
                        },
                    );
            }
            // date-string
            frame.text(
                roboto_font,
                (dial_center.0, dial_center.1 + dial_radius * 0.7),
                format!("{:4}-{:02}-{:02}", year, month, day),
                    TextOptions {
                        color: silver,
                        size: font_size,
                        align: Alignment::new().center().baseline(),
                        ..Default::default()
                    },
                );


            //Draw the dial
            frame.path(
                |path| {
                    path.circle(origin, dial_radius);
                    path.stroke(
                        Style::Color(silver),
                        StrokeOptions {
                            width: 3.0,
                            ..Default::default()
                        }
                    );
                    path.fill(Style::Color(dial_color), Default::default());
                },
                PathOptions {
                    composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Lighter),
                    alpha: 1.0, //elapsed.cos() * 0.5 + 0.5,
                    transform: Some(Transform::new().translate(dial_center.0, dial_center.1)),
                    ..Default::default()
                },
            );

            let draw_hand = |theta: f32, length: f32, width: f32| {
                frame.path(
                    |path| {
                        path.move_to(origin);
                        path.line_to((0.0, -length));
                        path.close();
                        path.stroke(
                            Style::Color(white),
                            StrokeOptions {
                                width: width,
                                ..Default::default()
                            }
                        );
                        path.fill(Style::Color(white), Default::default());
                    },
                    PathOptions {
                        composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Lighter),
                        alpha: 1.0,
                        transform: Some(Transform::new().translate(dial_center.0, dial_center.1).rotate(theta)),
                        ..Default::default()
                    },
                );
            };

            // draw the hands

            //let hour_angle = hour*radians_per_hour + minute*PI/360.0;
            let hour_angle = (((hour * 60.0 + minute) / 60.0) / 12.0) * two_pi;
            let minute_angle = minute * radians_per_sec;
            let second_angle = second * radians_per_sec;

            draw_hand(second_angle, second_hand_len, 1.0);
            draw_hand(minute_angle, minute_hand_len, 3.0);
            draw_hand(hour_angle, hour_hand_len, 5.0);


            //Draw the boss
            frame.path(
                |path| {
                    let boss_rad = 6.0;
                    path.circle(origin, boss_rad);
                    path.stroke(
                        Style::Color(darkgray),
                        StrokeOptions {
                            width: 1.0,
                            ..Default::default()
                        }
                    );
                    path.fill(
                        Style::Paint(
                            Paint::with_radial_gradient(
                                origin,
                                0.0, boss_rad,
                                silver,
                                darksilver,
                            )
                        ),
                        Default::default()
                    );
                },
                PathOptions {
                    composite_operation: CompositeOperation::Basic(BasicCompositeOperation::SourceOver),
                    alpha: 1.0,
                    transform: Some(Transform::new().translate(dial_center.0, dial_center.1)),
                    ..Default::default()
                },
            );

        });

        gl_window.swap_buffers().unwrap();
    }
}

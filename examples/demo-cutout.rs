extern crate gl;
extern crate glutin;
extern crate nanovg;
extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::time::Instant;
use glutin::GlContext;
use nanovg::{Color, Frame, Solidity, Winding, Transform, PathOptions, StrokeOptions};
use rand::{Rng, Rand, thread_rng};
use std::collections::HashMap;
use std::f32::consts::PI;

const INIT_WINDOW_SIZE: (u32, u32) = (1200, 825);

lazy_static! {
    static ref COLORS: [Color; 4] = [
        Color::from_rgb(0x00, 0xBF, 0xA8),
        Color::from_rgb(0x99, 0x66, 0xFF),
        Color::from_rgb(0xFF, 0x64, 0x64),
        Color::from_rgb(0x00, 0xC8, 0xFF)
    ];
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("NanoVG Cutout")
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

    let mut rng = thread_rng();
    let mut running = true;
    let mut mouse = (0.0f32, 0.0f32);
    let mut smoothed_mouse = (0.0f32, 0.0f32);
    let mut prev_time = 0.0;
    let start_time = Instant::now();
    let mut shapes = ShapeCache::new();

    loop {
        let elapsed = get_elapsed(&start_time);
        let delta_time = elapsed - prev_time;
        prev_time = elapsed;

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

        smoothed_mouse = smooth_mouse(mouse, smoothed_mouse, delta_time, 7.0);

        context.frame((width, height), gl_window.hidpi_factor(), |frame| {
            let (width, height) = (width as f32, height as f32);
            let block_size = 75.0;
            let offset = block_size / 2.0;

            // background
            render_rectangle(&frame, (0.0, 0.0), (width, height), Color::from_rgb(0xFF, 0xFF, 0xAF));

            let max_cols = (width / block_size) as u16 + 2;
            let max_rows = (height / block_size) as u16 + 2;

            for x in 0..max_cols {
                for y in 0..max_rows {
                    let mut shape = shapes.get((x, y), &mut rng);
                    shape.update(delta_time);
                    let x = x as f32 * block_size - offset;
                    let y = y as f32 * block_size - offset;
                    shape.draw(&frame, (x, y), block_size);
                }
            }

            render_cutout(&frame, (0.0, 0.0), (width, height), smoothed_mouse);
        });

        gl_window.swap_buffers().unwrap();
    }
}

fn smooth_mouse(mouse: (f32, f32), prev_smoothed_mouse: (f32, f32), dt: f32, speed: f32) -> (f32, f32) {
    let smx = lerp(prev_smoothed_mouse.0, mouse.0, dt * speed);
    let smy = lerp(prev_smoothed_mouse.1, mouse.1, dt * speed);
    (smx, smy)
}

/// Holds Shapes and generates them as needed.
struct ShapeCache(HashMap<u32, Shape>);

impl ShapeCache {
    /// Creates new ShapeCache
    fn new() -> ShapeCache {
        ShapeCache(HashMap::new())
    }

    /// Get shape with position stored in 'pair'.
    /// If Shape at this position does not exists, it gets created and then returned,
    /// Otherwise, if Shape at this position exists, it gets returned.
    fn get<T: Rng>(&mut self, pair: (u16, u16), rng: &mut T) -> &mut Shape {
        let index = ShapeCache::elegent_pair(pair);
        self.0.entry(index).or_insert_with(|| Shape::new(rng))
    }

    /// Pairs position into one bigger integer.
    fn elegent_pair((x, y): (u16, u16)) -> u32 {
        let a = x as u32;
        let b = y as u32;

        if a >= b {
            a * a + a + b
        }
        else {
            a + b * b
        }
    }
}

/// All possible kinds of shapes that we can draw.
enum ShapeKind {
    /// Polygon with number of sides to draw.
    /// Should be bigger or equal to 3.
    Polygon(u8),
    /// Squiggle with number of peaks to draw.
    Squiggle(u8),
}

/// Implements rand for ShapeKind, so we can easily
/// generate random kinds of shapes with their random properties.
impl Rand for ShapeKind {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 2) {
            0 => ShapeKind::Polygon(rng.gen_range(3, 6)),
            1 => ShapeKind::Squiggle(rng.gen_range(3, 6)),
            _ => unreachable!(),
        }
    }
}

/// Shape struct holding data for rendering given shape.
struct Shape {
    /// Current rotation in radians.
    rotation: f32,
    // Current speed of rotation, can be negative (rotates in opposite direction)
    speed: f32,
    /// Color of the shape.
    color: Color,
    /// Describes how to render shape.
    kind: ShapeKind,
}

impl Shape {
    /// Creates new random shape.
    fn new<T: Rng>(rng: &mut T) -> Shape {
        let color = rng.choose(&*COLORS).unwrap();
        let direction = rng.choose(&[-1.0f32, 1.0f32]).unwrap();

        Shape {
            rotation: rng.gen_range(0.0, 2.0 * PI),
            speed: rng.gen_range(1.0, 4.0) * direction,
            color: *color,
            kind: ShapeKind::rand(rng),
        }
    }

    /// Updates properties of this shape.
    fn update(&mut self, dt: f32) {
        self.rotation = self.rotation + dt * self.speed;
    }

    // Draws shape onto 'frame' at specified position, with available square area with side length of 'size'.
    fn draw(&self, frame: &Frame, (x, y): (f32, f32), size: f32) {
        let margin = size * 0.2;
        let x = x + margin;
        let y = y + margin;
        let size = size - margin * 2.0;
        let half_size = size / 2.0;
        let pos = (x + half_size, y + half_size);
        match self.kind {
            ShapeKind::Polygon(sides) => Shape::render_polygon(frame, pos, size, self.rotation, self.color, sides),
            ShapeKind::Squiggle(phi) => Shape::render_squiggle(frame, pos, (size, size / 3.0), self.rotation, self.color, phi),
        };
    }

    /// Renders polygon at center '(cx, cy)' that is inside circle of diameter 'diameter'.
    /// 'color' The colors of the shape.
    /// 'rotation' Specifies the rotation of shape in radians.
    /// 'num_sides' Number of polygon sides.
    fn render_polygon(frame: &Frame, (cx, cy): (f32, f32), diameter: f32, rotation: f32, color: Color, num_sides: u8) {
        assert!(num_sides >= 3);

        let radius = diameter / 2.0;
        let num_sides = num_sides as u32;

        frame.path(
            |path| {
                path.move_to((Shape::get_polygon_point(0, num_sides, radius)));
                for i in 1..num_sides {
                    path.line_to(Shape::get_polygon_point(i, num_sides, radius));
                }
                path.close();
                path.fill(color, Default::default());
            },
            PathOptions {
                transform: Some(Transform::new().with_translation(cx, cy).rotate(rotation)),
                ..Default::default()
            }
        );
    }

    /// Renders squiggly line at center '(cx, cy)' that is inside rectangle '(w, h)'.
    /// 'color' The colors of the shape.
    /// 'rotation' Specifies the rotation of shape in radians.
    /// 'phi' Specifies number of peaks (oscillations).
    fn render_squiggle(frame: &Frame, (cx, cy): (f32, f32), (w, h): (f32, f32), rotation: f32, color: Color, phi: u8) {
        let phi = phi as f32;
        let mut points = [(0.0, 0.0); 64];
        for i in 0..points.len() {
            let pct = i as f32 / (points.len() as f32 - 1.0);
            let theta = pct * PI * 2.0 * phi + PI / 2.0;
            let sx = w * pct - w / 2.0;
            let sy = h / 2.0 * theta.sin();
            points[i as usize] = (sx, sy);
        }

        frame.path(
            |path| {
                path.move_to(points[0]);
                for point in points.iter().skip(1) {
                    path.line_to(*point);
                }
                path.stroke(
                    color,
                    StrokeOptions {
                        width: 3.0,
                        ..Default::default()
                    }
                );
            },
            PathOptions {
                transform: Some(Transform::new().with_translation(cx, cy).rotate(rotation)),
                ..Default::default()
            }
        );
    }

    /// Get coordinate of point in polygon at index.
    /// 'index' The index of point in polygon.
    /// 'num_sides' Specifies how many sides does polygon have.
    /// 'radius' The radius of polygon.
    fn get_polygon_point(index: u32, num_sides: u32, radius: f32) -> (f32, f32) {
        let px = radius * (2.0 * PI * index as f32 / num_sides as f32).cos();
        let py = radius * (2.0 * PI * index as f32 / num_sides as f32).sin();
        (px, py)
    }
}

/// Linearly interpolates between 'from' to 'to' by 't'.
fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

/// Returns elased time starting from 'instant'.
fn get_elapsed(instant: &Instant) -> f32 {
    let elapsed = instant.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    elapsed as f32
}

/// Renders foreground with hole cut in it.
fn render_cutout(frame: &Frame, (x, y): (f32, f32), (w, h): (f32, f32), (mx, my): (f32, f32)) {
    let base_circle_size = 200.0;
    let circle_thickness = 25.0;
    frame.path(
        |path| {
            path.rect((x, y), (w, h));
            path.move_to((0.0, 0.0));
            path.circle((mx, my), base_circle_size);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.close();
            path.fill(Color::from_rgba(255, 255, 255, 255), Default::default());
        },
        Default::default()
    );

    frame.path(
        |path| {
            path.move_to((0.0, 0.0));
            path.circle((mx, my), base_circle_size + circle_thickness);
            path.circle((mx, my), base_circle_size);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.close();
            path.fill(Color::from_rgba(90, 94, 100, 25), Default::default());
        },
        Default::default()
    );

    frame.path(
        |path| {
            path.move_to((0.0, 0.0));
            path.circle((mx, my), base_circle_size);
            path.circle((mx, my), base_circle_size - circle_thickness);
            path.winding(Winding::Solidity(Solidity::Hole));
            path.close();

            path.fill(Color::from_rgba(0, 0, 0, 25), Default::default());
        },
        Default::default()
    );
}

/// Renders rectangle on position with specified dimensions and color.
fn render_rectangle(frame: &Frame, (x, y): (f32, f32), (w, h): (f32, f32), color: Color) {
    frame.path(
        |path| {
            path.rect((x, y), (w, h));
            path.fill(color, Default::default());
        },
        Default::default()
    );
}

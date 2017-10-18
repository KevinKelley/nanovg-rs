#[macro_use]
extern crate bitflags;
extern crate libc;

pub mod ffi;

use std::ops::Drop;
use libc::{c_int, c_float, c_uchar};

#[cfg(any(feature = "gl2", feature = "gl3", feature = "gles2", feature = "gles3"))]
pub struct CreateFlags {
    flags: ffi::NVGcreateFlags,
}

#[cfg(any(feature = "gl2", feature = "gl3", feature = "gles2", feature = "gles3"))]
impl CreateFlags {
    pub fn new() -> Self {
        CreateFlags {
            flags: ffi::NVGcreateFlags::empty(),
        }
    }

    pub fn antialias(mut self) -> Self {
        self.flags |= ffi::NVGcreateFlags::NVG_ANTIALIAS;
        self
    }

    pub fn stencil_strokes(mut self) -> Self {
        self.flags |= ffi::NVGcreateFlags::NVG_STENCIL_STROKES;
        self
    }

    pub fn debug(mut self) -> Self {
        self.flags |= ffi::NVGcreateFlags::NVG_DEBUG;
        self
    }

    fn bits(&self) -> c_int {
        self.flags.bits()
    }
}

pub struct Context(*mut ffi::NVGcontext);

impl Context {
    pub fn raw(&self) -> *mut ffi::NVGcontext {
        self.0
    }

    #[cfg(feature = "gl3")]
    pub fn with_gl3(flags: CreateFlags) -> Result<Self, ()> {
        let raw = unsafe { ffi::nvgCreateGL3(flags.bits()) };
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }

    #[cfg(feature = "gl2")]
    pub fn with_gl2(flags: CreateFlags) -> Result<Self, ()> {
        let raw = unsafe { ffi::nvgCreateGL2(flags.bits()) };
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }

    #[cfg(feature = "gles3")]
    pub fn with_gles3(flags: CreateFlags) -> Result<Self, ()> {
        let raw = unsafe { ffi::nvgCreateGLES3(flags.bits()) };
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }

    #[cfg(feature = "gles2")]
    pub fn with_gles2(flags: CreateFlags) -> Result<Self, ()> {
        let raw = unsafe { ffi::nvgCreateGLES2(flags.bits()) };
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }

    pub fn frame<F: FnOnce(Frame)>(&self, (width, height): (i32, i32), device_pixel_ratio: f32, handler: F) {
        unsafe { ffi::nvgBeginFrame(self.raw(), width as c_int, height as c_int, device_pixel_ratio as c_float);  }
        {
            let frame = Frame::new(self);
            handler(frame);
        }
        unsafe { ffi::nvgEndFrame(self.raw()); }
    }

    pub fn global_composite_operation(&self, operation: CompositeOperation) {
        let ctx = self.raw();
        match operation {
            CompositeOperation::Basic(basic) => unsafe {
                ffi::nvgGlobalCompositeOperation(ctx, basic.into_raw() as c_int);
            },
            CompositeOperation::BlendFunc { source: src, destination: dst } => unsafe {
                ffi::nvgGlobalCompositeBlendFunc(ctx, src.into_raw().bits(), dst.into_raw().bits());
            },
            CompositeOperation::BlendFuncSeparate { rgb_source: rs, rgb_destination: rd, alpha_source: als, alpha_destination: ald } => unsafe {
                let (rs, rd, als, ald) = (rs.into_raw().bits(), rd.into_raw().bits(), als.into_raw().bits(), ald.into_raw().bits());
                ffi::nvgGlobalCompositeBlendFuncSeparate(ctx, rs, rd, als, ald);
            }
        }
    }

    pub fn global_alpha(&self, alpha: f32) {
        unsafe { ffi::nvgGlobalAlpha(self.raw(), alpha as c_float); }
    }
}

impl Drop for Context {
    #[cfg(feature = "gl3")]
    fn drop(&mut self) {
        unsafe { ffi::nvgDeleteGL3(self.0); }
    }

    #[cfg(feature = "gl2")]
    fn drop(&mut self) {
        unsafe { ffi::nvgDeleteGL2(self.0); }
    }

    #[cfg(feature = "gles3")]
    fn drop(&mut self) {
        unsafe { ffi::nvgDeleteGLES3(self.0); }
    }

    #[cfg(feature = "gles2")]
    fn drop(&mut self) {
        unsafe { ffi::nvgDeleteGLES2(self.0); }
    }

    #[cfg(not(any(feature = "gl3", feature = "gl2", feature = "gles3", feature = "gles2")))]
    fn drop(&mut self) {

    }
}

pub struct FrameOptions {

}

pub struct Frame<'a> {
    context: &'a Context,
}

impl<'a> Frame<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context,
        }
    }

    pub fn path<F: FnOnce(Path)>(&self, handler: F) {
        unsafe { ffi::nvgBeginPath(self.context.raw()); }
        handler(Path::new(self));
    }
}

pub struct Path<'a, 'b>
where
    'b: 'a
{
    frame: &'a Frame<'b>,
}

impl<'a, 'b> Path<'a, 'b> {
    fn new(frame: &'a Frame<'b>) -> Self {
        Self {
            frame,
        }
    }

    fn ctx(&self) -> *mut ffi::NVGcontext {
        self.frame.context.raw()
    }

    pub fn fill(&self, style: FillStyle) {
        let ctx = self.ctx();
        unsafe {
            ffi::nvgShapeAntiAlias(ctx, style.antialias as c_int);
            match style.coloring_style {
                ColoringStyle::Color(color) => ffi::nvgFillColor(ctx, color.into_raw()),
                ColoringStyle::Paint(paint) => ffi::nvgFillPaint(ctx, paint.into_raw()),
            }
            ffi::nvgFill(ctx);
        }
    }

    pub fn stroke(&self, style: StrokeStyle) {
        let ctx = self.ctx();
        unsafe {
            ffi::nvgShapeAntiAlias(ctx, style.antialias as c_int);
            match style.coloring_style {
                ColoringStyle::Color(color) => ffi::nvgStrokeColor(ctx, color.into_raw()),
                ColoringStyle::Paint(paint) => ffi::nvgStrokePaint(ctx, paint.into_raw()),
            }
            ffi::nvgStrokeWidth(ctx, style.width as c_float);
            ffi::nvgMiterLimit(ctx, style.miter_limit as c_float);
            ffi::nvgStroke(ctx);
        }
    }

    pub fn arc(&self, (cx, cy): (f32, f32), radius: f32, start_angle: f32, end_angle: f32, direction: Direction) {
        unsafe { ffi::nvgArc(self.ctx(), cx, cy, radius, start_angle, end_angle, direction.into_raw().bits()); }
    }

    pub fn rect(&self, (x, y): (f32, f32), (w, h): (f32, f32)) {
        unsafe { ffi::nvgRect(self.ctx(), x as c_float, y as c_float, w as c_float, h as c_float); }
    }

    pub fn rounded_rect(&self, (x, y): (f32, f32), (w, h): (f32, f32), radius: f32) {
        unsafe { ffi::nvgRoundedRect(self.ctx(), x, y, w, h, radius); }
    }

    /// `top_radii` and `bottom_radii` are both tuples in the form (left, right).
    pub fn rounded_rect_varying(&self, (x, y): (f32, f32), (w, h): (f32, f32), top_radii: (f32, f32), bottom_radii: (f32, f32)) {
        unsafe { ffi::nvgRoundedRectVarying(self.ctx(), x, y, w, h, top_radii.0, top_radii.1, bottom_radii.1, bottom_radii.0); }
    }

    pub fn ellipse(&self, (cx, cy): (f32, f32), radius_x: f32, radius_y: f32) {
        unsafe { ffi::nvgEllipse(self.ctx(), cx, cy, radius_x, radius_y); }
    }

    pub fn circle(&self, (cx, cy): (f32, f32), radius: f32) {
        unsafe { ffi::nvgCircle(self.ctx(), cx, cy, radius); }
    }

    pub fn sub_path<F: FnOnce(SubPath)>(&self, (x, y): (f32, f32), handler: F) {
        let ctx = self.ctx();
        unsafe { ffi::nvgMoveTo(ctx, x, y); }
        handler(SubPath::new(self));
        unsafe { ffi::nvgClosePath(ctx); }
    }
}

pub struct SubPath<'a, 'b, 'c>
where
    'b: 'a,
    'c: 'b,
{
    path: &'a Path<'b, 'c>,
}

impl<'a, 'b, 'c> SubPath<'a, 'b, 'c> {
    fn new(path: &'a Path<'b, 'c>) -> Self {
        Self {
            path,
        }
    }

    fn ctx(&self) -> *mut ffi::NVGcontext {
        self.path.ctx()
    }

    pub fn line_to(&self, (x, y): (f32, f32)) {
        unsafe { ffi::nvgLineTo(self.ctx(), x, y); }
    }

    pub fn cubic_bezier_to(&self, (x, y): (f32, f32), control1: (f32, f32), control2: (f32, f32)) {
        unsafe { ffi::nvgBezierTo(self.ctx(), control1.0, control1.1, control2.0, control2.1, x, y); }
    }

    pub fn quad_bezier_to(&self, (x, y): (f32, f32), control: (f32, f32)) {
        unsafe { ffi::nvgQuadTo(self.ctx(), control.0, control.1, x, y); }
    }

    pub fn arc_to(&self, p1: (f32, f32), p2: (f32, f32), radius: f32) {
        unsafe { ffi::nvgArcTo(self.ctx(), p1.0, p1.1, p2.0, p2.1, radius); }
    }

    pub fn winding(&self, direction: Direction) {
        unsafe { ffi::nvgPathWinding(self.ctx(), direction.into_raw().bits()); }
    }
}

pub struct FillStyle {
    pub coloring_style: ColoringStyle,
    pub antialias: bool,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            coloring_style: ColoringStyle::Color(Color::from_rgb(0, 0, 0)),
            antialias: true,
        }
    }
}

pub struct StrokeStyle {
    pub coloring_style: ColoringStyle,
    pub width: f32,
    pub miter_limit: f32,
    pub antialias: bool,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            coloring_style: ColoringStyle::Color(Color::from_rgb(0, 0, 0)),
            width: 1.0,
            miter_limit: 10.0,
            antialias: true,
        }
    }
}

pub enum ColoringStyle {
    Color(Color),
    Paint(Paint),
}

#[derive(Clone, Copy)]
pub struct Color(ffi::NVGcolor);

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(ffi::NVGcolor {
            rgba: [r, g, b, a],
        })
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color(unsafe { ffi::nvgRGB(r as c_uchar, g as c_uchar, b as c_uchar) })
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(unsafe { ffi::nvgRGBA(r as c_uchar, g as c_uchar, b as c_uchar, a as c_uchar) })
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Color(unsafe { ffi::nvgHSL(h as c_float, s as c_float, l as c_float) })
    }

    pub fn from_hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        Color(unsafe { ffi::nvgHSLA(h as c_float, s as c_float, l as c_float, a as c_uchar) })
    }

    fn into_raw(self) -> ffi::NVGcolor {
        self.0
    }

    pub fn red(&self) -> f32 {
        self.0.rgba[0]
    }
    pub fn green(&self) -> f32 {
        self.0.rgba[1]
    }
    pub fn blue(&self) -> f32 {
        self.0.rgba[2]
    }
    pub fn alpha(&self) -> f32 {
        self.0.rgba[3]
    }

    pub fn set_red(&mut self, red: f32) {
        self.0.rgba[0] = red;
    }
    pub fn set_green(&mut self, green: f32) {
        self.0.rgba[1] = green;
    }
    pub fn set_blue(&mut self, blue: f32) {
        self.0.rgba[2] = blue;
    }
    pub fn set_alpha(&mut self, alpha: f32) {
        self.0.rgba[3] = alpha;
    }

    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color(unsafe { ffi::nvgLerpRGBA(a.into_raw(), b.into_raw(), t as c_float) })
    }
}

#[derive(Copy, Clone)]
pub struct Paint(ffi::NVGpaint);

impl Paint {
    fn into_raw(self) -> ffi::NVGpaint {
        self.0
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    fn into_raw(self) -> ffi::NVGwinding {
        match self {
            Direction::Clockwise => ffi::NVGwinding::NVG_CW,
            Direction::CounterClockwise => ffi::NVGwinding::NVG_CCW,
        }
    }
}

#[derive(Copy, Clone)]
pub enum CompositeOperation {
    Basic(BasicCompositeOperation),
    BlendFunc {
        source: BlendFactor,
        destination: BlendFactor,
    },
    BlendFuncSeparate {
        rgb_source: BlendFactor,
        rgb_destination: BlendFactor,
        alpha_source: BlendFactor,
        alpha_destination: BlendFactor,
    }
}

#[derive(Copy, Clone)]
pub enum BasicCompositeOperation {
    SourceOver,
    SourceIn,
    SourceOut,
    Atop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Lighter,
    Copy,
    Xor,
}

impl BasicCompositeOperation {
    fn into_raw(self) -> ffi::NVGcompositeOperation {
        use BasicCompositeOperation::*;
        use ffi::NVGcompositeOperation::*;
        match self {
            SourceOver => NVG_SOURCE_OVER,
            SourceIn => NVG_SOURCE_IN,
            SourceOut => NVG_SOURCE_OUT,
            Atop => NVG_ATOP,
            DestinationOver => NVG_DESTINATION_OVER,
            DestinationIn => NVG_DESTINATION_IN,
            DestinationOut => NVG_DESTINATION_OUT,
            DestinationAtop => NVG_DESTINATION_ATOP,
            Lighter => NVG_LIGHTER,
            Copy => NVG_COPY,
            Xor => NVG_XOR,
        }
    }
}

#[derive(Copy, Clone)]
pub enum BlendFactor {
    Zero,
    One,
    SourceColor,
    OneMinusSourceColor,
    DestinationColor,
    OneMinusDestinationColor,
    SourceAlpha,
    OneMinusSourceAlpha,
    DestinationAlpha,
    OneMinusDestinationAlpha,
    SourceAlphaSaturate,
}

impl BlendFactor {
    fn into_raw(self) -> ffi::NVGblendFactor {
        use BlendFactor::*;
        match self {
            Zero => ffi::NVGblendFactor::NVG_ZERO,
            One => ffi::NVGblendFactor::NVG_ONE,
            SourceColor => ffi::NVGblendFactor::NVG_SRC_COLOR,
            OneMinusSourceColor => ffi::NVGblendFactor::NVG_ONE_MINUS_SRC_COLOR,
            DestinationColor => ffi::NVGblendFactor::NVG_DST_COLOR,
            OneMinusDestinationColor => ffi::NVGblendFactor::NVG_ONE_MINUS_DST_COLOR,
            SourceAlpha => ffi::NVGblendFactor::NVG_SRC_ALPHA,
            OneMinusSourceAlpha => ffi::NVGblendFactor::NVG_ONE_MINUS_SRC_ALPHA,
            DestinationAlpha => ffi::NVGblendFactor::NVG_DST_ALPHA,
            OneMinusDestinationAlpha => ffi::NVGblendFactor::NVG_ONE_MINUS_DST_ALPHA,
            SourceAlphaSaturate => ffi::NVGblendFactor::NVG_SRC_ALPHA_SATURATE,
        }
    }
}
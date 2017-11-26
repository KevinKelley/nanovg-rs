#[macro_use]
extern crate bitflags;
extern crate libc;

pub mod ffi;

use std::ops::Drop;
use std::path::Path as IoPath;
use std::ffi::{NulError, CString};
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

#[cfg(target_os = "windows")]
fn init_gl() -> Result<(), ()> {
    if unsafe { ffi::gladLoadGL() } == 1 {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(not(target_os = "windows"))]
fn init_gl() -> Result<(), ()> {
    Ok(())
}

pub struct Context(*mut ffi::NVGcontext);

impl Context {
    pub fn raw(&self) -> *mut ffi::NVGcontext {
        self.0
    }

    #[cfg(feature = "gl3")]
    pub fn with_gl3(flags: CreateFlags) -> Result<Self, ()> {
        init_gl()?;
        let raw = unsafe { ffi::nvgCreateGL3(flags.bits()) };
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }

    #[cfg(feature = "gl2")]
    pub fn with_gl2(flags: CreateFlags) -> Result<Self, ()> {
        init_gl()?;
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

    fn global_composite_operation(&self, operation: CompositeOperation) {
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

    fn global_alpha(&self, alpha: f32) {
        unsafe { ffi::nvgGlobalAlpha(self.raw(), alpha as c_float); }
    }

    fn scissor(&self, scissor: Option<Scissor>) {
        if let Some(scissor) = scissor {
            match scissor {
                Scissor::Rect { x, y, width, height } => unsafe {
                    ffi::nvgScissor(self.raw(), x, y, width, height);
                },
                Scissor::Intersect { x, y, width, height } => unsafe {
                    ffi::nvgIntersectScissor(self.raw(), x, y, width, height);
                }
            }
        } else {
            unsafe { ffi::nvgResetScissor(self.raw()); }
        }
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

pub enum Scissor {
    /// Defines a rectangular scissor.
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    /// Define the scissor to be the intersection between the current scissor rectangle
    /// and the specified rectangle.
    /// The current and specified rectangles are always transformed to be in the current transform space.
    Intersect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }
}

/// Provides options to change how a frame renders.
pub struct PathOptions {
    /// The scissor defines the rectangular boundary in which the frame is clipped into.
    pub scissor: Option<Scissor>,
    /// Defines how overlapping paths are composited together.
    pub composite_operation: CompositeOperation,
    /// The alpha component of the path.
    pub alpha: f32,
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            scissor: None,
            composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Atop),
            alpha: 1.0,
        }
    }
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

    pub fn path<F: FnOnce(Path)>(&self, handler: F, options: PathOptions) {
        self.context.global_composite_operation(options.composite_operation);
        self.context.global_alpha(options.alpha);
        self.context.scissor(options.scissor);

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

    pub fn close(&self) {
        unsafe { ffi::nvgClosePath(self.ctx()); }
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
    pub fn with_linear_gradient(context: &Context, start: (f32, f32), end: (f32, f32), start_color: Color, end_color: Color) -> Self {
        let ((sx, sy), (ex, ey)) = (start, end);
        Paint(unsafe { ffi::nvgLinearGradient(context.raw(), sx, sy, ex, ey, start_color.into_raw(), end_color.into_raw()) })
    }

    pub fn with_box_gradient(context: &Context, (x, y): (f32, f32), (w, h): (f32, f32), radius: f32, feather: f32, start_color: Color, end_color: Color) -> Self {
        Paint(unsafe { ffi::nvgBoxGradient(context.raw(), x, y, w, h, radius, feather, start_color.into_raw(), end_color.into_raw()) })
    }

    pub fn with_radial_gradient(context: &Context, center: (f32, f32), inner_radius: f32, outer_radius: f32, start_color: Color, end_color: Color) -> Self {
        let (cx, cy) = center;
        Paint(unsafe { ffi::nvgRadialGradient(context.raw(), cx, cy, inner_radius, outer_radius, start_color.into_raw(), end_color.into_raw()) })
    }

    pub fn with_image_pattern(context: &Context, image: &Image, origin: (f32, f32), size: (f32, f32), angle: f32, alpha: f32) -> Self {
        let ((ox, oy), (ex, ey)) = (origin, size);
        Paint(unsafe { ffi::nvgImagePattern(context.raw(), ox, oy, ex, ey, angle, image.raw(), alpha) })
    }

    fn into_raw(self) -> ffi::NVGpaint {
        self.0
    }
}

pub struct ImageBuilder<'a> {
    context: &'a Context,
    flags: ffi::NVGimageFlags,
}

impl<'a> ImageBuilder<'a> {
    fn new(context: &'a Context) -> Self {
        Self {
            context,
            flags: ffi::NVGimageFlags::empty(),
        }
    }

    /// Create mipmaps during the creation of the image.
    pub fn mipmaps(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_GENERATE_MIPMAPS;
        self
    }

    /// Repeat the image on the X axis.
    pub fn repeat_x(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_REPEATX;
        self
    }

    /// Repeat the image on the Y axis.
    pub fn repeat_y(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_REPEATY;
        self
    }

    /// Flip (invert) the image in the Y direction during rendering.
    pub fn flipy(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_FLIPY;
        self
    }

    /// The image data contains premultiplied alpha.
    pub fn premultiplied(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_PREMULTIPLIED;
        self
    }

    /// Use nearest interpolation instead of linear.
    pub fn nearest(mut self) -> Self {
        self.flags |= ffi::NVGimageFlags::NVG_IMAGE_NEAREST;
        self
    }

    /// Construct the image by loading it from an image file on the file system.
    pub fn build_from_file<P: AsRef<IoPath>>(self, file: P) -> ImageBuilderResult<'a> {
        let path = match file.as_ref().to_str() {
            Some(p) => CString::new(p.to_owned())?,
            None => return Err(ImageBuilderError::PathNotCString),
        };

        let handle = unsafe { ffi::nvgCreateImage(self.context.raw(), (*path).as_ptr(), self.flags.bits()) };
        if handle > 0 {
            Ok(Image(self.context, handle))
        } else {
            Err(ImageBuilderError::CreateImageFailed)
        }
    }

    /// Construct the image by loading it from an image file in memory.
    pub fn build_from_memory(self, data: &[u8]) -> ImageBuilderResult<'a> {
        let handle = unsafe { ffi::nvgCreateImageMem(self.context.raw(), self.flags.bits(), data.as_ptr() as *mut _, data.len() as c_int) };
        if handle > 0 {
            Ok(Image(self.context, handle))
        } else {
            Err(ImageBuilderError::CreateImageFailed)
        }
    }

    /// Construct the image by filling it with pixel data from memory (always 32bit RGBA).
    pub fn build_from_rgba(self, width: usize, height: usize, data: &[u32]) -> ImageBuilderResult<'a> {
        if data.len() < width * height {
            return Err(ImageBuilderError::NotEnoughData);
        }

        let handle = unsafe { ffi::nvgCreateImageRGBA(self.context.raw(), width as c_int, height as c_int, self.flags.bits(), data.as_ptr() as *const _) };
        if handle > 0 {
            Ok(Image(self.context, handle))
        } else {
            Err(ImageBuilderError::CreateImageFailed)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageBuilderError {
    /// The path for `build_from_file` could not be converted to a c-string.
    PathNotCString,
    /// The call to `nvgCreateImage`, or similar functions, failed.
    CreateImageFailed,
    /// For `from_rgba`, the passed data slice does not contain enough data for the specified image size.
    NotEnoughData,
}

impl From<NulError> for ImageBuilderError {
    fn from(_: NulError) -> Self {
        ImageBuilderError::PathNotCString
    }
}

pub type ImageBuilderResult<'a> = Result<Image<'a>, ImageBuilderError>;

/// Handle to an image.
pub struct Image<'a>(&'a Context, c_int);

impl<'a> Image<'a> {
    pub fn new(context: &'a Context) -> ImageBuilder {
        ImageBuilder::new(context)
    }

    pub fn size(&self) -> (usize, usize) {
        let (mut w, mut h): (c_int, c_int) = (0, 0);
        unsafe { ffi::nvgImageSize(self.ctx().raw(), self.raw(), &mut w as *mut _, &mut h as *mut _); }
        (w as usize, h as usize)
    }

    pub fn update(&mut self, data: &[u32]) {
        unsafe { ffi::nvgUpdateImage(self.ctx().raw(), self.raw(), data.as_ptr() as *const _); }
    }

    fn ctx(&self) -> &Context {
        self.0
    }

    fn raw(&self) -> c_int {
        self.1
    }
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        unsafe { ffi::nvgDeleteImage(self.ctx().raw(), self.raw()); }
        self.1 = 0;
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
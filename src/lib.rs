extern crate nanovg_sys as ffi;

use std::ops::Drop;
use std::path::Path as IoPath;
use std::ffi::{NulError, CString};
use std::os::raw::{c_int, c_float, c_uchar, c_char};
use std::{mem, ptr};

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

#[cfg(feature = "gl3")]
fn create_gl(flags: ffi::NVGcreateFlags) -> *mut ffi::NVGcontext {
    unsafe { ffi::nvgCreateGL3(flags.bits()) }
}

#[cfg(feature = "gl2")]
fn create_gl(flags: ffi::NVGcreateFlags) -> *mut ffi::NVGcontext {
    unsafe { ffi::nvgCreateGL2(flags.bits()) }
}

#[cfg(feature = "gles3")]
fn create_gl(flags: ffi::NVGcreateFlags) -> *mut ffi::NVGcontext {
    unsafe { ffi::nvgCreateGLES3(flags.bits()) }
}

#[cfg(feature = "gles2")]
fn create_gl(flags: ffi::NVGcreateFlags) -> *mut ffi::NVGcontext {
    unsafe { ffi::nvgCreateGLES2(flags.bits()) }
}

#[cfg(not(any(feature = "gl3", feature = "gl2", feature = "gles3", feature = "gles2")))]
fn create_gl(flags: ffi::NVGcreateFlags) -> *mut ffi::NVGcontext {
    panic!("Unable to determine the backend / implementation. Have you enabled one of the features?")
}

/// A builder that configures and constructs a NanoVG context.
#[derive(Debug)]
pub struct ContextBuilder {
    flags: ffi::NVGcreateFlags,
}

impl ContextBuilder {
    /// Create a new context builder.
    pub fn new() -> Self {
        Self {
            flags: ffi::NVGcreateFlags::empty(),
        }
    }

    /// Enable antialiased rasterization. Not needed if you have multisampling enabled.
    pub fn antialias(mut self) -> Self {
        self.flags.insert(ffi::NVGcreateFlags::NVG_ANTIALIAS);
        self
    }

    /// Enable stencil strokes. Overlapping, stroked paths will only be drawn once, for a little performance loss.
    pub fn stencil_strokes(mut self) -> Self {
        self.flags.insert(ffi::NVGcreateFlags::NVG_STENCIL_STROKES);
        self
    }

    /// Enable additional debug checks.
    pub fn debug(mut self) -> Self {
        self.flags.insert(ffi::NVGcreateFlags::NVG_DEBUG);
        self
    }

    /// Construct the context.
    /// Make sure you have enabled one of the 4 OpenGL features, or this function will panic.
    pub fn build(self) -> Result<Context, ()> {
        init_gl()?;
        let raw = create_gl(self.flags);
        if !raw.is_null() {
            Ok(Context(raw))
        } else {
            Err(())
        }
    }
}

/// A initialized NanoVG context - the central type which all operations rely on.
#[derive(Debug)]
pub struct Context(*mut ffi::NVGcontext);

impl Context {
    /// Return the raw FFI C-struct pointer to the context.
    pub fn raw(&self) -> *mut ffi::NVGcontext {
        self.0
    }

    /// Begin drawing a frame.
    /// All NanoVG drawing takes place within a frame.
    ///
    /// `width` and `height` should be the width and height of the framebuffer / window client size.
    /// `device_pixel_ratio` defines the pixel ratio. NanoVG doesn't guess this automatically to allow for Hi-DPI devices.
    /// Basically, this is your hidpi factor.
    /// `handler` is the callback in which you draw your paths. You cannot draw paths outside of this callback.
    pub fn frame<'a, F: FnOnce(Frame<'a>)>(
        &'a self,
        (width, height): (i32, i32),
        device_pixel_ratio: f32,
        handler: F,
    ) {
        unsafe {
            ffi::nvgBeginFrame(
                self.raw(),
                width as c_int,
                height as c_int,
                device_pixel_ratio as c_float,
            );
        }
        {
            let frame = Frame::new(self, Transform::new());
            handler(frame);
        }
        unsafe {
            ffi::nvgEndFrame(self.raw());
        }
    }

    fn global_composite_operation(&self, operation: CompositeOperation) {
        let ctx = self.raw();
        match operation {
            CompositeOperation::Basic(basic) => unsafe {
                ffi::nvgGlobalCompositeOperation(ctx, basic.into_raw() as c_int);
            },
            CompositeOperation::BlendFunc {
                source: src,
                destination: dst,
            } => unsafe {
                ffi::nvgGlobalCompositeBlendFunc(ctx, src.into_raw().bits(), dst.into_raw().bits());
            },
            CompositeOperation::BlendFuncSeparate {
                rgb_source: rs,
                rgb_destination: rd,
                alpha_source: als,
                alpha_destination: ald,
            } => unsafe {
                let (rs, rd, als, ald) = (
                    rs.into_raw().bits(),
                    rd.into_raw().bits(),
                    als.into_raw().bits(),
                    ald.into_raw().bits(),
                );
                ffi::nvgGlobalCompositeBlendFuncSeparate(ctx, rs, rd, als, ald);
            },
        }
    }

    fn global_alpha(&self, alpha: f32) {
        unsafe {
            ffi::nvgGlobalAlpha(self.raw(), alpha as c_float);
        }
    }

    fn scissor(&self, scissor: Option<Scissor>) {
        match scissor {
            Some(ref scissor) =>  {
                self.with_applied_transform(scissor.transform,
                    || unsafe {
                        ffi::nvgScissor(self.raw(), scissor.x, scissor.y, scissor.width, scissor.height);
                    }
                );
            },
            None => unsafe {
                ffi::nvgResetScissor(self.raw());
            }
        }
    }

    fn intersect(&self, intersect: &Intersect) {
        self.scissor(Some(intersect.with));

        self.with_applied_transform(intersect.transform,
            || unsafe {
                ffi::nvgIntersectScissor(self.raw(), intersect.x, intersect.y, intersect.width, intersect.height);
            }
        );
    }

    fn clip(&self, clip: Clip) {
        match clip {
            Clip::Scissor(scissor) => self.scissor(Some(scissor)),
            Clip::Intersect(ref intersect) => self.intersect(intersect),
            Clip::None => (),
        }
    }

    fn transform(&self, transform: Option<Transform>) {
        match transform {
            Some(ref transform) => {
                let t = transform.matrix;
                unsafe { ffi::nvgTransform(self.raw(), t[0], t[1], t[2], t[3], t[4], t[5]); }
            },
            None => unsafe { ffi::nvgResetTransform(self.raw()); }
        }
    }

    fn current_transform(&self) -> Transform {
        let mut current = Transform::new();
        unsafe {
            ffi::nvgCurrentTransform(self.raw(), current.matrix.as_mut_ptr());
        }
        current
    }

    fn with_applied_transform<F: FnOnce()>(&self, transform: Option<Transform>, handler: F) {
        let current = self.current_transform();

        if let Some(transform) = transform {
            if transform.absolute {
                self.transform(None);
            }

            self.transform(Some(transform));
        }

        handler();

        self.transform(None);
        self.transform(Some(current));
    }
}

impl Drop for Context {
    #[cfg(feature = "gl3")]
    fn drop(&mut self) {
        unsafe {
            ffi::nvgDeleteGL3(self.0);
        }
    }

    #[cfg(feature = "gl2")]
    fn drop(&mut self) {
        unsafe {
            ffi::nvgDeleteGL2(self.0);
        }
    }

    #[cfg(feature = "gles3")]
    fn drop(&mut self) {
        unsafe {
            ffi::nvgDeleteGLES3(self.0);
        }
    }

    #[cfg(feature = "gles2")]
    fn drop(&mut self) {
        unsafe {
            ffi::nvgDeleteGLES2(self.0);
        }
    }

    #[cfg(not(any(feature = "gl3", feature = "gl2", feature = "gles3", feature = "gles2")))]
    fn drop(&mut self) {}
}

/// A scissor defines a region on the screen in which drawing operations are allowed.
/// Pixels drawn outside of this region are clipped.
#[derive(Clone, Copy, Debug)]
pub struct Scissor {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub transform: Option<Transform>
}

/// Define intersection scissor which gets intersected with 'with' Scissor.
/// Pixels drawn outside of this intersection are clipped.
/// When 'with' Scissor or this Intersection have rotation, the intersection will be an approximation.
#[derive(Clone, Copy, Debug)]
pub struct Intersect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub with: Scissor,
    pub transform: Option<Transform>
}

/// Define how to clip specified region.
#[derive(Clone, Copy, Debug)]
pub enum Clip {
    Scissor(Scissor),
    Intersect(Intersect),
    None,
}

/// Options which control how a path is rendered.
#[derive(Clone, Copy, Debug)]
pub struct PathOptions {
    /// The clip defines the rectangular region in which the frame is clipped into.
    /// All overflowing pixels will be discarded.
    pub clip: Clip,
    /// Defines how overlapping paths are composited together.
    pub composite_operation: CompositeOperation,
    /// The alpha component of the path.
    pub alpha: f32,
    /// A transformation which 'transforms' the coordinate system and consequently the path.
    pub transform: Option<Transform>,
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            clip: Clip::None,
            composite_operation: CompositeOperation::Basic(BasicCompositeOperation::Atop),
            alpha: 1.0,
            transform: None,
        }
    }
}

/// A frame which can draw paths.
/// All NanoVG path drawing operations are done on a frame.
#[derive(Debug)]
pub struct Frame<'a> {
    context: &'a Context,
    transform: Transform,
}

impl<'a> Frame<'a> {
    fn new(context: &'a Context, transform: Transform) -> Self {
        Self { context, transform }
    }

    /// Get the underlying context this frame was created on.
    pub fn context(&self) -> &'a Context {
        self.context
    }

    /// Get current transform which the frame is transformed by.
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// Transform current Frame by 'transform' and
    /// call 'handler' with transformed Frame as its only parameter.
    /// You can get the passed transform by calling get_transform on Frame instance.
    ///
    /// `transform` frame gets transformed by this Transform (it takes previous frame transform into account)
    /// `handler` the callback where you use the new transformed Frame
    pub fn transformed<'b, F: FnOnce(Frame<'b>)>(&'b mut self, transform: Transform, handler: F) {
        let frame = Frame::new(self.context, transform * self.transform);
        handler(frame);
    }

    /// Draw a new path.
    ///
    /// `handler` is the callback in which you operate the path.
    /// `options` control how the path is rendered.
    pub fn path<F: FnOnce(Path)>(&self, handler: F, options: PathOptions) {
        self.context.global_composite_operation(options.composite_operation);
        self.context.global_alpha(options.alpha);

        self.draw_prepare(options.clip, options.transform);

        unsafe { ffi::nvgBeginPath(self.context.raw()); }
        handler(Path::new(self));
    }

    fn draw_prepare(&self, clip: Clip, transform: Option<Transform>) {
        self.context.transform(None);
        self.context.scissor(None);
        self.context.transform(Some(self.transform));

        self.context.clip(clip);

        if let Some(transform) = transform {
            if transform.absolute {
                self.context.transform(None);
            }
            self.context.transform(Some(transform));
        }
    }

    fn text_prepare(&self, font: Font, options: TextOptions) {
        unsafe {
            ffi::nvgFontFaceId(self.context.raw(), font.id());
            ffi::nvgFillColor(self.context.raw(), options.color.into_raw());
            ffi::nvgFontSize(self.context.raw(), options.size);
            ffi::nvgFontBlur(self.context.raw(), options.blur);
            ffi::nvgTextLetterSpacing(self.context.raw(), options.letter_spacing);
            ffi::nvgTextLineHeight(self.context.raw(), options.line_height);
            ffi::nvgTextAlign(self.context.raw(), options.align.into_raw().bits());
        }
    }

    /// Draw a single line on the screen. Newline characters are ignored.
    /// `font` the font face to use.
    /// `(x, y)` the origin / position to draw the text at. The origin is relative to the alignment of `options`.
    /// `text` the string to draw.
    /// `options` optional (`Default::default`) options that control the visual appearance of the text.
    pub fn text<S: AsRef<str>>(
        &self,
        font: Font,
        (x, y): (f32, f32),
        text: S,
        options: TextOptions,
    ) {
        let text = CString::new(text.as_ref()).unwrap();
        self.text_prepare(font, options);

        self.draw_prepare(options.clip, options.transform);

        unsafe {
            ffi::nvgText(self.context.raw(), x, y, text.as_ptr(), ptr::null());
        }
    }

    /// Draw multiline text on the screen.
    /// `font` the font face to use.
    /// `(x, y)` the origin / position to draw the text at. The origin is relative to the alignment of `options`.
    /// `text` the string to draw.
    /// `options` optional (`Default::default`) options that control the visual appearance of the text.
    pub fn text_box<S: AsRef<str>>(
        &self,
        font: Font,
        (x, y): (f32, f32),
        text: S,
        options: TextOptions,
    ) {
        let text = CString::new(text.as_ref()).unwrap();
        self.text_prepare(font, options);

        self.draw_prepare(options.clip, options.transform);

        unsafe {
            ffi::nvgTextBox(
                self.context.raw(),
                x,
                y,
                options.line_max_width,
                text.as_ptr(),
                ptr::null(),
            );
        }
    }

    /// Measures specified text string.
    /// Returns tuple (f32, TextBounds) where the first element specifies horizontal advance of measured text
    /// and the second element specifies the bounding box of measured text.
    /// `font` the font face to use.
    /// `(x, y)` the origin / position to measure the text from.
    /// `text` the string to measure.
    /// `options` optional (`Default::default`) options that controls how the text is measured.
    pub fn text_bounds<S: AsRef<str>>(
        &self,
        font: Font,
        (x, y): (f32, f32),
        text: S,
        options: TextOptions,
    ) -> (f32, TextBounds) {
        let text = CString::new(text.as_ref()).unwrap();
        self.text_prepare(font, options);
        let mut bounds = [0.0f32; 4];
        let measure = unsafe {
            ffi::nvgTextBounds(
                self.context.raw(),
                x,
                y,
                text.as_ptr(),
                ptr::null(),
                bounds.as_mut_ptr(),
            )
        };
        (measure, TextBounds::new(&bounds))
    }

    /// Measures specified multi-text string.
    /// Returns bounding box of measured multi-text.
    /// `font` the font face to use.
    /// `(x, y)` the origin / position to measure the text from.
    /// `text` the string to measure.
    /// `options` optional (`Default::default`) options that controls how the text is measured.
    pub fn text_box_bounds<S: AsRef<str>>(
        &self,
        font: Font,
        (x, y): (f32, f32),
        text: S,
        options: TextOptions,
    ) -> TextBounds {
        let text = CString::new(text.as_ref()).unwrap();
        self.text_prepare(font, options);
        let mut bounds = [0.0f32; 4];
        unsafe {
            ffi::nvgTextBoxBounds(
                self.context.raw(),
                x,
                y,
                options.line_max_width,
                text.as_ptr(),
                ptr::null(),
                bounds.as_mut_ptr(),
            );
        }
        TextBounds::new(&bounds)
    }

    /// Calculates and breaks text into series of glyph positions.
    /// Returns iterator over all glyph positions in text.
    /// `(x, y)` the coordinate space from which to offset coordinates in `GlyphPosition`
    /// `text` the text to break into glyph positions
    pub fn text_glyph_positions<S: AsRef<str>>(
        &self,
        (x, y): (f32, f32),
        text: S,
    ) -> TextGlyphPositions {
        TextGlyphPositions::new(
            self.context,
            x,
            y,
            CString::new(text.as_ref()).unwrap()
        )
    }

    /// Returns vertical text metrics based on given font and text options
    /// Measured values are stored in TextMetrics struct in local coordinate space.
    /// `options` the options specify how metrics should be calculated.
    /// `font` the font for which to calculate metrics.
    pub fn text_metrics(&self, font: Font, options: TextOptions) -> TextMetrics {
        self.text_prepare(font, options);
        let mut metrics = TextMetrics::new();
        unsafe {
            ffi::nvgTextMetrics(
                self.context.raw(),
                &mut metrics.ascender,
                &mut metrics.descender,
                &mut metrics.line_height
            );
        }
        metrics
    }

    /// Breaks text into lines.
    /// Text is split at word boundaries, new-line character or when row width exceeds break_row_width.
    /// Returns iterator over text lines.
    /// `text` the text to break into lines
    /// `break_row_width` maximum width of row
    pub fn text_break_lines<S: AsRef<str>>(
        &self,
        text: S,
        break_row_width: f32,
    ) -> TextBreakLines {
        TextBreakLines::new(
            self.context,
            CString::new(text.as_ref()).unwrap(),
            break_row_width
        )
    }
}

/// A path, the main type for most NanoVG drawing operations.
#[derive(Debug)]
pub struct Path<'a, 'b>
where
    'b: 'a,
{
    frame: &'a Frame<'b>,
}

impl<'a, 'b> Path<'a, 'b> {
    fn new(frame: &'a Frame<'b>) -> Self {
        Self { frame }
    }

    fn ctx(&self) -> *mut ffi::NVGcontext {
        self.frame.context.raw()
    }

    /// Get the underlying context this path was created on.
    pub fn context(&self) -> &'a Context {
        self.frame.context()
    }

    /// Draw the current path by filling in it's shape.
    /// 'paint' specifies in which color/paint should fill be drawn.
    ///         pass variables that implement Paint trait
    ///         for now these are: Color, Gradient, ImagePattern
    /// 'options' specifies how filling should be done.
    pub fn fill<T: Paint>(&self, paint: T, options: FillOptions) {
        let ctx = self.ctx();
        unsafe {
            ffi::nvgShapeAntiAlias(ctx, options.antialias as c_int);
            paint.fill(self.context());
            ffi::nvgFill(ctx);
        }
    }

    /// Draw the current path by stroking it's perimeter.
    /// 'paint' specifies in which color/paint should stroke be drawn.
    ///         pass variables that implement Paint trait
    ///         for now these are: Color, Gradient, ImagePattern
    /// 'options' specifies how stroking should be done.
    pub fn stroke<T: Paint>(&self, paint: T, options: StrokeOptions) {
        let ctx = self.ctx();
        unsafe {
            ffi::nvgShapeAntiAlias(ctx, options.antialias as c_int);
            ffi::nvgStrokeWidth(ctx, options.width as c_float);
            ffi::nvgLineCap(ctx, options.line_cap.into_raw() as c_int);
            ffi::nvgLineJoin(ctx, options.line_join.into_raw() as c_int);
            ffi::nvgMiterLimit(ctx, options.miter_limit as c_float);
            paint.stroke(self.context());
            ffi::nvgStroke(ctx);
        }
    }

    /// Add an arc to the path.
    pub fn arc(
        &self,
        (cx, cy): (f32, f32),
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        winding: Winding,
    ) {
        unsafe {
            ffi::nvgArc(
                self.ctx(),
                cx,
                cy,
                radius,
                start_angle,
                end_angle,
                winding.into_raw(),
            );
        }
    }

    /// Add a rectangle to the path.
    pub fn rect(&self, (x, y): (f32, f32), (w, h): (f32, f32)) {
        unsafe {
            ffi::nvgRect(
                self.ctx(),
                x as c_float,
                y as c_float,
                w as c_float,
                h as c_float,
            );
        }
    }

    /// Add a rounded rectangle to the path.
    pub fn rounded_rect(&self, (x, y): (f32, f32), (w, h): (f32, f32), radius: f32) {
        unsafe {
            ffi::nvgRoundedRect(self.ctx(), x, y, w, h, radius);
        }
    }

    /// Add a rounded rectangle with varying corners to the path.
    /// `top_radii` and `bottom_radii` are both tuples in the form (left, right).
    pub fn rounded_rect_varying(
        &self,
        (x, y): (f32, f32),
        (w, h): (f32, f32),
        top_radii: (f32, f32),
        bottom_radii: (f32, f32),
    ) {
        unsafe {
            ffi::nvgRoundedRectVarying(
                self.ctx(),
                x,
                y,
                w,
                h,
                top_radii.0,
                top_radii.1,
                bottom_radii.1,
                bottom_radii.0,
            );
        }
    }

    /// Add an ellipse to the path.
    pub fn ellipse(&self, (cx, cy): (f32, f32), radius_x: f32, radius_y: f32) {
        unsafe {
            ffi::nvgEllipse(self.ctx(), cx, cy, radius_x, radius_y);
        }
    }

    /// Add a circle to the path.
    pub fn circle(&self, (cx, cy): (f32, f32), radius: f32) {
        unsafe {
            ffi::nvgCircle(self.ctx(), cx, cy, radius);
        }
    }

    /// Add a line to the subpath.
    pub fn line_to(&self, (x, y): (f32, f32)) {
        unsafe {
            ffi::nvgLineTo(self.ctx(), x, y);
        }
    }

    /// Add a cubic bezier curve to the subpath.
    pub fn cubic_bezier_to(&self, (x, y): (f32, f32), control1: (f32, f32), control2: (f32, f32)) {
        unsafe {
            ffi::nvgBezierTo(
                self.ctx(),
                control1.0,
                control1.1,
                control2.0,
                control2.1,
                x,
                y,
            );
        }
    }

    /// Add a quadratic bezier curve to the subpath.
    pub fn quad_bezier_to(&self, (x, y): (f32, f32), control: (f32, f32)) {
        unsafe {
            ffi::nvgQuadTo(self.ctx(), control.0, control.1, x, y);
        }
    }

    /// Add a arc to the subpath.
    pub fn arc_to(&self, p1: (f32, f32), p2: (f32, f32), radius: f32) {
        unsafe {
            ffi::nvgArcTo(self.ctx(), p1.0, p1.1, p2.0, p2.1, radius);
        }
    }

    /// Set the winding of the subpath.
    /// The winding defines which parts of the subpath are 'inside' and which are 'outside'.
    pub fn winding(&self, winding: Winding) {
        unsafe {
            ffi::nvgPathWinding(self.ctx(), winding.into_raw());
        }
    }

    /// Start new sub-path with specified coordinates as the first point.
    pub fn move_to(&self, (x, y): (f32, f32)) {
        unsafe {
            ffi::nvgMoveTo(self.ctx(), x, y);
        }
    }

    /// Close the path, ie. connect the first point and last point with a line.
    pub fn close(&self) {
        unsafe {
            ffi::nvgClosePath(self.ctx());
        }
    }
}

/// Controls how filling in a path should look.
#[derive(Debug)]
pub struct FillOptions {
    pub antialias: bool,
}

impl Default for FillOptions {
    fn default() -> Self {
        Self {
            antialias: true,
        }
    }
}

/// Controls how stroking a path should look.
#[derive(Debug)]
pub struct StrokeOptions {
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub antialias: bool,
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self {
            width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            antialias: true,
        }
    }
}

/// Controls how the end of line is drawn.
#[derive(Clone, Copy, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl LineCap {
    fn into_raw(self) -> ffi::NVGlineCap {
        match self {
            LineCap::Butt => ffi::NVGlineCap::NVG_BUTT,
            LineCap::Round => ffi::NVGlineCap::NVG_ROUND,
            LineCap::Square => ffi::NVGlineCap::NVG_SQUARE,
        }
    }
}

/// Controls how lines are joined together.
#[derive(Clone, Copy, Debug)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel
}

impl LineJoin {
    fn into_raw(self) -> ffi::NVGlineCap {
        match self {
            LineJoin::Miter => ffi::NVGlineCap::NVG_MITER,
            LineJoin::Round => ffi::NVGlineCap::NVG_ROUND,
            LineJoin::Bevel => ffi::NVGlineCap::NVG_BEVEL,
        }
    }
}

pub trait Paint {
    fn fill(&self, context: &Context);
    fn stroke(&self, context: &Context);
}

/// A 32-bit color value.
/// Used to fill or stroke paths with solid color.
#[derive(Clone, Copy, Debug)]
pub struct Color(ffi::NVGcolor);

impl Color {
    /// Create a new color by setting all components manually.
    /// Values are in the range 0.0...1.0.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(ffi::NVGcolor { rgba: [r, g, b, a] })
    }

    /// Create a new color from three 8-bit color channels.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color(unsafe {
            ffi::nvgRGB(r as c_uchar, g as c_uchar, b as c_uchar)
        })
    }

    /// Create a new color from three 8-bit color channels and an 8-bit alpha channel.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(unsafe {
            ffi::nvgRGBA(r as c_uchar, g as c_uchar, b as c_uchar, a as c_uchar)
        })
    }

    /// Create a new color from three hsl channels.
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Color(unsafe {
            ffi::nvgHSL(h as c_float, s as c_float, l as c_float)
        })
    }

    /// Create a new color from three hsl channels and an 8-bit alpha channel.
    pub fn from_hsla(h: f32, s: f32, l: f32, a: u8) -> Self {
        Color(unsafe {
            ffi::nvgHSLA(h as c_float, s as c_float, l as c_float, a as c_uchar)
        })
    }

    fn into_raw(self) -> ffi::NVGcolor {
        self.0
    }

    /// Get the red component.
    pub fn red(&self) -> f32 {
        self.0.rgba[0]
    }

    /// Get the green component.
    pub fn green(&self) -> f32 {
        self.0.rgba[1]
    }

    /// Get the blue component.
    pub fn blue(&self) -> f32 {
        self.0.rgba[2]
    }

    /// Get the alpha component.
    pub fn alpha(&self) -> f32 {
        self.0.rgba[3]
    }

    /// Set the red component.
    pub fn set_red(&mut self, red: f32) {
        self.0.rgba[0] = red;
    }

    /// Get the green component.
    pub fn set_green(&mut self, green: f32) {
        self.0.rgba[1] = green;
    }

    /// Get the blue component.
    pub fn set_blue(&mut self, blue: f32) {
        self.0.rgba[2] = blue;
    }

    /// Get the alpha component.
    pub fn set_alpha(&mut self, alpha: f32) {
        self.0.rgba[3] = alpha;
    }

    /// Create a new color by linearly interpolating between two existing colors.
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color(unsafe {
            ffi::nvgLerpRGBA(a.into_raw(), b.into_raw(), t as c_float)
        })
    }
}

impl Paint for Color {
    fn fill(&self, context: &Context) {
        unsafe {
            ffi::nvgFillColor(context.raw(), self.into_raw());
        }
    }

    fn stroke(&self, context: &Context) {
        unsafe {
            ffi::nvgStrokeColor(context.raw(), self.into_raw());
        }
    }
}

impl Paint for Gradient {
    fn fill(&self, context: &Context) {
        let raw = self.create_raw();
        unsafe {
            ffi::nvgFillPaint(context.raw(), raw);
        }
    }

    fn stroke(&self, context: &Context) {
        let raw = self.create_raw();
        unsafe {
            ffi::nvgStrokePaint(context.raw(), raw);
        }
    }
}

/// Gradient paint used to fill or stroke paths with gradient.
#[derive(Copy, Clone, Debug)]
pub enum Gradient {
    Linear {
        start: (f32, f32),
        end: (f32, f32),
        start_color: Color,
        end_color: Color,
    },
    Box {
        position: (f32, f32),
        size: (f32, f32),
        radius: f32,
        feather: f32,
        start_color: Color,
        end_color: Color,
    },
    Radial {
        center: (f32, f32),
        inner_radius: f32,
        outer_radius: f32,
        start_color: Color,
        end_color: Color,
    }
}

impl Gradient {
    fn create_raw(&self) -> ffi::NVGpaint {
        match self {
            &Gradient::Linear {
                start,
                end,
                start_color,
                end_color,
            } => {
                let (sx, sy) = start;
                let (ex, ey) = end;
                unsafe {
                    ffi::nvgLinearGradient(
                        ptr::null_mut(),
                        sx,
                        sy,
                        ex,
                        ey,
                        start_color.into_raw(),
                        end_color.into_raw(),
                    )
                }
            },
            &Gradient::Box {
                position,
                size,
                radius,
                feather,
                start_color,
                end_color,
            } => {
                unsafe {
                    let (x, y) = position;
                    let (w, h) = size;
                    ffi::nvgBoxGradient(
                        ptr::null_mut(),
                        x,
                        y,
                        w,
                        h,
                        radius,
                        feather,
                        start_color.into_raw(),
                        end_color.into_raw(),
                    )
                }
            },
            &Gradient::Radial {
                center,
                inner_radius,
                outer_radius,
                start_color,
                end_color,
            } => {
                unsafe {
                    let (cx, cy) = center;
                    ffi::nvgRadialGradient(
                        ptr::null_mut(),
                        cx,
                        cy,
                        inner_radius,
                        outer_radius,
                        start_color.into_raw(),
                        end_color.into_raw(),
                    )
                }
            },
        }
    }
}

/// Image pattern paint used to fill or stroke paths with image pattern.
#[derive(Copy, Clone, Debug)]
pub struct ImagePattern<'a> {
    pub image: &'a Image<'a>,
    pub origin: (f32, f32),
    pub size: (f32, f32),
    pub angle: f32,
    pub alpha: f32,
}

impl<'a> ImagePattern<'a> {
    fn create_raw(&self) -> ffi::NVGpaint {
        let (ox, oy) = self.origin;
        let (ex, ey) = self.size;
        unsafe {
            ffi::nvgImagePattern(ptr::null_mut(), ox, oy, ex, ey, self.angle, self.image.raw(), self.alpha)
        }
    }
}

impl<'a> Paint for ImagePattern<'a> {
    fn fill(&self, context: &Context) {
        let raw = self.create_raw();
        unsafe {
            ffi::nvgFillPaint(context.raw(), raw);
        }
    }

    fn stroke(&self, context: &Context) {
        let raw = self.create_raw();
        unsafe {
            ffi::nvgStrokePaint(context.raw(), raw);
        }
    }
}

#[derive(Debug)]
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

    /// Get the underlying context this ImageBuilder was created on.
    pub fn context(&self) -> &'a Context {
        self.context
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
            None => return Err(ImageBuilderError::CStringError),
        };

        let handle =
            unsafe { ffi::nvgCreateImage(self.context.raw(), (*path).as_ptr(), self.flags.bits()) };
        if handle > 0 {
            Ok(Image(self.context, handle))
        } else {
            Err(ImageBuilderError::CreateImageFailed)
        }
    }

    /// Construct the image by loading it from an image file in memory.
    pub fn build_from_memory(self, data: &[u8]) -> ImageBuilderResult<'a> {
        let handle = unsafe {
            ffi::nvgCreateImageMem(
                self.context.raw(),
                self.flags.bits(),
                data.as_ptr() as *mut _,
                data.len() as c_int,
            )
        };
        if handle > 0 {
            Ok(Image(self.context, handle))
        } else {
            Err(ImageBuilderError::CreateImageFailed)
        }
    }

    /// Construct the image by filling it with pixel data from memory (always 32bit RGBA).
    pub fn build_from_rgba(
        self,
        width: usize,
        height: usize,
        data: &[u32],
    ) -> ImageBuilderResult<'a> {
        if data.len() < width * height {
            return Err(ImageBuilderError::NotEnoughData);
        }

        let handle = unsafe {
            ffi::nvgCreateImageRGBA(
                self.context.raw(),
                width as c_int,
                height as c_int,
                self.flags.bits(),
                data.as_ptr() as *const _,
            )
        };
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
    CStringError,
    /// The call to `nvgCreateImage`, or similar functions, failed.
    CreateImageFailed,
    /// For `from_rgba`, the passed data slice does not contain enough data for the specified image size.
    NotEnoughData,
}

impl From<NulError> for ImageBuilderError {
    fn from(_: NulError) -> Self {
        ImageBuilderError::CStringError
    }
}

pub type ImageBuilderResult<'a> = Result<Image<'a>, ImageBuilderError>;

/// Handle to an image.
#[derive(Debug)]
pub struct Image<'a>(&'a Context, c_int);

impl<'a> Image<'a> {
    pub fn new(context: &'a Context) -> ImageBuilder {
        ImageBuilder::new(context)
    }

    /// Get the underlying context this image was created on.
    pub fn context(&self) -> &'a Context {
        self.0
    }

    pub fn size(&self) -> (usize, usize) {
        let (mut w, mut h): (c_int, c_int) = (0, 0);
        unsafe {
            ffi::nvgImageSize(
                self.ctx().raw(),
                self.raw(),
                &mut w as *mut _,
                &mut h as *mut _,
            );
        }
        (w as usize, h as usize)
    }

    pub fn update(&mut self, data: &[u32]) {
        unsafe {
            ffi::nvgUpdateImage(self.ctx().raw(), self.raw(), data.as_ptr() as *const _);
        }
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
        unsafe {
            ffi::nvgDeleteImage(self.ctx().raw(), self.raw());
        }
        self.1 = 0;
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum Solidity {
    Hole,
    Solid,
}

impl Solidity {
    fn into_raw(self) -> ffi::NVGsolidity {
        match self {
            Solidity::Hole => ffi::NVGsolidity::NVG_HOLE,
            Solidity::Solid => ffi::NVGsolidity::NVG_SOLID,
        }
    }
}

/// Winding enum that holds either Direction or Solidity enum
/// These two are identical aliases.
/// They are here for different meanings in different contexts
#[derive(Debug)]
pub enum Winding {
    Direction(Direction),
    Solidity(Solidity),
}

impl Winding {
    fn into_raw(self) -> c_int {
        match self {
            Winding::Direction(direction) => direction.into_raw().bits(),
            Winding::Solidity(solidity) => solidity.into_raw().bits(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
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
    },
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

/// A handle to a font.
/// Fonts are managed by the NanoVG context itself. View this type only as a 'reference' to a font.
#[derive(Clone, Copy, Debug)]
pub struct Font<'a>(&'a Context, c_int);

#[derive(Debug)]
pub enum CreateFontError {
    /// Conversion from a Rust-utf8-string to a CString failed.
    CStringError,
    /// A specified path is invalid somehow.
    InvalidPath,
    /// The font handle returned by the ffi functions is invalid.
    InvalidHandle,
}

impl From<NulError> for CreateFontError {
    fn from(_: NulError) -> Self {
        CreateFontError::CStringError
    }
}

pub type CreateFontResult<'a> = Result<Font<'a>, CreateFontError>;

impl<'a> Font<'a> {
    fn ctx(&self) -> *mut ffi::NVGcontext {
        self.0.raw()
    }

    fn id(&self) -> c_int {
        self.1
    }

    /// Get the underlying context this font was created on.
    pub fn context(&self) -> &'a Context {
        self.0
    }

    /// Attempt to load a font from the file at `path`.
    /// Fonts are always named (specified with `name`).
    pub fn from_file<S: AsRef<str>, P: AsRef<IoPath>>(
        context: &'a Context,
        name: S,
        path: P,
    ) -> CreateFontResult {
        let name = CString::new(name.as_ref())?;
        let path = CString::new(path.as_ref().to_str().ok_or(CreateFontError::InvalidPath)?)?;
        let handle = unsafe { 
            ffi::nvgCreateFont(context.raw(), name.as_ptr(), path.as_ptr())
        };
        if handle > ffi::FONS_INVALID {
            Ok(Font(context, handle))
        } else {
            Err(CreateFontError::InvalidHandle)
        }
    }

    /// Attempt to load a font from memory.
    /// Fonts are always named (specified with `name`).
    pub fn from_memory<'b, S: AsRef<str>>(
        context: &'a Context,
        name: S,
        memory: &'b [u8],
    ) -> CreateFontResult<'a> {
        let name = CString::new(name.as_ref())?;
        let handle = unsafe {
            ffi::nvgCreateFontMem(
                context.raw(),
                name.as_ptr(),
                memory.as_ptr() as *mut _,
                memory.len() as c_int,
                0,
            )         
        };
        if handle > ffi::FONS_INVALID {
            Ok(Font(context, handle))
        } else {
            Err(CreateFontError::InvalidHandle)
        }
    }

    /// Try to find a already loaded font with the given `name`.
    pub fn find<S: AsRef<str>>(context: &'a Context, name: S) -> CreateFontResult {
        let name = CString::new(name.as_ref())?;
        let handle = unsafe { 
            ffi::nvgFindFont(context.raw(), name.as_ptr())
        };
        if handle > ffi::FONS_INVALID {
            Ok(Font(context, handle))
        } else {
            Err(CreateFontError::InvalidHandle)
        }
    }

    /// Add `fallback` as a fallback for the current font.
    /// If the font renderer fails to rasterize a glyph with the main font, it will automatically
    /// attempt to rasterize the same glyph with the fallback font.
    /// This process continues until no working font is found, then the glyph is skipped.
    pub fn add_fallback(&self, fallback: Font) -> bool {
        let res = unsafe { ffi::nvgAddFallbackFontId(self.ctx(), self.id(), fallback.id()) };
        res != 0
    }
}

/// Options which control the visual appearance of a text.
#[derive(Clone, Copy, Debug)]
pub struct TextOptions {
    /// The size of the text in points.
    pub size: f32,
    /// The radial blur of the text, in pixels.
    pub blur: f32,
    /// How much each individual letter of the text should be apart.
    pub letter_spacing: f32,
    /// The height for each line. Specified in multiplies of the font height.
    /// Ex.: a `line_height` of 3.0 means each line is font height * 3 apart.
    pub line_height: f32,
    /// The width at which multiline text is automatically wrapped.
    pub line_max_width: f32,
    /// How to align the text.
    pub align: Alignment,
    /// The fill color of the text.
    pub color: Color,
    /// The clip defines the rectangular region in which the text is clipped into.
    /// All overflowing pixels will be discarded.
    pub clip: Clip,
    /// A transformation which 'transforms' the coordinate system and consequently the text.
    pub transform: Option<Transform>,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            size: 12.0,
            blur: 0.0,
            letter_spacing: 0.0,
            line_height: 1.0,
            line_max_width: std::f32::MAX,
            align: Alignment::new(),
            color: Color::new(0.0, 0.0, 0.0, 0.0),
            clip: Clip::None,
            transform: None,
        }
    }
}

/// Struct to store min and max bounds when measuring text with text_bounds or text_box_bounds
#[derive(Clone, Copy, Debug)]
pub struct TextBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl TextBounds {
    /// Creates new TextBounds struct instance from array
    fn new(bounds: &[f32; 4]) -> TextBounds {
        TextBounds {
            min_x: bounds[0],
            min_y: bounds[1],
            max_x: bounds[2],
            max_y: bounds[3],
        }
    }
}

/// Iterator over text glyph positions, calculated by Context::text_glyph_positions
pub struct TextGlyphPositions<'a> {
    context: &'a Context,
    x: f32,
    y: f32,
    start: *const c_char,
    glyphs: [ffi::NVGglyphPosition; 2],
}

impl<'a> TextGlyphPositions<'a> {
    /// Creates new TextGlyphPositions iterator with needed variables for iterating over glyphs in text
    fn new(context: &'a Context, x: f32, y: f32, text: CString) -> TextGlyphPositions<'a> {
        TextGlyphPositions {
            context: context,
            x: x,
            y: y,
            start: text.into_raw(),
            glyphs: [unsafe { mem::zeroed() }; 2]
        }
    }
}

impl<'a> Iterator for TextGlyphPositions<'a> {
    type Item = GlyphPosition;

    /// Returns next glyph in text
    fn next(&mut self) -> Option<Self::Item> {
        let num_glyphs = unsafe {
             ffi::nvgTextGlyphPositions(
                self.context.raw(),
                self.x,
                self.y,
                self.start,
                ptr::null(),
                self.glyphs.as_mut_ptr(),
                2
            )
        };

        match num_glyphs {
            1 => {
                self.start = &('\0' as c_char);
                Some(GlyphPosition::new(&self.glyphs[0], None))
            },
            2 => {
                self.x = self.glyphs[1].x;
                self.start = self.glyphs[1].s;

                Some(
                    GlyphPosition::new(
                        &self.glyphs[0],
                        Some(Box::new(
                                GlyphPosition::new(
                                    &self.glyphs[1],
                                    None
                                )
                            )
                        )
                    )
                )
            },
            _ => None
        }
    }
}

/// Holds computed values for given row.
#[derive(Clone, Copy, Debug)]
pub struct TextRow<'a> {
    pub width: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub text: &'a str,
}

impl<'a> TextRow<'a> {
    /// Creates new TextRow from raw nanovg text row
    /// and also adds text contained in this row.
    fn new(row: &ffi::NVGtextRow, text: &'a str) -> TextRow<'a> {
        TextRow {
            width: row.width,
            min_x: row.minx,
            max_x: row.maxx,
            text: text,
        }
    }
}

/// Iterator over rows in text
/// Returned by Context::text_break_lines
#[derive(Debug)]
pub struct TextBreakLines<'a> {
    context: &'a Context,
    start: *const c_char,
    break_row_width: f32,
    row: ffi::NVGtextRow,
}

impl<'a> TextBreakLines<'a> {
    /// Creates new TextBreakLines iterator which iterated over all text rows in text.
    /// break_row_width specifies max length of row.
    fn new(context: &'a Context, text: CString, break_row_width: f32) -> TextBreakLines<'a> {
        TextBreakLines {
            context: context,
            start: text.into_raw(),
            break_row_width: break_row_width,
            row: unsafe { mem::zeroed() },
        }
    }
}

impl<'a> Iterator for TextBreakLines<'a> {
    type Item = TextRow<'a>;

    /// Returns next row in text
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let nrows = ffi::nvgTextBreakLines(self.context.raw(), self.start, ptr::null(), self.break_row_width, &mut self.row, 1);
            self.start = self.row.next;

            if nrows > 0 {
                let string_length = self.row.end as usize - self.row.start as usize;
                let string_slice = std::slice::from_raw_parts(self.row.start as *const u8, string_length);
                let text_str = std::str::from_utf8(string_slice).unwrap();
                Some(TextRow::new(&self.row, text_str))
            } else {
                None
            }
        }
    }
}

// Stores position of glyph returned by iterator Context::text_glyph_positions
#[derive(Clone, Debug)]
pub struct GlyphPosition {
    pub x: f32,
    pub min_x: f32,
    pub max_x: f32,
    /// Next GlyphPosition for convenience (stores only one glyph position in advance)
    pub next: Option<Box<GlyphPosition>>,
}

impl GlyphPosition {
    /// Creates new GlyphPosition from raw nanovg glyph position.
    /// We can optionally pass next glyph position
    /// (there is usually some if it is not the last glyph in text, otherwise it is none for last glyph).
    fn new(glyph: &ffi::NVGglyphPosition, next: Option<Box<GlyphPosition>>) -> GlyphPosition {
        GlyphPosition {
            x: glyph.x,
            min_x: glyph.minx,
            max_x: glyph.maxx,
            next: next
        }
    }
}

/// Struct to store measured text metrics computed with Context::text_metrics
#[derive(Clone, Copy, Debug)]
pub struct TextMetrics {
    pub ascender: f32,
    pub descender: f32,
    pub line_height: f32,
}

impl TextMetrics {
    fn new() -> TextMetrics {
        TextMetrics {
            ascender: 0.0,
            descender: 0.0,
            line_height: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Alignment(ffi::NVGalign);

impl Alignment {
    fn into_raw(self) -> ffi::NVGalign {
        self.0
    }

    /// Create a new top-left alignment.
    pub fn new() -> Self {
        Alignment(ffi::NVGalign::empty()).top().left()
    }

    /// Set the horizontal alignment to left.
    pub fn left(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_RIGHT);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_CENTER);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_LEFT);
        self
    }

    /// Set the horizontal alignment to center.
    pub fn center(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_LEFT);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_RIGHT);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_CENTER);
        self
    }

    /// Set the horizontal alignment to right.
    pub fn right(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_LEFT);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_CENTER);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_RIGHT);
        self
    }

    /// Set the vertical alignment to top.
    pub fn top(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BOTTOM);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_MIDDLE);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BASELINE);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_TOP);
        self
    }

    /// Set the vertical alignment to middle.
    pub fn middle(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_TOP);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BOTTOM);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BASELINE);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_MIDDLE);
        self
    }

    /// Set the vertical alignment to bottom.
    pub fn bottom(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_TOP);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_MIDDLE);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BASELINE);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_BOTTOM);
        self
    }

    /// Set the vertical alignment to baseline.
    pub fn baseline(mut self) -> Self {
        self.0.remove(ffi::NVGalign::NVG_ALIGN_TOP);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_MIDDLE);
        self.0.remove(ffi::NVGalign::NVG_ALIGN_BOTTOM);
        self.0.insert(ffi::NVGalign::NVG_ALIGN_BASELINE);
        self
    }
}

/// Represents a transformation in 2D space.
/// A transformation is a column-major matrix in the following form:
/// [a c e] - indices [0 2 4]
/// [b d f] - indices [1 3 5]
/// [0 0 1] - not passed.
/// The last row however is not specified; it is always [0 0 1] behind the scenes.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub matrix: [f32; 6],
    /// Controls whether paths or texts that gets transformed by this Transform
    /// are drawn in absolute coordinate space or coordinate space relative to the one
    /// previously active (relative positioning is default)
    /// This is just flag to tell drawing functions to use this Transform for drawing,
    /// it does not modify the underlying matrix.
    absolute: bool,
}

impl Transform {
    /// Construct a new transform with an identity matrix.
    pub fn new() -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            absolute: false,
        }
    }

    /// Set flag on this transform to use it in absolute coordinate space.
    pub fn absolute(mut self) -> Self {
        self.absolute = true;
        self
    }

    /// Set flag on this transform to use it in local (relative) coordinate space.
    pub fn relative(mut self) -> Self {
        self.absolute = false;
        self
    }

    /// Set the translation of the transform.
    pub fn with_translation(self, x: f32, y: f32) -> Self {
        let mut new = self.clone();
        new.matrix[4] = x;
        new.matrix[5] = y;
        new
    }

    /// Set the scale of the transform.
    pub fn with_scale(self, x: f32, y: f32) -> Self {
        let mut new = self.clone();
        new.matrix[0] = x;
        new.matrix[3] = y;
        new
    }

    /// Set the skew of the transform.
    pub fn with_skew(self, x: f32, y: f32) -> Self {
        let mut new = self.clone();
        new.matrix[2] = x;
        new.matrix[1] = y;
        new
    }

    /// Set the rotation of the transform.
    pub fn with_rotation(self, theta: f32) -> Self {
        let mut new = self.clone();
        new.matrix[0] = theta.cos();
        new.matrix[2] = -theta.sin();
        new.matrix[1] = theta.sin();
        new.matrix[3] = theta.cos();
        new
    }

    /// Translate transform by x and y.
    pub fn translate(self, x: f32, y: f32) -> Self {
        let mut new = self.clone();
        let mut t = [0.0f32; 6];
        unsafe {
            ffi::nvgTransformTranslate(t.as_mut_ptr(), x, y);
            ffi::nvgTransformPremultiply(new.matrix.as_mut_ptr(), t.as_mut_ptr());
        }
        new
    }

    /// Rotate transform with spcified angle.
    pub fn rotate(self, angle: f32) -> Self {
        let mut new = self.clone();
        let mut t = [0.0f32; 6];
        unsafe {
            ffi::nvgTransformRotate(t.as_mut_ptr(), angle);
            ffi::nvgTransformPremultiply(new.matrix.as_mut_ptr(), t.as_mut_ptr());
        }
        new
    }

    /// Skew transform along x axis with specified angle.
    pub fn skew_x(self, angle: f32) -> Self {
        let mut new = self.clone();
        let mut t = [0.0f32; 6];
        unsafe {
            ffi::nvgTransformSkewX(t.as_mut_ptr(), angle);
            ffi::nvgTransformPremultiply(new.matrix.as_mut_ptr(), t.as_mut_ptr());
        }
        new
    }

    /// Skew transform along y axis with specified angle.
    pub fn skew_y(self, angle: f32) -> Self {
        let mut new = self.clone();
        let mut t = [0.0f32; 6];
        unsafe {
            ffi::nvgTransformSkewY(t.as_mut_ptr(), angle);
            ffi::nvgTransformPremultiply(new.matrix.as_mut_ptr(), t.as_mut_ptr());
        }
        new
    }

    /// Scale transform along x and y.
    pub fn scale(self, x: f32, y: f32) -> Self {
        let mut new = self.clone();
        let mut t = [0.0f32; 6];
        unsafe {
            ffi::nvgTransformScale(t.as_mut_ptr(), x, y);
            ffi::nvgTransformPremultiply(new.matrix.as_mut_ptr(), t.as_mut_ptr());
        }
        new
    }

    /// Transforms a point with this transform.
    /// Returns transformed point (x, y).
    pub fn transform_point(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let mut transformed = (0.0f32, 0.0f32);
        unsafe {
            ffi::nvgTransformPoint(&mut transformed.0, &mut transformed.1, self.matrix.as_ptr(), x, y);
        }
        transformed
    }

    /// Inverses this transform.
    /// Returns inversed copy or None if inversion fails.
    pub fn try_inverse(&self) -> Option<Transform> {
        let mut inv = Transform::new();
        let result = unsafe {
            ffi::nvgTransformInverse(inv.matrix.as_mut_ptr(), self.matrix.as_ptr())
        };

        if result == 1 {
            Some(inv)
        }
        else {
            None
        }
    }
}

/// Implementation of multiplication Trait for Transform.
/// The order in which you multiplicate matters (you are multiplicating matrices)
impl std::ops::Mul for Transform {
    type Output = Transform;

    /// Multiplies transform with other transform (the order matters).
    fn mul(self, rhs: Transform) -> Self::Output {
        let mut result = self.clone();
        unsafe {
            ffi::nvgTransformMultiply(result.matrix.as_mut_ptr(), rhs.matrix.as_ptr());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! trans_eq_bool {
        ($t1:expr, $t2:expr) => {
            f32_eq!($t1.matrix[0], $t2.matrix[0]) &&
            f32_eq!($t1.matrix[1], $t2.matrix[1]) &&
            f32_eq!($t1.matrix[2], $t2.matrix[2]) &&
            f32_eq!($t1.matrix[3], $t2.matrix[3]) &&
            f32_eq!($t1.matrix[4], $t2.matrix[4]) &&
            f32_eq!($t1.matrix[5], $t2.matrix[5])
        };
    }

    macro_rules! trans_eq {
        ($t1:expr, $t2:expr) => {
            assert!(trans_eq_bool!($t1, $t2))
        };
    }

    macro_rules! trans_not_eq {
        ($t1:expr, $t2:expr) => {
            assert!(!trans_eq_bool!($t1, $t2))
        };
    }

    #[test]
    fn test_transform() {
        // Contructors
        trans_eq!(Transform::new(), Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        });

        trans_eq!(Transform::new().with_translation(11.1, 22.2), Transform {
            matrix: [1.0, 0.0, 0.0, 1.0, 11.1, 22.2],
        });

        trans_eq!(Transform::new().with_scale(11.1, 22.2), Transform {
            matrix: [11.1, 0.0, 0.0, 22.2, 0.0, 0.0],
        });

        trans_eq!(Transform::new().with_skew(11.1, 22.2), Transform {
            matrix: [1.0, 22.2, 11.1, 1.0, 0.0, 0.0],
        });

        let angle = 90f32.to_radians();
        trans_eq!(Transform::new().with_rotation(angle), Transform {
            matrix: [angle.cos(), angle.sin(), -angle.sin(), angle.cos(), 0.0, 0.0],
        });

        // Multiplication
        let identity = Transform::new();
        let trans = Transform::new().with_translation(10.0, 20.0);
        trans_eq!(identity * trans, trans);
        trans_eq!(trans * identity, trans);
        trans_eq!(identity * identity, identity);
        let a = Transform::new().with_rotation(123.0);
        let b = Transform::new().with_skew(66.6, 1337.2);
        trans_not_eq!(a * b, b * a);
    }
}

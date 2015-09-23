#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

#![feature(const_fn, optin_builtin_traits, convert)]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_qualifications)]
#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(dead_code)]
//#![warn(missing_doc)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_results)]

#[macro_use]
extern crate bitflags;
extern crate libc;

use std::fmt;
use std::ptr;
use std::str;
use std::path::Path;
use std::ffi::CString;

use libc::{c_char, c_int, c_void, c_float};

use ffi::NVGcolor;
use ffi::NVGpaint;
use ffi::NVGglyphPosition;
use ffi::NVGtextRow;

mod ffi;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum Winding {
    CCW                     = ffi::NVG_CCW,
    CW                      = ffi::NVG_CW,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum Solidity {
    SOLID                   = ffi::NVG_SOLID,
    HOLE                    = ffi::NVG_HOLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum LineCap {
    BUTT                    = ffi::NVG_BUTT,
    ROUND                   = ffi::NVG_ROUND,
    SQUARE                  = ffi::NVG_SQUARE,
    BEVEL                   = ffi::NVG_BEVEL,
    MITER                   = ffi::NVG_MITER,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum PatternRepeat {
    NOREPEAT                = ffi::NVG_NOREPEAT,
    REPEATX                 = ffi::NVG_REPEATX,
    REPEATY                 = ffi::NVG_REPEATY,
}

bitflags!{
    flags Align: u32 {
        const LEFT         = ffi::NVG_ALIGN_LEFT,
        const CENTER       = ffi::NVG_ALIGN_CENTER,
        const RIGHT        = ffi::NVG_ALIGN_RIGHT,
        const TOP          = ffi::NVG_ALIGN_TOP,
        const MIDDLE       = ffi::NVG_ALIGN_MIDDLE,
        const BOTTOM       = ffi::NVG_ALIGN_BOTTOM,
        const BASELINE     = ffi::NVG_ALIGN_BASELINE
    }
}

bitflags!{
    flags CreationFlags: u32 {
        const ANTIALIAS        = ffi::NVG_ANTIALIAS,
        const STENCIL_STROKES  = ffi::NVG_STENCIL_STROKES
    }
}

bitflags!{
    flags ImageFlags: u32 {
        const GENERATE_MIPMAPS = ffi::NVG_IMAGE_GENERATE_MIPMAPS
    }
}

// Color

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    nvg: NVGcolor
}

impl Color {
    #[inline]
    fn wrap(nvg: NVGcolor) -> Color { Color { nvg: nvg } }

    #[inline]
    pub fn r(&self) -> f32 { self.nvg.r }

    #[inline]
    pub fn g(&self) -> f32 { self.nvg.g }

    #[inline]
    pub fn b(&self) -> f32 { self.nvg.b }

    #[inline]
    pub fn a(&self) -> f32 { self.nvg.a }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::wrap(unsafe { ffi::nvgRGB(r, g, b) })
    }
    pub const fn rgb_f(r: f32, g: f32, b: f32) -> Color {
        Color { nvg: ffi::NVGcolor { r: r, g: g, b: b, a: 1.0 } }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::wrap(unsafe { ffi::nvgRGBA(r, g, b, a) })
    }
    pub const fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { nvg: ffi::NVGcolor { r: r, g: g, b: b, a: a } }
    }
    pub fn lerp_rgba(c0: Color, c1: Color, u: f32) -> Color {
        Color::wrap(unsafe { ffi::nvgLerpRGBA(c0.nvg, c1.nvg, u) })
    }
    pub fn trans_rgba(c0: Color, a: u8) -> Color {
        Color::wrap(unsafe { ffi::nvgTransRGBA(c0.nvg, a) })
    }
    pub fn trans_rgba_f(c0: Color, a: f32) -> Color {
        Color::wrap(unsafe { ffi::nvgTransRGBAf(c0.nvg, a) })
    }
    pub fn hsl(h: f32, s: f32, l: f32) -> Color {
        Color::wrap(unsafe { ffi::nvgHSL(h, s, l) })
    }
    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Color {
        Color::wrap(unsafe { ffi:: nvgHSLA(h, s, l, a) })
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r:{}, g:{}, b:{}, a:{}", self.r(), self.g(), self.b(), self.a())
    }
}

// Paint

pub struct Paint {
    nvg: NVGpaint
}

impl Paint {
    #[inline]
    fn wrap(nvg: NVGpaint) -> Paint { Paint { nvg: nvg } }
}

impl fmt::Debug for Paint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p: *const NVGpaint = &self.nvg;
        write!(f, "Paint @ {:?}", p)
    }
}

// Image

pub struct Image {
    handle: c_int
}

impl Image {
    #[inline]
    fn wrap(handle: c_int) -> Image { Image { handle: handle } }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image #{}", self.handle)
    }
}

//impl Drop for Image {
//    fn drop(&mut self) {
//        Context::delete_image(nvg, self.handle);
//        self.handle = ffi::STB_IMAGE_INVALID;
//    }
//}

// Font

pub struct Font {
    handle: c_int
}

impl Font {
    #[inline]
    fn wrap(handle: c_int) -> Font { Font { handle: handle } }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font #{}", self.handle)
    }
}

//impl Drop for Font {
//    fn drop(&mut self) {
//        // seems there's no API in nanovg for unloading fonts!
//        self.handle = ffi::FONS_INVALID;
//    }
//}

// TextRow

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TextRow {
    start_index: usize,
    end_index: usize,
    next_index: usize,
    width: c_float,
    minx: c_float,
    maxx: c_float,
}

impl TextRow {
    pub fn start_index(&self) -> usize { self.start_index }
    pub fn end_index(&self) -> usize { self.end_index }
    pub fn next_index(&self) -> usize { self.next_index }
    pub fn width(&self) -> f32 { self.width }
    pub fn minx(&self) -> f32 { self.minx }
    pub fn maxx(&self) -> f32 { self.maxx }
}

// GlyphPosition

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GlyphPosition {
    byte_index: usize,   // start index of this glyph in string
    x: f32,             // glyph's x position
    minx: f32,          // glyph spans from minx to max x
    maxx: f32           // (span may or may not actually contain x, depending on the font)
}

impl GlyphPosition {
    pub fn byte_index(&self) -> usize { self.byte_index }
    pub fn x(&self) -> f32 { self.x }
    pub fn minx(&self) -> f32 { self.minx }
    pub fn maxx(&self) -> f32 { self.maxx }
}

// Transform

#[derive(Clone, Copy)]
pub struct Transform {
    array: [f32; 6]
}

impl fmt::Debug for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transform(tx: {}, ty: {}, sx: {}, sy: {}, kx: {}, ky: {})",
               self.e(), self.f(), self.a(), self.d(), self.c(), self.b())
    }
}

//macro_rules! accessors(
//    ($($name:ident -> $idx:expr),+) => (
//        $(#[inline] pub fn $name(&self) -> f32 { self.array[$idx] })+
//    )
//)

//macro_rules! mutators(
//    ($($name:ident ($($p:ident : $t:ty),+) via $d:ident),+) => (
//        $(
//        pub fn $name(mut self, $($p:$t),+) -> Transform {
//            let mut t = Transform::new_zero();
//            t.$d($($p),+);
//            self.set_premultiply(&t);
//            self
//        }
//        )+
//    )
//)

impl Transform {
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut f32 { self.array.as_mut_ptr() }

    #[inline]
    fn as_ptr(&self) -> *const f32 { self.array.as_ptr() }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [f32] { &mut self.array }

    #[inline]
    pub fn into_array(self) -> [f32; 6] { self.array }

    #[inline]
    pub fn from_array(array: [f32; 6]) -> Transform {
        Transform { array: array }
    }

    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Transform {
        Transform { array: [a, b, c, d, e, f] }
    }

    #[inline]
    fn new_zero() -> Transform {
        Transform { array: [0.0; 6] }
    }

    pub fn new_identity() -> Transform {
        let mut t = Transform::new_zero();
        t.set_identity();
        t
    }

    pub fn new_from_slice(slice: &[f32]) -> Option<Transform> {
        if slice.len() >= 6 {
            Some(Transform::new(slice[0], slice[1], slice[2], slice[3], slice[4], slice[5]))
        }
        else {
            None
        }
    }

    //accessors!(a -> 0, b -> 1, c -> 2, d -> 3, e -> 4, f -> 5)
    #[inline] pub fn a(&self) -> f32 { self.array[0] }
    #[inline] pub fn b(&self) -> f32 { self.array[1] }
    #[inline] pub fn c(&self) -> f32 { self.array[2] }
    #[inline] pub fn d(&self) -> f32 { self.array[3] }
    #[inline] pub fn e(&self) -> f32 { self.array[4] }
    #[inline] pub fn f(&self) -> f32 { self.array[5] }

    pub fn set_identity(&mut self) {
        unsafe { ffi::nvgTransformIdentity(self.as_mut_ptr()) }
    }

    pub fn set_translate(&mut self, tx: f32, ty: f32) {
        unsafe { ffi::nvgTransformTranslate(self.as_mut_ptr(), tx, ty) }
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        unsafe { ffi::nvgTransformScale(self.as_mut_ptr(), sx, sy) }
    }

    pub fn set_rotate(&mut self, a: f32) {
        unsafe { ffi::nvgTransformRotate(self.as_mut_ptr(), a) }
    }

    pub fn set_skew_x(&mut self, a: f32) {
        unsafe { ffi::nvgTransformSkewX(self.as_mut_ptr(), a) }
    }

    pub fn set_skew_y(&mut self, a: f32) {
        unsafe { ffi::nvgTransformSkewY(self.as_mut_ptr(), a) }
    }

    pub fn set_multiply(&mut self, src: &Transform) {
        unsafe { ffi::nvgTransformMultiply(self.as_mut_ptr(), src.as_ptr()) }
    }

    pub fn set_premultiply(&mut self, src: &Transform) {
        unsafe { ffi::nvgTransformPremultiply(self.as_mut_ptr(), src.as_ptr()) }
    }

    pub fn set_inverse(&mut self, src: &Transform) -> bool {
        unsafe { ffi::nvgTransformInverse(self.as_mut_ptr(), src.as_ptr()) == 1 }
    }

//    mutators!(
//        translate(tx: f32, ty: f32) via set_translate,
//        scale(sx: f32, sy: f32) via set_scale,
//        rotate(a: f32) via set_rotate,
//        skew_x(a: f32) via set_skew_x,
//        skew_y(a: f32) via set_skew_y,
//        multiply(src: &Transform) via set_multiply,
//        premultiply(src: &Transform) via set_premultiply
//    )
    pub fn translate(mut self, tx: f32, ty: f32) -> Transform {
        let mut t = Transform::new_zero();
        t.set_translate(tx, ty);
        self.set_premultiply(&t);
        self
    }
    pub fn scale(mut self, sx: f32, sy: f32) -> Transform {
        let mut t = Transform::new_zero();
        t.set_scale(sx, sy);
        self.set_premultiply(&t);
        self
    }
    pub fn rotate(mut self, a: f32) -> Transform {
        let mut t = Transform::new_zero();
        t.set_rotate(a);
        self.set_premultiply(&t);
        self
    }
    pub fn skew_x(mut self, a: f32) -> Transform {
        let mut t = Transform::new_zero();
        t.set_skew_x(a);
        self.set_premultiply(&t);
        self
    }
    pub fn skew_y(mut self, a: f32) -> Transform {
        let mut t = Transform::new_zero();
        t.set_skew_y(a);
        self.set_premultiply(&t);
        self
    }
    pub fn multiply(mut self, src: &Transform) -> Transform {
        let mut t = Transform::new_zero();
        t.set_multiply(src);
        self.set_premultiply(&t);
        self
    }
    pub fn premultiply(mut self, src: &Transform) -> Transform {
        let mut t = Transform::new_zero();
        t.set_premultiply(src);
        self.set_premultiply(&t);
        self
    }


    pub fn inverted(mut self) -> Result<Transform, Transform> {
        let copy = self;
        if self.set_inverse(&copy) {
            Ok(self)
        } else {
            Err(copy)
        }
    }

    pub fn transform_point(&self, (srcx, srcy): (f32, f32)) -> (f32, f32) {
        let (mut dstx, mut dsty) = (0.0f32, 0.0f32);
        unsafe { ffi::nvgTransformPoint(&mut dstx, &mut dsty, self.as_ptr(), srcx, srcy); }
        (dstx, dsty)
    }
}

// Context

pub struct Context {
    ptr: *mut ffi::NVGcontext
}

impl !Send for Context {}

impl !Sync for Context {}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NVGcontext @ {:?}", self.ptr)
    }
}

#[cfg(any(feature = "gl2", feature = "gl3",
          feature = "gles2", feature = "gles3"))]
impl Drop for Context {
    #[cfg(feature = "gl2")]
    fn drop(&mut self) {
        self.delete_gl2();
        self.ptr = ptr::null_mut();
    }

    #[cfg(feature = "gl3")]
    fn drop(&mut self) {
        self.delete_gl3();
        self.ptr = ptr::null_mut();
    }

    #[cfg(feature = "gles2")]
    fn drop(&mut self) {
        self.delete_gles2();
        self.ptr = ptr::null_mut();
    }

    #[cfg(feature = "gles3")]
    fn drop(&mut self) {
        self.delete_gles3();
        self.ptr = ptr::null_mut();
    }
}

impl Context {
    #[cfg(feature = "gl2")]
    pub fn create_gl2(flags: CreationFlags) -> Context {
        Context {
            ptr: unsafe { ffi::nvgCreateGL2(flags.bits) }
        }
    }

    #[cfg(feature = "gl2")]
    fn delete_gl2(&self) {
        unsafe { ffi::nvgDeleteGL2(self.ptr) }
    }

    #[cfg(feature = "gl3")]
    pub fn create_gl3(flags: CreationFlags) -> Context {
        Context {
            ptr: unsafe { ffi::nvgCreateGL3(flags.bits) }
        }
    }

    #[cfg(feature = "gl3")]
    fn delete_gl3(&self) {
        unsafe { ffi::nvgDeleteGL3(self.ptr) }
    }

    #[cfg(feature = "gles2")]
    pub fn create_gles2(flags: CreationFlags) -> Context {
        Context {
            ptr: unsafe { ffi::nvgCreateGLES2(flags.bits) }
        }
    }

    #[cfg(feature = "gles2")]
    fn delete_gles2(&self) {
        unsafe { ffi::nvgDeleteGLES2(self.ptr) }
    }

    #[cfg(feature = "gles3")]
    pub fn create_gles3(flags: CreationFlags) -> Context {
        Context {
            ptr: unsafe { ffi::nvgCreateGLES3(flags.bits) }
        }
    }

    #[cfg(feature = "gles3")]
    fn delete_gles3(&self) {
        unsafe { ffi::nvgDeleteGLES3(self.ptr) }
    }

    pub fn begin_frame(&self, window_width: u32, window_height: u32, device_pixel_ratio: f32) {
        unsafe { ffi::nvgBeginFrame(self.ptr,
                        window_width as i32, window_height as i32, device_pixel_ratio) }
    }
    pub fn end_frame(&self) {
        unsafe { ffi::nvgEndFrame(self.ptr) }
    }

    pub fn save(&self) {
        unsafe { ffi::nvgSave(self.ptr) }
    }
    pub fn restore(&self) {
        unsafe { ffi::nvgRestore(self.ptr) }
    }
    pub fn reset(&self) {
        unsafe { ffi::nvgReset(self.ptr) }
    }

    pub fn stroke_color(&self, color: Color) {
        unsafe { ffi::nvgStrokeColor(self.ptr, color.nvg) }
    }
    pub fn stroke_paint(&self, paint: Paint) {
        unsafe { ffi::nvgStrokePaint(self.ptr, paint.nvg) }
    }
    pub fn fill_color(&self, color: Color) {
        unsafe { ffi::nvgFillColor(self.ptr, color.nvg) }
    }
    pub fn fill_paint(&self, paint: Paint) {
        unsafe { ffi::nvgFillPaint(self.ptr, paint.nvg) }
    }
    pub fn miter_limit(&self, limit: f32) {
        unsafe { ffi::nvgMiterLimit(self.ptr, limit) }
    }
    pub fn stroke_width(&self, size: f32) {
        unsafe { ffi::nvgStrokeWidth(self.ptr, size) }
    }
    pub fn line_cap(&self, cap: LineCap) {
        unsafe { ffi::nvgLineCap(self.ptr, cap as c_int) }
    }
    pub fn line_join(&self, join: LineCap) {
        unsafe { ffi::nvgLineJoin(self.ptr, join as c_int) }
    }
    pub fn global_alpha(&self, alpha: f32) {
        unsafe { ffi::nvgGlobalAlpha(self.ptr, alpha) }
    }

    pub fn reset_transform(&self) {
        unsafe { ffi::nvgResetTransform(self.ptr) }
    }
    pub fn transform(&self, t: Transform) {
        unsafe { ffi::nvgTransform(self.ptr, t.a(), t.b(), t.c(), t.d(), t.e(), t.f()) }
    }
    pub fn translate(&self, x: f32, y: f32) {
        unsafe { ffi::nvgTranslate(self.ptr, x, y) }
    }
    pub fn rotate(&self, angle: f32) {
        unsafe { ffi::nvgRotate(self.ptr, angle) }
    }
    pub fn skew_x(&self, angle: f32) {
        unsafe { ffi::nvgSkewX(self.ptr, angle) }
    }
    pub fn skew_y(&self, angle: f32) {
        unsafe { ffi::nvgSkewY(self.ptr, angle) }
    }
    pub fn scale(&self, x: f32, y: f32) {
        unsafe { ffi::nvgScale(self.ptr, x, y) }
    }
    pub fn current_transform(&self) -> Transform {
        let mut arr = [0.0f32; 6];
		unsafe { ffi::nvgCurrentTransform(self.ptr, arr.as_mut_ptr()) }
        Transform::from_array(arr)
    }

    #[inline]
    pub fn create_image<P>(&self, filename: P) -> Option<Image>
            where P: AsRef<Path> {
        self.create_image_flags(filename, ImageFlags::empty())
    }

    pub fn create_image_flags<P>(&self, filename: P, flags: ImageFlags) -> Option<Image>
            where P: AsRef<Path> {
        let c_filename = match filename.as_ref().as_os_str().to_cstring() {
            Some(o) => o,
            None => return None
        };
        // stb_image returns 0 for failure; unlike fontstash which returns -1
        match unsafe { ffi::nvgCreateImage(self.ptr, c_filename.as_ptr(),
                                           flags.bits() as c_int) } {
            ffi::STB_IMAGE_INVALID => { None },
            handle => { Some(Image::wrap(handle)) }
        }
    }

    #[inline]
    pub fn create_image_mem(&self, data: &[u8]) -> Option<Image> {
        self.create_image_mem_flags(data, ImageFlags::empty())
    }

    pub fn create_image_mem_flags(&self, data: &[u8], flags: ImageFlags) -> Option<Image> {
        let handle = unsafe { ffi::nvgCreateImageMem(self.ptr, flags.bits() as c_int, data.as_ptr(), data.len() as c_int) };
        match handle {
            ffi::STB_IMAGE_INVALID => { None },
            _ => { Some(Image::wrap(handle)) }
        }
    }

    #[inline]
    pub fn create_image_rgba(&self, w: u32, h: u32, data: &[u8]) -> Option<Image> {
        self.create_image_rgba_flags(w, h, data, ImageFlags::empty())
    }

    pub fn create_image_rgba_flags(&self, w: u32, h: u32, data: &[u8], flags: ImageFlags) -> Option<Image> {
        let handle = unsafe {
            ffi::nvgCreateImageRGBA(self.ptr, w as i32, h as i32, flags.bits() as c_int, data.as_ptr())
        };
        match handle {
            ffi::STB_IMAGE_INVALID => { None },
            _ => { Some(Image::wrap(handle)) }
        }
    }

    pub fn update_image(&self, image: &Image, data: &[u8]) {
        unsafe { ffi::nvgUpdateImage(self.ptr, image.handle, data.as_ptr()) }
    }

    pub fn image_size(&self, image: &Image) -> (u32, u32) {
        let (mut w, mut h) = (0, 0);
        unsafe { ffi::nvgImageSize(self.ptr, image.handle, &mut w, &mut h) };
        (w as u32, h as u32)
    }

    pub fn delete_image(&self, image: Image) {
        unsafe { ffi::nvgDeleteImage(self.ptr, image.handle) }
    }

    pub fn linear_gradient(&self, sx: f32, sy: f32, ex: f32, ey: f32, icol: Color, ocol: Color) -> Paint {
        Paint::wrap(unsafe { ffi::nvgLinearGradient(self.ptr, sx, sy, ex, ey, icol.nvg, ocol.nvg) })
    }
    pub fn box_gradient(&self, x: f32, y: f32, w: f32, h: f32, r: f32, f: f32, icol: Color, ocol: Color) -> Paint {
        Paint::wrap(unsafe { ffi::nvgBoxGradient(self.ptr, x, y, w, h, r, f, icol.nvg, ocol.nvg) })
    }
    pub fn radial_gradient(&self, cx: f32, cy: f32, inr: f32, outr: f32, icol: Color, ocol: Color) -> Paint {
        Paint::wrap(unsafe { ffi::nvgRadialGradient(self.ptr, cx, cy, inr, outr, icol.nvg, ocol.nvg) })
    }
    pub fn image_pattern(&self, ox: f32, oy: f32, ex: f32, ey: f32, angle: f32, image: &Image, repeat: PatternRepeat, alpha: f32) -> Paint {
        Paint::wrap(unsafe { ffi::nvgImagePattern(self.ptr, ox, oy, ex, ey, angle, image.handle, repeat as c_int, alpha) })
    }

    pub fn scissor(&self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ffi::nvgScissor(self.ptr, x, y, w, h) }
    }
    pub fn reset_scissor(&self) {
        unsafe { ffi::nvgResetScissor(self.ptr) }
    }

    pub fn begin_path(&self) {
        unsafe { ffi::nvgBeginPath(self.ptr) }
    }
    pub fn move_to(&self, x: f32, y: f32) {
        unsafe { ffi::nvgMoveTo(self.ptr, x, y) }
    }
    pub fn line_to(&self, x: f32, y: f32) {
        unsafe { ffi::nvgLineTo(self.ptr, x, y) }
    }
    pub fn bezier_to(&self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32) {
        unsafe { ffi::nvgBezierTo(self.ptr, c1x, c1y, c2x, c2y, x, y) }
    }
    pub fn quad_to(&self, cx: f32, cy: f32, x: f32, y: f32) {
        unsafe { ffi::nvgQuadTo(self.ptr, cx, cy, x, y) }
    }
    pub fn arc_to(&self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
        unsafe { ffi::nvgArcTo(self.ptr, x1, y1, x2, y2, radius) }
    }
    pub fn close_path(&self) {
        unsafe { ffi::nvgClosePath(self.ptr) }
    }
    pub fn path_winding(&self, dir: Solidity) {
        unsafe { ffi::nvgPathWinding(self.ptr, dir as c_int) }
    }

    pub fn arc(&self, cx: f32, cy: f32, r: f32, a0: f32, a1: f32, dir: Winding) {
        unsafe { ffi::nvgArc(self.ptr, cx, cy, r, a0, a1, dir as c_int) }
    }
    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ffi::nvgRect(self.ptr, x, y, w, h) }
    }
    pub fn rounded_rect(&self, x: f32, y: f32, w: f32, h: f32, r: f32) {
        unsafe { ffi::nvgRoundedRect(self.ptr, x, y, w, h, r) }
    }
    pub fn ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32) {
        unsafe { ffi::nvgEllipse(self.ptr, cx, cy, rx, ry) }
    }
    pub fn circle(&self, cx: f32, cy: f32, r: f32) {
        unsafe { ffi::nvgCircle(self.ptr, cx, cy, r) }
    }
    pub fn fill(&self) {
        unsafe { ffi::nvgFill(self.ptr) }
    }
    pub fn stroke(&self) {
        unsafe { ffi::nvgStroke(self.ptr) }
    }

    pub fn create_font<P>(&self, name: &str, filename: P) -> Option<Font>
            where P: AsRef<Path> {
        let c_name = match CString::new(name.as_bytes()){
            Ok(o) => o,
            Err(_) => return None
        };
        let c_filename = match filename.as_ref().as_os_str().to_cstring() {
            Some(o) => o,
            None => return None
        };
        match unsafe { ffi::nvgCreateFont(self.ptr, c_name.as_ptr(), c_filename.as_ptr()) } {
            ffi::FONT_INVALID => None,
            handle => Some(Font::wrap(handle))
        }
    }

    pub fn create_font_mem(&self, name: &str, data: &[u8]) -> Option<Font> {
        let c_name = match CString::new(name.as_bytes()){
            Ok(o) => o,
            Err(_) => return None
        };
        let handle = unsafe {
            ffi::nvgCreateFontMem(self.ptr, c_name.as_ptr(),
                                  data.as_ptr() as *mut u8, data.len() as c_int,
                                  0 /* do not free */)
        };
        match handle {
            ffi::FONT_INVALID => None,
            _ => Some(Font::wrap(handle))
        }
    }

    pub fn find_font(&self, name: &str) -> Option<Font> {
        let c_name = match CString::new(name.as_bytes()){
            Ok(o) => o,
            Err(_) => return None
        };
        let handle = unsafe { ffi::nvgFindFont(self.ptr, c_name.as_ptr()) };
        match handle {
            ffi::FONT_INVALID => None,
            _ => Some(Font::wrap(handle))
        }
    }
    pub fn font_size(&self, size: f32) {
        unsafe { ffi::nvgFontSize(self.ptr, size) }
    }
    pub fn font_blur(&self, blur: f32) {
        unsafe { ffi::nvgFontBlur(self.ptr, blur) }
    }
    pub fn text_letter_spacing(&self, spacing: f32) {
        unsafe { ffi::nvgTextLetterSpacing(self.ptr, spacing) }
    }
    pub fn text_line_height(&self, line_height: f32) {
        unsafe { ffi::nvgTextLineHeight(self.ptr, line_height) }
    }
    pub fn text_align(&self, align: Align) {
        unsafe { ffi::nvgTextAlign(self.ptr, align.bits) }
    }
    pub fn font_face_id(&self, font: &Font) {
        unsafe { ffi::nvgFontFaceId(self.ptr, font.handle) }
    }
    pub fn font_face(&self, font: &str) {
        let c_font = match CString::new(font.as_bytes()){
            Ok(o) => o,
            Err(_) => return
        };
        unsafe { ffi::nvgFontFace(self.ptr, c_font.as_ptr()) }
    }
    pub fn text(&self, x: f32, y: f32, text: &str) -> f32 {
        let c_text = match CString::new(text.as_bytes()){
            Ok(o) => o,
            _ => return 0.
        };
        unsafe { ffi::nvgText(self.ptr, x, y, c_text.as_ptr(), ptr::null()) }
    }
    pub fn text_box(&self, x: f32, y: f32, break_row_width: f32, text: &str) {
        let c_text = match CString::new(text.as_bytes()){
            Ok(o) => o,
            _ => return
        };
        unsafe { ffi::nvgTextBox(self.ptr, x, y, break_row_width, c_text.as_ptr(), ptr::null()) }
    }
    // Measures the specified text string. Parameter bounds should be a pointer to float[4],
    // if the bounding box of the text should be returned. The bounds value are [xmin,ymin, xmax,ymax]
    // Returns the horizontal advance of the measured text (i.e. where the next character should drawn).
    // Measured values are returned in local coordinate space.
    pub fn text_bounds(&self, x: f32, y: f32, text: &str, bounds: &mut [f32; 4]) -> f32 {
        let c_text = match CString::new(text.as_bytes()){
            Ok(o) => o,
            _ => return 0.
        };
        unsafe { ffi::nvgTextBounds(self.ptr, x, y, c_text.as_ptr(), ptr::null(), bounds.as_mut_ptr()) }
    }
    // Measures the needed advance for text, without computing complete bounds
    pub fn text_advance(&self, x:f32, y:f32, text: &str) -> f32 {
        let c_text = match CString::new(text.as_bytes()){
            Ok(o) => o,
            _ => return 0.
        };
        unsafe { ffi::nvgTextBounds(self.ptr, x, y, c_text.as_ptr(), ptr::null(), ptr::null_mut()) }
    }
    // Measures the specified multi-text string. Parameter bounds should be float[4],
    // if the bounding box of the text should be returned. The bounds value are [xmin,ymin, xmax,ymax]
    // Measured values are returned in local coordinate space.
    pub fn text_box_bounds(&self, x: f32, y: f32, break_row_width: f32, text: &str, bounds: &mut [f32; 4]) {
        let c_text = match CString::new(text.as_bytes()){
            Ok(o) => o,
            _ => return
        };
        //let bptr: *mut f32 = match bounds {
        //    Some(vec) => { bptr = vec.as_mut_ptr() }
        //    None => ptr::null()
        //}
        unsafe { ffi::nvgTextBoxBounds(self.ptr, x, y, break_row_width, c_text.as_ptr(), ptr::null(), bounds.as_mut_ptr()) }
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////
    /// return a vector of position info for all glyphs in 'text'.
    /// 'text' is utf8-encoded unicode, so the number of glyphs isn't necessarily the byte-length of the text.
    pub fn text_glyph_positions(&self, x: f32, y: f32, text: &str) -> Vec<GlyphPosition> {
        let mut positions: Vec<NVGglyphPosition> = Vec::with_capacity(text.len());
        for _ in 0..text.len() { // we may not need all of them, but if text is ascii, we will
            positions.push(NVGglyphPosition {
                byte_ptr: ptr::null(),
                x: 0.0,
                minx: 0.0,
                maxx: 0.0
            })
        }
        let st = text.as_ptr() as *const i8;
        let en = unsafe { st.offset(text.len() as isize) };

        let actual_n = unsafe {
            ffi::nvgTextGlyphPositions(self.ptr, x, y, st, en, positions.as_mut_ptr(), positions.len() as c_int)
        };
        assert!(actual_n >= 0);
        let actual_n = actual_n as usize;

        // convert pointers to indexes
        let mut ret_vec:Vec<GlyphPosition> = Vec::with_capacity(actual_n);
        for i in (0..actual_n) {
            let nvg = positions[i];
            ret_vec.push(GlyphPosition {
                byte_index: relative_index(text, nvg.byte_ptr),
                x: nvg.x,
                minx: nvg.minx,
                maxx: nvg.maxx
            });
        }

        return ret_vec;
    }

    pub fn text_break_lines(&self, text: &str, break_row_width: f32, max_rows: usize) -> Vec<TextRow> {
        let st = text.as_ptr() as *const i8;
        let en = unsafe { st.offset(text.len() as isize) };
        let mut rows: Vec<NVGtextRow> = Vec::with_capacity(max_rows);
        for _ in (0..max_rows) {
            rows.push(NVGtextRow {
                start: ptr::null(),
                end:   ptr::null(),
                next:  ptr::null(),
                width: 0.0,
                minx:  0.0,
                maxx:  0.0,
            })
        }

        let actual_n = unsafe {
            ffi::nvgTextBreakLines(self.ptr, st, en, break_row_width, rows.as_mut_ptr(), max_rows as c_int)
        };
        assert!(actual_n >= 0);
        let actual_n = actual_n as usize;

        // convert pointers to indexes
        let mut ret_vec:Vec<TextRow> = Vec::with_capacity(actual_n);
        for i in (0..actual_n) {
            let nvg = rows[i];
            ret_vec.push(TextRow {
                start_index: relative_index(text, nvg.start),
                end_index:   relative_index(text, nvg.end),
                next_index:  relative_index(text, nvg.next),
                width: nvg.width,
                minx: nvg.minx,
                maxx: nvg.maxx
            });
        }

        return ret_vec;
    }

    pub fn text_metrics(&self, ascender: *mut f32, descender: *mut f32, lineh: *mut f32) {
        unsafe { ffi::nvgTextMetrics(self.ptr, ascender, descender, lineh) }
    }

    pub fn debug_dump_path_cache(&self) {
        unsafe { ffi::nvgDebugDumpPathCache(self.ptr) }
    }

}

// given a utf8 string, and a ptr that walks through it,
// return instead the corresponding byte-index into the string.
pub fn relative_index(text: &str, p: *const i8) -> usize {
    let st = text.as_ptr();
    let stix: usize = st as usize;
    let pix: usize = p as usize;
    assert!(pix >= stix);               // require that 'p' point somewhere in the
    assert!(pix - stix <= text.len());  // string, or at most 1 past end (where C null would be)
    pix - stix
}

pub fn deg_to_rad(deg: f32) -> f32 {
    unsafe { ffi::nvgDegToRad(deg) }
}
pub fn rad_to_deg(rad: f32) -> f32 {
    unsafe { ffi::nvgRadToDeg(rad) }
}

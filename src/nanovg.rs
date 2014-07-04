#![crate_type = "lib"]
//#![crate_type = "rlib"]
//#![crate_type = "dylib"]
#![crate_id = "github.com/KevinKelley/nanovg-rs#nanovg:0.1"]
#![comment = "Binding for NanoVG vector-graphics library"]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

#![feature(unsafe_destructor)]  // use Option instead
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]
#![deny(unnecessary_parens)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
//#![warn(missing_doc)] // FIXME: should be denied.
#![deny(unused_result)]
#![allow(unused_imports)]
#![allow(unused_attribute)]
#![deny(unnecessary_typecast)]
#![warn(visible_private_types)] // FIXME: should be denied.
#![allow(dead_code)]
//#![feature(globs)]
//#![feature(macro_rules)]
//#![feature(managed_boxes)]
//#![feature(unsafe_destructor)]

extern crate libc;


use std::fmt;
use std::kinds::marker;
use std::ptr;
use std::str;
use std::bitflags;
use libc::{c_double, c_float, c_int, c_char, c_uint, c_ushort, c_uchar, c_void};

pub use NVGcolor         = ffi::NVGcolor;
pub use NVGpaint         = ffi::NVGpaint;
pub use NVGglyphPosition = ffi::NVGglyphPosition;
pub use NVGtextRow       = ffi::NVGtextRow;

mod ffi;




///
#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Winding {
    CCW                     = ffi::NVG_CCW,
    CW                      = ffi::NVG_CW,
}

///
#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Solidity {
    SOLID                   = ffi::NVG_SOLID,
    HOLE                    = ffi::NVG_HOLE,
}

///
#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum LineCap {
    BUTT                    = ffi::NVG_BUTT,
    ROUND                   = ffi::NVG_ROUND,
    SQUARE                  = ffi::NVG_SQUARE,
    BEVEL                   = ffi::NVG_BEVEL,
    MITER                   = ffi::NVG_MITER,
}

///
#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum PatternRepeat {
    NOREPEAT                = ffi::NVG_NOREPEAT,
    REPEATX                 = ffi::NVG_REPEATX,
    REPEATY                 = ffi::NVG_REPEATY,
}

///
pub bitflags!(
    flags Align: u32 {
        static LEFT         = ffi::NVG_ALIGN_LEFT,
        static CENTER       = ffi::NVG_ALIGN_CENTER,
        static RIGHT        = ffi::NVG_ALIGN_RIGHT,
        static TOP          = ffi::NVG_ALIGN_TOP,
        static MIDDLE       = ffi::NVG_ALIGN_MIDDLE,
        static BOTTOM       = ffi::NVG_ALIGN_BOTTOM,
        static BASELINE     = ffi::NVG_ALIGN_BASELINE
    }
)

/////
//#[repr(u32)]
//#[deriving(Clone, Eq, Hash, PartialEq, Show)]
//pub enum CreationFlags {
//    ANTIALIAS        = ffi::NVG_ANTIALIAS,
//    STENCIL_STROKES  = ffi::NVG_STENCIL_STROKES,
//}

pub bitflags!(
    flags CreationFlags: u32 {
        static ANTIALIAS        = ffi::NVG_ANTIALIAS,
        static STENCIL_STROKES  = ffi::NVG_STENCIL_STROKES
    }
)

//#[repr(C)]
//pub struct NVGcolor {
//    pub r: c_float,
//    pub g: c_float,
//    pub b: c_float,
//    pub a: c_float,
//
//}
//
//#[repr(C)]
//pub struct Union_Unnamed1 {
//    pub data: [u32, ..4u],
//}
//
//impl Union_Unnamed1 {
//    pub fn rgba(&mut self) -> *mut [c_float, ..4u] {
//        unsafe { ::std::mem::transmute(self) }
//    }
//}
//#[repr(C)]
//pub struct Unnamed2 {
//    pub r: c_float,
//    pub g: c_float,
//    pub b: c_float,
//    pub a: c_float,
//}
//
//#[repr(C)]
//pub struct NVGpaint {
//    pub xform: [c_float, ..6u],
//    pub extent: [c_float, ..2u],
//    pub radius: c_float,
//    pub feather: c_float,
//    pub innerColor: NVGcolor,
//    pub outerColor: NVGcolor,
//    pub image: c_int,
//    pub repeat: c_int,
//}
//#[repr(C)]
//pub struct NVGglyphPosition {
//    pub _str: *const c_char,
//    pub x: c_float,
//    pub minx: c_float,
//    pub maxx: c_float,
//}
//#[repr(C)]
//pub struct NVGtextRow {
//    pub start: *const c_char,
//    pub end: *const c_char,
//    pub next: *const c_char,
//    pub width: c_float,
//    pub minx: c_float,
//    pub maxx: c_float,
//}
//
//pub type Enum_NVGtexture = c_uint;
//pub static NVG_TEXTURE_ALPHA: c_uint = 1;
//pub static NVG_TEXTURE_RGBA: c_uint = 2;
//#[repr(C)]
//pub struct NVGscissor {
//    pub xform: [c_float, ..6u],
//    pub extent: [c_float, ..2u],
//}
//#[repr(C)]
//pub struct NVGvertex {
//    pub x: c_float,
//    pub y: c_float,
//    pub u: c_float,
//    pub v: c_float,
//}
//#[repr(C)]
//pub struct NVGpath {
//    pub first: c_int,
//    pub count: c_int,
//    pub closed: c_uchar,
//    pub nbevel: c_int,
//    pub fill: *mut NVGvertex,
//    pub nfill: c_int,
//    pub stroke: *mut NVGvertex,
//    pub nstroke: c_int,
//    pub winding: c_int,
//    pub convex: c_int,
//}

//#[deriving(Show)]
pub struct Ctx {
    pub ptr: *mut ffi::NVGcontext,
    no_send: marker::NoSend,
    no_share: marker::NoShare,
}

impl fmt::Show for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "opaque pointer @ {}", self.ptr)
    }
}

#[unsafe_destructor]
impl Drop for Ctx {
    fn drop(&mut self) {
        self.delete_gl3();
        self.ptr = ptr::mut_null();
    }
}

impl Ctx {

    //#if defined NANOVG_GL3
    pub fn create_gL3(flags: CreationFlags) -> Ctx {
        Ctx {
            ptr: unsafe { ffi::nvgCreateGL3(flags.bits) },
            no_send: marker::NoSend,
            no_share: marker::NoShare,
        }
    }
    fn delete_gl3(&self) {
        unsafe { ffi::nvgDeleteGL3(self.ptr) }
    }


    pub fn begin_frame(&self, windowWidth: c_int, windowHeight: c_int, devicePixelRatio: c_float) {
		unsafe { ffi::nvgBeginFrame(self.ptr, windowWidth, windowHeight, devicePixelRatio) }
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

    pub fn stroke_color(&self, color: NVGcolor) {
		unsafe { ffi::nvgStrokeColor(self.ptr, color) }
	}
    pub fn stroke_paint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgStrokePaint(self.ptr, paint) }
	}
    pub fn fill_color(&self, color: NVGcolor) {
		unsafe { ffi::nvgFillColor(self.ptr, color) }
	}
    pub fn fill_paint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgFillPaint(self.ptr, paint) }
	}
    pub fn miter_limit(&self, limit: c_float) {
		unsafe { ffi::nvgMiterLimit(self.ptr, limit) }
	}
    pub fn stroke_width(&self, size: c_float) {
		unsafe { ffi::nvgStrokeWidth(self.ptr, size) }
	}
    pub fn line_cap(&self, cap: c_int) {
		unsafe { ffi::nvgLineCap(self.ptr, cap) }
	}
    pub fn line_join(&self, join: c_int) {
		unsafe { ffi::nvgLineJoin(self.ptr, join) }
	}
    pub fn global_alpha(&self, alpha: c_float) {
		unsafe { ffi::nvgGlobalAlpha(self.ptr, alpha) }
	}

    pub fn reset_transform(&self) {
		unsafe { ffi::nvgResetTransform(self.ptr) }
	}
    pub fn transform(&self, a: c_float, b: c_float, c: c_float, d: c_float, e: c_float, f: c_float) {
		unsafe { ffi::nvgTransform(self.ptr, a, b, c, d, e, f) }
	}
    pub fn translate(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgTranslate(self.ptr, x, y) }
	}
    pub fn rotate(&self, angle: c_float) {
		unsafe { ffi::nvgRotate(self.ptr, angle) }
	}
    pub fn skew_x(&self, angle: c_float) {
		unsafe { ffi::nvgSkewX(self.ptr, angle) }
	}
    pub fn skew_y(&self, angle: c_float) {
		unsafe { ffi::nvgSkewY(self.ptr, angle) }
	}
    pub fn scale(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgScale(self.ptr, x, y) }
	}
    pub fn current_transform(&self, xform: *mut c_float) {
		unsafe { ffi::nvgCurrentTransform(self.ptr, xform) }
	}

    pub fn create_image(&self, filename: &str) -> c_int {
        filename.with_c_str(|filename| {
            unsafe { ffi::nvgCreateImage(self.ptr, filename) }
        })
	}
    pub fn create_image_mem(&self, data: *mut c_uchar, ndata: c_int) -> c_int {
		unsafe { ffi::nvgCreateImageMem(self.ptr, data, ndata) }
	}
    pub fn create_image_rgba(&self, w: c_int, h: c_int, data: *const c_uchar) -> c_int {
		unsafe { ffi::nvgCreateImageRGBA(self.ptr, w, h, data) }
	}
    pub fn update_image(&self, image: c_int, data: *const c_uchar) {
		unsafe { ffi::nvgUpdateImage(self.ptr, image, data) }
	}
    pub fn image_size(&self, image: c_int, w: *mut c_int, h: *mut c_int) {
		unsafe { ffi::nvgImageSize(self.ptr, image, w, h) }
	}
    pub fn delete_image(&self, image: c_int) {
		unsafe { ffi::nvgDeleteImage(self.ptr, image) }
	}

    pub fn linear_gradient(&self, sx: c_float, sy: c_float, ex: c_float, ey: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgLinearGradient(self.ptr, sx, sy, ex, ey, icol, ocol) }
	}
    pub fn box_gradient(&self, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float, f: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgBoxGradient(self.ptr, x, y, w, h, r, f, icol, ocol) }
	}
    pub fn radial_gradient(&self, cx: c_float, cy: c_float, inr: c_float, outr: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgRadialGradient(self.ptr, cx, cy, inr, outr, icol, ocol) }
	}
    pub fn image_pattern(&self, ox: c_float, oy: c_float, ex: c_float, ey: c_float, angle: c_float, image: c_int, repeat: c_int, alpha: c_float) -> NVGpaint {
		unsafe { ffi::nvgImagePattern(self.ptr, ox, oy, ex, ey, angle, image, repeat, alpha) }
	}

    pub fn scissor(&self, x: c_float, y: c_float, w: c_float, h: c_float) {
		unsafe { ffi::nvgScissor(self.ptr, x, y, w, h) }
	}
    pub fn reset_scissor(&self) {
		unsafe { ffi::nvgResetScissor(self.ptr) }
	}

    pub fn begin_path(&self) {
		unsafe { ffi::nvgBeginPath(self.ptr) }
	}
    pub fn move_to(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgMoveTo(self.ptr, x, y) }
	}
    pub fn line_to(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgLineTo(self.ptr, x, y) }
	}
    pub fn bezier_to(&self, c1x: c_float, c1y: c_float, c2x: c_float, c2y: c_float, x: c_float, y: c_float) {
		unsafe { ffi::nvgBezierTo(self.ptr, c1x, c1y, c2x, c2y, x, y) }
	}
    pub fn quad_to(&self, cx: c_float, cy: c_float, x: c_float, y: c_float) {
		unsafe { ffi::nvgQuadTo(self.ptr, cx, cy, x, y) }
	}
    pub fn arc_to(&self, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float) {
		unsafe { ffi::nvgArcTo(self.ptr, x1, y1, x2, y2, radius) }
	}
    pub fn close_path(&self) {
		unsafe { ffi::nvgClosePath(self.ptr) }
	}
    pub fn path_winding(&self, dir: Solidity) {
		unsafe { ffi::nvgPathWinding(self.ptr, dir as i32) }
	}

    pub fn arc(&self, cx: c_float, cy: c_float, r: c_float, a0: c_float, a1: c_float, dir: c_int) {
		unsafe { ffi::nvgArc(self.ptr, cx, cy, r, a0, a1, dir) }
	}
    pub fn rect(&self, x: c_float, y: c_float, w: c_float, h: c_float) {
		unsafe { ffi::nvgRect(self.ptr, x, y, w, h) }
	}
    pub fn rounded_rect(&self, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float) {
		unsafe { ffi::nvgRoundedRect(self.ptr, x, y, w, h, r) }
	}
    pub fn ellipse(&self, cx: c_float, cy: c_float, rx: c_float, ry: c_float) {
		unsafe { ffi::nvgEllipse(self.ptr, cx, cy, rx, ry) }
	}
    pub fn circle(&self, cx: c_float, cy: c_float, r: c_float) {
		unsafe { ffi::nvgCircle(self.ptr, cx, cy, r) }
	}
    pub fn fill(&self) {
		unsafe { ffi::nvgFill(self.ptr) }
	}
    pub fn stroke(&self) {
		unsafe { ffi::nvgStroke(self.ptr) }
	}

    pub fn create_font(&self, name: &str, filename: &str) -> c_int {
        name.with_c_str(|name| {
            filename.with_c_str(|filename| {
		unsafe { ffi::nvgCreateFont(self.ptr, name, filename) }
            })
        })
	}
    pub fn create_font_mem(&self, name: *const c_char, data: *mut c_uchar, ndata: c_int, freeData: c_int) -> c_int {
		unsafe { ffi::nvgCreateFontMem(self.ptr, name, data, ndata, freeData) }
	}
    pub fn find_font(&self, name: *const c_char) -> c_int {
		unsafe { ffi::nvgFindFont(self.ptr, name) }
	}
    pub fn font_size(&self, size: c_float) {
		unsafe { ffi::nvgFontSize(self.ptr, size) }
	}
    pub fn font_blur(&self, blur: c_float) {
		unsafe { ffi::nvgFontBlur(self.ptr, blur) }
	}
    pub fn text_letter_spacing(&self, spacing: c_float) {
		unsafe { ffi::nvgTextLetterSpacing(self.ptr, spacing) }
	}
    pub fn text_line_height(&self, lineHeight: c_float) {
		unsafe { ffi::nvgTextLineHeight(self.ptr, lineHeight) }
	}
    pub fn text_align(&self, align: Align) {
		unsafe { ffi::nvgTextAlign(self.ptr, align.bits) }
	}
    pub fn font_face_id(&self, font: c_int) {
		unsafe { ffi::nvgFontFaceId(self.ptr, font) }
	}
    pub fn font_face(&self, font: &str) {
        font.with_c_str(|font| {
		unsafe { ffi::nvgFontFace(self.ptr, font) }
        })
	}
    pub fn text(&self, x: c_float, y: c_float, text: &str) -> c_float {
        text.with_c_str(|text| {
            unsafe { ffi::nvgText(self.ptr, x, y, text, ptr::null()) }
        })
    }
    //pub fn text(&self, x: c_float, y: c_float, text: &str, end: &str) -> c_float {
    //    text.with_c_str(|text| {
    //        end.with_c_str(|end| {
    //            unsafe { ffi::nvgText(self.ptr, x, y, text, end) }
    //        })
    //    })
    //}
    pub fn text_box(&self, x: c_float, y: c_float, breakRowWidth: c_float, text: &str, end: &str) {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBox(self.ptr, x, y, breakRowWidth, text, end) }
            })
        })
	}
    pub fn text_bounds(&self, x: c_float, y: c_float, text: &str, bounds: *mut c_float) -> c_float {
        text.with_c_str(|text| {
    	unsafe { ffi::nvgTextBounds(self.ptr, x, y, text, ptr::null(), bounds) }
        })
	}
    pub fn text_box_bounds(&self, x: c_float, y: c_float, breakRowWidth: c_float, text: &str, end: &str, bounds: *mut c_float) {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBoxBounds(self.ptr, x, y, breakRowWidth, text, end, bounds) }
            })
        })
	}
    pub fn text_glyph_positions(&self, x: c_float, y: c_float, text: &str, end: &str, positions: *mut NVGglyphPosition, maxPositions: c_int) -> c_int {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextGlyphPositions(self.ptr, x, y, text, end, positions, maxPositions) }
            })
        })
	}
    pub fn text_metrics(&self, ascender: *mut c_float, descender: *mut c_float, lineh: *mut c_float) {
		unsafe { ffi::nvgTextMetrics(self.ptr, ascender, descender, lineh) }
	}
    pub fn text_break_lines(&self, text: &str, end: &str, breakRowWidth: c_float, rows: *mut NVGtextRow, maxRows: c_int) -> c_int {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBreakLines(self.ptr, text, end, breakRowWidth, rows, maxRows) }
            })
        })
	}

}

//    pub fn nvgTransformIdentity(dst: *mut c_float);
//    pub fn nvgTransformTranslate(dst: *mut c_float, tx: c_float, ty: c_float);
//    pub fn nvgTransformScale(dst: *mut c_float, sx: c_float, sy: c_float);
//    pub fn nvgTransformRotate(dst: *mut c_float, a: c_float);
//    pub fn nvgTransformSkewX(dst: *mut c_float, a: c_float);
//    pub fn nvgTransformSkewY(dst: *mut c_float, a: c_float);
//    pub fn nvgTransformMultiply(dst: *mut c_float, src: *const c_float);
//    pub fn nvgTransformPremultiply(dst: *mut c_float, src: *const c_float);
//    pub fn nvgTransformInverse(dst: *mut c_float, src: *const c_float) -> c_int;
//    pub fn nvgTransformPoint(dstx: *mut c_float, dsty: *mut c_float, xform: *const c_float, srcx: c_float, srcy: c_float);
//
//    pub fn nvgDegToRad(deg: c_float) -> c_float;
//    pub fn nvgRadToDeg(rad: c_float) -> c_float;
//
//    pub fn nvgCreateInternal(params: *mut NVGparams) -> *mut NVGcontext;
//    pub fn nvgDeleteInternal(ctx: *mut NVGcontext);
//    pub fn nvgInternalParams(ctx: *mut NVGcontext) -> *mut NVGparams;
//    pub fn nvgDebugDumpPathCache(ctx: *mut NVGcontext);
//
pub fn rgb(r: u8, g: u8, b: u8) -> NVGcolor {
    unsafe { ffi::nvgRGB(r, g, b) }
}
//    pub fn nvgRGBf(r: c_float, g: c_float, b: c_float) -> NVGcolor;
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> NVGcolor {
    unsafe { ffi::nvgRGBA(r, g, b, a) }
}
//    pub fn nvgRGBAf(r: c_float, g: c_float, b: c_float, a: c_float) -> NVGcolor;
//    pub fn nvgLerpRGBA(c0: NVGcolor, c1: NVGcolor, u: c_float) -> NVGcolor;
//    pub fn nvgTransRGBA(c0: NVGcolor, a: c_uchar) -> NVGcolor;
//    pub fn nvgTransRGBAf(c0: NVGcolor, a: c_float) -> NVGcolor;
//    pub fn nvgHSL(h: c_float, s: c_float, l: c_float) -> NVGcolor;
//    pub fn nvgHSLA(h: c_float, s: c_float, l: c_float, a: c_uchar) -> NVGcolor;
pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> NVGcolor {
    unsafe { ffi:: nvgHSLA(h,s,l, a) }
}



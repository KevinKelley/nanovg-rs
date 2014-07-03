#![crate_type = "lib"]
//#![crate_type = "rlib"]
//#![crate_type = "dylib"]
#![crate_id = "github.com/KevinKelley/nanovg-rs#nanovg:0.1"]
#![comment = "Binding for NanoVG vector-graphics library"]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

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
#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Align {
    ALIGN_LEFT              = ffi::NVG_ALIGN_LEFT,
    ALIGN_CENTER            = ffi::NVG_ALIGN_CENTER,
    ALIGN_RIGHT             = ffi::NVG_ALIGN_RIGHT,
    ALIGN_TOP               = ffi::NVG_ALIGN_TOP,
    ALIGN_MIDDLE            = ffi::NVG_ALIGN_MIDDLE,
    ALIGN_BOTTOM            = ffi::NVG_ALIGN_BOTTOM,
    ALIGN_BASELINE          = ffi::NVG_ALIGN_BASELINE,
}

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

impl Ctx {

    //#if defined NANOVG_GL3
    pub fn CreateGL3(flags: CreationFlags) -> Ctx {
        Ctx {
            ptr: unsafe { ffi::nvgCreateGL3(flags.bits) },
            no_send: marker::NoSend,
            no_share: marker::NoShare,
        }
    }
    pub fn DeleteGL3(&self) {
        unsafe { ffi::nvgDeleteGL3(self.ptr) }
    }


    pub fn BeginFrame(&self, windowWidth: c_int, windowHeight: c_int, devicePixelRatio: c_float) {
		unsafe { ffi::nvgBeginFrame(self.ptr, windowWidth, windowHeight, devicePixelRatio) }
	}
    pub fn EndFrame(&self) {
		unsafe { ffi::nvgEndFrame(self.ptr) }
	}

    pub fn Save(&self) {
		unsafe { ffi::nvgSave(self.ptr) }
	}
    pub fn Restore(&self) {
		unsafe { ffi::nvgRestore(self.ptr) }
	}
    pub fn Reset(&self) {
		unsafe { ffi::nvgReset(self.ptr) }
	}

    pub fn StrokeColor(&self, color: NVGcolor) {
		unsafe { ffi::nvgStrokeColor(self.ptr, color) }
	}
    pub fn StrokePaint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgStrokePaint(self.ptr, paint) }
	}
    pub fn FillColor(&self, color: NVGcolor) {
		unsafe { ffi::nvgFillColor(self.ptr, color) }
	}
    pub fn FillPaint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgFillPaint(self.ptr, paint) }
	}
    pub fn MiterLimit(&self, limit: c_float) {
		unsafe { ffi::nvgMiterLimit(self.ptr, limit) }
	}
    pub fn StrokeWidth(&self, size: c_float) {
		unsafe { ffi::nvgStrokeWidth(self.ptr, size) }
	}
    pub fn LineCap(&self, cap: c_int) {
		unsafe { ffi::nvgLineCap(self.ptr, cap) }
	}
    pub fn LineJoin(&self, join: c_int) {
		unsafe { ffi::nvgLineJoin(self.ptr, join) }
	}
    pub fn GlobalAlpha(&self, alpha: c_float) {
		unsafe { ffi::nvgGlobalAlpha(self.ptr, alpha) }
	}

    pub fn ResetTransform(&self) {
		unsafe { ffi::nvgResetTransform(self.ptr) }
	}
    pub fn Transform(&self, a: c_float, b: c_float, c: c_float, d: c_float, e: c_float, f: c_float) {
		unsafe { ffi::nvgTransform(self.ptr, a, b, c, d, e, f) }
	}
    pub fn Translate(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgTranslate(self.ptr, x, y) }
	}
    pub fn Rotate(&self, angle: c_float) {
		unsafe { ffi::nvgRotate(self.ptr, angle) }
	}
    pub fn SkewX(&self, angle: c_float) {
		unsafe { ffi::nvgSkewX(self.ptr, angle) }
	}
    pub fn SkewY(&self, angle: c_float) {
		unsafe { ffi::nvgSkewY(self.ptr, angle) }
	}
    pub fn Scale(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgScale(self.ptr, x, y) }
	}
    pub fn CurrentTransform(&self, xform: *mut c_float) {
		unsafe { ffi::nvgCurrentTransform(self.ptr, xform) }
	}

    pub fn CreateImage(&self, filename: *const c_char) -> c_int {
		unsafe { ffi::nvgCreateImage(self.ptr, filename) }
	}
    pub fn CreateImageMem(&self, data: *mut c_uchar, ndata: c_int) -> c_int {
		unsafe { ffi::nvgCreateImageMem(self.ptr, data, ndata) }
	}
    pub fn CreateImageRGBA(&self, w: c_int, h: c_int, data: *const c_uchar) -> c_int {
		unsafe { ffi::nvgCreateImageRGBA(self.ptr, w, h, data) }
	}
    pub fn UpdateImage(&self, image: c_int, data: *const c_uchar) {
		unsafe { ffi::nvgUpdateImage(self.ptr, image, data) }
	}
    pub fn ImageSize(&self, image: c_int, w: *mut c_int, h: *mut c_int) {
		unsafe { ffi::nvgImageSize(self.ptr, image, w, h) }
	}
    pub fn DeleteImage(&self, image: c_int) {
		unsafe { ffi::nvgDeleteImage(self.ptr, image) }
	}

    pub fn LinearGradient(&self, sx: c_float, sy: c_float, ex: c_float, ey: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgLinearGradient(self.ptr, sx, sy, ex, ey, icol, ocol) }
	}
    pub fn BoxGradient(&self, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float, f: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgBoxGradient(self.ptr, x, y, w, h, r, f, icol, ocol) }
	}
    pub fn RadialGradient(&self, cx: c_float, cy: c_float, inr: c_float, outr: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint {
		unsafe { ffi::nvgRadialGradient(self.ptr, cx, cy, inr, outr, icol, ocol) }
	}
    pub fn ImagePattern(&self, ox: c_float, oy: c_float, ex: c_float, ey: c_float, angle: c_float, image: c_int, repeat: c_int, alpha: c_float) -> NVGpaint {
		unsafe { ffi::nvgImagePattern(self.ptr, ox, oy, ex, ey, angle, image, repeat, alpha) }
	}

    pub fn Scissor(&self, x: c_float, y: c_float, w: c_float, h: c_float) {
		unsafe { ffi::nvgScissor(self.ptr, x, y, w, h) }
	}
    pub fn ResetScissor(&self) {
		unsafe { ffi::nvgResetScissor(self.ptr) }
	}

    pub fn BeginPath(&self) {
		unsafe { ffi::nvgBeginPath(self.ptr) }
	}
    pub fn MoveTo(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgMoveTo(self.ptr, x, y) }
	}
    pub fn LineTo(&self, x: c_float, y: c_float) {
		unsafe { ffi::nvgLineTo(self.ptr, x, y) }
	}
    pub fn BezierTo(&self, c1x: c_float, c1y: c_float, c2x: c_float, c2y: c_float, x: c_float, y: c_float) {
		unsafe { ffi::nvgBezierTo(self.ptr, c1x, c1y, c2x, c2y, x, y) }
	}
    pub fn QuadTo(&self, cx: c_float, cy: c_float, x: c_float, y: c_float) {
		unsafe { ffi::nvgQuadTo(self.ptr, cx, cy, x, y) }
	}
    pub fn ArcTo(&self, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float) {
		unsafe { ffi::nvgArcTo(self.ptr, x1, y1, x2, y2, radius) }
	}
    pub fn ClosePath(&self) {
		unsafe { ffi::nvgClosePath(self.ptr) }
	}
    pub fn PathWinding(&self, dir: c_int) {
		unsafe { ffi::nvgPathWinding(self.ptr, dir) }
	}

    pub fn Arc(&self, cx: c_float, cy: c_float, r: c_float, a0: c_float, a1: c_float, dir: c_int) {
		unsafe { ffi::nvgArc(self.ptr, cx, cy, r, a0, a1, dir) }
	}
    pub fn Rect(&self, x: c_float, y: c_float, w: c_float, h: c_float) {
		unsafe { ffi::nvgRect(self.ptr, x, y, w, h) }
	}
    pub fn RoundedRect(&self, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float) {
		unsafe { ffi::nvgRoundedRect(self.ptr, x, y, w, h, r) }
	}
    pub fn Ellipse(&self, cx: c_float, cy: c_float, rx: c_float, ry: c_float) {
		unsafe { ffi::nvgEllipse(self.ptr, cx, cy, rx, ry) }
	}
    pub fn Circle(&self, cx: c_float, cy: c_float, r: c_float) {
		unsafe { ffi::nvgCircle(self.ptr, cx, cy, r) }
	}
    pub fn Fill(&self) {
		unsafe { ffi::nvgFill(self.ptr) }
	}
    pub fn Stroke(&self) {
		unsafe { ffi::nvgStroke(self.ptr) }
	}

    pub fn CreateFont(&self, name: *const c_char, filename: *const c_char) -> c_int {
		unsafe { ffi::nvgCreateFont(self.ptr, name, filename) }
	}
    pub fn CreateFontMem(&self, name: *const c_char, data: *mut c_uchar, ndata: c_int, freeData: c_int) -> c_int {
		unsafe { ffi::nvgCreateFontMem(self.ptr, name, data, ndata, freeData) }
	}
    pub fn FindFont(&self, name: *const c_char) -> c_int {
		unsafe { ffi::nvgFindFont(self.ptr, name) }
	}
    pub fn FontSize(&self, size: c_float) {
		unsafe { ffi::nvgFontSize(self.ptr, size) }
	}
    pub fn FontBlur(&self, blur: c_float) {
		unsafe { ffi::nvgFontBlur(self.ptr, blur) }
	}
    pub fn TextLetterSpacing(&self, spacing: c_float) {
		unsafe { ffi::nvgTextLetterSpacing(self.ptr, spacing) }
	}
    pub fn TextLineHeight(&self, lineHeight: c_float) {
		unsafe { ffi::nvgTextLineHeight(self.ptr, lineHeight) }
	}
    pub fn TextAlign(&self, align: c_uint) {
		unsafe { ffi::nvgTextAlign(self.ptr, align) }
	}
    pub fn FontFaceId(&self, font: c_int) {
		unsafe { ffi::nvgFontFaceId(self.ptr, font) }
	}
    pub fn FontFace(&self, font: &str) {
        font.with_c_str(|font| {
		unsafe { ffi::nvgFontFace(self.ptr, font) }
        })
	}
    pub fn Text(&self, x: c_float, y: c_float, text: &str) -> c_float {
        text.with_c_str(|text| {
            unsafe { ffi::nvgText(self.ptr, x, y, text, ptr::null()) }
        })
    }
    //pub fn Text(&self, x: c_float, y: c_float, text: &str, end: &str) -> c_float {
    //    text.with_c_str(|text| {
    //        end.with_c_str(|end| {
    //            unsafe { ffi::nvgText(self.ptr, x, y, text, end) }
    //        })
    //    })
    //}
    pub fn TextBox(&self, x: c_float, y: c_float, breakRowWidth: c_float, text: &str, end: &str) {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBox(self.ptr, x, y, breakRowWidth, text, end) }
            })
        })
	}
    pub fn TextBounds(&self, x: c_float, y: c_float, text: &str, end: &str, bounds: *mut c_float) -> c_float {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBounds(self.ptr, x, y, text, end, bounds) }
            })
        })
	}
    pub fn TextBoxBounds(&self, x: c_float, y: c_float, breakRowWidth: c_float, text: &str, end: &str, bounds: *mut c_float) {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextBoxBounds(self.ptr, x, y, breakRowWidth, text, end, bounds) }
            })
        })
	}
    pub fn TextGlyphPositions(&self, x: c_float, y: c_float, text: &str, end: &str, positions: *mut NVGglyphPosition, maxPositions: c_int) -> c_int {
        text.with_c_str(|text| {
            end.with_c_str(|end| {
		unsafe { ffi::nvgTextGlyphPositions(self.ptr, x, y, text, end, positions, maxPositions) }
            })
        })
	}
    pub fn TextMetrics(&self, ascender: *mut c_float, descender: *mut c_float, lineh: *mut c_float) {
		unsafe { ffi::nvgTextMetrics(self.ptr, ascender, descender, lineh) }
	}
    pub fn TextBreakLines(&self, text: &str, end: &str, breakRowWidth: c_float, rows: *mut NVGtextRow, maxRows: c_int) -> c_int {
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
pub fn RGB(r: u8, g: u8, b: u8) -> NVGcolor {
    unsafe { ffi::nvgRGB(r, g, b) }
}
//    pub fn nvgRGBf(r: c_float, g: c_float, b: c_float) -> NVGcolor;
pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> NVGcolor {
    unsafe { ffi::nvgRGBA(r, g, b, a) }
}
//    pub fn nvgRGBAf(r: c_float, g: c_float, b: c_float, a: c_float) -> NVGcolor;
//    pub fn nvgLerpRGBA(c0: NVGcolor, c1: NVGcolor, u: c_float) -> NVGcolor;
//    pub fn nvgTransRGBA(c0: NVGcolor, a: c_uchar) -> NVGcolor;
//    pub fn nvgTransRGBAf(c0: NVGcolor, a: c_float) -> NVGcolor;
//    pub fn nvgHSL(h: c_float, s: c_float, l: c_float) -> NVGcolor;
//    pub fn nvgHSLA(h: c_float, s: c_float, l: c_float, a: c_uchar) -> NVGcolor;




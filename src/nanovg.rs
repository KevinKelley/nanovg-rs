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
#![allow(unnecessary_qualification)]
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

pub use Color            = ffi::NVGcolor;
pub use NVGpaint         = ffi::NVGpaint;
pub use NVGglyphPosition = ffi::NVGglyphPosition;
pub use NVGtextRow       = ffi::NVGtextRow;

mod ffi;


#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Winding {
    CCW                     = ffi::NVG_CCW,
    CW                      = ffi::NVG_CW,
}

#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Solidity {
    SOLID                   = ffi::NVG_SOLID,
    HOLE                    = ffi::NVG_HOLE,
}

#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum LineCap {
    BUTT                    = ffi::NVG_BUTT,
    ROUND                   = ffi::NVG_ROUND,
    SQUARE                  = ffi::NVG_SQUARE,
    BEVEL                   = ffi::NVG_BEVEL,
    MITER                   = ffi::NVG_MITER,
}

#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum PatternRepeat {
    NOREPEAT                = ffi::NVG_NOREPEAT,
    REPEATX                 = ffi::NVG_REPEATX,
    REPEATY                 = ffi::NVG_REPEATY,
}

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

pub bitflags!(
    flags CreationFlags: u32 {
        static ANTIALIAS        = ffi::NVG_ANTIALIAS,
        static STENCIL_STROKES  = ffi::NVG_STENCIL_STROKES
    }
)

//#[repr(C)]
//#[deriving(Clone, PartialEq, Show)]
//pub struct Color {
//    pub r: f32,
//    pub g: f32,
//    pub b: f32,
//    pub a: f32,
//}
//}

//impl ffi::NVGcolor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        unsafe { ffi::nvgRGB(r, g, b) }
    }
    pub fn rgb_f(r: f32, g: f32, b: f32) -> Color {
        unsafe { ffi::nvgRGBf(r, g, b) }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        unsafe { ffi::nvgRGBA(r, g, b, a) }
    }
    pub fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Color {
        unsafe { ffi::nvgRGBAf(r, g, b, a) }
    }
    pub fn lerp_rgba(c0: Color, c1: Color, u: f32) -> Color {
        unsafe { ffi::nvgLerpRGBA(c0, c1, u) }
    }
    pub fn trans_rgba(c0: Color, a: u8) -> Color {
        unsafe { ffi::nvgTransRGBA(c0, a) }
    }
    pub fn trans_rgba_f(c0: Color, a: f32) -> Color {
        unsafe { ffi::nvgTransRGBAf(c0, a) }
    }
    pub fn hsl(h: f32, s: f32, l: f32) -> Color {
        unsafe { ffi::nvgHSL(h, s, l) }
    }
    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> Color {
        unsafe { ffi:: nvgHSLA(h,s,l, a) }
    }
//
//}

//#[repr(C)]
//pub struct NVGpaint {
//    pub xform: [f32, ..6u],
//    pub extent: [f32, ..2u],
//    pub radius: f32,
//    pub feather: f32,
//    pub innerColor: NVGcolor,
//    pub outerColor: NVGcolor,
//    pub image: i32,
//    pub repeat: i32,
//}
//#[repr(C)]
//pub struct NVGglyphPosition {
//    pub _str: *const c_char,
//    pub x: f32,
//    pub minx: f32,
//    pub maxx: f32,
//}
//#[repr(C)]
//pub struct NVGtextRow {
//    pub start: *const c_char,
//    pub end: *const c_char,
//    pub next: *const c_char,
//    pub width: f32,
//    pub minx: f32,
//    pub maxx: f32,
//}
//
//pub type Enum_NVGtexture = u32;
//pub static NVG_TEXTURE_ALPHA: u32 = 1;
//pub static NVG_TEXTURE_RGBA: u32 = 2;
//#[repr(C)]
//pub struct NVGscissor {
//    pub xform: [f32, ..6u],
//    pub extent: [f32, ..2u],
//}
//#[repr(C)]
//pub struct NVGvertex {
//    pub x: f32,
//    pub y: f32,
//    pub u: f32,
//    pub v: f32,
//}
//#[repr(C)]
//pub struct NVGpath {
//    pub first: i32,
//    pub count: i32,
//    pub closed: u8,
//    pub nbevel: i32,
//    pub fill: *mut NVGvertex,
//    pub nfill: i32,
//    pub stroke: *mut NVGvertex,
//    pub nstroke: i32,
//    pub winding: i32,
//    pub convex: i32,
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

impl Ctx
{
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


    pub fn begin_frame(&self, windowWidth: i32, windowHeight: i32, devicePixelRatio: f32) {
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

    pub fn stroke_color(&self, color: Color) {
		unsafe { ffi::nvgStrokeColor(self.ptr, color) }
	}
    pub fn stroke_paint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgStrokePaint(self.ptr, paint) }
	}
    pub fn fill_color(&self, color: Color) {
		unsafe { ffi::nvgFillColor(self.ptr, color) }
	}
    pub fn fill_paint(&self, paint: NVGpaint) {
		unsafe { ffi::nvgFillPaint(self.ptr, paint) }
	}
    pub fn miter_limit(&self, limit: f32) {
		unsafe { ffi::nvgMiterLimit(self.ptr, limit) }
	}
    pub fn stroke_width(&self, size: f32) {
		unsafe { ffi::nvgStrokeWidth(self.ptr, size) }
	}
    pub fn line_cap(&self, cap: LineCap) {
		unsafe { ffi::nvgLineCap(self.ptr, cap as i32) }
	}
    pub fn line_join(&self, join: LineCap) {
		unsafe { ffi::nvgLineJoin(self.ptr, join as i32) }
	}
    pub fn global_alpha(&self, alpha: f32) {
		unsafe { ffi::nvgGlobalAlpha(self.ptr, alpha) }
	}

    pub fn reset_transform(&self) {
		unsafe { ffi::nvgResetTransform(self.ptr) }
	}
    pub fn transform(&self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
		unsafe { ffi::nvgTransform(self.ptr, a, b, c, d, e, f) }
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
    pub fn current_transform(&self, xform: *mut f32) {
		unsafe { ffi::nvgCurrentTransform(self.ptr, xform) }
	}

    pub fn create_image(&self, filename: &str) -> i32 {
        filename.with_c_str(|filename| {
            unsafe { ffi::nvgCreateImage(self.ptr, filename) }
        })
	}
    pub fn create_image_mem(&self, data: *mut u8, ndata: i32) -> i32 {
		unsafe { ffi::nvgCreateImageMem(self.ptr, data, ndata) }
	}
    pub fn create_image_rgba(&self, w: i32, h: i32, data: *const u8) -> i32 {
		unsafe { ffi::nvgCreateImageRGBA(self.ptr, w, h, data) }
	}
    pub fn update_image(&self, image: i32, data: *const u8) {
		unsafe { ffi::nvgUpdateImage(self.ptr, image, data) }
	}
    pub fn image_size(&self, image: i32, w: *mut i32, h: *mut i32) {
		unsafe { ffi::nvgImageSize(self.ptr, image, w, h) }
	}
    pub fn delete_image(&self, image: i32) {
		unsafe { ffi::nvgDeleteImage(self.ptr, image) }
	}

    pub fn linear_gradient(&self, sx: f32, sy: f32, ex: f32, ey: f32, icol: Color, ocol: Color) -> NVGpaint {
		unsafe { ffi::nvgLinearGradient(self.ptr, sx, sy, ex, ey, icol, ocol) }
	}
    pub fn box_gradient(&self, x: f32, y: f32, w: f32, h: f32, r: f32, f: f32, icol: Color, ocol: Color) -> NVGpaint {
		unsafe { ffi::nvgBoxGradient(self.ptr, x, y, w, h, r, f, icol, ocol) }
	}
    pub fn radial_gradient(&self, cx: f32, cy: f32, inr: f32, outr: f32, icol: Color, ocol: Color) -> NVGpaint {
		unsafe { ffi::nvgRadialGradient(self.ptr, cx, cy, inr, outr, icol, ocol) }
	}
    pub fn image_pattern(&self, ox: f32, oy: f32, ex: f32, ey: f32, angle: f32, image: i32, repeat: PatternRepeat, alpha: f32) -> NVGpaint {
		unsafe { ffi::nvgImagePattern(self.ptr, ox, oy, ex, ey, angle, image, repeat as i32, alpha) }
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
		unsafe { ffi::nvgPathWinding(self.ptr, dir as i32) }
	}

    pub fn arc(&self, cx: f32, cy: f32, r: f32, a0: f32, a1: f32, dir: Winding) {
		unsafe { ffi::nvgArc(self.ptr, cx, cy, r, a0, a1, dir as i32) }
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

    pub fn create_font(&self, name: &str, filename: &str) -> i32 {
        name.with_c_str(|name| {
            filename.with_c_str(|filename| {
		      unsafe { ffi::nvgCreateFont(self.ptr, name, filename) }
            })
        })
	}
    pub fn create_font_mem(&self, name: &str, data: *mut u8, ndata: i32, freeData: bool) -> i32 {
        name.with_c_str(|name| {
            unsafe { ffi::nvgCreateFontMem(self.ptr, name, data, ndata, if freeData {1} else {0}) }
        })
	}
    pub fn find_font(&self, name: &str) -> i32 {
        name.with_c_str(|name| {
            unsafe { ffi::nvgFindFont(self.ptr, name) }
        })
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
    pub fn text_line_height(&self, lineHeight: f32) {
		unsafe { ffi::nvgTextLineHeight(self.ptr, lineHeight) }
	}
    pub fn text_align(&self, align: Align) {
		unsafe { ffi::nvgTextAlign(self.ptr, align.bits) }
	}
    pub fn font_face_id(&self, font: i32) {
		unsafe { ffi::nvgFontFaceId(self.ptr, font) }
	}
    pub fn font_face(&self, font: &str) {
        font.with_c_str(|font| {
		unsafe { ffi::nvgFontFace(self.ptr, font) }
        })
	}
    pub fn text(&self, x: f32, y: f32, text: &str) -> f32 {
        text.with_c_str(|text| {
            unsafe { ffi::nvgText(self.ptr, x, y, text, ptr::null()) }
        })
    }
    //pub fn text(&self, x: f32, y: f32, text: &str, end: &str) -> f32 {
    //    text.with_c_str(|text| {
    //        end.with_c_str(|end| {
    //            unsafe { ffi::nvgText(self.ptr, x, y, text, end) }
    //        })
    //    })
    //}
    pub fn text_box(&self, x: f32, y: f32, breakRowWidth: f32, text: &str) {
        text.with_c_str(|text| {
             unsafe { ffi::nvgTextBox(self.ptr, x, y, breakRowWidth, text, ptr::null()) }
        })
	}
    pub fn text_bounds(&self, x: f32, y: f32, text: &str, bounds: *mut f32) -> f32 {
        text.with_c_str(|text| {
    	   unsafe { ffi::nvgTextBounds(self.ptr, x, y, text, ptr::null(), bounds) }
        })
	}
    pub fn text_box_bounds(&self, x: f32, y: f32, breakRowWidth: f32, text: &str, bounds: *mut f32) {
        text.with_c_str(|text| {
            unsafe { ffi::nvgTextBoxBounds(self.ptr, x, y, breakRowWidth, text, ptr::null(), bounds) }
        })
	}

    //////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn text_glyph_positions(&self, x: f32, y: f32, text: &str, positions: *mut NVGglyphPosition, maxPositions: i32) -> i32 {
        let st: *const u8 = &text[0];
        let en: *const u8 = unsafe { st.offset(text.len() as int) };
        text.with_c_str(|text| {
            unsafe { ffi::nvgTextGlyphPositions(self.ptr, x, y, st as *i8, en as *i8, positions, maxPositions) }
        })
	}
    pub fn text_break_lines(&self, text: &str, breakRowWidth: f32, rows: *mut NVGtextRow, maxRows: i32) -> i32 {
        if text.len() == 0 { return 0; }
        let st: *const u8 = &text[0];
        let en: *const u8 = unsafe { st.offset(text.len() as int) };
        //let en: *const u8 = &text[text.len()];
        text.with_c_str(|text| {
            unsafe { ffi::nvgTextBreakLines(self.ptr, st as *i8, en as *i8, breakRowWidth, rows, maxRows) }
        })
	}
    //////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn text_metrics(&self, ascender: *mut f32, descender: *mut f32, lineh: *mut f32) {
        unsafe { ffi::nvgTextMetrics(self.ptr, ascender, descender, lineh) }
    }



    pub fn debug_dump_path_cache(&self) {
        unsafe { ffi::nvgDebugDumpPathCache(self.ptr) }
    }

}

pub fn relative_index(text: &str, p: *const i8) -> uint {
    let st: *const u8 = &text[0];
    let stix: uint = st.to_uint();
    let pix: uint = p.to_uint();
    assert!(pix >= stix);
    pix - stix
}


// use [f32, ..6], or wrap that in a Transform impl

pub fn transform_identity(dst: *mut f32) {
	unsafe { ffi::nvgTransformIdentity(dst) }
}
pub fn transform_translate(dst: *mut f32, tx: f32, ty: f32) {
	unsafe { ffi::nvgTransformTranslate(dst, tx, ty) }
}
pub fn transform_scale(dst: *mut f32, sx: f32, sy: f32) {
	unsafe { ffi::nvgTransformScale(dst, sx, sy) }
}
pub fn transform_rotate(dst: *mut f32, a: f32) {
	unsafe { ffi::nvgTransformRotate(dst, a) }
}
pub fn transform_skew_x(dst: *mut f32, a: f32) {
	unsafe { ffi::nvgTransformSkewX(dst, a) }
}
pub fn transform_skew_y(dst: *mut f32, a: f32) {
	unsafe { ffi::nvgTransformSkewY(dst, a) }
}
pub fn transform_multiply(dst: *mut f32, src: *const f32) {
	unsafe { ffi::nvgTransformMultiply(dst, src) }
}
pub fn transform_premultiply(dst: *mut f32, src: *const f32) {
	unsafe { ffi::nvgTransformPremultiply(dst, src) }
}
pub fn transform_inverse(dst: *mut f32, src: *const f32) -> i32 {
	unsafe { ffi::nvgTransformInverse(dst, src) }
}
pub fn transform_point(dstx: *mut f32, dsty: *mut f32, xform: *const f32, srcx: f32, srcy: f32) {
	unsafe { ffi::nvgTransformPoint(dstx, dsty, xform, srcx, srcy) }
}

pub fn deg_to_rad(deg: f32) -> f32 {
	unsafe { ffi::nvgDegToRad(deg) }
}
pub fn rad_to_deg(rad: f32) -> f32 {
	unsafe { ffi::nvgRadToDeg(rad) }
}

//pub fn create_internal(params: *mut NVGparams) -> *mut Ctx {
//	unsafe { ffi::nvgCreateInternal(params) }
//}
//pub fn delete_internal(ctx: *mut Ctx) {
//	unsafe { ffi::nvgDeleteInternal(ctx) }
//}
//pub fn internal_params(ctx: *mut Ctx) -> *mut NVGparams {
//	unsafe { ffi::nvgInternalParams(ctx) }
//}


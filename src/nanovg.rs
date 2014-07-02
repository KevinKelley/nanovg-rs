#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_id = "github.com/KevinKelley/nanovg-rs#nanovg:0.1"]
#![comment = "Binding for NanoVG vector-graphics library"]

#![warn(non_camel_case_types)]
#![deny(unnecessary_parens)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
//#![warn(missing_doc)] // FIXME: should be denied.
#![deny(unused_result)]
#![deny(unnecessary_typecast)]
#![warn(visible_private_types)] // FIXME: should be denied.
//#![feature(globs)]
//#![feature(macro_rules)]
//#![feature(managed_boxes)]
//#![feature(unsafe_destructor)]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

extern crate libc;

use std::kinds::marker;

use libc::{c_double, c_float, c_int};
use libc::{c_uint, c_ushort, c_void};

pub struct NVGcontext {
    x: *const ::libc::c_void,
    no_send: marker::NoSend,
    no_share: marker::NoShare,
}

#[repr(C)]
pub struct NVGcolor {
    pub r: ::libc::c_float,
    pub g: ::libc::c_float,
    pub b: ::libc::c_float,
    pub a: ::libc::c_float,

}

#[repr(C)]
pub struct Union_Unnamed1 {
    pub data: [u32, ..4u],
}

impl Union_Unnamed1 {
    pub fn rgba(&mut self) -> *mut [::libc::c_float, ..4u] {
        unsafe { ::std::mem::transmute(self) }
    }
}
#[repr(C)]
pub struct Unnamed2 {
    pub r: ::libc::c_float,
    pub g: ::libc::c_float,
    pub b: ::libc::c_float,
    pub a: ::libc::c_float,
}

#[repr(C)]
pub struct NVGpaint {
    pub xform: [::libc::c_float, ..6u],
    pub extent: [::libc::c_float, ..2u],
    pub radius: ::libc::c_float,
    pub feather: ::libc::c_float,
    pub innerColor: NVGcolor,
    pub outerColor: NVGcolor,
    pub image: ::libc::c_int,
    pub repeat: ::libc::c_int,
}
pub type Enum_NVGwinding = ::libc::c_uint;
pub static NVG_CCW: ::libc::c_uint = 1;
pub static NVG_CW: ::libc::c_uint = 2;
pub type Enum_NVGsolidity = ::libc::c_uint;
pub static NVG_SOLID: ::libc::c_uint = 1;
pub static NVG_HOLE: ::libc::c_uint = 2;
pub type Enum_NVGlineCap = ::libc::c_uint;
pub static NVG_BUTT: ::libc::c_uint = 0;
pub static NVG_ROUND: ::libc::c_uint = 1;
pub static NVG_SQUARE: ::libc::c_uint = 2;
pub static NVG_BEVEL: ::libc::c_uint = 3;
pub static NVG_MITER: ::libc::c_uint = 4;
pub type Enum_NVGpatternRepeat = ::libc::c_uint;
pub static NVG_NOREPEAT: ::libc::c_uint = 0;
pub static NVG_REPEATX: ::libc::c_uint = 1;
pub static NVG_REPEATY: ::libc::c_uint = 2;
pub type Enum_NVGalign = ::libc::c_uint;
pub static NVG_ALIGN_LEFT: ::libc::c_uint = 1;
pub static NVG_ALIGN_CENTER: ::libc::c_uint = 2;
pub static NVG_ALIGN_RIGHT: ::libc::c_uint = 4;
pub static NVG_ALIGN_TOP: ::libc::c_uint = 8;
pub static NVG_ALIGN_MIDDLE: ::libc::c_uint = 16;
pub static NVG_ALIGN_BOTTOM: ::libc::c_uint = 32;
pub static NVG_ALIGN_BASELINE: ::libc::c_uint = 64;
#[repr(C)]
pub struct NVGglyphPosition {
    pub _str: *const ::libc::c_char,
    pub x: ::libc::c_float,
    pub minx: ::libc::c_float,
    pub maxx: ::libc::c_float,
}
#[repr(C)]
pub struct NVGtextRow {
    pub start: *const ::libc::c_char,
    pub end: *const ::libc::c_char,
    pub next: *const ::libc::c_char,
    pub width: ::libc::c_float,
    pub minx: ::libc::c_float,
    pub maxx: ::libc::c_float,
}

pub type Enum_NVGtexture = ::libc::c_uint;
pub static NVG_TEXTURE_ALPHA: ::libc::c_uint = 1;
pub static NVG_TEXTURE_RGBA: ::libc::c_uint = 2;
#[repr(C)]
pub struct NVGscissor {
    pub xform: [::libc::c_float, ..6u],
    pub extent: [::libc::c_float, ..2u],
}
#[repr(C)]
pub struct NVGvertex {
    pub x: ::libc::c_float,
    pub y: ::libc::c_float,
    pub u: ::libc::c_float,
    pub v: ::libc::c_float,
}
#[repr(C)]
pub struct NVGpath {
    pub first: ::libc::c_int,
    pub count: ::libc::c_int,
    pub closed: ::libc::c_uchar,
    pub nbevel: ::libc::c_int,
    pub fill: *mut NVGvertex,
    pub nfill: ::libc::c_int,
    pub stroke: *mut NVGvertex,
    pub nstroke: ::libc::c_int,
    pub winding: ::libc::c_int,
    pub convex: ::libc::c_int,
}
#[repr(C)]
pub struct NVGparams {
    pub userPtr: *mut ::libc::c_void,
    pub edgeAntiAlias: ::libc::c_int,
    pub renderCreate: ::std::option::Option<extern "C" fn
                                                (arg1: *mut ::libc::c_void)
                                                -> ::libc::c_int>,
    pub renderCreateTexture: ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *mut ::libc::c_void,
                                                        arg2: ::libc::c_int,
                                                        arg3: ::libc::c_int,
                                                        arg4: ::libc::c_int,
                                                        arg5:
                                                            *const ::libc::c_uchar)
                                                       -> ::libc::c_int>,
    pub renderDeleteTexture: ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *mut ::libc::c_void,
                                                        arg2: ::libc::c_int)
                                                       -> ::libc::c_int>,
    pub renderUpdateTexture: ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *mut ::libc::c_void,
                                                        arg2: ::libc::c_int,
                                                        arg3: ::libc::c_int,
                                                        arg4: ::libc::c_int,
                                                        arg5: ::libc::c_int,
                                                        arg6: ::libc::c_int,
                                                        arg7:
                                                            *const ::libc::c_uchar)
                                                       -> ::libc::c_int>,
    pub renderGetTextureSize: ::std::option::Option<extern "C" fn
                                                        (arg1:
                                                             *mut ::libc::c_void,
                                                         arg2: ::libc::c_int,
                                                         arg3:
                                                             *mut ::libc::c_int,
                                                         arg4:
                                                             *mut ::libc::c_int)
                                                        -> ::libc::c_int>,
    pub renderViewport: ::std::option::Option<extern "C" fn
                                                  (arg1: *mut ::libc::c_void,
                                                   arg2: ::libc::c_int,
                                                   arg3: ::libc::c_int)>,
    pub renderFlush: ::std::option::Option<extern "C" fn
                                               (arg1: *mut ::libc::c_void)>,
    pub renderFill: ::std::option::Option<extern "C" fn
                                              (arg1: *mut ::libc::c_void,
                                               arg2: *mut NVGpaint,
                                               arg3: *mut NVGscissor,
                                               arg4: ::libc::c_float,
                                               arg5: *const ::libc::c_float,
                                               arg6: *const NVGpath,
                                               arg7: ::libc::c_int)>,
    pub renderStroke: ::std::option::Option<extern "C" fn
                                                (arg1: *mut ::libc::c_void,
                                                 arg2: *mut NVGpaint,
                                                 arg3: *mut NVGscissor,
                                                 arg4: ::libc::c_float,
                                                 arg5: ::libc::c_float,
                                                 arg6: *const NVGpath,
                                                 arg7: ::libc::c_int)>,
    pub renderTriangles: ::std::option::Option<extern "C" fn
                                                   (arg1: *mut ::libc::c_void,
                                                    arg2:
                                                        *mut NVGpaint,
                                                    arg3:
                                                        *mut NVGscissor,
                                                    arg4: *const NVGvertex,
                                                    arg5: ::libc::c_int)>,
    pub renderDelete: ::std::option::Option<extern "C" fn
                                                (arg1: *mut ::libc::c_void)>,
}

#[link(name = "nanovg")]
extern "C" {
    pub fn nvgBeginFrame(ctx: *mut NVGcontext,
                         windowWidth: ::libc::c_int,
                         windowHeight: ::libc::c_int,
                         devicePixelRatio: ::libc::c_float);
    pub fn nvgEndFrame(ctx: *mut NVGcontext);
    pub fn nvgRGB(r: ::libc::c_uchar, g: ::libc::c_uchar, b: ::libc::c_uchar)
     -> NVGcolor;
    pub fn nvgRGBf(r: ::libc::c_float, g: ::libc::c_float, b: ::libc::c_float)
     -> NVGcolor;
    pub fn nvgRGBA(r: ::libc::c_uchar, g: ::libc::c_uchar, b: ::libc::c_uchar,
                   a: ::libc::c_uchar) -> NVGcolor;
    pub fn nvgRGBAf(r: ::libc::c_float, g: ::libc::c_float,
                    b: ::libc::c_float, a: ::libc::c_float) ->
     NVGcolor;
    pub fn nvgLerpRGBA(c0: NVGcolor, c1: NVGcolor,
                       u: ::libc::c_float) -> NVGcolor;
    pub fn nvgTransRGBA(c0: NVGcolor, a: ::libc::c_uchar) ->
     NVGcolor;
    pub fn nvgTransRGBAf(c0: NVGcolor, a: ::libc::c_float) ->
     NVGcolor;
    pub fn nvgHSL(h: ::libc::c_float, s: ::libc::c_float, l: ::libc::c_float)
     -> NVGcolor;
    pub fn nvgHSLA(h: ::libc::c_float, s: ::libc::c_float, l: ::libc::c_float,
                   a: ::libc::c_uchar) -> NVGcolor;
    pub fn nvgSave(ctx: *mut NVGcontext);
    pub fn nvgRestore(ctx: *mut NVGcontext);
    pub fn nvgReset(ctx: *mut NVGcontext);
    pub fn nvgStrokeColor(ctx: *mut NVGcontext,
                          color: NVGcolor);
    pub fn nvgStrokePaint(ctx: *mut NVGcontext,
                          paint: NVGpaint);
    pub fn nvgFillColor(ctx: *mut NVGcontext, color: NVGcolor);
    pub fn nvgFillPaint(ctx: *mut NVGcontext, paint: NVGpaint);
    pub fn nvgMiterLimit(ctx: *mut NVGcontext, limit: ::libc::c_float);
    pub fn nvgStrokeWidth(ctx: *mut NVGcontext, size: ::libc::c_float);
    pub fn nvgLineCap(ctx: *mut NVGcontext, cap: ::libc::c_int);
    pub fn nvgLineJoin(ctx: *mut NVGcontext, join: ::libc::c_int);
    pub fn nvgGlobalAlpha(ctx: *mut NVGcontext,
                          alpha: ::libc::c_float);
    pub fn nvgResetTransform(ctx: *mut NVGcontext);
    pub fn nvgTransform(ctx: *mut NVGcontext, a: ::libc::c_float,
                        b: ::libc::c_float, c: ::libc::c_float,
                        d: ::libc::c_float, e: ::libc::c_float,
                        f: ::libc::c_float);
    pub fn nvgTranslate(ctx: *mut NVGcontext, x: ::libc::c_float,
                        y: ::libc::c_float);
    pub fn nvgRotate(ctx: *mut NVGcontext, angle: ::libc::c_float);
    pub fn nvgSkewX(ctx: *mut NVGcontext, angle: ::libc::c_float);
    pub fn nvgSkewY(ctx: *mut NVGcontext, angle: ::libc::c_float);
    pub fn nvgScale(ctx: *mut NVGcontext, x: ::libc::c_float,
                    y: ::libc::c_float);
    pub fn nvgCurrentTransform(ctx: *mut NVGcontext,
                               xform: *mut ::libc::c_float);
    pub fn nvgTransformIdentity(dst: *mut ::libc::c_float);
    pub fn nvgTransformTranslate(dst: *mut ::libc::c_float,
                                 tx: ::libc::c_float, ty: ::libc::c_float);
    pub fn nvgTransformScale(dst: *mut ::libc::c_float, sx: ::libc::c_float,
                             sy: ::libc::c_float);
    pub fn nvgTransformRotate(dst: *mut ::libc::c_float, a: ::libc::c_float);
    pub fn nvgTransformSkewX(dst: *mut ::libc::c_float, a: ::libc::c_float);
    pub fn nvgTransformSkewY(dst: *mut ::libc::c_float, a: ::libc::c_float);
    pub fn nvgTransformMultiply(dst: *mut ::libc::c_float,
                                src: *const ::libc::c_float);
    pub fn nvgTransformPremultiply(dst: *mut ::libc::c_float,
                                   src: *const ::libc::c_float);
    pub fn nvgTransformInverse(dst: *mut ::libc::c_float,
                               src: *const ::libc::c_float) -> ::libc::c_int;
    pub fn nvgTransformPoint(dstx: *mut ::libc::c_float,
                             dsty: *mut ::libc::c_float,
                             xform: *const ::libc::c_float, srcx: ::libc::c_float,
                             srcy: ::libc::c_float);
    pub fn nvgDegToRad(deg: ::libc::c_float) -> ::libc::c_float;
    pub fn nvgRadToDeg(rad: ::libc::c_float) -> ::libc::c_float;
    pub fn nvgCreateImage(ctx: *mut NVGcontext,
                          filename: *const ::libc::c_char) -> ::libc::c_int;
    pub fn nvgCreateImageMem(ctx: *mut NVGcontext,
                             data: *mut ::libc::c_uchar, ndata: ::libc::c_int)
     -> ::libc::c_int;
    pub fn nvgCreateImageRGBA(ctx: *mut NVGcontext, w: ::libc::c_int,
                              h: ::libc::c_int, data: *const ::libc::c_uchar) ->
     ::libc::c_int;
    pub fn nvgUpdateImage(ctx: *mut NVGcontext, image: ::libc::c_int,
                          data: *const ::libc::c_uchar);
    pub fn nvgImageSize(ctx: *mut NVGcontext, image: ::libc::c_int,
                        w: *mut ::libc::c_int, h: *mut ::libc::c_int);
    pub fn nvgDeleteImage(ctx: *mut NVGcontext, image: ::libc::c_int);
    pub fn nvgLinearGradient(ctx: *mut NVGcontext, sx: ::libc::c_float,
                             sy: ::libc::c_float, ex: ::libc::c_float,
                             ey: ::libc::c_float, icol: NVGcolor,
                             ocol: NVGcolor) -> NVGpaint;
    pub fn nvgBoxGradient(ctx: *mut NVGcontext, x: ::libc::c_float,
                          y: ::libc::c_float, w: ::libc::c_float,
                          h: ::libc::c_float, r: ::libc::c_float,
                          f: ::libc::c_float, icol: NVGcolor,
                          ocol: NVGcolor) -> NVGpaint;
    pub fn nvgRadialGradient(ctx: *mut NVGcontext, cx: ::libc::c_float,
                             cy: ::libc::c_float, inr: ::libc::c_float,
                             outr: ::libc::c_float, icol: NVGcolor,
                             ocol: NVGcolor) -> NVGpaint;
    pub fn nvgImagePattern(ctx: *mut NVGcontext, ox: ::libc::c_float,
                           oy: ::libc::c_float, ex: ::libc::c_float,
                           ey: ::libc::c_float, angle: ::libc::c_float,
                           image: ::libc::c_int, repeat: ::libc::c_int,
                           alpha: ::libc::c_float) -> NVGpaint;
    pub fn nvgScissor(ctx: *mut NVGcontext, x: ::libc::c_float,
                      y: ::libc::c_float, w: ::libc::c_float,
                      h: ::libc::c_float);
    pub fn nvgResetScissor(ctx: *mut NVGcontext);
    pub fn nvgBeginPath(ctx: *mut NVGcontext);
    pub fn nvgMoveTo(ctx: *mut NVGcontext, x: ::libc::c_float,
                     y: ::libc::c_float);
    pub fn nvgLineTo(ctx: *mut NVGcontext, x: ::libc::c_float,
                     y: ::libc::c_float);
    pub fn nvgBezierTo(ctx: *mut NVGcontext, c1x: ::libc::c_float,
                       c1y: ::libc::c_float, c2x: ::libc::c_float,
                       c2y: ::libc::c_float, x: ::libc::c_float,
                       y: ::libc::c_float);
    pub fn nvgQuadTo(ctx: *mut NVGcontext, cx: ::libc::c_float,
                     cy: ::libc::c_float, x: ::libc::c_float,
                     y: ::libc::c_float);
    pub fn nvgArcTo(ctx: *mut NVGcontext, x1: ::libc::c_float,
                    y1: ::libc::c_float, x2: ::libc::c_float,
                    y2: ::libc::c_float, radius: ::libc::c_float);
    pub fn nvgClosePath(ctx: *mut NVGcontext);
    pub fn nvgPathWinding(ctx: *mut NVGcontext, dir: ::libc::c_int);
    pub fn nvgArc(ctx: *mut NVGcontext, cx: ::libc::c_float,
                  cy: ::libc::c_float, r: ::libc::c_float,
                  a0: ::libc::c_float, a1: ::libc::c_float,
                  dir: ::libc::c_int);
    pub fn nvgRect(ctx: *mut NVGcontext, x: ::libc::c_float,
                   y: ::libc::c_float, w: ::libc::c_float,
                   h: ::libc::c_float);
    pub fn nvgRoundedRect(ctx: *mut NVGcontext, x: ::libc::c_float,
                          y: ::libc::c_float, w: ::libc::c_float,
                          h: ::libc::c_float, r: ::libc::c_float);
    pub fn nvgEllipse(ctx: *mut NVGcontext, cx: ::libc::c_float,
                      cy: ::libc::c_float, rx: ::libc::c_float,
                      ry: ::libc::c_float);
    pub fn nvgCircle(ctx: *mut NVGcontext, cx: ::libc::c_float,
                     cy: ::libc::c_float, r: ::libc::c_float);
    pub fn nvgFill(ctx: *mut NVGcontext);
    pub fn nvgStroke(ctx: *mut NVGcontext);
    pub fn nvgCreateFont(ctx: *mut NVGcontext, name: *const ::libc::c_char,
                         filename: *const ::libc::c_char) -> ::libc::c_int;
    pub fn nvgCreateFontMem(ctx: *mut NVGcontext,
                            name: *const ::libc::c_char, data: *mut ::libc::c_uchar,
                            ndata: ::libc::c_int, freeData: ::libc::c_int) ->
     ::libc::c_int;
    pub fn nvgFindFont(ctx: *mut NVGcontext, name: *const ::libc::c_char) ->
     ::libc::c_int;
    pub fn nvgFontSize(ctx: *mut NVGcontext, size: ::libc::c_float);
    pub fn nvgFontBlur(ctx: *mut NVGcontext, blur: ::libc::c_float);
    pub fn nvgTextLetterSpacing(ctx: *mut NVGcontext,
                                spacing: ::libc::c_float);
    pub fn nvgTextLineHeight(ctx: *mut NVGcontext,
                             lineHeight: ::libc::c_float);
    pub fn nvgTextAlign(ctx: *mut NVGcontext, align: ::libc::c_int);
    pub fn nvgFontFaceId(ctx: *mut NVGcontext, font: ::libc::c_int);
    pub fn nvgFontFace(ctx: *mut NVGcontext, font: *const ::libc::c_char);
    pub fn nvgText(ctx: *mut NVGcontext, x: ::libc::c_float,
                   y: ::libc::c_float, string: *const ::libc::c_char,
                   end: *const ::libc::c_char) -> ::libc::c_float;
    pub fn nvgTextBox(ctx: *mut NVGcontext, x: ::libc::c_float,
                      y: ::libc::c_float, breakRowWidth: ::libc::c_float,
                      string: *const ::libc::c_char, end: *const ::libc::c_char);
    pub fn nvgTextBounds(ctx: *mut NVGcontext, x: ::libc::c_float,
                         y: ::libc::c_float, string: *const ::libc::c_char,
                         end: *const ::libc::c_char, bounds: *mut ::libc::c_float)
     -> ::libc::c_float;
    pub fn nvgTextBoxBounds(ctx: *mut NVGcontext, x: ::libc::c_float,
                            y: ::libc::c_float,
                            breakRowWidth: ::libc::c_float,
                            string: *const ::libc::c_char, end: *const ::libc::c_char,
                            bounds: *mut ::libc::c_float);
    pub fn nvgTextGlyphPositions(ctx: *mut NVGcontext,
                                 x: ::libc::c_float, y: ::libc::c_float,
                                 string: *const ::libc::c_char,
                                 end: *const ::libc::c_char,
                                 positions: *mut NVGglyphPosition,
                                 maxPositions: ::libc::c_int) ->
     ::libc::c_int;
    pub fn nvgTextMetrics(ctx: *mut NVGcontext,
                          ascender: *mut ::libc::c_float,
                          descender: *mut ::libc::c_float,
                          lineh: *mut ::libc::c_float);
    pub fn nvgTextBreakLines(ctx: *mut NVGcontext,
                             string: *const ::libc::c_char, end: *const ::libc::c_char,
                             breakRowWidth: ::libc::c_float,
                             rows: *mut NVGtextRow,
                             maxRows: ::libc::c_int) -> ::libc::c_int;
    pub fn nvgCreateInternal(params: *mut NVGparams) ->
     *mut NVGcontext;
    pub fn nvgDeleteInternal(ctx: *mut NVGcontext);
    pub fn nvgInternalParams(ctx: *mut NVGcontext) ->
     *mut NVGparams;
    pub fn nvgDebugDumpPathCache(ctx: *mut NVGcontext);



//// Creates NanoVG contexts for different OpenGL (ES) versions.
//// Flags should be combination of the create flags above.
//
//#if defined NANOVG_GL2
//
//struct NVGcontext* nvgCreateGL2(int flags);
//void nvgDeleteGL2(struct NVGcontext* ctx);
//
//#endif
//
//#if defined NANOVG_GL3
//
//struct NVGcontext* nvgCreateGL3(int flags);
pub fn nvgCreateGL3(flags: c_uint) -> *mut NVGcontext;

//void nvgDeleteGL3(struct NVGcontext* ctx);
pub fn nvgDeleteGL3(ctx: *mut NVGcontext);
//
//#endif
//
//#if defined NANOVG_GLES2
//
//struct NVGcontext* nvgCreateGLES2(int flags);
//void nvgDeleteGLES2(struct NVGcontext* ctx);
//
//#endif
//
//#if defined NANOVG_GLES3
//
//struct NVGcontext* nvgCreateGLES3(int flags);
//pub fn nvgCreateGLES3(flags: c_uint) -> *mut NVGcontext;
//
//void nvgDeleteGLES3(struct NVGcontext* ctx);
//pub fn nvgDeleteGLES3(ctx: *mut NVGcontext);
//
//#endif
}


//type Enum_NVGflags = ::libc::c_uint;
// Flag indicating if geoemtry based anti-aliasing is used (may not be needed when using MSAA).
pub static NVG_ANTIALIAS: ::libc::c_uint = 1;
// Flag indicating if strokes should be drawn using stencil buffer. The rendering will be a little
// slower, but path overlaps (i.e. self-intersecting or sharp turns) will be drawn just once.
pub static NVG_STENCIL_STROKES: ::libc::c_uint = 2;


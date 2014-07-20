
use libc::{c_double, c_float, c_int, c_char};
use libc::{c_uint, c_ushort, c_uchar, c_void};
use std::kinds::marker;


pub type Enum_NVGwinding = c_uint;
pub static NVG_CCW: c_uint = 1;
pub static NVG_CW: c_uint = 2;
pub type Enum_NVGsolidity = c_uint;
pub static NVG_SOLID: c_uint = 1;
pub static NVG_HOLE: c_uint = 2;
pub type Enum_NVGlineCap = c_uint;
pub static NVG_BUTT: c_uint = 0;
pub static NVG_ROUND: c_uint = 1;
pub static NVG_SQUARE: c_uint = 2;
pub static NVG_BEVEL: c_uint = 3;
pub static NVG_MITER: c_uint = 4;
pub type Enum_NVGpatternRepeat = c_uint;
pub static NVG_NOREPEAT: c_uint = 0;
pub static NVG_REPEATX: c_uint = 1;
pub static NVG_REPEATY: c_uint = 2;
pub type Enum_NVGalign = c_uint;
pub static NVG_ALIGN_LEFT: c_uint = 1;
pub static NVG_ALIGN_CENTER: c_uint = 2;
pub static NVG_ALIGN_RIGHT: c_uint = 4;
pub static NVG_ALIGN_TOP: c_uint = 8;
pub static NVG_ALIGN_MIDDLE: c_uint = 16;
pub static NVG_ALIGN_BOTTOM: c_uint = 32;
pub static NVG_ALIGN_BASELINE: c_uint = 64;


//type Enum_NVGcreateflags = c_uint;
// Flag indicating if geoemtry based anti-aliasing is used (may not be needed when using MSAA).
pub static NVG_ANTIALIAS: c_uint = 1;
// Flag indicating if strokes should be drawn using stencil buffer. The rendering will be a little
// slower, but path overlaps (i.e. self-intersecting or sharp turns) will be drawn just once.
pub static NVG_STENCIL_STROKES: c_uint = 2;


pub enum NVGcontext {}

#[repr(C)]
pub struct NVGcolor {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float
}
//impl NVGcolor {
//    pub fn rgb(r: u8, g: u8, b: u8) -> NVGcolor {
//        unsafe { nvgRGB(r, g, b) }
//    }
//    pub fn rgb_f(r: f32, g: f32, b: f32) -> NVGcolor {
//        unsafe { nvgRGBf(r, g, b) }
//    }
//    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> NVGcolor {
//        unsafe { nvgRGBA(r, g, b, a) }
//    }
//    pub fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> NVGcolor {
//        unsafe { nvgRGBAf(r, g, b, a) }
//    }
//    pub fn lerp_rgba(c0: NVGcolor, c1: NVGcolor, u: f32) -> NVGcolor {
//        unsafe { nvgLerpRGBA(c0, c1, u) }
//    }
//    pub fn trans_rgba(c0: NVGcolor, a: u8) -> NVGcolor {
//        unsafe { nvgTransRGBA(c0, a) }
//    }
//    pub fn trans_rgba_f(c0: NVGcolor, a: f32) -> NVGcolor {
//        unsafe { nvgTransRGBAf(c0, a) }
//    }
//    pub fn hsl(h: f32, s: f32, l: f32) -> NVGcolor {
//        unsafe { nvgHSL(h, s, l) }
//    }
//    pub fn hsla(h: f32, s: f32, l: f32, a: u8) -> NVGcolor {
//        unsafe {  nvgHSLA(h,s,l, a) }
//    }
//}
//#[repr(C)]
//pub struct Union_Unnamed1 {
//    pub data: [u32, ..4u],
//}
//impl Union_Unnamed1 {
//    pub fn rgba(&mut self) -> *mut [c_float, ..4u] {
//        unsafe { ::std::mem::transmute(self) }
//    }
//}


#[repr(C)]
pub struct NVGpaint {
    pub xform: [c_float, ..6u],
    pub extent: [c_float, ..2u],
    pub radius: c_float,
    pub feather: c_float,
    pub innerColor: NVGcolor,
    pub outerColor: NVGcolor,
    pub image: c_int,
    pub repeat: c_int,
}

#[repr(C)]
pub struct NVGglyphPosition {
    pub _str: *const c_char,
    pub x: c_float,
    pub minx: c_float,
    pub maxx: c_float,
}
#[repr(C)]
pub struct NVGtextRow {
    pub start: *const c_char,
    pub end: *const c_char,
    pub next: *const c_char,
    pub width: c_float,
    pub minx: c_float,
    pub maxx: c_float,
}

pub type Enum_NVGtexture = c_uint;
pub static NVG_TEXTURE_ALPHA: c_uint = 1;
pub static NVG_TEXTURE_RGBA: c_uint = 2;
#[repr(C)]
pub struct NVGscissor {
    pub xform: [c_float, ..6u],
    pub extent: [c_float, ..2u],
}
#[repr(C)]
pub struct NVGvertex {
    pub x: c_float,
    pub y: c_float,
    pub u: c_float,
    pub v: c_float,
}
#[repr(C)]
pub struct NVGpath {
    pub first: c_int,
    pub count: c_int,
    pub closed: c_uchar,
    pub nbevel: c_int,
    pub fill: *mut NVGvertex,
    pub nfill: c_int,
    pub stroke: *mut NVGvertex,
    pub nstroke: c_int,
    pub winding: c_int,
    pub convex: c_int,
}
#[repr(C)]
pub struct NVGparams {
    pub userPtr: *mut c_void,
    pub edgeAntiAlias: c_int,
    pub renderCreate: Option<extern "C" fn (arg1: *mut c_void) -> c_int>,
    pub renderCreateTexture: Option<extern "C" fn (arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *const c_uchar) -> c_int>,
    pub renderDeleteTexture: Option<extern "C" fn (arg1: *mut c_void, arg2: c_int) -> c_int>,
    pub renderUpdateTexture: Option<extern "C" fn (arg1: *mut c_void, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int, arg7: *const c_uchar) -> c_int>,
    pub renderGetTextureSize: Option<extern "C" fn (arg1: *mut c_void, arg2: c_int, arg3: *mut c_int, arg4: *mut c_int) -> c_int>,
    pub renderViewport: Option<extern "C" fn (arg1: *mut c_void, arg2: c_int, arg3: c_int)>,
    pub renderFlush: Option<extern "C" fn (arg1: *mut c_void)>,
    pub renderFill: Option<extern "C" fn (arg1: *mut c_void, arg2: *mut NVGpaint, arg3: *mut NVGscissor, arg4: c_float, arg5: *const c_float, arg6: *const NVGpath, arg7: c_int)>,
    pub renderStroke: Option<extern "C" fn (arg1: *mut c_void, arg2: *mut NVGpaint, arg3: *mut NVGscissor, arg4: c_float, arg5: c_float, arg6: *const NVGpath, arg7: c_int)>,
    pub renderTriangles: Option<extern "C" fn (arg1: *mut c_void, arg2: *mut NVGpaint, arg3: *mut NVGscissor, arg4: *const NVGvertex, arg5: c_int)>,
    pub renderDelete: Option<extern "C" fn (arg1: *mut c_void)>,
}


#[link(name = "nanovg")]
extern "C"
{
    pub fn nvgBeginFrame(ctx: *mut NVGcontext, windowWidth: c_int, windowHeight: c_int, devicePixelRatio: c_float);
    pub fn nvgEndFrame(ctx: *mut NVGcontext);

    pub fn nvgRGB(r: c_uchar, g: c_uchar, b: c_uchar) -> NVGcolor;
    pub fn nvgRGBf(r: c_float, g: c_float, b: c_float) -> NVGcolor;
    pub fn nvgRGBA(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> NVGcolor;
    pub fn nvgRGBAf(r: c_float, g: c_float, b: c_float, a: c_float) -> NVGcolor;
    pub fn nvgLerpRGBA(c0: NVGcolor, c1: NVGcolor, u: c_float) -> NVGcolor;
    pub fn nvgTransRGBA(c0: NVGcolor, a: c_uchar) -> NVGcolor;
    pub fn nvgTransRGBAf(c0: NVGcolor, a: c_float) -> NVGcolor;
    pub fn nvgHSL(h: c_float, s: c_float, l: c_float) -> NVGcolor;
    pub fn nvgHSLA(h: c_float, s: c_float, l: c_float, a: c_uchar) -> NVGcolor;

    pub fn nvgSave(ctx: *mut NVGcontext);
    pub fn nvgRestore(ctx: *mut NVGcontext);
    pub fn nvgReset(ctx: *mut NVGcontext);

    pub fn nvgStrokeColor(ctx: *mut NVGcontext, color: NVGcolor);
    pub fn nvgStrokePaint(ctx: *mut NVGcontext, paint: NVGpaint);
    pub fn nvgFillColor(ctx: *mut NVGcontext, color: NVGcolor);
    pub fn nvgFillPaint(ctx: *mut NVGcontext, paint: NVGpaint);
    pub fn nvgMiterLimit(ctx: *mut NVGcontext, limit: c_float);
    pub fn nvgStrokeWidth(ctx: *mut NVGcontext, size: c_float);
    pub fn nvgLineCap(ctx: *mut NVGcontext, cap: c_int);
    pub fn nvgLineJoin(ctx: *mut NVGcontext, join: c_int);
    pub fn nvgGlobalAlpha(ctx: *mut NVGcontext, alpha: c_float);
    pub fn nvgResetTransform(ctx: *mut NVGcontext);

    pub fn nvgTransform(ctx: *mut NVGcontext, a: c_float, b: c_float, c: c_float, d: c_float, e: c_float, f: c_float);
    pub fn nvgTranslate(ctx: *mut NVGcontext, x: c_float, y: c_float);
    pub fn nvgRotate(ctx: *mut NVGcontext, angle: c_float);
    pub fn nvgSkewX(ctx: *mut NVGcontext, angle: c_float);
    pub fn nvgSkewY(ctx: *mut NVGcontext, angle: c_float);
    pub fn nvgScale(ctx: *mut NVGcontext, x: c_float, y: c_float);
    pub fn nvgCurrentTransform(ctx: *mut NVGcontext, xform: *mut c_float);
    pub fn nvgTransformIdentity(dst: *mut c_float);
    pub fn nvgTransformTranslate(dst: *mut c_float, tx: c_float, ty: c_float);
    pub fn nvgTransformScale(dst: *mut c_float, sx: c_float, sy: c_float);
    pub fn nvgTransformRotate(dst: *mut c_float, a: c_float);
    pub fn nvgTransformSkewX(dst: *mut c_float, a: c_float);
    pub fn nvgTransformSkewY(dst: *mut c_float, a: c_float);
    pub fn nvgTransformMultiply(dst: *mut c_float, src: *const c_float);
    pub fn nvgTransformPremultiply(dst: *mut c_float, src: *const c_float);
    pub fn nvgTransformInverse(dst: *mut c_float, src: *const c_float) -> c_int;
    pub fn nvgTransformPoint(dstx: *mut c_float, dsty: *mut c_float, xform: *const c_float, srcx: c_float, srcy: c_float);

    pub fn nvgDegToRad(deg: c_float) -> c_float;
    pub fn nvgRadToDeg(rad: c_float) -> c_float;

    pub fn nvgCreateImage(ctx: *mut NVGcontext, filename: *const c_char) -> c_int;
    pub fn nvgCreateImageMem(ctx: *mut NVGcontext, data: *mut c_uchar, ndata: c_int) -> c_int;
    pub fn nvgCreateImageRGBA(ctx: *mut NVGcontext, w: c_int, h: c_int, data: *const c_uchar) -> c_int;
    pub fn nvgUpdateImage(ctx: *mut NVGcontext, image: c_int, data: *const c_uchar);
    pub fn nvgImageSize(ctx: *mut NVGcontext, image: c_int, w: *mut c_int, h: *mut c_int);
    pub fn nvgDeleteImage(ctx: *mut NVGcontext, image: c_int);

    pub fn nvgLinearGradient(ctx: *mut NVGcontext, sx: c_float, sy: c_float, ex: c_float, ey: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
    pub fn nvgBoxGradient(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float, f: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
    pub fn nvgRadialGradient(ctx: *mut NVGcontext, cx: c_float, cy: c_float, inr: c_float, outr: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
    pub fn nvgImagePattern(ctx: *mut NVGcontext, ox: c_float, oy: c_float, ex: c_float, ey: c_float, angle: c_float, image: c_int, repeat: c_int, alpha: c_float) -> NVGpaint;

    pub fn nvgScissor(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float);
    pub fn nvgResetScissor(ctx: *mut NVGcontext);

    pub fn nvgBeginPath(ctx: *mut NVGcontext);
    pub fn nvgMoveTo(ctx: *mut NVGcontext, x: c_float, y: c_float);
    pub fn nvgLineTo(ctx: *mut NVGcontext, x: c_float, y: c_float);
    pub fn nvgBezierTo(ctx: *mut NVGcontext, c1x: c_float, c1y: c_float, c2x: c_float, c2y: c_float, x: c_float, y: c_float);
    pub fn nvgQuadTo(ctx: *mut NVGcontext, cx: c_float, cy: c_float, x: c_float, y: c_float);
    pub fn nvgArcTo(ctx: *mut NVGcontext, x1: c_float, y1: c_float, x2: c_float, y2: c_float, radius: c_float);
    pub fn nvgClosePath(ctx: *mut NVGcontext);
    pub fn nvgPathWinding(ctx: *mut NVGcontext, dir: c_int);

    pub fn nvgArc(ctx: *mut NVGcontext, cx: c_float, cy: c_float, r: c_float, a0: c_float, a1: c_float, dir: c_int);
    pub fn nvgRect(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float);
    pub fn nvgRoundedRect(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float);
    pub fn nvgEllipse(ctx: *mut NVGcontext, cx: c_float, cy: c_float, rx: c_float, ry: c_float);
    pub fn nvgCircle(ctx: *mut NVGcontext, cx: c_float, cy: c_float, r: c_float);
    pub fn nvgFill(ctx: *mut NVGcontext);
    pub fn nvgStroke(ctx: *mut NVGcontext);

    pub fn nvgCreateFont(ctx: *mut NVGcontext, name: *const c_char, filename: *const c_char) -> c_int;
    pub fn nvgCreateFontMem(ctx: *mut NVGcontext, name: *const c_char, data: *mut c_uchar, ndata: c_int, freeData: c_int) -> c_int;
    pub fn nvgFindFont(ctx: *mut NVGcontext, name: *const c_char) -> c_int;
    pub fn nvgFontSize(ctx: *mut NVGcontext, size: c_float);
    pub fn nvgFontBlur(ctx: *mut NVGcontext, blur: c_float);
    pub fn nvgTextLetterSpacing(ctx: *mut NVGcontext, spacing: c_float);
    pub fn nvgTextLineHeight(ctx: *mut NVGcontext, lineHeight: c_float);
    pub fn nvgTextAlign(ctx: *mut NVGcontext, align: c_uint);
    pub fn nvgFontFaceId(ctx: *mut NVGcontext, font: c_int);
    pub fn nvgFontFace(ctx: *mut NVGcontext, font: *const c_char);
    pub fn nvgText(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char) -> c_float;
    pub fn nvgTextBox(ctx: *mut NVGcontext, x: c_float, y: c_float, breakRowWidth: c_float, string: *const c_char, end: *const c_char);
    pub fn nvgTextBounds(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char, bounds: *mut c_float) -> c_float;
    pub fn nvgTextBoxBounds(ctx: *mut NVGcontext, x: c_float, y: c_float, breakRowWidth: c_float, string: *const c_char, end: *const c_char, bounds: *mut c_float);
    pub fn nvgTextGlyphPositions(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char, positions: *mut NVGglyphPosition, maxPositions: c_int) -> c_int;
    pub fn nvgTextMetrics(ctx: *mut NVGcontext, ascender: *mut c_float, descender: *mut c_float, lineh: *mut c_float);
    pub fn nvgTextBreakLines(ctx: *mut NVGcontext, string: *const c_char, end: *const c_char, breakRowWidth: c_float, rows: *mut NVGtextRow, maxRows: c_int) -> c_int;

    pub fn nvgCreateInternal(params: *mut NVGparams) -> *mut NVGcontext;
    pub fn nvgDeleteInternal(ctx: *mut NVGcontext);
    pub fn nvgInternalParams(ctx: *mut NVGcontext) -> *mut NVGparams;
    pub fn nvgDebugDumpPathCache(ctx: *mut NVGcontext);


    // GL version specific

    //// Creates NanoVG contexts for different OpenGL (ES) versions.
    //// Flags should be combination of the create flags above.

    //#if defined NANOVG_GL2
    //struct NVGcontext* nvgCreateGL2(int flags);
    //void nvgDeleteGL2(struct NVGcontext* ctx);

    //#if defined NANOVG_GL3
    pub fn nvgCreateGL3(flags: c_uint) -> *mut NVGcontext;
    pub fn nvgDeleteGL3(ctx: *mut NVGcontext);

    //#if defined NANOVG_GLES2
    //struct NVGcontext* nvgCreateGLES2(int flags);
    //void nvgDeleteGLES2(struct NVGcontext* ctx);

    //#if defined NANOVG_GLES3
    //pub fn nvgCreateGLES3(flags: c_uint) -> *mut NVGcontext;
    //pub fn nvgDeleteGLES3(ctx: *mut NVGcontext);



    pub fn stbi_write_png(filename: *const c_char, w: c_int, h: c_int, comp: c_int, data: *const c_void, stride_in_bytes: c_int) -> c_int;
    pub fn stbi_write_bmp(filename: *const c_char, w: c_int, h: c_int, comp: c_int, data: *const c_void) -> c_int;
    pub fn stbi_write_tga(filename: *const c_char, w: c_int, h: c_int, comp: c_int, data: *const c_void) -> c_int;
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_char, c_uchar, c_int, c_float, c_void};

pub type NVGcontext = c_void;

// No reason to use a union here, since the nanovg guys
// only used it for convenience.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NVGcolor {
	pub rgba: [c_float; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NVGpaint {
	pub xform: [c_float; 6],
	pub extent: [c_float; 2],
	pub radius: c_float,
	pub feather: c_float,
	pub innerColor: NVGcolor,
	pub outerColor: NVGcolor,
	pub image: c_int,
}

bitflags! {
	pub struct NVGwinding: c_int {
		const NVG_CCW = 1;
		const NVG_CW = 2;
	}
}

bitflags! {
	pub struct NVGsolidity: c_int {
		const NVG_SOLID = 1;
		const NVG_HOLE = 2;
	}
}

bitflags! {
	pub struct NVGalign: c_int {
		const NVG_ALIGN_LEFT = 1 << 0;
		const NVG_ALIGN_CENTER = 1 << 1;
		const NVG_ALIGN_RIGHT = 1 << 2;
		const NVG_ALIGN_TOP = 1 << 3;
		const NVG_ALIGN_MIDDLE = 1 << 4;
		const NVG_ALIGN_BOTTOM = 1 << 5;
		const NVG_ALIGN_BASELINE = 1 << 6;
	}
}

bitflags! {
	pub struct NVGblendFactor: c_int {
		const NVG_ZERO = 1<<0;
		const NVG_ONE = 1<<1;
		const NVG_SRC_COLOR = 1<<2;
		const NVG_ONE_MINUS_SRC_COLOR = 1<<3;
		const NVG_DST_COLOR = 1<<4;
		const NVG_ONE_MINUS_DST_COLOR = 1<<5;
		const NVG_SRC_ALPHA = 1<<6;
		const NVG_ONE_MINUS_SRC_ALPHA = 1<<7;
		const NVG_DST_ALPHA = 1<<8;
		const NVG_ONE_MINUS_DST_ALPHA = 1<<9;
		const NVG_SRC_ALPHA_SATURATE = 1<<10;
	}
}

bitflags! {
	pub struct NVGimageFlags: c_int {
		const NVG_IMAGE_GENERATE_MIPMAPS = 1 << 0;
		const NVG_IMAGE_REPEATX = 1 << 1;
		const NVG_IMAGE_REPEATY = 1 << 2;
		const NVG_IMAGE_FLIPY = 1 << 3;
		const NVG_IMAGE_PREMULTIPLIED = 1 << 4;
		const NVG_IMAGE_NEAREST = 1 << 5;
	}
}

bitflags! {
	// #[cfg(any(feature = "gl2", feature = "gl3", feature = "gles2", feature = "gles3"))]
	pub struct NVGcreateFlags: c_int {
		const NVG_ANTIALIAS = 1 << 0;
		const NVG_STENCIL_STROKES = 1 << 1;
		const NVG_DEBUG = 1 << 2;
	}
}

#[repr(C)]
pub enum NVGlineCap {
	NVG_BUTT,
	NVG_ROUND,
	NVG_SQUARE,
	NVG_BEVEL,
	NVG_MITER,
}

#[repr(C)]
pub enum NVGcompositeOperation {
	NVG_SOURCE_OVER,
	NVG_SOURCE_IN,
	NVG_SOURCE_OUT,
	NVG_ATOP,
	NVG_DESTINATION_OVER,
	NVG_DESTINATION_IN,
	NVG_DESTINATION_OUT,
	NVG_DESTINATION_ATOP,
	NVG_LIGHTER,
	NVG_COPY,
	NVG_XOR,
}

#[repr(C)]
pub struct NVGcompositeOperationState {
	pub srcRGB: c_int,
	pub dstRGB: c_int,
	pub srcAlpha: c_int,
	pub dstAlpha: c_int,
}

#[repr(C)]
pub struct NVGglyphPosition {
	pub s: *const c_char,
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

#[link(name = "nanovg", kind = "static")]
extern {
	#[cfg(target_os = "windows")]
	pub fn gladLoadGL() -> i32;
	pub fn nvgBeginFrame(ctx: *mut NVGcontext, windowWidth: c_int, windowHeight: c_int, devicePixelRatio: c_float);
	pub fn nvgCancelFrame(ctx: *mut NVGcontext);
	pub fn nvgEndFrame(ctx: *mut NVGcontext);
	pub fn nvgGlobalCompositeOperation(ctx: *mut NVGcontext, op: c_int);
	pub fn nvgGlobalCompositeBlendFunc(ctx: *mut NVGcontext, sfactor: c_int, dfactor: c_int);
	pub fn nvgGlobalCompositeBlendFuncSeparate(ctx: *mut NVGcontext, srcRGB: c_int, dstRGB: c_int, srcAlpha: c_int, dstAlpha: c_int);
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
	pub fn nvgShapeAntiAlias(ctx: *mut NVGcontext, enabled: c_int);
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
	// Degrees <-> radians conversion functions are not exported. Use the rust builtins instead.
	pub fn nvgCreateImage(ctx: *mut NVGcontext, filename: *const c_char, imageFlags: c_int) -> c_int;
	pub fn nvgCreateImageMem(ctx: *mut NVGcontext, imageFlags: c_int, data: *mut c_uchar, ndata: c_int) -> c_int;
	pub fn nvgCreateImageRGBA(ctx: *mut NVGcontext, w: c_int, h: c_int, imageFlags: c_int, data: *const c_uchar) -> c_int;
	pub fn nvgUpdateImage(ctx: *mut NVGcontext, image: c_int, data: *const c_uchar);
	pub fn nvgImageSize(ctx: *mut NVGcontext, image: c_int, w: *mut c_int, h: *mut c_int);
	pub fn nvgDeleteImage(ctx: *mut NVGcontext, image: c_int);
	pub fn nvgLinearGradient(ctx: *mut NVGcontext, sx: c_float, sy: c_float, ex: c_float, ey: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
	pub fn nvgBoxGradient(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float, r: c_float, f: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
	pub fn nvgRadialGradient(ctx: *mut NVGcontext, cx: c_float, cy: c_float, inr: c_float, outr: c_float, icol: NVGcolor, ocol: NVGcolor) -> NVGpaint;
	pub fn nvgImagePattern(ctx: *mut NVGcontext, ox: c_float, oy: c_float, ex: c_float, ey: c_float, angle: c_float, image: c_int, alpha: c_float) -> NVGpaint;
	pub fn nvgScissor(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float);
	pub fn nvgIntersectScissor(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float);
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
	pub fn nvgRoundedRectVarying(ctx: *mut NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float, radTopLeft: c_float, radTopRight: c_float, radBottomRight: c_float, radBottomLeft: c_float);
	pub fn nvgEllipse(ctx: *mut NVGcontext, cx: c_float, cy: c_float, rx: c_float, ry: c_float);
	pub fn nvgCircle(ctx: *mut NVGcontext, cx: c_float, cy: c_float, r: c_float);
	pub fn nvgFill(ctx: *mut NVGcontext);
	pub fn nvgStroke(ctx: *mut NVGcontext);
	pub fn nvgCreateFont(ctx: *mut NVGcontext, name: *const c_char, filename: *const c_char) -> c_int;
	pub fn nvgCreateFontMem(ctx: *mut NVGcontext, name: *const c_char, data: *mut c_uchar, ndata: c_int, freeData: c_int) -> c_int;
	pub fn nvgFindFont(ctx: *mut NVGcontext, name: *const c_char) -> c_int;
	pub fn nvgAddFallbackFontId(ctx: *mut NVGcontext, baseFont: c_int, fallbackFont: c_int) -> c_int;
	pub fn nvgAddFallbackFont(ctx: *mut NVGcontext, baseFont: *const c_char, fallbackFont: *const c_char) -> c_int;
	pub fn nvgFontSize(ctx: *mut NVGcontext, size: c_float);
	pub fn nvgFontBlur(ctx: *mut NVGcontext, blur: c_float);
	pub fn nvgTextLetterSpacing(ctx: *mut NVGcontext, spacing: c_float);
	pub fn nvgTextLineHeight(ctx: *mut NVGcontext, lineHeight: c_float);
	pub fn nvgTextAlign(ctx: *mut NVGcontext, align: c_int);
	pub fn nvgFontFaceId(ctx: *mut NVGcontext, font: c_int);
	pub fn nvgFontFace(ctx: *mut NVGcontext, font: *const c_char);
	pub fn nvgText(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char) -> c_float;
	pub fn nvgTextBox(ctx: *mut NVGcontext, x: c_float, y: c_float, breakRowWidth: c_float, string: *const c_char, end: *const c_char);
	pub fn nvgTextBounds(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char, bounds: *mut c_float) -> c_float;
	pub fn nvgTextBoxBounds(ctx: *mut NVGcontext, x: c_float, y: c_float, breakRowWidth: c_float, string: *const c_char, end: *const c_char, bounds: *mut c_float);
	pub fn nvgTextGlyphPositions(ctx: *mut NVGcontext, x: c_float, y: c_float, string: *const c_char, end: *const c_char, positions: *mut NVGglyphPosition, maxPositions: c_int) -> c_int;
	pub fn nvgTextMetrics(ctx: *mut NVGcontext, ascender: *mut c_float, descender: *mut c_float, lineh: *mut c_float);
	pub fn nvgTextBreakLines(ctx: *mut NVGcontext, string: *const c_char, end: *const c_char, breakRowWidth: c_float, rows: *mut NVGtextRow, maxRows: c_int) -> c_int;
	#[cfg(feature = "gl2")]
	pub fn nvgCreateGL2(flags: c_int) -> *mut NVGcontext;
	#[cfg(feature = "gl2")]	
	pub fn nvgDeleteGL2(ctx: *mut NVGcontext);
	#[cfg(feature = "gl3")]
	pub fn nvgCreateGL3(flags: c_int) -> *mut NVGcontext;
	#[cfg(feature = "gl3")]	
	pub fn nvgDeleteGL3(ctx: *mut NVGcontext);
	#[cfg(feature = "gles2")]
	pub fn nvgCreateGLES2(flags: c_int) -> *mut NVGcontext;
	#[cfg(feature = "gles2")]	
	pub fn nvgDeleteGLES2(ctx: *mut NVGcontext);
	#[cfg(feature = "gles3")]
	pub fn nvgCreateGLES3(flags: c_int) -> *mut NVGcontext;
	#[cfg(feature = "gles3")]	
	pub fn nvgDeleteGLES3(ctx: *mut NVGcontext);
}
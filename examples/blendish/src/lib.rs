#![crate_type = "lib"]
//#![crate_type = "rlib"]
//#![crate_type = "dylib"]
#![crate_id = "github.com/KevinKelley/nanovg-rs#nanovg:0.1"]
#![comment = "Binding for NanoVG vector-graphics library"]
#![doc(html_root_url = "https://github.com/KevinKelley/nanovg-rs")]

#![feature(unsafe_destructor)]  // use Option instead
#![feature(globs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]
#![deny(unnecessary_parens)]
#![deny(non_uppercase_statics)]
#![allow(unnecessary_qualification)]
//#![warn(missing_doc)]
#![deny(unused_result)]
#![allow(unused_imports)]
#![allow(unused_attribute)]
#![deny(unnecessary_typecast)]
#![warn(visible_private_types)]
#![allow(dead_code)]


extern crate libc;
extern crate nanovg;

use libc::{c_double, c_float, c_int, c_char};
use libc::{c_uint, c_ushort, c_uchar, c_void};
use std::ptr::null;

//use nanovg::{Color};

use NVGcolor = nanovg::Color;
use NVGcontext = nanovg::Ctx;
use nanovg::{NVGglyphPosition};
use ffi::{BNDwidgetTheme};
use ffi::*;
use theme::{bnd_theme, bnd_font, bnd_icon_image};
use theme::*;

mod ffi;
mod theme;

fn fminf(a:c_float, b:c_float) -> c_float { if a<b { a } else { b } }
fn fmaxf(a:c_float, b:c_float) -> c_float { if a>b { a } else { b } }

////////////////////////////////////////////////////////////////////////////////
// High Level Functions
// --------------------
// Use these functions to draw themed widgets with your NVGcontext.

// Draw a label with its lower left origin at (x,y) and size of (w,h).
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndLabel(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, iconid: c_int, label: *const c_char
) {
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bnd_theme.regularTheme.textColor, BND_LEFT,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a tool button  with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndToolButton(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    iconid: c_int, label: *const c_char
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_TOOL_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.toolTheme, state, 1);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.toolTheme.outlineColor));
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bndTextColor(&bnd_theme.toolTheme, state), BND_CENTER,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a radio button with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndRadioButton(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    iconid: c_int, label: *const c_char
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_OPTION_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.radioTheme, state, 1);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.radioTheme.outlineColor));
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bndTextColor(&bnd_theme.radioTheme, state), BND_CENTER,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a text field with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if iconid >= 0, an icon will be added to the widget
// if text is not NULL, text will be printed to the widget
// cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
// cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
// if cend < cbegin, then no caret will be drawn
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndTextField(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    iconid: c_int, text: *const c_char, cbegin: c_int, cend: c_int
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_TEXT_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.textFieldTheme, state, 0);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.textFieldTheme.outlineColor));
    if (state != BND_ACTIVE) {
        cend = -1;
    }
    bndIconLabelCaret(ctx,x,y,w,h,iconid,
        bndTextColor(&bnd_theme.textFieldTheme, state), BND_LABEL_FONT_SIZE,
        text, bnd_theme.textFieldTheme.itemColor, cbegin, cend);
}

// Draw an option button with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndOptionButton(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
    label: *const c_char
) {
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    let ox = x;
    let oy = y+h-BND_OPTION_HEIGHT-3.0;

    bndBevelInset(ctx,ox,oy,
        BND_OPTION_WIDTH,BND_OPTION_HEIGHT,
        BND_OPTION_RADIUS,BND_OPTION_RADIUS);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.optionTheme, state, 1);
    bndInnerBox(ctx,ox,oy,
        BND_OPTION_WIDTH,BND_OPTION_HEIGHT,
        BND_OPTION_RADIUS,BND_OPTION_RADIUS,BND_OPTION_RADIUS,BND_OPTION_RADIUS,
        shade_top, shade_down);
    bndOutlineBox(ctx,ox,oy,
        BND_OPTION_WIDTH,BND_OPTION_HEIGHT,
        BND_OPTION_RADIUS,BND_OPTION_RADIUS,BND_OPTION_RADIUS,BND_OPTION_RADIUS,
        bndTransparent(bnd_theme.optionTheme.outlineColor));
    if (state == BND_ACTIVE) {
        bndCheck(ctx,ox,oy, bndTransparent(bnd_theme.optionTheme.itemColor));
    }
    bndIconLabelValue(ctx,x+12.0,y,w-12.0,h, -1,
        bndTextColor(&bnd_theme.optionTheme, state), BND_LEFT,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a choice button with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndChoiceButton(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    iconid: c_int, label: *const c_char
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_OPTION_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.choiceTheme, state, 1);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.choiceTheme.outlineColor));
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bndTextColor(&bnd_theme.choiceTheme, state), BND_LEFT,
        BND_LABEL_FONT_SIZE, label, null());
    bndUpDownArrow(ctx,x+w-10.0,y+10.0,5.0,
        bndTransparent(bnd_theme.choiceTheme.itemColor));
}

// Draw a number field with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// if label is not NULL, a label will be added to the widget
// if value is not NULL, a value will be added to the widget, along with
// a ":" separator
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndNumberField(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    label: *const c_char, value: *const c_char
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_NUMBER_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.numberFieldTheme, state, 0);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.numberFieldTheme.outlineColor));
    bndIconLabelValue(ctx,x,y,w,h,-1,
        bndTextColor(&bnd_theme.numberFieldTheme, state), BND_CENTER,
        BND_LABEL_FONT_SIZE, label, value);
    bndArrow(ctx,x+8.0,y+10.0,-BND_NUMBER_ARROW_SIZE,
        bndTransparent(bnd_theme.numberFieldTheme.itemColor));
    bndArrow(ctx,x+w-8.0,y+10.0,BND_NUMBER_ARROW_SIZE,
        bndTransparent(bnd_theme.numberFieldTheme.itemColor));
}

// Draw slider control with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags and state denotes
// the widgets current UI state.
// progress must be in the range 0..1 and controls the size of the slider bar
// if label is not NULL, a label will be added to the widget
// if value is not NULL, a value will be added to the widget, along with
// a ":" separator
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndSlider(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int, state: BNDwidgetState,
    progress: c_float, label: *const c_char, value: *const c_char
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_NUMBER_RADIUS, flags);
    bndBevelInset(ctx,x,y,w,h,cr[2],cr[3]);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.sliderTheme, state, 0);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);

    if (state == BND_ACTIVE) {
        shade_top = bndOffsetColor(
            bnd_theme.sliderTheme.itemColor, bnd_theme.sliderTheme.shadeTop);
        shade_down = bndOffsetColor(
            bnd_theme.sliderTheme.itemColor, bnd_theme.sliderTheme.shadeDown);
    } else {
        shade_top = bndOffsetColor(
            bnd_theme.sliderTheme.itemColor, bnd_theme.sliderTheme.shadeDown);
        shade_down = bndOffsetColor(
            bnd_theme.sliderTheme.itemColor, bnd_theme.sliderTheme.shadeTop);
    }
    ctx.scissor(x,y,8.0+(w-8.0)*bnd_clamp(progress,0.0,1.0),h);
    bndInnerBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    ctx.reset_scissor();

    bndOutlineBox(ctx,x,y,w,h,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.sliderTheme.outlineColor));
    bndIconLabelValue(ctx,x,y,w,h,-1,
        bndTextColor(&bnd_theme.sliderTheme, state), BND_CENTER,
        BND_LABEL_FONT_SIZE, label, value);
}

// Draw scrollbar with its lower left origin at (x,y) and size of (w,h),
// where state denotes the widgets current UI state.
// offset is in the range 0..1 and controls the position of the scroll handle
// size is in the range 0..1 and controls the size of the scroll handle
// horizontal widget looks best when height is BND_SCROLLBAR_HEIGHT,
// vertical looks best when width is BND_SCROLLBAR_WIDTH
fn bndScrollBar(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
    offset: c_float, size: c_float
) {
    bndBevelInset(ctx,x,y,w,h,
        BND_SCROLLBAR_RADIUS, BND_SCROLLBAR_RADIUS);
    bndInnerBox(ctx,x,y,w,h,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        bndOffsetColor(
            bnd_theme.scrollBarTheme.innerColor, 3*bnd_theme.scrollBarTheme.shadeDown),
        bndOffsetColor(
            bnd_theme.scrollBarTheme.innerColor, 3*bnd_theme.scrollBarTheme.shadeTop));
    bndOutlineBox(ctx,x,y,w,h,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        bndTransparent(bnd_theme.scrollBarTheme.outlineColor));

    let itemColor = bndOffsetColor(
        bnd_theme.scrollBarTheme.itemColor,
        if (state == BND_ACTIVE) {BND_SCROLLBAR_ACTIVE_SHADE} else {0.0});

    bndScrollHandleRect(&x,&y,&w,&h,offset,size);

    bndInnerBox(ctx,x,y,w,h,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        bndOffsetColor(
            itemColor, 3*bnd_theme.scrollBarTheme.shadeTop),
        bndOffsetColor(
            itemColor, 3*bnd_theme.scrollBarTheme.shadeDown));
    bndOutlineBox(ctx,x,y,w,h,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        BND_SCROLLBAR_RADIUS,BND_SCROLLBAR_RADIUS,
        bndTransparent(bnd_theme.scrollBarTheme.outlineColor));
}

// Draw a menu background with its lower left origin at (x,y) and size of (w,h),
// where flags is one or multiple flags from BNDcornerFlags.
fn bndMenuBackground(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, flags: c_int
) {
    let cr: [c_float, ..4]; //float cr[4];
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndSelectCorners(cr, BND_MENU_RADIUS, flags);
    bndInnerColors(&shade_top, &shade_down, &bnd_theme.menuTheme,
        BND_DEFAULT, 0);
    bndInnerBox(ctx,x,y,w,h+1.0,cr[0],cr[1],cr[2],cr[3], shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h+1.0,cr[0],cr[1],cr[2],cr[3],
        bndTransparent(bnd_theme.menuTheme.outlineColor));
    bndDropShadow(ctx,x,y,w,h,BND_MENU_RADIUS,
        BND_SHADOW_FEATHER,BND_SHADOW_ALPHA);
}

// Draw a menu label with its lower left origin at (x,y) and size of (w,h).
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndMenuLabel(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, iconid: c_int, label: *const c_char
) {
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bnd_theme.menuTheme.textColor, BND_LEFT,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a menu item with its lower left origin at (x,y) and size of (w,h),
// where state denotes the widgets current UI state.
// if iconid >= 0, an icon will be added to the widget
// if label is not NULL, a label will be added to the widget
// widget looks best when height is BND_WIDGET_HEIGHT
fn bndMenuItem(ctx: &NVGcontext,
    x: c_float, y: c_float, w: c_float, h: c_float, state: BNDwidgetState,
    iconid: c_int, label: *const c_char
) {
    if (state != BND_DEFAULT) {
        bndInnerBox(ctx,x,y,w,h,0.0,0.0,0.0,0.0,
            bndOffsetColor(bnd_theme.menuItemTheme.innerSelectedColor,
                bnd_theme.menuItemTheme.shadeTop),
            bndOffsetColor(bnd_theme.menuItemTheme.innerSelectedColor,
                bnd_theme.menuItemTheme.shadeDown));
        state = BND_ACTIVE;
    }
    bndIconLabelValue(ctx,x,y,w,h,iconid,
        bndTextColor(&bnd_theme.menuItemTheme, state), BND_LEFT,
        BND_LABEL_FONT_SIZE, label, null());
}

// Draw a tooltip background with its lower left origin at (x,y) and size of (w,h)
fn bndTooltipBackground(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float
) {
    let shade_top: NVGcolor;
    let shade_down: NVGcolor;

    bndInnerColors(&shade_top, &shade_down, &bnd_theme.tooltipTheme,
        BND_DEFAULT, 0);
    bndInnerBox(ctx,x,y,w,h+1.0,
        BND_MENU_RADIUS,BND_MENU_RADIUS,BND_MENU_RADIUS,BND_MENU_RADIUS,
        shade_top, shade_down);
    bndOutlineBox(ctx,x,y,w,h+1.0,
        BND_MENU_RADIUS,BND_MENU_RADIUS,BND_MENU_RADIUS,BND_MENU_RADIUS,
        bndTransparent(bnd_theme.tooltipTheme.outlineColor));
    bndDropShadow(ctx,x,y,w,h,BND_MENU_RADIUS,
        BND_SHADOW_FEATHER,BND_SHADOW_ALPHA);
}


////////////////////////////////////////////////////////////////////////////////

// Estimator Functions
// -------------------
// Use these functions to estimate sizes for widgets with your NVGcontext.

// returns the ideal width for a label with given icon and text
fn bndLabelWidth(ctx: &NVGcontext, iconid: c_int, label: *const c_char
) -> c_float {
    let w = (BND_PAD_LEFT + BND_PAD_RIGHT) as f32;
    if (iconid >= 0) {
        w += BND_ICON_SHEET_RES;
    }
    if (label && (bnd_font >= 0)) {
        ctx.font_face_id( bnd_font);
        ctx.font_size(BND_LABEL_FONT_SIZE);
        w += ctx.text_bounds(1.0, 1.0, label, null());
    }
    return w;
}


////////////////////////////////////////////////////////////////////////////////


// Low Level Functions
// -------------------
// these are part of the implementation detail and can be used to theme
// new kinds of controls in a similar fashion.

// make color transparent using the default alpha value
fn bndTransparent(color: NVGcolor) -> NVGcolor
{
    color.a *= BND_TRANSPARENT_ALPHA;
    return color;
}


// offset a color by a given integer delta in the range -100 to 100
fn bndOffsetColor(color: NVGcolor, delta: c_int) -> NVGcolor
{
    if delta != 0 {
	    let offset = (delta as c_float) / 255.0;
        return nanovg::rgba_f(
            bnd_clamp(color.r+offset, 0.0, 1.0),
            bnd_clamp(color.g+offset, 0.0, 1.0),
            bnd_clamp(color.b+offset, 0.0, 1.0),
            color.a)
    }
    return color;
}


// assigns radius r to the four entries of array radiuses depending on whether
// the corner is marked as sharp or not; see BNDcornerFlags for possible
// flag values.
fn bndSelectCorners(radiuses: *const c_float, r: c_float, flags: c_int
) {
    radiuses[0] = if flags & BND_CORNER_TOP_LEFT {0} else {r};
    radiuses[1] = if flags & BND_CORNER_TOP_RIGHT {0} else {r};
    radiuses[2] = if flags & BND_CORNER_DOWN_RIGHT {0} else {r};
    radiuses[3] = if flags & BND_CORNER_DOWN_LEFT {0} else {r};
}

// computes the upper and lower gradient colors for the inner box from a widget
// theme and the widgets state. If flipActive is set and the state is
// BND_ACTIVE, the upper and lower colors will be swapped.
fn bndInnerColors(shade_top: *const NVGcolor, shade_down: *const NVGcolor,
    theme: *const BNDwidgetTheme, state: BNDwidgetState, flipActive: c_int
) {
    match state {
	    //default:
	    BND_DEFAULT => {
	        *shade_top = bndOffsetColor(theme.innerColor, theme.shadeTop);
	        *shade_down = bndOffsetColor(theme.innerColor, theme.shadeDown);
	    },
	    BND_HOVER => {
	        let color = bndOffsetColor(theme.innerColor, BND_HOVER_SHADE);
	        *shade_top = bndOffsetColor(color, theme.shadeTop);
	        *shade_down = bndOffsetColor(color, theme.shadeDown);
	    },
	    BND_ACTIVE => {
	        *shade_top = bndOffsetColor(theme.innerSelectedColor,
	            if flipActive {theme.shadeDown} else {theme.shadeTop});
	        *shade_down = bndOffsetColor(theme.innerSelectedColor,
	            if flipActive {theme.shadeTop} else {theme.shadeDown});
	    }
    }
}

// computes the text color for a widget label from a widget theme and the
// widgets state.
fn bndTextColor(theme: *const BNDwidgetTheme, state: BNDwidgetState) -> NVGcolor
{
    return if (state == BND_ACTIVE) {theme.textSelectedColor} else {theme.textColor};
}


// computes the bounds of the scrollbar handle from the scrollbar size
// and the handles offset and size.
// offset is in the range 0..1 and defines the position of the scroll handle
// size is in the range 0..1 and defines the size of the scroll handle
fn bndScrollHandleRect(x: *const c_float, y: *const c_float, w: *const c_float, h: *const c_float,
    offset: c_float, size: c_float
) {
    size = bnd_clamp(size,0.0,1.0);
    offset = bnd_clamp(offset,0.0,1.0);
    if ((*h) > (*w)) {
        let hs = fmaxf(size*(*h), (*w)+1.0);
        *y = (*y) + ((*h)-hs)*offset;
        *h = hs;
    } else {
        let ws = fmaxf(size*(*w), (*h)-1.0);
        *x = (*x) + ((*w)-ws)*offset;
        *w = ws;
    }
}

// Add a rounded box path at position (x,y) with size (w,h) and a separate
// radius for each corner listed in clockwise order, so that cr0 = top left,
// cr1 = top right, cr2 = bottom right, cr3 = bottom left;
// this is a low level drawing function: the path must be stroked or filled
// to become visible.
fn bndRoundedBox(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float
) {
    let d;

    w = fmaxf(0.0, w);
    h = fmaxf(0.0, h);
    d = fminf(w, h);

    ctx.move_to(x,y+h*0.5);
    ctx.arc_to(x,y, x+w,y, fminf(cr0, d/2.0));
    ctx.arc_to(x+w,y, x+w,y+h, fminf(cr1, d/2.0));
    ctx.arc_to(x+w,y+h, x,y+h, fminf(cr2, d/2.0));
    ctx.arc_to(x,y+h, x,y, fminf(cr3, d/2.0));
    ctx.close_path();
}

// Draw a flat panel without any decorations at position (x,y) with size (w,h)
// and fills it with backgroundColor
fn bndBackground(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float
) {
    ctx.begin_path();
    ctx.rect(x, y, w, h);
    ctx.fill_color(bnd_theme.backgroundColor);
    ctx.fill();
}

// Draw a beveled border at position (x,y) with size (w,h) shaded with
// lighter and darker versions of backgroundColor
fn bndBevel(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float
) {
    ctx.stroke_width(1.0);

    x += 0.5;
    y += 0.5;
    w -= 1.0;
    h -= 1.0;

    ctx.begin_path();
    ctx.move_to(x, y+h);
    ctx.line_to(x+w, y+h);
    ctx.line_to(x+w, y);
    ctx.stroke_color(bndTransparent(
        bndOffsetColor(bnd_theme.backgroundColor, -BND_BEVEL_SHADE)));
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(x, y+h);
    ctx.line_to(x, y);
    ctx.line_to(x+w, y);
    ctx.stroke_color(bndTransparent(
        bndOffsetColor(bnd_theme.backgroundColor, BND_BEVEL_SHADE)));
    ctx.stroke();
}

// Draw a lower inset for a rounded box at position (x,y) with size (w,h)
// that gives the impression the surface has been pushed in.
// cr2 and cr3 contain the radiuses of the bottom right and bottom left
// corners of the rounded box.
fn bndBevelInset(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    cr2: c_float, cr3: c_float
) {
    let d;

    y -= 0.5;
    d = fminf(w, h);
    cr2 = fminf(cr2, d/2.0);
    cr3 = fminf(cr3, d/2.0);

    ctx.begin_path();
    ctx.move_to(x+w,y+h-cr2);
    ctx.arc_to(x+w,y+h, x,y+h, cr2);
    ctx.arc_to(x,y+h, x,y, cr3);

    let bevelColor = bndOffsetColor(bnd_theme.backgroundColor,
        BND_INSET_BEVEL_SHADE);

    ctx.stroke_width(1.0);
    ctx.stroke_paint(
        ctx.linear_gradient(
            x,y+h-fmaxf(cr2,cr3)-1.0,
            x,y+h-1.0,
        nanovg::rgba_f(bevelColor.r, bevelColor.g, bevelColor.b, 0.0),
        bevelColor));
    ctx.stroke();
}

// Draw an icon with (x,y) as its upper left coordinate; the iconid selects
// the icon from the sheet; use the BND_ICONID macro to build icon IDs.
fn bndIcon(ctx: &NVGcontext, x: c_float, y: c_float, iconid: c_int
) {
    if (bnd_icon_image < 0) {return}; // no icons loaded

    let ix = iconid & 0xff;
    let iy = (iconid>>8) & 0xff;
    let u = (BND_ICON_SHEET_OFFSET_X + ix*BND_ICON_SHEET_GRID) as f32;
    let v = (BND_ICON_SHEET_OFFSET_Y + iy*BND_ICON_SHEET_GRID) as f32;

    ctx.begin_path();
    ctx.rect(x,y,BND_ICON_SHEET_RES,BND_ICON_SHEET_RES);
    ctx.fill_paint(
        ctx.image_pattern(x-u,y-v,
        BND_ICON_SHEET_WIDTH as f32,
        BND_ICON_SHEET_HEIGHT as f32,
        0.0,bnd_icon_image,0.0,1.0));
    ctx.fill();
}

// Draw a drop shadow around the rounded box at (x,y) with size (w,h) and
// radius r, with feather as its maximum range in pixels.
// No shadow will be painted inside the rounded box.
fn bndDropShadow(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    r: c_float, feather: c_float, alpha: c_float
) {
    ctx.begin_path();
    y += feather;
    h -= feather;

    ctx.move_to(x-feather, y-feather);
    ctx.line_to(x, y-feather);
    ctx.line_to(x, y+h-feather);
    ctx.arc_to(x,y+h,x+r,y+h,r);
    ctx.arc_to(x+w,y+h,x+w,y+h-r,r);
    ctx.line_to(x+w, y-feather);
    ctx.line_to(x+w+feather, y-feather);
    ctx.line_to(x+w+feather, y+h+feather);
    ctx.line_to(x-feather, y+h+feather);
    ctx.close_path();

    ctx.fill_paint(ctx.box_gradient(
        x - feather*0.5,y - feather*0.5,
        w + feather,h+feather,
        r+feather*0.5,
        feather,
        nanovg::rgba_f(0.0,0.0,0.0,alpha*alpha),
        nanovg::rgba_f(0.0,0.0,0.0,0.0)));
    ctx.fill();
}

// Draw the inner part of a widget box, with a gradient from shade_top to
// shade_down. If h>w, the gradient will be horizontal instead of
// vertical.
fn bndInnerBox(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float,
    shade_top: NVGcolor, shade_down: NVGcolor
) {
    ctx.begin_path();
    bndRoundedBox(ctx,x+1.0,y+1.0,w-2.0,h-3.0,
        fmaxf(0.0,cr0-1.0),fmaxf(0.0,cr1-1.0),fmaxf(0.0,cr2-1.0),fmaxf(0.0,cr3-1.0));
    ctx.fill_paint(
    	if (h-2.0)>w  {ctx.linear_gradient(x,y,x+w,y,shade_top,shade_down)}
        else 		{ctx.linear_gradient(x,y,x,y+h,shade_top,shade_down)});
    ctx.fill();
}

// Draw the outline part of a widget box with the given color
fn bndOutlineBox(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    cr0: c_float, cr1: c_float, cr2: c_float, cr3: c_float, color: NVGcolor
) {
    ctx.begin_path();
    bndRoundedBox(ctx,x+0.5,y+0.5,w-1.0,h-2.0,cr0,cr1,cr2,cr3);
    ctx.stroke_color(color);
    ctx.stroke_width(1.0);
    ctx.stroke();
}

// Draw an optional icon specified by <iconid> and an optional label with
// given alignment (BNDtextAlignment), fontsize and color within a widget box.
// if iconid is >= 0, an icon will be drawn and the labels remaining space
// will be adjusted.
// if label is not NULL, it will be drawn with the specified alignment, fontsize
// and color.
// if value is not NULL, label and value will be drawn with a ":" separator
// inbetween.
fn bndIconLabelValue(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    iconid: c_int, color: NVGcolor, align: c_int, fontsize: c_float, label: *const c_char,
    value: *const c_char
) {
    let pleft = BND_PAD_LEFT;
    if (label) {
        if (iconid >= 0) {
            bndIcon(ctx,x+4.0,y+2.0,iconid);
            pleft += BND_ICON_SHEET_RES;
        }

        if (bnd_font < 0) {return};
        ctx.font_face_id(bnd_font);
        ctx.font_size(fontsize);
        ctx.begin_path();
        ctx.fill_color(color);
        if (value) {
            let label_width = ctx.text_bounds(1.0, 1.0, label);
            let sep_width = ctx.text_bounds(1.0, 1.0,
                theme::BND_LABEL_SEPARATOR);

            ctx.text_align(nanovg::LEFT|nanovg::BASELINE);
            x += pleft as f32;
            if (align == BND_CENTER) {
                let width = label_width + sep_width
                    + ctx.text_bounds(1.0, 1.0, value);
                x += ((w-(BND_PAD_RIGHT-pleft) as f32)-width)*0.5;
            }
            y += h-BND_TEXT_PAD_DOWN as f32;
            ctx.text(x, y, label);
            x += label_width;
            ctx.text(x, y, theme::BND_LABEL_SEPARATOR);
            x += sep_width;
            ctx.text(x, y, value);
        } else {
            ctx.text_align(
                if align==BND_LEFT  {nanovg::LEFT  |nanovg::BASELINE}
                else 				{nanovg::CENTER|nanovg::BASELINE});
            ctx.text_box(x+pleft as f32,y+h-BND_TEXT_PAD_DOWN as f32,
                w-BND_PAD_RIGHT as f32-pleft as f32,label);
        }
    } else if (iconid >= 0) {
        bndIcon(ctx,x+2.0,y+2.0,iconid);
    }
}

// Draw an optional icon specified by <iconid>, an optional label and
// a caret with given fontsize and color within a widget box.
// if iconid is >= 0, an icon will be drawn and the labels remaining space
// will be adjusted.
// if label is not NULL, it will be drawn with the specified alignment, fontsize
// and color.
// cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
// cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
// if cend < cbegin, then no caret will be drawn
fn bndIconLabelCaret(ctx: &NVGcontext, x: c_float, y: c_float, w: c_float, h: c_float,
    iconid: c_int, color: NVGcolor, fontsize: c_float, label: *const c_char,
    caretcolor: NVGcolor, cbegin: c_int, cend: c_int
) {
    let bounds: [c_float, ..4];
    let pleft = theme::BND_TEXT_RADIUS;
    if (!label) {return};
    if (iconid >= 0) {
        bndIcon(ctx,x+4.0,y+2.0,iconid);
        pleft += BND_ICON_SHEET_RES as f32;
    }

    if (bnd_font < 0) {return};

    x+=pleft;
    y+=h-BND_TEXT_PAD_DOWN as f32;

    ctx.font_face_id(bnd_font);
    ctx.font_size(fontsize);
    ctx.text_align(nanovg::LEFT|nanovg::BASELINE);

    if (cend >= cbegin) {
        //const char *cb;const char *ce;
        let /*static*/ glyphs: [NVGglyphPosition, ..theme::BND_MAX_GLYPHS];
        let nglyphs = ctx.text_glyph_positions(
            x, y, label, label+cend+1, glyphs, theme::BND_MAX_GLYPHS);
        let c0=glyphs[0].x;
        let c1=glyphs[nglyphs-1].x;
        let cb = label+cbegin;
        let ce = label+cend;
        // TODO: this is slow
        for i in range(0, nglyphs) {
            if (glyphs[i].str == cb) {
                c0 = glyphs[i].x;
            }
            if (glyphs[i].str == ce) {
                c1 = glyphs[i].x;
            }
        }

        ctx.text_bounds(x,y,label, bounds);
        ctx.begin_path();
        if (cbegin == cend) {
            ctx.fill_color(nanovg::rgb_f(0.337,0.502,0.761));
            ctx.rect(c0-1.0, bounds[1], 2.0, bounds[3]-bounds[1]);
        } else {
            ctx.fill_color(caretcolor);
            ctx.rect(c0-1.0, bounds[1], c1-c0+1.0, bounds[3]-bounds[1]);
        }
        ctx.fill();
    }

    ctx.begin_path();
    ctx.fill_color(color);
    ctx.text_box(x,y,w-theme::BND_TEXT_RADIUS-pleft,label);
}

// Draw a checkmark for an option box with the given upper left coordinates
// (ox,oy) with the specified color.
fn bndCheck(ctx: &NVGcontext, ox: c_float, oy: c_float, color: NVGcolor
) {
    ctx.begin_path();
    ctx.stroke_width(2.0);
    ctx.stroke_color(color);
    ctx.line_cap(nanovg::BUTT);
    ctx.line_join(nanovg::MITER);
    ctx.move_to(ox+4.0,oy+5.0);
    ctx.line_to(ox+7.0,oy+8.0);
    ctx.line_to(ox+14.0,oy+1.0);
    ctx.stroke();
}


// Draw a horizontal arrow for a number field with its center at (x,y) and
// size s; if s is negative, the arrow points to the left.
fn bndArrow(ctx: &NVGcontext, x: c_float, y: c_float, s: c_float, color: NVGcolor
) {
    ctx.begin_path();
    ctx.move_to(x,y);
    ctx.line_to(x-s,y+s);
    ctx.line_to(x-s,y-s);
    ctx.close_path();
    ctx.fill_color(color);
    ctx.fill();
}

// Draw an up/down arrow for a choice box with its center at (x,y) and size s
fn bndUpDownArrow(ctx: &NVGcontext, x: c_float, y: c_float, s: c_float, color: NVGcolor
) {
    ctx.begin_path();
    let w = 1.1*s;
    ctx.move_to(x,y-1.0);
    ctx.line_to(x+0.5*w,y-s-1.0);
    ctx.line_to(x+w,y-1.0);
    ctx.close_path();
    ctx.move_to(x,y-1.0);
    ctx.line_to(x+0.5*w,y+s+1.0);
    ctx.line_to(x+w,y-1.0);
    ctx.close_path();
    ctx.fill_color(color);
    ctx.fill();
}

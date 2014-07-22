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
#![warn(uppercase_variables)]
#![deny(unnecessary_parens)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_qualification)]
//#![warn(missing_doc)]
#![deny(unused_result)]
#![allow(unused_imports)]
#![allow(unused_attribute)]
#![allow(unused_variable)]
#![deny(unnecessary_typecast)]
#![warn(visible_private_types)]
#![allow(dead_code)]


extern crate libc;
extern crate nanovg;


pub use nanovg::Color;
pub use nanovg::Winding;
pub use CCW = nanovg::CCW;
pub use constants::*;
pub use theme::Theme;
pub use theme::*;

mod ffi;
mod constants;
mod theme;


fn min(a:f32, b:f32) -> f32 { if a<b { a } else { b } }
fn max(a:f32, b:f32) -> f32 { if a>b { a } else { b } }
fn clamp(v: f32, mn: f32, mx: f32) -> f32 { max(mn, min(mx, v) ) } //if v>mx {mx} else { if v<mn {mn} else {v} }

fn rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color { Color::rgba_f(r, g, b, a) }
fn black() -> Color { Color::rgba(0,0,0,1) }



/// how text on a control is aligned
#[repr(u32)]
pub enum TextAlignment {
    LEFT   = ffi::BND_LEFT,
    CENTER = ffi::BND_CENTER
}

#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum WidgetState {
    /// not interacting
    DEFAULT  = ffi::BND_DEFAULT,
    /// the mouse is hovering over the control
    HOVER    = ffi::BND_HOVER,
    /// the widget is activated (pressed) or in an active state (toggled)
    ACTIVE   = ffi::BND_ACTIVE,
}

/// flags indicating which corners are sharp (for grouping widgets)
//pub type BNDcornerFlags = c_int;
bitflags!(
    flags CornerFlags: u32 {
        // all corners are round
        static CORNER_NONE         = ffi::BND_CORNER_NONE,
        // sharp top left corner
        static CORNER_TOP_LEFT     = ffi::BND_CORNER_TOP_LEFT,
        // sharp top right corner
        static CORNER_TOP_RIGHT    = ffi::BND_CORNER_TOP_RIGHT,
        // sharp bottom right corner
        static CORNER_DOWN_RIGHT   = ffi::BND_CORNER_DOWN_RIGHT,
        // sharp bottom left corner
        static CORNER_DOWN_LEFT    = ffi::BND_CORNER_DOWN_LEFT,
        // all corners are sharp;
        // you can invert a set of flags using ^= BND_CORNER_ALL
        static CORNER_ALL          = ffi::BND_CORNER_ALL,
        // top border is sharp
        static CORNER_TOP          = ffi::BND_CORNER_TOP,
        // bottom border is sharp
        static CORNER_DOWN         = ffi::BND_CORNER_DOWN,
        // left border is sharp
        static CORNER_LEFT         = ffi::BND_CORNER_LEFT,
        // right border is sharp
        static CORNER_RIGHT        = ffi::BND_CORNER_RIGHT
    }
)


pub trait ThemedDraw
{
    fn draw_label(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, label: &str);
    fn draw_tool_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
    fn draw_radio_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
    fn draw_text_field(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, text: &str, cbegin: i32, cend: i32);
    fn draw_option_button(&mut self, x:f32,y:f32, w:f32,h:f32, state: WidgetState, label: &str);
    fn draw_choice_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
    fn draw_number_field(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, label: &str, value: &str);
    fn draw_slider(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, progress: f32, label: &str, value: &str);
    fn draw_scrollbar(&mut self, x:f32,y:f32, w:f32,h:f32, state: WidgetState, offset: f32, size: f32);
    fn draw_menu_background(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags);
    fn draw_menu_label(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, label: &str);
    fn draw_menu_item(&mut self, x:f32,y:f32, w:f32,h:f32, state: &mut WidgetState, iconid: i32, label: &str);
    fn draw_tooltip_background(&mut self, x:f32,y:f32, w:f32,h:f32);
}

impl<'a> ThemedDraw for Theme<'a>
{

    ////////////////////////////////////////////////////////////////////////////////
    // High Level Functions
    // --------------------
    // Use these functions to draw themed widgets with your NVGcontext.

    // Draw a label with its lower left origin at (x, y) and size of (w, h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_label(&mut self,
        x:f32,y:f32, w:f32,h:f32, iconid: i32, label: &str
    ) {
        let color = self.theme().regularTheme.textColor;
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid,
            color, LEFT,
            LABEL_FONT_SIZE, label, None);
    }

    // Draw a tool button  with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_tool_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: i32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, TOOL_RADIUS, flags);
        let color = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], color);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().toolTheme, state, true);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().toolTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(color));
        let color = bnd_TextColor(&self.theme().toolTheme, state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid,
            color, CENTER,
            LABEL_FONT_SIZE, label, None);
    }

    // Draw a radio button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_radio_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: i32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, OPTION_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().radioTheme, state, true);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().radioTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(outline));
        let color = bnd_TextColor(&self.theme().radioTheme, state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid,
            color, CENTER,
            LABEL_FONT_SIZE, label, None);
    }

    // Draw a text field with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if text is not NULL, text will be printed to the widget
    // cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
    // cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
    // if cend < cbegin, then no caret will be drawn
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_text_field(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: i32, text: &str, cbegin: i32, cend: i32
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, TEXT_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().textFieldTheme, state, false);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().textFieldTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(outline));
        let mut cend = cend;
        if state != ACTIVE {
            cend = -1;
        }
        let itemcolor = self.theme().textFieldTheme.itemColor;
        let textcolor = bnd_TextColor(&self.theme().textFieldTheme, state);
        self.nvg().bnd_IconLabelCaret(x, y, w, h, iconid,
            textcolor, LABEL_FONT_SIZE,
            text, itemcolor, cbegin, cend);
    }

    // Draw an option button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_option_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: WidgetState,
        label: &str
    ) {
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        let ox = x;
        let oy = y+h-OPTION_HEIGHT-3.0;

        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS,
            bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().optionTheme, state, true);
        self.nvg().bnd_InnerBox(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS,
            shade_top, shade_down);
        let color = self.theme().optionTheme.outlineColor;
        self.nvg().bnd_OutlineBox(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS,
            bnd_Transparent(color));
        if state == ACTIVE {
            let color = self.theme().optionTheme.itemColor;
            self.nvg().bnd_Check(ox, oy, bnd_Transparent(color));
        }
        let color = bnd_TextColor(&self.theme().optionTheme, state);
        self.nvg().bnd_IconLabelValue(x+12.0, y, w-12.0, h, -1,
            color, LEFT,
            LABEL_FONT_SIZE, label, None);
    }

    // Draw a choice button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_choice_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: i32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, OPTION_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().choiceTheme, state, true);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().choiceTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(color));
        let color = bnd_TextColor(&self.theme().choiceTheme, state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid,
            color, LEFT,
            LABEL_FONT_SIZE, label, None);
        let color = self.theme().choiceTheme.itemColor;
        self.nvg().bnd_UpDownArrow(x+w-10.0, y+10.0, 5.0,
            bnd_Transparent(color));
    }

    // Draw a number field with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_number_field(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        label: &str, value: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, NUMBER_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().numberFieldTheme, state, false);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().numberFieldTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(color));
        let color = bnd_TextColor(&self.theme().numberFieldTheme, state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, -1,
            color, CENTER, LABEL_FONT_SIZE, label, Some(value));
        let color = self.theme().numberFieldTheme.itemColor;
        self.nvg().bnd_Arrow(x+8.0, y+10.0, -NUMBER_ARROW_SIZE,
            bnd_Transparent(color));
        self.nvg().bnd_Arrow(x+w-8.0, y+10.0, NUMBER_ARROW_SIZE,
            bnd_Transparent(color));
    }

    // Draw slider control with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // progress must be in the range 0..1 and controls the size of the slider bar
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_slider(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        progress: f32, label: &str, value: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        let bg = self.theme().backgroundColor;
        bnd_SelectCorners(&mut cr, NUMBER_RADIUS, flags);
        self.nvg().bnd_BevelInset(x, y, w, h, cr[2], cr[3], bg);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().sliderTheme, state, false);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);

        let top = self.theme().sliderTheme.shadeTop;
        let down = self.theme().sliderTheme.shadeDown;
        if state == ACTIVE {
            shade_top = bnd_OffsetColor(
                self.theme().sliderTheme.itemColor, top);
            shade_down = bnd_OffsetColor(
                self.theme().sliderTheme.itemColor, down);
        } else {
            shade_top = bnd_OffsetColor(
                self.theme().sliderTheme.itemColor, down);
            shade_down = bnd_OffsetColor(
                self.theme().sliderTheme.itemColor, top);
        }
        self.nvg().scissor(x, y, 8.0+(w-8.0)*clamp(progress, 0.0, 1.0), h);
        self.nvg().bnd_InnerBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        self.nvg().reset_scissor();

        let outline = self.theme().sliderTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(outline));
        let color = bnd_TextColor(&self.theme().sliderTheme, state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, -1,
            color, CENTER, LABEL_FONT_SIZE, label, Some(value));
    }

    // Draw scrollbar with its lower left origin at (x, y) and size of (w, h),
    // where state denotes the widgets current UI state.
    // offset is in the range 0..1 and controls the position of the scroll handle
    // size is in the range 0..1 and controls the size of the scroll handle
    // horizontal widget looks best when height is SCROLLBAR_HEIGHT,
    // vertical looks best when width is SCROLLBAR_WIDTH
    fn draw_scrollbar(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: WidgetState,
        offset: f32, size: f32
    ) {
        let bg = self.theme().backgroundColor;
        self.nvg().bnd_BevelInset(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bg);
        let top = self.theme().scrollBarTheme.shadeTop;
        let down = self.theme().scrollBarTheme.shadeDown;
        let inner = self.theme().scrollBarTheme.innerColor;
        let outline = self.theme().scrollBarTheme.outlineColor;
        self.nvg().bnd_InnerBox(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bnd_OffsetColor(inner, 3*down),
            bnd_OffsetColor(inner, 3*top));
        self.nvg().bnd_OutlineBox(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bnd_Transparent(outline));

        let itemcolor = self.theme().scrollBarTheme.itemColor;
        let itemColor = bnd_OffsetColor(
            itemcolor,
            if state == ACTIVE {SCROLLBAR_ACTIVE_SHADE} else {0});

        let mut x = x; let mut y = y;
        let mut w = w; let mut h = h;
        bnd_ScrollHandleRect(&mut x, &mut y, &mut w, &mut h, offset, size);

        let top = self.theme().scrollBarTheme.shadeTop;
        let down = self.theme().scrollBarTheme.shadeDown;
        let outline = self.theme().scrollBarTheme.outlineColor;
        self.nvg().bnd_InnerBox(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bnd_OffsetColor(itemColor, 3*top),
            bnd_OffsetColor(itemColor, 3*down));
        self.nvg().bnd_OutlineBox(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bnd_Transparent(outline));
    }

    // Draw a menu background with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags.
    fn draw_menu_background(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        bnd_SelectCorners(&mut cr, MENU_RADIUS, flags);
        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().menuTheme,
            DEFAULT, false);
        self.nvg().bnd_InnerBox(x, y, w, h+1.0, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().menuTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h+1.0, cr[0], cr[1], cr[2], cr[3],
            bnd_Transparent(outline));
        self.nvg().bnd_DropShadow(x, y, w, h, MENU_RADIUS,
            SHADOW_FEATHER, SHADOW_ALPHA);
    }

    // Draw a menu label with its lower left origin at (x, y) and size of (w, h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_menu_label(&mut self,
        x:f32,y:f32, w:f32,h:f32, iconid: i32, label: &str
    ) {
        let color = self.theme().menuTheme.textColor;
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid,
            color, LEFT,
            LABEL_FONT_SIZE, label, None);
    }

    // Draw a menu item with its lower left origin at (x, y) and size of (w, h),
    // where state denotes the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_menu_item(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: &mut WidgetState,
        iconid: i32, label: &str
    ) {
        let top = self.theme().menuItemTheme.shadeTop;
        let down = self.theme().menuItemTheme.shadeDown;
        if *state != DEFAULT {
            let color = self.theme().menuItemTheme.innerSelectedColor;
            self.nvg().bnd_InnerBox(x, y, w, h, 0.0, 0.0, 0.0, 0.0,
                bnd_OffsetColor(color, top),
                bnd_OffsetColor(color, down));
            *state = ACTIVE;
        }
        let color = bnd_TextColor(&self.theme().menuItemTheme, *state);
        self.nvg().bnd_IconLabelValue(x, y, w, h, iconid, color,
            LEFT, LABEL_FONT_SIZE, label, None);
    }

    // Draw a tooltip background with its lower left origin at (x, y) and size of (w, h)
    fn draw_tooltip_background(&mut self, x:f32,y:f32, w:f32,h:f32
    ) {
        let mut shade_top = black();
        let mut shade_down = black();

        bnd_InnerColors(&mut shade_top, &mut shade_down, &self.theme().tooltipTheme,
            DEFAULT, false);
        self.nvg().bnd_InnerBox(x, y, w, h+1.0,
            MENU_RADIUS, MENU_RADIUS, MENU_RADIUS, MENU_RADIUS,
            shade_top, shade_down);
        let ocolor = self.theme().tooltipTheme.outlineColor;
        self.nvg().bnd_OutlineBox(x, y, w, h+1.0,
            MENU_RADIUS, MENU_RADIUS, MENU_RADIUS, MENU_RADIUS,
            bnd_Transparent(ocolor));
        self.nvg().bnd_DropShadow(x, y, w, h, MENU_RADIUS,
            SHADOW_FEATHER, SHADOW_ALPHA);
    }
}
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////

// Estimator Functions
// -------------------
// Use these functions to estimate sizes for widgets with your NVGcontext.

// returns the ideal width for a label with given icon and text
fn draw_labelWidth(ctx: &nanovg::Ctx, iconid: i32, label: &str, bnd_font: i32
) -> f32 {
    let mut w = (PAD_LEFT + PAD_RIGHT) as f32;
    if iconid >= 0 {
        w += ICON_SHEET_RES as f32;
    }
    if label.len() > 0  && (bnd_font >= 0) {
        ctx.font_face_id( bnd_font);
        ctx.font_size(LABEL_FONT_SIZE);
        w += ctx.text_advance(1.0, 1.0, label);
    }
    return w;
}


// Low Level Functions
// -------------------
// these are part of the implementation detail and can be used to theme
// new kinds of controls in a similar fashion.

// make color transparent using the default alpha value
fn bnd_Transparent(color: Color) -> Color
{
    return rgba_f(
        color.r(),
        color.g(),
        color.b(),
        color.a() * TRANSPARENT_ALPHA
    );
}


// offset a color by a given integer delta in the range -100 to 100
fn bnd_OffsetColor(color: Color, delta: i32) -> Color
{
    if delta != 0 {
	    let offset = (delta as f32) / 255.0;
        return rgba_f(
            clamp(color.r()+offset, 0.0, 1.0),
            clamp(color.g()+offset, 0.0, 1.0),
            clamp(color.b()+offset, 0.0, 1.0),
            color.a())
    }
    return color;
}


// assigns radius r to the four entries of array radiuses depending on whether
// the corner is marked as sharp or not; see BNDcornerFlags for possible
// flag values.
fn bnd_SelectCorners(radiuses: &mut [f32, ..4], r: f32, flags: CornerFlags
) {
    radiuses[0] = if flags.contains(CORNER_TOP_LEFT  ) {0.0} else {r};
    radiuses[1] = if flags.contains(CORNER_TOP_RIGHT ) {0.0} else {r};
    radiuses[2] = if flags.contains(CORNER_DOWN_RIGHT) {0.0} else {r};
    radiuses[3] = if flags.contains(CORNER_DOWN_LEFT ) {0.0} else {r};
}

// computes the upper and lower gradient colors for the inner box from a widget
// theme and the widgets state. If flipActive is set and the state is
// ACTIVE, the upper and lower colors will be swapped.
fn bnd_InnerColors(shade_top: &mut Color, shade_down: &mut Color,
    theme: &WidgetTheme, state: WidgetState, flipActive: bool
) {
    match state {
	    //default:
	    DEFAULT => {
	        *shade_top = bnd_OffsetColor(theme.innerColor, theme.shadeTop);
	        *shade_down = bnd_OffsetColor(theme.innerColor, theme.shadeDown);
	    },
	    HOVER => {
	        let color = bnd_OffsetColor(theme.innerColor, HOVER_SHADE);
	        *shade_top = bnd_OffsetColor(color, theme.shadeTop);
	        *shade_down = bnd_OffsetColor(color, theme.shadeDown);
	    },
	    ACTIVE => {
	        *shade_top = bnd_OffsetColor(theme.innerSelectedColor,
	            if flipActive {theme.shadeDown} else {theme.shadeTop});
	        *shade_down = bnd_OffsetColor(theme.innerSelectedColor,
	            if flipActive {theme.shadeTop} else {theme.shadeDown});
	    }
    }
}

// computes the text color for a widget label from a widget theme and the
// widgets state.
fn bnd_TextColor(theme: &WidgetTheme, state: WidgetState) -> Color
{
    return if state == ACTIVE {theme.textSelectedColor} else {theme.textColor};
}


// computes the bounds of the scrollbar handle from the scrollbar size
// and the handles offset and size.
// offset is in the range 0..1 and defines the position of the scroll handle
// size is in the range 0..1 and defines the size of the scroll handle
fn bnd_ScrollHandleRect(x: &mut f32, y: &mut f32, w: &mut f32, h: &mut f32,
    offset: f32, size: f32
) {
    let size = clamp(size, 0.0, 1.0);
    let offset = clamp(offset, 0.0, 1.0);
    if (*h) > (*w) {
        let hs = max(size*(*h), (*w)+1.0);
        *y = (*y) + ((*h)-hs)*offset;
        *h = hs;
    } else {
        let ws = max(size*(*w), (*h)-1.0);
        *x = (*x) + ((*w)-ws)*offset;
        *w = ws;
    }
}


/////////////////////////////////////////////
// NVG context extenders begin here

trait LowLevelDraw
{
    // misc utility

    //fn bnd_Transparent(color: Color) -> Color;
    //fn bnd_OffsetColor(color: Color, delta: i32) -> Color;
    //fn bnd_SelectCorners(radiuses: [f32, ..4], r: f32, flags: CornerFlags);
    //fn bnd_InnerColors(shade_top: &Color, shade_down: &Color, theme: &WidgetTheme, state: WidgetState, flipActive: bool);
    //fn bnd_TextColor(theme: &WidgetTheme, state: WidgetState) -> Color;
    //fn bnd_ScrollHandleRect(x: *const f32, y: *const f32, w: *const f32, h: *const f32, offset: f32, size: f32);

    // context related

    fn bnd_RoundedBox(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32);
    fn bnd_Background(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color);
    fn bnd_Bevel(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color);
    fn bnd_BevelInset(&mut self, x:f32,y:f32, w:f32,h:f32, cr2: f32, cr3: f32, bg: Color);
    fn bnd_DropShadow(&mut self, x:f32,y:f32, w:f32,h:f32, r: f32, feather: f32, alpha: f32);
    fn bnd_InnerBox(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32, shade_top: Color, shade_down: Color);
    fn bnd_OutlineBox(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32, color: Color);
    fn bnd_Check(&mut self, ox: f32, oy: f32, color: Color);
    fn bnd_Arrow(&mut self, x: f32, y: f32, s: f32, color: Color);
    fn bnd_UpDownArrow(&mut self, x: f32, y: f32, s: f32, color: Color);
    fn bnd_Icon(&mut self, x: f32, y: f32, iconid: i32);
    fn bnd_IconLabelValue(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, color: Color, align: TextAlignment, fontsize: f32, label: &str, value: Option<&str>);
    fn bnd_IconLabelCaret(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, color: Color, fontsize: f32, label: &str, caretcolor: Color, cbegin: i32, cend: i32);
}
impl LowLevelDraw for nanovg::Ctx {

    // Add a rounded box path at position (x, y) with size (w, h) and a separate
    // radius for each corner listed in clockwise order, so that cr0 = top left,
    // cr1 = top right, cr2 = bottom right, cr3 = bottom left;
    // this is a low level drawing function: the path must be stroked or filled
    // to become visible.
    fn bnd_RoundedBox(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32
    ) {
        let w = max(0.0, w);
        let h = max(0.0, h);
        let d = min(w, h);

        self.move_to(x, y+h*0.5);
        self.arc_to(x, y, x+w, y, min(cr0, d/2.0));
        self.arc_to(x+w, y, x+w, y+h, min(cr1, d/2.0));
        self.arc_to(x+w, y+h, x, y+h, min(cr2, d/2.0));
        self.arc_to(x, y+h, x, y, min(cr3, d/2.0));
        self.close_path();
    }

    // Draw a flat panel without any decorations at position (x, y) with size (w, h)
    // and fills it with backgroundColor
    fn bnd_Background(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color
    ) {
        self.begin_path();
        self.rect(x, y, w, h);
        self.fill_color(bg);
        self.fill();
    }

    // Draw a beveled border at position (x, y) with size (w, h) shaded with
    // lighter and darker versions of backgroundColor
    fn bnd_Bevel(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color
    ) {
        self.stroke_width(1.0);

        let x = x + 0.5;
        let y = y + 0.5;
        let w = w - 1.0;
        let h = h - 1.0;

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x+w, y+h);
        self.line_to(x+w, y);
        self.stroke_color(bnd_Transparent(
            bnd_OffsetColor(bg, -BEVEL_SHADE)));
        self.stroke();

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x, y);
        self.line_to(x+w, y);
        self.stroke_color(bnd_Transparent(
            bnd_OffsetColor(bg, BEVEL_SHADE)));
        self.stroke();
    }

    // Draw a lower inset for a rounded box at position (x, y) with size (w, h)
    // that gives the impression the surface has been pushed in.
    // cr2 and cr3 contain the radiuses of the bottom right and bottom left
    // corners of the rounded box.
    fn bnd_BevelInset(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr2: f32, cr3: f32,
        bg: Color
    ) {
        let mut y = y - 0.5;
        let d = min(w, h);
        let mut cr2 = min(cr2, d/2.0);
        let mut cr3 = min(cr3, d/2.0);

        self.begin_path();
        self.move_to(x+w, y+h-cr2);
        self.arc_to(x+w, y+h, x, y+h, cr2);
        self.arc_to(x, y+h, x, y, cr3);

        let bevelColor = bnd_OffsetColor(bg,
            INSET_BEVEL_SHADE);

        self.stroke_width(1.0);
        self.stroke_paint(
            self.linear_gradient(
                x, y+h-max(cr2, cr3)-1.0,
                x, y+h-1.0,
            rgba_f(bevelColor.r(), bevelColor.g(), bevelColor.b(), 0.0),
            bevelColor));
        self.stroke();
    }

    // Draw a drop shadow around the rounded box at (x, y) with size (w, h) and
    // radius r, with feather as its maximum range in pixels.
    // No shadow will be painted inside the rounded box.
    fn bnd_DropShadow(&mut self, x:f32,y:f32, w:f32,h:f32,
        r: f32, feather: f32, alpha: f32
    ) {
        self.begin_path();

        let mut y = y;
        let mut h = h;
        y += feather;
        h -= feather;

        self.move_to(x-feather, y-feather);
        self.line_to(x, y-feather);
        self.line_to(x, y+h-feather);
        self.arc_to(x, y+h, x+r, y+h, r);
        self.arc_to(x+w, y+h, x+w, y+h-r, r);
        self.line_to(x+w, y-feather);
        self.line_to(x+w+feather, y-feather);
        self.line_to(x+w+feather, y+h+feather);
        self.line_to(x-feather, y+h+feather);
        self.close_path();

        self.fill_paint(self.box_gradient(
            x - feather*0.5, y - feather*0.5,
            w + feather, h+feather,
            r+feather*0.5,
            feather,
            rgba_f(0.0, 0.0, 0.0, alpha*alpha),
            rgba_f(0.0, 0.0, 0.0, 0.0)));
        self.fill();
    }

    // Draw the inner part of a widget box, with a gradient from shade_top to
    // shade_down. If h>w, the gradient will be horizontal instead of
    // vertical.
    fn bnd_InnerBox(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32,
        shade_top: Color, shade_down: Color
    ) {
        self.begin_path();
        self.bnd_RoundedBox(x+1.0, y+1.0, w-2.0, h-3.0,
            max(0.0, cr0-1.0), max(0.0, cr1-1.0), max(0.0, cr2-1.0), max(0.0, cr3-1.0));
        self.fill_paint(
        	if (h-2.0)>w  {self.linear_gradient(x, y, x+w, y, shade_top, shade_down)}
            else 		{self.linear_gradient(x, y, x, y+h, shade_top, shade_down)});
        self.fill();
    }

    // Draw the outline part of a widget box with the given color
    fn bnd_OutlineBox(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32, color: Color
    ) {
        self.begin_path();
        self.bnd_RoundedBox(x+0.5, y+0.5, w-1.0, h-2.0, cr0, cr1, cr2, cr3);
        self.stroke_color(color);
        self.stroke_width(1.0);
        self.stroke();
    }

    // Draw an icon with (x, y) as its upper left coordinate; the iconid selects
    // the icon from the sheet; use the ICONID macro to build icon IDs.
    fn bnd_Icon(&mut self, x: f32, y: f32, iconid: i32
    ) {
    //    let icons = self.theme().icon_image;
    //    if (icons < 0) {return}  // no icons loaded
    //
    //    let ix = iconid & 0xff;
    //    let iy = (iconid>>8) & 0xff;
    //    let u = (ICON_SHEET_OFFSET_X + ix*ICON_SHEET_GRID) as f32;
    //    let v = (ICON_SHEET_OFFSET_Y + iy*ICON_SHEET_GRID) as f32;
    //
    //    self.begin_path();
    //    self.rect(x, y, ICON_SHEET_RES, ICON_SHEET_RES);
    //    self.fill_paint(
    //        self.image_pattern(x-u, y-v,
    //        ICON_SHEET_WIDTH as f32,
    //        ICON_SHEET_HEIGHT as f32,
    //        0.0, icons, 0.0, 1.0));
    //    self.fill();
    }

    // Draw an optional icon specified by <iconid> and an optional label with
    // given alignment (BNDtextAlignment), fontsize and color within a widget box.
    // if iconid is >= 0, an icon will be drawn and the labels remaining space
    // will be adjusted.
    // if label is not NULL, it will be drawn with the specified alignment, fontsize
    // and color.
    // if value is not NULL, label and value will be drawn with a ":" separator
    // inbetween.
    fn bnd_IconLabelValue(&mut self, x:f32,y:f32, w:f32,h:f32,
        iconid: i32, color: Color, align: TextAlignment, fontsize: f32, label: &str,
        value: Option<&str>
    ) {
    //    let pleft = PAD_LEFT;
    //    if label {
    //        if iconid >= 0 {
    //            bnd_Icon(self, x+4.0, y+2.0, iconid);
    //            pleft += ICON_SHEET_RES;
    //        }
    //
    //        if bnd_font < 0 {return};
    //        self.font_face_id(bnd_font);
    //        self.font_size(fontsize);
    //        self.begin_path();
    //        self.fill_color(color);
    //        if value {
    //            let label_width = self.text_bounds(1.0, 1.0, label);
    //            let sep_width = self.text_bounds(1.0, 1.0,
    //                theme::LABEL_SEPARATOR);
    //
    //            self.text_align(nanovg::LEFT|nanovg::BASELINE);
    //            x += pleft as f32;
    //            if (align == CENTER) {
    //                let width = label_width + sep_width
    //                    + self.text_bounds(1.0, 1.0, value);
    //                x += ((w-(PAD_RIGHT-pleft) as f32)-width)*0.5;
    //            }
    //            y += h-TEXT_PAD_DOWN as f32;
    //            self.text(x, y, label);
    //            x += label_width;
    //            self.text(x, y, theme::LABEL_SEPARATOR);
    //            x += sep_width;
    //            self.text(x, y, value);
    //        } else {
    //            self.text_align(
    //                if align==LEFT  {nanovg::LEFT  |nanovg::BASELINE}
    //                else 				{nanovg::CENTER|nanovg::BASELINE});
    //            self.text_box(x+pleft as f32, y+h-TEXT_PAD_DOWN as f32,
    //                w-PAD_RIGHT as f32-pleft as f32, label);
    //        }
    //    } else if (iconid >= 0) {
    //        bnd_Icon(self, x+2.0, y+2.0, iconid);
    //    }
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
    fn bnd_IconLabelCaret(&mut self, x:f32,y:f32, w:f32,h:f32,
        iconid: i32, color: Color, fontsize: f32, label: &str,
        caretcolor: Color, cbegin: i32, cend: i32
    ) {
    //    let bounds: [f32, ..4];
    //    let pleft = theme::TEXT_RADIUS;
    //    if (!label) {return};
    //    if (iconid >= 0) {
    //        bnd_Icon(self, x+4.0, y+2.0, iconid);
    //        pleft += ICON_SHEET_RES as f32;
    //    }
    //
    //    if (bnd_font < 0) {return};
    //
    //    x+=pleft;
    //    y+=h-TEXT_PAD_DOWN as f32;
    //
    //    self.font_face_id(bnd_font);
    //    self.font_size(fontsize);
    //    self.text_align(nanovg::LEFT|nanovg::BASELINE);
    //
    //    if (cend >= cbegin) {
    //        //const char *cb;const char *ce;
    //        let /*static*/ glyphs: [NVGglyphPosition, ..theme::MAX_GLYPHS];
    //        let nglyphs = self.text_glyph_positions(
    //            x, y, label, label+cend+1, glyphs, theme::MAX_GLYPHS);
    //        let c0=glyphs[0].x;
    //        let c1=glyphs[nglyphs-1].x;
    //        let cb = label+cbegin;
    //        let ce = label+cend;
    //        // TODO: this is slow
    //        for i in range(0, nglyphs) {
    //            if (glyphs[i].str == cb) {
    //                c0 = glyphs[i].x;
    //            }
    //            if (glyphs[i].str == ce) {
    //                c1 = glyphs[i].x;
    //            }
    //        }
    //
    //        self.text_bounds(x, y, label, bounds);
    //        self.begin_path();
    //        if (cbegin == cend) {
    //            self.fill_color(nanovg::rgb_f(0.337, 0.502, 0.761));
    //            self.rect(c0-1.0, bounds[1], 2.0, bounds[3]-bounds[1]);
    //        } else {
    //            self.fill_color(caretcolor);
    //            self.rect(c0-1.0, bounds[1], c1-c0+1.0, bounds[3]-bounds[1]);
    //        }
    //        self.fill();
    //    }
    //
    //    self.begin_path();
    //    self.fill_color(color);
    //    self.text_box(x, y, w-theme::TEXT_RADIUS-pleft, label);
    }

    // Draw a checkmark for an option box with the given upper left coordinates
    // (ox, oy) with the specified color.
    fn bnd_Check(&mut self, ox: f32, oy: f32, color: Color
    ) {
        self.begin_path();
        self.stroke_width(2.0);
        self.stroke_color(color);
        self.line_cap(nanovg::BUTT);
        self.line_join(nanovg::MITER);
        self.move_to(ox+4.0, oy+5.0);
        self.line_to(ox+7.0, oy+8.0);
        self.line_to(ox+14.0, oy+1.0);
        self.stroke();
    }


    // Draw a horizontal arrow for a number field with its center at (x, y) and
    // size s; if s is negative, the arrow points to the left.
    fn bnd_Arrow(&mut self, x: f32, y: f32, s: f32, color: Color
    ) {
        self.begin_path();
        self.move_to(x, y);
        self.line_to(x-s, y+s);
        self.line_to(x-s, y-s);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }

    // Draw an up/down arrow for a choice box with its center at (x, y) and size s
    fn bnd_UpDownArrow(&mut self, x: f32, y: f32, s: f32, color: Color
    ) {
        self.begin_path();
        let w = 1.1*s;
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y-s-1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y+s+1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }
}
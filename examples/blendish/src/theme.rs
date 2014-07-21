
use libc::{c_double, c_float, c_int, c_char};
use libc::{c_uint, c_ushort, c_uchar, c_void};

//use nanovg::{Color};

use nanovg::{Color, rgba_f};
use super::ffi::{BNDtheme, BNDwidgetTheme};

////////////////////////////////////////////////////////////////////////////////

// default text size
pub static BND_LABEL_FONT_SIZE: c_float = 13.0;

// default text padding in inner box
pub static BND_PAD_LEFT: c_uint = 8;
pub static BND_PAD_RIGHT: c_uint = 8;

// label: value separator string
pub static BND_LABEL_SEPARATOR: &'static str = ": ";

// alpha intensity of transparent items (0xa4)
pub static BND_TRANSPARENT_ALPHA: c_float = 0.643;

// shade intensity of beveled panels (expressed in percentage, -100..100)
pub static BND_BEVEL_SHADE: c_int = 30;
// shade intensity of beveled insets
pub static BND_INSET_BEVEL_SHADE: c_int = 30;
// shade intensity of hovered inner boxes
pub static BND_HOVER_SHADE: c_int = 15;

// width of icon sheet
pub static BND_ICON_SHEET_WIDTH: c_uint = 602;
// height of icon sheet
pub static BND_ICON_SHEET_HEIGHT: c_uint = 640;
// gridsize of icon sheet in both dimensions
pub static BND_ICON_SHEET_GRID: c_uint = 21;
// offset of first icon tile relative to left border
pub static BND_ICON_SHEET_OFFSET_X: c_uint = 5;
// offset of first icon tile relative to top border
pub static BND_ICON_SHEET_OFFSET_Y: c_uint = 10;
// resolution of single icon
pub static BND_ICON_SHEET_RES: c_uint = 16;

// size of number field arrow
pub static BND_NUMBER_ARROW_SIZE: c_float = 4.0;

// default text color
pub static BND_COLOR_TEXT: Color = rgba_f( 0.0,0.0,0.0,1.0);
// default highlighted text color
pub static BND_COLOR_TEXT_SELECTED: Color = rgba_f( 1.0,1.0,1.0,1.0);

// radius of tool button
pub static BND_TOOL_RADIUS: c_float = 4.0;

// radius of option button
pub static BND_OPTION_RADIUS: c_float = 4.0;
// width of option button checkbox
pub static BND_OPTION_WIDTH: c_float = 14.0;
// height of option button checkbox
pub static BND_OPTION_HEIGHT: c_float = 15.0;

// radius of text field
pub static BND_TEXT_RADIUS: c_float = 4.0;

// radius of number button
pub static BND_NUMBER_RADIUS: c_float = 10.0;

// radius of menu popup
pub static BND_MENU_RADIUS: c_float = 3.0;
// feather of menu popup shadow
pub static BND_SHADOW_FEATHER: c_float = 12.0;
// alpha of menu popup shadow
pub static BND_SHADOW_ALPHA: c_float = 0.5;

// radius of scrollbar
pub static BND_SCROLLBAR_RADIUS: c_float = 7.0;
// shade intensity of active scrollbar
pub static BND_SCROLLBAR_ACTIVE_SHADE: c_float = 15.0;

// max glyphs for position testing
pub static BND_MAX_GLYPHS: c_uint = 1024;

// text distance from bottom
pub static BND_TEXT_PAD_DOWN: c_uint = 7;

////////////////////////////////////////////////////////////////////////////////

//BND_INLINE
pub fn bnd_clamp(v: c_float, mn: c_float, mx: c_float) -> c_float {
    if v>mx {mx} else { if v<mn {mn} else {v} };
}

////////////////////////////////////////////////////////////////////////////////


// the initial theme
pub static bnd_theme: BNDtheme = BNDtheme {
    // backgroundColor
    backgroundColor: rgba_f( 0.447, 0.447, 0.447, 1.0 ),
    regularTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.098,0.098,0.098,1.0 ), // color_outline
        itemColor: rgba_f( 0.098,0.098,0.098,1.0 ), // color_item
        innerColor: rgba_f( 0.6,0.6,0.6,1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.392,0.392,0.392,1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 0, // shade_top
        shadeDown: 0, // shade_down
    },
    toolTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.098,0.098,0.098, 1.0 ), // color_outline
        itemColor: rgba_f( 0.098,0.098,0.098, 1.0 ), // color_item
        innerColor: rgba_f( 0.6,0.6,0.6, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.392,0.392,0.392, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 15, // shade_top
        shadeDown: -15, // shade_down
    },
    radioTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 1.0,1.0,1.0, 1.0 ), // color_item
        innerColor: rgba_f( 0.275,0.275,0.275, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.337,0.502,0.761, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT_SELECTED, // color_text
        textSelectedColor: BND_COLOR_TEXT, // color_text_selected
        shadeTop: 15, // shade_top
        shadeDown: -15, // shade_down
    },
    textFieldTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.098,0.098,0.098, 1.0 ), // color_outline
        itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ), // color_item
        innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 0, // shade_top
        shadeDown: 25, // shade_down
    },
    optionTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 1.0,1.0,1.0, 1.0 ), // color_item
        innerColor: rgba_f( 0.275,0.275,0.275, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.275,0.275,0.275, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 15, // shade_top
        shadeDown: -15, // shade_down
    },
    choiceTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 1.0,1.0,1.0, 1.0 ), // color_item
        innerColor: rgba_f( 0.275,0.275,0.275, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.275,0.275,0.275, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT_SELECTED, // color_text
        textSelectedColor: rgba_f( 0.8,0.8,0.8, 1.0 ), // color_text_selected
        shadeTop: 15, // shade_top
        shadeDown: -15, // shade_down
    },
    numberFieldTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.098,0.098,0.098, 1.0 ), // color_outline
        itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ), // color_item
        innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: -20, // shade_top
        shadeDown: 0, // shade_down
    },
    sliderTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.098,0.098,0.098, 1.0 ), // color_outline
        itemColor: rgba_f( 0.502,0.502,0.502, 1.0 ), // color_item
        innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: -20, // shade_top
        shadeDown: 0, // shade_down
    },
    scrollBarTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.196,0.196,0.196, 1.0 ), // color_outline
        itemColor: rgba_f( 0.502,0.502,0.502, 1.0 ), // color_item
        innerColor: rgba_f( 0.314, 0.314, 0.314,0.706 ), // color_inner
        innerSelectedColor: rgba_f( 0.392, 0.392, 0.392,0.706 ), // color_inner_selected
        textColor: BND_COLOR_TEXT, // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 5, // shade_top
        shadeDown: -5, // shade_down
    },
    tooltipTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 0.392,0.392,0.392, 1.0 ), // color_item
        innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ), // color_inner
        innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ), // color_inner_selected
        textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ), // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 0, // shade_top
        shadeDown: 0, // shade_down
    },
    menuTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 0.392,0.392,0.392, 1.0 ), // color_item
        innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ), // color_inner
        innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ), // color_inner_selected
        textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ), // color_text
        textSelectedColor: BND_COLOR_TEXT_SELECTED, // color_text_selected
        shadeTop: 0, // shade_top
        shadeDown: 0, // shade_down
    },
    menuItemTheme: BNDwidgetTheme {
        outlineColor: rgba_f( 0.0,0.0,0.0, 1.0 ), // color_outline
        itemColor: rgba_f( 0.675,0.675,0.675,0.502 ), // color_item
        innerColor: rgba_f( 0.0,0.0,0.0, 0.0 ), // color_inner
        innerSelectedColor: rgba_f( 0.337,0.502,0.761, 1.0 ), // color_inner_selected
        textColor: BND_COLOR_TEXT_SELECTED, // color_text
        textSelectedColor: BND_COLOR_TEXT, // color_text_selected
        shadeTop: 38, // shade_top
        shadeDown: 0, // shade_down
    },
};

////////////////////////////////////////////////////////////////////////////////

pub fn bndSetTheme(theme: BNDtheme) {
    bnd_theme = theme;
}

pub fn bndGetTheme() -> *const BNDtheme {
    return &bnd_theme;
}

// the handle to the image containing the icon sheet
static bnd_icon_image: c_int = -1;

pub fn bndSetIconImage(image: c_int) {
    bnd_icon_image = image;
}

// the handle to the UI font
static bnd_font: c_int = -1;

pub fn bndSetFont(font: c_int) {
    bnd_font = font;
}



use nanovg::{Color, Ctx};
use super::ffi::{BNDtheme};

pub use WidgetTheme = super::ffi::BNDwidgetTheme;


fn rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color { Color::rgba_f(r, g, b, a) }

////////////////////////////////////////////////////////////////////////////////


// the initial theme
//pub static bnd_theme: BNDtheme =
pub fn initial_theme() -> BNDtheme {
    // default text color
    let TEXT: Color = rgba_f( 0.0, 0.0, 0.0, 1.0);
    // default highlighted text color
    let TEXT_SELECTED: Color = rgba_f( 1.0, 1.0, 1.0, 1.0);

    BNDtheme {
        // backgroundColor
        backgroundColor: rgba_f( 0.447, 0.447, 0.447, 1.0 ),
        regularTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 0, // shade_top
            shadeDown: 0, // shade_down
        },
        toolTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 15, // shade_top
            shadeDown: -15, // shade_down
        },
        radioTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.337, 0.502, 0.761, 1.0 ),
            textColor: TEXT_SELECTED,
            textSelectedColor: TEXT,
            shadeTop: 15, // shade_top
            shadeDown: -15, // shade_down
        },
        textFieldTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 0, // shade_top
            shadeDown: 25, // shade_down
        },
        optionTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 15, // shade_top
            shadeDown: -15, // shade_down
        },
        choiceTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            textColor: TEXT_SELECTED,
            textSelectedColor: rgba_f( 0.8, 0.8, 0.8, 1.0 ),
            shadeTop: 15, // shade_top
            shadeDown: -15, // shade_down
        },
        numberFieldTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ),
            innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: -20, // shade_top
            shadeDown: 0, // shade_down
        },
        sliderTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.502, 0.502, 0.502, 1.0 ),
            innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: -20, // shade_top
            shadeDown: 0, // shade_down
        },
        scrollBarTheme: WidgetTheme {
            outlineColor: rgba_f( 0.196, 0.196, 0.196, 1.0 ),
            itemColor: rgba_f( 0.502, 0.502, 0.502, 1.0 ),
            innerColor: rgba_f( 0.314, 0.314, 0.314, 0.706 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 0.706 ),
            textColor: TEXT,
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 5, // shade_top
            shadeDown: -5, // shade_down
        },
        tooltipTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ),
            innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ),
            textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ),
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 0, // shade_top
            shadeDown: 0, // shade_down
        },
        menuTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ),
            innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ),
            textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ),
            textSelectedColor: TEXT_SELECTED,
            shadeTop: 0, // shade_top
            shadeDown: 0, // shade_down
        },
        menuItemTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.675, 0.675, 0.675, 0.502 ),
            innerColor: rgba_f( 0.0, 0.0, 0.0, 0.0 ),
            innerSelectedColor: rgba_f( 0.337, 0.502, 0.761, 1.0 ),
            textColor: TEXT_SELECTED,
            textSelectedColor: TEXT,
            shadeTop: 38, // shade_top
            shadeDown: 0, // shade_down
        },
    }
}

////////////////////////////////////////////////////////////////////////////////
/// describes the theme used to draw widgets

pub struct Theme<'a> {
    pub bnd_theme: BNDtheme,

    nvg: Ctx,

    pub bnd_icon_image: i32, // handle, icon image spritesheet
    pub bnd_font: i32,       // handle
}

impl<'a> Theme<'a> {
    pub fn new(nvg: Ctx) -> Theme<'a> {
        Theme {
            nvg: nvg,
            bnd_theme: initial_theme(),
            bnd_icon_image: -1,
            bnd_font: -1
        }
    }

    pub fn nvg(&mut self) -> &mut Ctx { &mut self.nvg }

    pub fn theme(&self) -> &BNDtheme { &self.bnd_theme }

}

//pub fn bndSetTheme(theme: BNDtheme) {
//    bnd_theme = theme;
//}
//
//pub fn bndGetTheme<'a>() -> &'a BNDtheme {
//    return &bnd_theme;
//}
//
//// the handle to the image containing the icon sheet
//static bnd_icon_image: c_int = -1;
//
//pub fn bndSetIconImage(image: c_int) {
//    bnd_icon_image = image;
//}
//
//// the handle to the UI font
//static bnd_font: c_int = -1;
//
//pub fn bndSetFont(font: c_int) {
//    bnd_font = font;
//}

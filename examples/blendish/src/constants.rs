
/// default widget height
pub static WIDGET_HEIGHT: u32 = 21;

/// default horizontal spacing
pub static HSPACING: u32 = 8;
/// default vertical spacing
pub static VSPACING: u32 = 1;
/// default vertical spacing between groups
pub static VSPACING_GROUP: u32 = 8;

/// default text padding in inner box
pub static PAD_LEFT: u32 = 8;
pub static PAD_RIGHT: u32 = 8;
/// text distance from bottom
pub static TEXT_PAD_DOWN: u32 = 7;


/// width of vertical scrollbar
pub static SCROLLBAR_WIDTH: u32 = 13;
/// height of horizontal scrollbar
pub static SCROLLBAR_HEIGHT: u32 = 14;
/// radius of scrollbar
pub static SCROLLBAR_RADIUS: f32 = 7.0;
/// shade intensity of active scrollbar (percentage delta, -100..100)
pub static SCROLLBAR_ACTIVE_SHADE: i32 = 15;	// percent, -100..100


/// alpha of disabled widget groups
/// can be used in conjunction with nvgGlobalAlpha()
pub static DISABLED_ALPHA: f32 = 0.5;
/// alpha intensity of transparent items (0xa4)
pub static TRANSPARENT_ALPHA: f32 = 0.643;


/// default text size
pub static LABEL_FONT_SIZE: f32 = 13.0;
/// label: value separator string
pub static LABEL_SEPARATOR: &'static str = ": ";


/// shade intensity of beveled panels (expressed in percentage, -100..100)
pub static BEVEL_SHADE: i32 = 30;			// percent, -100..100
/// shade intensity of beveled insets
pub static INSET_BEVEL_SHADE: i32 = 30;		// percent, -100..100
/// shade intensity of hovered inner boxes
pub static HOVER_SHADE: i32 = 15;			// percent, -100..100


/// size of number field arrow
pub static NUMBER_ARROW_SIZE: f32 = 4.0;

/// default text color
/// TODO fix Color so that it can be struct-initialized
///pub static COLOR_TEXT: Color = rgba_f( 0.0, 0.0, 0.0, 1.0);
/// default highlighted text color
///pub static COLOR_TEXT_SELECTED: Color = rgba_f( 1.0, 1.0, 1.0, 1.0);

/// default toolbutton width (if icon only)
pub static TOOL_WIDTH: u32 = 20;
/// radius of tool button
pub static TOOL_RADIUS: f32 = 4.0;

/// radius of option button
pub static OPTION_RADIUS: f32 = 4.0;
/// width of option button checkbox
pub static OPTION_WIDTH: f32 = 14.0;
/// height of option button checkbox
pub static OPTION_HEIGHT: f32 = 15.0;

/// radius of text field
pub static TEXT_RADIUS: f32 = 4.0;

/// radius of number button
pub static NUMBER_RADIUS: f32 = 10.0;

/// radius of menu popup
pub static MENU_RADIUS: f32 = 3.0;
/// feather of menu popup shadow
pub static SHADOW_FEATHER: f32 = 12.0;
/// alpha of menu popup shadow
pub static SHADOW_ALPHA: f32 = 0.5;


/// max glyphs for position testing
pub static MAX_GLYPHS: u32 = 1024;


/// width of icon sheet
pub static ICON_SHEET_WIDTH: u32 = 602;
/// height of icon sheet
pub static ICON_SHEET_HEIGHT: u32 = 640;
/// gridsize of icon sheet in both dimensions
pub static ICON_SHEET_GRID: u32 = 21;
/// offset of first icon tile relative to left border
pub static ICON_SHEET_OFFSET_X: u32 = 5;
/// offset of first icon tile relative to top border
pub static ICON_SHEET_OFFSET_Y: u32 = 10;
/// resolution of single icon
pub static ICON_SHEET_RES: u32 = 16;

extern crate libc;
extern crate gl;
extern crate png;

use std::vec::Vec;

use nanovg::*;
use gl::{ReadPixels, RGBA, UNSIGNED_BYTE};
use std::os::raw::{c_void};

/// use unicode characters for icons
const NO_ICON:            char = '\0';
const ICON_SEARCH:        char = '\u{01F50D}';
const ICON_CIRCLED_CROSS: char = '\u{2716}';
const ICON_CHEVRON_RIGHT: char = '\u{E75E}';
const ICON_CHECK:         char = '\u{2713}';
const ICON_LOGIN:         char = '\u{E740}';
const ICON_TRASH:         char = '\u{E729}';

const PI: f32 = 3.1415926535;

fn min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }
fn max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
//fn abs(a: f32) -> f32 { if a >= 0.0 { a } else { -a } }
fn clamp(a: f32, mn: f32, mx: f32) -> f32 { if a < mn { mn } else { if a > mx { mx } else { a } } }
fn floor(x: f32) -> f32 { x.floor() }
fn sqrt(x: f32) -> f32 { x.sqrt() }
fn cos(x: f32) -> f32 { x.cos() }
fn sin(x: f32) -> f32 { x.sin() }

fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color { Color::rgba(r,g,b,a) }
fn hsla(h:f32, s:f32, l:f32, a:u8) -> Color { Color::hsla(h,s,l,a) }



fn cp_to_utf8(cp:char) -> String { format!{"{}", cp} }

pub struct DemoData {
    //vg: &Context,
    fontNormal: Font,
    fontBold: Font,
    fontIcons: Font,
    images: Vec<Image>
}

/// load and hold resources used in demo
impl DemoData {
    pub fn load(vg: &Context, res_path: &str) -> DemoData
    {
        let mut images: Vec<Image> = Vec::new();
        for i in 0..12 {
            let filename = format!{"{}/images/image{}.jpg", res_path, i+1};
            let img = vg.create_image(&filename)
                .expect(&format!{"Could not load {}.", filename});
            images.push(img);
        }

        let fontIcons = vg.create_font("icons", &format!{"{}/entypo.ttf", res_path})
            .expect("Could not add font 'icons'.");

        let fontNormal = vg.create_font("sans", &format!{"{}/Roboto-Regular.ttf", res_path})
            .expect("Could not add font 'sans'.");

        let fontBold = vg.create_font("sans-bold", &format!{"{}/Roboto-Bold.ttf", res_path})
            .expect("Could not add font 'sans-bold'.");


        DemoData {
            fontNormal: fontNormal,
            fontBold:   fontBold,
            fontIcons:  fontIcons,
            images:     images
        }
    }
}



pub fn render_demo(vg: &Context, mx: f32, my: f32, width: f32, height: f32, t: f32, blowup: bool, data: &DemoData)
{
    draw_eyes(vg, width - 250.0, 50.0, 150.0, 100.0, mx, my, t);
    draw_paragraph(vg, width - 450.0, 50.0, 150.0, 100.0, mx, my);
    draw_graph(vg, 0.0, height/2.0, width, height/2.0, t);
    draw_colorwheel(vg, width - 300.0, height - 300.0, 250.0, 250.0, t);

    // Line joints
    draw_lines(vg, 50.0, height-50.0, 600.0, 50.0, t);

    // Line caps
    draw_widths(vg, 10.0, 50.0, 30.0);

    // Line caps
    draw_caps(vg, 10.0, 300.0, 30.0);

    vg.save();
    if blowup {
        vg.rotate(sin(t*0.3)*5.0/180.0*PI);
        vg.scale(2.0, 2.0);
    }

    // Widgets
    draw_window(vg, "Widgets `n Stuff", 50.0, 50.0, 300.0, 400.0);
    let x = 60.0; let mut y = 95.0;
    draw_searchbox(vg, "Search", x,y,280.0,25.0);
    y += 40.0;
    draw_dropdown(vg, "Effects", x,y,280.0,28.0);
    let popy = y + 14.0;
    y += 45.0;

    // Form
    draw_label(vg, "Login", x,y, 280.0,20.0);
    y += 25.0;
    draw_editbox(vg, "Email",  x,y, 280.0,28.0);
    y += 35.0;
    draw_editbox(vg, "Password", x,y, 280.0,28.0);
    y += 38.0;
    draw_checkbox(vg, "Remember me", x,y, 140.0,28.0);
    draw_button(vg, ICON_LOGIN, "Sign in", x+138.0, y, 140.0, 28.0, rgba(0,96,128,255));
    y += 45.0;

    // Slider
    draw_label(vg, "Diameter", x,y, 280.0,20.0);
    y += 25.0;
    draw_editbox_num(vg, "123.00", "px", x+180.0,y, 100.0,28.0);
    draw_slider(vg, 0.4, x,y, 170.0,28.0);
    y += 55.0;

    draw_button(vg, ICON_TRASH, "Delete", x, y, 160.0, 28.0, rgba(128,16,8,255));
    draw_button(vg, NO_ICON, "Cancel", x+170.0, y, 110.0, 28.0, rgba(0,0,0,0));

    // Thumbnails box
    draw_thumbnails(vg, 365.0, popy-30.0, 160.0, 300.0, &data.images, t);

    vg.restore();
}


fn is_black(col: Color) -> bool {
    col.r() == 0.0 && col.g() == 0.0 && col.b() == 0.0 && col.a() == 0.0
}

fn draw_window(vg: &Context, title: &str, x: f32, y: f32, w: f32, h: f32)
{
    let cornerRadius = 3.0;

    vg.save();
//    vg.clear_state();

    // Window
    vg.begin_path();
    vg.rounded_rect(x,y, w,h, cornerRadius);
    vg.fill_color(rgba(28,30,34,192));
//    vg.fill_color(rgba(0,0,0,128));
    vg.fill();

    // Drop shadow
    let shadowPaint = vg.box_gradient(x,y+2.0, w,h, cornerRadius*2.0, 10.0, rgba(0,0,0,128), rgba(0,0,0,0));
    vg.begin_path();
    vg.rect(x-10.0,y-10.0, w+20.0,h+30.0);
    vg.rounded_rect(x,y, w,h, cornerRadius);
    vg.path_winding(Solidity::HOLE);
    vg.fill_paint(shadowPaint);
    vg.fill();

    // Header
    let headerPaint = vg.linear_gradient(x,y,x,y+15.0, rgba(255,255,255,8), rgba(0,0,0,16));
    vg.begin_path();
    vg.rounded_rect(x+1.0,y+1.0, w-2.0,30.0, cornerRadius-1.0);
    vg.fill_paint(headerPaint);
    vg.fill();
    vg.begin_path();
    vg.move_to(x+0.5, y+0.5+30.0);
    vg.line_to(x+0.5+w-1.0, y+0.5+30.0);
    vg.stroke_color(rgba(0,0,0,32));
    vg.stroke();

    vg.font_size(18.0);
    vg.font_face("sans-bold");
    vg.text_align(CENTER|MIDDLE);

    vg.font_blur(2.0);
    vg.fill_color(rgba(0,0,0,128));
    vg.text(x+w/2.0,y+16.0+1.0, title);

    vg.font_blur(0.0);
    vg.fill_color(rgba(220,220,220,160));
    vg.text(x+w/2.0,y+16.0, title);

    vg.restore();
}

fn draw_searchbox(vg: &Context, text: &str, x: f32, y: f32, w: f32, h: f32)
{
    let cornerRadius = h/2.0 - 1.0;

    // Edit
    let bg = vg.box_gradient(x,y+1.5, w,h, h/2.0,5.0, rgba(0,0,0,16), rgba(0,0,0,92));
    vg.begin_path();
    vg.rounded_rect(x,y, w,h, cornerRadius);
    vg.fill_paint(bg);
    vg.fill();

  /*    vg.begin_path();
    vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
    vg.stroke_color(rgba(0,0,0,48));
    vg.stroke();*/

    vg.font_size(h*1.3);
    vg.font_face("icons");
    vg.fill_color(rgba(255,255,255,64));
    vg.text_align(CENTER|MIDDLE);
    vg.text(x+h*0.55, y+h*0.55, &cp_to_utf8(ICON_SEARCH));

    vg.font_size(20.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,32));

    vg.text_align(LEFT|MIDDLE);
    vg.text(x+h*1.05,y+h*0.5,text);

    vg.font_size(h*1.3);
    vg.font_face("icons");
    vg.fill_color(rgba(255,255,255,32));
    vg.text_align(CENTER|MIDDLE);
    vg.text(x+w-h*0.55, y+h*0.55, &cp_to_utf8(ICON_CIRCLED_CROSS));
}

fn draw_dropdown(vg: &Context, text: &str, x: f32, y: f32, w: f32, h: f32)
{
    let cornerRadius = 4.0;

    let bg = vg.linear_gradient(x,y,x,y+h, rgba(255,255,255,16), rgba(0,0,0,16));
    vg.begin_path();
    vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, cornerRadius-1.0);
    vg.fill_paint(bg);
    vg.fill();

    vg.begin_path();
    vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, cornerRadius-0.5);
    vg.stroke_color(rgba(0,0,0,48));
    vg.stroke();

    vg.font_size(20.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,160));
    vg.text_align(LEFT|MIDDLE);
    vg.text(x+h*0.3,y+h*0.5,text);

    vg.font_size(h*1.3);
    vg.font_face("icons");
    vg.fill_color(rgba(255,255,255,64));
    vg.text_align(CENTER|MIDDLE);
    vg.text(x+w-h*0.5, y+h*0.5, &cp_to_utf8(ICON_CHEVRON_RIGHT));
}

fn draw_label(vg: &Context, text: &str, x: f32, y: f32, w: f32, h: f32)
{
    vg.font_size(18.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,128));

    vg.text_align(LEFT|MIDDLE);
    vg.text(x,y+h*0.5,text);
}

fn draw_editbox_base(vg: &Context, x: f32, y: f32, w: f32, h: f32)
{
    // Edit
    let bg = vg.box_gradient(x+1.0,y+1.0+1.5, w-2.0,h-2.0, 3.0,4.0, rgba(255,255,255,32), rgba(32,32,32,32));
    vg.begin_path();
    vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, 4.0-1.0);
    vg.fill_paint(bg);
    vg.fill();

    vg.begin_path();
    vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, 4.0-0.5);
    vg.stroke_color(rgba(0,0,0,48));
    vg.stroke();
}

fn draw_editbox(vg: &Context, text: &str, x: f32, y: f32, w: f32, h: f32)
{
    draw_editbox_base(vg, x,y, w,h);

    vg.font_size(20.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,64));
    vg.text_align(LEFT|MIDDLE);
    vg.text(x+h*0.3,y+h*0.5,text);
}

fn draw_editbox_num(vg: &Context, text: &str, units: &str, x: f32, y: f32, w: f32, h: f32)
{
    draw_editbox_base(vg, x,y, w,h);

    let mut bounds = [0.0; 4];
    let uw = vg.text_bounds(0.0,0.0, units, &mut bounds);

    vg.font_size(18.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,64));
    vg.text_align(RIGHT|MIDDLE);
    vg.text(x+w-h*0.3,y+h*0.5,units);

    vg.font_size(20.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,128));
    vg.text_align(RIGHT|MIDDLE);
    vg.text(x+w-uw-h*0.5,y+h*0.5,text);
}

fn draw_checkbox(vg: &Context, text: &str, x: f32, y: f32, w: f32, h: f32)
{
    vg.font_size(18.0);
    vg.font_face("sans");
    vg.fill_color(rgba(255,255,255,160));

    vg.text_align(LEFT|MIDDLE);
//    vg.text(x+28,y+h*0.5,text, NULL);

    let bg = vg.box_gradient(x+1.0,y+floor(h*0.5)-9.0+1.0, 18.0,18.0, 3.0,3.0, rgba(0,0,0,32), rgba(0,0,0,92));
    vg.begin_path();
    vg.rounded_rect(x+1.0,y+floor(h*0.5)-9.0, 18.0,18.0, 3.0);
    vg.fill_paint(bg);
    vg.fill();

    vg.font_size(40.0);
    vg.font_face("icons");
    vg.fill_color(rgba(255,255,255,128));
    vg.text_align(CENTER|MIDDLE);
//    vg.text(x+9+2, y+h*0.5, cp_to_utf8(ICON_CHECK,icon), NULL);
}

fn draw_button(vg: &Context, preicon: char, text: &str, x: f32, y: f32, w: f32, h: f32, col: Color)
{
    let cornerRadius = 4.0;

    let bg = vg.linear_gradient(
        x,y,x,y+h,
        rgba(255,255,255,
        if is_black(col) {
            16
        } else {
            32
        }),
        rgba(0,0,0,if is_black(col){16}else{32}));
    vg.begin_path();
    vg.rounded_rect(x+1.0,y+1.0, w-2.0,h-2.0, cornerRadius-1.0);
    if !is_black(col) {
        vg.fill_color(col);
        vg.fill();
    }
    vg.fill_paint(bg);
    vg.fill();

    vg.begin_path();
    vg.rounded_rect(x+0.5,y+0.5, w-1.0,h-1.0, cornerRadius-0.5);
    vg.stroke_color(rgba(0,0,0,48));
    vg.stroke();

    vg.font_size(20.0);
    vg.font_face("sans-bold");
    let mut bounds = [0.0; 4];
    let tw = vg.text_bounds(0.0,0.0, text, &mut bounds);
    let mut iw = 0.0;
    if preicon != NO_ICON {
        vg.font_size(h*1.3);
        vg.font_face("icons");
        iw = vg.text_bounds(0.0,0.0, &cp_to_utf8(preicon), &mut bounds);
        iw += h*0.15;
    }

    if preicon != NO_ICON {
        vg.font_size(h*1.3);
        vg.font_face("icons");
        vg.fill_color(rgba(255,255,255,96));
        vg.text_align(LEFT|MIDDLE);
        vg.text(x+w*0.5-tw*0.5-iw*0.75, y+h*0.5, &cp_to_utf8(preicon));
    }

    vg.font_size(20.0);
    vg.font_face("sans-bold");
    vg.text_align(LEFT|MIDDLE);
    vg.fill_color(rgba(0,0,0,160));
    vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5-1.0,text);
    vg.fill_color(rgba(255,255,255,160));
    vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5,text);
}

fn draw_slider(vg: &Context, pos: f32, x: f32, y: f32, w: f32, h: f32)
{
    let cy: f32 = y+floor(h*0.5);
    let kr: f32 = floor(h*0.25);

    vg.save();
//    vg.clear_state();

    // Slot
    let bg = vg.box_gradient(x,cy-2.0+1.0, w,4.0, 2.0,2.0, rgba(0,0,0,32), rgba(0,0,0,128));
    vg.begin_path();
    vg.rounded_rect(x,cy-2.0, w,4.0, 2.0);
    vg.fill_paint(bg);
    vg.fill();

    // Knob Shadow
    let shadow = vg.radial_gradient(x+floor(pos*w),cy+1.0, kr-3.0,kr+3.0, rgba(0,0,0,64), rgba(0,0,0,0));
    vg.begin_path();
    vg.rect(x+floor(pos*w)-kr-5.0,cy-kr-5.0,kr*2.0+5.0+5.0,kr*2.0+5.0+5.0+3.0);
    vg.circle(x+floor(pos*w),cy, kr);
    vg.path_winding(Solidity::HOLE);
    vg.fill_paint(shadow);
    vg.fill();

    // Knob
    let knob = vg.linear_gradient(x,cy-kr,x,cy+kr, rgba(255,255,255,16), rgba(0,0,0,16));
    vg.begin_path();
    vg.circle(x+floor(pos*w),cy, kr-1.0);
    vg.fill_color(rgba(40,43,48,255));
    vg.fill();
    vg.fill_paint(knob);
    vg.fill();

    vg.begin_path();
    vg.circle(x+floor(pos*w),cy, kr-0.5);
    vg.stroke_color(rgba(0,0,0,92));
    vg.stroke();

    vg.restore();
}

fn draw_eyes(vg: &Context, x: f32,
            y: f32, w: f32,
            h: f32, mx: f32,
            my: f32, t: f32)
{
    let ex = w *0.23;
    let ey = h * 0.5;
    let lx = x + ex;
    let ly = y + ey;
    let rx = x + w - ex;
    let ry = y + ey;
    let br = min(ex, ey) * 0.5;
    let blink: f32 = 1.0 - (t*0.5).sin().powi(200)*0.8;

    let bg = vg.linear_gradient(x,y+h*0.5,x+w*0.1,y+h, rgba(0,0,0,32), rgba(0,0,0,16));
    vg.begin_path();
    vg.ellipse(lx+3.0,ly+16.0, ex,ey);
    vg.ellipse(rx+3.0,ry+16.0, ex,ey);
    vg.fill_paint(bg);
    vg.fill();

    let shadow = vg.linear_gradient(x,y+h*0.25,x+w*0.1,y+h, rgba(220,220,220,255), rgba(128,128,128,255));
    vg.begin_path();
    vg.ellipse(lx,ly, ex,ey);
    vg.ellipse(rx,ry, ex,ey);
    vg.fill_paint(shadow);
    vg.fill();

    let mut dx = (mx - rx) / (ex * 10.0);
    let mut dy = (my - ry) / (ey * 10.0);
    let mut d = sqrt(dx*dx+dy*dy);
    if d > 1.0 {
        dx /= d; dy /= d;
    }
    dx *= ex*0.4;
    dy *= ey*0.5;
    vg.begin_path();
    vg.ellipse(lx+dx,ly+dy+ey*0.25*(1.0-blink), br,br*blink);
    vg.fill_color(rgba(32,32,32,255));
    vg.fill();

    dx = (mx - rx) / (ex * 10.0);
    dy = (my - ry) / (ey * 10.0);
    d = sqrt(dx*dx+dy*dy);
    if d > 1.0 {
        dx /= d; dy /= d;
    }
    dx *= ex*0.4;
    dy *= ey*0.5;
    vg.begin_path();
    vg.ellipse(rx+dx,ry+dy+ey*0.25*(1.0-blink), br,br*blink);
    vg.fill_color(rgba(32,32,32,255));
    vg.fill();

    let lgloss = vg.radial_gradient(lx-ex*0.25,ly-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
    vg.begin_path();
    vg.ellipse(lx,ly, ex,ey);
    vg.fill_paint(lgloss);
    vg.fill();

    let rgloss = vg.radial_gradient(rx-ex*0.25,ry-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
    vg.begin_path();
    vg.ellipse(rx,ry, ex,ey);
    vg.fill_paint(rgloss);
    vg.fill();
}

fn draw_graph(vg: &Context, x: f32,
             y: f32, w: f32,
             h: f32, t: f32)
{
    let mut samples: [f32; 6] = [0.0; 6];
    let mut sx: [f32; 6] = [0.0; 6];
    let mut sy: [f32; 6] = [0.0; 6];
    let dx = w/5.0;

    samples[0] = (1.0+sin(t*1.2345+cos(t*0.33457)*0.44))*0.5;
    samples[1] = (1.0+sin(t*0.68363+cos(t*1.3)*1.55))*0.5;
    samples[2] = (1.0+sin(t*1.1642+cos(t*0.33457)*1.24))*0.5;
    samples[3] = (1.0+sin(t*0.56345+cos(t*1.63)*0.14))*0.5;
    samples[4] = (1.0+sin(t*1.6245+cos(t*0.254)*0.3))*0.5;
    samples[5] = (1.0+sin(t*0.345+cos(t*0.03)*0.6))*0.5;

    for i in 0..6 {
        sx[i] = x+ (i as f32)*dx;
        sy[i] = y+h*samples[i]*0.8;
    }

    // Graph background
    let bg = vg.linear_gradient(x,y,x,y+h, rgba(0,160,192,0), rgba(0,160,192,64));
    vg.begin_path();
    vg.move_to(sx[0], sy[0]);
    for i in 1..6 {
        vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
    }
    vg.line_to(x+w, y+h);
    vg.line_to(x, y+h);
    vg.fill_paint(bg);
    vg.fill();

    // Graph line
    vg.begin_path();
    vg.move_to(sx[0], sy[0]+2.0);
    for i in 1..6 {
        vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1]+2.0, sx[i]-dx*0.5,sy[i]+2.0, sx[i],sy[i]+2.0);
    }
    vg.stroke_color(rgba(0,0,0,32));
    vg.stroke_width(3.0);
    vg.stroke();

    vg.begin_path();
    vg.move_to(sx[0], sy[0]);
    for i in 1..6 {
        vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
    }
    vg.stroke_color(rgba(0,160,192,255));
    vg.stroke_width(3.0);
    vg.stroke();

    // Graph sample pos
    for i in 0..6 {
        let bg = vg.radial_gradient(sx[i],sy[i]+2.0, 3.0,8.0, rgba(0,0,0,32), rgba(0,0,0,0));
        vg.begin_path();
        vg.rect(sx[i]-10.0, sy[i]-10.0+2.0, 20.0,20.0);
        vg.fill_paint(bg);
        vg.fill();
    }

    vg.begin_path();
    for i in 0..6 {
        vg.circle(sx[i], sy[i], 4.0);
    }
    vg.fill_color(rgba(0,160,192,255));
    vg.fill();
    vg.begin_path();
    for i in 0..6 {
        vg.circle(sx[i], sy[i], 2.0);
    }
    vg.fill_color(rgba(220,220,220,255));
    vg.fill();

    vg.stroke_width(1.0);
}

fn draw_spinner(vg: &Context, cx: f32, cy: f32, r: f32, t: f32)
{
    let a0 = 0.0 + t*6.0;
    let a1 = PI + t*6.0;
    let r0 = r;
    let r1 = r * 0.75;

    vg.save();

    vg.begin_path();
    vg.arc(cx,cy, r0, a0, a1, Winding::CW);
    vg.arc(cx,cy, r1, a1, a0, Winding::CCW);
    vg.close_path();
    let ax = cx + cos(a0) * (r0+r1)*0.5;
    let ay = cy + sin(a0) * (r0+r1)*0.5;
    let bx = cx + cos(a1) * (r0+r1)*0.5;
    let by = cy + sin(a1) * (r0+r1)*0.5;
    let paint = vg.linear_gradient(ax,ay, bx,by, rgba(0,0,0,0), rgba(0,0,0,128));
    vg.fill_paint(paint);
    vg.fill();

    vg.restore();
}

fn draw_thumbnails(vg: &Context, x: f32, y: f32, w: f32, h: f32,
                  images: &Vec<Image>, t: f32)
{
    let nimages = images.len();
    let cornerRadius = 3.0;

    let thumb: f32 = 60.0;
    let arry : f32 = 30.5;
    let stackh: f32 = (nimages/2) as f32 * (thumb+10.0) + 10.0;
    let u : f32 = (1.0+cos(t*0.5))*0.5;
    let u2: f32 = (1.0-cos(t*0.2))*0.5;

    vg.save();
//    vg.clear_state();

    // Drop shadow
    let mut shadowPaint = vg.box_gradient(x,y+4.0, w,h, cornerRadius*2.0, 20.0, rgba(0,0,0,128), rgba(0,0,0,0));
    vg.begin_path();
    vg.rect(x-10.0,y-10.0, w+20.0,h+30.0);
    vg.rounded_rect(x,y, w,h, cornerRadius);
    vg.path_winding(Solidity::HOLE);
    vg.fill_paint(shadowPaint);
    vg.fill();

    // Window
    vg.begin_path();
    vg.rounded_rect(x,y, w,h, cornerRadius);
    vg.move_to(x-10.0,y+arry);
    vg.line_to(x+1.0,y+arry-11.0);
    vg.line_to(x+1.0,y+arry+11.0);
    vg.fill_color(rgba(200,200,200,255));
    vg.fill();

    vg.save();
    vg.scissor(x,y,w,h);
    vg.translate(0.0, -(stackh-h)*u);

    let dv = 1.0 / (nimages as f32 - 1.0);

    for i in 0..nimages {
        let mut tx = x+10.0;
        let mut ty = y+10.0;
        tx += (i%2) as f32 * (thumb+10.0);
        ty += (i/2) as f32 * (thumb+10.0);
        let imgw: i32 = 0;
        let imgh: i32 = 0;
        let ix: f32;
        let iy: f32;
        let iw: f32;
        let ih: f32;
        let (imgw, imgh) = vg.image_size(&images[i]);
        if imgw < imgh {
            iw = thumb;
            ih = iw * (imgh as f32) / (imgw as f32);
            ix = 0.0;
            iy = -(ih-thumb)*0.5;
        } else {
            ih = thumb;
            iw = ih * (imgw as f32) / (imgh as f32);
            ix = -(iw-thumb)*0.5;
            iy = 0.0;
        }

        let v = i as f32 * dv;
        let a = clamp((u2-v) / dv, 0.0, 1.0);

        if a < 1.0 {
            draw_spinner(vg, tx+thumb/2.0,ty+thumb/2.0, thumb*0.25, t);
        }

        let imgPaint = vg.image_pattern(tx+ix, ty+iy, iw,ih, 0.0/180.0*PI, &images[i], PatternRepeat::NOREPEAT, a);
        vg.begin_path();
        vg.rounded_rect(tx,ty, thumb,thumb, 5.0);
        vg.fill_paint(imgPaint);
        vg.fill();

        shadowPaint = vg.box_gradient(tx-1.0,ty, thumb+2.0,thumb+2.0, 5.0, 3.0, rgba(0,0,0,128), rgba(0,0,0,0));
        vg.begin_path();
        vg.rect(tx-5.0,ty-5.0, thumb+10.0,thumb+10.0);
        vg.rounded_rect(tx,ty, thumb,thumb, 6.0);
        vg.path_winding(Solidity::HOLE);
        vg.fill_paint(shadowPaint);
        vg.fill();

        vg.begin_path();
        vg.rounded_rect(tx+0.5,ty+0.5, thumb-1.0,thumb-1.0, 4.0-0.5);
        vg.stroke_width(1.0);
        vg.stroke_color(rgba(255,255,255,192));
        vg.stroke();
    }
    vg.restore();

    // Hide fades
    let mut fadePaint = vg.linear_gradient(x,y,x,y+6.0, rgba(200,200,200,255), rgba(200,200,200,0));
    vg.begin_path();
    vg.rect(x+0.4,y,w-8.0,6.0);
    vg.fill_paint(fadePaint);
    vg.fill();

    fadePaint = vg.linear_gradient(x,y+h,x,y+h-6.0, rgba(200,200,200,255), rgba(200,200,200,0));
    vg.begin_path();
    vg.rect(x+4.0,y+h-6.0,w-8.0,6.0);
    vg.fill_paint(fadePaint);
    vg.fill();

    // Scroll bar
    shadowPaint = vg.box_gradient(x+w-12.0+1.0,y+4.0+1.0, 8.0,h-8.0, 3.0,4.0, rgba(0,0,0,32), rgba(0,0,0,92));
    vg.begin_path();
    vg.rounded_rect(x+w-12.0,y+4.0, 8.0,h-8.0, 3.0);
    vg.fill_paint(shadowPaint);
//    vg.fill_color(rgba(255,0,0,128));
    vg.fill();

    let scrollh = (h/stackh) * (h-8.0);
    shadowPaint = vg.box_gradient(x+w-12.0-1.0,y+4.0+(h-8.0-scrollh)*u-1.0, 8.0,scrollh, 3.0,4.0, rgba(220,220,220,255), rgba(128,128,128,255));
    vg.begin_path();
    vg.rounded_rect(x+w-12.0+1.0,y+4.0+1.0 + (h-8.0-scrollh)*u, 8.0-2.0,scrollh-2.0, 2.0);
    vg.fill_paint(shadowPaint);
//    vg.fill_color(rgba(0,0,0,128));
    vg.fill();

    vg.restore();
}

fn draw_colorwheel(vg: &Context, x: f32,
                  y: f32, w: f32,
                  h: f32, t: f32)
{
    //f32 r0, r1, ax,ay, bx,by, cx,cy, aeps, r;
    let hue = sin(t * 0.12);

    vg.save();

  /*    vg.begin_path();
    vg.rect(x,y,w,h);
    vg.fill_color(rgba(255,0,0,128));
    vg.fill();*/

    let cx = x + w*0.5;
    let cy = y + h*0.5;
    let r1 = min(w,h) * 0.5 - 5.0;
    let r0 = r1 - 20.0;
    let aeps = 0.5 / r1;    // half a pixel arc length in radians (2pi cancels out).

    for i in 0..6 {
        let a0 = (i as f32) / 6.0 * PI * 2.0 - aeps;
        let a1 = ((i as f32)+1.0) / 6.0 * PI * 2.0 + aeps;
        vg.begin_path();
        vg.arc(cx,cy, r0, a0, a1, Winding::CW);
        vg.arc(cx,cy, r1, a1, a0, Winding::CCW);
        vg.close_path();
        let ax = cx + cos(a0) * (r0+r1)*0.5;
        let ay = cy + sin(a0) * (r0+r1)*0.5;
        let bx = cx + cos(a1) * (r0+r1)*0.5;
        let by = cy + sin(a1) * (r0+r1)*0.5;
        let paint = vg.linear_gradient(ax,ay, bx,by, hsla(a0/(PI*2.0),1.0,0.55,255), hsla(a1/(PI*2.0),1.0,0.55,255));
        vg.fill_paint(paint);
        vg.fill();
    }

    vg.begin_path();
    vg.circle(cx,cy, r0-0.5);
    vg.circle(cx,cy, r1+0.5);
    vg.stroke_color(rgba(0,0,0,64));
    vg.stroke_width(1.0);
    vg.stroke();

    // Selector
    vg.save();
    vg.translate(cx,cy);
    vg.rotate(hue*PI*2.0);

    // Marker on
    vg.stroke_width(2.0);
    vg.begin_path();
    vg.rect(r0-1.0,-3.0,r1-r0+2.0,6.0);
    vg.stroke_color(rgba(255,255,255,192));
    vg.stroke();

    let mut paint = vg.box_gradient(r0-3.0,-5.0,r1-r0+6.0,10.0, 2.0,4.0, rgba(0,0,0,128), rgba(0,0,0,0));
    vg.begin_path();
    vg.rect(r0-2.0-10.0,-4.0-10.0,r1-r0+4.0+20.0,8.0+20.0);
    vg.rect(r0-2.0,-4.0,r1-r0+4.0,8.0);
    vg.path_winding(Solidity::HOLE);
    vg.fill_paint(paint);
    vg.fill();

    // Center triangle
    let r = r0 - 6.0;
    let mut ax = cos(120.0/180.0*PI) * r;
    let mut ay = sin(120.0/180.0*PI) * r;
    let bx = cos(-120.0/180.0*PI) * r;
    let by = sin(-120.0/180.0*PI) * r;
    vg.begin_path();
    vg.move_to(r,0.0);
    vg.line_to(ax,ay);
    vg.line_to(bx,by);
    vg.close_path();
    paint = vg.linear_gradient(r,0.0, ax,ay, hsla(hue,1.0,0.5, 255), rgba(255,255,255,255));
    vg.fill_paint(paint);
    vg.fill();
    paint = vg.linear_gradient((r+ax)*0.5,(0.0+ay)*0.5, bx,by, rgba(0,0,0,0), rgba(0,0,0,255));
    vg.fill_paint(paint);
    vg.fill();
    vg.stroke_color(rgba(0,0,0,64));
    vg.stroke();

    // Select circle on triangle
    ax = cos(120.0/180.0*PI) * r*0.3;
    ay = sin(120.0/180.0*PI) * r*0.4;
    vg.stroke_width(2.0);
    vg.begin_path();
    vg.circle(ax,ay,5.0);
    vg.stroke_color(rgba(255,255,255,192));
    vg.stroke();

    paint = vg.radial_gradient(ax,ay, 7.0,9.0, rgba(0,0,0,64), rgba(0,0,0,0));
    vg.begin_path();
    vg.rect(ax-20.0,ay-20.0,40.0,40.0);
    vg.circle(ax,ay,7.0);
    vg.path_winding(Solidity::HOLE);
    vg.fill_paint(paint);
    vg.fill();

    vg.restore();

    vg.restore();
}

fn draw_lines(vg: &Context, x: f32, y: f32, w: f32, h: f32, t: f32)
{
    let pad = 5.0;
    let s = w/9.0 - pad*2.0;
    let mut pts: [f32; 4*2] = [0.0; 4*2];
    let joins: [LineCap; 3] = [LineCap::MITER, LineCap::ROUND, LineCap::BEVEL];
    let caps: [LineCap; 3] = [LineCap::BUTT, LineCap::ROUND, LineCap::SQUARE];

    vg.save();
    pts[0] = -s*0.25 + cos(t*0.3) * s*0.5;
    pts[1] = sin(t*0.3) * s*0.5;
    pts[2] = -s*0.25;
    pts[3] = 0.0;
    pts[4] = s*0.25;
    pts[5] = 0.0;
    pts[6] = s*0.25 + cos(-t*0.3) * s*0.5;
    pts[7] = sin(-t*0.3) * s*0.5;

    for i in 0..3 {
        for j in 0..3 {
            let fx = x + s*0.5 + ((i as f32)*3.0+(j as f32))/9.0*w + pad;
            let fy = y - s*0.5 + pad;

            vg.line_cap(caps[i]);
            vg.line_join(joins[j]);

            vg.stroke_width(s*0.3);
            vg.stroke_color(rgba(0,0,0,160));
            vg.begin_path();
            vg.move_to(fx+pts[0], fy+pts[1]);
            vg.line_to(fx+pts[2], fy+pts[3]);
            vg.line_to(fx+pts[4], fy+pts[5]);
            vg.line_to(fx+pts[6], fy+pts[7]);
            vg.stroke();

            vg.line_cap(LineCap::BUTT);
            vg.line_join(LineCap::BEVEL);

            vg.stroke_width(1.0);
            vg.stroke_color(rgba(0,192,255,255));
            vg.begin_path();
            vg.move_to(fx+pts[0], fy+pts[1]);
            vg.line_to(fx+pts[2], fy+pts[3]);
            vg.line_to(fx+pts[4], fy+pts[5]);
            vg.line_to(fx+pts[6], fy+pts[7]);
            vg.stroke();
        }
    }


    vg.restore();
}

fn draw_paragraph(vg: &Context, x: f32, y: f32, width: f32, height: f32, mx: f32, my: f32)
{
    let mut y:f32 = y;
    let text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.";
    let mut asc: f32 = 0.0;
    let mut desc: f32 = 0.0;
    let mut lineh: f32 = 0.0;
    let mut gx: f32 = 0.0;
    let mut gy: f32 = 0.0;
    let mut gutter:i32 = 0;
    let mut bounds: [f32; 4] = [0.0; 4];

    vg.save();

    vg.font_size(18.0);
    vg.font_face("sans");
    vg.text_align(LEFT|TOP);
    vg.text_metrics(&mut asc, &mut desc, &mut lineh);

    // The text break API can be used to fill a large buffer of rows,
    // or to iterate over the text just few lines (or just one) at a time.
    // The "next" variable of the last returned item tells where to continue.
    let mut start: usize = 0;    // byte pos in utf8 'text' str
    let end: usize = text.len(); // exclusive
    let mut lnum = 0;
    'chunks: loop {
        let text = &text[start..end];
        let rows = vg.text_break_lines(text, width, 3);
        let nrows = rows.len();
        if nrows == 0 { break 'chunks; }

        for i in 0..nrows {
            let row = &rows[i];
            let hit: bool = mx > x && mx < (x+width) && my >= y && my < (y+lineh);

            vg.begin_path();
            vg.fill_color(rgba(255,255,255, if hit {64} else {16}));
            vg.rect(x, y, row.width(), lineh);
            vg.fill();

            vg.fill_color(rgba(255,255,255,255));
            let line = &text[row.start_index()..row.end_index()];
            vg.text(x, y, line);

            if hit { // test for mouse-hit and display cursor
                let mut caretx = if mx < x+row.width()/2.0 { x } else { x+row.width() };
                let mut px = x;
                let glyphs = vg.text_glyph_positions(x, y, line);
                let nglyphs = glyphs.len();
                for j in 0..nglyphs {
                    let x0 = glyphs[j].x();
                    let x1 = if j+1 < nglyphs { glyphs[j+1].x() } else { x+row.width() };
                    let gx = x0 * 0.3 + x1 * 0.7;
                    if mx >= px && mx < gx {
                        caretx = glyphs[j].x();
                    }
                    px = gx;
                }
                vg.begin_path();
                vg.fill_color(rgba(255,192,0,255));
                vg.rect(caretx, y, 1.0, lineh);
                vg.fill();

                gutter = lnum+1;
                gx = x - 10.0;
                gy = y + lineh/2.0;
            }
            lnum += 1;
            y += lineh;
        }
        // Keep going...
        start += rows[nrows-1].next_index();
    }

    if gutter > 0 {
        //char txt[16]; snprintf(txt, sizeof(txt), "%d", gutter);
        let txt = format!{"{}", gutter};
        vg.font_size(13.0);
        vg.text_align(RIGHT|MIDDLE);

        vg.text_bounds(gx,gy, &txt, &mut bounds);

        vg.begin_path();
        vg.fill_color(rgba(255,192,0,255));
        vg.rounded_rect(
            floor(bounds[0]) - 4.0,
            floor(bounds[1]) - 2.0,
            floor(bounds[2]-bounds[0]) + 8.0,
            floor(bounds[3]-bounds[1]) + 4.0,
           (floor(bounds[3]-bounds[1]) + 4.0) / 2.0 - 1.0);
        vg.fill();

        vg.fill_color(rgba(32,32,32,255));
        vg.text(gx,gy, &txt);
    }

    y += 20.0;

    vg.font_size(13.0);
    vg.text_align(LEFT|TOP);
    vg.text_line_height(1.2);

    vg.text_box_bounds(x,y,
        150.0, "Hover your mouse over the text to see calculated caret position.",
        &mut bounds);

    // Fade the tooltip out when close to it.
    gx = ((mx - (bounds[0]+bounds[2])*0.5) / (bounds[0] - bounds[2])).abs();
    gy = ((my - (bounds[1]+bounds[3])*0.5) / (bounds[1] - bounds[3])).abs();
    let a = clamp( max(gx, gy) - 0.5,  0.0, 1.0);
    vg.global_alpha(a);

    vg.begin_path();
    vg.fill_color(rgba(220,220,220,255));
    vg.rounded_rect(
        bounds[0]-2.0,
        bounds[1]-2.0,
        floor(bounds[2]-bounds[0])+4.0,
        floor(bounds[3]-bounds[1])+4.0,
        3.0);
    let px = floor((bounds[2]+bounds[0])/2.0);
    vg.move_to(px,bounds[1] - 10.0);
    vg.line_to(px+7.0,bounds[1]+1.0);
    vg.line_to(px-7.0,bounds[1]+1.0);
    vg.fill();

    vg.fill_color(rgba(0,0,0,220));
    vg.text_box(x,y, 150.0, "Hover your mouse over the text to see calculated caret position.");

    vg.restore();
}

fn draw_widths(vg: &Context, x: f32,
              y: f32, width: f32)
{
    vg.save();
    let mut y = y;

    vg.stroke_color(rgba(0,0,0,255));

    for i in 0..20 {
        let w = ((i as f32)+0.5)*0.1;
        vg.stroke_width(w);
        vg.begin_path();
        vg.move_to(x,y);
        vg.line_to(x+width,y+width*0.3);
        vg.stroke();
        y += 10.0;
    }

    vg.restore();
}

fn draw_caps(vg: &Context, x: f32,
            y: f32, width: f32)
{
    let caps: [LineCap; 3] = [LineCap::BUTT, LineCap::ROUND, LineCap::SQUARE];
    let lineWidth = 8.0;

    vg.save();

    vg.begin_path();
    vg.rect(x-lineWidth/2.0, y, width+lineWidth, 40.0);
    vg.fill_color(rgba(255,255,255,32));
    vg.fill();

    vg.begin_path();
    vg.rect(x, y, width, 40.0);
    vg.fill_color(rgba(255,255,255,32));
    vg.fill();

    vg.stroke_width(lineWidth);
    for i in 0..3 {
        vg.line_cap(caps[i]);
        vg.stroke_color(rgba(0,0,0,255));
        vg.begin_path();
        vg.move_to(x, y + (i as f32)*10.0 + 5.0);
        vg.line_to(x+width, y + (i as f32)*10.0 + 5.0);
        vg.stroke();
    }

    vg.restore();
}


fn unpremultiply_alpha(image: &mut Vec<u8>, w: u32, h: u32, stride: u32)
{
    let w: usize = w as usize; let h: usize = h as usize; let stride: usize = stride as usize;

    // Unpremultiply
    for y in 0..h {
        //unsigned char *row = &image[y*stride];
        let row = &mut image[y*stride..y*stride + w*4];
        for x in 0..w {
            let pix = &mut row[x*4..x*4 + 4];
            let r = pix[0] as f32;
            let g = pix[1] as f32;
            let b = pix[2] as f32;
            let a = pix[3] as f32;
            if a != 0.0 {
                pix[0] = min(r*255.0/a, 255.0) as u8;
                pix[1] = min(g*255.0/a, 255.0) as u8;
                pix[2] = min(b*255.0/a, 255.0) as u8;
            }
        }
    }

    // Defringe
    for y in 0..h {
        for x in 0..w {
            let ix = y*stride + x*4;
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let a = image[ix+3];
            let mut n = 0;
            if a == 0 {
                if x-1 > 0 && image[ix-1] != 0 {
                    r += image[ix-4];
                    g += image[ix-3];
                    b += image[ix-2];
                    n += 1;
                }
                if x+1 < w && image[ix+7] != 0 {
                    r += image[ix+4];
                    g += image[ix+5];
                    b += image[ix+6];
                    n += 1;
                }
                if y-1 > 0 && image[ix-stride+3] != 0 {
                    r += image[ix-stride];
                    g += image[ix-stride+1];
                    b += image[ix-stride+2];
                    n += 1;
                }
                if y+1 < h && image[ix+stride+3] != 0 {
                    r += image[ix+stride];
                    g += image[ix+stride+1];
                    b += image[ix+stride+2];
                    n += 1;
                }
                if n > 0 {
                    image[ix+0] = r/n;
                    image[ix+1] = g/n;
                    image[ix+2] = b/n;
                }
            }
        }
    }
}

fn set_alpha(image: &mut Vec<u8>, w: u32, h: u32, stride: u32, a: u8)
{
    let w: usize = w as usize; let h: usize = h as usize; let stride: usize = stride as usize;
    for y in 0..h {
        let row = &mut image[y*stride..y*stride + w*4]; //&image[y*stride];
        for x in 0..w {
            row[x*4+3] = a;
        }
    }
}

fn flip_image(image: &mut Vec<u8>, w: u32, h: u32, stride: u32)
{
    let w: usize = w as usize; let h: usize = h as usize; let stride: usize = stride as usize;
    let mut i: usize = 0;
    let mut j: usize = h-1;
    while i < j {
        //let row_i = image.slice_mut(i*stride, i*stride + w*4); //&image[i * stride]; //unsigned char*
        //let row_j = image.slice_mut(j*stride, j*stride + w*4); //&image[j * stride]; //unsigned char*
        // error; can't borrow twice from the same source
        let ix: usize = i*stride;
        let jx: usize = j*stride;
        for k in 0..w*4 {
            let t       = image[ix+k];  // let t = row_i[k];
            image[ix+k] = image[jx+k];  // row_i[k] = row_j[k];
            image[jx+k] = t;            // row_j[k] = t;
        }
        i += 1;
        j -= 1;
    }
}

pub fn save_screenshot(w: u32, h: u32, premult: bool, name: &str)
{
    let sz: usize = (w*h*4) as usize;
    //let mut image: [u8; sz] = [0; sz];
    let mut image: Vec<u8> = Vec::with_capacity(sz);
    unsafe {image.set_len(sz);}
    assert!(image.len() == sz);

    let addr: *mut u8 = image.as_mut_ptr();
    unsafe {gl::ReadPixels(0, 0, w as i32, h as i32, gl::RGBA, gl::UNSIGNED_BYTE, addr as *mut c_void)};
    if premult {
        unpremultiply_alpha(&mut image, w, h, w*4);
    }
    else {
        set_alpha(&mut image, w, h, w*4, 255);
    }
    flip_image(&mut image, w, h, w*4);

    let mut image = png::Image {
        width: w,
        height: h,
        pixels: png::PixelsByColorType::RGBA8(image)
    };
    png::store_png(&mut image, name).unwrap()
}

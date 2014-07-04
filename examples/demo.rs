
use std::fmt;
use std::ptr;
use std::str;
use std::bitflags;
use libc::{c_double, c_float, c_int, c_char, c_uint, c_ushort, c_uchar, c_void};

use std::num::*;
use std::num::Float;
use nanovg;
use nanovg::*;

//#include "stb_image_write.h"

static ICON_SEARCH :       u32 = 0x1F50D;
static ICON_CIRCLED_CROSS: u32 =  0x2716;
static ICON_CHEVRON_RIGHT: u32 =  0xE75E;
static ICON_CHECK:         u32 =  0x2713;
static ICON_LOGIN:         u32 =  0xE740;
static ICON_TRASH:         u32 =  0xE729;

static PI: f32 = 3.1415926535;

fn min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }
fn max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
fn abs(a: f32) -> f32 { if a >= 0.0 { a } else { -a } }
fn clamp(a: f32, mn: f32, mx: f32) -> f32 { if a < mn { mn } else { if a > mx { mx } else { a } } }
fn floor(x: f32) -> f32 { x.floor() }
fn sqrt(x: f32) -> f32 { x.sqrt() }
//fn pow(x: f32, e: uint) -> f32 { x.pow(e) }
fn cos(x: f32) -> f32 { x.cos() }
fn sin(x: f32) -> f32 { x.sin() }

//static char* cpToUTF8(int cp, char* utf)
fn cpToUTF8(cp:char) -> String
{
	format!("{}", cp)
//	int n = 0;
//	if (cp < 0x80) n = 1;
//	else if (cp < 0x800) n = 2;
//	else if (cp < 0x10000) n = 3;
//	else if (cp < 0x200000) n = 4;
//	else if (cp < 0x4000000) n = 5;
//	else if (cp <= 0x7fffffff) n = 6;
//	utf[n] = '\0';
//	switch (n) {
//	case 6: utf[5] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x4000000;
//	case 5: utf[4] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x200000;
//	case 4: utf[3] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x10000;
//	case 3: utf[2] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x800;
//	case 2: utf[1] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0xc0;
//	case 1: utf[0] = cp;
//	}
//	return utf;
}

pub struct DemoData {
	fontNormal: i32,
	fontBold: i32,
	fontIcons: i32,
	images: [i32, ..12],
}

impl DemoData
{
	//int loadDemoData(struct NVGcontext* vg, struct DemoData* data)
	pub fn load(vg: &Ctx) -> DemoData
	{
		let mut data = DemoData {
			fontNormal: -1,
			fontBold:   -1,
			fontIcons:  -1,
			images: [-1, ..12]
		};

		for i in range(1, 12u) {
			let filename = format!("../example/images/image{}.jpg", i);
			data.images[i] = vg.create_image(filename.as_slice());
			if (data.images[i] == 0) {
				println!("Could not load {}.", filename);
				//return -1;
			}
		}

		data.fontIcons = vg.create_font("icons", "../example/entypo.ttf");
		if (data.fontIcons == -1) {
			println!("Could not add font icons.");
			//return -1;
		}
		data.fontNormal = vg.create_font("sans", "../example/Roboto-Regular.ttf");
		if (data.fontNormal == -1) {
			println!("Could not add font italic.");
			//return -1;
		}
		data.fontBold = vg.create_font("sans-bold", "../example/Roboto-Bold.ttf");
		if (data.fontBold == -1) {
			println!("Could not add font bold.");
			//return -1;
		}

		return data;
	}
}



//void freeDemoData(struct NVGcontext* vg, struct DemoData* data)
pub fn free_demo_data(vg: &Ctx,
                    data: *mut DemoData)
{
//	for i in range(0, 12) {
//		vg.delete_image(data.images[i]);
//	}
}


//void renderDemo(struct NVGcontext* vg, f32 mx, f32 my, f32 width, f32 height, f32 t, int blowup, struct DemoData* data)
pub fn render_demo(vg: &Ctx, mx: f32,
                  my: f32, width: f32,
                  height: f32, t: f32,
                  blowup: bool, data: &DemoData)
{
	draw_eyes(vg, width - 150.0, 50.0, 150.0, 100.0, mx, my, t);
//	draw_paragraph(vg, width - 450, 50, 150, 100, mx, my);
//	draw_graph(vg, 0, height/2, width, height/2, t);
//	draw_colorwheel(vg, width - 300, height - 300, 250.0, 250.0, t);
//
//	// Line joints
//	draw_lines(vg, 50, height-50, 600, 50, t);
//
//	// Line caps
//	draw_widths(vg, 10, 50, 30);
//
//	// Line caps
//	draw_caps(vg, 10, 300, 30);
//
//	vg.save();
//	if (blowup) {
//		vg.rotate(sin(t*0.3)*5.0/180.0*PI);
//		vg.scale(2.0, 2.0);
//	}
//
//	// Widgets
//	draw_window(vg, "Widgets `n Stuff", 50, 50, 300, 400);
//	let mut x = 60; let mut y = 95;
//	draw_searchbox(vg, "Search", x,y,280,25);
//	y += 40;
//	draw_dropdown(vg, "Effects", x,y,280,28);
//	let mut popy = y + 14;
//	y += 45;
//
//	// Form
//	draw_label(vg, "Login", x,y, 280,20);
//	y += 25;
//	draw_editbox(vg, "Email",  x,y, 280,28);
//	y += 35;
//	draw_editbox(vg, "Password", x,y, 280,28);
//	y += 38;
//	draw_checkbox(vg, "Remember me", x,y, 140,28);
//	draw_button(vg, ICON_LOGIN, "Sign in", x+138, y, 140, 28, rgba(0,96,128,255));
//	y += 45;
//
//	// Slider
//	draw_label(vg, "Diameter", x,y, 280,20);
//	y += 25;
//	draw_editbox_num(vg, "123.00", "px", x+180,y, 100,28);
//	draw_slider(vg, 0.4, x,y, 170,28);
//	y += 55;
//
//	draw_button(vg, ICON_TRASH, "Delete", x, y, 160, 28, rgba(128,16,8,255));
//	draw_button(vg, 0, "Cancel", x+170, y, 110, 28, rgba(0,0,0,0));
//
//	// Thumbnails box
//	draw_thumbnails(vg, 365, popy-30, 160, 300, data.images, 12, t);

	vg.restore();
}


//void saveScreenShot(int w, int h, int premult, const char* name)
pub fn save_screenshot(w: c_int, h: c_int,
                      premult: c_int, name: *c_char)
{
//	let image: [u8, ..w*h*4];
//	glReadPixels(0, 0, w, h, GL_RGBA, GL_UNSIGNED_BYTE, &image);
//	if (premult) {
//		unpremultiplyAlpha(image, w, h, w*4);
//	}
//	else {
//		setAlpha(image, w, h, w*4, 255);
//	}
//	flipHorizontal(image, w, h, w*4);
// 	stbi_write_png(name, w, h, 4, image, w*4);
}



fn is_black(col: NVGcolor) -> bool {
	col.r == 0.0 && col.g == 0.0 && col.b == 0.0 && col.a == 0.0
}

fn draw_window(vg: &Ctx, title: &str,
              x: c_float, y: c_float,
              w: c_float, h: c_float)
{
	let cornerRadius = 3.0;

	vg.save();
//	vg.clear_state();

	// Window
	vg.begin_path();
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.fill_color(rgba(28,30,34,192));
//	vg.fill_color(rgba(0,0,0,128));
	vg.fill();

	// Drop shadow
	let shadowPaint = vg.box_gradient(x,y+2.0, w,h, cornerRadius*2.0, 10.0, rgba(0,0,0,128), rgba(0,0,0,0));
	vg.begin_path();
	vg.rect(x-10.0,y-10.0, w+20.0,h+30.0);
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.path_winding(HOLE);
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
////
//
//fn draw_searchbox(vg: &Ctx, text: *c_char,
//                 x: c_float, y: c_float,
//                 w: c_float, h: c_float)
//{
//	let mut icon: [u8, ..8]; //char icon[8];
//	let cornerRadius = h/2.0 - 1.0;
//
//	// Edit
//	let bg = vg.box_gradient(x,y+1.5, w,h, h/2,5, rgba(0,0,0,16), rgba(0,0,0,92));
//	vg.begin_path();
//	vg.rounded_rect(x,y, w,h, cornerRadius);
//	vg.fill_paint(bg);
//	vg.fill();
//
//  /*	vg.begin_path();
//	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
//	vg.stroke_color(rgba(0,0,0,48));
//	vg.stroke();*/
//
//	vg.font_size(h*1.3);
//	vg.font_face("icons");
//	vg.fill_color(rgba(255,255,255,64));
//	vg.text_align(CENTER|MIDDLE);
//	vg.text(x+h*0.55, y+h*0.55, cpToUTF8(ICON_SEARCH,icon));
//
//	vg.font_size(20.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,32));
//
//	vg.text_align(LEFT|MIDDLE);
//	vg.text(x+h*1.05,y+h*0.5,text);
//
//	vg.font_size(h*1.3);
//	vg.font_face("icons");
//	vg.fill_color(rgba(255,255,255,32));
//	vg.text_align(CENTER|MIDDLE);
//	vg.text(x+w-h*0.55, y+h*0.55, cpToUTF8(ICON_CIRCLED_CROSS,icon));
//}
//
//fn draw_dropdown(vg: &Ctx, text: *c_char,
//                x: c_float, y: c_float,
//                w: c_float, h: c_float)
//{
//	let mut icon: [u8, ..8]; //char icon[8];
//	let cornerRadius = 4.0;
//
//	let bg = vg.linear_gradient(x,y,x,y+h, rgba(255,255,255,16), rgba(0,0,0,16));
//	vg.begin_path();
//	vg.rounded_rect(x+1,y+1, w-2,h-2, cornerRadius-1);
//	vg.fill_paint(bg);
//	vg.fill();
//
//	vg.begin_path();
//	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
//	vg.stroke_color(rgba(0,0,0,48));
//	vg.stroke();
//
//	vg.font_size(20.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,160));
//	vg.text_align(LEFT|MIDDLE);
//	vg.text(x+h*0.3,y+h*0.5,text);
//
//	vg.font_size(h*1.3);
//	vg.font_face("icons");
//	vg.fill_color(rgba(255,255,255,64));
//	vg.text_align(CENTER|MIDDLE);
//	vg.text(x+w-h*0.5, y+h*0.5, cpToUTF8(ICON_CHEVRON_RIGHT,icon));
//}
//
//fn draw_label(vg: &Ctx, text: *c_char,
//             x: c_float, y: c_float,
//             w: c_float, h: c_float)
//{
//	vg.font_size(18.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,128));
//
//	vg.text_align(LEFT|MIDDLE);
//	vg.text(x,y+h*0.5,text);
//}
//
//fn draw_editbox_base(vg: &Ctx, x: c_float,
//                   y: c_float, w: c_float,
//                   h: c_float)
//{
//	// Edit
//	let bg = vg.box_gradient(x+1,y+1+1.5, w-2,h-2, 3,4, rgba(255,255,255,32), rgba(32,32,32,32));
//	vg.begin_path();
//	vg.rounded_rect(x+1,y+1, w-2,h-2, 4-1);
//	vg.fill_paint(bg);
//	vg.fill();
//
//	vg.begin_path();
//	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, 4-0.5);
//	vg.stroke_color(rgba(0,0,0,48));
//	vg.stroke();
//}
//
//fn draw_editbox(vg: &Ctx, text: *c_char,
//               x: c_float, y: c_float,
//               w: c_float, h: c_float)
//{
//	draw_editbox_base(vg, x,y, w,h);
//
//	vg.font_size(20.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,64));
//	vg.text_align(LEFT|MIDDLE);
//	vg.text(x+h*0.3,y+h*0.5,text);
//}
//
//fn draw_editbox_num(vg: &Ctx, text: *c_char,
//                  units: *c_char, x: c_float,
//                  y: c_float, w: c_float,
//				  h: c_float)
//{
//	draw_editbox_base(vg, x,y, w,h);
//
//	let mut bounds: f32 = 0.0;
//	let uw = vg.text_bounds(0,0, units, &bounds);
//
//	vg.font_size(18.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,64));
//	vg.text_align(RIGHT|MIDDLE);
//	vg.text(x+w-h*0.3,y+h*0.5,units);
//
//	vg.font_size(20.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,128));
//	vg.text_align(RIGHT|MIDDLE);
//	vg.text(x+w-uw-h*0.5,y+h*0.5,text);
//}
//
//fn draw_checkbox(vg: &Ctx, text: *c_char,
//                x: c_float, y: c_float,
//                w: c_float, h: c_float)
//{
////	char icon[8];
////	NOTUSED(w);
//
//	vg.font_size(18.0);
//	vg.font_face("sans");
//	vg.fill_color(rgba(255,255,255,160));
//
//	vg.text_align(LEFT|MIDDLE);
////	vg.text(x+28,y+h*0.5,text, NULL);
//
//	let bg = vg.box_gradient(x+1,y+floor(h*0.5)-9+1, 18,18, 3,3, rgba(0,0,0,32), rgba(0,0,0,92));
//	vg.begin_path();
//	vg.rounded_rect(x+1,y+floor(h*0.5)-9, 18,18, 3);
//	vg.fill_paint(bg);
//	vg.fill();
//
//	vg.font_size(40);
//	vg.font_face("icons");
//	vg.fill_color(rgba(255,255,255,128));
//	vg.text_align(CENTER|MIDDLE);
////	vg.text(x+9+2, y+h*0.5, cpToUTF8(ICON_CHECK,icon), NULL);
//}
//
//fn draw_button(vg: &Ctx, preicon: c_int,
//              text: *c_char, x: c_float,
//              y: c_float, w: c_float,
//			  h: c_float, col: NVGcolor)
//{
//	let mut icon: [u8, ..8];
//	let cornerRadius = 4.0;
//	let tw = 0.0;
//	let iw = 0.0;
//
//	let bg = vg.linear_gradient(x,y,x,y+h, rgba(255,255,255,if is_black(col){16}else{32}), rgba(0,0,0,if is_black(col){16}else{32}));
//	vg.begin_path();
//	vg.rounded_rect(x+1,y+1, w-2,h-2, cornerRadius-1);
//	if (!is_black(col)) {
//		vg.fill_color(col);
//		vg.fill();
//	}
//	vg.fill_paint(bg);
//	vg.fill();
//
//	vg.begin_path();
//	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
//	vg.stroke_color(rgba(0,0,0,48));
//	vg.stroke();
//
//	vg.font_size(20.0);
//	vg.font_face("sans-bold");
//	let mut bounds: f32 = 0.0;
//	tw = vg.text_bounds(0,0, text, &bounds);
//	if (preicon != 0) {
//		vg.font_size(h*1.3);
//		vg.font_face("icons");
//		iw = vg.text_bounds(0,0, cpToUTF8(preicon,icon), &bounds);
//		iw += h*0.15;
//	}
//
//	if (preicon != 0) {
//		vg.font_size(h*1.3);
//		vg.font_face("icons");
//		vg.fill_color(rgba(255,255,255,96));
//		vg.text_align(LEFT|MIDDLE);
//		vg.text(x+w*0.5-tw*0.5-iw*0.75, y+h*0.5, cpToUTF8(preicon,icon));
//	}
//
//	vg.font_size(20.0);
//	vg.font_face("sans-bold");
//	vg.text_align(LEFT|MIDDLE);
//	vg.fill_color(rgba(0,0,0,160));
//	vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5-1,text);
//	vg.fill_color(rgba(255,255,255,160));
//	vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5,text);
//}
//
//fn draw_slider(vg: &Ctx, pos: c_float,
//              x: c_float, y: c_float,
//              w: c_float, h: c_float)
//{
//	let cy: f32 = y+floor(h*0.5);
//	let kr: f32 = floor(h*0.25);
//
//	vg.save();
////	vg.clear_state();
//
//	// Slot
//	let bg = vg.box_gradient(x,cy-2+1, w,4, 2,2, rgba(0,0,0,32), rgba(0,0,0,128));
//	vg.begin_path();
//	vg.rounded_rect(x,cy-2, w,4, 2);
//	vg.fill_paint(bg);
//	vg.fill();
//
//	// Knob Shadow
//	let shadow = vg.radial_gradient(x+floor(pos*w),cy+1, kr-3,kr+3, rgba(0,0,0,64), rgba(0,0,0,0));
//	vg.begin_path();
//	vg.rect(x+floor(pos*w)-kr-5,cy-kr-5,kr*2+5+5,kr*2+5+5+3);
//	vg.circle(x+floor(pos*w),cy, kr);
//	vg.path_winding(HOLE);
//	vg.fill_paint(shadow);
//	vg.fill();
//
//	// Knob
//	let knob = vg.linear_gradient(x,cy-kr,x,cy+kr, rgba(255,255,255,16), rgba(0,0,0,16));
//	vg.begin_path();
//	vg.circle(x+floor(pos*w),cy, kr-1);
//	vg.fill_color(rgba(40,43,48,255));
//	vg.fill();
//	vg.fill_paint(knob);
//	vg.fill();
//
//	vg.begin_path();
//	vg.circle(x+floor(pos*w),cy, kr-0.5);
//	vg.stroke_color(rgba(0,0,0,92));
//	vg.stroke();
//
//	vg.restore();
//}
//
fn draw_eyes(vg: &Ctx, x: c_float,
            y: c_float, w: c_float,
            h: c_float, mx: c_float,
            my: c_float, t: c_float)
{
	let ex = w *0.23;
	let ey = h * 0.5;
	let lx = x + ex;
	let ly = y + ey;
	let rx = x + w - ex;
	let ry = y + ey;
	let br = min(ex, ey) * 0.5;
	let blink: f32 = 1.0 - pow(sin(t*0.5),200)*0.8;

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
	if (d > 1.0) {
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
	if (d > 1.0) {
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

//fn draw_graph(vg: &Ctx, x: c_float,
//             y: c_float, w: c_float,
//             h: c_float, t: c_float)
//{
////	f32 samples[6];
////	f32 sx[6], sy[6];
////	f32 dx = w/5.0;
////	int i;
////
////	samples[0] = (1+sin(t*1.2345+cos(t*0.33457)*0.44))*0.5;
////	samples[1] = (1+sin(t*0.68363+cos(t*1.3)*1.55))*0.5;
////	samples[2] = (1+sin(t*1.1642+cos(t*0.33457)*1.24))*0.5;
////	samples[3] = (1+sin(t*0.56345+cos(t*1.63)*0.14))*0.5;
////	samples[4] = (1+sin(t*1.6245+cos(t*0.254)*0.3))*0.5;
////	samples[5] = (1+sin(t*0.345+cos(t*0.03)*0.6))*0.5;
////
////	for (i = 0; i < 6; i++) {
////		sx[i] = x+i*dx;
////		sy[i] = y+h*samples[i]*0.8;
////	}
////
////	// Graph background
////	let bg = vg.linear_gradient(x,y,x,y+h, rgba(0,160,192,0), rgba(0,160,192,64));
////	vg.begin_path();
////	vg.move_to(sx[0], sy[0]);
////	for (i = 1; i < 6; i++)
////		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
////	vg.line_to(x+w, y+h);
////	vg.line_to(x, y+h);
////	vg.fill_paint(bg);
////	vg.fill();
////
////	// Graph line
////	vg.begin_path();
////	vg.move_to(sx[0], sy[0]+2);
////	for (i = 1; i < 6; i++)
////		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1]+2, sx[i]-dx*0.5,sy[i]+2, sx[i],sy[i]+2);
////	vg.stroke_color(rgba(0,0,0,32));
////	vg.stroke_width(3.0);
////	vg.stroke();
////
////	vg.begin_path();
////	vg.move_to(sx[0], sy[0]);
////	for (i = 1; i < 6; i++)
////		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
////	vg.stroke_color(rgba(0,160,192,255));
////	vg.stroke_width(3.0);
////	vg.stroke();
////
////	// Graph sample pos
////	for (i = 0; i < 6; i++) {
////		bg = vg.radial_gradient(sx[i],sy[i]+2, 3.0,8.0, rgba(0,0,0,32), rgba(0,0,0,0));
////		vg.begin_path();
////		vg.rect(sx[i]-10, sy[i]-10+2, 20,20);
////		vg.fill_paint(bg);
////		vg.fill();
////	}
////
////	vg.begin_path();
////	for (i = 0; i < 6; i++)
////		vg.circle(sx[i], sy[i], 4.0);
////	vg.fill_color(rgba(0,160,192,255));
////	vg.fill();
////	vg.begin_path();
////	for (i = 0; i < 6; i++)
////		vg.circle(sx[i], sy[i], 2.0);
////	vg.fill_color(rgba(220,220,220,255));
////	vg.fill();
////
////	vg.stroke_width(1.0);
//}
//
//fn draw_spinner(vg: &Ctx, cx: c_float,
//               cy: c_float, r: c_float,
//               t: c_float)
//{
//	let a0 = 0.0 + t*6;
//	let a1 = PI + t*6;
//	let r0 = r;
//	let r1 = r * 0.75;
//
//	vg.save();
//
//	vg.begin_path();
//	vg.arc(cx,cy, r0, a0, a1, CW);
//	vg.arc(cx,cy, r1, a1, a0, CCW);
//	vg.close_path();
//	let ax = cx + cos(a0) * (r0+r1)*0.5;
//	let ay = cy + sin(a0) * (r0+r1)*0.5;
//	let bx = cx + cos(a1) * (r0+r1)*0.5;
//	let by = cy + sin(a1) * (r0+r1)*0.5;
//	let paint = vg.linear_gradient(ax,ay, bx,by, rgba(0,0,0,0), rgba(0,0,0,128));
//	vg.fill_paint(paint);
//	vg.fill();
//
//	vg.restore();
//}
//
//fn draw_thumbnails(vg: &Ctx, x: c_float,
//                  y: c_float, w: c_float,
//                  h: c_float, images: *c_int,
//                  nimages: c_int, t: c_float)
//{
////	let cornerRadius = 3.0;
////
////	f32 ix,iy,iw,ih;
////	f32 thumb = 60.0;
////	f32 arry = 30.5;
////	int imgw, imgh;
////	f32 stackh = (nimages/2) * (thumb+10) + 10;
////	int i;
////	f32 u = (1+cos(t*0.5))*0.5;
////	f32 u2 = (1-cos(t*0.2))*0.5;
////	f32 scrollh, dv;
////
////	vg.save();
//////	vg.clear_state();
////
////	// Drop shadow
////	let shadowPaint = vg.box_gradient(x,y+4, w,h, cornerRadius*2, 20, rgba(0,0,0,128), rgba(0,0,0,0));
////	vg.begin_path();
////	vg.rect(x-10,y-10, w+20,h+30);
////	vg.rounded_rect(x,y, w,h, cornerRadius);
////	vg.path_winding(HOLE);
////	vg.fill_paint(shadowPaint);
////	vg.fill();
////
////	// Window
////	vg.begin_path();
////	vg.rounded_rect(x,y, w,h, cornerRadius);
////	vg.move_to(x-10,y+arry);
////	vg.line_to(x+1,y+arry-11);
////	vg.line_to(x+1,y+arry+11);
////	vg.fill_color(rgba(200,200,200,255));
////	vg.fill();
////
////	vg.save();
////	vg.scissor(x,y,w,h);
////	vg.translate(0, -(stackh - h)*u);
////
////	dv = 1.0 / (f32)(nimages-1);
////
////	for (i = 0; i < nimages; i++) {
////		f32 tx, ty, v, a;
////		tx = x+10;
////		ty = y+10;
////		tx += (i%2) * (thumb+10);
////		ty += (i/2) * (thumb+10);
////		vg.image_size(images[i], &imgw, &imgh);
////		if (imgw < imgh) {
////			iw = thumb;
////			ih = iw * (f32)imgh/(f32)imgw;
////			ix = 0;
////			iy = -(ih-thumb)*0.5;
////		} else {
////			ih = thumb;
////			iw = ih * (f32)imgw/(f32)imgh;
////			ix = -(iw-thumb)*0.5;
////			iy = 0;
////		}
////
////		v = i * dv;
////		a = clampf((u2-v) / dv, 0, 1);
////
////		if (a < 1.0)
////			drawSpinner(vg, tx+thumb/2,ty+thumb/2, thumb*0.25, t);
////
////		imgPaint = vg.image_pattern(tx+ix, ty+iy, iw,ih, 0.0/180.0*PI, images[i], NVG_NOREPEAT, a);
////		vg.begin_path();
////		vg.rounded_rect(tx,ty, thumb,thumb, 5);
////		vg.fill_paint(imgPaint);
////		vg.fill();
////
////		shadowPaint = vg.box_gradient(tx-1,ty, thumb+2,thumb+2, 5, 3, rgba(0,0,0,128), rgba(0,0,0,0));
////		vg.begin_path();
////		vg.rect(tx-5,ty-5, thumb+10,thumb+10);
////		vg.rounded_rect(tx,ty, thumb,thumb, 6);
////		vg.path_winding(HOLE);
////		vg.fill_paint(shadowPaint);
////		vg.fill();
////
////		vg.begin_path();
////		vg.rounded_rect(tx+0.5,ty+0.5, thumb-1,thumb-1, 4-0.5);
////		vg.stroke_width(1.0);
////		vg.stroke_color(rgba(255,255,255,192));
////		vg.stroke();
////	}
////	vg.restore();
////
////	// Hide fades
////	let fadePaint = vg.linear_gradient(x,y,x,y+6, rgba(200,200,200,255), rgba(200,200,200,0));
////	vg.begin_path();
////	vg.rect(x+4,y,w-8,6);
////	vg.fill_paint(fadePaint);
////	vg.fill();
////
////	fadePaint = vg.linear_gradient(x,y+h,x,y+h-6, rgba(200,200,200,255), rgba(200,200,200,0));
////	vg.begin_path();
////	vg.rect(x+4,y+h-6,w-8,6);
////	vg.fill_paint(fadePaint);
////	vg.fill();
////
////	// Scroll bar
////	shadowPaint = vg.box_gradient(x+w-12+1,y+4+1, 8,h-8, 3,4, rgba(0,0,0,32), rgba(0,0,0,92));
////	vg.begin_path();
////	vg.rounded_rect(x+w-12,y+4, 8,h-8, 3);
////	vg.fill_paint(shadowPaint);
//////	vg.fill_color(rgba(255,0,0,128));
////	vg.fill();
////
////	scrollh = (h/stackh) * (h-8);
////	shadowPaint = vg.box_gradient(x+w-12-1,y+4+(h-8-scrollh)*u-1, 8,scrollh, 3,4, rgba(220,220,220,255), rgba(128,128,128,255));
////	vg.begin_path();
////	vg.rounded_rect(x+w-12+1,y+4+1 + (h-8-scrollh)*u, 8-2,scrollh-2, 2);
////	vg.fill_paint(shadowPaint);
//////	vg.fill_color(rgba(0,0,0,128));
////	vg.fill();
////
////	vg.restore();
//}
//
//fn draw_colorwheel(vg: &Ctx, x: c_float,
//                  y: c_float, w: c_float,
//                  h: c_float, t: c_float)
//{
//	//f32 r0, r1, ax,ay, bx,by, cx,cy, aeps, r;
//	let hue = sin(t * 0.12);
//
//	vg.save();
//
//  /*	vg.begin_path();
//	vg.rect(x,y,w,h);
//	vg.fill_color(rgba(255,0,0,128));
//	vg.fill();*/
//
//	let cx = x + w*0.5;
//	let cy = y + h*0.5;
//	let r1 = min(w,h) * 0.5 - 5.0;
//	let r0 = r1 - 20.0;
//	let aeps = 0.5 / r1;	// half a pixel arc length in radians (2pi cancels out).
//
//	for i in range(0, 6) {
//		let a0 = i / 6.0 * PI * 2.0 - aeps;
//		let a1 = (i+1.0) / 6.0 * PI * 2.0 + aeps;
//		vg.begin_path();
//		vg.arc(cx,cy, r0, a0, a1, CW);
//		vg.arc(cx,cy, r1, a1, a0, CCW);
//		vg.close_path();
//		let ax = cx + cos(a0) * (r0+r1)*0.5;
//		let ay = cy + sin(a0) * (r0+r1)*0.5;
//		let bx = cx + cos(a1) * (r0+r1)*0.5;
//		let by = cy + sin(a1) * (r0+r1)*0.5;
//		let paint = vg.linear_gradient(ax,ay, bx,by, hsla(a0/(PI*2),1.0,0.55,255), hsla(a1/(PI*2),1.0,0.55,255));
//		vg.fill_paint(paint);
//		vg.fill();
//	}
//
//	vg.begin_path();
//	vg.circle(cx,cy, r0-0.5);
//	vg.circle(cx,cy, r1+0.5);
//	vg.stroke_color(rgba(0,0,0,64));
//	vg.stroke_width(1.0);
//	vg.stroke();
//
//	// Selector
//	vg.save();
//	vg.translate(cx,cy);
//	vg.rotate(hue*PI*2);
//
//	// Marker on
//	vg.stroke_width(2.0);
//	vg.begin_path();
//	vg.rect(r0-1,-3,r1-r0+2,6);
//	vg.stroke_color(rgba(255,255,255,192));
//	vg.stroke();
//
//	let paint = vg.box_gradient(r0-3,-5,r1-r0+6,10, 2,4, rgba(0,0,0,128), rgba(0,0,0,0));
//	vg.begin_path();
//	vg.rect(r0-2-10,-4-10,r1-r0+4+20,8+20);
//	vg.rect(r0-2,-4,r1-r0+4,8);
//	vg.path_winding(HOLE);
//	vg.fill_paint(paint);
//	vg.fill();
//
//	// Center triangle
//	let r = r0 - 6;
//	let ax = cos(120.0/180.0*PI) * r;
//	let ay = sin(120.0/180.0*PI) * r;
//	let bx = cos(-120.0/180.0*PI) * r;
//	let by = sin(-120.0/180.0*PI) * r;
//	vg.begin_path();
//	vg.move_to(r,0);
//	vg.line_to(ax,ay);
//	vg.line_to(bx,by);
//	vg.close_path();
//	let paint = vg.linear_gradient(r,0, ax,ay, hsla(hue,1.0,0.5,255), rgba(255,255,255,255));
//	vg.fill_paint(paint);
//	vg.fill();
//	paint = vg.linear_gradient((r+ax)*0.5,(0+ay)*0.5, bx,by, rgba(0,0,0,0), rgba(0,0,0,255));
//	vg.fill_paint(paint);
//	vg.fill();
//	vg.stroke_color(rgba(0,0,0,64));
//	vg.stroke();
//
//	// Select circle on triangle
//	ax = cos(120.0/180.0*PI) * r*0.3;
//	ay = sin(120.0/180.0*PI) * r*0.4;
//	vg.stroke_width(2.0);
//	vg.begin_path();
//	vg.circle(ax,ay,5);
//	vg.stroke_color(rgba(255,255,255,192));
//	vg.stroke();
//
//	paint = vg.radial_gradient(ax,ay, 7,9, rgba(0,0,0,64), rgba(0,0,0,0));
//	vg.begin_path();
//	vg.rect(ax-20,ay-20,40,40);
//	vg.circle(ax,ay,7);
//	vg.path_winding(HOLE);
//	vg.fill_paint(paint);
//	vg.fill();
//
//	vg.restore();
//
//	vg.restore();
//}
//
//fn draw_lines(vg: &Ctx, x: c_float,
//             y: c_float, w: c_float,
//             h: c_float, t: c_float)
//{
//	let pad = 5.0;
//	let s = w/9.0 - pad*2;
//	let pts: [f32, ..4*2];
//	let joins: [int, ..3] = [MITER, ROUND, BEVEL];
//	let caps: [int, ..3] = [BUTT, ROUND, SQUARE];
//	//NOTUSED(h);
//
//	vg.save();
//	pts[0] = -s*0.25 + cos(t*0.3) * s*0.5;
//	pts[1] = sin(t*0.3) * s*0.5;
//	pts[2] = -s*0.25;
//	pts[3] = 0;
//	pts[4] = s*0.25;
//	pts[5] = 0;
//	pts[6] = s*0.25 + cos(-t*0.3) * s*0.5;
//	pts[7] = sin(-t*0.3) * s*0.5;
//
//	for i in range(0, 3) {
//		for j in range(0, 3) {
//			let fx = x + s*0.5 + (i*3+j)/9.0*w + pad;
//			let fy = y - s*0.5 + pad;
//
//			vg.line_cap(caps[i]);
//			vg.line_join(joins[j]);
//
//			vg.stroke_width(s*0.3);
//			vg.stroke_color(rgba(0,0,0,160));
//			vg.begin_path();
//			vg.move_to(fx+pts[0], fy+pts[1]);
//			vg.line_to(fx+pts[2], fy+pts[3]);
//			vg.line_to(fx+pts[4], fy+pts[5]);
//			vg.line_to(fx+pts[6], fy+pts[7]);
//			vg.stroke();
//
//			vg.line_cap(BUTT);
//			vg.line_join(BEVEL);
//
//			vg.stroke_width(1.0);
//			vg.stroke_color(rgba(0,192,255,255));
//			vg.begin_path();
//			vg.move_to(fx+pts[0], fy+pts[1]);
//			vg.line_to(fx+pts[2], fy+pts[3]);
//			vg.line_to(fx+pts[4], fy+pts[5]);
//			vg.line_to(fx+pts[6], fy+pts[7]);
//			vg.stroke();
//		}
//	}
//
//
//	vg.restore();
//}
//
//fn draw_paragraph(vg: &Ctx, x: c_float,
//                 y: c_float, width: c_float,
//                 height: c_float, mx: c_float,
//                 my: c_float)
//{
////	struct NVGtextRow rows[3];
////	struct NVGglyphPosition glyphs[100];
////	const char* text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.";
////	const char* start;
////	const char* end;
////	int nrows, i, nglyphs, j, lnum = 0;
////	f32 lineh;
////	f32 caretx, px;
////	f32 bounds[4];
////	f32 a;
////	f32 gx,gy;
////	int gutter = 0;
////	NOTUSED(height);
////
////	vg.save();
////
////	vg.font_size(18.0);
////	vg.font_face("sans");
////	vg.text_align(LEFT|TOP);
////	vg.text_metrics(NULL, NULL, &lineh);
////
////	// The text break API can be used to fill a large buffer of rows,
////	// or to iterate over the text just few lines (or just one) at a time.
////	// The "next" variable of the last returned item tells where to continue.
////	start = text;
////	end = text + strlen(text);
////	while ((nrows = vg.text_break_lines(start, end, width, rows, 3))) {
////		for (i = 0; i < nrows; i++) {
////			struct NVGtextRow* row = &rows[i];
////			int hit = mx > x && mx < (x+width) && my >= y && my < (y+lineh);
////
////			vg.begin_path();
////			vg.fill_color(rgba(255,255,255, if hit {64} else {16}));
////			vg.rect(x, y, row.width, lineh);
////			vg.fill();
////
////			vg.fill_color(rgba(255,255,255,255));
////			vg.text(x, y, row.start, row.end);
////
////			if (hit) {
////				caretx = if mx < x+row.width/2 { x } else { x+row.width };
////				px = x;
////				nglyphs = vg.text_glyph_positions(x, y, row.start, row.end, glyphs, 100);
////				for (j = 0; j < nglyphs; j++) {
////					f32 x0 = glyphs[j].x;
////					f32 x1 = if (j+1 < nglyphs) { glyphs[j+1].x } else { x+row.width };
////					f32 gx = x0 * 0.3 + x1 * 0.7;
////					if (mx >= px && mx < gx)
////						caretx = glyphs[j].x;
////					px = gx;
////				}
////				vg.begin_path();
////				vg.fill_color(rgba(255,192,0,255));
////				vg.rect(caretx, y, 1, lineh);
////				vg.fill();
////
////				gutter = lnum+1;
////				gx = x - 10;
////				gy = y + lineh/2;
////			}
////			lnum++;
////			y += lineh;
////		}
////		// Keep going...
////		start = rows[nrows-1].next;
////	}
////
////	if (gutter) {
////		char txt[16];
////		snprintf(txt, sizeof(txt), "%d", gutter);
////		vg.font_size(13.0);
////		vg.text_align(RIGHT|MIDDLE);
////
////		vg.text_bounds(gx,gy, txt, NULL, bounds);
////
////		vg.begin_path();
////		vg.fill_color(rgba(255,192,0,255));
////		vg.rounded_rect((int)bounds[0]-4,(int)bounds[1]-2, (int)(bounds[2]-bounds[0])+8, (int)(bounds[3]-bounds[1])+4, ((int)(bounds[3]-bounds[1])+4)/2-1);
////		vg.fill();
////
////		vg.fill_color(rgba(32,32,32,255));
////		vg.text(gx,gy, txt, NULL);
////	}
////
////	y += 20.0;
////
////	vg.font_size(13.0);
////	vg.text_align(LEFT|TOP);
////	vg.text_line_height(1.2);
////
////	vg.text_box_bounds(x,y, 150, "Hover your mouse over the text to see calculated caret position.", NULL, bounds);
////
////	// Fade the tooltip out when close to it.
////	gx = fabsf((mx - (bounds[0]+bounds[2])*0.5) / (bounds[0] - bounds[2]));
////	gy = fabsf((my - (bounds[1]+bounds[3])*0.5) / (bounds[1] - bounds[3]));
////	a = maxf(gx, gy) - 0.5;
////	a = clampf(a, 0, 1);
////	vg.global_alpha(a);
////
////	vg.begin_path();
////	vg.fill_color(rgba(220,220,220,255));
////	vg.rounded_rect(bounds[0]-2,bounds[1]-2, (int)(bounds[2]-bounds[0])+4, (int)(bounds[3]-bounds[1])+4, 3);
////	px = (int)((bounds[2]+bounds[0])/2);
////	vg.move_to(px,bounds[1] - 10);
////	vg.line_to(px+7,bounds[1]+1);
////	vg.line_to(px-7,bounds[1]+1);
////	vg.fill();
////
////	vg.fill_color(rgba(0,0,0,220));
////	vg.text_box(x,y, 150, "Hover your mouse over the text to see calculated caret position.", NULL);
////
////	vg.restore();
//}
//
//fn draw_widths(vg: &Ctx, x: c_float,
//              y: c_float, width: c_float)
//{
//	vg.save();
//
//	vg.stroke_color(rgba(0,0,0,255));
//
//	for i in range(0, 20) {
//		let w = (i+0.5)*0.1;
//		vg.stroke_width(w);
//		vg.begin_path();
//		vg.move_to(x,y);
//		vg.line_to(x+width,y+width*0.3);
//		vg.stroke();
//		y += 10;
//	}
//
//	vg.restore();
//}
//
//fn draw_caps(vg: &Ctx, x: c_float,
//            y: c_float, width: c_float)
//{
//	let caps: [int, ..3] = [BUTT, ROUND, SQUARE];
//	let lineWidth = 8.0;
//
//	vg.save();
//
//	vg.begin_path();
//	vg.rect(x-lineWidth/2, y, width+lineWidth, 40);
//	vg.fill_color(rgba(255,255,255,32));
//	vg.fill();
//
//	vg.begin_path();
//	vg.rect(x, y, width, 40);
//	vg.fill_color(rgba(255,255,255,32));
//	vg.fill();
//
//	vg.stroke_width(lineWidth);
//	for i in range(0, 3) {
//		vg.line_cap(caps[i]);
//		vg.stroke_color(rgba(0,0,0,255));
//		vg.begin_path();
//		vg.move_to(x, y + i*10 + 5);
//		vg.line_to(x+width, y + i*10 + 5);
//		vg.stroke();
//	}
//
//	vg.restore();
//}
//
//
//
//fn unpremultiplyAlpha(image: &mut [u8], w: int, h: int, stride: int)
//{
////	// Unpremultiply
////	for y in range(0, h) {
////		unsigned char *row = &image[y*stride];
////		for x in range(0, w) {
////			int r = row[0], g = row[1], b = row[2], a = row[3];
////			if (a != 0) {
////				row[0] = (int)mini(r*255/a, 255);
////				row[1] = (int)mini(g*255/a, 255);
////				row[2] = (int)mini(b*255/a, 255);
////			}
////			row += 4;
////		}
////	}
////
////	// Defringe
////	for y in range(0, h) {
////		unsigned char *row = &image[y*stride];
////		for x in range(0, w) {
////			let mut r = 0;
////			let mut g = 0;
////			let mut b = 0;
////			let mut a = row[3];
////			let mut n = 0;
////			if (a == 0) {
////				if (x-1 > 0 && row[-1] != 0) {
////					r += row[-4];
////					g += row[-3];
////					b += row[-2];
////					n++;
////				}
////				if (x+1 < w && row[7] != 0) {
////					r += row[4];
////					g += row[5];
////					b += row[6];
////					n++;
////				}
////				if (y-1 > 0 && row[-stride+3] != 0) {
////					r += row[-stride];
////					g += row[-stride+1];
////					b += row[-stride+2];
////					n++;
////				}
////				if (y+1 < h && row[stride+3] != 0) {
////					r += row[stride];
////					g += row[stride+1];
////					b += row[stride+2];
////					n++;
////				}
////				if (n > 0) {
////					row[0] = r/n;
////					row[1] = g/n;
////					row[2] = b/n;
////				}
////			}
////			row += 4;
////		}
////	}
//}
//
//fn setAlpha(image: *mut u8, w: int, h: int, stride: int, a: u8)
//{
////	for y in range(0, h) {
////		unsigned char* row = &image[y*stride];
////		for x in range(0, w) {
////			row[x*4+3] = a;
////		}
////	}
//}
//
//fn flipHorizontal(image: *mut u8, w: int, h: int, stride: int)
//{
////	let i = 0;
////	let j = h-1;
////	while (i < j) {
////		unsigned char* ri = &image[i * stride];
////		unsigned char* rj = &image[j * stride];
////		for k in range(0, w*4) {
////			let t = ri[k];
////			ri[k] = rj[k];
////			rj[k] = t;
////		}
////		i++;
////		j--;
////	}
//}

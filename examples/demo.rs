//#include "stb_image_write.h"

static ICON_SEARCH :       u32 = 0x1F50D;
static ICON_CIRCLED_CROSS: u32 =  0x2716;
static ICON_CHEVRON_RIGHT: u32 =  0xE75E;
static ICON_CHECK:         u32 =  0x2713;
static ICON_LOGIN:         u32 =  0xE740;
static ICON_TRASH:         u32 =  0xE729;

static float minf(float a, float b) { if a < b { a } else { b } }
static float maxf(float a, float b) { if a > b { a } else { b } }
static float absf(float a) { if a >= 0.0 { a } else { -a } }
static float clampf(float a, float mn, float mx) { if a < mn { mn } else { if a > mx { mx } else { a } } }

static char* cpToUTF8(int cp, char* str)
{
	int n = 0;
	if (cp < 0x80) n = 1;
	else if (cp < 0x800) n = 2;
	else if (cp < 0x10000) n = 3;
	else if (cp < 0x200000) n = 4;
	else if (cp < 0x4000000) n = 5;
	else if (cp <= 0x7fffffff) n = 6;
	str[n] = '\0';
	switch (n) {
	case 6: str[5] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x4000000;
	case 5: str[4] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x200000;
	case 4: str[3] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x10000;
	case 3: str[2] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0x800;
	case 2: str[1] = 0x80 | (cp & 0x3f); cp = cp >> 6; cp |= 0xc0;
	case 1: str[0] = cp;
	}
	return str;
}

struct DemoData {
	fontNormal: int,
	fontBold: int,
	fontIcons: int,
	images: [int, ..12],
};


//int loadDemoData(struct NVGcontext* vg, struct DemoData* data)
pub fn load_demo_data(vg: *mut Struct_NVGcontext,
                    data: *mut Struct_DemoData) -> c_int {
	int i;

	if (vg == NULL)
		return -1;

	for (i = 0; i < 12; i++) {
		char file[128];
		snprintf(file, 128, "../example/images/image%d.jpg", i+1);
		data->images[i] = vg.create_image(file);
		if (data->images[i] == 0) {
			printf("Could not load %s.\n", file);
			return -1;
		}
	}

	data->fontIcons = vg.create_font("icons", "../example/entypo.ttf");
	if (data->fontIcons == -1) {
		printf("Could not add font icons.\n");
		return -1;
	}
	data->fontNormal = vg.create_font("sans", "../example/Roboto-Regular.ttf");
	if (data->fontNormal == -1) {
		printf("Could not add font italic.\n");
		return -1;
	}
	data->fontBold = vg.create_font("sans-bold", "../example/Roboto-Bold.ttf");
	if (data->fontBold == -1) {
		printf("Could not add font bold.\n");
		return -1;
	}

	return 0;
}


//void freeDemoData(struct NVGcontext* vg, struct DemoData* data)
pub fn free_demo_data(vg: *mut Struct_NVGcontext,
                    data: *mut Struct_DemoData) {
	int i;

	if (vg == NULL)
		return;

	for (i = 0; i < 12; i++)
		vg.delete_image(data->images[i]);
}


//void renderDemo(struct NVGcontext* vg, float mx, float my, float width, float height, float t, int blowup, struct DemoData* data)
pub fn render_demo(vg: *mut Struct_NVGcontext, mx: c_float,
                  my: c_float, width: c_float,
                  height: c_float, t: c_float,
                  blowup: c_int, data: *mut Struct_DemoData) {
	float x,y,popy;

	drawEyes(vg, width - 250, 50, 150, 100, mx, my, t);
	drawParagraph(vg, width - 450, 50, 150, 100, mx, my);
	drawGraph(vg, 0, height/2, width, height/2, t);
	drawColorwheel(vg, width - 300, height - 300, 250.0, 250.0, t);

	// Line joints
	drawLines(vg, 50, height-50, 600, 50, t);

	// Line caps
	drawWidths(vg, 10, 50, 30);

	// Line caps
	drawCaps(vg, 10, 300, 30);

	vg.save();
	if (blowup) {
		vg.rotate(sinf(t*0.3)*5.0/180.0*NVG_PI);
		vg.scale(2.0, 2.0);
	}

	// Widgets
	drawWindow(vg, "Widgets `n Stuff", 50, 50, 300, 400);
	x = 60; y = 95;
	drawSearchBox(vg, "Search", x,y,280,25);
	y += 40;
	drawDropDown(vg, "Effects", x,y,280,28);
	popy = y + 14;
	y += 45;

	// Form
	drawLabel(vg, "Login", x,y, 280,20);
	y += 25;
	drawEditBox(vg, "Email",  x,y, 280,28);
	y += 35;
	drawEditBox(vg, "Password", x,y, 280,28);
	y += 38;
	drawCheckBox(vg, "Remember me", x,y, 140,28);
	drawButton(vg, ICON_LOGIN, "Sign in", x+138, y, 140, 28, nvgRGBA(0,96,128,255));
	y += 45;

	// Slider
	drawLabel(vg, "Diameter", x,y, 280,20);
	y += 25;
	drawEditBoxNum(vg, "123.00", "px", x+180,y, 100,28);
	drawSlider(vg, 0.4, x,y, 170,28);
	y += 55;

	drawButton(vg, ICON_TRASH, "Delete", x, y, 160, 28, nvgRGBA(128,16,8,255));
	drawButton(vg, 0, "Cancel", x+170, y, 110, 28, nvgRGBA(0,0,0,0));

	// Thumbnails box
	drawThumbnails(vg, 365, popy-30, 160, 300, data->images, 12, t);

	vg.restore();
}


//void saveScreenShot(int w, int h, int premult, const char* name)
pub fn save_screenshot(w: c_int, h: c_int,
                      premult: c_int, name: *c_char) {
	unsigned char* image = (unsigned char*)malloc(w*h*4);
	if (image == NULL)
		return;
	glReadPixels(0, 0, w, h, GL_RGBA, GL_UNSIGNED_BYTE, image);
	if (premult)
		unpremultiplyAlpha(image, w, h, w*4);
	else
		setAlpha(image, w, h, w*4, 255);
	flipHorizontal(image, w, h, w*4);
 	stbi_write_png(name, w, h, 4, image, w*4);
 	free(image);
}



fn is_black(col: Struct_NVGcolor) -> bool {
	col.r == 0.0 && col.g == 0.0 && col.b == 0.0 && col.a == 0.0
}

fn draw_window(vg: *mut Struct_NVGcontext, title: *c_char,
              x: c_float, y: c_float,
              w: c_float, h: c_float) {
	float cornerRadius = 3.0;
	struct NVGpaint shadowPaint;
	struct NVGpaint headerPaint;

	vg.save();
//	vg.clear_state();

	// Window
	vg.begin_path();
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.fill_color(nvgRGBA(28,30,34,192));
//	vg.fill_color(nvgRGBA(0,0,0,128));
	vg.fill();

	// Drop shadow
	shadowPaint = vg.box_gradient(x,y+2, w,h, cornerRadius*2, 10, nvgRGBA(0,0,0,128), nvgRGBA(0,0,0,0));
	vg.begin_path();
	vg.rect(x-10,y-10, w+20,h+30);
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.path_winding(NVG_HOLE);
	vg.fill_paint(shadowPaint);
	vg.fill();

	// Header
	headerPaint = vg.linear_gradient(x,y,x,y+15, nvgRGBA(255,255,255,8), nvgRGBA(0,0,0,16));
	vg.begin_path();
	vg.rounded_rect(x+1,y+1, w-2,30, cornerRadius-1);
	vg.fill_paint(headerPaint);
	vg.fill();
	vg.begin_path();
	vg.move_to(x+0.5, y+0.5+30);
	vg.line_to(x+0.5+w-1, y+0.5+30);
	vg.stroke_color(nvgRGBA(0,0,0,32));
	vg.stroke();

	vg.font_size(18.0);
	vg.font_face("sans-bold");
	vg.text_align(,NVG_ALIGN_CENTER|NVG_ALIGN_MIDDLE);

	vg.font_blur(,2);
	vg.fill_color(nvgRGBA(0,0,0,128));
	vg.text(x+w/2,y+16+1, title, NULL);

	vg.font_blur(,0);
	vg.fill_color(nvgRGBA(220,220,220,160));
	vg.text(x+w/2,y+16, title, NULL);

	vg.restore();
}

fn draw_searchbox(vg: *mut Struct_NVGcontext, text: *c_char,
                 x: c_float, y: c_float,
                 w: c_float, h: c_float) {
	struct NVGpaint bg;
	char icon[8];
	float cornerRadius = h/2-1;

	// Edit
	bg = vg.box_gradient(x,y+1.5, w,h, h/2,5, nvgRGBA(0,0,0,16), nvgRGBA(0,0,0,92));
	vg.begin_path();
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.fill_paint(bg);
	vg.fill();

/*	vg.begin_path();
	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
	vg.stroke_color(nvgRGBA(0,0,0,48));
	vg.stroke();*/

	vg.font_size(h*1.3);
	vg.font_face("icons");
	vg.fill_color(nvgRGBA(255,255,255,64));
	vg.text_align(,NVG_ALIGN_CENTER|NVG_ALIGN_MIDDLE);
	vg.text(x+h*0.55, y+h*0.55, cpToUTF8(ICON_SEARCH,icon), NULL);

	vg.font_size(20.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,32));

	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.text(x+h*1.05,y+h*0.5,text, NULL);

	vg.font_size(h*1.3);
	vg.font_face("icons");
	vg.fill_color(nvgRGBA(255,255,255,32));
	vg.text_align(,NVG_ALIGN_CENTER|NVG_ALIGN_MIDDLE);
	vg.text(x+w-h*0.55, y+h*0.55, cpToUTF8(ICON_CIRCLED_CROSS,icon), NULL);
}

fn draw_dropdown(vg: *mut Struct_NVGcontext, text: *c_char,
                x: c_float, y: c_float,
                w: c_float, h: c_float) {
	struct NVGpaint bg;
	char icon[8];
	float cornerRadius = 4.0;

	bg = vg.linear_gradient(x,y,x,y+h, nvgRGBA(255,255,255,16), nvgRGBA(0,0,0,16));
	vg.begin_path();
	vg.rounded_rect(x+1,y+1, w-2,h-2, cornerRadius-1);
	vg.fill_paint(bg);
	vg.fill();

	vg.begin_path();
	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
	vg.stroke_color(nvgRGBA(0,0,0,48));
	vg.stroke();

	vg.font_size(20.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,160));
	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.text(x+h*0.3,y+h*0.5,text, NULL);

	vg.font_size(h*1.3);
	vg.font_face("icons");
	vg.fill_color(nvgRGBA(255,255,255,64));
	vg.text_align(,NVG_ALIGN_CENTER|NVG_ALIGN_MIDDLE);
	vg.text(x+w-h*0.5, y+h*0.5, cpToUTF8(ICON_CHEVRON_RIGHT,icon), NULL);
}

fn draw_label(vg: *mut Struct_NVGcontext, text: *c_char,
             x: c_float, y: c_float,
             w: c_float, h: c_float) {
	NVG_NOTUSED(w);

	vg.font_size(18.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,128));

	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.text(x,y+h*0.5,text, NULL);
}

fn draw_editbox_base(vg: *mut Struct_NVGcontext, x: c_float,
                   y: c_float, w: c_float,
                   h: c_float) {
	struct NVGpaint bg;
	// Edit
	bg = vg.box_gradient(x+1,y+1+1.5, w-2,h-2, 3,4, nvgRGBA(255,255,255,32), nvgRGBA(32,32,32,32));
	vg.begin_path();
	vg.rounded_rect(x+1,y+1, w-2,h-2, 4-1);
	vg.fill_paint(bg);
	vg.fill();

	vg.begin_path();
	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, 4-0.5);
	vg.stroke_color(nvgRGBA(0,0,0,48));
	vg.stroke();
}

fn draw_editbox(vg: *mut Struct_NVGcontext, text: *c_char,
               x: c_float, y: c_float,
               w: c_float, h: c_float) {
	drawEditBoxBase(vg, x,y, w,h);

	vg.font_size(20.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,64));
	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.text(x+h*0.3,y+h*0.5,text, NULL);
}

fn draw_editbox_num(vg: *mut Struct_NVGcontext, text: *c_char,
                  units: *c_char, x: c_float,
                  y: c_float, w: c_float,
				  h: c_float) {
	float uw;

	drawEditBoxBase(vg, x,y, w,h);

	uw = vg.text_bounds(0,0, units, NULL, NULL);

	vg.font_size(18.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,64));
	vg.text_align(,NVG_ALIGN_RIGHT|NVG_ALIGN_MIDDLE);
	vg.text(x+w-h*0.3,y+h*0.5,units, NULL);

	vg.font_size(20.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,128));
	vg.text_align(,NVG_ALIGN_RIGHT|NVG_ALIGN_MIDDLE);
	vg.text(x+w-uw-h*0.5,y+h*0.5,text, NULL);
}

fn draw_checkbox(vg: *mut Struct_NVGcontext, text: *c_char,
                x: c_float, y: c_float,
                w: c_float, h: c_float) {
	struct NVGpaint bg;
	char icon[8];
	NVG_NOTUSED(w);

	vg.font_size(18.0);
	vg.font_face("sans");
	vg.fill_color(nvgRGBA(255,255,255,160));

	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.text(x+28,y+h*0.5,text, NULL);

	bg = vg.box_gradient(x+1,y+(int)(h*0.5)-9+1, 18,18, 3,3, nvgRGBA(0,0,0,32), nvgRGBA(0,0,0,92));
	vg.begin_path();
	vg.rounded_rect(x+1,y+(int)(h*0.5)-9, 18,18, 3);
	vg.fill_paint(bg);
	vg.fill();

	vg.font_size(40);
	vg.font_face("icons");
	vg.fill_color(nvgRGBA(255,255,255,128));
	vg.text_align(,NVG_ALIGN_CENTER|NVG_ALIGN_MIDDLE);
	vg.text(x+9+2, y+h*0.5, cpToUTF8(ICON_CHECK,icon), NULL);
}

fn draw_button(vg: *mut Struct_NVGcontext, preicon: c_int,
              text: *c_char, x: c_float,
              y: c_float, w: c_float,
			  h: c_float, col: Struct_NVGcolor) {
	struct NVGpaint bg;
	char icon[8];
	float cornerRadius = 4.0;
	float tw = 0, iw = 0;

	bg = vg.linear_gradient(x,y,x,y+h, nvgRGBA(255,255,255,if isBlack(col){16}else{32}), nvgRGBA(0,0,0,if isBlack(col){16}else{32}));
	vg.begin_path();
	vg.rounded_rect(x+1,y+1, w-2,h-2, cornerRadius-1);
	if (!isBlack(col)) {
		vg.fill_color(col);
		vg.fill();
	}
	vg.fill_paint(bg);
	vg.fill();

	vg.begin_path();
	vg.rounded_rect(x+0.5,y+0.5, w-1,h-1, cornerRadius-0.5);
	vg.stroke_color(nvgRGBA(0,0,0,48));
	vg.stroke();

	vg.font_size(20.0);
	vg.font_face("sans-bold");
	tw = vg.text_bounds(0,0, text, NULL, NULL);
	if (preicon != 0) {
		vg.font_size(h*1.3);
		vg.font_face("icons");
		iw = vg.text_bounds(0,0, cpToUTF8(preicon,icon), NULL, NULL);
		iw += h*0.15;
	}

	if (preicon != 0) {
		vg.font_size(h*1.3);
		vg.font_face("icons");
		vg.fill_color(nvgRGBA(255,255,255,96));
		vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
		vg.text(x+w*0.5-tw*0.5-iw*0.75, y+h*0.5, cpToUTF8(preicon,icon), NULL);
	}

	vg.font_size(20.0);
	vg.font_face("sans-bold");
	vg.text_align(,NVG_ALIGN_LEFT|NVG_ALIGN_MIDDLE);
	vg.fill_color(nvgRGBA(0,0,0,160));
	vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5-1,text, NULL);
	vg.fill_color(nvgRGBA(255,255,255,160));
	vg.text(x+w*0.5-tw*0.5+iw*0.25,y+h*0.5,text, NULL);
}

fn draw_slider(vg: *mut Struct_NVGcontext, pos: c_float,
              x: c_float, y: c_float,
              w: c_float, h: c_float) {
	struct NVGpaint bg, knob;
	float cy = y+(int)(h*0.5);
	float kr = (int)(h*0.25);

	vg.save();
//	vg.clear_state();

	// Slot
	bg = vg.box_gradient(x,cy-2+1, w,4, 2,2, nvgRGBA(0,0,0,32), nvgRGBA(0,0,0,128));
	vg.begin_path();
	vg.rounded_rect(x,cy-2, w,4, 2);
	vg.fill_paint(bg);
	vg.fill();

	// Knob Shadow
	bg = vg.radial_gradient(x+(int)(pos*w),cy+1, kr-3,kr+3, nvgRGBA(0,0,0,64), nvgRGBA(0,0,0,0));
	vg.begin_path();
	vg.rect(x+(int)(pos*w)-kr-5,cy-kr-5,kr*2+5+5,kr*2+5+5+3);
	vg.circle(x+(int)(pos*w),cy, kr);
	vg.path_winding(NVG_HOLE);
	vg.fill_paint(bg);
	vg.fill();

	// Knob
	knob = vg.linear_gradient(x,cy-kr,x,cy+kr, nvgRGBA(255,255,255,16), nvgRGBA(0,0,0,16));
	vg.begin_path();
	vg.circle(x+(int)(pos*w),cy, kr-1);
	vg.fill_color(nvgRGBA(40,43,48,255));
	vg.fill();
	vg.fill_paint(knob);
	vg.fill();

	vg.begin_path();
	vg.circle(x+(int)(pos*w),cy, kr-0.5);
	vg.stroke_color(nvgRGBA(0,0,0,92));
	vg.stroke();

	vg.restore();
}

fn draw_eyes(vg: *mut Struct_NVGcontext, x: c_float,
            y: c_float, w: c_float,
            h: c_float, mx: c_float,
            my: c_float, t: c_float) {
	struct NVGpaint gloss, bg;
	float ex = w *0.23;
	float ey = h * 0.5;
	float lx = x + ex;
	float ly = y + ey;
	float rx = x + w - ex;
	float ry = y + ey;
	float dx,dy,d;
	float br = (ex < ey ? ex : ey) * 0.5;
	float blink = 1 - pow(sinf(t*0.5),200)*0.8;

	bg = vg.linear_gradient(x,y+h*0.5,x+w*0.1,y+h, nvgRGBA(0,0,0,32), nvgRGBA(0,0,0,16));
	vg.begin_path();
	vg.ellipse(lx+3.0,ly+16.0, ex,ey);
	vg.ellipse(rx+3.0,ry+16.0, ex,ey);
	vg.fill_paint(bg);
	vg.fill();

	bg = vg.linear_gradient(x,y+h*0.25,x+w*0.1,y+h, nvgRGBA(220,220,220,255), nvgRGBA(128,128,128,255));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(bg);
	vg.fill();

	dx = (mx - rx) / (ex * 10);
	dy = (my - ry) / (ey * 10);
	d = sqrtf(dx*dx+dy*dy);
	if (d > 1.0) {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(lx+dx,ly+dy+ey*0.25*(1-blink), br,br*blink);
	vg.fill_color(nvgRGBA(32,32,32,255));
	vg.fill();

	dx = (mx - rx) / (ex * 10);
	dy = (my - ry) / (ey * 10);
	d = sqrtf(dx*dx+dy*dy);
	if (d > 1.0) {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(rx+dx,ry+dy+ey*0.25*(1-blink), br,br*blink);
	vg.fill_color(nvgRGBA(32,32,32,255));
	vg.fill();

	gloss = vg.radial_gradient(lx-ex*0.25,ly-ey*0.5, ex*0.1,ex*0.75, nvgRGBA(255,255,255,128), nvgRGBA(255,255,255,0));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.fill_paint(gloss);
	vg.fill();

	gloss = vg.radial_gradient(rx-ex*0.25,ry-ey*0.5, ex*0.1,ex*0.75, nvgRGBA(255,255,255,128), nvgRGBA(255,255,255,0));
	vg.begin_path();
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(gloss);
	vg.fill();
}

fn draw_graph(vg: *mut Struct_NVGcontext, x: c_float,
             y: c_float, w: c_float,
             h: c_float, t: c_float) {
	struct NVGpaint bg;
	float samples[6];
	float sx[6], sy[6];
	float dx = w/5.0;
	int i;

	samples[0] = (1+sinf(t*1.2345+cosf(t*0.33457)*0.44))*0.5;
	samples[1] = (1+sinf(t*0.68363+cosf(t*1.3)*1.55))*0.5;
	samples[2] = (1+sinf(t*1.1642+cosf(t*0.33457)*1.24))*0.5;
	samples[3] = (1+sinf(t*0.56345+cosf(t*1.63)*0.14))*0.5;
	samples[4] = (1+sinf(t*1.6245+cosf(t*0.254)*0.3))*0.5;
	samples[5] = (1+sinf(t*0.345+cosf(t*0.03)*0.6))*0.5;

	for (i = 0; i < 6; i++) {
		sx[i] = x+i*dx;
		sy[i] = y+h*samples[i]*0.8;
	}

	// Graph background
	bg = vg.linear_gradient(x,y,x,y+h, nvgRGBA(0,160,192,0), nvgRGBA(0,160,192,64));
	vg.begin_path();
	vg.move_to(sx[0], sy[0]);
	for (i = 1; i < 6; i++)
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
	vg.line_to(x+w, y+h);
	vg.line_to(x, y+h);
	vg.fill_paint(bg);
	vg.fill();

	// Graph line
	vg.begin_path();
	vg.move_to(sx[0], sy[0]+2);
	for (i = 1; i < 6; i++)
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1]+2, sx[i]-dx*0.5,sy[i]+2, sx[i],sy[i]+2);
	vg.stroke_color(nvgRGBA(0,0,0,32));
	vg.stroke_width(3.0);
	vg.stroke();

	vg.begin_path();
	vg.move_to(sx[0], sy[0]);
	for (i = 1; i < 6; i++)
		vg.bezier_to(sx[i-1]+dx*0.5,sy[i-1], sx[i]-dx*0.5,sy[i], sx[i],sy[i]);
	vg.stroke_color(nvgRGBA(0,160,192,255));
	vg.stroke_width(3.0);
	vg.stroke();

	// Graph sample pos
	for (i = 0; i < 6; i++) {
		bg = vg.radial_gradient(sx[i],sy[i]+2, 3.0,8.0, nvgRGBA(0,0,0,32), nvgRGBA(0,0,0,0));
		vg.begin_path();
		vg.rect(sx[i]-10, sy[i]-10+2, 20,20);
		vg.fill_paint(bg);
		vg.fill();
	}

	vg.begin_path();
	for (i = 0; i < 6; i++)
		vg.circle(sx[i], sy[i], 4.0);
	vg.fill_color(nvgRGBA(0,160,192,255));
	vg.fill();
	vg.begin_path();
	for (i = 0; i < 6; i++)
		vg.circle(sx[i], sy[i], 2.0);
	vg.fill_color(nvgRGBA(220,220,220,255));
	vg.fill();

	vg.stroke_width(1.0);
}

fn draw_spinner(vg: *mut Struct_NVGcontext, cx: c_float,
               cy: c_float, r: c_float,
               t: c_float) {
	float a0 = 0.0 + t*6;
	float a1 = NVG_PI + t*6;
	float r0 = r;
	float r1 = r * 0.75;
	float ax,ay, bx,by;
	struct NVGpaint paint;

	vg.save();

	vg.begin_path();
	vg.arc(cx,cy, r0, a0, a1, NVG_CW);
	vg.arc(cx,cy, r1, a1, a0, NVG_CCW);
	vg.close_path();
	ax = cx + cosf(a0) * (r0+r1)*0.5;
	ay = cy + sinf(a0) * (r0+r1)*0.5;
	bx = cx + cosf(a1) * (r0+r1)*0.5;
	by = cy + sinf(a1) * (r0+r1)*0.5;
	paint = vg.linear_gradient(ax,ay, bx,by, nvgRGBA(0,0,0,0), nvgRGBA(0,0,0,128));
	vg.fill_paint(paint);
	vg.fill();

	vg.restore();
}

fn draw_thumbnails(vg: *mut Struct_NVGcontext, x: c_float,
                  y: c_float, w: c_float,
                  h: c_float, images: *c_int,
                  nimages: c_int, t: c_float) {
	float cornerRadius = 3.0;
	struct NVGpaint shadowPaint, imgPaint, fadePaint;
	float ix,iy,iw,ih;
	float thumb = 60.0;
	float arry = 30.5;
	int imgw, imgh;
	float stackh = (nimages/2) * (thumb+10) + 10;
	int i;
	float u = (1+cosf(t*0.5))*0.5;
	float u2 = (1-cosf(t*0.2))*0.5;
	float scrollh, dv;

	vg.save();
//	vg.clear_state();

	// Drop shadow
	shadowPaint = vg.box_gradient(x,y+4, w,h, cornerRadius*2, 20, nvgRGBA(0,0,0,128), nvgRGBA(0,0,0,0));
	vg.begin_path();
	vg.rect(x-10,y-10, w+20,h+30);
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.path_winding(NVG_HOLE);
	vg.fill_paint(shadowPaint);
	vg.fill();

	// Window
	vg.begin_path();
	vg.rounded_rect(x,y, w,h, cornerRadius);
	vg.move_to(x-10,y+arry);
	vg.line_to(x+1,y+arry-11);
	vg.line_to(x+1,y+arry+11);
	vg.fill_color(nvgRGBA(200,200,200,255));
	vg.fill();

	vg.save();
	vg.scissor(x,y,w,h);
	vg.translate(0, -(stackh - h)*u);

	dv = 1.0 / (float)(nimages-1);

	for (i = 0; i < nimages; i++) {
		float tx, ty, v, a;
		tx = x+10;
		ty = y+10;
		tx += (i%2) * (thumb+10);
		ty += (i/2) * (thumb+10);
		vg.image_size(images[i], &imgw, &imgh);
		if (imgw < imgh) {
			iw = thumb;
			ih = iw * (float)imgh/(float)imgw;
			ix = 0;
			iy = -(ih-thumb)*0.5;
		} else {
			ih = thumb;
			iw = ih * (float)imgw/(float)imgh;
			ix = -(iw-thumb)*0.5;
			iy = 0;
		}

		v = i * dv;
		a = clampf((u2-v) / dv, 0, 1);

		if (a < 1.0)
			drawSpinner(vg, tx+thumb/2,ty+thumb/2, thumb*0.25, t);

		imgPaint = vg.image_pattern(tx+ix, ty+iy, iw,ih, 0.0/180.0*NVG_PI, images[i], NVG_NOREPEAT, a);
		vg.begin_path();
		vg.rounded_rect(tx,ty, thumb,thumb, 5);
		vg.fill_paint(imgPaint);
		vg.fill();

		shadowPaint = vg.box_gradient(tx-1,ty, thumb+2,thumb+2, 5, 3, nvgRGBA(0,0,0,128), nvgRGBA(0,0,0,0));
		vg.begin_path();
		vg.rect(tx-5,ty-5, thumb+10,thumb+10);
		vg.rounded_rect(tx,ty, thumb,thumb, 6);
		vg.path_winding(NVG_HOLE);
		vg.fill_paint(shadowPaint);
		vg.fill();

		vg.begin_path();
		vg.rounded_rect(tx+0.5,ty+0.5, thumb-1,thumb-1, 4-0.5);
		vg.stroke_width(,1.0);
		vg.stroke_color(nvgRGBA(255,255,255,192));
		vg.stroke();
	}
	vg.restore();

	// Hide fades
	fadePaint = vg.linear_gradient(x,y,x,y+6, nvgRGBA(200,200,200,255), nvgRGBA(200,200,200,0));
	vg.begin_path();
	vg.rect(x+4,y,w-8,6);
	vg.fill_paint(fadePaint);
	vg.fill();

	fadePaint = vg.linear_gradient(x,y+h,x,y+h-6, nvgRGBA(200,200,200,255), nvgRGBA(200,200,200,0));
	vg.begin_path();
	vg.rect(x+4,y+h-6,w-8,6);
	vg.fill_paint(fadePaint);
	vg.fill();

	// Scroll bar
	shadowPaint = vg.box_gradient(x+w-12+1,y+4+1, 8,h-8, 3,4, nvgRGBA(0,0,0,32), nvgRGBA(0,0,0,92));
	vg.begin_path();
	vg.rounded_rect(x+w-12,y+4, 8,h-8, 3);
	vg.fill_paint(shadowPaint);
//	vg.fill_color(nvgRGBA(255,0,0,128));
	vg.fill();

	scrollh = (h/stackh) * (h-8);
	shadowPaint = vg.box_gradient(x+w-12-1,y+4+(h-8-scrollh)*u-1, 8,scrollh, 3,4, nvgRGBA(220,220,220,255), nvgRGBA(128,128,128,255));
	vg.begin_path();
	vg.rounded_rect(x+w-12+1,y+4+1 + (h-8-scrollh)*u, 8-2,scrollh-2, 2);
	vg.fill_paint(shadowPaint);
//	vg.fill_color(nvgRGBA(0,0,0,128));
	vg.fill();

	vg.restore();
}

fn draw_colorwheel(vg: *mut Struct_NVGcontext, x: c_float,
                  y: c_float, w: c_float,
                  h: c_float, t: c_float) {
	int i;
	float r0, r1, ax,ay, bx,by, cx,cy, aeps, r;
	float hue = sinf(t * 0.12);
	struct NVGpaint paint;

	vg.save();

/*	vg.begin_path();
	vg.rect(x,y,w,h);
	vg.fill_color(nvgRGBA(255,0,0,128));
	vg.fill();*/

	cx = x + w*0.5;
	cy = y + h*0.5;
	r1 = (w < h ? w : h) * 0.5 - 5.0;
	r0 = r1 - 20.0;
	aeps = 0.5 / r1;	// half a pixel arc length in radians (2pi cancels out).

	for (i = 0; i < 6; i++) {
		float a0 = (float)i / 6.0 * NVG_PI * 2.0 - aeps;
		float a1 = (float)(i+1.0) / 6.0 * NVG_PI * 2.0 + aeps;
		vg.begin_path();
		vg.arc(cx,cy, r0, a0, a1, NVG_CW);
		vg.arc(cx,cy, r1, a1, a0, NVG_CCW);
		vg.close_path();
		ax = cx + cosf(a0) * (r0+r1)*0.5;
		ay = cy + sinf(a0) * (r0+r1)*0.5;
		bx = cx + cosf(a1) * (r0+r1)*0.5;
		by = cy + sinf(a1) * (r0+r1)*0.5;
		paint = vg.linear_gradient(ax,ay, bx,by, nvgHSLA(a0/(NVG_PI*2),1.0,0.55,255), nvgHSLA(a1/(NVG_PI*2),1.0,0.55,255));
		vg.fill_paint(paint);
		vg.fill();
	}

	vg.begin_path();
	vg.circle(cx,cy, r0-0.5);
	vg.circle(cx,cy, r1+0.5);
	vg.stroke_color(nvgRGBA(0,0,0,64));
	vg.stroke_width(1.0);
	vg.stroke();

	// Selector
	vg.save();
	vg.translate(cx,cy);
	vg.rotate(hue*NVG_PI*2);

	// Marker on
	vg.stroke_width(2.0);
	vg.begin_path();
	vg.rect(r0-1,-3,r1-r0+2,6);
	vg.stroke_color(nvgRGBA(255,255,255,192));
	vg.stroke();

	paint = vg.box_gradient(r0-3,-5,r1-r0+6,10, 2,4, nvgRGBA(0,0,0,128), nvgRGBA(0,0,0,0));
	vg.begin_path();
	vg.rect(r0-2-10,-4-10,r1-r0+4+20,8+20);
	vg.rect(r0-2,-4,r1-r0+4,8);
	vg.path_winding(NVG_HOLE);
	vg.fill_paint(paint);
	vg.fill();

	// Center triangle
	r = r0 - 6;
	ax = cosf(120.0/180.0*NVG_PI) * r;
	ay = sinf(120.0/180.0*NVG_PI) * r;
	bx = cosf(-120.0/180.0*NVG_PI) * r;
	by = sinf(-120.0/180.0*NVG_PI) * r;
	vg.begin_path();
	vg.move_to(r,0);
	vg.line_to(ax,ay);
	vg.line_to(bx,by);
	vg.close_path();
	paint = vg.linear_gradient(r,0, ax,ay, nvgHSLA(hue,1.0,0.5,255), nvgRGBA(255,255,255,255));
	vg.fill_paint(paint);
	vg.fill();
	paint = vg.linear_gradient((r+ax)*0.5,(0+ay)*0.5, bx,by, nvgRGBA(0,0,0,0), nvgRGBA(0,0,0,255));
	vg.fill_paint(paint);
	vg.fill();
	vg.stroke_color(nvgRGBA(0,0,0,64));
	vg.stroke();

	// Select circle on triangle
	ax = cosf(120.0/180.0*NVG_PI) * r*0.3;
	ay = sinf(120.0/180.0*NVG_PI) * r*0.4;
	vg.stroke_width(2.0);
	vg.begin_path();
	vg.circle(ax,ay,5);
	vg.stroke_color(nvgRGBA(255,255,255,192));
	vg.stroke();

	paint = vg.radial_gradient(ax,ay, 7,9, nvgRGBA(0,0,0,64), nvgRGBA(0,0,0,0));
	vg.begin_path();
	vg.rect(ax-20,ay-20,40,40);
	vg.circle(ax,ay,7);
	vg.path_winding(NVG_HOLE);
	vg.fill_paint(paint);
	vg.fill();

	vg.restore();

	vg.restore();
}

fn draw_lines(vg: *mut Struct_NVGcontext, x: c_float,
             y: c_float, w: c_float,
             h: c_float, t: c_float) {
	int i, j;
	float pad = 5.0, s = w/9.0 - pad*2;
	float pts[4*2], fx, fy;
	int joins[3] = {NVG_MITER, NVG_ROUND, NVG_BEVEL};
	int caps[3] = {NVG_BUTT, NVG_ROUND, NVG_SQUARE};
	NVG_NOTUSED(h);

	vg.save();
	pts[0] = -s*0.25 + cosf(t*0.3) * s*0.5;
	pts[1] = sinf(t*0.3) * s*0.5;
	pts[2] = -s*0.25;
	pts[3] = 0;
	pts[4] = s*0.25;
	pts[5] = 0;
	pts[6] = s*0.25 + cosf(-t*0.3) * s*0.5;
	pts[7] = sinf(-t*0.3) * s*0.5;

	for (i = 0; i < 3; i++) {
		for (j = 0; j < 3; j++) {
			fx = x + s*0.5 + (i*3+j)/9.0*w + pad;
			fy = y - s*0.5 + pad;

			vg.line_cap(caps[i]);
			vg.line_join(joins[j]);

			vg.stroke_width(s*0.3);
			vg.stroke_color(nvgRGBA(0,0,0,160));
			vg.begin_path();
			vg.move_to(fx+pts[0], fy+pts[1]);
			vg.line_to(fx+pts[2], fy+pts[3]);
			vg.line_to(fx+pts[4], fy+pts[5]);
			vg.line_to(fx+pts[6], fy+pts[7]);
			vg.stroke();

			vg.line_cap(NVG_BUTT);
			vg.line_join(NVG_BEVEL);

			vg.stroke_width(1.0);
			vg.stroke_color(nvgRGBA(0,192,255,255));
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

fn draw_paragraph(vg: *mut Struct_NVGcontext, x: c_float,
                 y: c_float, width: c_float,
                 height: c_float, mx: c_float,
                 my: c_float) {
	struct NVGtextRow rows[3];
	struct NVGglyphPosition glyphs[100];
	const char* text = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.";
	const char* start;
	const char* end;
	int nrows, i, nglyphs, j, lnum = 0;
	float lineh;
	float caretx, px;
	float bounds[4];
	float a;
	float gx,gy;
	int gutter = 0;
	NVG_NOTUSED(height);

	vg.save();

	vg.font_size(18.0);
	vg.font_face("sans");
	vg.text_align(NVG_ALIGN_LEFT|NVG_ALIGN_TOP);
	vg.text_metrics(NULL, NULL, &lineh);

	// The text break API can be used to fill a large buffer of rows,
	// or to iterate over the text just few lines (or just one) at a time.
	// The "next" variable of the last returned item tells where to continue.
	start = text;
	end = text + strlen(text);
	while ((nrows = vg.text_break_lines(start, end, width, rows, 3))) {
		for (i = 0; i < nrows; i++) {
			struct NVGtextRow* row = &rows[i];
			int hit = mx > x && mx < (x+width) && my >= y && my < (y+lineh);

			vg.begin_path();
			vg.fill_color(nvgRGBA(255,255,255,hit?64:16));
			vg.rect(x, y, row->width, lineh);
			vg.fill();

			vg.fill_color(nvgRGBA(255,255,255,255));
			vg.text(x, y, row->start, row->end);

			if (hit) {
				caretx = (mx < x+row->width/2) ? x : x+row->width;
				px = x;
				nglyphs = vg.text_glyph_positions(x, y, row->start, row->end, glyphs, 100);
				for (j = 0; j < nglyphs; j++) {
					float x0 = glyphs[j].x;
					float x1 = (j+1 < nglyphs) ? glyphs[j+1].x : x+row->width;
					float gx = x0 * 0.3 + x1 * 0.7;
					if (mx >= px && mx < gx)
						caretx = glyphs[j].x;
					px = gx;
				}
				vg.begin_path();
				vg.fill_color(nvgRGBA(255,192,0,255));
				vg.rect(caretx, y, 1, lineh);
				vg.fill();

				gutter = lnum+1;
				gx = x - 10;
				gy = y + lineh/2;
			}
			lnum++;
			y += lineh;
		}
		// Keep going...
		start = rows[nrows-1].next;
	}

	if (gutter) {
		char txt[16];
		snprintf(txt, sizeof(txt), "%d", gutter);
		vg.font_size(13.0);
		vg.text_align(NVG_ALIGN_RIGHT|NVG_ALIGN_MIDDLE);

		vg.text_bounds(gx,gy, txt, NULL, bounds);

		vg.begin_path();
		vg.fill_color(nvgRGBA(255,192,0,255));
		vg.rounded_rect((int)bounds[0]-4,(int)bounds[1]-2, (int)(bounds[2]-bounds[0])+8, (int)(bounds[3]-bounds[1])+4, ((int)(bounds[3]-bounds[1])+4)/2-1);
		vg.fill();

		vg.fill_color(nvgRGBA(32,32,32,255));
		vg.text(gx,gy, txt, NULL);
	}

	y += 20.0;

	vg.font_size(13.0);
	vg.text_align(NVG_ALIGN_LEFT|NVG_ALIGN_TOP);
	vg.text_line_height(1.2);

	vg.text_box_bounds(x,y, 150, "Hover your mouse over the text to see calculated caret position.", NULL, bounds);

	// Fade the tooltip out when close to it.
	gx = fabsf((mx - (bounds[0]+bounds[2])*0.5) / (bounds[0] - bounds[2]));
	gy = fabsf((my - (bounds[1]+bounds[3])*0.5) / (bounds[1] - bounds[3]));
	a = maxf(gx, gy) - 0.5;
	a = clampf(a, 0, 1);
	vg.global_alpha(a);

	vg.begin_path();
	vg.fill_color(nvgRGBA(220,220,220,255));
	vg.rounded_rect(bounds[0]-2,bounds[1]-2, (int)(bounds[2]-bounds[0])+4, (int)(bounds[3]-bounds[1])+4, 3);
	px = (int)((bounds[2]+bounds[0])/2);
	vg.move_to(px,bounds[1] - 10);
	vg.line_to(px+7,bounds[1]+1);
	vg.line_to(px-7,bounds[1]+1);
	vg.fill();

	vg.fill_color(nvgRGBA(0,0,0,220));
	vg.text_box(x,y, 150, "Hover your mouse over the text to see calculated caret position.", NULL);

	vg.restore();
}

fn draw_widths(vg: *mut Struct_NVGcontext, x: c_float,
              y: c_float, width: c_float) {
	int i;

	vg.save();

	vg.stroke_color(nvgRGBA(0,0,0,255));

	for (i = 0; i < 20; i++) {
		float w = (i+0.5)*0.1;
		vg.stroke_width(w);
		vg.begin_path();
		vg.move_to(x,y);
		vg.line_to(x+width,y+width*0.3);
		vg.stroke();
		y += 10;
	}

	vg.restore();
}

fn draw_caps(vg: *mut Struct_NVGcontext, x: c_float,
            y: c_float, width: c_float) {
	int i;
	int caps[3] = {NVG_BUTT, NVG_ROUND, NVG_SQUARE};
	float lineWidth = 8.0;

	vg.save();

	vg.begin_path();
	vg.rect(x-lineWidth/2, y, width+lineWidth, 40);
	vg.fill_color(nvgRGBA(255,255,255,32));
	vg.fill();

	vg.begin_path();
	vg.rect(x, y, width, 40);
	vg.fill_color(nvgRGBA(255,255,255,32));
	vg.fill();

	vg.stroke_width(lineWidth);
	for (i = 0; i < 3; i++) {
		vg.line_cap(caps[i]);
		vg.stroke_color(nvgRGBA(0,0,0,255));
		vg.begin_path();
		vg.move_to(x, y + i*10 + 5);
		vg.line_to(x+width, y + i*10 + 5);
		vg.stroke();
	}

	vg.restore();
}



fn mini(int a, int b): int { return a < b ? a : b; }

fn unpremultiplyAlpha(unsigned char* image, int w, int h, int stride)
{
	int x,y;

	// Unpremultiply
	for (y = 0; y < h; y++) {
		unsigned char *row = &image[y*stride];
		for (x = 0; x < w; x++) {
			int r = row[0], g = row[1], b = row[2], a = row[3];
			if (a != 0) {
				row[0] = (int)mini(r*255/a, 255);
				row[1] = (int)mini(g*255/a, 255);
				row[2] = (int)mini(b*255/a, 255);
			}
			row += 4;
		}
	}

	// Defringe
	for (y = 0; y < h; y++) {
		unsigned char *row = &image[y*stride];
		for (x = 0; x < w; x++) {
			int r = 0, g = 0, b = 0, a = row[3], n = 0;
			if (a == 0) {
				if (x-1 > 0 && row[-1] != 0) {
					r += row[-4];
					g += row[-3];
					b += row[-2];
					n++;
				}
				if (x+1 < w && row[7] != 0) {
					r += row[4];
					g += row[5];
					b += row[6];
					n++;
				}
				if (y-1 > 0 && row[-stride+3] != 0) {
					r += row[-stride];
					g += row[-stride+1];
					b += row[-stride+2];
					n++;
				}
				if (y+1 < h && row[stride+3] != 0) {
					r += row[stride];
					g += row[stride+1];
					b += row[stride+2];
					n++;
				}
				if (n > 0) {
					row[0] = r/n;
					row[1] = g/n;
					row[2] = b/n;
				}
			}
			row += 4;
		}
	}
}

fn setAlpha(unsigned char* image, int w, int h, int stride, unsigned char a)
{
	int x, y;
	for (y = 0; y < h; y++) {
		unsigned char* row = &image[y*stride];
		for (x = 0; x < w; x++)
			row[x*4+3] = a;
	}
}

fn flipHorizontal(unsigned char* image, int w, int h, int stride)
{
	int i = 0, j = h-1, k;
	while (i < j) {
		unsigned char* ri = &image[i * stride];
		unsigned char* rj = &image[j * stride];
		for (k = 0; k < w*4; k++) {
			unsigned char t = ri[k];
			ri[k] = rj[k];
			rj[k] = t;
		}
		i++;
		j--;
	}
}


struct DemoData {
	int fontNormal, fontBold, fontIcons; 
	int images[12];
};


//int loadDemoData(struct NVGcontext* vg, struct DemoData* data);
pub fn loadDemoData(vg: *mut Struct_NVGcontext,
                    data: *mut Struct_DemoData) -> ::libc::c_int;

//void freeDemoData(struct NVGcontext* vg, struct DemoData* data);
pub fn freeDemoData(vg: *mut Struct_NVGcontext,
                    data: *mut Struct_DemoData);

//void renderDemo(struct NVGcontext* vg, float mx, float my, float width, float height, float t, int blowup, struct DemoData* data);
pub fn renderDemo(vg: *mut Struct_NVGcontext, mx: ::libc::c_float,
                  my: ::libc::c_float, width: ::libc::c_float,
                  height: ::libc::c_float, t: ::libc::c_float,
                  blowup: ::libc::c_int, data: *mut Struct_DemoData);

//void saveScreenShot(int w, int h, int premult, const char* name);
pub fn saveScreenShot(w: ::libc::c_int, h: ::libc::c_int,
                      premult: ::libc::c_int, name: *::libc::c_char);


fn isBlack(col: Struct_NVGcolor) -> ::libc::c_int;
fn drawWindow(vg: *mut Struct_NVGcontext, title: *::libc::c_char,
              x: ::libc::c_float, y: ::libc::c_float,
              w: ::libc::c_float, h: ::libc::c_float);
fn drawSearchBox(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
                 x: ::libc::c_float, y: ::libc::c_float,
                 w: ::libc::c_float, h: ::libc::c_float);
fn drawDropDown(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
                x: ::libc::c_float, y: ::libc::c_float,
                w: ::libc::c_float, h: ::libc::c_float);
fn drawLabel(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
             x: ::libc::c_float, y: ::libc::c_float,
             w: ::libc::c_float, h: ::libc::c_float);
fn drawEditBoxBase(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
                   y: ::libc::c_float, w: ::libc::c_float,
                   h: ::libc::c_float);
fn drawEditBox(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
               x: ::libc::c_float, y: ::libc::c_float,
               w: ::libc::c_float, h: ::libc::c_float);
fn drawEditBoxNum(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
                  units: *::libc::c_char, x: ::libc::c_float,
                  y: ::libc::c_float, w: ::libc::c_float,
                  h: ::libc::c_float);
fn drawCheckBox(vg: *mut Struct_NVGcontext, text: *::libc::c_char,
                x: ::libc::c_float, y: ::libc::c_float,
                w: ::libc::c_float, h: ::libc::c_float);
fn drawButton(vg: *mut Struct_NVGcontext, preicon: ::libc::c_int,
              text: *::libc::c_char, x: ::libc::c_float,
              y: ::libc::c_float, w: ::libc::c_float,
              h: ::libc::c_float, col: Struct_NVGcolor);
fn drawSlider(vg: *mut Struct_NVGcontext, pos: ::libc::c_float,
              x: ::libc::c_float, y: ::libc::c_float,
              w: ::libc::c_float, h: ::libc::c_float);
fn drawEyes(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
            y: ::libc::c_float, w: ::libc::c_float,
            h: ::libc::c_float, mx: ::libc::c_float,
            my: ::libc::c_float, t: ::libc::c_float);
fn drawGraph(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
             y: ::libc::c_float, w: ::libc::c_float,
             h: ::libc::c_float, t: ::libc::c_float);
fn drawSpinner(vg: *mut Struct_NVGcontext, cx: ::libc::c_float,
               cy: ::libc::c_float, r: ::libc::c_float,
               t: ::libc::c_float);
fn drawThumbnails(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
                  y: ::libc::c_float, w: ::libc::c_float,
                  h: ::libc::c_float, images: *::libc::c_int,
                  nimages: ::libc::c_int, t: ::libc::c_float);
fn drawColorwheel(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
                  y: ::libc::c_float, w: ::libc::c_float,
                  h: ::libc::c_float, t: ::libc::c_float);
fn drawLines(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
             y: ::libc::c_float, w: ::libc::c_float,
             h: ::libc::c_float, t: ::libc::c_float);
fn drawParagraph(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
                 y: ::libc::c_float, width: ::libc::c_float,
                 height: ::libc::c_float, mx: ::libc::c_float,
                 my: ::libc::c_float);
fn drawWidths(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
              y: ::libc::c_float, width: ::libc::c_float);
fn drawCaps(vg: *mut Struct_NVGcontext, x: ::libc::c_float,
            y: ::libc::c_float, width: ::libc::c_float);

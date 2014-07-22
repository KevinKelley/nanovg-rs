
pub fn render_demo(vg: &Ctx, mx: f32, my: f32, w: f32, h: f32, t: f32, blowup: bool, data: &DemoData)
{
	draw_eyes(vg, w - 250.0, 50.0, 150.0, 100.0, mx, my, t);
	draw_paragraph(vg, w - 450.0, 50.0, 150.0, 100.0, mx, my);
	draw_graph(vg, 0.0, h/2.0, w, h/2.0, t);
	draw_colorwheel(vg, w - 300.0, h - 300.0, 250.0, 250.0, t);

	// Line joints
	draw_lines(vg, 50.0, h-50.0, 600.0, 50.0, t);

	// Line caps
	draw_ws(vg, 10.0, 50.0, 30.0);

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
	draw_thumbnails(vg, 365.0, popy-30.0, 160.0, 300.0, data.images, 12, t);

	vg.restore();
}

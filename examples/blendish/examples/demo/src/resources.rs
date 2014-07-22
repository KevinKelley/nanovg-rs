

pub struct DemoData {
	//vg: &Ctx,
	fontNormal: i32,
	fontBold: i32,
	fontIcons: i32,
	images: [i32, ..12],
}

/// load and hold resources used in demo
impl DemoData
{
	pub fn load(vg: &Ctx) -> DemoData
	{
		let mut data = DemoData {
			//vg: vg,
			fontNormal: -1,
			fontBold:   -1,
			fontIcons:  -1,
			images: [-1, ..12]
		};

		for i in range(0, 12u) {
			let filename = format!("../res/images/image{}.jpg", i+1);
			data.images[i] = vg.create_image(filename.as_slice());
			if data.images[i] == 0 {
				println!("Could not load {}.", filename);
			}
		}

		data.fontIcons = vg.create_font("icons", "../res/entypo.ttf");
		if data.fontIcons == -1 {
			println!("Could not add font 'icons'.");
		}
		data.fontNormal = vg.create_font("sans", "../res/Roboto-Regular.ttf");
		if data.fontNormal == -1 {
			println!("Could not add font 'sans'.");
		}
		data.fontBold = vg.create_font("sans-bold", "../res/Roboto-Bold.ttf");
		if data.fontBold == -1 {
			println!("Could not add font 'sans-bold'.");
		}

		return data;
	}
}

impl Drop for DemoData {
	fn drop(&mut self) {
		for i in range(0, 12u) {
			// need to borrow & hold nanovg context, or
			// need to be able to pass it to 'drop'
//			self.vg.delete_image(self.images[i]);
			self.images[i] = -1;
		}
	}
}

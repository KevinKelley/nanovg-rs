use nanovg::*;
use std::ptr;

#[repr(i32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Style {
    FPS,
    MS
}

static CAP:uint = 100;

pub struct PerfGraph {
	pub style: Style,
	pub name: String,
	values: [f32, ..CAP],
	head: uint,
	count: uint,
}

impl PerfGraph
{
	//void initGraph(struct PerfGraph* fps, int style, const char* name);
	pub fn init(style: Style, name: &str) -> PerfGraph
	{
		PerfGraph {
			style: style,
			name: String::from_str(name),
			values: [0.0, ..CAP],
			head: 0,
			count: 0,
		}
	}

	//void updateGraph(struct PerfGraph* fps, float frameTime);
	pub fn update(&mut self, frameTime: f64)
	{
		//fps->head = (fps->head+1) % GRAPH_HISTORY_COUNT;
		//fps->values[fps->head] = frameTime;
		if self.count == CAP { self.head = (self.head + 1) % CAP }
		self.count = if self.count < CAP { self.count + 1 } else { CAP } ;
		self.values[((self.head+self.count) % CAP) as uint] = frameTime as f32;
	}

	//void renderGraph(struct NVGcontext* vg, float x, float y, struct PerfGraph* fps);
	pub fn render(&self, vg: &Ctx, x: f32, y: f32)
	{
		//int i;
		//float avg, w, h;
		//char str[64];

		let avg = self.getGraphAverage();

		let w = 200.0;
		let h = 35.0;

		vg.BeginPath();
		vg.Rect(x,y, w,h);
		vg.FillColor(RGBA(0,0,0,128));
		vg.Fill();

		vg.BeginPath();
		vg.MoveTo(x, y+h);
		if (self.style == FPS) {
			for i in range(0, CAP) { //(i = 0; i < CAP; i++) {
				let mut v = 1.0 / (0.00001 + self.values[(self.head+i) % CAP]);
				if v > 80.0 {v = 80.0;}
				let vx = x + (i as f32 / (CAP-1) as f32) * w;
				let vy = y + h - ((v / 80.0) * h);
				vg.LineTo(vx, vy);
			}
		} else {
			for i in range(0, CAP) {
				let mut v = self.values[(self.head+i) % CAP] * 1000.0;
				if v > 20.0 {v = 20.0;}
				let vx = x + (i as f32 / (CAP-1) as f32) * w;
				let vy = y + h - ((v / 20.0) * h);
				vg.LineTo(vx, vy);
			}
		}
		vg.LineTo(x+w, y+h);
		vg.FillColor(RGBA(255,192,0,128));
		vg.Fill();

		vg.FontFace("sans");

		if self.name.is_empty() {
			vg.FontSize(14.0);
			//vg.TextAlign(ALIGN_LEFT|ALIGN_TOP);
			vg.FillColor(RGBA(240,240,240,192));
			//vg.Text(x+3.0,y+1.0, &self.name);
		}

		if (self.style == FPS) {
			vg.FontSize(18.0);
			//vg.TextAlign(ALIGN_RIGHT|ALIGN_TOP);
			vg.FillColor(RGBA(240,240,240,255));
			let str = format!("{:3.2f} FPS", 1.0 / avg);
			//vg.Text(x+w-3.0,y+1.0, &str);

			vg.FontSize(15.0);
			//vg.TextAlign(ALIGN_RIGHT|ALIGN_BOTTOM);
			vg.FillColor(RGBA(240,240,240,160));
			let str = format!("{:3.2f} ms", avg * 1000.0);
			//vg.Text(x+w-3.0,y+h-1.0, &str);
		} else {
			vg.FontSize(18.0);
			//vg.TextAlign(ALIGN_RIGHT|ALIGN_TOP);
			vg.FillColor(RGBA(240,240,240,255));
			let str = format!("{:3.2f} ms", avg * 1000.0);
			//vg.Text(x+w-3.0,y+1.0, &str);
		}

	}

	//float getGraphAverage(struct PerfGraph* fps);
	pub fn getGraphAverage(&self) -> f32
	{
		let mut sum: f32 = 0.0;
		let mut i = self.head;
		while i < self.head + self.count {
			let ix = i % CAP;
			sum += self.values[ix as uint];
			i = i+1;
		}
		sum / self.count as f32
	}
}



//#define GPU_QUERY_COUNT 5
//struct GPUtimer {
//	int supported;
//	int cur, ret;
//	unsigned int queries[GPU_QUERY_COUNT];
//};

//void initGPUTimer(struct GPUtimer* timer);
//pub fn initGPUTimer(timer: *mut Struct_GPUtimer);

//void startGPUTimer(struct GPUtimer* timer);
//pub fn startGPUTimer(timer: *mut Struct_GPUtimer);

//int stopGPUTimer(struct GPUtimer* timer, float* times, int maxTimes);
//pub fn stopGPUTimer(timer: *mut Struct_GPUtimer,
//                    times: *mut ::libc::c_float,
//                    maxTimes: ::libc::c_int) -> ::libc::c_int;

use nanovg::NVGcontext;

pub enum Style {
    FPS,
    MS
}

static CAP:int = 100;

pub struct PerfGraph {
	pub style: Style,
	pub name: String,
	values: [f64, ..CAP],
	head: int,
	count: int,
}

impl PerfGraph
{
	//void initGraph(struct PerfGraph* fps, int style, const char* name);
	pub fn init(style: Style, name: &str) -> PerfGraph
	{
		PerfGraph {
			style: style,
			name: String::from_str(name),
			values: [0.0, ..CAP as uint],
			head: 0,
			count: 0,
		}
	}

	//void updateGraph(struct PerfGraph* fps, float frameTime);
	pub fn update(&mut self, frameTime: f64)
	{
		if self.count == CAP { self.head = (self.head + 1) % CAP }
		self.count = if self.count < CAP { self.count + 1 } else { CAP } ;
		self.values[((self.head+self.count) % CAP) as uint] = frameTime;
	}

	//void renderGraph(struct NVGcontext* vg, float x, float y, struct PerfGraph* fps);
	pub fn render(&self, vg: *mut NVGcontext, x: f64, y: f64)
	{}
	//float getGraphAverage(struct PerfGraph* fps);
	pub fn getGraphAverage(&self) -> f64
	{
		let mut sum: f64 = 0.0;
		let mut i = self.head;
		while i < self.head + self.count {
			let ix: uint = (i % CAP) as uint;
			sum += self.values[ix];
			i = i+1;
		}
		sum / self.count as f64
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

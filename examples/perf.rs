
use nanovg::NVGcontext;

pub enum Style {
    FPS,
    MS
}

static GRAPH_HISTORY_COUNT:int = 100;

pub struct PerfGraph {
	style: Style,
	name: String,
	values: [f64, ..GRAPH_HISTORY_COUNT], 
	head: int,
}


//void initGraph(struct PerfGraph* fps, int style, const char* name);
pub fn initGraph(fps: &PerfGraph, style: Style,
                 name: *::libc::c_char)
{}

//void updateGraph(struct PerfGraph* fps, float frameTime);
pub fn updateGraph(fps: &PerfGraph, frameTime: f64)
{}

//void renderGraph(struct NVGcontext* vg, float x, float y, struct PerfGraph* fps);
pub fn renderGraph(vg: &NVGcontext, x: ::libc::c_float,
                   y: ::libc::c_float, fps: &PerfGraph)
{}

//float getGraphAverage(struct PerfGraph* fps);
pub fn getGraphAverage(fps: &PerfGraph) -> ::libc::c_float
{ 0.0 }



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

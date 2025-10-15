#[cfg(test)]
use crate::{algorithm::{Context, ContextWrite, EvalResult, InitialCtx}, domain::{Eval, Image}};
use std::{sync::Once, time::Duration};
use opencv::imgcodecs;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        ContextRead, FastEdgesConf, FastEdges, FastContours, FastContoursConf, FastScanConf, FastScanCtx,
        GeometryDefect, GeometryDefectCtx, Mad, ResultCtx, RopeDimensionsConf, TemporalFilterConf, Threshold
    }, 
};
///
///
static INIT: Once = Once::new();
///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
        // implement your initialisation code to be called only once for current test file
    })
}
///
/// returns:
///  - ...
fn init_each() -> () {}
///
/// Testing 'eval'
#[test]
fn eval() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("eval");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let test_data = [
        (
            1,
            "src/test/unit/algorithm/detecting_contours/testing_files/rope_0.jpeg",
            // "src/test/complex/testing_files/rope_0.jpeg",
            vec![]
        )
    ];
    let conf = FastScanConf {
        fast_contours: FastContoursConf::default(),
        temporal_filter: TemporalFilterConf::default(),
        fast_edges: FastEdgesConf::default(),
        rope_dimensions: RopeDimensionsConf::default(),
        geometry_defect_threshold: Threshold(1.1),
    };
    let geometry_defect = GeometryDefect::<FastScanCtx>::new(
        conf.geometry_defect_threshold,
        *Box::new(Mad::new()),
        FastEdges::new(
            conf.fast_edges.otsu_tune,
            conf.fast_edges.threshold,
            conf.fast_edges.smooth,
            FastContours::new(
                conf.fast_contours,
                FakePassImg::new(),
                false,
            ),
        ),
    );
    for (step, testing_frame, target) in test_data {
        let frame_mat = imgcodecs::imread(
            testing_frame,
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        let src_frame = Image::with(frame_mat);
        let result = geometry_defect.eval(src_frame);
        match result {
            Ok(result) => {
                let result = ContextRead::<GeometryDefectCtx<FastScanCtx>>::read(&result)
                    .result.clone();
                assert!(
                    result == target, 
                    "step {} \nresult: {:?}\ntarget: {:?}", 
                    step, 
                    result, 
                    target
                );
            },
            Err(err) => panic!("step {} \nerror: {:#?}", step, err),
        }
    }
    test_duration.exit();
}
///
/// Fake implements `Eval` for testing [FastEdges]
struct FakePassImg {}
impl FakePassImg{
    pub fn new() -> Self {
        Self {}
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassImg {
    fn eval(&self, val: Image) -> EvalResult {
        let ctx = Context::new(
            InitialCtx::new()
        );
        ctx.write(ResultCtx { val })
    }
}

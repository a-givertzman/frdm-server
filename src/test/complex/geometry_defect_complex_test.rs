#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrastCtx, Context, ContextWrite, EvalResult, InitialCtx}, domain::{Eval, Image}};
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
        ContextRead, 
        DetectingContoursCv, 
        EdgeDetection, 
        GeometryDefect, 
        GeometryDefectCtx, 
        Mad, 
        Threshold
    }, 
    conf::{
        Conf, DetectingContoursConf, EdgeDetectionConf, FastScanConf, FineScanConf
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
            "src/test/complex/testing_files/rope_0.jpeg",
            vec![]
        )
    ];
    let conf = Conf {
        contours: DetectingContoursConf::default(),
        edge_detection: EdgeDetectionConf::default(),
        fast_scan: FastScanConf {
            geometry_defect_threshold: Threshold::min(),
        },
        fine_scan: FineScanConf {},
    };
    let geometry_defect = GeometryDefect::new(
        conf.fast_scan.geometry_defect_threshold,
        *Box::new(Mad::new()),
        EdgeDetection::new(
            conf.edge_detection.otsu_tune,
            conf.edge_detection.threshold,
            DetectingContoursCv::new(
                conf.contours,
                FakePassImg::new(),
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
                let result = ContextRead::<GeometryDefectCtx>::read(&result)
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
/// Fake implements `Eval` for testing [EdgeDetection]
struct FakePassImg {}
impl FakePassImg{
    pub fn new() -> Self {
        Self {}
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassImg {
    fn eval(&self, frame: Image) -> EvalResult {
        let ctx = Context::new(
            InitialCtx::new()
        );
        ctx.write(AutoBrightnessAndContrastCtx { result: frame })
    }
}

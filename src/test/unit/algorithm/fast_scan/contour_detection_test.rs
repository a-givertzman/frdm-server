#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrast, AutoBrightnessAndContrastCtx, AutoGamma, AutoGammaCtx, Context, ContextWrite, DetectingContoursCvCtx, EdgeDetectionCtx, EvalResult, Initial, InitialCtx, Side}, domain::{Eval, Image}};
use std::{sync::Once, time::Duration};
use opencv::{core::{self, Mat, MatTrait, Vec3b, ROTATE_90_CLOCKWISE}, highgui, imgcodecs};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use walkdir::WalkDir;
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
    let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
    test_duration.run().unwrap();
    let test_data = [
        (
            1,
            "/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp95_rope/retrived/exp95_rope_frame_5.jpeg",
        )
    ];
    let conf = Conf {
        detecting_contours: DetectingContoursConf::default(),
        edge_detection: EdgeDetectionConf::default(),
        fast_scan: FastScanConf {
            geometry_defect_threshold: Threshold::min(),
        },
        fine_scan: FineScanConf {},
    };
    let scan_rope = 
        EdgeDetection::new(
            conf.edge_detection.threshold,
            DetectingContoursCv::new(
                conf.detecting_contours.clone(),
                AutoBrightnessAndContrast::new(
                    conf.detecting_contours.brightness_contrast.histogram_clipping,
                    AutoGamma::new(
                        Initial::new(
                            InitialCtx::new(),
                        ),
                    ),
                ),
            )
        );
    // let winp = "Inp";
    let wgamma = "Gamma";
    let wbright = "Bright";
    let wcontours = "Contours";
    let wedges = "Edges";
    // if let Err(err) = opencv::highgui::named_window(winp, opencv::highgui::WINDOW_NORMAL) {
    //     log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    // }
    if let Err(err) = opencv::highgui::named_window(wgamma, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wbright, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wcontours, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wedges, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }

    let image_dir = "/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp95_rope/retrived/"; 

    for entry in WalkDir::new(image_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        match path.extension() {
            Some(ext) if ext == "jpg" || ext == "png" || ext == "jpeg" => {
                let mut frame_mat = imgcodecs::imread(
                    path.to_str().unwrap(),
                imgcodecs::IMREAD_COLOR,
                ).unwrap();
                let inp = frame_mat.clone();
                let mut rotated = Mat::default();
                core::rotate(&inp, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                let mut edges = rotated.clone();
                let src_frame = Image::with(rotated);
                let ctx = scan_rope.eval(src_frame).unwrap();
                let gamma: &AutoGammaCtx = ctx.read();
                let bright: &AutoBrightnessAndContrastCtx = ctx.read();
                let contours: &DetectingContoursCvCtx = ctx.read();
                let Edges: &EdgeDetectionCtx = ctx.read();
                let mut edges_cont = contours.result.mat.clone();
                let upper = Edges.result.get(Side::Upper);
                let lower = Edges.result.get(Side::Lower);
                for dot in upper {
                    if dot.x >= 0 && dot.y >= 0 {
                        let x = dot.x as i32;
                        let y = dot.y as i32;
                        *edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
                    }
                }
                for dot in lower {
                    if dot.x >= 0 && dot.y >= 0 {
                        let x = dot.x as i32;
                        let y = dot.y as i32;
                        *edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
                    }
                }
                // let mut transposed = Mat::default();
                // core::transpose(&inp, &mut transposed).unwrap();


                // highgui::imshow(winp, &rotated).unwrap();
                highgui::imshow(wgamma, &gamma.result.mat).unwrap();
                highgui::imshow(wbright, &bright.result.mat).unwrap();
                highgui::imshow(wcontours, &contours.result.mat).unwrap();
                highgui::imshow(wedges, &edges).unwrap();
                // highgui::imshow(winp, &inp).unwrap();
                highgui::wait_key(0).unwrap();
                highgui::destroy_all_windows().unwrap();
            },
            _ => continue,
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
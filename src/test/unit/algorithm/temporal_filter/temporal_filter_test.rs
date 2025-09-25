#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrast, AutoBrightnessAndContrastCtx, AutoGamma, AutoGammaCtx, Context, ContextWrite, DetectingContoursCvCtx, EdgeDetectionCtx, EvalResult, Initial, InitialCtx, Side}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use opencv::{core::{self, Mat, MatTrait, Vec3b, ROTATE_90_CLOCKWISE}, highgui, imgcodecs, imgproc};
use sal_sync::services::conf::ConfTree;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        ContextRead, Cropping, CroppingCtx, DetectingContoursCv, EdgeDetection, TemporalFilter
    }, 
    conf::Conf,
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
/// Testing 'TemporalFilter.eval'
#[test]
fn eval() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("TemporalFilter-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1000));
    test_duration.run().unwrap();
    let conf = ConfTree::new_root(
        serde_yaml::from_str(&format!(r#"
            contours:
                cropping:
                    x: 250           # new left edge
                    width: 964     # new image width
                    y: 50           # new top edge
                    height: 1100    # new image height
                gamma:
                    factor: 99.0              # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
                brightness-contrast:
                    hist-clip-left: 1.5     # optional histogram clipping from right, default = 0.0 %
                    hist-clip-right: 1.5    # optional histogram clipping from right, default = 0.0 %
                temporal-filter:
                    amplify_factor: 1.0     # factor amplifies the highlighting the oftenly changing pixels
                    reduce_factor: 1.0      # factor amplifies the hiding the lower changing pixels
                    threshold: 1.0
                gausian:
                    blur-size:
                        width: 7
                        height: 7
                    sigma-x: 0.0
                    sigma-y: 0.0
                sobel:
                    kernel-size: 3
                    scale: 1.0
                    delta: 0.0
                overlay:
                    src1-weight: 0.5
                    src2-weight: 0.5
                    gamma: 0.0
            edge-detection:
                otsu-tune: 1.0      # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
                threshold: 50       # 0...255, used if otsu-tune is not specified
            fast-scan:
                geometry-defect-threshold: 1.2      # 1.1..1.3, absolute threshold to detect the geometry deffects
            fine-scan:
                no-params: not implemented yet
        "#)).unwrap(),
    );
    let conf = Conf::new(&dbg, conf);
    // let cropp = Cropping::new(100, 1000, 100, 1000, Initial::new(InitialCtx::new()));
    let temporal_filter = 
        // EdgeDetection::new(
        //     conf.edge_detection.otsu_tune,
        //     conf.edge_detection.threshold,
        //     DetectingContoursCv::new(
        //         conf.contours.clone(),
                TemporalFilter::new(
                    conf.contours.temporal_filter.amplify_factor,
                    conf.contours.temporal_filter.reduce_factor,
                    conf.contours.temporal_filter.threshold,
                    AutoBrightnessAndContrast::new(
                        conf.contours.brightness_contrast.hist_clip_left,
                        conf.contours.brightness_contrast.hist_clip_right,
                        AutoGamma::new(
                            conf.contours.gamma.factor,
                            Cropping::new(
                                conf.contours.cropping.x,
                                conf.contours.cropping.width,
                                conf.contours.cropping.y,
                                conf.contours.cropping.height,
                                Initial::new(
                                    InitialCtx::new(),
                                ),
                            ),
                        ),
                    ),
                );
        //     ),
        // );
    let wcropped = "Cropped";
    let wgamma = "Gamma";
    let wbright = "Brightness & Contrast";
    let w_temp_filter = "Temporal Filter";
    if let Err(err) = opencv::highgui::named_window(wgamma, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wbright, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(w_temp_filter, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }
    if let Err(err) = opencv::highgui::named_window(wcropped, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", "dbg", err);
    }

    let image_dir = "src/test/unit/algorithm/detecting_contours/testing_files";
    // "/home/ilyarizo/deffect_photos/rope_rotated/gap_pit/exp95/retrived"; 

    for path in std::fs::read_dir(image_dir).unwrap().into_iter()
        .filter_map(|e| {
            let path = e.unwrap().path();
            path.is_file().then(|| path)
        })
    {
        match path.extension() {
            Some(ext) if ext == "jpg" || ext == "png" || ext == "jpeg" => {
                let frame = Image::load(path.to_str().unwrap()).unwrap();
                let mut rotated = Mat::default();
                core::rotate(&frame.mat, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                let src = Image::with(rotated);
                log::debug!("{dbg}.eval | src frame: {} x {}", src.width, src.height);
                // let test = src.clone();
                let t = Instant::now();
                let ctx = temporal_filter.eval(src).unwrap();
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let crop: &CroppingCtx = ctx.read();    
                let gamma: &AutoGammaCtx = ctx.read();
                let bright: &AutoBrightnessAndContrastCtx = ctx.read();
                let contours: &DetectingContoursCvCtx = ctx.read();
                // let edges: &EdgeDetectionCtx = ctx.read();
                // let mut res = crop.result.mat.clone();
                // let edges_cont = contours.result.mat.clone();
                // let upper = edges.result.get(Side::Upper);
                // let lower = edges.result.get(Side::Lower);
                // for dot in upper {
                //     if dot.x >= 0 && dot.y >= 0 {
                //         let x = dot.x as i32;
                //         let y = dot.y as i32;
                //         *res.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
                //     }
                // }
                // for dot in lower {
                //     if dot.x >= 0 && dot.y >= 0 {
                //         let x = dot.x as i32;
                //         let y = dot.y as i32;
                //         *res.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
                //     }
                // }
                highgui::imshow(wcropped, &crop.result.mat).unwrap();
                highgui::imshow(wgamma, &gamma.result.mat).unwrap();
                highgui::imshow(wbright, &bright.result.mat).unwrap();
                highgui::imshow(w_temp_filter, &contours.result.mat).unwrap();
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
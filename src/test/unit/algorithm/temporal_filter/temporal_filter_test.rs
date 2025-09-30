#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrastCtx, AutoGammaCtx, Context, ContextWrite, EvalResult, Initial, InitialCtx}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use opencv::{core::{self, Mat, MatTrait, MatTraitConst, Vec3b, ROTATE_90_CLOCKWISE}, highgui};
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
        AutoBrightnessAndContrast, AutoGamma, ContextRead, Cropping, CroppingCtx, DetectingContoursCv, EdgeDetection, EdgeDetectionCtx, Gray, GrayCtx, ResultCtx, Side, TemporalFilter
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
                    x: 230           # new left edge
                    width: 1410     # new image width
                    y: 300           # new top edge
                    height: 1000    # new image height
                gamma:
                    factor: 100.0              # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
                brightness-contrast:
                    hist-clip-left: 97.0     # optional histogram clipping from right, default = 0.0 %
                    hist-clip-right: 0.0    # optional histogram clipping from right, default = 0.0 %
                temporal-filter:
                    amplify-factor: 12.0     # factor amplifies the highlighting the oftenly changing pixels
                    grow-speed: 0.02          # speed of `rate` growing for changed pixels, 1 - default speed, depends on pixel change value
                    reduce-factor: 72.0      # factor amplifies the hiding the lower changing pixels
                    down-speed: 2.4          # speed of `rate` reducing for static pixels, 1 - default speed, depends on pixel change value
                    threshold: 64.0
                gausian:
                    blur-size:
                        width: 11
                        height: 3
                    sigma-x: 0.0
                    sigma-y: 0.0
                sobel:
                    kernel-size: 1
                    scale: 5.0
                    delta: 0.0
                overlay:
                    src1-weight: 1.0
                    src2-weight: 1.0
                    gamma: 0.0
            edge-detection:
                otsu-tune: 1.40       # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
                # threshold: 50       # 0...255, used if otsu-tune is not specified
                smooth: 8             # Smoothing of edge line factor. The higher the factor the smoother the line.
            fast-scan:
                geometry-defect-threshold: 1.0      # 1.1..1.3, absolute threshold to detect the geometry deffects
            fine-scan:
                no-params: not implemented yet
        "#)).unwrap(),
    );
    let conf = Conf::new(&dbg, conf);
    // let cropp = Cropping::new(100, 1000, 100, 1000, Initial::new(InitialCtx::new()));
    let temporal_filter = 
        EdgeDetection::new(
            conf.edge_detection.otsu_tune,
            conf.edge_detection.threshold,
            conf.edge_detection.smooth,
            DetectingContoursCv::new(
                conf.contours.clone(),
                TemporalFilter::new(
                    conf.contours.temporal_filter.amplify_factor,
                    conf.contours.temporal_filter.grow_speed,
                    conf.contours.temporal_filter.reduce_factor,
                    conf.contours.temporal_filter.down_speed,
                    conf.contours.temporal_filter.threshold,
                    Gray::new(
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
                    ),
                ),
            ),
        );
    let wgray = "Gray";
    let wcrop = "Cropped";
    let wgamma = "Gamma";
    let wbright = "Brightness & Contrast";
    let w_temp_filter = "Temporal Filter";
    if let Err(err) = opencv::highgui::named_window(wgray, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }
    if let Err(err) = opencv::highgui::named_window(wgamma, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }
    if let Err(err) = opencv::highgui::named_window(wbright, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }
    if let Err(err) = opencv::highgui::named_window(w_temp_filter, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }
    if let Err(err) = opencv::highgui::named_window(wcrop, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }

    let image_dir = "src/test/unit/algorithm/temporal_filter/frames";
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
                // let mut rotated = Mat::default();
                // core::rotate(&frame.mat, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                // let src = Image::with(rotated);
                log::debug!("{dbg}.eval | src frame: {} x {}", frame.width, frame.height);
                // let test = src.clone();
                let t = Instant::now();
                let ctx = temporal_filter.eval(frame).unwrap();
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let gray: &GrayCtx = ctx.read();    
                let crop: &CroppingCtx = ctx.read();    
                let mut crop = crop.result.mat.clone();
                let gamma: &AutoGammaCtx = ctx.read();
                let bright: &AutoBrightnessAndContrastCtx = ctx.read();
                let result: &ResultCtx = ctx.read();
                let edges: &EdgeDetectionCtx = ctx.read();
                // let mut res = crop.result.mat.clone();
                // let edges_cont = contours.result.mat.clone();
                let upper = edges.result.get(Side::Upper);
                let lower = edges.result.get(Side::Lower);
                log::trace!("{dbg}.eval | upper: {:?}", upper);
                log::trace!("{dbg}.eval | lower: {:?}", lower);
                for dot in upper {
                    *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 0, 255]);
                }
                for dot in lower {
                    *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 255, 0]);
                }
                if !gray.frame.mat.empty() { highgui::imshow(wgray, &gray.frame.mat).unwrap() };
                if !gamma.result.mat.empty() { highgui::imshow(wgamma, &gamma.result.mat).unwrap() };
                if !bright.result.mat.empty() { highgui::imshow(wbright, &bright.result.mat).unwrap() };
                if !crop.empty() { highgui::imshow(wcrop, &crop).unwrap() };
                if !result.frame.mat.empty() { highgui::imshow(w_temp_filter, &result.frame.mat).unwrap() };
                highgui::wait_key(0).unwrap();
            },
            _ => continue,
        }
    }
    highgui::destroy_all_windows().unwrap();
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
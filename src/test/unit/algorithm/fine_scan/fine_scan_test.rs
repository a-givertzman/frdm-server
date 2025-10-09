#[cfg(test)]
use crate::{algorithm::{AutoGammaCtx, Initial, InitialCtx}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use opencv::{core::{MatTrait, MatTraitConst, Point2i, Vec3b, VecN}, highgui};
use sal_sync::{services::conf::ConfTree, thread_pool::ThreadPool};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        AutoGamma, Context, ContextRead, ContextWrite, Cropping, CroppingCtx, CvContoursCtx, EdgeDetection, EdgeDetectionCtx, EvalResult, FineContours, FineUnion, FineUnionCtx, GaussianBlur, Gray, GrayCtx, RopeDimensions, RopeDimensionsCtx, Side, TemporalFilter, TemporalFilterCtx
    }, 
    conf::Conf, domain::Error,
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
    let dbg = Dbg::own("FineScan-test");
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
                    factor: 120.0              # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
                brightness-contrast:
                    hist-clip-left: 97.0     # optional histogram clipping from right, default = 0.0 %
                    hist-clip-right: 0.0    # optional histogram clipping from right, default = 0.0 %
                temporal-filter:
                    amplify-factor: 12.0     # factor amplifies the highlighting the oftenly changing pixels
                    grow-speed: 2.6          # speed of `rate` growing for changed pixels, 1 - default speed, depends on pixel change value
                    reduce-factor: 72.0      # factor amplifies the hiding the lower changing pixels
                    down-speed: 2.8          # speed of `rate` reducing for static pixels, 1 - default speed, depends on pixel change value
                    threshold: 12.0
                gausian:
                    blur-size:
                        width: 11
                        height: 11
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
                # otsu-tune: 0.90       # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
                threshold: 128       # 0...255, used if otsu-tune is not specified
                smooth: 36             # Smoothing of edge line factor. The higher the factor the smoother the line.
            rope-dimensions:
                rope-width: 380               # Standart rope width, px
                width-tolerance: 25.0         # Tolerance for rope width, %
                square-tolerance: 100.0       # Tolerance for rope square, %
            fast-scan:
                geometry-defect-threshold: 1.0      # 1.1..1.3, absolute threshold to detect the geometry deffects
            fine-scan:
                no-params: not implemented yet
        "#)).unwrap(),
    );
    let conf = Conf::new(&dbg, conf);
    // let cropp = Cropping::new(100, 1000, 100, 1000, Initial::new(InitialCtx::new()));
    let debug = false;
    let tp = ThreadPool::new(&dbg, Some(4));
    let temporal_filter = 
        EdgeDetection::new(
            conf.edge_detection.otsu_tune,
            conf.edge_detection.threshold,
            conf.edge_detection.smooth,
            FineUnion::new(
                tp.scheduler(),
                TemporalFilter::new(
                    conf.cv_contours.temporal_filter.amplify_factor,
                    conf.cv_contours.temporal_filter.grow_speed,
                    conf.cv_contours.temporal_filter.reduce_factor,
                    conf.cv_contours.temporal_filter.down_speed,
                    conf.cv_contours.temporal_filter.threshold,
                        Gray::new(
                            AutoGamma::new(
                                conf.cv_contours.gamma.factor,
                                Cropping::new(
                                    conf.cv_contours.cropping.x,
                                    conf.cv_contours.cropping.width,
                                    conf.cv_contours.cropping.y,
                                    conf.cv_contours.cropping.height,
                                    Initial::new(
                                        InitialCtx::new(),
                                    ),
                                    debug,
                                ),
                                debug,
                            ),
                        debug,
                    ),
                    debug,
                ),
                FineContours::new(
                    conf.cv_contours.clone(),
                    GaussianBlur::new(
                        conf.cv_contours.gausian.blur_w,
                        conf.cv_contours.gausian.blur_h,
                        conf.cv_contours.gausian.sigma_x,
                        conf.cv_contours.gausian.sigma_y,
                        Gray::new(
                            AutoGamma::new(
                                conf.cv_contours.gamma.factor,
                                Cropping::new(
                                    conf.cv_contours.cropping.x,
                                    conf.cv_contours.cropping.width,
                                    conf.cv_contours.cropping.y,
                                    conf.cv_contours.cropping.height,
                                    Initial::new(
                                        InitialCtx::new(),
                                    ),
                                    debug,
                                ),
                                debug,
                            ),
                            debug,
                        ),
                        debug,
                    ),
                    debug,
                ),
            ),
        );
    let w_gray = "Gray";
    let w_crop = "Cropped";
    let w_gamma = "Gamma";
    let w_contours = "Contours";
    let w_union = "Union";
    let w_temp_filter = "Temporal Filter";
    for window in [w_gray, w_crop, w_gamma, w_contours, w_union, w_temp_filter] {
        if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
            log::warn!("{dbg} | Create Window Error: {}", err);
        }
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
                let ctx = temporal_filter.eval(frame.clone()).unwrap();
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let gray: &GrayCtx = ctx.read();    
                let crop: &CroppingCtx = ctx.read();    
                let mut crop = crop.result.mat.clone();
                let gamma: &AutoGammaCtx = ctx.read();
                let contours: &CvContoursCtx = ctx.read();
                let union: &FineUnionCtx = ctx.read();
                let edges: &EdgeDetectionCtx = ctx.read();
                let temp_filter: &TemporalFilterCtx = ctx.read();
                // let mut res = crop.result.mat.clone();
                // let edges_cont = contours.result.mat.clone();
                let upper = edges.result.get(Side::Upper);
                let lower = edges.result.get(Side::Lower);
                log::trace!("{dbg}.eval | upper: {:?}", upper);
                log::trace!("{dbg}.eval | lower: {:?}", lower);
                for dot in &upper {
                    *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 0, 255]);
                }
                for dot in &lower {
                    *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 255, 0]);
                }
                let (text, text_color) = match RopeDimensions::new(
                    conf.rope_dimensions.rope_width,
                    conf.rope_dimensions.width_tolerance,
                    conf.rope_dimensions.square_tolerance,
                    FakePassDots::new(edges.clone()),
                ).eval(frame.clone()) {
                    Ok(ctx) => {
                        let dimensions: &RopeDimensionsCtx = ctx.read();
                        let width_error = (100.0 - dimensions.width * 100.0 / conf.rope_dimensions.rope_width as f64).abs();
                        let square_error = (100.0 - dimensions.square * 100.0 / (conf.rope_dimensions.rope_width * upper.len()) as f64).abs();
                        (format!("Rope width: {:.3} ({:.2}%), square: {} ({:.2}%)", dimensions.width, width_error, dimensions.square, square_error), VecN::from_array([255.0, 0.0, 0.0, 0.0]))
                    }
                    Err(err) => (format!("Error: {:?}", err), VecN::from_array([0.0, 0.0, 255.0, 0.0]))
                };
                opencv::imgproc::put_text(&mut crop, &text, Point2i::new(10, 30), 1, 2.0, text_color, 2, -1, false).unwrap();
                if !gray.frame.mat.empty() { highgui::imshow(w_gray, &gray.frame.mat).unwrap() };
                if !gamma.result.mat.empty() { highgui::imshow(w_gamma, &gamma.result.mat).unwrap() };
                if !contours.result.mat.empty() { highgui::imshow(w_contours, &contours.result.mat).unwrap() };
                if !union.frame.mat.empty() { highgui::imshow(w_union, &union.frame.mat).unwrap() };
                if !crop.empty() { highgui::imshow(w_crop, &crop).unwrap() };
                if !temp_filter.frame.mat.empty() { highgui::imshow(w_temp_filter, &temp_filter.frame.mat).unwrap() };
                highgui::wait_key(0).unwrap();
            },
            _ => continue,
        }
    }
    highgui::destroy_all_windows().unwrap();
    test_duration.exit();
}
///
/// Fake implements `Eval` for testing [RopeDimensions]
struct FakePassDots {
    dots: EdgeDetectionCtx,
}
impl FakePassDots{
    pub fn new(dots: EdgeDetectionCtx) -> Self {
        Self { dots }
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassDots {
    fn eval(&self, _: Image) -> Result<Context, Error> {
        let ctx = Context::new(
            InitialCtx::new(),
        );
        ctx.write(self.dots.clone())
    }
}

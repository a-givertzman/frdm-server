#[cfg(test)]
use crate::{algorithm::{AutoGammaCtx, InitialCtx}, domain::{Eval, Image}};
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
        AutoGamma, Context, ContextRead, ContextWrite, Cropping, CroppingCtx, EvalResult, FastEdgesCtx, FastScan, FastScanConf, FastScanCtx, FastUnionCtx, Gray, GrayCtx, Initial, RopeDimensions, RopeDimensionsCtx, Side
    }, 
    domain::Error,
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
    let dbg = Dbg::own("FastScan-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1000));
    test_duration.run().unwrap();
    let conf = ConfTree::new_root(
        serde_yaml::from_str(&format!(r#"
            add-weighted:
                weight1: 1.0            # Weight of the first array elements.
                weight2: 1.0            # Weight of the second array elements.
                gamma: 0.0
            fast-contours:
                cropping:
                    x: 230              # New left edge
                    y: 300              # New top edge
                    width: 1410         # New image width
                    height: 1000        # New image height
                gamma:
                    factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
                otsu-tune: 0.40
            temporal-filter:
                gaussian:
                    kernel: [11, 11]    # Gausian blur kernel size, must be odd
                    sigma: [0.0, 0.0]   # Standard deviation in [X, Y] direction, The higher the value, the more pixels are used to count each pixel and the smoother blur will be
                open-kernel: [3, 3]     # Morphology open operation kernel size [w, h], default [5, 5]
                erode-kernel: [3, 3]    # Morphology erode operation kernel size [w, h], default [5, 5]
                threshold: 12.0         # Threshold to detect the pixel whas changed or not in the each next frame
            fast-edges:
                otsu-tune: 1.40         # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
                # threshold: 128        # 0...255, used if otsu-tune is not specified
                smooth: 36              # Smoothing of edge line factor. The higher the factor the smoother the line.
            rope-dimensions:        # Verifaing the rope dimensions 
                rope-width: 380               # Standart rope width, px
                width-tolerance: 25.0         # Tolerance for rope width, %
                square-tolerance: 100.0       # Tolerance for rope square, %
            geometry-defect-threshold: 1.0    # 1.1..1.3, absolute threshold to detect the geometry deffects
        "#)).unwrap(),
    );
    let conf = FastScanConf::new(&dbg, conf);
    let tp = ThreadPool::new(&dbg, Some(4));
    let fast_scan = FastScan::new(
        conf.clone(),
        tp.scheduler(),
        Gray::new(
            AutoGamma::new(
                120.0,
                Cropping::new(
                    230,
                    1410,
                    300,
                    1000,
                    Initial::new(
                        InitialCtx::new(),
                    ),
                    true,
                ),
                true,
            ),
            true
        ),
        false,
    );
    let w_gray = "Gray";
    let w_crop = "Cropped";
    let w_gamma = "Auto Gamma";
    let w_contours = "Contours";
    for window in [w_gray, w_crop, w_gamma, w_contours] {
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
                log::debug!("{dbg}.eval | src frame: {} x {}", frame.width(), frame.height());
                // let test = src.clone();
                let t = Instant::now();
                let ctx = fast_scan.eval(frame.clone()).unwrap();
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let gray: &GrayCtx = ctx.read();    
                let crop: &CroppingCtx = ctx.read();    
                let gamma: &AutoGammaCtx = ctx.read();
                let contours: &FastUnionCtx = ctx.read();
                let mut crop = if crop.result.mat.empty() {
                    let mut dst = opencv::core::Mat::default();
                    opencv::imgproc::cvt_color(&contours.frame.mat, &mut dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
                    dst
                } else {
                    crop.result.mat.clone()
                };
                let edges: &FastEdgesCtx = ctx.read();
                let upper = edges.edges.get(Side::Upper);
                let lower = edges.edges.get(Side::Lower);
                log::trace!("{dbg}.eval | upper: {:?}", upper);
                log::trace!("{dbg}.eval | lower: {:?}", lower);
                if !crop.empty() {
                    for dot in &upper {
                        *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 0, 255]);
                    }
                    for dot in &lower {
                        *crop.at_2d_mut::<Vec3b>(dot.y as i32, dot.x as i32).unwrap() = Vec3b::from_array([0, 255, 0]);
                    }
                }
                let (text, text_color) = match RopeDimensions::<FastScanCtx>::new(
                    conf.rope_dimensions.rope_width,
                    conf.rope_dimensions.width_tolerance,
                    conf.rope_dimensions.square_tolerance,
                    FakePassDots::new(edges.clone()),
                ).eval(frame.clone()) {
                    Ok(ctx) => {
                        let dimensions: &RopeDimensionsCtx<FastScanCtx> = ctx.read();
                        let width_error = (100.0 - dimensions.width * 100.0 / conf.rope_dimensions.rope_width as f64).abs();
                        let square_error = (100.0 - dimensions.square * 100.0 / (conf.rope_dimensions.rope_width * upper.len()) as f64).abs();
                        (format!("Rope width: {:.3} ({:.2}%), square: {} ({:.2}%)", dimensions.width, width_error, dimensions.square, square_error), VecN::from_array([255.0, 0.0, 0.0, 0.0]))
                    }
                    Err(err) => (format!("Error: {:?}", err), VecN::from_array([0.0, 0.0, 255.0, 0.0]))
                };
                opencv::imgproc::put_text(&mut crop, &text, Point2i::new(10, 30), 1, 2.0, text_color, 2, -1, false).unwrap();
                if !gray.frame.mat.empty() { highgui::imshow(w_gray, &gray.frame.mat).unwrap() };
                if !gamma.result.mat.empty() { highgui::imshow(w_gamma, &gamma.result.mat).unwrap() };
                if !contours.frame.mat.empty() { highgui::imshow(w_contours, &contours.frame.mat).unwrap() };
                if !crop.empty() { highgui::imshow(w_crop, &crop).unwrap() };
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
    dots: FastEdgesCtx,
}
impl FakePassDots{
    pub fn new(dots: FastEdgesCtx) -> Self {
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

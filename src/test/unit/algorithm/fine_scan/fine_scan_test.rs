#[cfg(test)]
use crate::{algorithm::{Initial, InitialCtx}, domain::{Eval, Image}};
use std::{any::TypeId, sync::Once, time::{Duration, Instant}};
use opencv::{core::{Mat, MatTrait, MatTraitConst, Point2i, Vec3b}, highgui};
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
        AutoGamma, Context, ContextRead, Cropping, CroppingCtx, EvalResult, FastScanCtx, FineContoursCtx,
        FineEdgesCtx, FineScan, FineScanConf, FineScanCtx, FineUnionCtx, Gray, GrayCtx, RopeDimensions,
        RopeDimensionsConf, RopeDimensionsCtx, Side, FastEdgesCtx,
    }, 
    domain::{Color, ColorProps, Error},
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
/// Drawing the edges points on the image
fn draw_dots<Branch: 'static>(dbg: &Dbg, mut img: Mat, ctx: &Context) -> Result<Mat, Error> {
    let (upper, lower) = match TypeId::of::<Branch>() {
        typ if typ == TypeId::of::<FastScanCtx>() => {
            let edges: &FastEdgesCtx = ctx.read();
            (edges.edges.get(Side::Upper), edges.edges.get(Side::Lower))
        }
        typ if typ == TypeId::of::<FineScanCtx>() => {
            let edges: &FineEdgesCtx = ctx.read();
            (edges.edges.get(Side::Upper), edges.edges.get(Side::Lower))
        }
        _ => {
            return  Err(Error::new(dbg, "draw_dots").err(format!("Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>())));
        }
    };
    log::trace!("{dbg}.eval | upper: {:?}", upper);
    log::trace!("{dbg}.eval | lower: {:?}", lower);
    if !img.empty() {
        for (upper, lower) in upper.iter().zip(lower) {
            *img.at_2d_mut::<Vec3b>(upper.y as i32, upper.x as i32).unwrap() = Color::Blue.bgr().into();
            *img.at_2d_mut::<Vec3b>(lower.y as i32, lower.x as i32).unwrap() = Color::Green.bgr().into();
        }
    }
    Ok(img)
}
///
/// Drawing Rope dimensions verification result
fn draw_rope_dimensions<Branch: 'static>(mut img: Mat, ctx: &Context, conf: &RopeDimensionsConf) -> Mat {
    let (text, text_color) = match RopeDimensions::<Branch>::new(
        conf.rope_width,
        conf.width_tolerance,
        conf.square_tolerance,
        FakePassCtx::new(ctx.clone()),
    ).eval(Image::with(img.clone())) {
        Ok(ctx) => {
            let dimensions: &RopeDimensionsCtx<FastScanCtx> = ctx.read();
            let width_error = (100.0 - dimensions.width * 100.0 / conf.rope_width as f64).abs();
            let square_error = (100.0 - dimensions.square * 100.0 / (conf.rope_width as f64 * img.cols() as f64)).abs();
            (format!("Rope width: {:.3} ({:.2}%), square: {} ({:.2}%)", dimensions.width, width_error, dimensions.square, square_error), Color::SkyBlue)
        }
        Err(err) => (format!("Error: {:?}", err), Color::Red)
    };
    opencv::imgproc::put_text(
        &mut img, &text,
        Point2i::new(10, 30),
        1,
        2.0,
        text_color.bgra(255.0).into(),
        2,
        -1,
        false,
    ).unwrap();
    img
}
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
            fine-contours:
                otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
                merge-distance: 24.0    # Maximum distance between contours to be merged
            temporal-filter:
                gaussian:
                    kernel: [11, 11]    # Gausian blur kernel size, must be odd
                    sigma: [0.0, 0.0]   # Standard deviation in [X, Y] direction, The higher the value, the more pixels are used to count each pixel and the smoother blur will be
                open-kernel: [3, 3]     # Morphology open operation kernel size [w, h], default [5, 5]
                erode-kernel: [3, 3]    # Morphology erode operation kernel size [w, h], default [5, 5]
                threshold: 12.0         # Threshold to detect the pixel whas changed or not in the each next frame
            fine-edges:
                # otsu-tune: 1.40       # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
                threshold: 16           # 0...255, used if otsu-tune is not specified
                smooth: 16              # Smoothing of edge line factor. The higher the factor the smoother the line.
            union:
                bitwise-and:
                    no-params: ~
                # add-weighted:
                #     weight1: 1.0            # Weight of the first array elements.
                #     weight2: 1.0            # Weight of the second array elements.
            rope-dimensions:        # Verifaing the rope dimensions 
                rope-width: 380               # Standart rope width, px
                width-tolerance: 30.0         # Tolerance for rope width, %
                square-tolerance: 100.0       # Tolerance for rope square, %
            geometry-defect-threshold: 1.0    # 1.1..1.3, absolute threshold to detect the geometry deffects
        "#)).unwrap(),
    );
    let conf = FineScanConf::new(&dbg, conf);
    // let cropp = Cropping::new(100, 1000, 100, 1000, Initial::new(InitialCtx::new()));
    let tp = ThreadPool::new(&dbg, Some(4));
    let fine_scan = FineScan::new(
        conf,
        tp.scheduler(),
        None::<Box<dyn Fn(&Context) + Send + Sync>>,
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
                    true
                ),
                false,
            ),
        ),
        true,
    );
    let w_gray = "Gray";
    let w_crop = "Cropped";
    let w_contours = "Fine Contours";
    let w_union = "Union";
    // let w_temp_filter = "Temporal Filter";
    for window in [w_gray, w_crop, w_contours, w_union] {
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
                let t = Instant::now();
                // let test = src.clone();
                let ctx = fine_scan.eval(frame.clone()).wait().unwrap().unwrap();
                let gray: &GrayCtx = ctx.read();
                let crop: &CroppingCtx = ctx.read();    
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let contours: &FineContoursCtx = ctx.read();
                // let temp_filter: &TemporalFilterCtx<FineScanCtx> = ctx.read();
                let crop = if crop.result.mat.empty() {
                    let mut dst = opencv::core::Mat::default();
                    opencv::imgproc::cvt_color(&gray.frame.mat, &mut dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
                    dst
                } else {
                    crop.result.mat.clone()
                };
                let union: &FineUnionCtx = ctx.read();
                let crop = draw_dots::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
                let crop = draw_rope_dimensions::<FineScanCtx>(crop, &ctx, &conf.rope_dimensions);
                if !gray.frame.mat.empty() { highgui::imshow(w_gray, &gray.frame.mat).unwrap() };
                if !contours.result.mat.empty() { highgui::imshow(w_contours, &contours.result.mat).unwrap() };
                if !union.frame.mat.empty() { highgui::imshow(w_union, &union.frame.mat).unwrap() };
                if !crop.empty() { highgui::imshow(w_crop, &crop).unwrap() };
                // if !temp_filter.frame.mat.empty() { highgui::imshow(w_temp_filter, &temp_filter.frame.mat).unwrap() };
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
struct FakePassCtx {
    ctx: Context,
}
impl FakePassCtx{
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassCtx {
    fn eval(&self, _: Image) -> Result<Context, Error> {
        Ok(self.ctx.clone())
    }
}

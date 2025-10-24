#[cfg(test)]
use crate::{algorithm::{Initial, InitialCtx}, domain::{Eval, Image}};
use std::{any::TypeId, sync::Once, time::{Duration, Instant}};
use opencv::{core::{Mat, MatTrait, MatTraitConst, Point2i, Rect, Vec3b}, highgui, imgproc::LineTypes};
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
        AutoGamma, Bend, Context, ContextRead, ContextWrite, Cropping, CroppingCtx, EvalResult, FastEdgesCtx, FastScanCtx, FineContoursCtx, FineEdgesCtx, FineScan, FineScanConf, FineScanCtx, FineUnionCtx, Gray, GrayCtx, MadCtx, MetaCtx, RopeDefectCtx, RopeDefectKind, RopeDimensions, RopeDimensionsConf, RopeDimensionsCtx, RopeDistortionsCtx, Side
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
    ).eval(Image::from(img.clone(), 0)) {
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
/// Drawing Rope defects
fn draw_rope_distortions<Branch: 'static>(dbg: &Dbg, mut img: Mat, ctx: &Context) -> Result<Mat, Error> {
    let error = Error::new(dbg, "draw_rope_defects");
    let (distortions, mad) = match TypeId::of::<Branch>() {
        typ if typ == TypeId::of::<FastScanCtx>() => {
            let result = ContextRead::<RopeDistortionsCtx<FastScanCtx>>::read(ctx);
            (&result.result, result.mad)
        },
        typ if typ == TypeId::of::<FineScanCtx>() => {
            let result = ContextRead::<RopeDistortionsCtx<FineScanCtx>>::read(ctx);
            (&result.result, result.mad)
        }
        _ => return  Err(error.err(format!("Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
    };
    let offset = 64;
    if distortions.is_empty() {
        opencv::imgproc::put_text(
            &mut img, "No distortions",
            Point2i::new(10, offset ), 1, 2.0,
            Color::Green.bgra(0.0).into(),
            2, -1, false,
        ).map_err(|err| error.pass(err.to_string()))?;
    } else {
        let width = img.cols();
        let median = mad.median.round() as i32;
        opencv::imgproc::line(
            &mut img, Point2i::new(0, median), Point2i::new(width, median),
            Color::Red.bgra(0.0).into(), 1, LineTypes::LINE_8 as i32, 0,
        ).map_err(|err| error.pass(err.to_string()))?;
        for bend in distortions {
            for (upper, lower) in bend.upper.iter().zip(&bend.lower) {
                opencv::imgproc::circle(
                    &mut img,
                    Point2i::new(upper.x as i32, upper.y as i32),
                    1, Color::Orange.bgra(0.0).into(),
                    2, LineTypes::LINE_8 as i32, 0,
                ).map_err(|err| error.pass(err.to_string()))?;
                opencv::imgproc::circle(
                    &mut img,
                    Point2i::new(lower.x as i32, lower.y as i32),
                    1, Color::Orange.bgra(0.0).into(),
                    2, LineTypes::LINE_8 as i32, 0,
                ).map_err(|err| error.pass(err.to_string()))?;
            }
        }
    }
    Ok(img)
}
///
/// Drawing Rope defects
fn draw_rope_defects<Branch: 'static>(dbg: &Dbg, mut img: Mat, ctx: &Context) -> Result<Mat, Error> {
    let error = Error::new(dbg, "draw_rope_defects");
    let defects = match TypeId::of::<Branch>() {
        // typ if typ == TypeId::of::<FastScanCtx>() => &ContextRead::<RopeDefectCtx<FastScanCtx>>::read(ctx).result,
        typ if typ == TypeId::of::<FineScanCtx>() => &ContextRead::<RopeDefectCtx<FineScanCtx>>::read(ctx).result,
        _ => return  Err(error.err(format!("Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
    };
    let offset = 64;
    if defects.is_empty() {
        opencv::imgproc::put_text(
            &mut img, "No defects",
            Point2i::new(10, offset + 24), 1, 2.0,
            Color::Green.bgra(0.0).into(),
            2, -1, false,
        ).map_err(|err| error.pass(err.to_string()))?;
    } else {
        opencv::imgproc::put_text(
            &mut img, "Defects:",
            Point2i::new(10, offset ), 1, 2.0,
            Color::Red.bgra(0.0).into(),
            2, -1, false,
        ).map_err(|err| error.pass(err.to_string()))?;
        for defect in defects {
            let (text, start, end) = match defect {
                RopeDefectKind::Expansion(start, end) => ("Expansion", start, end),
                RopeDefectKind::Compressing(start, end) => ("Compressing", start, end),
                RopeDefectKind::Hill(start, end) => ("Hill", start, end), // Холмик
                RopeDefectKind::Pit(start, end) => ("Pit", start, end),   // Ямка
            };
            let height = img.rows() - 200;
            opencv::imgproc::rectangle(
                &mut img,
                Rect::new(*start as i32, 100, (end - start) as i32, height),
                Color::Orange.bgra(0.0).into(),
                1, LineTypes::LINE_8 as i32, 0,
            ).map_err(|err| error.pass(err.to_string()))?;
            opencv::imgproc::put_text(
                &mut img, &text,
                Point2i::new(*start as i32  + 5, 115 ), 1, 0.5,
                // Point2i::new(20, (i as i32 + 1) * line_height + offset ), 1, 2.0,
                Color::OrangeRed.bgra(0.0).into(),
                1, -1, false,
            ).map_err(|err| error.pass(err.to_string()))?;
        }
    }
    Ok(img)
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
            distortion-threshold: 1.2    # 1.1..1.3, absolute threshold to detect the geometry deffects
            defect-threshold: 1.2    # 1.1..1.3, absolute threshold to detect the geometry deffects
            absolute-threshold: 16         # 4..36, pixels to detect the rope distortions
        "#)).unwrap(),
    );
    let conf = FineScanConf::new(&dbg, conf);
    // let cropp = Cropping::new(100, 1000, 100, 1000, Initial::new(InitialCtx::new()));
    let tp = ThreadPool::new(&dbg, Some(4));
    let fine_scan = FineScan::new(
        conf,
        tp.scheduler(),
        None::<Box<dyn Fn(&Context) + Send + Sync>>,
        FakeDistortions::new(
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

    for (meta, path) in std::fs::read_dir(image_dir).unwrap().into_iter()
        .filter_map(|e| {
            let path = e.unwrap().path();
            path.is_file().then(|| path)
        })
        .enumerate()
    {
        match path.extension() {
            Some(ext) if ext == "jpg" || ext == "png" || ext == "jpeg" => {
                let frame = Image::load(path.to_str().unwrap(), meta).unwrap();
                // let mut rotated = Mat::default();
                // core::rotate(&frame.mat, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                // let src = Image::with(rotated);
                log::debug!("{dbg}.eval | src frame: {} x {}", frame.width(), frame.height());
                let t = Instant::now();
                // let test = src.clone();
                let ctx = fine_scan.eval(frame.clone()).wait().unwrap().unwrap();
                let result_meta: &MetaCtx = ctx.read();
                assert!(*result_meta == meta, "{dbg} | \nresult: {:?}\ntarget: {:?}", result_meta, meta);
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
                let crop = draw_rope_distortions::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
                let crop = draw_rope_defects::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
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
        let distortions = RopeDistortionsCtx::<FastScanCtx>::new(vec![Bend::from([0, 0, 0, 0])], MadCtx::default());
        self.ctx.clone().write(distortions)
        // Ok(self.ctx.clone())
    }
}
///
/// Adds fake distortions result into the context
struct FakeDistortions {
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
impl FakeDistortions{
    pub fn new(ctx: impl Eval<Image, EvalResult> + 'static,) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FakeDistortions {
    fn eval(&self, frame: Image) -> Result<Context, Error> {
        let error = Error::new("FakeDistortions", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let distortions = RopeDistortionsCtx::<FastScanCtx>::new(
                    vec![Bend::from([0, 0, 0, 0])],
                    MadCtx::default(),
                );
                ctx.write(distortions)
            }
            Err(err) => Err(error.pass(err)),
        }
        // Ok(self.ctx.clone())
    }
}

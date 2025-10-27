extern crate frdm_tools;
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
use std::any::TypeId;

use crossterm::event::{KeyEventKind, KeyEventState};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use opencv::{core::{Mat, MatTrait, MatTraitConst, Point2i, Rect, Vec3b}, imgproc::LineTypes};
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::{services::conf::ConfTree, sync::Owner, thread_pool::ThreadPool};
use crate::{
    algorithm::{
        AutoGamma, Context, ContextRead, Cropping, CroppingCtx, EvalResult, FastContoursCtx, FastEdgesCtx, FastScan, FastScanCtx, FineContoursCtx, FineEdgesCtx, FineScan, FineScanCtx, Gray, GrayCtx, Initial, InitialCtx, MetaCtx, RopeDefectCtx, RopeDefectKind, RopeDimensions, RopeDimensionsConf, RopeDimensionsCtx, RopeDistortionsCtx, Side
    }, conf::Conf, domain::{Color, ColorProps, Eval, Image}, infrostructure::camera::{Camera, CameraConf}
};
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
fn draw_rope_dimensions<Branch: 'static>(dbg: &Dbg, mut img: Mat, ctx: &Context, conf: &RopeDimensionsConf) -> Result<Mat, Error> {
    let error = Error::new(dbg, "draw_rope_dimensions");
    let (text, text_color) = match RopeDimensions::<Branch>::new(
        conf.rope_width,
        conf.width_tolerance,
        conf.square_tolerance,
        FakePassCtx::new(ctx.clone()),
    ).eval(Image::from(img.clone(), 0)) {
        Ok(ctx) => {
            let (width, square) = match TypeId::of::<Branch>() {
                typ if typ == TypeId::of::<FastScanCtx>() => (
                    ContextRead::<RopeDimensionsCtx<FastScanCtx>>::read(&ctx).width,
                    ContextRead::<RopeDimensionsCtx<FastScanCtx>>::read(&ctx).square,
                ),
                typ if typ == TypeId::of::<FineScanCtx>() => (
                    ContextRead::<RopeDimensionsCtx<FineScanCtx>>::read(&ctx).width,
                    ContextRead::<RopeDimensionsCtx<FineScanCtx>>::read(&ctx).square,
                ),
                _ => return  Err(error.err(format!("Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
            };
            let width_error = (100.0 - width * 100.0 / conf.rope_width as f64).abs();
            let square_error = (100.0 - square * 100.0 / (conf.rope_width as f64 * img.cols() as f64)).abs();
            (format!("Rope width: {:.3} ({:.2}%), square: {} ({:.2}%)", width, width_error, square, square_error), Color::SkyBlue)
        }
        Err(err) => (format!("Error: {:?}", err), Color::Red)
    };
    opencv::imgproc::put_text(
        &mut img, &text,
        Point2i::new(10, 30),
        1,
        2.0,
        text_color.bgra(0.0).into(),
        2,
        -1,
        false,
    ).map_err(|err| error.pass(err.to_string()))?;
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
/// Application entry point
/// 
/// - For basic test execute:
/// 
///     `clear && cargo run --bin complex-test --release -- --nocapture`
/// 
/// - For Pause / Resume the Camera execute:
/// 
///     `clear && cargo run --bin complex-test --release -- --nocapture --cam-pause`
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = Dbg::own("complex-test");
    let source = Source::Path("src/test/unit/algorithm/temporal_filter/frames");
    let source = Source::Path("/home/lobanov/code/rust/cma-server/src/tests/unit/services/frdm_service/frames");
    // let source = Source::Camera("src/complex-test-camera.yaml");
    let mut exposure = 0.0;
    let stream: Box<dyn Iterator<Item = Image>> = match source {
        Source::Path(path) => {
            Box::new(std::fs::read_dir(path).unwrap().into_iter()
                .filter_map(|e| {
                    let path = e.unwrap().path();
                    path.is_file().then(|| path)
                })
                .enumerate()
                .filter_map(|(meta, path)| {
                    if let Some(ext) = path.extension() {
                         if ext == "jpg" || ext == "png" || ext == "jpeg" {
                            return Some(Image::load(path.to_str().unwrap(), meta).unwrap());
                         }
                    }
                    None
                }),
            )
        }
        Source::Camera(path) => {
            let conf = CameraConf::read(&dbg, path);
            exposure = conf.exposure.time;
            let mut handles = vec![];
            let mut camera = Camera::new(None::<Box<dyn Fn() -> usize + Send + Sync>>, conf);
            handles.push(
                camera.read().unwrap()
            );
            let camera_stream: Box<dyn Iterator<Item = Image>> = Box::new(camera.stream());
            println!(r#"
                Add '--cam-pause' argumet to the cli commad to anable Pause / Resume for the Camera
                    Then press a key:
                        Esc or 'q' to exit,
                        'p' to suspend / resume camera"#);
            if let Some(arg) = std::env::args().find(|arg| arg == "--cam-pause") {
                println!("Arg: {}", arg);
                let handle = std::thread::spawn(move || {
                    crossterm::terminal::enable_raw_mode().unwrap();
                    let mut paused = false;
                    loop {
                        match crossterm::event::read().unwrap() {
                            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                                code: crossterm::event::KeyCode::Char('p'),
                                modifiers: crossterm::event::KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                paused = !paused;
                                match paused {
                                    true => camera.suspend(),
                                    false => camera.resume(),
                                }
                            },
                            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                                code: crossterm::event::KeyCode::Char('q'),
                                modifiers: crossterm::event::KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                crossterm::terminal::disable_raw_mode().unwrap();
                                std::process::exit(0);
                            }
                            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                                code: crossterm::event::KeyCode::Esc,
                                modifiers: crossterm::event::KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                crossterm::terminal::disable_raw_mode().unwrap();
                                std::process::exit(0);
                            }
                            _ => (),
                        }
                    }
                });
                handles.push(handle);
            }
            camera_stream
        }
    };
    let w_source = "Source";
    let w_crop = "Cropped";
    let w_gray = "Gray";
    let w_fast_contours = "Fast Contours";
    let w_fine_contours = "Fine Contours";
    let w_fast = "Fast Scan";
    let w_fine = "Fine Scan";
    for window in [w_source, w_crop, w_gray, w_fast_contours, w_fine_contours, w_fast, w_fine] {
        if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
            log::warn!("{dbg} | Create Window Error: {}", err);
        }
    }
    let mut counter = 0;
    let mut frame_counter = 0;
    let conf = std::fs::OpenOptions::new().read(true).open("src/complex-test.yaml").unwrap();
    let conf = ConfTree::new_root(serde_yaml::from_reader(conf).unwrap());
    let conf = Conf::new(&dbg, conf);
    let tp = ThreadPool::new(&dbg, Some(8));
    let fine_scan = FineScan::new(
        conf.fine_scan,
        tp.scheduler(),
        None::<Box<dyn Fn(&Context) + Send + Sync>>,
        FastScan::new(
            conf.fast_scan,
            tp.scheduler(),
            Gray::new(
                AutoGamma::new(
                    conf.normalize.gamma.factor,
                    Cropping::new(
                        conf.normalize.cropping.x,
                        conf.normalize.cropping.width,
                        conf.normalize.cropping.y,
                        conf.normalize.cropping.height,
                        Initial::new(
                            InitialCtx::new(),
                        ),
                        true,
                    ),
                    true,
                ),
            ),
            true,
        ),
        true,
    );
    for frame in stream {
        log::trace!("{dbg} | Frame width: {},  height: {}, timestamp: {}", frame.width(), frame.height(), frame.meta);
        opencv::highgui::imshow(w_source, &frame.mat).unwrap();
        let ctx = fine_scan.eval(frame.clone()).wait().unwrap().unwrap();
        let result_meta: &MetaCtx = ctx.read();
        assert!(*result_meta == frame.meta, "{dbg} | \nresult: {:?}\ntarget: {:?}", result_meta, frame.meta);
        let gray: &GrayCtx = ctx.read();
        let crop: &CroppingCtx = ctx.read();
        let fast_contours_ctx: &FastContoursCtx = ctx.read();
        let fine_contours_ctx: &FineContoursCtx = ctx.read();
        let fast_ctx: &FastScanCtx = ctx.read();
        let fine_ctx: &FineScanCtx = ctx.read();
        let crop = if crop.result.mat.empty() {
            let mut dst = opencv::core::Mat::default();
            opencv::imgproc::cvt_color(&gray.frame.mat, &mut dst, opencv::imgproc::COLOR_GRAY2BGR, 3).unwrap();
            dst
        } else {
            crop.result.mat.clone()
        };
        let crop = draw_dots::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
        let crop = draw_rope_dimensions::<FineScanCtx>(&dbg, crop, &ctx, &conf.fine_scan.rope_dimensions).unwrap();
        let crop = draw_rope_defects::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
        let crop = draw_rope_distortions::<FineScanCtx>(&dbg, crop, &ctx).unwrap();
        if !crop.empty() { opencv::highgui::imshow(w_crop, &crop).unwrap(); }
        if !gray.frame.mat.empty() { opencv::highgui::imshow(w_gray, &gray.frame.mat).unwrap(); }
        if !fast_contours_ctx.result.mat.empty() { opencv::highgui::imshow(w_fast_contours, &fast_contours_ctx.result.mat).unwrap(); }
        if !fine_contours_ctx.result.mat.empty() { opencv::highgui::imshow(w_fine_contours, &fine_contours_ctx.result.mat).unwrap(); }
        if !fast_ctx.union.frame.mat.empty() { opencv::highgui::imshow(w_fast, &fast_ctx.union.frame.mat).unwrap(); }
        if !fine_ctx.union.frame.mat.empty() { opencv::highgui::imshow(w_fine, &fine_ctx.union.frame.mat).unwrap(); }
        if counter == 5{
            //_2lightAngle45_600rpm_
            // let path_retr = &format!("/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp{}_rope/retrived/", exposure);
            // let path_proc = &format!("/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp{}_rope/processed/", exposure);
            // let file_name = &format!("exp{}_rope_frame_{}.jpeg", exposure, frame_counter);
            // fs::create_dir_all(path_retr).unwrap();
            // fs::create_dir_all(path_proc).unwrap();
            // opencv::imgcodecs::imwrite(&format!("{}/{}", path_retr, file_name), &frame.mat, &opencv::core::Vector::new()).unwrap();
            // opencv::imgcodecs::imwrite(&format!("{}/{}", path_proc, file_name), &contours_ctx.result.mat, &opencv::core::Vector::new()).unwrap();
            frame_counter = frame_counter + 1;
            counter = 0;
        }
        counter = counter + 1;
        opencv::highgui::wait_key(0).unwrap();
    }
    // let _: Vec<()> = handles.into_iter().map(|h| h.join().unwrap()).collect();
}
///
/// Select image source
enum Source<'a> {
    Path(&'a str),
    Camera(&'a str),
}
///
/// Fake implements `Eval` for testing [RopeDimensions]
struct FakePassCtx {
    ctx: Owner<Context>,
}
impl FakePassCtx{
    pub fn new(ctx: Context) -> Self {
        Self { ctx: Owner::new(ctx) }
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassCtx {
    fn eval(&self, _: Image) -> Result<Context, Error> {
        self.ctx.take().ok_or(Error::new("FakePassCtx", "eval").err("Can't get Context"))
    }
}

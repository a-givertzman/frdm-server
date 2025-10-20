extern crate frdm_tools;
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
use std::fs;
use crossterm::event::{KeyEventKind, KeyEventState};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use opencv::core::MatTraitConst;
use sal_core::dbg::Dbg;
use sal_sync::{services::conf::ConfTree, thread_pool::ThreadPool};
use crate::{
    algorithm::{
        AutoGamma, Context, ContextRead, Cropping, CroppingCtx, FastContoursCtx, FastScan, FastScanCtx, FineScan, FineScanCtx, Gray, GrayCtx, Initial, InitialCtx
    }, conf::Conf, domain::{Eval, Image}, infrostructure::camera::{Camera, CameraConf}
};
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
    // let source = Source::Camera("src/complex-test-camera.yaml");
    let mut exposure = 0.0;
    let stream: Box<dyn Iterator<Item = Image>> = match source {
        Source::Path(path) => {
            Box::new(std::fs::read_dir(path).unwrap().into_iter()
                .filter_map(|e| {
                    let path = e.unwrap().path();
                    path.is_file().then(|| path)
                })
                .filter_map(|path| {
                    if let Some(ext) = path.extension() {
                         if ext == "jpg" || ext == "png" || ext == "jpeg" {
                            return Some(Image::load(path.to_str().unwrap()).unwrap());
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
            let mut camera = Camera::new(conf);
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
    let w_gamma = "Auto Gamma";
    let w_gray = "Gray";
    let w_fast = "Fast Scan";
    let w_fine = "Fine Scan";
    let w_contours = "Contours";
    for window in [w_source, w_crop, w_gamma, w_gray, w_fast, w_fine, w_contours] {
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
        log::trace!("{dbg} | Frame width: {},  height: {}, timestamp: {}", frame.width(), frame.height(), frame.timestamp);
        opencv::highgui::imshow(w_source, &frame.mat).unwrap();
        let ctx = fine_scan.eval(frame.clone()).wait().unwrap().unwrap();
        let gray: &GrayCtx = ctx.read();
        let crop: &CroppingCtx = ctx.read();
        let fast_ctx: &FastScanCtx = ctx.read();
        let fine_ctx: &FineScanCtx = ctx.read();
        let contours_ctx: &FastContoursCtx = ctx.read();
        if !crop.result.mat.empty() { opencv::highgui::imshow(w_crop, &crop.result.mat).unwrap(); }
        if !gray.frame.mat.empty() { opencv::highgui::imshow(w_gray, &gray.frame.mat).unwrap(); }
        if !fast_ctx.union.frame.mat.empty() { opencv::highgui::imshow(w_fast, &fast_ctx.union.frame.mat).unwrap(); }
        if !fine_ctx.union.frame.mat.empty() { opencv::highgui::imshow(w_fine, &fine_ctx.union.frame.mat).unwrap(); }
        if !contours_ctx.result.mat.empty() { opencv::highgui::imshow(w_contours, &contours_ctx.result.mat).unwrap(); }
        if !contours_ctx.result.mat.empty() { opencv::highgui::imshow(w_contours, &contours_ctx.result.mat).unwrap(); }
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
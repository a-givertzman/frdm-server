extern crate frdm_tools;
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
use std::fs;
use crossterm::event::{KeyEventKind, KeyEventState};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use sal_sync::{services::conf::ConfTree, thread_pool::ThreadPool};
use crate::{
    algorithm::{
        AutoGamma, Context, ContextRead, Cropping, FastContoursCtx, FastScan, FineScan, Gray, Initial, InitialCtx,
    }, conf::Conf, domain::Eval, infrostructure::camera::{Camera, CameraConf}
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
    let path = "./config.yaml";
    let conf = CameraConf::read(&dbg, path);
    let mut handles = vec![];
    let mut camera = Camera::new(conf);
    handles.push(
        camera.read().unwrap()
    );
    let camera_stream = camera.stream();
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
    let window = "Retrived";
    let window2 = "Processed";
    let mut counter = 0;
    let mut frame_counter = 0;
    let conf_temp = CameraConf::read(&dbg, path);
    let exposure = conf_temp.exposure.time;
    if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", dbg, err);
    }
    if let Err(err) = opencv::highgui::named_window(window2, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", dbg, err);
    }
    opencv::highgui::wait_key(1).unwrap();
    let conf = std::fs::OpenOptions::new().read(true).open("config.yaml").unwrap();
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
            true,
        ),
        true,
    );
    for frame in camera_stream {
        log::trace!("{dbg} | Frame width: {},  height: {}, timestamp: {}", frame.width(), frame.height(), frame.timestamp);
        opencv::highgui::imshow(window, &frame.mat).unwrap();
        let ctx = fine_scan.eval(frame.clone()).wait().unwrap().unwrap();
        let contours_ctx: &FastContoursCtx = ctx.read();
        opencv::highgui::imshow(window2, &contours_ctx.result.mat).unwrap();
        if counter == 5{
            //_2lightAngle45_600rpm_
            let path_retr = &format!("/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp{}_rope/retrived/", exposure);
            let path_proc = &format!("/home/ilyarizo/deffect_photos/exp_gradient_rope_2diod/exp{}_rope/processed/", exposure);
            let file_name = &format!("exp{}_rope_frame_{}.jpeg", exposure, frame_counter);
            fs::create_dir_all(path_retr).unwrap();
            fs::create_dir_all(path_proc).unwrap();
            opencv::imgcodecs::imwrite(&format!("{}/{}", path_retr, file_name), &frame.mat, &opencv::core::Vector::new()).unwrap();
            opencv::imgcodecs::imwrite(&format!("{}/{}", path_proc, file_name), &contours_ctx.result.mat, &opencv::core::Vector::new()).unwrap();
            frame_counter = frame_counter + 1;
            counter = 0;
        }
        counter = counter + 1;
        opencv::highgui::wait_key(1).unwrap();
    }
    let _: Vec<()> = handles.into_iter().map(|h| h.join().unwrap()).collect();
}

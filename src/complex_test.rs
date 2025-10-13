extern crate frdm_tools;
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
use std::fs;
use crossterm::event::{KeyEventKind, KeyEventState};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        AutoGamma, ContextRead, Cropping, FastContours,
        FastContoursCtx, Gray, Initial, InitialCtx,
        TemporalFilter, FastScanConf, FineScanConf,
        FastContoursConf,
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
    let mut camera = Camera::new(conf);
    let recv = camera.stream();
    let mut handles = vec![];
    handles.push(
        camera.read().unwrap()
    );
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
    let conf = Conf {
        fast_scan: FastScanConf::default(),
        fine_scan: FineScanConf::default(),
    };
    for frame in recv {
        log::trace!("{} | Frame width : {:?}", dbg, frame.width);
        log::trace!("{} | Frame height: {:?}", dbg, frame.height);
        log::trace!("{} | Frame timestamp: {:?}", dbg, frame.timestamp);
        if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
            log::warn!("{}.stream | Display img error: {:?}", dbg, err);
        };
        let debug = false;
        let contours_result = FastContours::new(
            FastContoursConf::default(),
            TemporalFilter::new(
                conf.fast_scan.temporal_filter.gaussian,
                conf.fast_scan.temporal_filter.open_kernel,
                conf.fast_scan.temporal_filter.erode_kernel,
                conf.fast_scan.temporal_filter.threshold,
                Gray::new(
                    AutoGamma::new(
                        conf.fast_scan.fast_contours.gamma.factor,
                        Cropping::new(
                            conf.fast_scan.fast_contours.cropping.x,
                            conf.fast_scan.fast_contours.cropping.width,
                            conf.fast_scan.fast_contours.cropping.y,
                            conf.fast_scan.fast_contours.cropping.height,
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
        ).eval(frame.clone()).unwrap();
        let contours_ctx = ContextRead::<FastContoursCtx>::read(&contours_result);
        if let Err(e) = opencv::highgui::imshow(window2, &contours_ctx.result.mat) {
            log::error!("Display error: {}", e);
        }
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
        // let conf = Conf {
        //     fast_scan: FastScanConf {
        //         geometry_defect_threshold: Threshold::min(),
        //     },
        //     fine_scan: FineScanConf {},
        // };
        // let result = GeometryDefect::new(
        //     conf.fast_scan.geometry_defect_threshold,
        //     *Box::new(Mad::new()),
        //     EdgeDetection::new(
        //         FastContours::new(
        //             Initial::new(
        //                 InitialCtx::new(frame),
        //             ),
        //         ),
        //     ),
        // )
        // .eval(());
        // _ = result;
    }
    let _: Vec<()> = handles.into_iter().map(|h| h.join().unwrap()).collect();
}
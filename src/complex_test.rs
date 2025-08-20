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
        AutoBrightnessAndContrast, AutoGamma, ContextRead, Cropping, DetectingContoursCv, DetectingContoursCvCtx, Initial, InitialCtx, Threshold
    }, conf::{Conf, DetectingContoursConf, EdgeDetectionConf, FastScanConf, FineScanConf}, domain::Eval, infrostructure::camera::{Camera, CameraConf}
};
///
/// Application entry point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = Dbg::own("main");
    let path = "./config.yaml";
    let conf = CameraConf::read(&dbg, path);
    let mut camera = Camera::new(conf);
    let recv = camera.stream();
    let mut handles = vec![];
    handles.push(
        camera.read().unwrap()
    );
    println!(r#"
        Add '--cam-pause' argumet to test commad to Pause / Resume the Camera
            Then press a key:
                Esc or 'q' to exit,
                'p' to suspend / resume camera"#);
    if let Some(arg) = std::env::args().find(|arg| arg == "--cam-pause") {
        println!("Arg: {}", arg);
        let handle = std::thread::spawn(move || {
            // input key detection
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
                        match paused {
                            true => camera.suspend(),
                            false => camera.resume(),
                        }
                        paused = !paused;
                    },
                    crossterm::event::Event::Key(crossterm::event::KeyEvent {
                        code: crossterm::event::KeyCode::Char('q'),
                        modifiers: crossterm::event::KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => std::process::exit(0),
                    crossterm::event::Event::Key(crossterm::event::KeyEvent {
                        code: crossterm::event::KeyCode::Esc,
                        modifiers: crossterm::event::KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    }) => std::process::exit(0),
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
    let mut exposure = conf_temp.exposure.time;
    if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", dbg, err);
    }
    if let Err(err) = opencv::highgui::named_window(window2, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", dbg, err);
    }
    opencv::highgui::wait_key(1).unwrap();
    let conf = Conf {
        contours: DetectingContoursConf::default(),
        edge_detection: EdgeDetectionConf::default(),
        fast_scan: FastScanConf {
            geometry_defect_threshold: Threshold::min(),
        },
        fine_scan: FineScanConf {},
    };
    for frame in recv {
        log::trace!("{} | Frame width : {:?}", dbg, frame.width);
        log::trace!("{} | Frame height: {:?}", dbg, frame.height);
        log::trace!("{} | Frame timestamp: {:?}", dbg, frame.timestamp);
        if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
            log::warn!("{}.stream | Display img error: {:?}", dbg, err);
        };
        let contours_result = DetectingContoursCv::new(
            DetectingContoursConf::default(),
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
        ).eval(frame.clone()).unwrap();
        let contours_ctx = ContextRead::<DetectingContoursCvCtx>::read(&contours_result);
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
        //         DetectingContoursCv::new(
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
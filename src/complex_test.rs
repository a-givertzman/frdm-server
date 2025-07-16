extern crate frdm_tools;
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
#[cfg(test)]
mod test;
use std::fs;

//
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        ContextRead, DetectingContoursCv, DetectingContoursCvCtx, EdgeDetection, GeometryDefect, Initial, InitialCtx, Mad, Threshold
    }, conf::{Conf, FastScanConf, FineScanConf}, domain::Eval, infrostructure::camera::{Camera, CameraConf}
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
    let handle = camera.read().unwrap();
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
    for frame in recv {
        log::trace!("{} | Frame width : {:?}", dbg, frame.width);
        log::trace!("{} | Frame height: {:?}", dbg, frame.height);
        log::trace!("{} | Frame timestamp: {:?}", dbg, frame.timestamp);
        if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
            log::warn!("{}.stream | Display img error: {:?}", dbg, err);
        };
        let contours_result = DetectingContoursCv::new(
            Initial::new(InitialCtx::new(frame.clone()))
        ).eval(()).unwrap();

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
    handle.join().unwrap()
}
mod algorithm;
mod conf;
mod domain;
mod infrostructure;
#[cfg(test)]
mod test;
//
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use crate::{
    domain::Eval,
    infrostructure::camera::{Camera, CameraConf},
    conf::{Conf, FastScanConf, FineScanConf},
    algorithm::{
        DetectingContoursCv, EdgeDetection, GeometryDefect, Initial, InitialCtx, Mad, Threshold,
    }
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
    if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
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
        opencv::highgui::wait_key(1).unwrap();
        let conf = Conf {
            fast_scan: FastScanConf {
                geometry_defect_threshold: Threshold::min(),
            },
            fine_scan: FineScanConf {},
        };
        let result = GeometryDefect::new(
            conf.fast_scan.geometry_defect_threshold,
            *Box::new(Mad::new()),
            EdgeDetection::new(
                DetectingContoursCv::new(
                    Initial::new(
                        InitialCtx::new(frame),
                    ),
                ),
            ),
        )
        .eval(());
        _ = result;
    }
    handle.join().unwrap()
}

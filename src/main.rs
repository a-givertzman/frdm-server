extern crate frdm_tools;
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
    algorithm::{
        AutoGamma, Cropping, FastContours, FastEdges, FastScanConf, FastScanCtx, FineScanConf, RopeDefect, Gray, Initial, InitialCtx, Mad, TemporalFilter, RopeDistortions
    }, conf::{Conf, NormalizeConf}, domain::Eval, infrostructure::camera::{Camera, CameraConf}
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
    let conf = Conf {
        normalize: NormalizeConf::default(),
        fast_scan: FastScanConf::default(),
        fine_scan: FineScanConf::default(),
    };
    let debug = false;
    let scan_rope = RopeDefect::<FastScanCtx>::new(
        conf.fast_scan.distortion_threshold,
        *Box::new(Mad::new()),
        RopeDistortions::<FastScanCtx>::new(
            conf.fast_scan.distortion_threshold,
            *Box::new(Mad::new()),
            FastEdges::new(
                conf.fast_scan.fast_edges.otsu_tune,
                conf.fast_scan.fast_edges.threshold,
                conf.fast_scan.fast_edges.smooth,
                FastContours::new(
                    conf.fast_scan.fast_contours.clone(),
                    TemporalFilter::<FastScanCtx>::new(
                        conf.fast_scan.temporal_filter.gaussian,
                        conf.fast_scan.temporal_filter.open_kernel,
                        conf.fast_scan.temporal_filter.erode_kernel,
                        conf.fast_scan.temporal_filter.threshold,
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
                                    debug,
                                ),
                                debug,
                            ),
                        ),
                        debug,
                    ),
                    debug,
                ),
            ),
        ),
    );
    for frame in recv {
        log::trace!("{dbg} | Frame width: {},  height: {}, timestamp: {}", frame.width(), frame.height(), frame.meta);
        if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
            log::warn!("{}.stream | Display img error: {:?}", dbg, err);
        };
        opencv::highgui::wait_key(1).unwrap();
        let result = scan_rope.eval(frame);
        _ = result;
    }
    handle.join().unwrap()
}

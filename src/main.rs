mod algorithm;
mod domain;
mod infrostructure;
mod conf;
#[cfg(test)]
mod test;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use infrostructure::camera::{camera::Camera, camera_conf::CameraConf};
use sal_sync::services::entity::dbg_id::DbgId;
///
/// Appliacation entri point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = DbgId("main".into());
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
    }
    handle.join().unwrap()
}

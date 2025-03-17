#[cfg(test)]

mod fast_scan {
    use std::{net::SocketAddr, os::linux::net::SocketAddrExt, sync::Once, time::{Duration, Instant}};
    use sal_sync::services::conf::conf_tree::ConfTree;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use opencv::{
        highgui, imgcodecs::{self, imread, IMREAD_COLOR}, imgproc, prelude::*, videoio, Result
    };
    use crate::{domain::dbg::dbgid::DbgId, infrostructure::camera::{camera::Camera, camera_conf::CameraConf, camera_resolution::CameraResolution, pimage::PImage}};
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
    /// Testing read from USB
    #[test]
    fn grad() {
    }
}

#[cfg(test)]

mod camera {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{domain::dbg::dbgid::DbgId, infrostructure::camera::{camera::Camera, camera_conf::CameraConf, camera_resolution::CameraResolution}};
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
    fn read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let conf: serde_yaml::Value = serde_yaml::from_str(r#"
            service Camera Camera1:
                fps: 30
                address: 192.168.10.12:2020
                resolution: 
                    width: 1200
                    height: 800
        "#).unwrap();
        let conf = CameraConf::from_yaml(&dbg, &conf);
        log::debug!("{}.read | conf: {:#?}", dbg, conf);
        let camera = Camera::new(conf);
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        for frame in camera {
            log::debug!("frame: {:?}", frame);
        }
        test_duration.exit();
    }
}

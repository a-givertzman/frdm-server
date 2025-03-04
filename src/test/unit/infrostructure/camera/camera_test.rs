#[cfg(test)]

mod camera {
    use std::{sync::Once, time::{Duration, Instant}};
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
    fn read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
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
        let mut camera_iter = camera.into_iter();
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        let mut cap = videoio::VideoCapture::from_file("src/video/video_test.mp4", videoio::CAP_ANY).unwrap();
        loop{
            let mut frame = Mat::default();
            cap.read(&mut frame).unwrap();
            if frame.empty() {
                break;
            }
            camera_iter.push_frame(PImage::new(frame));
        }
        while let Some(frame) = camera_iter.next() {
            highgui::imshow("Video", &frame.frame);
            if highgui::wait_key(30).unwrap() == 'q' as i32 {
                break;
            }
        }
        test_duration.exit();
    }
}

#[cfg(test)]

mod camera {
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
    fn read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                serde_yaml::from_str(r#"
                service Camera Camera1:
                    fps: 30
                    resolution: 
                        width: 1200
                        height: 800
                    address: 192.168.10.12:2020
                "#).unwrap(),
                CameraConf {
                    name: "/test/Camera1".into(),
                    fps: 30,
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    address: "192.168.10.12:2020".parse().unwrap(),
                }        
            ),
        ];
        for (step,yaml, target) in test_data {
            let result = CameraConf::from_yaml(&dbg, &yaml);
            assert_eq!(result,target,"step {} \nresult: {:?}\ntarget: {:?}",step, result, target);
        }
        test_duration.exit();
    }
    //
    //
    #[test]
    fn video(){
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                videoio::VideoCapture::from_file("src/video/video_test.mp4", videoio::CAP_ANY).unwrap(),
                Camera::new(CameraConf{
                    name: "/test/Camera1".into(),
                    fps: 30,
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    address: "192.168.10.12:2020".parse().unwrap(),
                }).into_iter(),
                214,
            ),
        ];
        for (step, mut video, mut camera, target) in test_data {
            loop {
                let mut frame = Mat::default();
                video.read(&mut frame).unwrap();
                if frame.empty() {
                    break;
                }
                camera.push_frame(PImage::new(frame));
            }
            let result = camera.count();
            assert_eq!(result, target);
        }
        test_duration.exit();
    }
}

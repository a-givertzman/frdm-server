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
        let dbg = DbgId::root("camera_test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                Camera::new(CameraConf{
                    name: "/test/Camera1".into(),
                    fps: 30,
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    address: "192.168.10.12:2020".parse().unwrap(),
                }).read("src/video/video_test.mp4"),
                videoio::VideoCapture::from_file("src/video/video_test.mp4", videoio::CAP_ANY).unwrap(),
            ),
        ];
        for (step, camera, mut target_video) in test_data {
            match camera {
                Ok(mut camera) => {
                    loop {
                        let mut target = Mat::default();
                        while let Ok(read) = target_video.read(&mut target) {
                            if read {
                                let result = camera.next().unwrap();
                                assert!(result == PImage::new(target.clone()), "{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target_video);
                            } else {
                                break;
                            }
                        }
                    }
                }
                Err(err) => panic!("{} | step {} Camera error: {:?}", dbg, step, err),
            }
        }
        test_duration.exit();
    }
}

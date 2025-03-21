#[cfg(test)]

mod camera {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use opencv::{
        prelude::*, videoio
    };
    use crate::{domain::dbg::dbgid::DbgId, infrostructure::{arena::{channel_packet_size::ChannelPacketSize, exposure::{Exposure, ExposureAuto}, frame_rate::FrameRate, pixel_format::PixelFormat}, camera::{camera::Camera, camera_conf::CameraConf, camera_resolution::CameraResolution, pimage::PImage}}};
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
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                serde_yaml::from_str(r#"
                service Camera Camera1:
                    fps: Min
                    resolution: 
                        width: 1200
                        height: 800
                    index: 0
                    address: 192.168.10.12:2020
                    pixel-format: BayerBG8        # Mono8/10/12/16, BayerBG8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411
                    exposure:
                        auto: Off               # Off / Continuous
                        time: 5000              # microseconds
                    auto-packet-size: true
                    channel-packet-size: Min
                    resend-packet: false
                "#).unwrap(),
                CameraConf {
                    name: "/test/Camera1".into(),
                    fps: FrameRate::Min,
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    index: Some(0),
                    address: Some("192.168.10.12:2020".parse().unwrap()),
                    pixel_format: PixelFormat::BayerBG8,
                    exposure: Exposure::new(ExposureAuto::Off, 5000.0),
                    auto_packet_size: true,
                    channel_packet_size: ChannelPacketSize::Min,
                    resend_packet: false,
                }        
            ),
            (
                2,
                serde_yaml::from_str(r#"
                service Camera Camera1:
                    fps: Max
                    resolution: 
                        width: 1200
                        height: 800
                    index: 0
                    address: 192.168.10.12:2020
                    pixel-format: BayerBG8        # Mono8/10/12/16, BayerBG8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411
                    exposure:
                        auto: Off               # Off / Continuous
                        time: 5000              # microseconds
                    auto-packet-size: true
                    channel-packet-size: Max
                    resend-packet: false
                "#).unwrap(),
                CameraConf {
                    name: "/test/Camera1".into(),
                    fps: FrameRate::Max,
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    index: Some(0),
                    address: Some("192.168.10.12:2020".parse().unwrap()),
                    pixel_format: PixelFormat::BayerBG8,
                    exposure: Exposure::new(ExposureAuto::Off, 5000.0),
                    auto_packet_size: true,
                    channel_packet_size: ChannelPacketSize::Max,
                    resend_packet: false,
                }        
            ),
            (
                3,
                serde_yaml::from_str(r#"
                service Camera Camera1:
                    fps: 30
                    resolution: 
                        width: 1200
                        height: 800
                    index: 0
                    address: 192.168.10.12:2020
                    pixel-format: BayerBG8        # Mono8/10/12/16, BayerBG8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411
                    exposure:
                        auto: Off               # Off / Continuous
                        time: 5000              # microseconds
                    auto-packet-size: true
                    channel-packet-size: 1024
                    resend-packet: false
                "#).unwrap(),
                CameraConf {
                    name: "/test/Camera1".into(),
                    fps: FrameRate::Val(30.0),
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    index: Some(0),
                    address: Some("192.168.10.12:2020".parse().unwrap()),
                    pixel_format: PixelFormat::BayerBG8,
                    exposure: Exposure::new(ExposureAuto::Off, 5000.0),
                    auto_packet_size: true,
                    channel_packet_size: ChannelPacketSize::Val(1024),
                    resend_packet: false,
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
                    fps: FrameRate::Val(30.0),
                    resolution: CameraResolution {
                        width: 1200,
                        height: 800,
                    },
                    index: Some(0),
                    address: Some("192.168.10.12:2020".parse().unwrap()),
                    pixel_format: PixelFormat::BayerBG8,
                    exposure: Exposure::new(ExposureAuto::Off, 5000.0),
                    auto_packet_size: true,
                    channel_packet_size: ChannelPacketSize::Max,
                    resend_packet: false,
                }).from_file("src/test/unit/infrostructure/camera/video_test.mp4"),
                videoio::VideoCapture::from_file("src/test/unit/infrostructure/camera/video_test.mp4", videoio::CAP_ANY).unwrap(),
            ),
        ];
        for (step, camera, mut target_video) in test_data {
            match camera {
                Ok(mut camera) => {
                    let mut frames = 0;
                    let mut target = Mat::default();
                    while let Ok(read) = target_video.read(&mut target) {
                        log::trace!("{} | step {} read: {:?}", dbg, step, read);
                        if read {
                            let result = camera.next().unwrap();
                            assert!(result == PImage::new(target.clone()), "{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target_video);
                            frames += 1;
                        } else {
                            break;
                        }
                    }
                    log::debug!("{} | step: {} Frames: {:?}", dbg, step, frames);
                }
                Err(err) => panic!("{} | step {} Camera error: {:?}", dbg, step, err),
            }
        }
        test_duration.exit();
    }
}

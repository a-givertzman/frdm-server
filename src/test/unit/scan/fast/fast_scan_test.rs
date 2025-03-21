#[cfg(test)]

mod fast_scan {
    use std::{net::SocketAddr, os::linux::net::SocketAddrExt, sync::Once, time::{Duration, Instant}};
    use sal_sync::services::conf::conf_tree::ConfTree;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use opencv::{
        core::{self, DMatch, KeyPoint, Vec4i, NORM_HAMMING}, features2d::{self, BFMatcher}, highgui, imgcodecs::{self, imread, IMREAD_COLOR}, imgproc, prelude::*, videoio, Result
    };
    use crate::{domain::dbg::dbgid::DbgId, infrostructure::camera::{camera::Camera, camera_conf::CameraConf, camera_resolution::CameraResolution, pimage::PImage}, scan::fast::Gradient};
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
                }).read("src/test/unit/infrostructure/camera/video_test.mp4"),
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
                            let scnaed = Gradient::new(result);
                            highgui::imshow("Video", &scnaed.gradient);
                            //assert!(result == PImage::new(target.clone()), "{} | step {} \nresult: {:?}\ntarget: {:?}", dbg, step, result, target_video);
                            frames += 1;
                            if highgui::wait_key(30).unwrap() == 'q' as i32 {
                                break;
                            }
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
    //
    //
    #[test]
    fn matching_test() -> opencv::Result<()> {
        let mut cap = videoio::VideoCapture::from_file("src/test/unit/infrostructure/camera/video_test.mp4", videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        println!("Не удалось открыть видеофайл");
        return Ok(());
    }
    highgui::named_window("Video", highgui::WINDOW_AUTOSIZE)?;
    let mask = imgcodecs::imread("src/test/unit/scan/fast/test3.jpg",imgcodecs::IMREAD_COLOR).unwrap();
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut gray_frame = gray_frame.roi(core::Rect::new(400,10,300,150))?;
        let mut canny_edges = Mat::default();
        imgproc::canny(&gray_frame,&mut canny_edges, 1.0, 255.0, 3, true)?;
        let mut ots = Mat::default();
        let mut thresh = Mat::default();
        imgproc::threshold(&gray_frame, &mut ots, 150.0, 255.0, imgproc::THRESH_OTSU)?;
        imgproc::threshold(&gray_frame, &mut thresh, 150.0, 255.0, imgproc::THRESH_BINARY_INV)?;
        let mut grad_x = Mat::default();
        let mut grad_y = Mat::default();
        let mut abs_grad_x = Mat::default();
        let mut abs_grad_y = Mat::default();
        let mut grad = Mat::default();
        let mut lines = core::Vector::<Vec4i>::new();
        imgproc::sobel(&gray_frame,&mut grad_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        imgproc::sobel(&gray_frame,&mut grad_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
        core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;
        core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut grad, -1)?;
        let mut orb = features2d::ORB::create_def()?;
        let mut kp1 = core::Vector::<KeyPoint>::new();
        let mut kp2 = core::Vector::<KeyPoint>::new();
        let mut des1 = Mat::default();
        let mut des2 = Mat::default();
        orb.detect_and_compute(&grad, &Mat::default(), &mut kp1, &mut des1, false)?;
        orb.detect_and_compute(&mask, &Mat::default(), &mut kp2, &mut des2, false)?;
        let mut bf = BFMatcher::new(NORM_HAMMING, true)?;
        let mut matches = core::Vector::<DMatch>::new();
        bf.match_(&des1, &mut matches, &des2)?;
        let mut img = Mat::default();
        //features2d::draw_keypoints(&grad, &kp1, &mut img, core::Scalar::new(0.0,255.0,0.0,0.0), features2d::DrawMatchesFlags::DEFAULT)?;
        features2d::draw_matches(&grad, &kp1, &mask, &kp2, &matches,&mut img,core::Scalar::new(0.0,255.0,0.0,0.0),core::Scalar::new(0.0,0.0,255.0,0.0), &core::Vector::<i8>::new(),features2d::DrawMatchesFlags::DEFAULT)?;
        //features2d::draw_matches(&grad, &kp1, &mask, &kp2, &matches,&mut img,core::Scalar::new(0.0,255.0,0.0,0.0),core::Scalar::new(0.0,0.0,255.0,0.0), &core::Vector::<i8>::new(),features2d::DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS)?;
        highgui::imshow("Video", &img)?;
        if highgui::wait_key(30)? == 'q' as i32 {
            break;
        }
    }
    cap.release()?;
    highgui::destroy_all_windows()?;
    Ok(())
    }
}

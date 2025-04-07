#[cfg(test)]

mod fast_scan {
    use std::{net::SocketAddr, os::linux::net::SocketAddrExt, sync::Once, time::{Duration, Instant}};
    use sal_sync::services::conf::conf_tree::ConfTree;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use opencv::{
        core::{self, DMatch, KeyPoint, Size, Vec4i, NORM_HAMMING}, features2d::{self, BFMatcher}, highgui, imgcodecs::{self, imread, IMREAD_COLOR}, imgproc, prelude::*, videoio::{self, VideoWriter}, Result
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
    /// Testing fast scan
    #[test]
    fn grad_one_frame() -> opencv::Result<()> {
        let img = imgcodecs::imread("src/test/unit/scan/fast/test_photo2.jpg",imgcodecs::IMREAD_COLOR).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE);
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&img, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        //let mut gray_frame = gray_frame.roi(core::Rect::new(400,10,300,150))?;
        let mut grad_x = Mat::default();
        let mut grad_y = Mat::default();
        let mut abs_grad_x = Mat::default();
        let mut abs_grad_y = Mat::default();
        let mut grad = Mat::default();
        imgproc::sobel(&gray_frame,&mut grad_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        imgproc::sobel(&gray_frame,&mut grad_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
        core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;
        core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut grad, -1)?;
        highgui::imshow("img", &grad);
        highgui::wait_key(0);
        highgui::destroy_all_windows();
        Ok(())
    }
    // 
    //
    #[test]
    fn method_compare() -> opencv::Result<()> {
        let img = imgcodecs::imread("src/test/unit/scan/fast/test_photo.jpg",imgcodecs::IMREAD_COLOR).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE);
        let kernel = imgproc::get_structuring_element(imgproc::MORPH_RECT, core::Size::new(3,3), core::Point::new(-1,-1))?;
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(&img, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut grad_x = Mat::default();
        let mut grad_y = Mat::default();
        let mut abs_grad_x = Mat::default();
        let mut abs_grad_y = Mat::default();
        let mut grad = Mat::default();
        let mut edges = Mat::default();
        imgproc::sobel(&gray_frame,&mut grad_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        imgproc::sobel(&gray_frame,&mut grad_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
        core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;
        core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut grad, -1)?;
        imgproc::canny_def(&grad, &mut edges, 100., 200.);
        highgui::imshow("img", &edges);
        highgui::wait_key(0);
        highgui::destroy_all_windows();
        Ok(())  
    }
    //
    //
    #[test]
    fn contours() -> opencv::Result<()> {
        let img = imgcodecs::imread("src/test/unit/scan/fast/test_photo2.jpg",imgcodecs::IMREAD_COLOR).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE);
        let kernel = imgproc::get_structuring_element(imgproc::MORPH_RECT, core::Size::new(3,3), core::Point::new(-1,-1))?;
        let time = Instant::now();
        let mut gray_frame = Mat::default();
        let mut otsu_thresh = Mat::default();
        let mut opened = Mat::default();
        let mut edges = Mat::default();
        imgproc::cvt_color(&img, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        imgproc::threshold(&gray_frame, &mut otsu_thresh, 100., 255., imgproc::THRESH_OTSU)?;
        imgproc::morphology_ex(&otsu_thresh, &mut opened, imgproc::MORPH_OPEN, &kernel, core::Point::new(-1,-1), 3, core::BORDER_CONSTANT, imgproc::morphology_default_border_value()?)?;
        core::bitwise_not_def(&opened.clone(), &mut opened);
        imgproc::canny_def(&opened, &mut edges, 100., 200.);
        let elapsed = time.elapsed();
        println!("Elapsed time: {}", elapsed.as_millis());
        highgui::imshow("img", &edges);
        highgui::wait_key(0);
        highgui::destroy_all_windows();
        Ok(())
    }
    //
    //
    #[test]
    fn grad_save() -> opencv::Result<()> {
        let mut cap = videoio::VideoCapture::from_file("src/test/unit/infrostructure/camera/video_test.mp4", videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            println!("Не удалось открыть видеофайл");
            return Ok(());
        }
        let fps = cap.get(videoio::CAP_PROP_FPS)?;
        let fourcc = VideoWriter::fourcc('X', '2', '6', '4')?;
        let mut writer = VideoWriter::new("src/test/unit/scan/fast/output.mp4", fourcc, fps, core::Size::new(900,450), false)?;
        if !writer.is_opened()?{
            panic!("writer not opened!");
        }
        loop {
            let mut frame = Mat::default();
            cap.read(&mut frame)?;
            if frame.empty() {
                break;
            }
            let mut gray_frame = Mat::default();
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
            let mut gray_frame = gray_frame.roi(core::Rect::new(400,10,300,150))?;
            let mut grad_x = Mat::default();
            let mut grad_y = Mat::default();
            let mut abs_grad_x = Mat::default();
            let mut abs_grad_y = Mat::default();
            let mut grad = Mat::default();
            imgproc::sobel(&gray_frame,&mut grad_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
            imgproc::sobel(&gray_frame,&mut grad_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
            core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
            core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;
            core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut grad, -1)?;
            let mut resized = Mat::default();
            imgproc::resize(&grad, &mut resized, core::Size::new(900, 450), 0.0, 0.0, imgproc::INTER_LINEAR)?;
            writer.write(&resized)?;
        }
        writer.release()?;
        Ok(())
    }
    //
    //
    #[test]
    fn grad_time() -> opencv::Result<()> {

        let mut cap = videoio::VideoCapture::from_file("src/test/unit/infrostructure/camera/video_test.mp4", videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            println!("Не удалось открыть видеофайл");
            return Ok(());
        }
        highgui::named_window("Video", highgui::WINDOW_AUTOSIZE)?;
        let mut max_time = 0;
        let mut min_time = 1000;
        let mut mid_time = 0;
        let mut frame_count = 0;
        loop {
            let time = Instant::now();
            let mut frame = Mat::default();
            cap.read(&mut frame)?;
            if frame.empty() {
                break;
            }
            let mut gray_frame = Mat::default();
            imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
            let mut gray_frame = gray_frame.roi(core::Rect::new(400,10,300,150))?;
            let mut grad_x = Mat::default();
            let mut grad_y = Mat::default();
            let mut abs_grad_x = Mat::default();
            let mut abs_grad_y = Mat::default();
            let mut grad = Mat::default();
            imgproc::sobel(&gray_frame,&mut grad_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
            imgproc::sobel(&gray_frame,&mut grad_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
            core::convert_scale_abs(&grad_x, &mut abs_grad_x, 1.0, 0.0)?;
            core::convert_scale_abs(&grad_y, &mut abs_grad_y, 1.0, 0.0)?;
            core::add_weighted(&abs_grad_x, 0.5, &abs_grad_y, 0.5, 0.0, &mut grad, -1)?;
            let elapsed = time.elapsed();
            println!("Elapsed time: {}", elapsed.as_millis());
            if elapsed.as_millis() > max_time {
                max_time = elapsed.as_millis();
            }
            if elapsed.as_millis() < min_time {
                min_time = elapsed.as_millis();
            }
            frame_count += 1;
            mid_time += elapsed.as_millis();
        }
        mid_time = mid_time/frame_count;
        println!("Max time: {max_time}");
        println!("Min time: {min_time}");
        println!("Mid time: {mid_time}");
        println!("Frames in video: {frame_count}");

        Ok(())
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
            let mut grad_x = Mat::default();
            let mut grad_y = Mat::default();
            let mut abs_grad_x = Mat::default();
            let mut abs_grad_y = Mat::default();
            let mut grad = Mat::default();
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

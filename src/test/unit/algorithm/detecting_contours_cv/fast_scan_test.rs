#[cfg(test)]

mod fast_scan {
    use std::{sync::Once, time::Instant};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use opencv::{
        core::{self, DMatch, KeyPoint, NORM_HAMMING}, features2d::{self, BFMatcher}, highgui, imgcodecs::{self}, imgproc, prelude::*, videoio::{self, VideoWriter}
    };
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
    fn grad_one_frame() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "grad_one_frame";
        log::debug!("\n{}", dbg);
        let img = imgcodecs::imread(
            "src/test/unit/algorithm/detecting_contours/testing_files/rope_0.jpeg",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(
            &img,
            &mut gray_frame,
            imgproc::COLOR_BGR2GRAY,
            0,
        ).unwrap();
        let mut grad_x = Mat::default();
        let mut grad_y = Mat::default();
        let mut abs_grad_x = Mat::default();
        let mut abs_grad_y = Mat::default();
        let mut grad = Mat::default();
        imgproc::sobel(
            &gray_frame,
            &mut grad_x,
            core::CV_16S,
            1,
            0,
            1,
            1.,
            0.,
            core::BORDER_DEFAULT,
        ).unwrap();
        imgproc::sobel(
            &gray_frame,
            &mut grad_y,
            core::CV_16S,
            0,
            1,
            1,
            1.,
            0.,
            core::BORDER_DEFAULT,
        ).unwrap();
        core::convert_scale_abs(
            &grad_x,
            &mut abs_grad_x,
            1.,
            0.,
        ).unwrap();
        core::convert_scale_abs(
            &grad_y,
            &mut abs_grad_y,
            1.,
            0.,
        ).unwrap();
        core::add_weighted(
            &abs_grad_x,
            0.5,
            &abs_grad_y,
            0.5,
            0.,
            &mut grad,
            -1
        ).unwrap();
        highgui::imshow("img", &abs_grad_y).unwrap();
        highgui::wait_key(0).unwrap();
        highgui::destroy_all_windows().unwrap();
    }
    // 
    //
    #[test]
    fn method_compare() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "method_compare";
        log::debug!("\n{}", dbg);
        let img = imgcodecs::imread(
            "src/test/unit/algorithm/detecting_contours/testing_files/rope_0.jpeg",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_RECT,
            core::Size::new(3, 3),
            core::Point::new(-1, -1),
        ).unwrap();
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(
            &img,
            &mut gray_frame,
            imgproc::COLOR_BGR2GRAY,
            0,
        ).unwrap();
        let mut grad_x = Mat::default();
        let mut grad_y = Mat::default();
        let mut abs_grad_x = Mat::default();
        let mut abs_grad_y = Mat::default();
        let mut grad = Mat::default();
        let mut edges = Mat::default();
        imgproc::sobel(
            &gray_frame,
            &mut grad_x,
            core::CV_16S,
            1,
            0,
            1,
            1.,
            0.,
            core::BORDER_DEFAULT,
        ).unwrap();
        imgproc::sobel(
            &gray_frame,
            &mut grad_y,
            core::CV_16S,
            0,
            1,
            1,
            1.,
            0.,
            core::BORDER_DEFAULT,
        ).unwrap();
        core::convert_scale_abs(
            &grad_x,
            &mut abs_grad_x,
            1.,
            0.,
        ).unwrap();
        core::convert_scale_abs(
            &grad_y,
            &mut abs_grad_y,
            1.,
            0.,
        ).unwrap();
        core::add_weighted(
            &abs_grad_x,
            0.5,
            &abs_grad_y,
            0.5,
            0.,
            &mut grad,
            -1,
        ).unwrap();
        imgproc::canny_def(
            &grad,
            &mut edges,
            100.,
            200.
        ).unwrap();
        highgui::imshow("img", &grad).unwrap();
        highgui::wait_key(0).unwrap();
        highgui::destroy_all_windows().unwrap();
    }
    //
    //
    #[test]
    fn contours() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "contours";
        log::debug!("\n{}", dbg);
        let img = imgcodecs::imread(
            "src/test/unit/algorithm/detecting_contours/testing_files/rope_31.jpeg",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
        let kernel = imgproc::get_structuring_element(
            imgproc::MORPH_RECT,
            core::Size::new(3, 3),
            core::Point::new(-1, -1),
        ).unwrap();
        let time = Instant::now();
        let mut gray_frame = Mat::default();
        let mut otsu_thresh = Mat::default();
        let mut opened = Mat::default();
        let mut edges = Mat::default();
        let mut blurred = Mat::default();
        imgproc::cvt_color(
            &img,
            &mut gray_frame,
            imgproc::COLOR_BGR2GRAY,
            0,
        ).unwrap();
        imgproc::gaussian_blur_def(
            &gray_frame,
            &mut blurred,
            core::Size::new(21,21),
            5.,
        ).unwrap();
        imgproc::threshold(
            &blurred,
            &mut otsu_thresh,
            100.,
            255.,
            imgproc::THRESH_OTSU,
        ).unwrap();
        imgproc::morphology_ex(
            &otsu_thresh,
            &mut opened,
            imgproc::MORPH_OPEN,
            &kernel,
            core::Point::new(-1, -1),
            3,
            core::BORDER_CONSTANT,
            imgproc::morphology_default_border_value().unwrap(),
        ).unwrap();
        core::bitwise_not_def(&opened.clone(), &mut opened).unwrap();
        imgproc::canny_def(
            &opened,
            &mut edges,
            100.,
            200.,
        ).unwrap();
        let elapsed = time.elapsed();
        log::debug!("Elapsed time: {}", elapsed.as_millis());
        highgui::imshow("img", &gray_frame).unwrap();
        highgui::wait_key(0).unwrap();
        highgui::destroy_all_windows().unwrap();
    }
    //
    //
    #[test]
    fn grad_save() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "grad_save";
        log::debug!("\n{}", dbg);
        let path ="src/test/unit/infrostructure/camera/video_test.mp4";
        let mut cap = videoio::VideoCapture::from_file(
            path,
            videoio::CAP_ANY,
        ).unwrap();
        match cap.is_opened(){
            Ok(_) => {
                let fps = cap.get(videoio::CAP_PROP_FPS).unwrap();
                let fourcc = VideoWriter::fourcc('X', '2', '6', '4').unwrap();
                let mut writer = VideoWriter::new(
                    "src/test/unit/scan/fast/output.mp4",
                    fourcc,
                    fps,
                    core::Size::new(900, 450),
                    false,
                ).unwrap();
                if !writer.is_opened().unwrap() {
                    panic!("writer not opened!");
                }
                loop {
                    let mut frame = Mat::default();
                    cap.read(&mut frame).unwrap();
                    if frame.empty() {
                        break;
                    }
                    let mut gray_frame = Mat::default();
                    imgproc::cvt_color(
                        &frame,
                        &mut gray_frame,
                        imgproc::COLOR_BGR2GRAY,
                        0,
                    ).unwrap();
                    let gray_frame = gray_frame.roi(core::Rect::new(
                        400,
                        10,
                        300,
                        150,
                    )).unwrap();
                    let mut grad_x = Mat::default();
                    let mut grad_y = Mat::default();
                    let mut abs_grad_x = Mat::default();
                    let mut abs_grad_y = Mat::default();
                    let mut grad = Mat::default();
                    imgproc::sobel(
                        &gray_frame,
                        &mut grad_x,
                        core::CV_16S,
                        1,
                        0,
                        1,
                        1.,
                        0.,
                        core::BORDER_DEFAULT,
                    ).unwrap();
                    imgproc::sobel(
                        &gray_frame,
                        &mut grad_y,
                        core::CV_16S,
                        0,
                        1,
                        1,
                        1.,
                        0.,
                        core::BORDER_DEFAULT,
                    ).unwrap();
                    core::convert_scale_abs(
                        &grad_x,
                        &mut abs_grad_x,
                        1.,
                        0.
                    ).unwrap();
                    core::convert_scale_abs(
                        &grad_y,
                        &mut abs_grad_y,
                        1.,
                        0.
                    ).unwrap();
                    core::add_weighted(
                        &abs_grad_x,
                        0.5,
                        &abs_grad_y,
                        0.5,
                        0.,
                        &mut grad,
                        -1,
                    ).unwrap();
                    let mut resized = Mat::default();
                    imgproc::resize(
                        &grad,
                        &mut resized,
                        core::Size::new(900, 450),
                        0.,
                        0.,
                        imgproc::INTER_LINEAR,
                    ).unwrap();
                    writer.write(&resized).unwrap();
                }
                writer.release().unwrap();
            }
            Err(err) => panic!("{} | Can't open file '{path}' error: {:?}", dbg, err),
        }
    }
    //
    //
    #[test]
    fn grad_time() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "grad_time";
        log::debug!("\n{}", dbg);
        let path = "src/test/unit/infrostructure/camera/video_test.mp4";
        let mut cap = videoio::VideoCapture::from_file(
            path,
            videoio::CAP_ANY,
        ).unwrap();
        match cap.is_opened(){
            Ok(_) => {
                highgui::named_window("Video", highgui::WINDOW_AUTOSIZE).unwrap();
                let mut max_time = 0;
                let mut min_time = 1000;
                let mut mid_time = 0;
                let mut frame_count = 0;
                loop {
                    let time = Instant::now();
                    let mut frame = Mat::default();
                    cap.read(&mut frame).unwrap();
                    if frame.empty() {
                        break;
                    }
                    let mut gray_frame = Mat::default();
                    imgproc::cvt_color(
                        &frame,
                        &mut gray_frame,
                        imgproc::COLOR_BGR2GRAY,
                        0,
                    ).unwrap();
                    let gray_frame = gray_frame.roi(core::Rect::new(400,10,300,150)).unwrap();
                    let mut grad_x = Mat::default();
                    let mut grad_y = Mat::default();
                    let mut abs_grad_x = Mat::default();
                    let mut abs_grad_y = Mat::default();
                    let mut grad = Mat::default();
                    imgproc::sobel(
                        &gray_frame,
                        &mut grad_x,
                        core::CV_16S,
                        1,
                        0,
                        1,
                        1.,
                        0.,
                        core::BORDER_DEFAULT,
                    ).unwrap();
                    imgproc::sobel(
                        &gray_frame,
                        &mut grad_y,
                        core::CV_16S,
                        0,
                        1,
                        1,
                        1.,
                        0.,
                        core::BORDER_DEFAULT,
                    ).unwrap();
                    core::convert_scale_abs(
                        &grad_x,
                        &mut abs_grad_x,
                        1.,
                        0.,
                    ).unwrap();
                    core::convert_scale_abs(
                        &grad_y,
                        &mut abs_grad_y,
                        1.,
                        0.,
                    ).unwrap();
                    core::add_weighted(
                        &abs_grad_x,
                        0.5,
                        &abs_grad_y,
                        0.5,
                        0.,
                        &mut grad,
                        -1,
                    ).unwrap();
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
            }
            Err(err) => panic!("{} | Can't open file '{path}' error: {:?}", dbg, err),
        }
    }
    //
    //
    #[test]
    fn matching_test() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        let dbg = "matching_test";
        log::debug!("\n{}", dbg);
        let mut cap = videoio::VideoCapture::from_file(
            "src/test/unit/infrostructure/camera/video_test.mp4",
            videoio::CAP_ANY,
        ).unwrap();
        if !cap.is_opened().unwrap() {
            println!("Не удалось открыть видеофайл");
            return;
        }
        highgui::named_window("Video", highgui::WINDOW_AUTOSIZE).unwrap();
        let mask = imgcodecs::imread(
            "src/test/unit/scan/fast/test3.jpg",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        loop {
            let mut frame = Mat::default();
            cap.read(&mut frame).unwrap();
            if frame.empty() {
                break;
            }
            let mut gray_frame = Mat::default();
            imgproc::cvt_color(
                &frame,
                &mut gray_frame,
                imgproc::COLOR_BGR2GRAY,
                0
            ).unwrap();
            let gray_frame = gray_frame.roi(core::Rect::new(
                400,
                10,
                300,
                150,
            )).unwrap();
            let mut grad_x = Mat::default();
            let mut grad_y = Mat::default();
            let mut abs_grad_x = Mat::default();
            let mut abs_grad_y = Mat::default();
            let mut grad = Mat::default();
            imgproc::sobel(
                &gray_frame,
                &mut grad_x,
                core::CV_16S,
                1,
                0,
                1,
                1.,
                0.,
                core::BORDER_DEFAULT,
            ).unwrap();
            imgproc::sobel(
                &gray_frame,
                &mut grad_y,
                core::CV_16S,
                0,
                1,
                1,
                1.,
                0.,
                core::BORDER_DEFAULT,
            ).unwrap();
            core::convert_scale_abs(
                &grad_x,
                &mut abs_grad_x,
                1.,
                 0.,
                ).unwrap();
            core::convert_scale_abs(
                &grad_y,
                &mut abs_grad_y,
                1.,
                 0.,
                ).unwrap();
            core::add_weighted(
                &abs_grad_x,
                0.5,
                &abs_grad_y,
                0.5,
                0.,
                &mut grad,
                -1,
            ).unwrap();
            let mut orb = features2d::ORB::create_def().unwrap();
            let mut kp1 = core::Vector::<KeyPoint>::new();
            let mut kp2 = core::Vector::<KeyPoint>::new();
            let mut des1 = Mat::default();
            let mut des2 = Mat::default();
            orb.detect_and_compute(
                &grad,
                &Mat::default(),
                &mut kp1,
                &mut des1,
                false,
            ).unwrap();
            orb.detect_and_compute(
                &mask,
                &Mat::default(),
                &mut kp2,
                &mut des2,
                false,
            ).unwrap();
            let mut bf = BFMatcher::new(NORM_HAMMING, true).unwrap();
            let mut matches = core::Vector::<DMatch>::new();
            bf.match_(
                &des1,
                &mut matches,
                &des2,
            ).unwrap();
            let mut img = Mat::default();
            //features2d::draw_keypoints(&grad, &kp1, &mut img, core::Scalar::new(0.0,255.0,0.0,0.0), features2d::DrawMatchesFlags::DEFAULT)?;
            features2d::draw_matches(
                &grad,
                &kp1,
                &mask,
                &kp2,
                &matches,
                &mut img,
                core::Scalar::new(0., 255., 0., 0.),
                core::Scalar::new(0., 0., 255., 0.),
                &core::Vector::<i8>::new(),
                features2d::DrawMatchesFlags::DEFAULT,
            ).unwrap();
            //features2d::draw_matches(&grad, &kp1, &mask, &kp2, &matches,&mut img,core::Scalar::new(0.0,255.0,0.0,0.0),core::Scalar::new(0.0,0.0,255.0,0.0), &core::Vector::<i8>::new(),features2d::DrawMatchesFlags::NOT_DRAW_SINGLE_POINTS)?;
            highgui::imshow("Video", &img).unwrap();
            if highgui::wait_key(30).unwrap() == 'q' as i32 {
                break;
            }
        }
        cap.release().unwrap();
        highgui::destroy_all_windows().unwrap();
    }
}

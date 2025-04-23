#[cfg(test)]

mod graham_scan {
    use std::{sync::Once, time::{Duration, Instant}};
    use photon_rs::native::open_image;
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use image::{RgbaImage, Rgba};
    use imageproc::drawing::draw_line_segment_mut;
    use crate::{algorithm::graham_scan::graham_scan::GrahamScan, domain::eval::eval::Eval};
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
    /// Testing `eval` method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("graham_scan");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(20));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                "src/test/unit/algorithm/detecting_contours/output_files/out_1.jpeg"
            )
        ];
        for (step, img_path) in test_data {
            let result = GrahamScan::new(
                open_image(img_path).expect("Error")
            ).eval(());
            // draw lines between convex hull
            let mut image = RgbaImage::new(1940, 1500);
            for pixel in image.pixels_mut() {
                *pixel = Rgba([0, 0, 0, 0]);
            }
            for i in 0..result.result.len() - 1 {
                let start = &result.result[i];
                let end = &result.result[i + 1];
                let color = Rgba([start.rgba[0], start.rgba[1], start.rgba[2], start.rgba[3]]);
                draw_line_segment_mut(
                    &mut image,
                    (start.x as f32, start.y as f32),
                    (end.x as f32, end.y as f32),
                    color,
                );
            }
            image.save(&format!("src/test/unit/algorithm/graham_scan/output_files/out_{}.png",step)).unwrap();
            //assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

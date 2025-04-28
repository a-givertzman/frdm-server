#[cfg(test)]

mod edge_detection_test {
    use std::{sync::Once, time::{Duration, Instant}};
    use opencv::{core::{Mat, MatExprTraitConst, MatTrait, MatTraitConst, Vec3b}, highgui, imgcodecs};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{domain::{dbg::dbgid::DbgId, eval::eval::Eval, graham::{dot::Dot, find_start::{FindStart, FindStartCtx}, sort::Sort}}, infrostructure::camera::pimage::PImage, scan::edge_detection::{self, EdgeDetection}};
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
    /// Testing such functionality / behavior
    #[test]
    fn edge_detection() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
        test_duration.run().unwrap();
        let img = imgcodecs::imread(
            "src/test/unit/scan/edge_detection/test_photo2.png",
            imgcodecs::IMREAD_GRAYSCALE,
        ).unwrap();
        let mut edges = EdgeDetection::new(PImage::new(img.clone()));
        let mut img_of_edges = imgcodecs::imread(
            "src/test/unit/scan/edge_detection/test_photo2.png",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        for dot in &edges.upper_edge {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]); // BGR: красный
            }
        }

        // Рисуем нижний край (зеленым)
        for dot in &edges.lower_edge {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]); // BGR: зеленый
            }
        }
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE);
        highgui::imshow("img", &img_of_edges);
        highgui::wait_key(0);
        highgui::destroy_all_windows();
        test_duration.exit();
    }
}

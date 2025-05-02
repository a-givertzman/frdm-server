#[cfg(test)]

mod edge_detection_test {
    use std::{sync::Once, time::Duration};
    use opencv::{core::{MatTrait, Vec3b}, highgui, imgcodecs};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{domain::{dbg::dbgid::DbgId, eval::eval::Eval}, infrostructure::camera::pimage::PImage, scan::edge_detection::EdgeDetection};
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
        let edges = EdgeDetection::new(PImage::new(img.clone())).eval(());
        let mut img_of_edges = imgcodecs::imread(
            "src/test/unit/scan/edge_detection/test_photo2.png",
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        for dot in &edges.upper_edge {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
            }
        }
        for dot in &edges.lower_edge {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 255, 0]);
            }
        }
        highgui::named_window("img", highgui::WINDOW_AUTOSIZE).unwrap();
        highgui::imshow("img", &img_of_edges).unwrap();
        highgui::wait_key(0).unwrap();
        highgui::destroy_all_windows().unwrap();
        test_duration.exit();
    }
}

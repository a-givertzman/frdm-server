#[cfg(test)]

mod edge_detection_test {
    use std::{sync::Once, time::Duration};
    use opencv::{core::{Mat, MatTrait, MatTraitConst, Vec3b}, highgui, imgcodecs, imgproc};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{domain::{eval::eval::Eval, graham::dot::Dot}, infrostructure::camera::pimage::PImage, scan::edge_detection::EdgeDetection};
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
    /// Visualize upper and lower edges on test image
    fn edge_visualization_img() {
        let path = "src/test/unit/scan/edge_detection/test_photo2.png";
        let img = imgcodecs::imread(
            path,
            imgcodecs::IMREAD_GRAYSCALE,
        ).unwrap();
        let edges = EdgeDetection::new(PImage::new(img.clone())).eval(());
        let mut img_of_edges = imgcodecs::imread(
            path,
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
    }
    ///
    /// Visualize upper and lower edges on test matrix
    fn edge_visualization_matrix(matrix: [[u8; 6]; 6]) {
        let matrix: Vec<Vec<u8>> = matrix.iter()
        .map(|row| row.iter().map(|&x| x * 255).collect())
        .collect();
        let img = Mat::from_slice_2d(&matrix).unwrap();
        let mut img_of_edges = Mat::default();
        imgproc::cvt_color(&img, &mut img_of_edges, imgproc::COLOR_GRAY2BGR, 0).unwrap();
        let edges = EdgeDetection::new(PImage::new(img)).eval(());
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
    }
    #[test]
    fn edge_detection() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                PImage::new( Mat::from_slice_2d(&MATRIX1).unwrap()),
                (
                    &[0,1, 1,0, 2,0, 3,1, 4,0, 5,0],
                    &[0,5, 1,4, 2,5, 3,5, 4,5, 5,4]
                )
            ),
            (
                2,
                PImage::new( Mat::from_slice_2d(&MATRIX2).unwrap()),
                (
                    &[0,2, 1,1, 2,0, 3,1, 4,0, 5,1],
                    &[0,3, 1,4, 2,4, 3,5, 4,4, 5,3]
                )
            )
        ];
        for (step, img, target) in test_data {
            let edges = EdgeDetection::new(img).eval(());
            let result = (edges.upper_edge, edges.lower_edge);
            let target_upper: Vec<Dot<isize>> = target.0.chunks(2).map(|d| Dot { x: d[0] as isize, y: d[1] as isize }).collect();
            let target_lower: Vec<Dot<isize>> = target.1.chunks(2).map(|d| Dot { x: d[0] as isize, y: d[1] as isize }).collect();
            let target = (target_upper, target_lower);
            assert!(
                result == target,
                "step {} \nresult upper: {:?}\ntarget upper: {:?} \nresult lower: {:?}\ntarget lower: {:?}",
                step,
                result.0,
                target.0,
                result.1,
                target.1
            );
        }
        test_duration.exit();
        // to visualize matrix use:
        // edge_visualization_matrix(MATRIX2);
        static MATRIX1: [[u8; 6]; 6] = [
            [0, 1, 1, 0, 1, 1],
            [1, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 1],
            [1, 0, 1, 1, 1, 0],
        ];
        static MATRIX2: [[u8; 6]; 6] = [
            [0, 0, 1, 0, 1, 0],
            [0, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1],
            [0, 1, 1, 1, 1, 0],
            [0, 0, 0, 1, 0, 0],
        ];
    }
}
//
//
// Questions:
// in camera and arena used different dbgs
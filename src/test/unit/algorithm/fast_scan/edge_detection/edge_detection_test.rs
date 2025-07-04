#[cfg(test)]

mod edge_detection_test {
    use std::{sync::Once, time::Duration};
    use opencv::{core::{Mat, MatTrait, Vec3b}, highgui, imgcodecs, imgproc};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{Context, ContextRead, EdgeDetection, EdgeDetectionCtx, InitialCtx, InitialPoints, Side}, domain::{Dot, Eval, Image}};
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
        let ctx = EdgeDetection::new(FakePassImg::new(Image::with(img.clone()))).eval(()).unwrap();
        let edges: &EdgeDetectionCtx = ctx.read();
        let mut img_of_edges = imgcodecs::imread(
            path,
            imgcodecs::IMREAD_COLOR,
        ).unwrap();
        for dot in edges.result.get(Side::Upper) {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
            }
        }
        for dot in edges.result.get(Side::Lower) {
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
        let ctx = EdgeDetection::new(FakePassImg::new(Image::with(img))).eval(()).unwrap();
        let edges: &EdgeDetectionCtx = ctx.read();
        for dot in edges.result.get(Side::Upper) {
            if dot.x >= 0 && dot.y >= 0 {
                let x = dot.x as i32;
                let y = dot.y as i32;
                *img_of_edges.at_2d_mut::<Vec3b>(y, x).unwrap() = Vec3b::from_array([0, 0, 255]);
            }
        }
        for dot in edges.result.get(Side::Lower) {
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
    /// Testing EdgeDetection.eval
    #[test]
    fn edge_detection() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        //
        // to visualize matrix use:
        let visualize_matrix = false;
        init_once();
        init_each();
        let dbg = Dbg::own("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
        test_duration.run().unwrap();
        fn into_dots(dots: &[usize]) -> Vec<Dot<usize>> {
            dots.chunks(2).map(|d| d.into()).collect()
        }
        let test_data: [(i32, Image, Result<EdgeDetectionCtx, Error>); 2] = [
            (
                1,
                Image::with( Mat::from_slice_2d(&MATRIX1).unwrap()),
                Ok(EdgeDetectionCtx {
                    result: InitialPoints::new(
                        into_dots(&[0,1, 1,0, 2,0, 3,1, 4,0, 5,0]),
                        into_dots(&[0,5, 1,4, 2,5, 3,5, 4,5, 5,4]),
                    )
                }),
            ),
            (
                2,
                Image::with( Mat::from_slice_2d(&MATRIX2).unwrap()),
                Ok(EdgeDetectionCtx {
                    result: InitialPoints::new(
                        into_dots(&[0,2, 1,1, 2,0, 3,1, 4,0, 5,1]),
                        into_dots(&[0,3, 1,4, 2,4, 3,5, 4,4, 5,3]),
                    )
                }),
            )
        ];
        for (step, img, target) in test_data {
            let result = EdgeDetection::new(
                FakePassImg::new(img)
            )
            .eval(())
            .map(|ctx| {
                let result: &EdgeDetectionCtx = ctx.read();
                result.to_owned()
            });
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(_), Err(_)) => {},
            }
        }
        test_duration.exit();
        if visualize_matrix {
            edge_visualization_matrix(MATRIX2);
        }
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
    ///
    /// Fake implements `Eval` for testing [EdgeDetection]
    struct FakePassImg {
        img: Image,
    }
    impl FakePassImg{
        pub fn new(img: Image) -> Self {
            Self { 
                img,
            }
        }
    }
    //
    //
    impl Eval<(), Result<Context, Error>> for FakePassImg {
        fn eval(&self, _: ()) -> Result<Context, Error> {
            Ok(
                Context::new(
                    InitialCtx::new(self.img.clone()),
                ),
            )
        }
    }
}

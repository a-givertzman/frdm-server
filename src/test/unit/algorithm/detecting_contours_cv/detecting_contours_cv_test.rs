#[cfg(test)]

mod detecting_contours_cv {
    use std::{sync::Once, time::Duration};
    use opencv::{highgui, imgcodecs};
    use photon_rs::native::{open_image, save_image};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::{DetectingContours, DetectingContoursCv}, domain::eval::eval::Eval, infrostructure::arena::image::Image};
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
    /// Testing 'eval' method
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("detecting_contours");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                "src/test/unit/algorithm/detecting_contours/testing_files/rope_0.jpeg",
            ),
        ];
        for (step,img_path) in test_data {
            let path = img_path;
            let img = imgcodecs::imread(
                path,
                imgcodecs::IMREAD_COLOR,
            ).unwrap();
            let result = DetectingContoursCv::new(FakePassImg::new(Image::with(img))).eval(()).unwrap();
            highgui::named_window("detected_contours_cv", highgui::WINDOW_NORMAL).unwrap();
            highgui::imshow("contours", &result.mat).unwrap();
            highgui::wait_key(0).unwrap();
            highgui::destroy_all_windows().unwrap();
        }
        test_duration.exit();
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
    impl Eval<(), Result<Image, Error>> for FakePassImg {
        fn eval(&mut self, _: ()) -> Result<Image, Error> {
            Ok(self.img.clone())
        }
    }
}
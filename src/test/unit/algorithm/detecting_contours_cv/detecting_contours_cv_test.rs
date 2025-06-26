#[cfg(test)]

mod detecting_contours_cv {
    use std::{sync::Once, time::Duration};
    use opencv::{highgui, imgcodecs};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        algorithm::{Context, ContextRead, DetectingContoursCv, DetectingContoursCvCtx, InitialCtx},
        domain::{Eval, Image},
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
        for (_step,img_path) in test_data {
            let path = img_path;
            let img = imgcodecs::imread(
                path,
                imgcodecs::IMREAD_COLOR,
            ).unwrap();
            let ctx = DetectingContoursCv::new(FakePassImg::new(Image::with(img))).eval(()).unwrap();
            let result: &DetectingContoursCvCtx = ctx.read();
            highgui::named_window("detected_contours_cv", highgui::WINDOW_NORMAL).unwrap();
            highgui::imshow("contours", &result.result.mat).unwrap();
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
    impl Eval<(), Result<Context, Error>> for FakePassImg {
        fn eval(&self, _: ()) -> Result<Context, Error> {
            Ok(
                Context::new(
                    InitialCtx::new(self.img.clone())
                )
            )
        }
    }
}
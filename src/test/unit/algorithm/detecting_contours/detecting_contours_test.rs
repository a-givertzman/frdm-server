#[cfg(test)]

mod detecting_contours {
    use std::{sync::Once, time::Duration};
    use photon_rs::native::{open_image, save_image};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{algorithm::DetectingContours, domain::Eval};
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
            // (
            //     2,
            //     "src/test/unit/algorithm/detecting_contours/testing_files/rope_1.jpeg",
            // ),
            // (
            //     3,
            //     "src/test/unit/algorithm/detecting_contours/testing_files/rope_2.jpg",
            // ),
            // (
            //     4,
            //     "src/test/unit/algorithm/detecting_contours/testing_files/rope_3.jpeg",
            // ),
        ];
        for (step,img_path) in test_data {
            let result = DetectingContours::new(
                open_image(img_path).expect("Error")
            )
            .eval(());
            let _ = save_image(result.result, &format!("src/test/unit/algorithm/detecting_contours/output_files/out_{}.jpeg",step));
        }
        test_duration.exit();
    }
}

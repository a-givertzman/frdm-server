#[cfg(test)]

mod graham {
    use std::{sync::Once, time::{Duration, Instant}};
    use photon_rs::native::open_image;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use sal_core::dbg::Dbg;

    use crate::{algorithm::graham::graham::Graham, domain::eval::eval::Eval};
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
        let dbg = Dbg::own("graham");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                "src/test/unit/algorithm/detecting_contours/output_files/out_1.jpeg",
            )
        ];
        for (step, contour) in test_data {
            let result = Graham::new(open_image(contour).expect("Error"))
            .eval(());
        }
        //assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

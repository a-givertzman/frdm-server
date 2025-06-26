#[cfg(test)]

mod graham {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::domain::{Eval, graham::{dot::Dot, find_start::FindStart}};
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
    fn find_start() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = &[
            (1, &[2,2, 4,4, 1,4, 5,2, 7,1, 4,5], 4)
        ];
        for (step, dots, target) in test_data {
            let dots: Vec<Dot<isize>> = dots.chunks(2).map(|d| Dot { x: d[0] as isize, y: d[1] as isize }).collect();
            // log::debug!("dots: {:?}", dots);
            let time = Instant::now();
            let result = FindStart::new(dots).eval(());
            log::debug!("result: {:#?};   elapsed: {:?}", result, time.elapsed());
            let result = result .start;
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

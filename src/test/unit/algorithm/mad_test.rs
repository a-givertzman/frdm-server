#[cfg(test)]

mod mad {
    use std::{
        sync::Once, 
        time::Duration
    };
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{
        DebugSession, 
        LogLevel, 
        Backtrace
    };
    use crate::{
        algorithm::Mad, 
        domain::Eval
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
    /// Testing `eval`
    #[test]
    fn eval() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("mad");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                vec![
                    100,
                    105,
                    110,
                    120,
                    130,
                    135,
                    130,
                    120,
                    110,
                    105,
                    100,
                ],
                10.0,
            )
        ];
        for (step, sample, target) in test_data {
            let result = Mad::new()
                .eval(sample)
            .mad;
            assert!(
                result == target, 
                "step {} \nresult: {:?}\ntarget: {:?}", 
                step, 
                result, 
                target
            );
        }
        test_duration.exit();
    }
}


#[cfg(test)]

mod mound {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::{geometry_defect::{groove::Groove, mound::Mound}, mad::bond::Bond}, domain::{eval::eval::Eval, graham::dot::Dot}};
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
        let dbg = Dbg::own("mound");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                vec![
                    Dot { x: 10  , y: 100 },
                    Dot { x: 20  , y: 105 },
                    Dot { x: 30  , y: 110 },
                    Dot { x: 40  , y: 120 },
                    Dot { x: 50  , y: 130 },
                    Dot { x: 60  , y: 135 },
                    Dot { x: 70  , y: 130 },
                    Dot { x: 80  , y: 120 },
                    Dot { x: 90  , y: 110 },
                    Dot { x: 100 , y: 105 },
                    Dot { x: 110 , y: 100 },
                ],
                vec![
                    Dot { x: 10  , y: 50 },
                    Dot { x: 20  , y: 45 },
                    Dot { x: 30  , y: 40 },
                    Dot { x: 40  , y: 30 },
                    Dot { x: 50  , y: 20 },
                    Dot { x: 60  , y: 15 },
                    Dot { x: 70  , y: 20 },
                    Dot { x: 80  , y: 30 },
                    Dot { x: 90  , y: 40 },
                    Dot { x: 100 , y: 45 },
                    Dot { x: 110 , y: 50 },
                ],
                vec![
                ]
            ),
            (
                2,
                vec![
                    Dot { x: 10  , y: 100 },
                    Dot { x: 20  , y: 105 },
                    Dot { x: 30  , y: 110 },
                    Dot { x: 40  , y: 120 },
                    Dot { x: 50  , y: 130 },
                    Dot { x: 60  , y: 135 },
                    Dot { x: 70  , y: 130 },
                    Dot { x: 80  , y: 120 },
                    Dot { x: 90  , y: 110 },
                    Dot { x: 100 , y: 105 },
                    Dot { x: 110 , y: 100 },
                ],
                vec![
                    Dot { x: 10  , y: 50 },
                    Dot { x: 20  , y: 45 },
                    Dot { x: 30  , y: 40 },
                    Dot { x: 40  , y: 40 },
                    Dot { x: 50  , y: 40 },
                    Dot { x: 60  , y: 45 },
                    Dot { x: 70  , y: 40 },
                    Dot { x: 80  , y: 40 },
                    Dot { x: 90  , y: 40 },
                    Dot { x: 100 , y: 45 },
                    Dot { x: 110 , y: 50 },
                ],
                vec![
                ]
            ),
            (
                3,
                vec![
                    Dot { x: 10  , y: 100 },
                    Dot { x: 20  , y: 105 },
                    Dot { x: 30  , y: 110 },
                    Dot { x: 40  , y: 80 },
                    Dot { x: 50  , y: 70 },
                    Dot { x: 60  , y: 65 },
                    Dot { x: 70  , y: 70 },
                    Dot { x: 80  , y: 80 },
                    Dot { x: 90  , y: 110 },
                    Dot { x: 100 , y: 105 },
                    Dot { x: 110 , y: 100 },
                ],
                vec![
                    Dot { x: 10  , y: 50 },
                    Dot { x: 20  , y: 45 },
                    Dot { x: 30  , y: 40 },
                    Dot { x: 40  , y: 30 },
                    Dot { x: 50  , y: 20 },
                    Dot { x: 60  , y: 15 },
                    Dot { x: 70  , y: 20 },
                    Dot { x: 80  , y: 30 },
                    Dot { x: 90  , y: 40 },
                    Dot { x: 100 , y: 45 },
                    Dot { x: 110 , y: 50 },
                ],
                vec![
                ]
            ),
        ];
        for (step, initial_points_upper, initial_points_lower, target) in test_data {
            let result = Mound::new(
                initial_points_upper,   
                initial_points_lower
            ).eval(());
            assert!(result.result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result.result, target);
        }
        test_duration.exit();
    }
}


#[cfg(test)]

mod z_score {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::expansion_contraction::mad::MAD, domain::{dbg::dbgid::DbgId, eval::eval::Eval, graham::dot::Dot}};
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
        let dbg = DbgId::root("z_score");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                vec![
                    Dot { x: 0, y: 100 },
                    Dot { x: 10, y: 105 },
                    Dot { x: 20, y: 110 },
                    Dot { x: 30, y: 120 },
                    Dot { x: 40, y: 130 },
                    Dot { x: 50, y: 135 },
                    Dot { x: 60, y: 130 },
                    Dot { x: 70, y: 120 },
                    Dot { x: 80, y: 110 },
                    Dot { x: 90, y: 105 },
                    Dot { x: 100, y: 100 },
                ],
                vec![
                    Dot { x: 0, y: 50 },
                    Dot { x: 10, y: 45 },
                    Dot { x: 20, y: 40 },
                    Dot { x: 30, y: 30 },
                    Dot { x: 40, y: 20 },
                    Dot { x: 50, y: 15 },
                    Dot { x: 60, y: 20 },
                    Dot { x: 70, y: 30 },
                    Dot { x: 80, y: 40 },
                    Dot { x: 90, y: 45 },
                    Dot { x: 100, y: 50 },
                ]

            )
        ];
        for (step, upper_points, lower_points) in test_data {
            let result = MAD::new(upper_points, lower_points)
            .eval(());
            println!("{:?}",result.bond_up);
            println!("");
            println!("{:?}",result.bond_low);

            //assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}

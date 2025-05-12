#[cfg(test)]

mod mad {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{algorithm::outliners_in_sample::{bond::Bond, mad::{MADCtx, MAD}}, domain::{eval::eval::Eval, graham::dot::Dot}};
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
    /// Function to create vector of Dot from default vector
    fn create_test_dots(y_values: &[u16]) -> Vec<Dot<u16>> {
        y_values.iter()
            .enumerate()
            .map(|(x, &y)| Dot { x: x as u16, y })
            .collect()
    }
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
                ],
                MADCtx {
                    bond_up: [
                            Bond { x: 40, y: 130 },
                            Bond { x: 50, y: 135 },
                            Bond { x: 60, y: 130 },
                        ].to_vec(),
                    bond_low: [
                            Bond { x: 40, y: 20 },
                            Bond { x: 50, y: 15 },
                            Bond { x: 60, y: 20 },
                        ].to_vec(),
                }
            )
        ];
        for (step, upper_points, lower_points, target) in test_data {
            let result = MAD::new(upper_points, lower_points)
            .eval(());
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}


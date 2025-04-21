#[cfg(test)]

mod graham {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::domain::{dbg::dbgid::DbgId, eval::eval::Eval, graham::{build_hull::Build_hull, dot::Dot, find_start::FindStartCtx, sort::Sort, sort::SortByAngCtx}};
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
    fn build_hull() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = &[
            (1, &[7,1, 4,5, 4,4, 1,4, 5,2, 2,2], 0, &[7,1, 4,5, 1,4, 2,2])
        ];
        for (step, dots, start, target) in test_data {
            let target: Vec<Dot<isize>> = target.chunks(2).map(|d| Dot { x: d[0] as isize, y: d[1] as isize }).collect();
            let dots: Vec<Dot<isize>> = dots.chunks(2).map(|d| Dot { x: d[0] as isize, y: d[1] as isize }).collect();
            let time = Instant::now();
            let result = Build_hull::new(
                MocEval { ctx: SortByAngCtx { points: dots, start: *start } }
            ).eval(());
            log::debug!("result: {:#?};   elapsed: {:?}", result, time.elapsed());
            let result = result.hull;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
    struct MocEval { ctx: SortByAngCtx }
    impl Eval<(), SortByAngCtx> for MocEval {
        fn eval(&mut self, _: ()) -> SortByAngCtx {
            self.ctx.clone()
        }
    }
}

// 7.1 ; 2.2; 1.4; 5.2; 4.4; 4.5;
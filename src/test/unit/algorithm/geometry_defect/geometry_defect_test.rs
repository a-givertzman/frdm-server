#[cfg(test)]

mod geometry_defect {
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
        algorithm::{
            GeometryDefect, GeometryDefectCtx, GeometryDefectType, Threshold,
            WidthEmissions, Context, ContextRead, ContextWrite, EdgeDetectionCtx, EvalResult, InitialCtx, InitialPoints, Mad,
        }, 
        domain::{Dot, Eval, Image},
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
        let dbg = Dbg::own("geometry_defect");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                Threshold(1.1),
                InitialPoints::new(
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
                        Dot { x: 120 , y: 100 },
                        Dot { x: 130 , y: 105 },
                        Dot { x: 140 , y: 110 },
                        Dot { x: 150 , y: 120 },
                        Dot { x: 160 , y: 130 },
                        Dot { x: 170 , y: 135 },
                        Dot { x: 180 , y: 130 },
                        Dot { x: 190 , y: 120 },
                        Dot { x: 200 , y: 110 },
                        Dot { x: 210 , y: 105 },
                        Dot { x: 220 , y: 100 },
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
                        Dot { x: 120 , y: 50 },
                        Dot { x: 130 , y: 45 },
                        Dot { x: 140 , y: 40 },
                        Dot { x: 150 , y: 30 },
                        Dot { x: 160 , y: 20 },
                        Dot { x: 170 , y: 15 },
                        Dot { x: 180 , y: 20 },
                        Dot { x: 190 , y: 30 },
                        Dot { x: 200 , y: 40 },
                        Dot { x: 210 , y: 45 },
                        Dot { x: 220 , y: 50 },
                    ],
                ),
                vec![
                    GeometryDefectType::Expansion
                ]
            ),
            (
                2,
                Threshold(1.1),
                InitialPoints::new(
                    vec![
                        Dot { x: 10  , y: 100 },
                        Dot { x: 20  , y: 105 },
                        Dot { x: 30  , y: 110 },
                        Dot { x: 40  , y: 85 },
                        Dot { x: 50  , y: 80 },
                        Dot { x: 60  , y: 75 },
                        Dot { x: 70  , y: 70 },
                        Dot { x: 80  , y: 85 },
                        Dot { x: 90  , y: 110 },
                        Dot { x: 100 , y: 105 },
                        Dot { x: 110 , y: 100 },
                    ],
                    vec![
                        Dot { x: 10  , y: 50 },
                        Dot { x: 20  , y: 45 },
                        Dot { x: 30  , y: 40 },
                        Dot { x: 40  , y: 70 },
                        Dot { x: 50  , y: 80 },
                        Dot { x: 60  , y: 73 },
                        Dot { x: 70  , y: 68 },
                        Dot { x: 80  , y: 40 },
                        Dot { x: 90  , y: 40 },
                        Dot { x: 100 , y: 45 },
                        Dot { x: 110 , y: 50 },
                    ],
                ),
                vec![
                    GeometryDefectType::Compressing
                ]
            ),
            (
                3,
                Threshold(1.1),
                InitialPoints::new(
                    vec![
                        Dot { x: 10  , y: 100 },
                        Dot { x: 20  , y: 105 },
                        Dot { x: 30  , y: 110 },
                        Dot { x: 40  , y: 85 },
                        Dot { x: 50  , y: 80 },
                        Dot { x: 60  , y: 75 },
                        Dot { x: 70  , y: 70 },
                        Dot { x: 80  , y: 85 },
                        Dot { x: 90  , y: 110 },
                        Dot { x: 100 , y: 105 },
                        Dot { x: 110 , y: 100 },
                    ],
                    vec![
                        Dot { x: 10  , y: 50 },
                        Dot { x: 20  , y: 45 },
                        Dot { x: 30  , y: 46 },
                        Dot { x: 40  , y: 50 },
                        Dot { x: 50  , y: 50 },
                        Dot { x: 60  , y: 53 },
                        Dot { x: 70  , y: 58 },
                        Dot { x: 80  , y: 50 },
                        Dot { x: 90  , y: 46 },
                        Dot { x: 100 , y: 45 },
                        Dot { x: 110 , y: 50 },
                    ],
                ),
                vec![
                    GeometryDefectType::Hill, 
                    GeometryDefectType::Compressing
                ]
            ),
        ];
        for (step, threshold, initial_points, target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(
                    InitialCtx::new()
                ),
            };
            ctx.ctx = ctx.ctx
                .clone()
                .write(EdgeDetectionCtx{ result: initial_points.clone() })
                .unwrap();
            let result = GeometryDefect::new(
                threshold,
                *Box::new(Mad::new()),
                WidthEmissions::new(threshold, 
                    *Box::new(Mad::new()), 
                    ctx
                ),
            ).eval(Image::default());
            match result {
                Ok(result) => {
                    let result = ContextRead::<GeometryDefectCtx>::read(&result)
                        .result.clone();
                    assert!(
                        result == target, 
                        "step {} \nresult: {:?}\ntarget: {:?}", 
                        step, 
                        result, 
                        target
                    );
                },
                Err(err) => panic!("step {} \nerror: {:#?}", step, err),
            }
        }
        test_duration.exit();
    }
    ///
    ///
    #[derive(Debug, Clone)]
    struct MocEval {
        pub ctx: Context,
    }
    //
    //
    impl Eval<(), EvalResult> for MocEval {
        fn eval(&self, _: ()) -> EvalResult {
            Result::Ok(self.ctx.clone())
        }
    }
}


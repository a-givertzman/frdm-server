#[cfg(test)]
mod width_emissions {
    use std::{
        sync::Once, 
        time::Duration
    };
    use indexmap::IndexMap;
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{
        DebugSession, 
        LogLevel, 
        Backtrace
    };
    use crate::{
        algorithm::{
            geometry_defect::Threshold, 
            mad::{
                Bond, 
                Mad
            }, 
            width_emissions::{
                WidthEmissions, 
                WidthEmissionsCtx
            }, 
            Context, 
            ContextRead, 
            ContextWrite, 
            EvalResult, 
            InitialCtx, 
            InitialPoints, 
            Side
        }, 
        domain::{
            graham::dot::Dot, 
            Eval
        }, 
        infrostructure::arena::Image
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
        let dbg = Dbg::own("width_emissions");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                1,
                Threshold(1.1),
                InitialPoints {
                    sides: IndexMap::from([
                        (
                            Side::Upper,
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
                        ),
                        (
                            Side::Lower,
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
                        )
                    ]),
                },
                vec![
                    Bond { x: 50, y: 130 },
                    Bond { x: 50, y: 20  },
                    Bond { x: 60, y: 135 },
                    Bond { x: 60, y: 15  },
                    Bond { x: 70, y: 130 },
                    Bond { x: 70, y: 20  }
                ]
            )
        ];
        for (step, threshold, initial_points, target) in test_data {
            let mut ctx = MocEval {
                ctx: Context::new(
                    InitialCtx::new(
                        Image::default()
                    )
                ),
            };
            ctx.ctx = ctx.ctx
                .clone()
                .write(initial_points)
                .unwrap();
            let result = WidthEmissions::new(
                threshold,
                *Box::new(Mad::new()),
                ctx,
            ).eval(());
            match result {
                Ok(result) => {
                    let result = ContextRead::<WidthEmissionsCtx>::read(&result)
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


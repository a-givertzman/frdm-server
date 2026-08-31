#[cfg(test)]
use std::{
    sync::Once,
    time::Duration
};
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::{
    DebugSession, LogLevel,
};
use crate::{
    algorithm::{
        Bend, Context, ContextRead, ContextWrite, Edges, EvalResult, FastEdgesCtx, FastScanCtx, InitialCtx, Mad, Threshold, RopeDistortions, RopeDistortionsCtx
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
    DebugSession::new().filter(LogLevel::Debug).init().unwrap();
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
            Edges::new(
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
            ),
            vec![
                Bend::from([50,130, 50,20]),
                Bend::from([60,135, 60,15]),
                Bend::from([70,130, 70,20]),
            ]
        )
    ];
    for (step, threshold, edges, target) in test_data {
        let mut ctx = MocEval {
            ctx: Context::new(
                InitialCtx::new()
            ),
        };
        ctx.ctx = ctx.ctx
            .clone()
            .write(FastEdgesCtx { edges })
            .unwrap();
        let result = RopeDistortions::<FastScanCtx>::new(
            threshold,
            *Box::new(Mad::new()),
            ctx,
        ).eval(Image::default());
        match result {
            Ok(result) => {
                let result = ContextRead::<RopeDistortionsCtx<FastScanCtx>>::read(&result)
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
impl Eval<Image, EvalResult> for MocEval {
    fn eval(&self, _: Image) -> EvalResult {
        Result::Ok(self.ctx.clone())
    }
}

#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::math::AproxEq;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::{
    algorithm::{Context, ContextRead, ContextWrite, EdgeDetectionCtx, EvalResult, InitialCtx, InitialPoints, RopeDimensions, RopeDimensionsCtx},
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
/// Testing RopeDimensions.eval
#[test]
fn eval() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("RopeDimensions-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(dbg, Duration::from_secs(100));
    test_duration.run().unwrap();
    fn into_dots(dots: &[usize]) -> Vec<Dot<usize>> {
        dots.chunks(2).map(|d| d.into()).collect()
    }
    let test_data: &[(usize, EdgeDetectionCtx, Result<(f64, f64), ()>)] = &[
        (
            1,
            EdgeDetectionCtx {
                result: InitialPoints::new(
                    into_dots(&[0,0, 1,0, 2,0, 3,0, 4,0, 5,0]),
                    into_dots(&[0,5, 1,5, 2,5, 3,5, 4,5, 5,5]),
                )
            },
            Ok((5.000, 30.000)),
        ),
        (
            2,
            EdgeDetectionCtx {
                result: InitialPoints::new(
                    into_dots(&[0,1, 1,0, 2,0, 3,1, 4,0, 5,0]),
                    into_dots(&[0,5, 1,4, 2,5, 3,5, 4,5, 5,4]),
                )
            },
            Err(()),
        ),
        (
            3,
            EdgeDetectionCtx {
                result: InitialPoints::new(
                    into_dots(&[0,0, 1,0, 2,0, 3,0, 4,0, 5,0]),
                    into_dots(&[0,5, 1,5, 2,5, 3,5, 4,5, 5,4]),
                )
            },
            Ok((4.833, 29.000)),
        ),
        (
            3,
            EdgeDetectionCtx {
                result: InitialPoints::new(
                    into_dots(&[0,1, 1,0, 2,0, 3,0, 4,0, 5,0]),
                    into_dots(&[0,5, 1,5, 2,5, 3,5, 4,5, 5,5]),
                )
            },
            Ok((4.833, 29.000)),
        ),
    ];
    for (step, dots, target) in test_data {
        let result = RopeDimensions::new(
            5,
            5.0,
            3.5,
            FakePassDots::new(dots.to_owned()),
        )
        .eval(Image::default())
        .map(|ctx| {
            let result: &RopeDimensionsCtx = ctx.read();
            result.to_owned()
        });
        match (result, target) {
            (Ok(result), Ok((target_width, target_square))) => {
                log::debug!("step {} width: {:?},  target: {:?}", step, result.width, target_width);
                log::debug!("step {} square: {:?},  target: {:?}", step, result.square, target_square);
                assert!(result.width.aprox_eq(*target_width, 3), "step {} \nresult: {:?}\ntarget: {:?}", step, result.width, target_width);
                assert!(result.square.aprox_eq(*target_square, 3), "step {} \nresult: {:?}\ntarget: {:?}", step, result.square, target_square);
            }
            (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
            (Err(_), Err(_)) => {},
        }
    }
    test_duration.exit();
}
///
/// Fake implements `Eval` for testing [RopeDimensions]
struct FakePassDots {
    dots: EdgeDetectionCtx,
}
impl FakePassDots{
    pub fn new(dots: EdgeDetectionCtx) -> Self {
        Self { dots }
    }
}
//
//
impl Eval<Image, EvalResult> for FakePassDots {
    fn eval(&self, _: Image) -> Result<Context, Error> {
        let ctx = Context::new(
            InitialCtx::new(),
        );
        ctx.write(self.dots.clone())
    }
}

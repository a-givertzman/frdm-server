#[cfg(test)]
use crate::{algorithm::{AutoBrightnessAndContrastCtx, AutoGammaCtx, Context, ContextWrite, EvalResult, Initial, InitialCtx}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use sal_sync::services::conf::ConfTree;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{
    DebugSession, 
    LogLevel, 
    Backtrace
};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        AutoBrightnessAndContrast, AutoGamma, ContextRead, Cropping, CroppingCtx, DetectingContoursCv, EdgeDetection, EdgeDetectionCtx, FilterHighPass, Gray, GrayCtx, ResultCtx, Side, TemporalFilter
    }, 
    conf::Conf, domain::Filter,
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
/// Testing 'FilterHighPass.add'
#[test]
fn eval() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("FilterHighPass-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1000));
    test_duration.run().unwrap();
    let test_data: &[(i32, u8, u8, f32)] = &[
        (01, 000, 00, 00.0),
        (02, 024, 00, 00.0),
        (03, 048, 00, 00.0),
        (04, 008, 00, 00.0),
        (05, 064, 00, 00.0),
        (06, 008, 00, 00.0),
        (07, 072, 00, 00.0),
        (08, 019, 00, 00.0),
        (09, 128, 00, 00.0),
        (10, 012, 00, 00.0),
        (11, 072, 00, 00.0),
        (12, 162, 00, 00.0),
        (13, 019, 00, 00.0),
        (14, 128, 00, 00.0),
    ];
    let mut filter = FilterHighPass::new(
        Some(0),
        0.0,
        64.0,
        32.0,
    );
    for (step, value, out, rate) in test_data {
        let result = filter.add(*value).unwrap();
        let target = *out;
        log::debug!("{dbg} | step {step}  value: {value},  result: {result},  target: {target},  rate: {}", filter.rate());
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        let result = filter.rate();
        let target = *rate;
        // log::debug!("{dbg} | step {step}  value: {value}, rate: {result}, target: {target}");
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    test_duration.exit();
}

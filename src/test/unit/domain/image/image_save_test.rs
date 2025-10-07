#[cfg(test)]

use std::{sync::Once, time::{Duration, Instant}};
use frdm_tools::Image;
use sal_core::dbg::Dbg;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
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
fn image_save() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("image_save_test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
    test_duration.run().unwrap();
    let img = opencv::imgcodecs::imread("src/test/unit/domain/image/test_pattern.png", opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();
    opencv::highgui::named_window("Loaded", opencv::highgui::WINDOW_NORMAL).unwrap();
    opencv::highgui::imshow("Loaded", &img).unwrap();
    let img = Image::with(img);
    let time = Instant::now();
    img.save("src/test/unit/domain/image/result.png").unwrap();
    log::debug!("{dbg} | Elapsed: {:?}", time.elapsed());
    let img = Image::load("src/test/unit/domain/image/result.png").unwrap();
    opencv::highgui::named_window("Result", opencv::highgui::WINDOW_NORMAL).unwrap();
    opencv::highgui::imshow("Result", &img.mat).unwrap();
    opencv::highgui::wait_key(1).unwrap();
    // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    test_duration.exit();
}

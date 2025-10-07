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
/// Testing [Image] => `Bytes`
#[test]
fn image_encode() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    init_once();
    init_each();
    let dbg = Dbg::own("Image.to_bytes");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(30));
    test_duration.run().unwrap();
    let test_data = [
        (01, "src/test/unit/algorithm/detecting_contours/testing_files/rope_0.jpeg"),
        (02, "src/test/unit/algorithm/detecting_contours/testing_files/rope_1.jpeg"),
        (03, "src/test/unit/algorithm/detecting_contours/testing_files/rope_2.jpeg"),
        (04, "src/test/unit/algorithm/detecting_contours/testing_files/rope_3.jpeg"),
        (05, "src/test/unit/algorithm/detecting_contours/testing_files/rope_4.jpeg"),
        (06, "src/test/unit/algorithm/detecting_contours/testing_files/rope_5.jpeg"),
        (07, "src/test/unit/algorithm/detecting_contours/testing_files/rope_6.jpeg"),
        (08, "src/test/unit/algorithm/detecting_contours/testing_files/rope_7.jpeg"),
        (09, "src/test/unit/algorithm/detecting_contours/testing_files/rope_8.jpeg"),
        (10, "src/test/unit/algorithm/detecting_contours/testing_files/rope_9.jpeg"),
        (11, "src/test/unit/domain/image/test_pattern.png"),
    ];
    for (step, path) in test_data {
        let img = opencv::imgcodecs::imread(path, opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();
        opencv::highgui::named_window("Loaded", opencv::highgui::WINDOW_NORMAL).unwrap();
        opencv::highgui::imshow("Loaded", &img).unwrap();
        let target = Image::with(img);
        let time = Instant::now();
        let bytes = target.to_bytes().unwrap();
        let elapsed = time.elapsed();
        log::debug!("{dbg} | Bytes: {:?}", bytes.len());
        log::debug!("{dbg} | Encode elapsed: {:?}", elapsed);
        let time = Instant::now();
        let result = Image::from_bytes(&bytes).unwrap();
        log::debug!("{dbg} | Decode elapsed: {:?}", time.elapsed());
        opencv::highgui::named_window("Result", opencv::highgui::WINDOW_NORMAL).unwrap();
        opencv::highgui::imshow("Result", &result.mat).unwrap();
        opencv::highgui::wait_key(10).unwrap();
        assert!(result == target, "{dbg} | step {step} \nresult: {:?}\ntarget: {:?}", result, target);
    }
    test_duration.exit();
}

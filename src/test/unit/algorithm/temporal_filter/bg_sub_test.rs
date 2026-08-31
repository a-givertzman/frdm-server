#[cfg(test)]
use crate::{algorithm::{Initial, InitialCtx}, domain::{Eval, Image}};
use std::{sync::Once, time::{Duration, Instant}};
use opencv::{core::{Mat, MatTraitConst}, highgui, video::BackgroundSubtractorTrait};
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::{
    DebugSession,
    LogLevel
};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        AutoGamma, ContextRead, Cropping, Gray, GrayCtx
    },
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
/// Testing 'TemporalFilter.eval'
#[test]
fn eval() {
    DebugSession::new().filter(LogLevel::Debug).init().unwrap();
    init_once();
    init_each();
    let dbg = Dbg::own("bg-sub-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1000));
    test_duration.run().unwrap();
    let debug = false;
    let gray = Gray::new(
        AutoGamma::new(
            120.0,
            Cropping::new(
                230,
                1410,
                300,
                1000,
                Initial::new(
                    InitialCtx::new(),
                ),
                debug,
            ),
            debug,
        ),
    );
    let wgray = "Gray";
    let wfgmask = "Fg Mask";
    if let Err(err) = opencv::highgui::named_window(wgray, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }
    if let Err(err) = opencv::highgui::named_window(wfgmask, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{dbg} | Create Window Error: {}", err);
    }

    let image_dir = "src/test/unit/algorithm/temporal_filter/frames";
    // "/home/ilyarizo/deffect_photos/rope_rotated/gap_pit/exp95/retrived";

    for path in std::fs::read_dir(image_dir).unwrap().into_iter()
        .filter_map(|e| {
            let path = e.unwrap().path();
            path.is_file().then(|| path)
        })
        .skip(22)
    {
        match path.extension() {
            Some(ext) if ext == "jpg" || ext == "png" || ext == "jpeg" => {
                let frame = Image::load(path.to_str().unwrap(), 0).unwrap();
                // let mut rotated = Mat::default();
                // core::rotate(&frame.mat, &mut rotated, ROTATE_90_CLOCKWISE).unwrap();
                // let src = Image::with(rotated);
                log::debug!("{dbg}.eval | src frame: {} x {}", frame.width(), frame.height());
                // let test = src.clone();
                let t = Instant::now();
                let ctx = gray.eval(frame).unwrap();
                // let mut bg_sub = opencv::bgsegm::create_background_subtractor_gsoc(
                //     opencv::bgsegm::LSBP_CAMERA_MOTION_COMPENSATION_NONE,
                //     20,
                //     0.3,
                //     0.01,
                //     5,
                //     0.01,
                //     0.0022,
                //     0.1,
                //     0.1,
                //     0.0004,
                //     0.0008,
                // ).unwrap();
                // let mut bg_sub = opencv::bgsegm::create_background_subtractor_cnt(
                //     18,
                //     true,
                //     18 * 60,
                //     true,
                // ).unwrap();
                // let mut bg_sub = opencv::bgsegm::create_background_subtractor_gmg(
                //     10,
                //     0.4,
                // ).unwrap();
                let mut bg_sub = opencv::bgsegm::create_background_subtractor_lsbp(
                    opencv::bgsegm::LSBP_CAMERA_MOTION_COMPENSATION_NONE,
                    20,
                    16,
                    2.0,
                    32.0,
                    1.0,
                    0.05,
                    10.0,
                    0.005,
                    0.0004,
                    0.0008,
                    8,
                    2,
                ).unwrap();

                // let mut bg_sub = opencv::video::create_background_subtractor_knn(500, 400.0, false).unwrap();
                // let mut bg_sub = opencv::video::create_background_subtractor_mog2(10, 16.0, false).unwrap();
                log::debug!("{dbg}.eval | Elapsed: {:?}", t.elapsed());
                let gray: &GrayCtx = ctx.read();
                let mut fgmask = Mat::default();
                bg_sub.apply(&gray.frame.mat, &mut fgmask, 0.8).unwrap();
                if !gray.frame.mat.empty() { highgui::imshow(wgray, &gray.frame.mat).unwrap() };
                if !fgmask.empty() { highgui::imshow(wfgmask, &fgmask).unwrap() };
                highgui::wait_key(0).unwrap();
            },
            _ => continue,
        }
    }
    highgui::destroy_all_windows().unwrap();
    test_duration.exit();
}

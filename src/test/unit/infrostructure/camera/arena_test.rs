#[cfg(test)]

mod arena {
    use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Once}, thread, time::{Duration, Instant}};
    use crate::infrostructure::arena::{ac_device::AcDevice, ac_system::AcSystem, pixel_format::PixelFormat};
    use sal_sync::services::entity::dbg_id::DbgId;
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
    fn list_devices() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId("arena_test".into());
        let dbg_clone = dbg.clone();
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(20));
        test_duration.run().unwrap();
        let read_time = Duration::from_secs(10);
        let frames = Arc::new(AtomicUsize::new(0));
        let frames_clone = frames.clone();
        let exit = Arc::new(AtomicBool::new(false));
        let exit_clone = exit.clone();
        let time = Instant::now();
        let handle = std::thread::spawn(move || {
            let mut ac_system = AcSystem::new(&dbg);
            match ac_system.run() {
                Ok(_) => {
                    match ac_system.devices() {
                        Some(devices) => {
                            log::debug!("Devices found: {}", devices);
                            for dev in 0..devices {
                                log::debug!("Retriving Device {}...", dev);
                                let device_vendor = ac_system.device_vendor(dev).unwrap();
                                let device_model = ac_system.device_model(dev).unwrap();
                                log::trace!("Device {} model: {}", dev, device_model);
                                let device_serial = ac_system.device_serial(dev).unwrap();
                                log::trace!("Device {} serial: {}", dev, device_serial);
                                let device_mac = ac_system.device_mac(dev).unwrap();
                                let device_ip = ac_system.device_ip(dev).unwrap();
                                log::trace!("Device {} IP: {}", dev, device_ip);
                                log::info!("Device {}: {:?} | {:?} | {:?} | {:?} | {:?}", dev, device_vendor, device_model, device_serial, device_mac, device_ip);
                            }
                            let selection = 0;
                            let mut device = AcDevice::new(&dbg, ac_system.system, selection, PixelFormat::BGR8, Some(exit_clone));
                            let window = "Retrived";
                            if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
                                log::warn!("{}.stream | Create Window Error: {}", dbg, err);
                            }
                            // opencv::highgui::wait_key(10).unwrap();
                            let result = device.listen(|frame| {
                                if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
                                    log::warn!("{}.stream | Display img error: {:?}", dbg, err);
                                };
                                opencv::highgui::wait_key(1).unwrap();
                                // let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap(); // 0 is the default camera
                                // if ! cam.is_opened().unwrap() {
                                //     log::warn!("{}.stream | Cam isn't opened", dbg);
                                // }
                                frames_clone.fetch_add(1, Ordering::SeqCst);
                            });
                            if let Err(err) = result {
                                log::warn!("{} | Error; {}", dbg, err);
                            }
                        }
                        None => {
                            log::warn!("{} | No devices detected", dbg);
                        }
                    }
                }
                Err(err) => panic!("{} | Error; {}", dbg, err),
            }
        });
        thread::sleep(read_time);
        exit.store(true, Ordering::SeqCst);
        handle.join().unwrap();
        let elapsed = time.elapsed();
        let frames = frames.load(Ordering::SeqCst);
        log::info!("{} | Retrived frames: {}", dbg_clone, frames);
        log::info!("{} | Elapsed: {:?}", dbg_clone, elapsed);
        log::info!("{} | FPS: {:?}", dbg_clone, (frames as f64) / (elapsed.as_secs() as f64));
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

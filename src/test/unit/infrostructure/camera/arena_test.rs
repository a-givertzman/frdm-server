#[cfg(test)]

mod arena {
    use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Once}, thread, time::{Duration, Instant}};
    use crate::{domain::{channel_unbounded, Image}, infrostructure::arena::{AcDevice, AcSystem}, CameraConf};
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
    /// Testing TRI028S-CC Image aqusion
    /// 
    /// [TRI028S-CC Technical spec](https://thinklucid.com/product/triton-2-8-mp-imx429/)
    #[test]
    fn listen_device() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("arena_test");
        let dbg_1 = dbg.clone();
        let dbg_2 = dbg.clone();
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(40));
        test_duration.run().unwrap();
        let read_time = Duration::from_secs(30);
        let frames = Arc::new(AtomicUsize::new(0));
        let frames_clone = frames.clone();
        let exit = Arc::new(AtomicBool::new(false));
        let exit_1 = exit.clone();
        let exit_2 = exit.clone();
        let conf = serde_yaml::from_str(r#"
            service Camera Camera1:
                fps: Max                    # Max / Min / 30.0
                resolution: 
                    width: 1200
                    height: 800
                index: 0
                # address: 192.168.10.12:2020
                pixel-format: BayerRG8          # Mono8/10/12/16, BayerRG8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411 | Default and fastest BayerRG8
                exposure:
                    auto: Off                   # Off / Continuous
                    time: 7000                   # microseconds
                auto-packet-size: true          # StreamAutoNegotiatePacketSize
                channel-packet-size: Max        # Maximizing packet size increases frame rate, Max / Min / 1500
                resend-packet: true             # StreamPacketResendEnable
        "#).unwrap();
        let conf = CameraConf::from_yaml(&dbg, &conf);
        let time = Instant::now();
        let (send, recv) = channel_unbounded::<Image>();
        let disp_handle = std::thread::spawn(move || {
            let dbg = dbg_1;
            let window = "Retrived";
            if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
                log::warn!("{}.stream | Create Window Error: {}", dbg, err);
            }
            opencv::highgui::wait_key(1).unwrap();
            for frame in recv {
                if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
                    log::warn!("{}.stream | Display img error: {:?}", dbg, err);
                };
                opencv::highgui::wait_key(1).unwrap();
                // let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap(); // 0 is the default camera
                // if ! cam.is_opened().unwrap() {
                //     log::warn!("{}.stream | Cam isn't opened", dbg);
                // }
                if exit_2.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        let handle = std::thread::spawn(move || {
            let dbg = dbg_2;
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
                            let mut device = AcDevice::new(&dbg, ac_system.system, selection, conf, Some(exit_1), None);
                            let result = device.listen(|frame| {
                                if let Err(err) = send.send(frame) {
                                    log::warn!("{} | Send Error; {}", dbg, err);
                                }
                                frames_clone.fetch_add(1, Ordering::SeqCst);
                            });
                            if let Err(err) = result {
                                log::warn!("{} | Error: {}", dbg, err);
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
        disp_handle.join().unwrap();
        let elapsed = time.elapsed();
        let frames = frames.load(Ordering::SeqCst);
        log::info!("{} | Retrived frames: {}", dbg, frames);
        log::info!("{} | Elapsed: {:?}", dbg, elapsed);
        log::info!("{} | FPS: {:?}", dbg, (frames as f64) / (elapsed.as_secs() as f64));
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

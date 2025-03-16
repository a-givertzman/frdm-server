#[cfg(test)]

mod tests {
    use std::{ffi::CString, str::FromStr, sync::Once, time::{Duration, Instant}};
    use crate::infrostructure::arena::{ac_device::AcDevice, ac_err::AcErr, ac_system::{self, AcSystem}, bindings::{
        acBuffer, acBufferGetSizeFilled, acCloseSystem, acDevice, acDeviceGetBuffer, acDeviceGetNodeMap, acDeviceGetTLStreamNodeMap, acDeviceRequeueBuffer, acDeviceStartStream, acDeviceStopStream, acImageGetHeight, acImageGetTimestampNs, acImageGetWidth, acNode, acNodeMap, acNodeMapGetNodeAndAccessMode, acNodeMapSetEnumerationValue, acOpenSystem, acSystemCreateDevice, acSystemDestroyDevice, acSystemGetDeviceIpAddressStr, acSystemGetDeviceModel, acSystemGetDeviceSerial, acSystemGetNumDevices, acSystemUpdateDevices, acValueFromString, acValueToString, AC_ACCESS_MODE, AC_ACCESS_MODE_NI, AC_ACCESS_MODE_RO, AC_ACCESS_MODE_RW, AC_ACCESS_MODE_WO, AC_ERROR, AC_ERROR_LIST_AC_ERR_SUCCESS
    }};
    use sal_sync::services::entity::error::str_err::StrErr;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::domain::dbg::dbgid::DbgId;
    ///
    ///
    static INIT: Once = Once::new();
    const MAX_BUF: usize = 1024;
    const IMAGE_TIMEOUT: u64 = 2000;
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
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let mut ac_system = AcSystem::new(&dbg);
        match ac_system.run() {
            Ok(_) => {
                match ac_system.devices() {
                    Some(devices) => {
                        log::debug!("Devices found: {}", devices);
                        for dev in 0..devices {
                            log::debug!("Retriving Device {}...", dev);
                            let device_model = ac_system.device_model(dev).unwrap();
                            log::debug!("Device {} model: {}", dev, device_model);
                            let device_serial = ac_system.device_serial(dev).unwrap();
                            log::debug!("Device {} serial: {}", dev, device_serial);
                            let device_ip = ac_system.device_ip(dev).unwrap();
                            log::debug!("Device {} IP: {}", dev, device_serial);
                            println!("Device {}: {:?} | {:?} | {:?}", dev, device_model, device_serial, device_ip);
                        }
                        let selection = 0;
                        let mut device = AcDevice::new(&dbg, ac_system.system, selection);
                        match device.run() {
                            Ok(_) => {

                            }
                            Err(err) => {
                                log::warn!("{} | Error; {}", dbg, err);
                            }
                        }
                    }
                    None => {
                        log::warn!("{} | No devices detected", dbg);
                    }
                }
            }
            Err(err) => panic!("{} | Error; {}", dbg, err),
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

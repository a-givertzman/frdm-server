#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use lucid_arena_sys::{acCloseSystem, acDeviceGetNodeMap, acOpenSystem, acSystemGetNumDevices, acSystemUpdateDevices, AC_ERROR_LIST_AC_ERR_SUCCESS};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::domain::dbg::dbgid::DbgId;
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
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        unsafe {
            let mut sys = std::mem::zeroed();

            let err = acOpenSystem(&mut sys);
            log_error("acOpenSystem", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let mut num_devices: usize = 0;

            let err = acSystemUpdateDevices(sys, 200);
            log_error("acSystemUpdateDevices", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let err = acSystemGetNumDevices(sys, &mut num_devices);
            log_error("acSystemGetNumDevices", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);
            log::debug!("Devices detected: {}", num_devices);

            let err = acCloseSystem(sys);
            log_error("acCloseSystem", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// 
    fn log_error(dbg: &str, err: i32) {
        if err > 0 {
            log::error!("{} | err: {}", dbg, err);
        } else {
            log::debug!("{} | Ok", dbg);
        }
    }
}

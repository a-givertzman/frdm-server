#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use lucid_arena_sys::{acDeviceGetNodeMap, acOpenSystem, acSystemUpdateDevices};
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
            let phSystem = std::ptr::null_mut();
            let err = acOpenSystem(phSystem);
            log::error!("acOpenSystem | err: {}", err);
            let err = acSystemUpdateDevices(*phSystem, 100);
            log::error!("acSystemUpdateDevices | err: {}", err);
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
}

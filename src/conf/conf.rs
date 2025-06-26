use crate::conf::{FastScanConf, FineScanConf};

///
/// Te application configuration
pub struct Conf {
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
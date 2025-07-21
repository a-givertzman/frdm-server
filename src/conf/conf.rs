use crate::conf::{DetectingContoursConf, FastScanConf, FineScanConf};

///
/// Te application configuration
pub struct Conf {
    pub detecting_contours: DetectingContoursConf,
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
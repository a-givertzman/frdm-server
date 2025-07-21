use crate::conf::{DetectingContoursConf, FastScanConf, FineScanConf};

///
/// Te application configuration
#[derive(Debug, PartialEq, Clone)]
pub struct Conf {
    pub detecting_contours: DetectingContoursConf,
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
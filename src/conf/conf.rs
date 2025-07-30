use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfDistance, ConfDistanceUnit, ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::{DetectingContoursConf, FastScanConf, FineScanConf};

///
/// The application configuration
/// 
/// ### Example
/// ```yaml
/// segment: 100 mm             # Whole rope will divided by the segments for the Camera defect detection, recomended: `segment length = camera.width * 0.10..0.20`
/// segment-threshold: 5 mm     # Acceptable camera position error in relation to exact segment position, default 5% of `segment`
/// detecting-contours:
///     gamma:
///         no-param: not parameters implemented 
///     brightness-contrast:
///         histogram-clipping: 1     # optional histogram clipping, default = 0 %
///     gausian:
///         kernel-size:
///             width: 3
///             height: 3
///         sigma-x: 0.0
///         sigma-y: 0.0
///     sobel:
///         kernel-size: 3
///         scale: 1.0
///         delta: 0.0
///     overlay:
///         src1-weight: 0.5
///         src2-weight: 0.5
///         gamma: 0.0
/// fast-scan:
///     geometry-defect-threshold: 1.2      # 1.1...1.3
/// fine-scan:
///     no-params: not implemented yet
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Conf {
    /// Rope segmetn length.
    /// Whole rope will divided by the segments for the Camera defect detection, recomended: `segment length = camera.width * 0.10..0.20`
    pub segment: ConfDistance,
    /// Acceptable camera position error in relation to exact segment position
    /// 
    /// Default: 5% of `segment`
    pub segment_threshold: ConfDistance,
    pub detecting_contours: DetectingContoursConf,
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
impl Conf {
    ///
    /// Returns [Conf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "Conf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let segment = conf.get_distance("segment").expect(&format!("{dbg}.new | 'segment' - not found or wrong configuration"));
        log::debug!("{dbg}.new | segment: {:?}", segment);
        let segment_threshold = conf.get_distance("segment-threshold").unwrap_or(ConfDistance::new(segment.as_mm() * 0.05, ConfDistanceUnit::Millimeter));
        log::debug!("{dbg}.new | segment-threshold: {:?}", segment_threshold);
        let detecting_contours = conf.get("detecting-contours").expect(&format!("{dbg}.new | 'detecting-contours' - not found or wrong configuration"));
        let detecting_contours = DetectingContoursConf::new(&name, detecting_contours);
        log::trace!("{dbg}.new | detecting-contours: {:#?}", detecting_contours);
        let fast_scan = conf.get("fast-scan").expect(&format!("{dbg}.new | 'fast-scan' - not found or wrong configuration"));
        let fast_scan = FastScanConf::new(&name, fast_scan);
        log::trace!("{dbg}.new | fast-scan: {:#?}", fast_scan);
        let fine_scan = conf.get("fine-scan").expect(&format!("{dbg}.new | 'fine-scan' - not found or wrong configuration"));
        let fine_scan = FineScanConf::new(&name, fine_scan);
        log::trace!("{dbg}.new | fine-scan: {:#?}", fine_scan);
        Self {
            segment,
            segment_threshold,
            detecting_contours,
            fast_scan,
            fine_scan,
        }
    }
}

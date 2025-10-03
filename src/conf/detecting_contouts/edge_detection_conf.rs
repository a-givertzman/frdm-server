use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for edge-detection algorithm
/// 
/// ### Example:
/// ```yaml
/// edge-detection:
///     otsu-tune: 1.0      # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0, if not specified, `threshold` will be used
///     threshold: 1        # 0...255, if not specified otsu auto threshold will be used, if nothing specified, otsu threshold will be used with otsu-tune = 1
///     smooth: 16          # Smoothing of edge line factor. The higher the factor the smoother the line.
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeDetectionConf {
    /// Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold
    /// 
    /// If not specified, then manual threshold will used
    /// 
    /// Default 1.0
    pub otsu_tune: Option<f64>,
    /// Manual threshold, no Otsu auto threshold used, 0...255
    pub threshold: Option<u8>,
    /// Smoothing of edge line factor. The higher the factor the smoother the line. Can't be <= 0
    pub smooth: Option<f64>,
}
//
// 
impl EdgeDetectionConf {
    ///
    /// Returns [EdgeDetectionConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "EdgeDetectionConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune");
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let threshold = conf.get("threshold").map(|val: u64| val as u8);
        log::trace!("{dbg}.new | threshold: {:#?}", threshold);
        let otsu_tune = match (otsu_tune, threshold) {
            (None, None) => Some(1.0),
            (None, Some(_)) => None,
            (Some(otsu_tune), None) => Some(otsu_tune),
            (Some(_), Some(_)) => panic!("{dbg}.new |  'otsu-tune' and 'threshold' - both specified, use on of them, otsu auto threshol with 'otsu-tune' or static 'threshold'"),
        };
        let smooth = conf.get("smooth");
        let smooth = match smooth {
            Some(smooth) => if smooth <= 0.0 {
                None
            } else {
                Some(smooth)
            }
            None => None,
        };
        log::trace!("{dbg}.new | smooth: {:#?}", smooth);
        Self {
            otsu_tune,
            threshold,
            smooth,
        }
    }
}
//
//
impl Default for EdgeDetectionConf {
    fn default() -> Self {
        Self {
            otsu_tune: Some(1.0),
            threshold: None,
            smooth: None,
        }
    }
}

use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for edge-detection algorithm
/// 
/// ### Example:
/// ```yaml
/// edge-detection:
///     otsu-tune: 1.0      # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
///     threshold: 1        # 0...255, used if otsu-tune is not specified
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
    pub threshold: u8,
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
        let otsu_tune: Option<f64> = conf.get("otsu-tune");
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let threshold: u64 = conf.get("threshold").expect(&format!("{dbg}.new | 'threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | threshold: {:#?}", threshold);
        Self {
            otsu_tune,
            threshold: threshold as u8,
        }
    }
}
//
//
impl Default for EdgeDetectionConf {
    fn default() -> Self {
        Self {
            otsu_tune: Some(1.0),
            threshold: 20,
        }
    }
}

use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Weighted Overlay`` algorithm
/// 
/// ### Example:
/// ```yaml
/// overlay:
///     src1-weight: 0.5
///     src2-weight: 0.5
///     gamma: 0.0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OverlayConf {
    /// Weight for X gradient
    /// 
    /// Default: 0.5
    pub src1_weight: f64,
    // Weight for Y gradient
    /// 
    /// Default: 0.5
    pub src2_weight: f64,
    /// Scalar added to weighted sum
    /// 
    /// Default: 0.0
    pub gamma: f64,
}
//
// 
impl OverlayConf {
    ///
    /// Returns [OverlayConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "OverlayConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let src1_weight = conf.get("alpha").unwrap_or(0.5);
        log::debug!("{dbg}.new | alpha: {:?}", src1_weight);
        let src2_weight = conf.get("beta").unwrap_or(0.5);
        log::debug!("{dbg}.new | beta: {:?}", src2_weight);
        let gamma = conf.get("gamma").unwrap_or(0.0);
        log::debug!("{dbg}.new | gamma: {:?}", gamma);
        Self {
            src1_weight,
            src2_weight,
            gamma,
        }
    }
}
//
//
impl Default for OverlayConf {
    fn default() -> Self {
        Self { src1_weight: 0.5, src2_weight: 0.5, gamma: 0.0 }
    }
}

use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Gamma auto correction` algorithm
/// 
/// ### Example:
/// ```yaml
/// gamma:
///     factor: 95.0              # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GammaConf {
    /// percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
    /// - exposure 35 us: beatter percent - 60 %
    /// - exposure 95 us: beatter percent - 95 %
    pub factor: f64,
}
impl GammaConf {
    ///
    /// Returns [OverlayConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "OverlayConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let factor = conf.get("factor").expect(&format!("{dbg}.new | 'factor' - not found or wrong configuration"));
        log::trace!("{dbg}.new | factor: {:?}", factor);
        Self {
            factor,
        }
    }
}
//
//
impl Default for GammaConf {
    /// by default used 100.0
    fn default() -> Self {
        Self {
            factor: 100.0,
        }
    }
}

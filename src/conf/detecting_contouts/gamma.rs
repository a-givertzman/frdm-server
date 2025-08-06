use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};

///
/// ## Configuration for `Gamma auto correction` algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct GammaConf {}
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
        Self {
        }
    }
}
//
//
impl Default for GammaConf {
    fn default() -> Self {
        Self { }
    }
}

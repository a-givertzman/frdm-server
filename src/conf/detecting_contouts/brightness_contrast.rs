use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Brightness and contrast auto correction`
/// 
/// ### Example:
/// ```yaml
/// brightness-contrast:
///     histogram_clipping: 1     # optional histogram clipping, default = 0 %
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BrightnessContrastConf {
    /// Optional histogram clipping
    /// 
    /// Default: 0
    pub histogram_clipping: i32,
}
//
// 
impl BrightnessContrastConf {
    ///
    /// Returns [BrightnessContrastConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "BrightnessContrastConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let histogram_clipping = conf.get("histogram-clipping").unwrap_or(0.0);
        log::debug!("{dbg}.new | histogram-clipping: {:?}", histogram_clipping);
        Self {
            histogram_clipping: histogram_clipping as i32,
        }
    }
}
//
//
impl Default for BrightnessContrastConf {
    fn default() -> Self {
        Self { histogram_clipping: 0 }
    }
}

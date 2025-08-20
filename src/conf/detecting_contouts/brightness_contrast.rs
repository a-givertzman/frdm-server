use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Brightness and contrast auto correction`
/// 
/// Works automatically using histogram clipping from left and right
/// 
/// ### Example:
/// ```yaml
/// brightness-contrast:
///     hist-clip-left: 1      # optional histogram clipping, default = 0 %
///     hist-clip-right: 99     # optional histogram clipping, default = 0 %
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BrightnessContrastConf {
    /// Optional histogram clipping from left, %
    /// 
    /// Default: 0
    pub hist_clip_left: i32,
    /// Optional histogram clipping from right, %
    /// 
    /// Default: 100
    pub hist_clip_right: i32,
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
        log::trace!("{}.new | name: {:?}", dbg, name);
        let hist_clip_left = conf.get("hist-clip-left").unwrap_or(0.0);
        log::trace!("{dbg}.new | hist-clip-left: {:?}", hist_clip_left);
        let hist_clip_right = conf.get("hist-clip-right").unwrap_or(0.0);
        log::trace!("{dbg}.new | hist-clip-right: {:?}", hist_clip_right);
        Self {
            hist_clip_left: hist_clip_left as i32,
            hist_clip_right: hist_clip_right as i32,
        }
    }
}
//
//
impl Default for BrightnessContrastConf {
    fn default() -> Self {
        Self {
            hist_clip_left: 0,
            hist_clip_right: 100,
        }
    }
}

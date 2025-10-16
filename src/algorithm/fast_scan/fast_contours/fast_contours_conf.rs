use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Contour dectection` algorithm
/// 
/// ### Example:
/// ```yaml
/// fast-contours:
///     cropping:
///         x: 230              # New left edge
///         y: 300              # New top edge
///         width: 1410         # New image width
///         height: 1000        # New image height
///     gamma:
///         factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
///     otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastContoursConf {
    /// Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
    pub otsu_tune: f64,
    // // /// Configuration for `Brightness and contrast auto correction`
    // // pub brightness_contrast: BrightnessContrastConf,
}
//
// 
impl FastContoursConf {
    ///
    /// Returns [DetectingContoursConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "DetectingContoursConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune").expect(&format!("{dbg}.new | 'otsu-tune' - not found or wrong configuration"));
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        Self {
            otsu_tune,
        }
    }
}
//
//
impl Default for FastContoursConf {
    fn default() -> Self {
        Self {
            otsu_tune: 0.4,
        }
    }
}

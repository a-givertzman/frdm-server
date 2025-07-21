use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::{GausianConf, OverlayConf, SobelConf};

///
/// ## Configuration for `Contour dectection` algorithm
/// 
/// ### Example:
/// ```yaml
/// gausian:
///     kernel-size:
///         width: 3
///         height: 3
///     sigma-x: 0.0
///     sigma-y: 0.0
/// sobel:
///     kernel-size: 3
///     scale: 1.0
///     delta: 0.0
/// overlay:
///     src1-weight: 0.5
///     src2-weight: 0.5
///     gamma: 0.0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DetectingContoursConf {
    /// Configuration for `Gaussian filter`
    pub gausian: GausianConf,
    /// Configuration for `Sobel operator`
    pub sobel: SobelConf,
    /// Configuration for `Weighted sum`
    pub overlay: OverlayConf,
}
//
// 
impl DetectingContoursConf {
    ///
    /// Returns [DetectingContoursConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "DetectingContoursConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let gausian = conf.get("gausian").expect(&format!("{dbg}.new | 'gausian' - not found or wrong configuration"));
        let gausian = GausianConf::new(&name, gausian);
        log::debug!("{dbg}.new | gausian: {:#?}", gausian);
        let sobel = conf.get("sobel").expect(&format!("{dbg}.new | 'sobel' - not found or wrong configuration"));
        let sobel = SobelConf::new(&name, sobel);
        log::debug!("{dbg}.new | sobel: {:#?}", sobel);
        let overlay = conf.get("overlay").expect(&format!("{dbg}.new | 'overlay' - not found or wrong configuration"));
        let overlay = OverlayConf::new(&name, overlay);
        log::debug!("{dbg}.new | overlay: {:#?}", overlay);
        Self {
            gausian,
            sobel,
            overlay,
        }
    }
}
//
//
impl Default for DetectingContoursConf {
    fn default() -> Self {
        Self {
            gausian: Default::default(),
            sobel: Default::default(),
            overlay: Default::default(),
        }
    }
}

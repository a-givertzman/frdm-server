use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{conf::{BrightnessContrastConf, GammaConf, GausianConf, OverlayConf, SobelConf}, algorithm::CroppingConf};

///
/// ## Configuration for `Contour dectection` algorithm
/// 
/// ### Example:
/// ```yaml
/// cropping:
///     x: 10           # new left edge
///     width: 1900     # new image width
///     y: 10           # new top edge
///     height: 1180    # new image height
/// gamma:
///     factor: 95.0             # percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
/// brightness-contrast:
///     hist-clip-left: 1.0      # optional histogram clipping, default = 0.0 %
///     hist-clip-right: 1.0     # optional histogram clipping, default = 0.0 %
/// gausian:
///     blur-size:            # blur radius
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
    /// Configuration for `Cropping` operator
    pub cropping: CroppingConf,
    /// Configuration for `Gamma auto correction` algorithm
    pub gamma: GammaConf,
    /// Configuration for `Brightness and contrast auto correction`
    pub brightness_contrast: BrightnessContrastConf,
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
        log::trace!("{}.new | name: {:?}", dbg, name);
        let cropping = conf.get("cropping").expect(&format!("{dbg}.new | 'cropping' - not found or wrong configuration"));
        let cropping = CroppingConf::new(&name, cropping);
        log::trace!("{dbg}.new | cropping: {:#?}", cropping);
        let gamma = conf.get("gamma").expect(&format!("{dbg}.new | 'gamma' - not found or wrong configuration"));
        let gamma = GammaConf::new(&name, gamma);
        log::trace!("{dbg}.new | gamma: {:#?}", gamma);
        let brightness_contrast = conf.get("brightness-contrast").expect(&format!("{dbg}.new | 'brightness-contrast' - not found or wrong configuration"));
        let brightness_contrast = BrightnessContrastConf::new(&name, brightness_contrast);
        log::trace!("{dbg}.new | brightness-contrast: {:#?}", brightness_contrast);
        let gausian = conf.get("gausian").expect(&format!("{dbg}.new | 'gausian' - not found or wrong configuration"));
        let gausian = GausianConf::new(&name, gausian);
        log::trace!("{dbg}.new | gausian: {:#?}", gausian);
        let sobel = conf.get("sobel").expect(&format!("{dbg}.new | 'sobel' - not found or wrong configuration"));
        let sobel = SobelConf::new(&name, sobel);
        log::trace!("{dbg}.new | sobel: {:#?}", sobel);
        let overlay = conf.get("overlay").expect(&format!("{dbg}.new | 'overlay' - not found or wrong configuration"));
        let overlay = OverlayConf::new(&name, overlay);
        log::trace!("{dbg}.new | overlay: {:#?}", overlay);
        Self {
            cropping,
            gamma,
            brightness_contrast,
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
            cropping: CroppingConf::default(),
            gamma: GammaConf::default(),
            brightness_contrast: BrightnessContrastConf::default(),
            gausian: GausianConf::default(),
            sobel: SobelConf::default(),
            overlay: OverlayConf::default(),
        }
    }
}

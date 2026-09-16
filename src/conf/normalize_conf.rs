use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::CroppingConf, conf::GammaConf};

///
/// ## Configuration for `Contour dectection` algorithm
/// 
/// ### Example:
/// ```yaml
/// normalize:
///     cropping:
///         x: 230              # New left edge
///         y: 300              # New top edge
///         width: 1410         # New image width
///         height: 1000        # New image height
///     gamma:
///         factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizeConf {
    /// Configuration for `Cropping` operator
    pub cropping: CroppingConf,
    /// Configuration for `Gamma auto correction` algorithm
    pub gamma: GammaConf,
    // // /// Configuration for `Brightness and contrast auto correction`
    // // pub brightness_contrast: BrightnessContrastConf,
}
//
// 
impl NormalizeConf {
    ///
    /// Returns [NormalizeConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "NormalizeConf";
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
        // let brightness_contrast = conf.get("brightness-contrast").expect(&format!("{dbg}.new | 'brightness-contrast' - not found or wrong configuration"));
        // let brightness_contrast = BrightnessContrastConf::new(&name, brightness_contrast);
        // log::trace!("{dbg}.new | brightness-contrast: {:#?}", brightness_contrast);
        // let gausian = conf.get("gausian").expect(&format!("{dbg}.new | 'gausian' - not found or wrong configuration"));
        // let gausian = GaussianConf::new(&name, gausian);
        // log::trace!("{dbg}.new | gausian: {:#?}", gausian);
        // let sobel = conf.get("sobel").expect(&format!("{dbg}.new | 'sobel' - not found or wrong configuration"));
        // let sobel = SobelConf::new(&name, sobel);
        // log::trace!("{dbg}.new | sobel: {:#?}", sobel);
        // let overlay = conf.get("overlay").expect(&format!("{dbg}.new | 'overlay' - not found or wrong configuration"));
        // let overlay = OverlayConf::new(&name, overlay);
        // log::trace!("{dbg}.new | overlay: {:#?}", overlay);
        Self {
            cropping,
            gamma,
            // brightness_contrast,
        }
    }
}
//
//
impl Default for NormalizeConf {
    fn default() -> Self {
        Self {
            cropping: CroppingConf::default(),
            gamma: GammaConf::default(),
            // brightness_contrast: BrightnessContrastConf::default(),
            // gausian: GaussianConf::default(),
            // sobel: SobelConf::default(),
            // overlay: OverlayConf::default(),
        }
    }
}

use crate::{
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx},
        CroppingCtx, GrayCtx, ResultCtx,
    }, Image,
};

///
/// Normalize algorithms results, cropp, auto gamma, brightness, contast, gray etc...
#[derive(Debug, Clone)]
pub struct NormalizedCtx {
    /// Normalized image
    pub(super) result: ResultCtx<Image>,
    /// Cropped image
    pub(super) cropping: CroppingCtx,
    /// Gamma-corrected image
    pub(super) auto_gamma: AutoGammaCtx,
    /// Image with corrected brightness and contrast
    pub(super) auto_brightness_and_contrast: AutoBrightnessAndContrastCtx,
    /// Gray scale image
    pub(super) gray: GrayCtx,
}
//
//
impl Default for NormalizedCtx {
    fn default() -> Self {
        Self {
            result: ResultCtx::default(),
            cropping: CroppingCtx::default(),
            auto_gamma: AutoGammaCtx::default(),
            auto_brightness_and_contrast: AutoBrightnessAndContrastCtx::default(),
            gray: GrayCtx::default(),
        }
    }
}


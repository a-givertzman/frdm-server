use crate::{
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx},
        geometry_defect::GeometryDefectCtx, width_emissions::WidthEmissionsCtx,
        CroppingCtx, DetectingContoursCvCtx, EdgeDetectionCtx, RopeDimensionsCtx, GrayCtx,
        InitialCtx, ResultCtx
    },
};
use super::testing_ctx::TestingCtx;
///
/// # Calculation context
/// - Provides read/write access to initial
/// - R/W access to the isoleted data of each step of computations
#[derive(Debug, Clone)]
pub struct Context {
    /// where store source frame
    pub(super) initial: InitialCtx,
    /// Common result image from current step
    pub(super) result: ResultCtx,
    /// Filtered and binarised image
    pub(super) detecting_contours_cv: DetectingContoursCvCtx,
    /// Cropped image
    pub(super) cropping: CroppingCtx,
    /// Gamma-corrected image
    pub(super) auto_gamma: AutoGammaCtx,
    /// Image with corrected brightness and contrast
    pub(super) auto_brightness_and_contrast: AutoBrightnessAndContrastCtx,
    /// Gray scale image
    pub(super) gray: GrayCtx,
    /// points of rope perimeter
    pub(super) edge_detection: EdgeDetectionCtx,
    /// Rope calculated dimensions
    pub(super) rope_dimensions: RopeDimensionsCtx,
    /// points that deviate in width from the threshold
    pub(super) width_emissions: WidthEmissionsCtx,
    /// result of detecting [GeometryDefect's](design/theory/geometry_rope_defects.md)
    pub(super) geometry_defect: GeometryDefectCtx,
    ///
    /// Uset for testing only
    #[allow(dead_code)]
    pub testing: Option<TestingCtx>,
}
//
//
impl Context {
    ///
    /// New instance [Context]
    /// - 'initial' - [InitialCtx] instance, where store initial data
    pub fn new(initial: InitialCtx) -> Self {
        Self {
            initial,
            result: ResultCtx::default(),
            detecting_contours_cv: DetectingContoursCvCtx::default(),
            cropping: CroppingCtx::default(),
            auto_gamma: AutoGammaCtx::default(),
            auto_brightness_and_contrast: AutoBrightnessAndContrastCtx::default(),
            gray: GrayCtx::default(),
            edge_detection: EdgeDetectionCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            width_emissions: WidthEmissionsCtx::default(),
            geometry_defect: GeometryDefectCtx::default(),
            testing: None,
        }
    }
}
    
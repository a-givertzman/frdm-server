use crate::algorithm::{
    geometry_defect::GeometryDefectCtx, width_emissions::WidthEmissionsCtx, DetectingContoursCvCtx, EdgeDetectionCtx, InitialCtx
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
    /// Filtered and binarised image
    pub(super) detecting_contours_cv: DetectingContoursCvCtx,
    /// points of rope perimeter
    pub(super) edge_detection: EdgeDetectionCtx,
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
            detecting_contours_cv: DetectingContoursCvCtx::default(),
            edge_detection: EdgeDetectionCtx::default(),
            width_emissions: WidthEmissionsCtx::default(),
            geometry_defect: GeometryDefectCtx::default(),
            testing: None,
        }
    }
}
    
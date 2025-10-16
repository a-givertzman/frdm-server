use crate::{
    algorithm::{
        GeometryDefectType, NormalizedCtx,
        FastScanCtx, FineScanCtx, InitialCtx,
        FineConvexCtx, ResultCtx,
    },
    domain::Image,
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
    /// Result of last evaluated step
    pub(super) result: ResultCtx<Image>,
    /// Normalize algorithms results, cropp, auto gamma, brightness, contast, gray etc...
    pub(super) normalized: NormalizedCtx,
    // /// Points that deviate in width from the threshold
    // pub(super) width_emissions: WidthEmissionsCtx,
    /// `FastScan` algorithm results
    pub(super) fast_scan: FastScanCtx,
    /// `FineScan` algorithm results
    pub(super) fine_scan: FineScanCtx,
    /// Result of `FineScan` convex - solid contour
    pub(super) convex: FineConvexCtx,
    /// Result of detecting [GeometryDefect's](design/theory/geometry_rope_defects.md)
    pub(super) defects: ResultCtx<Vec<GeometryDefectType>>,
    ///
    /// Used for testing only
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
            normalized: NormalizedCtx::default(),
            // width_emissions: WidthEmissionsCtx::default(),
            fast_scan: FastScanCtx::default(),
            fine_scan: FineScanCtx::default(),
            convex: FineConvexCtx::default(),
            defects: ResultCtx::default(),
            testing: None,
        }
    }
}
    
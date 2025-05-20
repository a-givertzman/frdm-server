use crate::algorithm::{
    geometry_defect::{ContractionCtx, ExpansionCtx, GrooveCtx, MoundCtx}, 
    width_emissions::WidthEmissionsCtx, 
    InitialCtx, 
    InitialPoints
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
    /// points of rope perimeter
    pub(super) initial_points: InitialPoints<usize>,
    /// points that deviate in width from the threshold
    pub(super) width_emissions: WidthEmissionsCtx,
    /// result of detecting defect 'Contraction'
    pub(super) contraction: ContractionCtx,
    /// result of detecting defect 'Expansion'
    pub(super) expansion: ExpansionCtx,
    /// result of detecting defect 'Groove'
    pub(super) groove: GrooveCtx,
    /// result of detecting defect 'Mound'
    pub(super) mound: MoundCtx,
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
            initial_points: InitialPoints::default(),
            width_emissions: WidthEmissionsCtx::default(),
            contraction: ContractionCtx::default(),
            expansion: ExpansionCtx::default(),
            groove: GrooveCtx::default(),
            mound: MoundCtx::default(),
            testing: None,
        }
    }
}
    
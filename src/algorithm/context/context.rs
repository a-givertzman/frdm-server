use crate::algorithm::{geometry_defect::contraciton::ContractionCtx, InitialCtx, InitialPoints};
use super::testing_ctx::TestingCtx;
///
/// # Calculation context
/// - Provides read/write access to initial
/// - R/W access to the isoleted data of each step of computations
#[derive(Debug, Clone)]
pub struct Context {
    /// where store ...
    pub(super) initial: InitialCtx,
    /// 
    pub(super) initial_points: InitialPoints<usize>,
    /// TODO
    pub(super) contraction: ContractionCtx,
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
            contraction: ContractionCtx::default(),
            testing: None,
        }
    }
}
    
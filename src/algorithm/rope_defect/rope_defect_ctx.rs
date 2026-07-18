use std::marker::PhantomData;
use crate::algorithm::RopeDefectKind;
///
/// Store result of `GeometryDefect`
/// - Index of the frame containing the defect
/// - The kind of the geometry defect
#[derive(Debug, Clone, Default)]
pub struct RopeDefectCtx<Branch> {
    pub result: Vec<RopeDefectKind>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> RopeDefectCtx<Branch> {
    pub fn new(result: Vec<RopeDefectKind>) -> Self {
        Self { result, branch: PhantomData }
    }
}
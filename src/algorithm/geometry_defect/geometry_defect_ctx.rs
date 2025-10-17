use std::marker::PhantomData;
use crate::algorithm::GeometryDefectType;
///
/// Store result of `GeometryDefect`
/// - Index of the frame containing the defect
/// - The kind of the geometry defect
#[derive(Debug, Clone, Default)]
pub struct GeometryDefectCtx<Branch> {
    pub result: Vec<GeometryDefectType>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> GeometryDefectCtx<Branch> {
    pub fn new(result: Vec<GeometryDefectType>) -> Self {
        Self { result, branch: PhantomData }
    }
}
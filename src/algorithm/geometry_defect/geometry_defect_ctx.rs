use crate::algorithm::geometry_defect::GeometryDefectType;
///
/// Store result of `GeometryDefect`
/// - Index of the frame containing the defect
/// - The kind of the geometry defect
#[derive(Debug, Clone, Default)]
pub struct GeometryDefectCtx {
    pub result: Vec<GeometryDefectType>,

}
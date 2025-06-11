use crate::algorithm::geometry_defect::GeometryDefectType;
///
/// Store result of `GeometryDefect`
#[derive(Debug, Clone, Default)]
pub struct GeometryDefectCtx {
    pub result: Vec<GeometryDefectType>,

}
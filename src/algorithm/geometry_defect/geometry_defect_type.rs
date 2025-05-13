use super::expansion::Expansion;
///
/// Enum of geometry defect type's
pub enum GeometryDefectType {
    Expansion(Expansion),
    Extraction,
    Groove,
    Mound,

}
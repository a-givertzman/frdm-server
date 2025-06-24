///
/// Enum of [geometry defect type's](design/theory/geometry_rope_defects.md)
/// containing the position of defect withing a frame
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryDefectType {
    /// Doc it...
    Expansion,
    /// Doc it...
    Compressing,
    /// Doc it...
    Hill,
    /// Doc it...
    Pit,
}
use crate::algorithm::mad::Bond;
///
/// Enum of [geometry defect type's](design/theory/geometry_rope_defects.md)
#[derive(Debug, Clone, PartialEq)]
pub enum GeometryDefectType {
    Expansion(Bond<usize>),
    Contraction(Bond<usize>),
    Groove(Bond<usize>),
    Mound(Bond<usize>),

}
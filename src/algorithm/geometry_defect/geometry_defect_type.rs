use super::{
    Contraction, 
    Expansion, 
    Groove, 
    Mound,
};

///
/// Enum of geometry defect type's
pub enum GeometryDefectType {
    Expansion(Expansion),
    Contraction(Contraction),
    Groove(Groove),
    Mound(Mound),

}
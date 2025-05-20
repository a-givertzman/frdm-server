use crate::algorithm::mad::Bond;
///
/// Store result of `Contraction`
#[derive(Debug, Clone, Default)]
pub struct ContractionCtx {
    pub result: Vec<Bond<usize>>
}
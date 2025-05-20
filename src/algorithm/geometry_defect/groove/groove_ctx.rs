use crate::algorithm::mad::Bond;
///
/// Store result of `Groove`
#[derive(Debug, Clone, Default)]
pub struct GrooveCtx {
    pub result: Vec<Bond<usize>>
}
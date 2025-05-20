use crate::algorithm::mad::Bond;
///
/// Store result of `Expansion`
#[derive(Debug, Clone, Default)]
pub struct ExpansionCtx {
    pub result: Vec<Bond<usize>>
}
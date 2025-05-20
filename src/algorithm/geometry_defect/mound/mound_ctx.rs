use crate::algorithm::mad::Bond;
///
/// Store result of `Mound`
#[derive(Debug, Default, Clone)]
pub struct MoundCtx {
    pub result: Vec<Bond<usize>>
}
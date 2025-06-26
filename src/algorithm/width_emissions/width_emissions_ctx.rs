use crate::algorithm::mad::Bond;
///
/// Store result of `WidthEmissions`
#[derive(Debug, Clone, Default)]
pub struct WidthEmissionsCtx {
    pub result: Vec<Bond<usize>>
}
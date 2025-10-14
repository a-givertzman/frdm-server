use crate::algorithm::mad::Bond;
///
/// Result of rope `WidthEmissions`
#[derive(Debug, Clone, Default)]
pub struct WidthEmissionsCtx {
    pub result: Vec<Bond<usize>>
}
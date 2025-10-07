///
/// Store result of algorithm `Mad`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MadCtx {
    pub median: f64,
    pub mad: f64,
}
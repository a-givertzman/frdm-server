///
/// Store result of `RopeDimensions`
/// - `width` - Average width of the detected rope
/// - `width` - Calculated square of the detected rope
#[derive(Debug, Clone, Default)]
pub struct RopeDimensionsCtx {
    pub width: f64,
    pub square: f64,
}
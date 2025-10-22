use std::marker::PhantomData;

///
/// Store result of `RopeDimensions`
/// - `width` - Average width of the detected rope
/// - `width` - Calculated square of the detected rope
#[derive(Debug, Clone, Default)]
pub struct RopeDimensionsCtx<Branch> {
    pub width: f64,
    pub square: f64,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> RopeDimensionsCtx<Branch> {
    pub fn new(width: f64, square: f64) -> Self {
        Self { width, square, branch: PhantomData }
    }
}
use std::marker::PhantomData;
use crate::algorithm::mad::Bond;

///
/// Result of rope `WidthEmissions`
#[derive(Debug, Clone, Default)]
pub struct WidthEmissionsCtx<Branch> {
    pub result: Vec<Bond<usize>>,
    branch: PhantomData<Branch>,
}
impl<Branch> WidthEmissionsCtx<Branch> {
    pub fn new(result: Vec<Bond<usize>>) -> Self {
        Self { result, branch: PhantomData }
    }
}
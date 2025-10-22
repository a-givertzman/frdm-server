use std::marker::PhantomData;
use crate::algorithm::mad::Bond;

///
/// Result of rope `RopeDistortions`
#[derive(Debug, Clone, Default)]
pub struct RopeDistortionsCtx<Branch> {
    pub result: Vec<Bond<usize>>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDistortionsCtx<Branch> {
    pub fn new(result: Vec<Bond<usize>>) -> Self {
        Self { result, branch: PhantomData }
    }
}
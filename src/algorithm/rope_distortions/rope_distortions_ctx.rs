use std::marker::PhantomData;
use crate::algorithm::mad::Bend;

///
/// Result of rope `RopeDistortions`
#[derive(Debug, Clone, Default)]
pub struct RopeDistortionsCtx<Branch> {
    pub result: Vec<Bend<usize>>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDistortionsCtx<Branch> {
    pub fn new(result: Vec<Bend<usize>>) -> Self {
        Self { result, branch: PhantomData }
    }
}
use std::marker::PhantomData;
use crate::algorithm::{Bend, MadCtx};

///
/// Result of rope `RopeDistortions`
#[derive(Debug, Clone, Default)]
pub struct RopeDistortionsCtx<Branch> {
    pub result: Vec<Bend<usize>>,
    pub mad: MadCtx,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDistortionsCtx<Branch> {
    pub fn new(result: Vec<Bend<usize>>, mad: MadCtx) -> Self {
        Self {
            result,
            mad,
            branch: PhantomData }
    }
}
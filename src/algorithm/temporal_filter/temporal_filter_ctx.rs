use std::marker::PhantomData;
use crate::domain::Image;
///
/// TemporalFilter result image
#[derive(Debug, Clone)]
pub struct TemporalFilterCtx<Branch> {
    pub frame: Image,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> TemporalFilterCtx<Branch> {
    pub fn new(frame: Image) -> Self {
        Self { 
            frame,
            branch: PhantomData,
         }
    }
}
//
//
impl<Branch> Default for TemporalFilterCtx<Branch> {
    fn default() -> Self {
        Self { 
            frame: Image::default(),
            branch: PhantomData,
         }
    }
}

use crate::{algorithm::Edges};
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct FastEdgesCtx {
    pub result: Edges<usize>,
}
//
//
impl Default for FastEdgesCtx {
    fn default() -> Self {
        Self { 
            result: Edges::default()
         }
    }
}

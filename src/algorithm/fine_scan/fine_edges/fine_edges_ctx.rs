use crate::{algorithm::Edges};
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct FineEdgesCtx {
    pub result: Edges<usize>,
}
//
//
impl Default for FineEdgesCtx {
    fn default() -> Self {
        Self { 
            result: Edges::default()
         }
    }
}

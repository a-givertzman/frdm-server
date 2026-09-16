use crate::{algorithm::Edges};
///
/// Context store of [FastEdges](src/scan/edge_detection.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct FineEdgesCtx {
    pub edges: Edges<usize>,
}
//
//
impl Default for FineEdgesCtx {
    fn default() -> Self {
        Self { 
            edges: Edges::default()
         }
    }
}

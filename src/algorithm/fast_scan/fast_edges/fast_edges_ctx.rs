use crate::{algorithm::Edges};
///
/// Context store of [FastEdges](src/scan/edge_detection.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct FastEdgesCtx {
    pub edges: Edges<usize>,
}
//
//
impl Default for FastEdgesCtx {
    fn default() -> Self {
        Self { 
            edges: Edges::default()
         }
    }
}

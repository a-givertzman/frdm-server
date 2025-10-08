use crate::{algorithm::InitialPoints};
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeDetectionCtx {
    pub result: InitialPoints<usize>,
}
//
//
impl Default for EdgeDetectionCtx {
    fn default() -> Self {
        Self { 
            result: InitialPoints::default()
         }
    }
}

use crate::{algorithm::InitialPoints, domain::graham::dot::Dot};
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
#[derive(Debug, PartialEq)]
pub struct EdgeDetectionCtx {
    pub result: InitialPoints<usize>,
}
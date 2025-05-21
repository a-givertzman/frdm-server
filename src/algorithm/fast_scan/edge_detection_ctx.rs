use crate::domain::graham::dot::Dot;
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
#[derive(Debug, PartialEq)]
pub struct EdgeDetectionCtx {
    pub upper_edge: Vec<Dot<usize>>,
    pub lower_edge: Vec<Dot<usize>>,
}
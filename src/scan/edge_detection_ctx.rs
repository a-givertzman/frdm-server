use crate::domain::graham::dot::Dot;
///
/// Context store of [EdgeDetection](src/scan/edge_detection.rs)
pub struct EdgeDetectionCtx {
    pub upper_edge: Vec<Dot<isize>>,
    pub lower_edge: Vec<Dot<isize>>,
}
use crate::domain::graham::dot::Dot;

pub struct EdgeDetectionCtx {
    pub upper_edge: Vec<Dot<isize>>,
    pub lower_edge: Vec<Dot<isize>>,
}
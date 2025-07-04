use crate::domain::{Dot, Eval};

///
/// First step of Graham scan
/// Find lowest y coordinate point
pub struct FindStart {
    points: Option<Vec<Dot<isize>>>,
}
//
//
impl FindStart {
    ///
    /// Returns [FindStart] new instance
    pub fn new(points: impl Into<Vec<Dot<isize>>>) -> Self {
        Self {
            points: Some(points.into()),
        }
    }
}
//
//
impl Eval<(), FindStartCtx> for FindStart {
    fn eval(&self, _: ()) -> FindStartCtx {
        let points = self.points.clone().take().unwrap();
        let start = points
            .iter()
            .enumerate()
            .min_by(|(_, dot1), (_, dot2)| {
                dot1.y.cmp(&dot2.y)
            });
        match start {
            Some((start, _)) => FindStartCtx { points, start: start as isize },
            None => FindStartCtx { points, start: 0 },
        }
    }
}
///
/// 
#[derive(Debug, Clone)]
pub struct FindStartCtx {
    pub points: Vec<Dot<isize>>,
    pub start: isize,
}

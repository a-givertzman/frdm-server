use std::cmp::Ordering;

use crate::domain::eval::eval::Eval;

use super::{dot::Dot, find_start::FindStartCtx};
///
/// Second step of Graham scan
/// Points sorted in increasing order of the angle they and the `Start` point make with the x-axis.
pub struct Sort {
    eval: Box<dyn Eval<(), FindStartCtx>>,
}
//
//
impl Sort {
    ///
    /// Returns [SortByX] new instance
    pub fn new(eval: impl Eval<(), FindStartCtx> + 'static) -> Self {
        Self {
            eval: Box::new(eval),
        }
    }
}
//
//
impl Eval<(), SortByAngCtx> for Sort {
    fn eval(&mut self, _: ()) -> SortByAngCtx {
        let mut ctx = self.eval.eval(());
        ctx.points.swap(ctx.start, 0);
        let dot0 = ctx.points[0];
        ctx.points.sort_by(|dot1, dot2| {
            let ang = (dot1.x - dot0.x) * (dot2.y - dot0.y) - (dot1.y - dot0.y) * (dot2.x - dot0.x);
            if ang < 0 {
                Ordering::Greater
            } else if ang > 0 {
                Ordering::Less                    
            } else {
                Ordering::Equal
            }
        });

        SortByAngCtx { points: ctx.points, start: ctx.start }
    }
}
///
/// 
#[derive(Debug, Clone)]
pub struct SortByAngCtx {
    pub points: Vec<Dot<isize>>,
    pub start: usize,
}
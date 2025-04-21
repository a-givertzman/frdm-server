use std::cmp::Ordering;

use crate::domain::eval::eval::Eval;

use super::{dot::Dot, find_start::FindStartCtx, sort::SortByAngCtx};
///
/// Third step of Graham scan
/// Points are stored in stack, building the convex hill
pub struct Build_hull {
    eval: Box<dyn Eval<(), SortByAngCtx>>,
}
//
//
impl Build_hull {
    ///
    /// Returns [SortByX] new instance
    pub fn new(eval: impl Eval<(), SortByAngCtx> + 'static) -> Self {
        Self {
            eval: Box::new(eval),
        }
    }
}
//
//
impl Eval<(), HullCtx> for Build_hull {
    fn eval(&mut self, _: ()) -> HullCtx {
        let mut ctx = self.eval.eval(());
        let mut hull = Vec::new();
        hull.push(ctx.points[ctx.start]);
        hull.push(ctx.points[(ctx.start + 1) % ctx.points.len()]);
        for i in 2..ctx.points.len() {
            let next_dot = ctx.points[(ctx.start + i) % ctx.points.len()];
            while hull.len() >= 2 {
                let dot1 = hull[hull.len() - 2];
                let dot2 = hull[hull.len() - 1];
                let ang = (dot2.x - dot1.x) * (next_dot.y - dot1.y) - (dot2.y - dot1.y) * (next_dot.x - dot1.x);
                if ang <= 0 {
                    hull.pop();
                } else {
                    hull.push(next_dot);
                    break;
                }
            }
        }
        HullCtx { hull }
    }
}

///
/// 
#[derive(Debug)]
pub struct HullCtx {
    pub hull: Vec<Dot<isize>>,
}
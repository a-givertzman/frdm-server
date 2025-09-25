use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
            geometry_defect::Threshold, mad::{
                Bond, 
                MadCtx
            }, ContextRead, ContextWrite, EdgeDetectionCtx, EvalResult, Side
        }, 
    domain::{
            Dot, 
            Error, 
            Eval, 
            Image,
        }
    };
use super::WidthEmissionsCtx;
///
/// Finding width emissions the rope
pub struct WidthEmissions {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, MadCtx> + Send>,
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl WidthEmissions {
    ///
    /// New instance [WidthEmissions]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, MadCtx> + Send + 'static,
        ctx: impl Eval<(), EvalResult> + Send + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("WidthEmissions"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Compute width between initial dots
    fn points_width(initial_points_upper: Vec<Dot<usize>>, initial_points_lower: Vec<Dot<usize>>) -> Vec<usize> {
        let mut dots_width = Vec::new();
        for i in 0..initial_points_upper.len() { // `for` only for one vector cause they must be same length
            let width = initial_points_upper[i].y - initial_points_lower[i].y;
            dots_width.push(width);
        };
        dots_width
    }
    ///
    /// Find emissions
    fn emissions(
        initial_points_upper: Vec<Dot<usize>>, 
        initial_points_lower: Vec<Dot<usize>>, 
        median: f64, 
        mad: f64, 
        threshold: f64
    ) -> Vec<Bond<usize>> {
        let mut emissions = Vec::new();
        for i in 0..initial_points_upper.len() { // `for` only for one vector cause they must be same length
            let deviation = ((initial_points_upper[i].y - initial_points_lower[i].y) as f64 - median).abs();
            if deviation > threshold * mad {
                emissions.push(
                    Bond {
                        x: initial_points_upper[i].x,
                        y: initial_points_upper[i].y,
                    }
                );
                emissions.push(
                    Bond {
                        x: initial_points_lower[i].x,
                        y: initial_points_lower[i].y,
                    }
                );
            }
        };
        emissions
    }
}
//
//
impl Eval<Image, EvalResult> for WidthEmissions {
    fn eval(&self, _: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let edge_detection_ctx = ContextRead::<EdgeDetectionCtx>::read(&ctx);
                let initial_points_upper = edge_detection_ctx.result.get(Side::Upper);
                let initial_points_lower = edge_detection_ctx.result.get(Side::Lower);
                let mad_result = self.mad.eval(
                    Self::points_width(
                            initial_points_upper.clone(),
                            initial_points_lower.clone(),
                    )
                );
                let result = WidthEmissionsCtx {
                    result: Self::emissions(
                                initial_points_upper.clone(),
                                initial_points_lower.clone(),
                                mad_result.median,
                                mad_result.mad,
                                self.threshold.0
                            ),
                };
                ctx.write(result)
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}

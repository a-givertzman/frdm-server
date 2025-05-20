use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        geometry_defect::Threshold, 
        mad::{
            Bond, 
            MadCtx
        }, 
        width_emissions::WidthEmissionsCtx, 
        ContextRead, 
        ContextWrite, 
        EvalResult, 
        InitialCtx, 
        InitialPoints, 
        Side
    }, 
    domain::{
        Error, 
        Eval
    }
};
use super::contraction_ctx::ContractionCtx;
///
/// Detecting contraction on the rope
pub struct Contraction {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, MadCtx> + Send>,
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl Contraction {
    ///
    /// New instance [Contraction]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, MadCtx> + Send + 'static,
        ctx: impl Eval<(), EvalResult> + Send + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("Contraction"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), EvalResult> for Contraction {
    fn eval(&self, _: ()) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let _initial: &InitialCtx = ctx.read();
                let initial_points = ContextRead::<InitialPoints<usize>>::read(&ctx);
                let initial_points_upper = initial_points.get(Side::Upper);
                let initial_points_lower = initial_points.get(Side::Lower);
                let width_emissions_result = ContextRead::<WidthEmissionsCtx>::read(&ctx);
                let mad_of_upper_points = self.mad.eval(
                    initial_points_upper.iter()
                        .map(|dot| dot.y)
                        .collect(),
                );
                let mad_of_lower_points = self.mad.eval(
                    initial_points_lower
                        .iter()
                        .map(|dot| dot.y)
                        .collect()
                );
                let mut result: Vec<Bond<usize>> = Vec::new();
                for i in (0..width_emissions_result.result.len()-1).step_by(2) {
                    let upper_point = width_emissions_result.result[i];
                    let lower_point = width_emissions_result.result[i+1];
                    let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
                    let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
                    if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
                       (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
                        result.push(
                            upper_point
                        );
                        result.push(
                            lower_point
                        );
                    }
                }
                let result = ContractionCtx {
                    result: result,
                };
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

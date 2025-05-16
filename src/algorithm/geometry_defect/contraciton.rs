use sal_core::dbg::Dbg;
use crate::{algorithm::{mad::{Bond, MadCtx}, width_emissions::width_emissions::WidthEmissions, ContextRead, ContextWrite, EvalResult, InitialCtx, InitialPoints, Side}, domain::{Error, Eval}};
use super::Threshold;
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
                let initial: &InitialCtx = ctx.read();
                let initial_points: &InitialPoints<usize> = ctx.read();
                let initial_points_upper = initial_points.get(Side::Upper);
                let initial_points_lower = initial_points.get(Side::Lower);
                let width_emissions_result = WidthEmissions::new(
                    self.threshold,
                    initial_points_upper,
                    initial_points_lower,
                ).eval(());
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
                let threshold = 1.1;
                for i in (0..width_emissions_result.result.len()-1).step_by(2) {
                    let upper_point = width_emissions_result.result[i];
                    let lower_point = width_emissions_result.result[i+1];
                    let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
                    let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
                    if (deviation_upper < -threshold * mad_of_upper_points.mad) &&
                       (deviation_lower > threshold * mad_of_lower_points.mad) {
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
///
/// Store result of [Contraction]
#[derive(Debug, Clone, Default)]
pub struct ContractionCtx {
    pub result: Vec<Bond<usize>>
}
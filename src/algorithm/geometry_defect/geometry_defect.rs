use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        geometry_defect::{
            GeometryDefectCtx, GeometryDefectType, Threshold
        }, mad::{Bond, MadCtx}, width_emissions::WidthEmissionsCtx, ContextRead, ContextWrite, EvalResult, InitialPoints, Side
    }, 
    domain::{Error, Eval}
};
///
/// Represents detecting [geometry defect's](design/theory/geometry_rope_defects.md)
pub struct GeometryDefect {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, MadCtx> + Send>,
    ctx: Box<dyn Eval<(), EvalResult> + Send>,
}
//
//
impl GeometryDefect {
    ///
    /// New instance [GeometryDefect]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, MadCtx> + Send + 'static,
        ctx: impl Eval<(), EvalResult> + Send + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("GeometryDefect"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Detecting expansion points
    fn expansion(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, result: &mut Vec<GeometryDefectType>, mad_of_upper_points: MadCtx, mad_of_lower_points: MadCtx) {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper > self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower < -self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Expansion(upper_point)
            );
            result.push(
                GeometryDefectType::Expansion(lower_point)
            );
        }
    }
    ///
    /// Detecting contraction points
    fn contraction(&self, result: &mut Vec<GeometryDefectType>, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: MadCtx, mad_of_lower_points: MadCtx) {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Contraction(upper_point)
            );
            result.push(
                GeometryDefectType::Contraction(lower_point)
            );
        }
    }
    ///
    /// Detecting groove points
    fn groove(&self, result: &mut Vec<GeometryDefectType>, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: MadCtx, mad_of_lower_points: MadCtx) {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        // checking groove on lower points
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Groove(lower_point)
            );
        }
        // checking groove on upper points
        else if (deviation_upper > self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Groove(upper_point)
            );
        }
    }
    ///
    /// Detecting mound points
    fn mound(&self, result: &mut Vec<GeometryDefectType>, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: MadCtx, mad_of_lower_points: MadCtx) {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        // checking mound on lower points
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower < -self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Mound(lower_point)
            );
        }
        // checking mound on upper points
        else if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            result.push(
                GeometryDefectType::Mound(upper_point)
            );
        }
    }   
}
//
//
impl Eval<(), EvalResult> for GeometryDefect {
    fn eval(&self, _: ()) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let mut result: Vec<GeometryDefectType> = Vec::new();
                let initial_points = ContextRead::<InitialPoints<usize>>::read(&ctx);
                let initial_points_upper = initial_points.get(Side::Upper);
                let initial_points_lower = initial_points.get(Side::Lower);
                let width_emissions_result = ContextRead::<WidthEmissionsCtx>::read(&ctx).result.clone();
                if width_emissions_result.is_empty() {
                    let result = GeometryDefectCtx {
                        result,
                    };
                    log::debug!("Frame without defect's");
                    return ctx.write(result)
                }
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
                for i in (0..width_emissions_result.len()-1).step_by(2) {
                    let upper_point = width_emissions_result[i];
                    let lower_point = width_emissions_result[i+1];
                    self.expansion(upper_point, lower_point, &mut result, mad_of_upper_points.clone(), mad_of_lower_points.clone());
                    self.contraction(&mut result, upper_point, lower_point, mad_of_upper_points.clone(), mad_of_lower_points.clone());
                    self.groove(&mut result, upper_point, lower_point, mad_of_upper_points.clone(), mad_of_lower_points.clone());
                    self.mound(&mut result, upper_point, lower_point, mad_of_upper_points.clone(), mad_of_lower_points.clone());

                }
                let result = GeometryDefectCtx {
                    result,
                };
                ctx.write(result)
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
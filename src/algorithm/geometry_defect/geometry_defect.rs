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
    /// Detecting both sides width growing
    fn expansion(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper > self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower < -self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting both sides width reduction
    fn compressing(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting one side raising
    fn hill(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        // checking groove on lower points
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
            // checking groove on upper points
        } else if (deviation_upper > self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting one side drooping
    fn pit(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        // checking mound on lower points
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower < -self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
            // checking mound on upper points
        } else if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
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
                    match self.expansion(upper_point, lower_point, &mad_of_upper_points, &mad_of_lower_points) {
                        Some(_) => result.push(GeometryDefectType::Expansion),
                        None => match self.compressing(upper_point, lower_point, &mad_of_upper_points, &mad_of_lower_points) {
                            Some(_) => result.push(GeometryDefectType::Compressing),
                            None => match self.hill(upper_point, lower_point, &mad_of_upper_points, &mad_of_lower_points) {
                                Some(_) => result.push(GeometryDefectType::Hill),
                                None => match self.pit(upper_point, lower_point, &mad_of_upper_points, &mad_of_lower_points) {
                                    Some(_) => result.push(GeometryDefectType::Pit),
                                    None => {}
                                }
                            }
                        }
                    }
                }
                result = result.into_iter().fold(vec![], |mut acc, defect| {
                    match acc.last() {
                        Some(prev) => {
                            if prev != &defect {
                                acc.push(defect);
                            }
                        }
                        None => acc.push(defect),
                    }
                    acc
                });
                let result = GeometryDefectCtx {
                    result,
                };
                ctx.write(result)
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
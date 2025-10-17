use std::{any::TypeId, marker::PhantomData};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        GeometryDefectCtx, GeometryDefectType, Threshold,
        mad::{Bond, MadCtx}, width_emissions::WidthEmissionsCtx, ContextRead, ContextWrite,
        FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx,
    }, 
    domain::{Error, Eval, Image},
};

///
/// Represents detecting [geometry defect's](design/theory/geometry_rope_defects.md)
pub struct GeometryDefect<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, MadCtx> + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> GeometryDefect<Branch> {
    ///
    /// New instance [GeometryDefect]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, MadCtx> + Send + Sync + 'static,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("GeometryDefect"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
            branch: PhantomData,
        }
    }
    ///
    /// Detecting [both sides width growing](design/references/GOST_33718-2015.pdf)
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
    /// Detecting [both sides width reduction](design/references/GOST_33718-2015.pdf)
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
    /// Detecting [one side drooping](design/references/GOST_33718-2015.pdf)
    fn pit(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
        (deviation_lower > self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        } else if (deviation_upper > self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
    }
    ///
    /// Detecting [one side raising](design/references/GOST_33718-2015.pdf)
    fn hill(&self, upper_point: Bond<usize>, lower_point: Bond<usize>, mad_of_upper_points: &MadCtx, mad_of_lower_points: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - mad_of_upper_points.median;
        let deviation_lower = lower_point.y as f64 - mad_of_lower_points.median;
        if (deviation_upper.abs() < self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower < -self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        } else if (deviation_upper < -self.threshold.0 * mad_of_upper_points.mad) &&
            (deviation_lower.abs() < self.threshold.0 * mad_of_lower_points.mad) {
            return Some(());
        }
        None
    }   
}
//
//
impl<Branch: 'static> Eval<Image, EvalResult> for GeometryDefect<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let mut result: Vec<GeometryDefectType> = Vec::new();
                // let width_emissions_result = ContextRead::<WidthEmissionsCtx>::read(&ctx).result.clone();
                let width_emissions_result = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => &ContextRead::<WidthEmissionsCtx<FastScanCtx>>::read(&ctx).result,
                    typ if typ == TypeId::of::<FineScanCtx>() => &ContextRead::<WidthEmissionsCtx<FineScanCtx>>::read(&ctx).result,
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                if width_emissions_result.is_empty() {
                    log::debug!("Frame without defect's");
                    return match TypeId::of::<Branch>() {
                        typ if typ == TypeId::of::<FastScanCtx>() => ContextWrite::<GeometryDefectCtx<FastScanCtx>>::write(
                            ctx, GeometryDefectCtx::new(result),
                        ),
                        typ if typ == TypeId::of::<FineScanCtx>() => ContextWrite::<GeometryDefectCtx<FineScanCtx>>::write(
                            ctx, GeometryDefectCtx::new(result),
                        ),
                        _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                    }
                }
                // let initial_points = ContextRead::<FastEdgesCtx>::read(&ctx);
                // let upper = initial_points.result.get(Side::Upper);
                // let lower = initial_points.result.get(Side::Lower);
                let (upper, lower) = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        let edges_ctx: &FastEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        let edges_ctx: &FineEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                let mad_of_upper_points = self.mad.eval(
                    upper.iter().map(|dot| dot.y).collect(),
                );
                let mad_of_lower_points = self.mad.eval(
                    lower.iter().map(|dot| dot.y).collect()
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
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => ContextWrite::<GeometryDefectCtx<FastScanCtx>>::write(ctx, GeometryDefectCtx::new(result)),
                    typ if typ == TypeId::of::<FineScanCtx>() => ContextWrite::<GeometryDefectCtx<FineScanCtx>>::write(ctx, GeometryDefectCtx::new(result)),
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
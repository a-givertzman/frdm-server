use std::{any::TypeId, marker::PhantomData};

use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        geometry_defect::Threshold, mad::{Bond, MadCtx},
        ContextRead, ContextWrite, FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx,
    },
    domain::{Dot, Error, Eval, Image}
};
use super::WidthEmissionsCtx;
///
/// Finding width emissions the rope
pub struct WidthEmissions<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> WidthEmissions<Branch> {
    ///
    /// New instance [WidthEmissions]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync + 'static,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("WidthEmissions"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
            branch: PhantomData,
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
impl<Branch: 'static> Eval<Image, EvalResult> for WidthEmissions<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
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
                let (result, mad) = if upper.is_empty() || lower.is_empty() {
                    (vec![], MadCtx::default())
                } else {
                    let mad = self.mad.eval(Self::points_width(upper.clone(), lower.clone())).map_err(|err| error.pass(err))?;
                    (Self::emissions(upper, lower, mad.median, mad.mad, self.threshold.0), mad)
                };
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        log::debug!("WidthEmissions<FastScanCtx>.eval | mad: {:?}", mad);
                        log::debug!("WidthEmissions<FastScanCtx>.eval | defects: {:?}", result);
                        ctx.write(WidthEmissionsCtx::<FastScanCtx>::new(result))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        log::debug!("WidthEmissions<FastScanCtx>.eval | mad: {:?}", mad);
                        log::debug!("WidthEmissions<FineScanCtx>.eval | defects: {:?}", result);
                        ctx.write(WidthEmissionsCtx::<FineScanCtx>::new(result))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}

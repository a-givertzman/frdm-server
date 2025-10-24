use std::{any::TypeId, marker::PhantomData, time::Instant};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        Threshold, mad::{Bend, MadCtx},
        ContextRead, ContextWrite, FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx, RopeDistortionsCtx,
    },
    domain::{Dot, Error, Eval, Image}
};
///
/// Finding rope width distortion by calculating the deviation of the rope side
/// 
/// Deviation detected by comparing the side deviation with threshold multiplied by Median Absolute Deviation (MAD)
/// 
/// Threshold can be in the range 1.1 ... 1.3, the greater the threshold, the less the sensitivity of the algorithm
pub struct RopeDistortions<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> RopeDistortions<Branch> {
    ///
    /// New instance [RopeDistortions]
    /// 
    /// - `Branch` - the calculation branch [FastScanCtx] or [FineScanCtx]
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync + 'static,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("RopeDistortions"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
            branch: PhantomData,
        }
    }
    ///
    /// Compute width between `upper` and `lower` dots
    fn points_width(upper: &[Dot<usize>], lower: &[Dot<usize>]) -> Vec<usize> {
        let mut dots_width = Vec::new();
        for i in 0..upper.len() { // `for` only for one vector cause they must be same length
            let width = (upper[i].y as isize - lower[i].y as isize).abs() as usize;
            dots_width.push(width);
        };
        dots_width
    }
    ///
    /// Finding rope distortion
    fn distortion(
        upper: Vec<Dot<usize>>, 
        lower: Vec<Dot<usize>>, 
        median: f64, 
        mad: f64, 
        threshold: Threshold,
    ) -> Vec<Bend<usize>> {
        let mut distortion = Vec::new();
        let mut bend: Option<Bend<usize>> = None;
        for (upper, lower) in upper.iter().zip(&lower) { // `for` only for one vector cause they must be same length
            let deviation = ((upper.y as f64 - lower.y as f64).abs() - median).abs();
            if deviation > threshold.0 * mad {
                match &mut bend {
                    Some(bend) => bend.push(*upper, *lower),
                    None => {
                        let mut init = Bend::new();
                        init.push(*upper, *lower);
                        bend = Some(init);
                    },
                }
            } else {
                if let Some(bend) = bend.take() {
                    distortion.push(bend);
                }
            }
        };
        distortion
    }
}
//
//
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDistortions<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
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
                    let mad = self.mad.eval(Self::points_width(&upper, &lower)).map_err(|err| error.pass(err))?;
                    (Self::distortion(upper, lower, mad.median, mad.mad, self.threshold), mad)
                };
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        log::debug!("RopeDistortions<FastScanCtx>.eval | Elapsed: {:?}", t.elapsed());
                        log::trace!("RopeDistortions<FastScanCtx>.eval | mad: {:?}", mad);
                        // log::debug!("RopeDistortions<FastScanCtx>.eval | defects: {:?}", result);
                        ctx.write(RopeDistortionsCtx::<FastScanCtx>::new(result, mad))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        log::debug!("RopeDistortions<FineScanCtx>.eval | Elapsed: {:?}", t.elapsed());
                        log::trace!("RopeDistortions<FineScanCtx>.eval | mad: {:?}", mad);
                        // log::debug!("RopeDistortions<FineScanCtx>.eval | defects: {:?}", result);
                        ctx.write(RopeDistortionsCtx::<FineScanCtx>::new(result, mad))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}

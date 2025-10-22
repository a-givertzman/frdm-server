use std::{any::TypeId, marker::PhantomData, time::Instant};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, FastEdgesCtx, FineEdgesCtx, RopeDimensionsCtx, EvalResult, Side, FastScanCtx, FineScanCtx},
    domain::{Eval, Image},
};
///
/// Rope Dimensions | Verifications of the rope width and square
pub struct RopeDimensions<Branch> {
    rope_width: f64,
    width_tolerance: f64,
    square_tolerance: f64,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> RopeDimensions<Branch> {
    ///
    /// Returns [RopeDimensions] new instance
    /// - `rope_width` - Standart rope width, px
    /// - `width_tolerance` - Tolerance for rope width, %
    /// - `square_tolerance` - Tolerance for rope square, %
    #[allow(unused)]
    pub fn new(rope_width: usize, width_tolerance: f64, square_tolerance: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            rope_width: rope_width as f64,
            width_tolerance: width_tolerance / 100.0,
            square_tolerance: square_tolerance / 100.0,
            ctx: Box::new(ctx),
            branch: PhantomData,
        }
    }
}
//
//
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDimensions<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("RopeDimensions", "eval");
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
                // let result: &FastEdgesCtx = ctx.read();
                // let upper = result.result.get(Side::Upper);
                // let lower = result.result.get(Side::Lower);
                let mut upper_average = 0.0f64;
                let mut lower_average = 0.0f64;
                let mut rope_square = 0.0;
                for (upper, lower) in upper.iter().zip(&lower) {
                    upper_average += upper.y as f64;
                    lower_average += lower.y as f64;
                    rope_square += (upper.y as f64 - lower.y as f64).abs();
                }
                upper_average = upper_average / upper.len() as f64;
                lower_average = lower_average / lower.len() as f64;
                let rope_width = (upper_average - lower_average).abs();
                // log::debug!("RopeDimensions.eval | Average rope_width: {:?} px", rope_width);
                // log::debug!("RopeDimensions.eval | Rope square: {:?} px", rope_square);
                let rope_width_error = (1.0 - rope_width / self.rope_width).abs();
                // log::debug!("RopeDimensions.eval | Rope width error: {:?} % of {}", rope_width_error, self.width_tolerance);
                if rope_width_error >= self.width_tolerance {
                    return Err(error.err(format!("Rope width error: {:.3}%, {rope_width} of {}", rope_width_error, self.rope_width)));
                }
                let rope_square_error = (1.0 - rope_square / (self.rope_width * upper.len() as f64)).abs();
                // log::debug!("RopeDimensions.eval | Rope square error: {:?} % of {}", rope_square_error, self.square_tolerance);
                if rope_square_error >= self.square_tolerance {
                    return Err(error.err(format!("Rope square error: {:.3}%, {rope_square} of {}", rope_square_error, self.rope_width * upper.len() as f64)));
                }
                log::debug!("RopeDimensions.eval | Elapsed: {:?}", t.elapsed());
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(RopeDimensionsCtx::<FastScanCtx>::new(rope_width, rope_square)),
                    typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDimensionsCtx::<FineScanCtx>::new(rope_width, rope_square)),
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

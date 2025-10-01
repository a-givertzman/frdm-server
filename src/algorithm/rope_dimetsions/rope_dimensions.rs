use std::time::Instant;
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EdgeDetectionCtx, EvalResult, Side},
    domain::{Eval, Filter, Image},
};
///
/// Rope Dimensions | Verifications of the rope width and square
pub struct RopeDimensions {
    rope_width: f64,
    width_tolerance: f64,
    square_tolerance: f64,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl RopeDimensions {
    ///
    /// Returns [RopeDimensions] new instance
    /// - `rope_width` - standart rope width, px
    /// - `width_tolerance` - tolerance for rope width, %
    /// - `square_tolerance` - tolerance for rope square, %
    pub fn new(rope_width: usize, width_tolerance: f64, square_tolerance: f64, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            rope_width: rope_width as f64,
            width_tolerance,
            square_tolerance,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for RopeDimensions {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("RopeDimensions", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &EdgeDetectionCtx = ctx.read();
                let upper_points = result.result.get(Side::Upper);
                let lower_points = result.result.get(Side::Lower);
                let mut upper_average = 0.0f64;
                let mut lower_average = 0.0f64;
                let mut square = 0.0;
                for (upper, lower) in upper_points.iter().zip(&lower_points) {
                    upper_average += upper.y as f64;
                    lower_average += lower.y as f64;
                    square += (upper.y as f64 - lower.y as f64).abs();
                }
                upper_average = upper_average / upper_points.len() as f64;
                lower_average = lower_average / lower_points.len() as f64;
                let rope_width = (upper_average - lower_average).abs();
                log::debug!("RopeDimensions.eval | Average rope_width: {:?} px", rope_width);
                log::debug!("RopeDimensions.eval | Rope square: {:?} px", square);
                let rope_width_error = rope_width * 100.0 / self.rope_width;
                if rope_width_error >= self.width_tolerance {
                    return Err(error.err(format!("Rope width error, {rope_width} > {rope_width_error}% of {}", self.rope_width)));
                }
                let rope_square_error = square * 100.0 / (self.rope_width * upper_points.len() as f64);
                if rope_square_error >= self.square_tolerance {
                    return Err(error.err(format!("Rope square error, {rope_width} > {rope_square_error}% of {}", self.rope_width)));
                }
                log::debug!("RopeDimensions.eval | Elapsed: {:?}", t.elapsed());
                Ok(ctx)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

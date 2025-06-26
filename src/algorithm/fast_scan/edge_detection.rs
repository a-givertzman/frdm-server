use opencv::core::MatTraitConst;
use sal_core::error::Error;
use crate::{
    algorithm::{Context, ContextRead, ContextWrite, DetectingContoursCvCtx, EvalResult, InitialPoints},
    domain::{graham::dot::Dot, Eval},
    infrostructure::arena::Image,
};
use super::edge_detection_ctx::EdgeDetectionCtx;
///
/// Take [Image]
/// Return vectors of [Dot] for upper and lower edges of rope
pub struct EdgeDetection {
    ctx: Box<dyn Eval<(), Result<Context, Error>>>,
}
//
//
impl EdgeDetection {
    ///
    /// Returns [EdgeDetection] new instance
    pub fn new(ctx: impl Eval<(), Result<Context, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), Result<Context, Error>> for EdgeDetection {
    fn eval(&self, _: ()) -> EvalResult {
        let error = Error::new("EdgeDetection", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let image = ContextRead::<DetectingContoursCvCtx>::read(&ctx).result;
                let rows = image.mat.rows();
                let cols = image.mat.cols();
                let threshold = 1;
                let mut upper_edge = Vec::new();
                let mut lower_edge = Vec::new();
                for col in 0..cols {
                    for row in 0..rows {
                        match image.mat.at_2d::<u8>(row, col) {
                            Ok(&pixel_value) => {
                                if pixel_value >= threshold {
                                    upper_edge.push(Dot {x: col as usize, y: row as usize});
                                    break;
                                }
                            }   
                            Err(err) => {
                                return Err(error.pass_with("Input image format error", err.to_string()));
                            }
                        }
                    }
                    for row in (0..rows).rev() {
                        match image.mat.at_2d::<u8>(row, col) {
                            Ok(&pixel_value) => {
                                if pixel_value >= threshold {
                                    lower_edge.push(Dot {x: col as usize, y: row as usize});
                                    break;
                                }
                            }
                            Err(err) => {
                                return Err(error.pass_with("Input image format error", err.to_string()));
                            }
                        }
                    }
                }
                let result = EdgeDetectionCtx {
                    result: InitialPoints::new(upper_edge, lower_edge),
                };
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

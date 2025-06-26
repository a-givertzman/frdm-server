use opencv::core::MatTraitConst;
use sal_core::error::Error;
use crate::{domain::{eval::eval::Eval, graham::dot::Dot}, infrostructure::arena::image::Image};
use super::edge_detection_ctx::EdgeDetectionCtx;
///
/// Take [Image]
/// Return vectors of [Dot] for upper and lower edges of rope
pub struct EdgeDetection {
    ctx: Box<dyn Eval<(), Result<Image, Error>>>,
}
//
//
impl EdgeDetection{
    ///
    /// Returns [EdgeDetection] new instance
    pub fn new(ctx: impl Eval<(), Result<Image, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), Result<EdgeDetectionCtx, Error>> for EdgeDetection {
    fn eval(&mut self, _: ()) -> Result<EdgeDetectionCtx, Error> {
        let error = Error::new("EdgeDetection", "eval");
        match self.ctx.eval(()) {
            Ok(image) => {
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
                                    upper_edge.push(Dot {x: col as isize, y: row as isize});
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
                                    lower_edge.push(Dot {x: col as isize, y: row as isize});
                                    break;
                                }
                            }
                            Err(err) => {
                                return Err(error.pass_with("Input image format error", err.to_string()));
                            }
                        }
                    }
                }
                Ok(EdgeDetectionCtx {
                    upper_edge,
                    lower_edge,
                })
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

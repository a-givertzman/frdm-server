use opencv::core::MatTraitConst;

use crate::{domain::{eval::eval::Eval, graham::dot::Dot}, infrostructure::{arena::pixel_format, camera::pimage::PImage}};

use super::edge_detection_ctx::EdgeDetectionCtx;
///
/// Take [PImage]
/// Return vectors of [Dot] for upper and lower edges of rope
pub struct EdgeDetection {
    contour: PImage,
    result: Option<EdgeDetectionCtx>
}
//
//
impl EdgeDetection{
    ///
    /// Returns [EdgeDetection] new instance
    pub fn new(contour: PImage) -> Self {
        Self { 
            contour,
            result: None, 
        }
    }
}
//
//
impl Eval<(), EdgeDetectionCtx> for EdgeDetection {
    fn eval(&mut self, _: ()) -> EdgeDetectionCtx {
        let rows = self.contour.frame.rows();
        let cols = self.contour.frame.cols();
        let threshold = 1;
        let mut upper_edge = Vec::new();
        let mut lower_edge = Vec::new();
        for col in 0..cols {
            for row in 0..rows {
                match self.contour.frame.at_2d::<u8>(row, col) {
                    Ok(&pixel_value) => {
                        if pixel_value >= threshold {
                            upper_edge.push(Dot {x: col as isize, y: row as isize});
                            break;
                        }
                    }
                    Err(_) => {
                        panic!("EdgeDetection.eval | Format error");
                    }
                }
            }
            for row in (0..rows).rev() {
                match self.contour.frame.at_2d::<u8>(row, col) {
                    Ok(&pixel_value) => {
                        if pixel_value >= threshold {
                            lower_edge.push(Dot {x: col as isize, y: row as isize});
                            break;
                        }
                    }
                    Err(_) => {
                        panic!("EdgeDetection.eval | Format error");
                    }
                }
            }
        }
        EdgeDetectionCtx {
            upper_edge,
            lower_edge,
        }
    }
}
//
//
// Questions:
// what should I put in test in new(PImage, /?/)
//
// pub struct EdgeDetection{
//     contour: PImage,
//     result: Box<dyn Eval<(), EdgeDetectionCtx> + Send>,
// }
// //
// //
// impl EdgeDetection{
//     pub fn new(contour: PImage, result: impl Eval<(), EdgeDetectionCtx> + Send + 'static) -> Self {
//         Self { 
//             contour,
//             result: Box::new(result), 
//         }
//     }
// }
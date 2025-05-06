use sal_core::error::Error;
use crate::{domain::eval::eval::Eval, infrostructure::arena::image::Image};
///
/// Take [Image]
/// Return binarised [Image] with contours detected
pub struct DetectingContoursCv {
    ctx: Box<dyn Eval<(), Result<Image, Error>>>,
}
//
//
impl DetectingContoursCv{
    ///
    /// Returns [DetectingContoursCv] new instance
    pub fn new(ctx: impl Eval<(), Result<Image, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), Result<Image, Error>> for DetectingContoursCv {
    fn eval(&mut self, _: ()) -> Result<Image, Error> {
        let error = Error::new("DetectingContoursCv", "eval");
        match self.ctx.eval(()) {
            Ok(image) => {
                Ok(Image {
                    width: todo!(),
                    height: todo!(),
                    timestamp: todo!(),
                    mat: todo!(),
                    bytes: todo!(),
                })
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

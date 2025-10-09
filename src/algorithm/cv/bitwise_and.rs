use opencv::core::{Mat, MatTraitConst};
use sal_core::error::Error;
use crate::domain::Eval;
///
/// Returns Bitwise And of input frames
pub struct BitwiseAnd {
    ctx1: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
    ctx2: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl BitwiseAnd {
    ///
    /// Returns [BitwiseAnd] new instance
    #[allow(unused)]
    pub fn new(
        ctx1: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static,
        ctx2: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static
    ) -> Self {
        Self {
            ctx1: Box::new(ctx1),
            ctx2: Box::new(ctx2),
        }
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for BitwiseAnd {
    fn eval(&self, frame: Mat) -> Result<Mat, Error> {
        let error = Error::new("BitwiseAnd", "eval");
        match (self.ctx1.eval(frame.clone()), self.ctx2.eval(frame)) {
            (Ok(mat1), Ok(mat2)) => {
                let mut dst = opencv::core::Mat::default();
                opencv::core::bitwise_and(&mat1, &mat2, &mut dst, &opencv::core::no_array())
                .map_err(|err| {
                    error.pass_with(
                        format!(
                            "Can't apply Bitwise And operation to src1: {}x{} and src2: {}x{}",
                            mat1.cols(), mat1.rows(), mat2.cols(), mat2.rows(),
                        ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}

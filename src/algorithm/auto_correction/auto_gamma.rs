use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    Context, ContextWrite,
    EvalResult,
};
use crate::algorithm::auto_correction::AutoGammaCtx;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return [Image] with corrected gamma
pub struct AutoGamma {
    ctx: Box<dyn Eval<(), Result<Context, Error>>>,
}
///
/// Returns [AutoGamma] new instance
impl AutoGamma{
    pub fn new(ctx: impl Eval<(), Result<Context, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, Result<Context, Error>> for AutoGamma {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoGamma", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                // build a lookup table mapping the pixel values [0, 255] to
                // their adjusted gamma values
                let mid = 0.5f64;
                match opencv::core::mean(&frame.mat, &Mat::default()){
                    Ok(mean_result) => {
                        let mean = mean_result.into_iter().take(3).map(|v| v as f64).sum::<f64>() / 3.0;
                        let gamma: f64 = (mid * 255.0).ln()/mean.ln();
                        let inv_gamma = 1.0 / gamma;
                        let table: Vec<_> = (0..256).map(|i| (255.0 * ((i as f64 / 255.0).powf(inv_gamma))) as u8 ).collect();
                        match Mat::from_slice(&table){
                            Ok(table_mat) => {
                                let mut dst = Mat::default();
                                match opencv::core::lut(&frame.mat, &table_mat, &mut dst){
                                    Ok(_) =>{
                                        let result = AutoGammaCtx {
                                            result: Image {
                                                width: frame.width,
                                                height: frame.height,
                                                timestamp: frame.timestamp,
                                                mat: dst,
                                                bytes: frame.bytes,
                                            }
                                        };
                                        ctx.write(result)
                                    }
                                    Err(err) => Err(error.pass(err.to_string())),
                                }
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
 
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
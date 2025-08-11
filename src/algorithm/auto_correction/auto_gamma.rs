use std::time::Instant;

use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite,
    EvalResult,
};
use crate::algorithm::auto_correction::AutoGammaCtx;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return [Image] with corrected gamma
/// 
/// Reference: [Automatic contrast and brightness adjustment of a color photo of a sheet of paper with OpenCV](https://stackoverflow.com/questions/56905592/automatic-contrast-and-brightness-adjustment-of-a-color-photo-of-a-sheet-of-pape)
pub struct AutoGamma {
    factor: f64,
    ctx: Box<dyn Eval<(), EvalResult>>,
}
impl AutoGamma {
    ///
    /// Returns [AutoGamma] new instance
    /// - `factor` - percent of influence of [AutoGamma] algorythm
    ///     bigger the value more the effect of [AutoGamma] algorythm
    ///     - exposure 35: beatter percent - 60 %
    ///     - exposure 95: beatter percent - 95 %
    pub fn new(factor: f64, ctx: impl Eval<(), EvalResult>+ 'static) -> Self {
        Self { 
            factor: factor,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for AutoGamma {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoGamma", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                // build a lookup table mapping the pixel values [0, 255] to
                // their adjusted gamma values
                let t = Instant::now();
                let factor = self.factor / 100.0;
                let mid = 0.5f64;
                match opencv::core::mean(&frame.mat, &Mat::default()){
                    Ok(mean_result) => {
                        let mean = mean_result.into_iter().take(3).map(|v| v as f64).sum::<f64>() / 3.0;
                        let gamma: f64 = (mid * 255.0).ln()/mean.ln();
                        let inv_gamma = 1.0 / (factor * gamma);
                        let table: Vec<_> = (0..256).map(|i| (255.0 * ((i as f64 / 255.0).powf(inv_gamma))) as u8 ).collect();
                        //log::debug!("AutoGamma.eval | Lut table: {:?}", table);
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
                                        log::debug!("AutoGamma.eval | Elapsed: {:?}", t.elapsed());
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
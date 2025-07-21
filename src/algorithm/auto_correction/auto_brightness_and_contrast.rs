use opencv::core::{Mat, MatTraitConst, Vector};
use opencv::imgproc;
use sal_core::error::Error;
use crate::algorithm::{
    Context, ContextWrite,
    EvalResult,
};
use crate::algorithm::auto_correction::AutoBrightnessAndContrastCtx;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return [Image] with corrected brightness and contrast
pub struct AutoBrightnessAndContrast {
    conf: Option<f32>,
    ctx: Box<dyn Eval<(), Result<Context, Error>>>,
}
///
/// Returns [AutoBrightnessAndContrast] new instance
impl AutoBrightnessAndContrast{
    pub fn new(conf: Option<f32>, ctx: impl Eval<(), Result<Context, Error>> + 'static) -> Self {
        Self { 
            conf,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, Result<Context, Error>> for AutoBrightnessAndContrast {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoGamma", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let mut clip_hist_percent = match self.conf {
                    Some(val) => val,
                    None => 1.0,
                };
                let mut gray = Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        // Grayscale histogram
                        let mut hist = Mat::default();
                        let hist_size = 256 as i32;
                        let imgs: Vector<Mat> = Vector::from_iter([gray.clone()]);
                        match opencv::imgproc::calc_hist(
                            &imgs,
                            &Vector::from_slice(&[0]),
                            &Mat::default(),
                            &mut hist,
                            &Vector::from_slice(&[hist_size]),
                            &Vector::from_slice(&[0.0 ,256.0]),
                            false,
                        ) {
                            Ok(_) => {
                                // Calculate cumulative distribution from the histogram
                                let mut accumulator = vec![];
                                match hist.at::<f32>(0) {
                                    Ok(val) => accumulator.push(*val),
                                    Err(err) => return Err(error.pass(err.to_string())),
                                }
                                for index in 1..(hist_size as usize) {
                                    match hist.at::<f32>(index as i32) {
                                        Ok(val) => accumulator.push(accumulator[index -1] + * val),
                                        Err(err) => return Err(error.pass(err.to_string())),
                                    }
                                }
                                // Locate points to clip
                                let maximum = match accumulator.last() {
                                    Some(max) => max,
                                    None => return Err(error.pass("Empty accumulator"))
                                };
                                clip_hist_percent = clip_hist_percent * (maximum / 100.0);
                                clip_hist_percent = clip_hist_percent / 2.0;
                                // Locate left cut
                                let mut minimum_gray = 0;
                                while accumulator[minimum_gray] < clip_hist_percent {
                                    minimum_gray += 1;
                                }
                                // Locate right cut
                                let mut maximum_gray = (hist_size - 1) as usize;
                                while accumulator[maximum_gray] >= (maximum - clip_hist_percent) {
                                    maximum_gray -= 1;
                                }
                                // Calculate alpha and beta values
                                let alpha = 255.0 / ((maximum_gray - minimum_gray) as f64);
                                let beta = - (minimum_gray as f64) * alpha;
                                let mut dst = Mat::default();
                                match opencv::core::convert_scale_abs(&frame.mat, &mut dst, alpha, beta) {
                                    Ok(_) => {
                                        let result = AutoBrightnessAndContrastCtx {
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
                            },
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
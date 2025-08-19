use std::time::Instant;
use opencv::core::{Mat, MatTraitConst, Vector};
use opencv::imgproc;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite, ContextRead,
    AutoGammaCtx,
    EvalResult,
};
use crate::algorithm::auto_correction::AutoBrightnessAndContrastCtx;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return [Image] with corrected brightness and contrast
/// 
/// 
/// Reference: [Automatic contrast and brightness adjustment of a color photo of a sheet of paper with OpenCV](https://stackoverflow.com/questions/56905592/automatic-contrast-and-brightness-adjustment-of-a-color-photo-of-a-sheet-of-pape)
pub struct AutoBrightnessAndContrast {
    histogram_clipping: i32,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
impl AutoBrightnessAndContrast {
    ///
    /// Returns [AutoBrightnessAndContrast] new instance
    /// - `histogram_clipping` - optional histogram clipping, default = 0 %
    pub fn new(histogram_clipping: i32, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self { 
            histogram_clipping,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for AutoBrightnessAndContrast {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoBrightnessAndContrast", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let frame = ContextRead::<AutoGammaCtx>::read(&ctx).result.clone();
                let mut clip_hist_percent = self.histogram_clipping as f32;
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
                                let mut accumulator = vec![255.0; 256];
                                match hist.at::<f32>(0) {
                                    Ok(val) => accumulator.push(*val),
                                    Err(err) => return Err(error.pass(err.to_string())),
                                }
                                for index in 1..(hist_size as usize) {
                                    match hist.at::<f32>(index as i32) {
                                        Ok(val) => {
                                            if let Some(acc_val) = accumulator.get(index -1) {
                                                accumulator.push(acc_val + * val)
                                            }
                                        }
                                        Err(err) => return Err(error.pass(err.to_string())),
                                    }
                                }
                                // Locate points to clip
                                let maximum = match accumulator.last() {
                                    Some(max) => max,
                                    None => return Err(error.pass("Empty `accumulator`"))
                                };
                                clip_hist_percent = clip_hist_percent * maximum / 100.0;
                                clip_hist_percent = clip_hist_percent / 2.0;
                                // Locate left cut
                                let mut minimum_gray = 0;
                                // accumulator[minimum_gray] < clip_hist_percent
                                while accumulator.get(minimum_gray).map_or(false, |acc_val| *acc_val < clip_hist_percent) {
                                    minimum_gray += 1;
                                }
                                // Locate right cut
                                let mut maximum_gray = (hist_size - 1) as usize;
                                // accumulator[maximum_gray] >= (maximum - clip_hist_percent)
                                while accumulator.get(maximum_gray).map_or(true, |acc_val| *acc_val >= (maximum - clip_hist_percent)) {
                                    maximum_gray -= 1;
                                }
                                // Calculate alpha and beta values
                                let alpha = 255.0 / ((maximum_gray - minimum_gray) as f64);
                                let beta = - (minimum_gray as f64) * alpha;
                                let mut dst = Mat::default();
                                match opencv::core::convert_scale_abs(&frame.mat, &mut dst, alpha * 1.99, beta) {
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
                                        log::debug!("AutoBrightnessAndContrast.eval | Elapsed: {:?}", t.elapsed());
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
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
    clip_left: i32,
    clip_right: i32,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
impl AutoBrightnessAndContrast {
    ///
    /// Returns [AutoBrightnessAndContrast] new instance
    /// - `clip_left` - optional histogram clipping from left (dark pixels), default = 0 %
    /// - `clip_right` - optional histogram clipping from right (light pixels), default = 100 %
    pub fn new(clip_left: i32, clip_right: i32, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self { 
            clip_left,
            clip_right,
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
                                        Ok(val) => {
                                            if let Some(acc_val) = accumulator.get(index -1) {
                                                accumulator.push(acc_val + * val)
                                            }
                                        }
                                        Err(err) => return Err(error.pass(err.to_string())),
                                    }
                                }
                                // log::debug!("AutoBrightnessAndContrast.eval | accumulator: {:?}", accumulator);
                                // Locate points to clip
                                let maximum = match accumulator.last() {
                                    Some(max) => max,
                                    None => return Err(error.pass("Empty `accumulator`"))
                                };
                                log::debug!("AutoBrightnessAndContrast.eval | maximum: {:?}", maximum);
                                let clip_hist_left = (self.clip_left as f32) * maximum / 100.0;
                                // let clip_hist_left = clip_hist_percent / 2.0;
                                let clip_hist_right = (self.clip_right as f32) * maximum / 100.0;
                                // clip_hist_right = clip_hist_percent / 2.0;
                                // Locate left cut
                                let mut minimum_gray = 0;
                                for i in 0..accumulator.len() {
                                    minimum_gray = i;
                                    if !(accumulator[i] < clip_hist_left) {
                                        break;
                                    }
                                }
                                // Locate right cut
                                let mut maximum_gray = (hist_size - 1) as usize;
                                for i in (0..accumulator.len()).rev() {
                                    maximum_gray = i;
                                    if !(accumulator[i] >= (maximum - clip_hist_right)) {
                                        break;
                                    }
                                }
                                log::debug!("AutoBrightnessAndContrast.eval | minimum_gray: {:?}", minimum_gray);
                                log::debug!("AutoBrightnessAndContrast.eval | maximum_gray: {:?}", maximum_gray);
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
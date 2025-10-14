use std::time::Instant;
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual};
use sal_core::error::Error;
use crate::{
    algorithm::{cv, ContextRead, ContextWrite, EvalResult, FilterIsChanged, ResultCtx, TemporalFilterCtx}, conf::GaussianConf, domain::{Eval, Filter, Image, RwLock}
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter {
    threshold: f64,
    filters: RwLock<Vec<FilterIsChanged::<f32>>>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `open_kernel` - Morphology open operation kernel size
    /// - `erode_kernel` - Morphology erode operation kernel size
    /// - `threshold` - used to detect movement by comparing with the delta between same pixel of each frame
    pub fn new(gaussian: GaussianConf, open_kernel: [i32; 2], erode_kernel: [i32; 2], threshold: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            threshold,
            filters: RwLock::new(vec![]),
            proc: Box::new(
                cv::Morphology::erode(
                    &erode_kernel,
                    cv::Morphology::open(
                        &open_kernel,
                        cv::GaussianBlur::new(
                            &gaussian.kernel,
                            PassCvMat::new(),
                        )
                    ),
                ),
            ),
            ctx: Box::new(ctx),
            debug,
        }
    }
}
//
//
impl Eval<Image, EvalResult> for TemporalFilter {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("TemporalFilter", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ContextRead::<>::read(&ctx);
                let frame = &result.val;
                match frame.mat.data_bytes() {
                    Ok(input) => {
                        let height = frame.mat.rows() as usize;
                        let width = frame.mat.cols() as usize;
                        let pixels = width * height * frame.mat.channels() as usize;
                        let mut dst = vec![0u8; pixels];
                        log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                        if self.filters.read().is_empty() {
                            *self.filters.write() = (0..pixels).map(|_| {
                                FilterIsChanged::<f32>::new(None, self.threshold)
                            }).collect();
                        }
                        log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                        let mut filters = self.filters.write();
                        for i in 0..pixels {
                            match input.get(i) {
                                Some(value) => {
                                    if let Some(filter) = filters.get_mut(i) {
                                        match dst.get_mut(i) {
                                            Some(pixel) => *pixel = match filter.add(*value as f32) {
                                                Some(_) => 255,
                                                None => 0,
                                            },
                                            None => return Err(error.err(format!("Out image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                                        }
                                    }
                                }
                                None => return Err(error.err(format!("Input image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                            }
                        }
                        // if self.proc.read().is_none() {
                        //     *self.proc.write() = Some(Box::new(
                        //         cv::Morphology::erode(
                        //             &self.erode_kernel,
                        //             cv::Morphology::open(
                        //                 &self.open_kernel,
                        //                 cv::GaussianBlur::new(
                        //                     &self.gaussian.kernel,
                        //                     PassCvMat::new(),
                        //                 )
                        //             ),
                        //         ),
                        //     ));
                        // }
                        log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        let dst = cv::CreateMat::gray8(width as i32, height as i32)
                            .filled()
                            .eval(&dst)
                            .map_err(|err| error.pass(err))?;
                        let dst = self.proc.eval(dst)
                            .map_err(|err| error.pass(err))?;
                        let frame = Image::with(dst);
                        let ctx = if self.debug {
                            let result = TemporalFilterCtx { frame: frame.clone() };
                            ctx.write(result).map_err(|err| error.pass(err))?
                        } else {
                            ctx
                        };
                        let result = ResultCtx { val: frame };
                        log::debug!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// Closes calculation sequence, passing input [Mat]
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}

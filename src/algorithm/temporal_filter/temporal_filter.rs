use std::{any::TypeId, marker::PhantomData, time::Instant};
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual};
use sal_core::error::Error;
use crate::{
    algorithm::{cv, ContextRead, ContextWrite, EvalResult, FilterIsChanged, ResultCtx, TemporalFilterCtx, FastScanCtx, FineScanCtx},
    conf::GaussianConf, domain::{Eval, Filter, Image, RwLock}
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter<Branch> {
    threshold: f64,
    filters: RwLock<Vec<FilterIsChanged::<f32>>>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
    branch: PhantomData<Branch>,
}
//
//
impl<Branch> TemporalFilter<Branch> {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `open_kernel` - Morphology open operation kernel size
    /// - `erode_kernel` - Morphology erode operation kernel size
    /// - `threshold` - used to detect movement by comparing with the delta between same pixel of each frame
    /// - `Branch` - the calculation branch [FastScanCtx] or [FineScanCtx]
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
            branch: PhantomData,
        }
    }
}
//
//
impl<Branch: 'static> Eval<Image, EvalResult> for TemporalFilter<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("TemporalFilter", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                match frame.mat.data_bytes() {
                    Ok(input) => {
                        let height = frame.mat.rows() as usize;
                        let width = frame.mat.cols() as usize;
                        let pixels = width * height * frame.mat.channels() as usize;
                        let mut dst = vec![0u8; pixels];
                        // log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                        if self.filters.read().is_empty() {
                            *self.filters.write() = (0..pixels).map(|_| {
                                FilterIsChanged::<f32>::new(None, self.threshold)
                            }).collect();
                        }
                        // log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        // log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                        let mut filters = self.filters.write();
                        // Гарантируем компилятору равенство длин, чтобы убрать проверки границ
                        if input.len() < pixels || filters.len() < pixels || dst.len() < pixels {
                            return Err(error.err("Image size mismatch").into());
                        }
                        // Новый быстрый вариант перебора
                        filters.iter_mut()
                            .zip(input)
                            .zip(dst.iter_mut())
                            // .take(pixels) // Можно удалить так как проверили длины массивов
                            .for_each(|((filter, value), pixel)| *pixel = match filter.add(*value as f32) {
                                    Some(_) => 255,
                                    None => 0,
                            });
                        // Старый медленный вариант перебора
                        // for i in 0..pixels {
                        //     match input.get(i) {
                        //         Some(value) => {
                        //             if let Some(filter) = filters.get_mut(i) {
                        //                 match dst.get_mut(i) {
                        //                     Some(pixel) => *pixel = match filter.add(*value as f32) {
                        //                         Some(_) => 255,
                        //                         None => 0,
                        //                     },
                        //                     None => Err(error.err(format!("Out image format error, index [{i}] out of image range {width}x{height}={pixels}")))?,
                        //                 }
                        //             }
                        //         }
                        //         None => Err(error.err(format!("Input image format error, index [{i}] out of image range {width}x{height}={pixels}")))?,
                        //     }
                        // }
                        // log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        let dst = cv::CreateMat::gray8(width as i32, height as i32)
                            .eval(dst)
                            .map_err(|err| error.pass(err))?;
                        let dst = self.proc.eval(dst)
                            .map_err(|err| error.pass(err))?;
                        let frame = Image::from(dst, meta);
                        let ctx = if self.debug {
                            match TypeId::of::<Branch>() {
                                typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(TemporalFilterCtx::<FastScanCtx>::new(frame.clone()))
                                    .map_err(|err| error.pass(err))?,
                                typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(TemporalFilterCtx::<FineScanCtx>::new(frame.clone()))
                                    .map_err(|err| error.pass(err))?,
                                _ => {
                                    log::warn!("TemporalFilter.eval | Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>());
                                    ctx
                                }
                            }
                        } else {
                            ctx
                        };
                        let result = ResultCtx { val: frame };
                        log::trace!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
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

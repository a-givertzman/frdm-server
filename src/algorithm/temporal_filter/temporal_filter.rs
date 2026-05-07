use std::{any::TypeId, marker::PhantomData, time::Instant};
use opencv::{core::{self, Mat, MatTraitConst}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{cv, ContextRead, ContextWrite, EvalResult, ResultCtx, TemporalFilterCtx, FastScanCtx, FineScanCtx},
    conf::GaussianConf, domain::{Eval, Image, RwLock}
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter<Branch> {
    threshold: f64,
    // RwLock для мутации и передачи класса между потокоами, но работает он в синхронном режиме
    prev: RwLock<Option<Mat>>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
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
            prev: RwLock::new(None),
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
                let mut prev_guard = self.prev.write();
                let mut dst = Mat::default();
                if let Some(prev) = prev_guard.as_ref() {
                    let mut diff = Mat::default();
                    // Находим разницу между кадрами
                    core::absdiff(prev, &frame.mat, &mut diff)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                    // Применяем порог: всё что больше threshold становится 255, остальное 0
                    imgproc::threshold(&diff, &mut dst, self.threshold, 255.0, imgproc::THRESH_BINARY)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                } else {
                    // Первый кадр: дельты нет, возвращаем черную матрицу нужного размера
                    dst = unsafe { Mat::new_rows_cols(frame.mat.rows(), frame.mat.cols(), core::CV_8UC1) }
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                    // Опционально можно залить нулями: dst.set_to(&core::Scalar::all(0.0), &Mat::default())...
                }
                // Сохраняем текущий кадр как фон для следующего цикла
                *prev_guard = Some(frame.mat.clone());
                // Отдаем результат в морфологию
                let dst = self.proc.eval(dst).map_err(|err| error.pass(err))?;
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

use std::time::Instant;
use opencv::core::Mat;
use opencv::core::Size2i;
use opencv::imgproc;
use opencv::core;
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead,
    CvContoursCtx,
    GrayCtx, EvalResult, ResultCtx,
};
use crate::conf::DetectingContoursConf;
use crate::{Eval, domain::Image};
///
/// Return filtered and binarised [Image] with contours detected
/// 
/// Binarization is based on the sharpness of the target segment
pub struct CvContours {
    conf: DetectingContoursConf,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl CvContours {
    ///
    /// Returns [CvContours] new instance
    /// - `ctx` - Prevouse step returns [Image] in [Context]
    /// - `conf` - Configuration for `Contour dectection` algorithm:
    ///     - gausian:
    ///         - `kernel` - Gausian blur kernel size
    ///         - `sigma_x` - Standard deviation in X direction
    ///         - `sigma_y` - Standard deviation in Y direction
    ///     - sobel:
    ///         - `kernel_size` - Sobel kernel size
    ///         - `scale` - Scale factor for computed derivative values
    ///         - `delta` - Delta values added to results
    ///     - overlay:
    ///         - `src1-weight` - Weight for X gradient
    ///         - `src1-weight` - Weight for Y gradient
    ///         - `gamma` - Scalar added to weighted sum
    pub fn new(conf: DetectingContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self { 
            conf,
            ctx: Box::new(ctx),
        }
    }
    /// 
    /// Returns Structure element of specified size
    fn structure_element(w: i32, h: i32) -> Result<Mat, Error> {
        opencv::imgproc::get_structuring_element(opencv::imgproc::MORPH_ELLIPSE, core::Size2i::new(5, 5), core::Point2i::new(-1, -1))
            .map_err(|err| Error::new("CvContours", "structure_element").pass_with("Can't Get Structuring Element", err.to_string()))
    }
    /// 
    /// Returns Brured image
    fn gaussian_blur(img: &Mat, kernel: i32) -> Result<Mat, Error> {
        let mut blur = Mat::default();
        imgproc::gaussian_blur(&img, &mut blur, Size2i::new(kernel, kernel), 0.0, 0.0, opencv::core::BORDER_DEFAULT)
            .map_err(|err| Error::new("CvContours", "gausian_blur").pass_with("Can't do Gausian Blur", err.to_string()))?;
        Ok(blur)
    }
    /// 
    /// Returns Morphology transformed image
    fn morphology(img: &Mat, op: i32, kernel: i32) -> Result<Mat, Error> {
        let error = Error::new("CvContours", "morphology");
        let mut transformed = core::Mat::default();
        opencv::imgproc::morphology_ex(
            &img,
            &mut transformed,
            op,
            &Self::structure_element(kernel, kernel).map_err(|err| error.pass_with("Can't do Morphology transform Open", err.to_string()))?,
            core::Point2i::new(-1, -1),
            2,
            opencv::core::BORDER_CONSTANT,
            opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass_with("Can't do Morphology transform Open", err.to_string()))?,
        ).map_err(|err| error.pass_with("Can't do Morphology transform Open", err.to_string()))?;
        Ok(transformed)
    }
}
//
//
impl Eval<Image, EvalResult> for CvContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("CvContours", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                // let result: &ResultCtx = ctx.read();
                let result: &GrayCtx = ctx.read();
                let frame = &result.frame;
                let kernel = 13;
                let ev = cv::GaussianBlur::new(
                    &[kernel, kernel],
                    cv::Morphology::new(
                        operation,
                        kernel,
                        ,
                    ),
                );
                cv::Morphology::dilate(
                    kernel,
                ),
                cv::GaussianBlur::new(
                    &[kernel, kernel],
                    cv::Laplacian::new(
                        kernel,
                        cv::GaussianBlur::new(
                            &[kernel, kernel],
                            PassMat::new(),
                        ),
                    ),
                ),
                let blur = Self::gaussian_blur(&frame.mat, kernel).map_err(|err| error.pass(err))?;
                let mut laplacian = Mat::default();
                opencv::imgproc::laplacian(&blur, &mut laplacian, opencv::core::CV_8UC1, 5, 1.0, 0.0, opencv::core::BorderTypes::BORDER_REFLECT_101 as i32)
                    .map_err(|err| error.pass_with("Can't do Laplacian", err.to_string()))?;
                let blur = Self::gaussian_blur(&laplacian, kernel).map_err(|err| error.pass(err))?;
                let mut contour = Mat::default();
                let threshold = opencv::imgproc::threshold(&blur, &mut contour, 0.0, 255.0, opencv::imgproc::ThresholdTypes::THRESH_OTSU as i32)
                    .map_err(|err| error.pass_with("Can't do Threshold", err.to_string()))?;
                opencv::imgproc::threshold(&blur, &mut contour, threshold * 0.4, 255.0, opencv::imgproc::ThresholdTypes::THRESH_BINARY as i32)
                    .map_err(|err| error.pass_with("Can't do Threshold", err.to_string()))?;
                let blur = Self::gaussian_blur(&contour, 5).map_err(|err| error.pass(err))?;
                let open = Self::morphology(&blur, opencv::imgproc::MORPH_OPEN, 5).map_err(|err| error.pass(err))?;
                let blur = Self::gaussian_blur(&open, 15).map_err(|err| error.pass(err))?;
                let delete = Self::morphology(&blur, opencv::imgproc::MORPH_DILATE, 7).map_err(|err| error.pass(err))?;
                let frame = Image {
                    width: frame.width,
                    height: frame.height,
                    timestamp: frame.timestamp,
                    mat: delete,
                    bytes: frame.bytes,
                };
                let result = CvContoursCtx { result: frame.clone() };
                let ctx = ctx.write(result)?;
                let result = ResultCtx { frame };
                log::debug!("CvContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// Closes calculation sequence, passing input `Mat`
struct PassMat {}
impl PassMat {
    fn new() -> Self {
        Self {  }
    }
}
impl<'a> Eval<&'a Mat, Result<Mat, Error>> for PassMat {
    fn eval(&self, mat: &'a Mat) -> Result<Mat, Error> {
        Ok(mat.to_owned())
    }
}
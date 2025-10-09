use opencv::{
    core::{BorderTypes, Mat, Size2i}, imgproc,
};
use sal_core::error::Error;
use crate::Eval;
///
/// Apply `OpenCv` Gaussian Blur to passed image
pub struct GaussianBlur {
    kernel: Vec<i32>,
    sigma: Vec<f64>,
    border: BorderTypes,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl GaussianBlur {
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `kernel` - Gaussian kernel size, [w, h].
    ///    ksize.width and ksize.height can differ but they both must be positive and odd.
    ///    Or, they can be zero's and then they are computed from sigma.
    #[allow(unused)]
    pub fn new(kernel: &[i32; 2], ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static) -> Self {
        Self { 
            kernel: kernel.into(),
            sigma: vec![0.0, 0.0],
            border: BorderTypes::BORDER_REFLECT_101,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `sigma` - Gaussian kernel standard deviation. [x, y]
    ///    If sigma Y is zero, it is set to be equal to sigma X,
    ///    if both sigmas are zeros, they are computed from ksize.width and ksize.height,
    ///    respectively (see get_gaussian_kernel for details);
    ///    To fully control the result regardless of possible future modifications of all this semantics,
    ///    it is recommended to specify all of ksize, sigmaX, and sigmaY.
    #[allow(unused)]
    pub fn with_sigma(mut self, xy: &[f64; 2]) -> Self {
        self.sigma = xy.into();
        self
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `border` - Pixel extrapolation method, see #BorderTypes. BORDER_WRAP is not supported. Default `BORDER_REFLECT_101`
    #[allow(unused)]
    pub fn with_border(mut self, border: BorderTypes) -> Self {
        self.border = border;
        self
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for GaussianBlur {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                imgproc::gaussian_blur(
                    &mat,
                    &mut dst,
                    Size2i::new(self.kernel[0], self.kernel[1]),
                    self.sigma[0],
                    self.sigma[1],
                    self.border as i32,
                )
                .map_err(|err| {
                    Error::new("GaussianBlur", "eval").pass_with(
                        format!("Can't apply Gausian Blur, kernel {:?}, sigma {:?}, border {:?}", self.kernel, self.sigma, self.border),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(Error::new("GaussianBlur", "eval").pass(err)),
        }
    }
}
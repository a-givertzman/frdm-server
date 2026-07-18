use opencv::core::{BorderTypes, Mat};
use sal_core::error::Error;
use crate::Eval;
///
/// Apply `OpenCv` Laplacian to passed image
pub struct Laplacian {
    out_depth: i32,
    kernel: i32,
    scale: f64,
    delta: f64,
    border: BorderTypes,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl Laplacian {
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `kernel` - Aperture size `[w, h]` used to compute the second-derivative filters. See get_deriv_kernels for details. The size must be positive and odd.
    /// - `scale` - default `1.0`
    /// - `delta` - default `0.0`
    /// - `border` - default `BORDER_REFLECT_101`
    #[allow(unused)]
    pub fn new(kernel: i32, ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static) -> Self {
        Self {
            out_depth: opencv::core::CV_8UC1,
            kernel,
            scale: 1.0,
            delta: 0.0,
            border: BorderTypes::BORDER_REFLECT_101,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `depth` - Desired depth of the destination image. Default `CV_8UC1`
    #[allow(unused)]
    pub fn with_depth(mut self, depth: i32) -> Self {
        self.out_depth = depth;
        self
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `scale` - Optional scale factor for the computed Laplacian values.
    ///    By default, no scaling is applied.
    /// 
    /// See [get_deriv_kernels](https://docs.rs/opencv/0.96.0/opencv/imgproc/fn.get_deriv_kernels.html) for details.
    #[allow(unused)]
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `delta` - Optional delta value that is added to the results prior to storing them in dst .
    #[allow(unused)]
    pub fn with_delta(mut self, delta: f64) -> Self {
        self.delta = delta;
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
    ///
    /// Returns image color depth name
    fn depth_name(depth: i32) -> String {
        match depth {
            opencv::core::CV_16FC1 => String::from("CV_16FC1"),
            opencv::core::CV_16FC2 => String::from("CV_16FC2"),
            opencv::core::CV_16FC3 => String::from("CV_16FC3"),
            opencv::core::CV_16FC4 => String::from("CV_16FC4"),
            opencv::core::CV_16SC1 => String::from("CV_16SC1"),
            opencv::core::CV_16SC2 => String::from("CV_16SC2"),
            opencv::core::CV_16SC3 => String::from("CV_16SC3"),
            opencv::core::CV_16SC4 => String::from("CV_16SC4"),
            opencv::core::CV_16UC1 => String::from("CV_16UC1"),
            opencv::core::CV_16UC2 => String::from("CV_16UC2"),
            opencv::core::CV_16UC3 => String::from("CV_16UC3"),
            opencv::core::CV_16UC4 => String::from("CV_16UC4"),
            opencv::core::CV_32FC1 => String::from("CV_32FC1"),
            opencv::core::CV_32FC2 => String::from("CV_32FC2"),
            opencv::core::CV_32FC3 => String::from("CV_32FC3"),
            opencv::core::CV_32FC4 => String::from("CV_32FC4"),
            opencv::core::CV_32SC1 => String::from("CV_32SC1"),
            opencv::core::CV_32SC2 => String::from("CV_32SC2"),
            opencv::core::CV_32SC3 => String::from("CV_32SC3"),
            opencv::core::CV_32SC4 => String::from("CV_32SC4"),
            opencv::core::CV_64FC1 => String::from("CV_64FC1"),
            opencv::core::CV_64FC2 => String::from("CV_64FC2"),
            opencv::core::CV_64FC3 => String::from("CV_64FC3"),
            opencv::core::CV_64FC4 => String::from("CV_64FC4"),
            opencv::core::CV_8SC1 => String::from("CV_8SC1"),
            opencv::core::CV_8SC2 => String::from("CV_8SC2"),
            opencv::core::CV_8SC3 => String::from("CV_8SC3"),
            opencv::core::CV_8SC4 => String::from("CV_8SC4"),
            opencv::core::CV_8UC1 => String::from("CV_8UC1"),
            opencv::core::CV_8UC2 => String::from("CV_8UC2"),
            opencv::core::CV_8UC3 => String::from("CV_8UC3"),
            opencv::core::CV_8UC4 => String::from("CV_8UC4"),
            _ => format!("{depth} - Unknown color depth"),
        }
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for Laplacian {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::laplacian(
                    &mat,
                    &mut dst,
                    self.out_depth,
                    self.kernel,
                    self.scale,
                    self.delta,
                    self.border as i32,
                )
                .map_err(|err| {
                    Error::new("Laplacian", "eval")
                        .pass_with(
                            format!(
                                "Can't apply Laplacian with depth {}, kernel {:?}, scale {:?}, delta {:?}, border {:?}",
                                Self::depth_name(self.out_depth), self.kernel, self.scale, self.delta, self.border),
                            err.to_string(),
                        )
                })?;
                Ok(dst)
            }
            Err(err) => Err(Error::new("Laplacian", "eval").pass(err)),
        }
    }
}

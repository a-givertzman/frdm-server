use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Gaussian filter`
/// 
/// ### Example:
/// ```yaml
/// gausian:
///     kernel-size:
///         width: 3
///         heidht: 3
///     sigma-x: 0.0
///     sigma-y: 0.0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GausianConf {
    /// Gausian blur kernel size
    /// This must be odd values (the matrix must have an exact center)
    /// The larger the kernel size, the greater the blur.
    /// 
    /// Default: Size( width: 3, heidht: 3)
    pub kernel_w: i32,
    pub kernel_h: i32,
    /// Standard deviation in X direction
    /// The higher the value, the more pixels are used to count each pixel and the smoother blur will be
    /// If the value is 0.0, it is calculated based on kernel_size
    /// sigma = 0.3 * (( kernel_size - 1 ) * 0.5 - 1 ) + 0.8
    /// 
    /// Default: 0.0 
    pub sigma_x: f64,
    /// Standard deviation in Y direction
    /// Same as in X direction
    /// 
    /// Default: 0.0
    pub sigma_y: f64,
}
//
// 
impl GausianConf {
    ///
    /// Returns [GausianConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "GausianConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let (kernel_w, kernel_h): (i64, i64) = match conf.get("kernel-size") {
            Some(kernel) => {
                let kernel: ConfTree = kernel;
                (kernel.get("width").unwrap(), kernel.get("height").unwrap())
            }
            None => (3, 3),
        };
        log::debug!("{dbg}.new | kernel-size: Size({}, {})", kernel_w, kernel_h);
        let sigma_x = conf.get("sigma-x").unwrap_or(0.0);
        log::debug!("{dbg}.new | sigma_x: {:?}", sigma_x);
        let sigma_y = conf.get("sigma-y").unwrap_or(0.0);
        log::debug!("{dbg}.new | sigma_y: {:?}", sigma_y);
        Self {
            kernel_w: kernel_w as i32,
            kernel_h: kernel_h as i32,
            sigma_x,
            sigma_y,
        }
    }
}
//
//
impl Default for GausianConf {
    fn default() -> Self {
        Self { kernel_w: 3, kernel_h: 3, sigma_x: 0.0, sigma_y: 0.0 }
    }
}

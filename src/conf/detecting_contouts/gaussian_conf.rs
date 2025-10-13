use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};

///
/// ## Configuration for `Gaussian filter`
/// 
/// ### Example:
/// ```yaml
/// gausian:
///     kernel: [3, 3]             # blur radius
///     sigma: [0.0, 0.0]
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaussianConf {
    /// Gausian blur kernel size
    /// This must be odd values (the matrix must have an exact center)
    /// The larger the kernel size, the greater the blur.
    /// 
    /// Default: Size( width: 3, height: 3)
    pub kernel: [i32; 2],
    /// Standard deviation in [X, Y] direction
    /// The higher the value, the more pixels are used to count each pixel and the smoother blur will be
    /// If the value is 0.0, it is calculated based on kernel_size
    /// sigma = 0.3 * (( kernel_size - 1 ) * 0.5 - 1 ) + 0.8
    /// 
    /// Default: [0.0, 0.0]
    pub sigma: [f64; 2],
}
//
// 
impl GaussianConf {
    ///
    /// Returns [GausianConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "GaussianConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let kernel: Vec<i32> = conf.as_vec("kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | kernel: {:?}", kernel);
        let sigma: Vec<f64> = conf.as_vec("sigma").map(|val| {
            val.into_iter().map(|v| v.as_f64().expect(&format!("{dbg}.new | 'sigma' - wrong configuration")))
        }).expect(&format!("{dbg}.new | 'sigma' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | sigma: {:?}", sigma);
        Self {
            kernel: kernel.try_into().expect(&format!("{dbg}.new | 'kernel' - wrong configuration")),
            sigma: sigma.try_into().expect(&format!("{dbg}.new | 'sigma' - wrong configuration")),
        }
    }
}
//
//
impl Default for GaussianConf {
    fn default() -> Self {
        Self { kernel: [3, 3], sigma: [0.0, 0.0] }
    }
}

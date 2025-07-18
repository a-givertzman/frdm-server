use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Sobel` operator
/// 
/// ### Example:
/// ```yaml
/// sobel:
///     kernel-size: 3
///     scale: 1.0
///     delta: 0.0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SobelConf {
    /// Sobel kernel size
    /// 
    /// Default: 3
    pub kernel_size: i32,
    /// Scale factor for computed derivative values
    /// 
    /// Default: 1.0
    pub scale: f64,
    /// Delta values added to results
    /// 
    /// Default: 0.0
    pub delta: f64,
}
//
// 
impl SobelConf {
    ///
    /// Returns [SobelConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "SobelConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        let kernel_size: i64 = conf.get("kernel-size").unwrap_or(3);
        log::debug!("{dbg}.new | kernel-size: {:?}", kernel_size);
        let scale = conf.get("scale").unwrap_or(1.0);
        log::debug!("{dbg}.new | scale: {:?}", scale);
        let delta = conf.get("delta").unwrap_or(0.0);
        log::debug!("{dbg}.new | delta: {:?}", delta);
        Self {
            kernel_size: kernel_size as i32,
            scale,
            delta,
        }
    }
}
//
//
impl Default for SobelConf {
    fn default() -> Self {
        Self { kernel_size: 3, scale: 1.0, delta: 0.0 }
    }
}

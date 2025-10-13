use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Cropping` operator
/// 
/// ### Example:
/// ```yaml
/// temporal-filter:
///     open-kernel: [5, 5]     # Morphology open operation kernel size [w, h], default [5, 5]
///     erode-kernel: [5, 5]    # Morphology erode operation kernel size [w, h], default [5, 5]
///     threshold: 1.0          # Threshold to detect the pixel whas changed or not in the each next frame, default 1
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalFilterConf {
    /// Morphology open operation kernel size
    pub open_kernel: [i32; 2],
    /// Morphology erode operation kernel size
    pub erode_kernel: [i32; 2],
    /// Threshold to detect the pixel whas changed or not in the each next frame
    pub threshold: f64,
}
//
// 
impl TemporalFilterConf {
    ///
    /// Returns [TemporalFilterConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "TemporalFilterConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let open_kernel: Vec<i32> = conf.as_vec("open-kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'open-kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'open-kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | open-kernel: {:?}", open_kernel);
        let erode_kernel: Vec<i32> = conf.as_vec("erode-kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'erode-kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'erode-kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | erode-kernel: {:?}", erode_kernel);
        let threshold = conf.get("threshold").unwrap_or(1.0);
        log::trace!("{dbg}.new | threshold: {:?}", threshold);
        Self {
            open_kernel: open_kernel.try_into().expect(&format!("{dbg}.new | 'open-kernel' - wrong configuration")),
            erode_kernel: erode_kernel.try_into().expect(&format!("{dbg}.new | 'erode-kernel' - wrong configuration")),
            threshold,
        }
    }
}
//
//
impl Default for TemporalFilterConf {
    fn default() -> Self {
        Self {
            open_kernel: [5; 2],
            erode_kernel: [5; 2],
            threshold: 1.0,
        }
    }
}

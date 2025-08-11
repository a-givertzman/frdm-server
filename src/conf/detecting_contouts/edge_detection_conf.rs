use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for edge-detection algorithm
/// 
/// ### Example:
/// ```yaml
/// edge-detection:
///     threshold: 1                        # 0...255
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeDetectionConf {
    /// Configuration for edge-detection algorithm, 0...255
    pub threshold: u8,
}
//
// 
impl EdgeDetectionConf {
    ///
    /// Returns [EdgeDetectionConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "EdgeDetectionConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let threshold: u64 = conf.get("threshold").expect(&format!("{dbg}.new | 'threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | threshold: {:#?}", threshold);
        Self {
            threshold: threshold as u8,
        }
    }
}
//
//
impl Default for EdgeDetectionConf {
    fn default() -> Self {
        Self {
            threshold: 20,
        }
    }
}

use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Contour dectection` algorithm
/// 
/// ### Example:
/// ```yaml
/// fine-contours:
///     otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
///     merge-distance: 24.0    # Maximum distance between contours to be merged
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FineContoursConf {
    ///     - otsu-tune: 0.40 - Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
    pub otsu_tune: f64,
    ///     - merge-distance: 24.0 - Maximum distance between contours to be merged
    pub merge_distance: f64
}
//
// 
impl FineContoursConf {
    ///
    /// Returns [DetectingContoursConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "DetectingContoursConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune").expect(&format!("{dbg}.new | 'otsu-tune' - not found or wrong configuration"));
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let merge_distance = conf.get("merge-distance").expect(&format!("{dbg}.new | 'merge-distance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | merge-distance: {:#?}", merge_distance);
        Self {
            otsu_tune,
            merge_distance,
        }
    }
}
//
//
impl Default for FineContoursConf {
    fn default() -> Self {
        Self {
            otsu_tune: 0.4,
            merge_distance: 24.0,
        }
    }
}

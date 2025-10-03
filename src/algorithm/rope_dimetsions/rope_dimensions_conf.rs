use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `RopeDimensions` verification
/// 
/// ### Example:
/// ```yaml
/// rope-dimensions:
///     rope-width: 35              # Standart rope width, px
///     width-tolerance: 5.0        # Tolerance for rope width, %
///     square-tolerance: 10.0      # Tolerance for rope square, %
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RopeDimensionsConf {
    /// Standart rope width, px
    pub rope_width: usize,
    /// Tolerance for rope width, %
    pub width_tolerance: f64,
    /// Tolerance for rope square, %
    pub square_tolerance: f64,
}
//
// 
impl RopeDimensionsConf {
    ///
    /// Returns [RopeDimensionsConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "RopeDimensionsConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let rope_width: u64 = conf.get("rope-width").expect(&format!("{dbg}.new | 'rope-width' - not found or wrong configuration"));
        log::trace!("{dbg}.new | rope-width: {:?}", rope_width);
        let width_tolerance = conf.get("width-tolerance").expect(&format!("{dbg}.new | 'width-tolerance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | width-tolerance: {:?}", width_tolerance);
        let square_tolerance = conf.get("square-tolerance").expect(&format!("{dbg}.new | 'square-tolerance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | square-tolerance: {:?}", square_tolerance);
        Self {
            rope_width: rope_width as usize,
            width_tolerance,
            square_tolerance,
        }
    }
}
//
//
impl Default for RopeDimensionsConf {
    fn default() -> Self {
        Self {
            rope_width: 35,
            width_tolerance: 10.0,
            square_tolerance: 10.0,
        }
    }
}

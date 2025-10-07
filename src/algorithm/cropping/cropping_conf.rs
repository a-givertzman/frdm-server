use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Cropping` operator
/// 
/// ### Example:
/// ```yaml
/// cropping:
///     x: 10           # new left edge
///     width: 1900     # new image width
///     y: 10           # new top edge
///     height: 1180    # new image height
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CroppingConf {
    /// - `x` - new left edge
    pub x: i32,
    /// - `width` - new image width
    pub width: i32,
    /// - `y` - new top edge
    pub y: i32,
    /// - `height` - new image height
    pub height: i32,
}
//
// 
impl CroppingConf {
    ///
    /// Returns [CroppingConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "CroppingConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let x: i64 = conf.get("x").expect(&format!("{dbg}.new | 'x' - not found or wrong configuration"));
        log::trace!("{dbg}.new | x: {:?}", x);
        let width: i64 = conf.get("width").expect(&format!("{dbg}.new | 'width' - not found or wrong configuration"));
        log::trace!("{dbg}.new | width: {:?}", width);
        let y: i64 = conf.get("y").expect(&format!("{dbg}.new | 'y' - not found or wrong configuration"));
        log::trace!("{dbg}.new | y: {:?}", y);
        let height: i64 = conf.get("height").expect(&format!("{dbg}.new | 'height' - not found or wrong configuration"));
        log::trace!("{dbg}.new | height: {:?}", height);
        Self {
            x: x as i32,
            width: width as i32,
            y: y as i32,
            height: height as i32,
        }
    }
}
//
//
impl Default for CroppingConf {
    fn default() -> Self {
        Self {
            x: 0,
            width: 1920,
            y: 0,
            height: 1200,
        }
    }
}

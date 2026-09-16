use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};

///
/// ## Configuration for `BitwiseAnd` algorithm
/// 
/// ### Example:
/// ```yaml
/// bitwise-and:
///     no-params: no parameters required
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitwiseAndConf {
}
//
// 
impl BitwiseAndConf {
    ///
    /// Returns [BitwiseAndConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "BitwiseAndConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        Self {
        }
    }
}
//
//
impl Default for BitwiseAndConf {
    fn default() -> Self {
        Self {
        }
    }
}

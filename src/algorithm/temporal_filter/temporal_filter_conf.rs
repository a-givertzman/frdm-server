use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Cropping` operator
/// 
/// ### Example:
/// ```yaml
/// temporal-filter:
///     amplify_factor: 10.0     # factor amplifies the highlighting the oftenly changing pixels, default 10.0
///     reduce_factor: 10.0      # factor amplifies the hiding the lower changing pixels, default 10.0
///     threshold: 1.0          # ..., default ...
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalFilterConf {
    /// factor amplifies the highlighting the oftenly changing pixels
    pub amplify_factor: f64,
    /// 
    pub reduce_factor: f64,
    /// 
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
        let amplify_factor = conf.get("amplify-factor").unwrap_or(1.0);
        log::trace!("{dbg}.new | amplify-factor: {:?}", amplify_factor);
        let reduce_factor = conf.get("reduce-factor").unwrap_or(1.0);
        log::trace!("{dbg}.new | reduce-factor: {:?}", reduce_factor);
        let threshold = conf.get("threshold").unwrap_or(1.0);
        log::trace!("{dbg}.new | threshold: {:?}", threshold);
        Self {
            amplify_factor,
            reduce_factor,
            threshold,
        }
    }
}
//
//
impl Default for TemporalFilterConf {
    fn default() -> Self {
        Self {
            amplify_factor: 10.0,
            reduce_factor: 10.0,
            threshold: 1.0,
        }
    }
}

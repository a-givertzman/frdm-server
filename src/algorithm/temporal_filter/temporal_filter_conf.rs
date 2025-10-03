use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `Cropping` operator
/// 
/// ### Example:
/// ```yaml
/// temporal-filter:
///     amplify-factor: 10.0     # factor amplifies the highlighting the oftenly changing pixels, default 10.0
///     grow-speed: 0.1          # speed of `rate` growing for changed pixels, 1 - default speed, depends on pixel change value
///     reduce-factor: 10.0      # factor amplifies the hiding the lower changing pixels, default 10.0
///     down-speed: 0.5          # speed of `rate` reducing for static pixels, 1 - default speed, depends on pixel change value
///     threshold: 1.0           # Threshold to detect yhe pixel whas changed or not in the each next frame, default 1
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalFilterConf {
    /// Factor amplifies the highlighting the oftenly changing pixels
    pub amplify_factor: f64,
    /// Speed of `rate` growing for changed pixels, 1 - default speed, depends on pixel change value
    pub grow_speed: f64,
    /// Factor amplifies the hiding the lower changing pixels
    pub reduce_factor: f64,
    /// Speed of `rate` reducing for static pixels, 1 - default speed, depends on pixel change value
    pub down_speed: f64,
    /// Threshold to detect yhe pixel whas changed or not in the each next frame
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
        let grow_speed = conf.get("grow-speed").unwrap_or(1.0);
        log::trace!("{dbg}.new | grow-speed: {:?}", grow_speed);
        let reduce_factor = conf.get("reduce-factor").unwrap_or(1.0);
        log::trace!("{dbg}.new | reduce-factor: {:?}", reduce_factor);
        let down_speed = conf.get("down-speed").unwrap_or(1.0);
        log::trace!("{dbg}.new | down-speed: {:?}", down_speed);
        let threshold = conf.get("threshold").unwrap_or(1.0);
        log::trace!("{dbg}.new | threshold: {:?}", threshold);
        Self {
            amplify_factor,
            grow_speed,
            reduce_factor,
            down_speed,
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
            grow_speed: 1.0,
            reduce_factor: 10.0,
            down_speed: 1.0,
            threshold: 1.0,
        }
    }
}

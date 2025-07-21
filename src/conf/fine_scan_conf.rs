use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};

#[derive(Debug, PartialEq, Clone)]
pub struct FineScanConf {}
//
//
impl FineScanConf {
    ///
    /// Returns [FineScanConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "RopeConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::debug!("{}.new | name: {:?}", dbg, name);
        // let geometry_defect_threshold = conf.get("geometry-defect-threshold").unwrap();
        // let geometry_defect_threshold = Threshold(geometry_defect_threshold);
        // log::debug!("{dbg}.new | geometry-defect-threshold: {:?}", geometry_defect_threshold);
        Self {
            // geometry_defect_threshold,
        }
    }
}
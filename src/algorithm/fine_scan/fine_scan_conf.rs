use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::algorithm::FineContoursConf;

#[derive(Debug, PartialEq, Clone)]
pub struct FineScanConf {
    pub fine_contours: FineContoursConf
}
//
//
impl FineScanConf {
    ///
    /// Returns [FineScanConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FineScanConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fine_contours = conf.get("fine-contours").expect(&format!("{dbg}.new | 'fine-contours' - not found or wrong configuration"));
        let fine_contours = FineContoursConf::new(&name, fine_contours);
        log::trace!("{dbg}.new | fine-contours: {:?}", fine_contours);
        Self {
            fine_contours
        }
    }
}
//
//
impl Default for FineScanConf {
    fn default() -> Self {
        Self {
            fine_contours: FineContoursConf::default(),
        }
    }
}

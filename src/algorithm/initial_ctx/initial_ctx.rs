use sal_core::dbg::Dbg;
use crate::infrostructure::arena::Image;
///
/// Storage of [initial data](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone)]
pub struct InitialCtx {
    dbg: Dbg,
    /// initial frame of the rope
    pub src_frame: Image,
} 
//
//
impl InitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new(src_frame: Image) -> Self {
        let dbg = Dbg::own("InitialCtx");
        Self {
            dbg,
            src_frame
        }
    }
}

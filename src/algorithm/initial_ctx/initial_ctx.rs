use sal_core::dbg::Dbg;

///
/// Storage of [initial data](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone)]
pub struct InitialCtx {
    dbg: Dbg,
} 
//
//
impl InitialCtx {
    ///
    /// Struct constructor
    /// - 'storage_initial_data' - [Storage] instance, where store initial data
    pub fn new() -> Self {
        let dbg = Dbg::own("InitialCtx");
        Self {
            dbg,
        }
    }
}

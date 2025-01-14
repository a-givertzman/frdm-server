use crate::domain::dbg::dbgid::DbgId;

///
/// # Description to the [Camera] class
/// - Connecting to the USB Camra
/// - Receive frames from the `Camera`
pub struct Camera {
    dbg: DbgId,
    conf: CameraConf,
}
//
//
impl Camera {
    ///
    /// Returns [Camera] new instance
    /// - [parent] - DbgId of parent entitie
    /// - `conf` - configuration parameters
    pub fn new(parent: DbgId, conf: CameraConf) -> Self {
        log::
        Self {
            dbg: DbgId::new(parent, "Camera"),
            conf: conf
        }
    }
    ///
    /// Receive frames from USB camera
    pub fn read(&self) -> Vec<Vec<f64>> {
        todo!("{}.read | To be implemented", self.dbg)
    }
}
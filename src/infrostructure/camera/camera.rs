use crate::domain::dbg::dbgid::DbgId;
use super::camera_conf::CameraConf;
///
/// # Description to the [Camera] class
/// - Connecting to the IP Camra
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
    pub fn new(parent: &DbgId, conf: CameraConf) -> Self {
        let dbg = DbgId::new(&parent, "Camera");
        log::debug!("{}.new | parent: {}", dbg, parent);
        Self {
            dbg,
            conf,
        }
    }
    ///
    /// Receive frames from IP camera
    pub fn read(&self) -> Vec<Vec<f64>> {
        todo!("{}.read | To be implemented", self.dbg)
    }
}
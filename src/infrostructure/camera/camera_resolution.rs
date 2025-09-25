use sal_sync::services::conf::ConfTree;
use serde::Deserialize;
///
/// The resolution of the camera
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CameraResolution {
    /// width parametr of camera
    pub width: usize,
    /// height parametr of camera
    pub height: usize,
}
//
//
impl CameraResolution{
    ///
    /// reads IP camera resolution from yaml
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        log::trace!("{}/CameraConf.new | conf_tree: {:?}", parent.into(), conf);
        serde_yaml::from_value(conf.conf.clone()).unwrap()
    }
}
//
//
impl Default for CameraResolution {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default() }
    }
}
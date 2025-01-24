use sal_sync::services::conf::conf_tree::ConfTree;
use serde::Deserialize;
///
/// The resolution of the camera
#[derive(Clone, Debug, Deserialize)]
pub struct CameraResolution {
    #[serde(alias = "w")]
    pub width: usize,
    #[serde(default)]
    pub height: usize,
}
//
//
impl CameraResolution{
    ///
    /// reads IP camera resolution from yaml
    pub fn new(parent: impl Into<String>, conf_tree: &mut ConfTree) -> Self {
    
    }
}
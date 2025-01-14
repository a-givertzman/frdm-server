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
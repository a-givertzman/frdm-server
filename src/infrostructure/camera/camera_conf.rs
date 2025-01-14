use std::net::SocketAddr;
use super::camera_resolution::CameraResolution;
///
/// Configuration parameters for ip [Camera] class
pub struct CameraConf {
    /// frames per second
    pub fps: usize,
    /// Camera cesolution setting
    pub resolution: CameraResolution,
    /// Ip and port address of the camera
    pub address: SocketAddr,
}

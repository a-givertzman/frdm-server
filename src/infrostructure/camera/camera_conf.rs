use std::net::SocketAddr;
use sal_sync::services::conf::conf_tree::ConfTree;
use super::camera_resolution::CameraResolution;
use serde::Deserialize;
use log::{trace, debug};
///
/// Configuration parameters for ip [Camera] class
#[derive(Clone, Debug, Deserialize)]
pub struct CameraConf {
    /// frames per second
    pub fps: usize,
    /// Camera cesolution setting
    pub resolution: CameraResolution,
    /// Ip and port address of the camera
    pub address: SocketAddr,
}
//
//
impl CameraConf {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// Camera:
    ///     fps: 30
    ///     addres: 192.168.10.12:2020
    ///     resolution: 
    ///         width: 1200
    ///         height: 800
    /// ```
    pub fn new(parent: impl Into<String>, conf_tree: &mut ConfTree) -> Self {
        println!();
        trace!("CameraConf.new | conf_tree: {:?}", conf_tree);
        let self_id = format!("CameraConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        
    }
}
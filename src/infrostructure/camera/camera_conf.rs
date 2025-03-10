use std::{fs, net::SocketAddr};
use sal_sync::services::{conf::conf_tree::ConfTree, entity::{dbg_id::DbgId, name::Name}};
use crate::conf::service_config::ServiceConfig;
use super::camera_resolution::CameraResolution;
use log::{trace, debug};
///
/// Configuration parameters for ip [Camera] class
#[derive(Clone, Debug, PartialEq)]
pub struct CameraConf {
    pub name: Name,
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
    /// service Camera Camera1:
    ///     fps: 30
    ///     addres: 192.168.10.12:2020
    ///     resolution: 
    ///         width: 1200
    ///         height: 800
    /// ```
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        println!();
        trace!("CameraConf.new | conf_tree: {:?}", conf_tree);
        let self_id = format!("CameraConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let sufix = self_conf.sufix();
        let self_name = Name::new(parent, if sufix.is_empty() {self_conf.name()} else {sufix});
        let self_fps = self_conf.get_param_value("fps").unwrap().as_u64().unwrap();
        debug!("{}.new | fps: {:?}", self_id, self_fps);
        let resolution = self_conf.get_param_conf("resolution").unwrap();
        let resolution = CameraResolution::new(self_name.join(), &resolution);
        debug!("{}.new | resolution: {:?}", self_id, resolution);
        let self_address: SocketAddr = self_conf.get_param_value("address").unwrap().as_str().unwrap().parse().unwrap();
        debug!("{}.new | address: {:?}", self_id, self_address);
        Self {
            name: self_name,
            fps: self_fps as usize, 
            resolution: resolution, 
            address: self_address, 
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> CameraConf {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(parent, &ConfTree::new(key.as_str().unwrap(), value.clone()))
            }
            None => {
                panic!("CameraConf.from_yaml | Format error or empty conf: {:#?}", value)
            }
        }        
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(parent: impl Into<String>, path: &str) -> CameraConf {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        CameraConf::from_yaml(parent, &config)
                    }
                    Err(err) => {
                        panic!("CameraConf.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    }
                }
            }
            Err(err) => {
                panic!("CameraConf.read | File {} reading error: {:?}", path, err)
            }
        }
    }
}
use log::{debug, trace};
use sal_sync::services::conf::conf_tree::ConfTree;
use serde::Deserialize;
use crate::conf::service_config::ServiceConfig;
///
/// The resolution of the camera
#[derive(Clone, Debug, Deserialize)]
pub struct CameraResolution {
    // #[serde(alias = "w")]
    pub width: usize,
    // #[serde(default)]
    pub height: usize,
}
//
//
impl CameraResolution{
    ///
    /// reads IP camera resolution from yaml
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        println!();
        trace!("CameraConf.new | conf_tree: {:?}", conf_tree);
        serde_yaml::from_value(conf_tree.conf.clone()).unwrap()
        // let self_id = format!("CameraConf({})", conf_tree.key);
        // let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        // trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        // let sufix = self_conf.sufix();
        // let self_width = self_conf.get_param_value("width").unwrap().as_u64().unwrap();
        // debug!("{}.new | width: {:?}", self_id, self_width);
        // let self_height = self_conf.get_param_value("height").unwrap().as_u64().unwrap();
        // debug!("{}.new | height: {:?}", self_id, self_height);
        // Self { 
        //     width: self_width as usize,
        //     height: self_height as usize,
        // }
    }
}
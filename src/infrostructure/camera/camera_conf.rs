use std::{fs, net::SocketAddr};
use sal_sync::services::{conf::conf_tree::ConfTree, entity::name::Name};
use crate::{conf::service_config::ServiceConfig, infrostructure::arena::{exposure::Exposure, pixel_format::PixelFormat}};
use super::camera_resolution::CameraResolution;
///
/// Configuration parameters for ip [Camera] class
#[derive(Clone, Debug, PartialEq)]
pub struct CameraConf {
    pub name: Name,
    ///
    /// frames per second
    pub fps: usize,
    ///
    /// Camera cesolution setting
    pub resolution: CameraResolution,
    ///
    /// Ip and port address of the camera
    pub address: SocketAddr,
    ///
    /// Pixel format
    /// - Mono8/10/12/16,
    /// - Bayer8/10/12/16,
    /// - RGB8, BGR8,
    /// - YCbCr8, YCbCr411, 
    /// - YUV422, YUV411,
    pub pixel_format: PixelFormat,
    ///
    /// Exposure settings for the camera
    ///     - Set exposure time
    ///     - Disable automatic exposure before setting an exposure time
    pub exposure: Exposure,
    ///
	/// Enable stream auto negotiate packet size
    /// 
	///    Setting the stream packet size is done before starting the stream.
	///    Setting the stream to automatically negotiate packet size instructs
	///    the camera to receive the largest packet size that the system will
	///    allow. This generally increases frame rate and results in fewer
	///    interrupts per image, thereby reducing CPU load on the host system.
	///    Ethernet settings may also be manually changed to allow for a
	///    larger packet size.
    pub auto_packet_size: bool,
    ///
	/// Enable stream packet resend
    ///
	///    Enable stream packet resend before starting the stream. Images are
	///    sent from the camera to the host in packets using UDP protocol,
	///    which includes a header image number, packet number, and timestamp
	///    information. If a packet is missed while receiving an image, a
	///    packet resend is requested and this information is used to retrieve
	///    and redeliver the missing packet in the correct order.
    pub packet_resend: bool,

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
    ///     index: 0                    # the number of Camera
    ///     resolution: 
    ///         width: 1200
    ///         height: 800
    ///     pixel-format: BayerBG8      # Mono8/10/12/16, Bayer8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411
    ///     exposure:
    ///         auto: Off               # Off / Continuous
    ///         time: 5000              # microseconds
    ///     auto-packet-size: true
    ///     packet-resend: false
    /// ```
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        log::trace!("CameraConf.new | conf_tree: {:?}", conf_tree);
        let self_id = format!("CameraConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        log::trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let sufix = self_conf.sufix();
        let self_name = Name::new(parent, if sufix.is_empty() {self_conf.name()} else {sufix});
        let self_fps = self_conf.get_param_value("fps").unwrap().as_u64().unwrap();
        log::debug!("{}.new | fps: {:?}", self_id, self_fps);
        let resolution = self_conf.get_param_conf("resolution").unwrap();
        let resolution = CameraResolution::new(self_name.join(), &resolution);
        log::debug!("{}.new | resolution: {:?}", self_id, resolution);
        let self_address: SocketAddr = self_conf.get_param_value("address").unwrap().as_str().unwrap().parse().unwrap();
        log::debug!("{}.new | address: {:?}", self_id, self_address);
        let pixel_format = self_conf.get_param_value("pixel-format").unwrap();
        let pixel_format: PixelFormat = serde_yaml::from_value(pixel_format).unwrap();
        log::debug!("{}.new | pixel-format: {:?}", self_id, pixel_format);

        let exposure = self_conf.get_param_value("exposure").unwrap();
        let exposure: Exposure = serde_yaml::from_value(exposure).unwrap();
        log::debug!("{}.new | exposure: {:?}", self_id, exposure);

        let auto_packet_size = self_conf.get_param_value("auto-packet-size").unwrap().as_bool().unwrap();
        log::debug!("{}.new | auto-packet-size: {:?}", self_id, auto_packet_size);

        let packet_resend = self_conf.get_param_value("packet-resend").unwrap().as_bool().unwrap();
        log::debug!("{}.new | packet-resend: {:?}", self_id, packet_resend);
        Self {
            name: self_name,
            fps: self_fps as usize, 
            resolution: resolution, 
            address: self_address,
            pixel_format,
            exposure,
            auto_packet_size,
            packet_resend,
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
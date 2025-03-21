use std::{fs, net::SocketAddr};
use sal_sync::services::{conf::conf_tree::ConfTree, entity::name::Name};
use crate::{conf::service_config::ServiceConfig, infrostructure::arena::{channel_packet_size::ChannelPacketSize, exposure::Exposure, frame_rate::FrameRate, pixel_format::PixelFormat}};
use super::camera_resolution::CameraResolution;
///
/// Configuration parameters for ip [Camera] class
#[derive(Clone, Debug, PartialEq)]
pub struct CameraConf {
    pub name: Name,
    ///
    /// Rame Rate (frames per second)
    /// - `Min` - Minimum supported
    /// - `Max` - Maximum supported
    /// - `Val` - User specified value, FPS
    pub fps: FrameRate,
    ///
    /// Camera cesolution setting
    pub resolution: CameraResolution,
    ///
    /// Camera index, if IP addres is dynamic or unknown
    pub index: Option<usize>,
    ///
    /// Ip and port address of the camera, if specified statically
    pub address: Option<SocketAddr>,
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
    /// Set maximum stream channel packet size
    /// 
	///    Maximizing packet size increases frame rate by reducing the amount of
	///    overhead required between images. This includes both extra
	///    header/trailer data per packet as well as extra time from intra-packet
	///    spacing (the time between packets). In order to grab images at the
	///    maximum packet size, the Ethernet adapter must be configured
	///    appropriately: 'Jumbo packet' must be set to its maximum, 'UDP checksum
	///    offload' must be set to 'Rx & Tx Enabled', and 'Received Buffers' must
	///    be set to its maximum.
    pub channel_packet_size: ChannelPacketSize,
    ///
	/// Enable stream packet resend
    ///
	///    Enable stream packet resend before starting the stream. Images are
	///    sent from the camera to the host in packets using UDP protocol,
	///    which includes a header image number, packet number, and timestamp
	///    information. If a packet is missed while receiving an image, a
	///    packet resend is requested and this information is used to retrieve
	///    and redeliver the missing packet in the correct order.
    pub resend_packet: bool,

}
//
//
impl CameraConf {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service Camera Camera1:
    ///     fps: 30.0                   # Max / Min / 30.0
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
    ///     resend-packet: false
    /// ```
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        log::trace!("CameraConf.new | conf_tree: {:?}", conf_tree);
        let self_id = format!("CameraConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        log::trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let sufix = self_conf.sufix();
        let name = Name::new(parent, if sufix.is_empty() {self_conf.name()} else {sufix});
        let fps = self_conf.get_param_value("fps").unwrap();
        let fps: FrameRate = serde_yaml::from_value(fps).unwrap();
        log::debug!("{}.new | fps: {:?}", self_id, fps);
        let resolution = self_conf.get_param_conf("resolution").unwrap();
        let resolution = CameraResolution::new(name.join(), &resolution);
        log::debug!("{}.new | resolution: {:?}", self_id, resolution);
        let index = self_conf.get_param_value("index").map(|ix| ix.as_u64().unwrap() as usize).ok();
        log::debug!("{}.new | index: {:?}", self_id, index);
        let address: Option<SocketAddr> = self_conf.get_param_value("address").map(|addr| addr.as_str().unwrap().parse().unwrap()).ok();
        log::debug!("{}.new | address: {:?}", self_id, address);
        let pixel_format = self_conf.get_param_value("pixel-format").unwrap();
        let pixel_format: PixelFormat = serde_yaml::from_value(pixel_format).unwrap();
        log::debug!("{}.new | pixel-format: {:?}", self_id, pixel_format);
        let exposure = self_conf.get_param_value("exposure").unwrap();
        let exposure: Exposure = serde_yaml::from_value(exposure).unwrap();
        log::debug!("{}.new | exposure: {:?}", self_id, exposure);
        let auto_packet_size = self_conf.get_param_value("auto-packet-size").unwrap().as_bool().unwrap();
        log::debug!("{}.new | auto-packet-size: {:?}", self_id, auto_packet_size);
        let channel_packet_size = self_conf.get_param_value("channel-packet-size").unwrap();
        let channel_packet_size: ChannelPacketSize = serde_yaml::from_value(channel_packet_size).unwrap();
        log::debug!("{}.new | channel-packet-size: {:?}", self_id, channel_packet_size);
        let resend_packet = self_conf.get_param_value("resend-packet").unwrap().as_bool().unwrap();
        log::debug!("{}.new | resend-packet: {:?}", self_id, resend_packet);
        Self {
            name,
            fps, 
            resolution, 
            index,
            address,
            pixel_format,
            exposure,
            auto_packet_size,
            channel_packet_size,
            resend_packet,
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
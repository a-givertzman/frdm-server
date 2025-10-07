use std::{fs, net::SocketAddr, str::FromStr};
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfCustomKeywd, ConfTree, ConfTreeGet}, entity::Name};
use crate::infrostructure::arena::{ChannelPacketSize, Exposure, FrameRate, PixelFormat};
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
    ///
    /// Frames can be read from the files if specified
    pub from_path: Option<String>,

}
//
//
impl CameraConf {
    ///
    /// Returns config from serde_yaml::Value of following format:
    /// ```yaml
    /// camera Camera1:
    ///     fps: Max                    # Max / Min / 30.0
    ///     resolution: 
    ///         width: 1200
    ///         height: 800
    ///     index: 0
    ///     # address: 192.168.10.12:2020
    ///     pixel-format: BayerRG8          # Mono8/10/12/16, Bayer8/10/12/16, RGB8, BGR8, YCbCr8, YCbCr411, YUV422, YUV411 | Default and fastest BayerRG8
    ///     exposure:
    ///         auto: Continuous                   # Off / Continuous
    ///         time: 10000                   # microseconds
    ///     auto-packet-size: true          # StreamAutoNegotiatePacketSize
    ///     channel-packet-size: Max        # Maximizing packet size increases frame rate
    ///     resend-packet: true             # StreamPacketResendEnable
    /// ```
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        let parent = parent.into();
        let conf_keywd = ConfCustomKeywd::from_str(&conf.key).unwrap();
        log::trace!("{}.new | conf.name: {:?}", "CameraConf", conf_keywd.name());
        let me = match conf_keywd.title().is_empty() {
            true => conf_keywd.name(),
            false => conf_keywd.title(),
        };
        let dbg = Dbg::new(&parent, format!("CameraConf({})", me));
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let from_path: Option<String> = conf.get("from-path");
        log::trace!("{}.new | from-path: {:?}", dbg, from_path);
        match from_path {
            Some(_) => {
                Self {
                    name,
                    fps: FrameRate::Min,
                    resolution: CameraResolution::default(),
                    index: None,
                    address: None,
                    pixel_format: PixelFormat::Mono8,
                    exposure: Exposure::default(),
                    auto_packet_size: false,
                    channel_packet_size: ChannelPacketSize::Min,
                    resend_packet: false,
                    from_path,
                }
            }
            None => {
                let fps = conf.get("fps").unwrap();
                let fps: FrameRate = serde_yaml::from_value(fps).unwrap();
                log::trace!("{}.new | fps: {:?}", dbg, fps);
                let resolution = conf.get("resolution").unwrap();
                let resolution = CameraResolution::new(name.join(), &resolution);
                log::trace!("{}.new | resolution: {:?}", dbg, resolution);
                let index = conf.get("index").map(|ix: u64| ix as usize);
                log::trace!("{}.new | index: {:?}", dbg, index);
                let address: Option<SocketAddr> = conf.get("address").map(|addr: String| addr.parse().unwrap());
                log::trace!("{}.new | address: {:?}", dbg, address);
                let pixel_format = conf.get("pixel-format").unwrap();
                let pixel_format: PixelFormat = serde_yaml::from_value(pixel_format).unwrap();
                log::trace!("{}.new | pixel-format: {:?}", dbg, pixel_format);
                let exposure = conf.get("exposure").unwrap();
                let exposure: Exposure = serde_yaml::from_value(exposure).unwrap();
                log::trace!("{}.new | exposure: {:?}", dbg, exposure);
                let auto_packet_size = conf.get("auto-packet-size").unwrap();
                log::trace!("{}.new | auto-packet-size: {:?}", dbg, auto_packet_size);
                let channel_packet_size = conf.get("channel-packet-size").unwrap();
                let channel_packet_size: ChannelPacketSize = serde_yaml::from_value(channel_packet_size).unwrap();
                log::trace!("{}.new | channel-packet-size: {:?}", dbg, channel_packet_size);
                let resend_packet = conf.get("resend-packet").unwrap();
                log::trace!("{}.new | resend-packet: {:?}", dbg, resend_packet);
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
                    from_path,
                }
            }
        }
    }
    ///
    /// Returns config from serde_yaml::Value of following format:
    pub fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> CameraConf {
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
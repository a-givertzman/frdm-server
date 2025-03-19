use sal_sync::services::conf::conf_tree::ConfTree;
use serde::Deserialize;

use super::{exposure::Exposure, pixel_format::PixelFormat};
///
/// The set of options for the camera
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AcDeviceConf {
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
    pub auto_negotiate_packet_size: bool,
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
impl AcDeviceConf{
    ///
    /// Returns [AcDeviceConf] new instance from yaml
    pub fn new(parent: impl Into<String>, conf_tree: &ConfTree) -> Self {
        log::trace!("{}/AcDeviceConf.new | conf_tree: {:?}", parent.into(), conf_tree);
        serde_yaml::from_value(conf_tree.conf.clone()).unwrap()
    }
}
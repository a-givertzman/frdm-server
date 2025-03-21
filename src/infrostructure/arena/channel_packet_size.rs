use serde::Deserialize;

///
/// Set maximum stream channel packet size
///    Maximizing packet size increases frame rate by reducing the amount of
///    overhead required between images. This includes both extra
///    header/trailer data per packet as well as extra time from intra-packet
///    spacing (the time between packets). In order to grab images at the
///    maximum packet size, the Ethernet adapter must be configured
///    appropriately: 'Jumbo packet' must be set to its maximum, 'UDP checksum
///    offload' must be set to 'Rx & Tx Enabled', and 'Received Buffers' must
///    be set to its maximum.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum ChannelPacketSize {
    Min,
    Max,
    #[serde(untagged)]
    Val(i64),
}

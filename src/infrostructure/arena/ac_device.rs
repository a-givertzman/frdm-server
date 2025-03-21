use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use sal_sync::services::entity::{error::str_err::StrErr, name::Name};
use crate::infrostructure::{arena::{ac_access_mode::AcAccessMode, bindings::{
    acBuffer, acDeviceGetBuffer, acDeviceGetTLStreamNodeMap, acDeviceStartStream, acDeviceStopStream,
}}, camera::camera_conf::CameraConf};

use super::{
    ac_buffer::AcBuffer, ac_err::AcErr, ac_image::AcImage, ac_node_map::AcNodeMap, bindings::{
        acDevice, acDeviceGetNodeMap, acNodeMap, acSystem, acSystemCreateDevice, acSystemDestroyDevice
    }, channel_packet_size::ChannelPacketSize, exposure::{Exposure, ExposureAuto}, frame_rate::FrameRate
};

///
/// Represents a Arena SDK device, used to configure and stream a device.
pub struct AcDevice {
    name: Name,
    index: usize,
    device: acDevice,
    system: acSystem,
    conf: CameraConf,
    // Maximum time to wait for an image buffer
    image_timeout: u64,
    exit: Arc<AtomicBool>,
}
//
//
impl AcDevice {
    ///
    /// Returns [AcDevice] new instance
    /// - `exit` - Exit signal, write true to stop reading.
    pub fn new(
        parent: impl Into<String>,
        system: acSystem,
        index: usize,
        conf: CameraConf,
        exit: Option<Arc<AtomicBool>>,
    ) -> Self {
        let name = Name::new(parent.into(), format!("AcDevice({index})"));
        Self {
            name,
            index,
            device: std::ptr::null_mut(),
            system,
            conf,
            image_timeout: 3000,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// 
    pub fn listen(&mut self, on_event: impl Fn(AcImage)) -> Result<(), StrErr> {
        log::debug!("{}.listen | Started", self.name);
        unsafe {
            let err = AcErr::from(acSystemCreateDevice(self.system, self.index, &mut self.device));
            match err {
                AcErr::Success => self.read(on_event),
                _ => {
                    Err(StrErr(format!("{}.listen | CreateDevice Error: {}", self.name, err)))
                }
            }
        }
    }
    ///
    /// Returns the already initialized DeviceNodeMap (`acNodeMap`), used to access a device's complete feature set of nodes (acNode).
    pub fn node(&self) -> Result<AcNodeMap, StrErr> {
        unsafe {
            let mut h_node_map: acNodeMap = std::ptr::null_mut();
            let err = AcErr::from(acDeviceGetNodeMap(self.device, &mut h_node_map));
            match err {
                AcErr::Success => Ok(AcNodeMap::new(&self.name, self.device, h_node_map, "DeviceNodeMap")),
                _ => Err(StrErr(format!("{}.node | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// Returns the already initialized TLStreamNodeMap node map (`acNodeMap`), used to access a device's complete feature set of nodes (acNode).
    pub fn tls_stream_node(&self) -> Result<AcNodeMap, StrErr> {
        unsafe {
            let mut h_tlstream_node_map: acNodeMap = std::ptr::null_mut();
            let err = AcErr::from(acDeviceGetTLStreamNodeMap(self.device, &mut h_tlstream_node_map));
            match err {
                AcErr::Success => Ok(AcNodeMap::new(&self.name, self.device, h_tlstream_node_map, "TLStreamNodeMap")),
                _ => Err(StrErr(format!("{}.tls_stream_node | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// Returns Device buffer
    fn get_buffer(&self) -> Result<AcBuffer, StrErr> {
        let mut buffer: acBuffer = std::ptr::null_mut();
        let err = AcErr::from(unsafe { acDeviceGetBuffer(self.device, self.image_timeout, &mut buffer) });
        match err {
            AcErr::Success => Ok(AcBuffer::new(&self.name, self.device, buffer, self.conf.pixel_format)),
            _ => Err(StrErr(format!("{}.buffer | Error: {}", self.name, err))),
        }
    }
    ///
    /// Set acquisition frame rate, FPS
    fn set_frame_rate(&self, node_map: &AcNodeMap, value: FrameRate) -> Result<(), StrErr> {
        let dbg = self.name.join();
        let acq_fr_en = true;
        match node_map.set_bool_value("AcquisitionFrameRateEnable", acq_fr_en) {
            Ok(_) => match node_map.get_node("AcquisitionFrameRate") {
                Ok(node) => {
                    log::debug!(
                        "{}.set_frame_rate | AcquisitionFrameRate range: {}...{} FPS", dbg,
                        node.get_float_min_value().map_or_else(|err| format!("{err}"), |v| format!("{:.3}", v)),
                        node.get_float_max_value().map_or_else(|err| format!("{err}"), |v| format!("{:.3}", v)),
                    );
                    log::debug!("{}.set_frame_rate | AcquisitionFrameRate prev: {} FPS", dbg, node.get_float_value().map_or_else(|err| format!("{err}"), |v| format!("{:.3}", v)),);
                    let val = match value {
                        FrameRate::Min => match node.get_float_min_value() {
                            Ok(val) => Ok(val),
                            Err(err) => Err(StrErr(format!("{}.set_frame_rate | Get Min Error: {}", dbg, err))),
                        }
                        FrameRate::Max => match node.get_float_max_value() {
                            Ok(val) => Ok(val),
                            Err(err) => Err(StrErr(format!("{}.set_frame_rate | Get Max Error: {}", dbg, err))),
                        },
                        FrameRate::Val(val) => Ok(val)
                    };
                    match val {
                        Ok(val) => match node.set_float_value(val) {
                            Ok(_) => {
                                log::debug!("{}.set_frame_rate | AcquisitionFrameRate changed to {:?} ({:.3} FPS)", dbg, value, val);
                                Ok(())
                            }
                            Err(err) => Err(StrErr(format!("{}.set_frame_rate | AcquisitionFrameRate change to {:?} Error: {}", dbg, value, err))),
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(StrErr(format!("{}.set_frame_rate | Error: {}", dbg, err))),
            }
            Err(err) => Err(StrErr(format!("{}.set_frame_rate | AcquisitionFrameRateEnable change to {} Error: {}", dbg, acq_fr_en, err))),
        }
    }
    ///
    /// Set Exposure time
    fn set_exposure(&self, node_map: &AcNodeMap, exposure: Exposure) -> Result<(), StrErr> {
        let dbg = self.name.join();
        match node_map.set_enumeration_value("ExposureAuto", exposure.auto.as_str()) {
            Ok(_) => log::debug!("{}.set_exposure | ExposureAuto changed to: {}", dbg, exposure.auto),
            Err(err) => log::warn!("{}.set_exposure | Set ExposureAuto Error: {}", dbg, err),
        };
        match exposure.auto {
            ExposureAuto::Off => match node_map.get_node("ExposureTime") {
                Ok(node) => {
                    if node.is_writable() {
                        log::debug!(
                            "{}.set_exposure | Exposure time range: {}...{} us", dbg,
                            node.get_float_min_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                            node.get_float_max_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                        );
                        log::debug!("{}.set_exposure | Exposure time prev: {} us", dbg, node.get_float_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),);
                        match node.set_float_value(exposure.time) {
                            Ok(_) => {
                                // log::debug!("{}.set_exposure | Set Exposure {} us - Ok", dbg, self.exposure.time);
                                if let Ok(exposure) = node.get_float_value() {
                                    log::debug!("{}.set_exposure | Exposure time changed to: {} us", dbg, exposure);
                                }
                                Ok(())
                            }
                            Err(err) => Err(StrErr(format!("{}.set_exposure | Set Exposure {} us Error: {}", dbg, exposure.time, err))),
                        }
                    } else {
                        Err(StrErr(format!("{}.set_exposure | Set Exposure {} us Error: ExposureTime Node - is not writable", dbg, exposure.time)))
                    }
                },
                Err(err) => Err(StrErr(format!("{}.set_exposure | Get ExposureTime Node Error: {}", dbg, err))),
            }
            ExposureAuto::Continuous => Ok(()),
        }
    }
    ///
    /// Set DeviceStream Channel Packet Size
    fn set_stream_channel_packet_size(&self, node_map: &AcNodeMap, size: ChannelPacketSize) -> Result<(), StrErr> {
        let dbg = self.name.join();
        match node_map.get_node("DeviceStreamChannelPacketSize") {
            Ok(node) => {
                log::debug!(
                    "{}.set_packet_size | Packet Size range: {}...{}", dbg,
                    node.get_int_min_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                    node.get_int_max_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                );
                let val = match size {
                    ChannelPacketSize::Min => match node.get_int_min_value() {
                        Ok(val) => Ok(val),
                        Err(err) => Err(StrErr(format!("{}.set_stream_channel_packet_size | Get Min Error: {}", dbg, err))),
                    }
                    ChannelPacketSize::Max => match node.get_int_max_value() {
                        Ok(val) => Ok(val),
                        Err(err) => Err(StrErr(format!("{}.set_stream_channel_packet_size | Get Max Error: {}", dbg, err))),
                    }
                    ChannelPacketSize::Val(val) => Ok(val),
                };
                log::debug!("{}.set_packet_size | Prev ChannelPacketSize: {}", dbg, node.get_int_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")));
                match val {
                    Ok(val) => match node.set_int_value(val) {
                        Ok(_) => {
                            if let Ok(val) = node.get_int_value() {
                                log::debug!("{}.set_packet_size | ChannelPacketSize changed to: {}", dbg, val);
                            }
                            Ok(())
                        }
                        Err(err) => Err(StrErr(format!("{}.set_packet_size | Set ChannelPacketSize {} Error: {}", dbg, self.conf.exposure.time, err))),
                    },
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(StrErr(format!("{}.set_packet_size | Get ChannelPacketSize Node Error: {}", dbg, err))),
        }
    }
    ///
    /// Image acquisition
    /// (1) sets acquisition mode
    /// (2) sets buffer handling mode
    /// (3) set transport stream protocol to TCP
    /// (4) starts the stream
    /// (5) gets a number of images
    /// (6) prints information from images
    /// (7) requeues buffers
    /// (8) stops the stream
    fn read(&self, on_event: impl Fn(AcImage)) -> Result<(), StrErr> {
        let dbg = self.name.join();
        let exit = self.exit.clone();
        log::debug!("{}.stream | Get node map...", dbg);
        match self.node() {
            Ok(node_map) => {
                log::debug!("{}.stream | Get node map - Ok", dbg);
                log::debug!("{}.stream | Pixel format prev: {}", dbg, node_map.get_enumeration_value("PixelFormat").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                match node_map.set_enumeration_value("PixelFormat", &self.conf.pixel_format.format()) {
                    Ok(_) => log::debug!("{}.stream | PixelFormat changed to: {}", dbg, self.conf.pixel_format.format()),
                    Err(err) => log::warn!("{}.stream | Set PixelFormat Error: {}", dbg, err),
                };
                log::debug!("{}.stream | Pixel format changed to: {}", dbg, node_map.get_enumeration_value("PixelFormat").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                if let Err(err) = self.set_stream_channel_packet_size(&node_map, self.conf.channel_packet_size) {
                    log::warn!("{}.stream | Error: {}", dbg, err);
                }
                if let Err(err) = self.set_exposure(&node_map, self.conf.exposure) {
                    log::warn!("{}.stream | Error: {}", dbg, err)
                } 
                if let Err(err) = self.set_frame_rate(&node_map, self.conf.fps) {
                    log::warn!("{}.stream | Error: {}", dbg, err)
                }
                match self.tls_stream_node() {
                    Ok(node) => {
                        if let Err(err) = node.set_bool_value("StreamAutoNegotiatePacketSize", self.conf.auto_packet_size){
                            log::warn!("{}.stream | Set StreamAutoNegotiatePacketSize Error: {}", dbg, err);
                        }
                        if let Err(err) = node.set_bool_value("StreamPacketResendEnable", self.conf.resend_packet){
                            log::warn!("{}.stream | Set StreamPacketResendEnable Error: {}", dbg, err);
                        }
                        if let Err(err) = node.set_value("StreamBufferHandlingMode", "NewestOnly"){
                            log::warn!("{}.stream | Set StreamBufferHandlingMode set 'NewestOnly' Error: {}", dbg, err);
                        }
                    }
                    Err(err) => log::warn!("{}.stream | Get TLS Node Error: {}", dbg, err)
                }
                let node_name = "AcquisitionMode";
                match node_map.get_value(node_name) {
                    Ok(initial_acquisition_mode) => {
                        log::debug!("{}.stream | Set acquisition mode to 'Continuous'...", dbg);
                        match node_map.set_value(node_name, "Continuous") {
                            Ok(_) => {
                                let result = match node_map.get_access_mode("TransportStreamProtocol") {
                                    Ok(transport_stream_protocol_access_mode) => match transport_stream_protocol_access_mode {
                                        AcAccessMode::NotImplemented => Err(StrErr(format!("{}.stream | Access denied, Mode: {}", dbg, transport_stream_protocol_access_mode))),
                                        AcAccessMode::Undefined(_) => Err(StrErr(format!("{}.stream | Access is undefined: {}", dbg, transport_stream_protocol_access_mode))),
                                        _ => {
                                            log::debug!("{}.stream | Start stream", dbg);
                                            let err = AcErr::from(unsafe { acDeviceStartStream(self.device) });
                                            match err {
                                                AcErr::Success => {
                                                    log::debug!("{}.stream | Retriving images...", dbg);
                                                    loop {
                                                        log::trace!("{}.stream | Read image...", dbg);
                                                        match self.get_buffer() {
                                                            Ok(buffer) => {
                                                                match buffer.get_image() {
                                                                    Ok(img) => {
                                                                        (on_event)(img)
                                                                    }
                                                                    Err(err) => log::warn!("{}.stream | Error: {}", dbg, err),
                                                                }
                                                            }
                                                            Err(err) => {
                                                                log::warn!("{}.stream | Error: {}", dbg, err);
                                                            }
                                                        };
                                                        if exit.load(Ordering::SeqCst) {
                                                            break;
                                                        }
                                                    }
                                                    // stop stream
                                                    log::debug!("{}.stream | Stop stream...", dbg);
                                                    let err = AcErr::from(unsafe { acDeviceStopStream(self.device) });
                                                    if err != AcErr::Success {
                                                        return Err(StrErr(format!("{}.stream | DeviceStopStream Error: {}", dbg, err)));
                                                    }
                                                    Ok(())
                                                    // return node to its initial values
                                                    // self.set_node_value(node_map, "TransportStreamProtocol", &p_transport_stream_protocol_initial)?;
                                                }
                                                _ => Err(StrErr(format!("{}.stream | DeviceStartStream Error: {}", dbg, err))),
                                            }
                                        }
                                    }
                                    Err(err) => Err(StrErr(format!("{}.stream | Get TransportStreamProtocol access mode Error: {}", dbg, err))),
                                };
                                if let Err(err) = node_map.set_value("AcquisitionMode", &initial_acquisition_mode) {
                                    log::debug!("{}.stream | Error return mode back: {}", dbg, err);
                                }
                                result
                            },
                            Err(err) => {
                                if let Err(err) = node_map.set_value("AcquisitionMode", &initial_acquisition_mode) {
                                    log::debug!("{}.stream | Error return mode back: {}", dbg, err);
                                }
                                Err(StrErr(format!("{}.stream | Set acquisition mode to 'Continuous' Error: {}", dbg, err)))
                            }
                        }
                    },
                    Err(err) => Err(StrErr(format!("{}.stream | Get `initial_acquisition_mode` Error: {}", dbg, err))),
                }
                // match self.tls_stream_node() {
                //     Err(err) => return Err(StrErr(format!("{}.stream | GetTLStreamNodeMap Error: {}", dbg, err))),
                //     Ok(h_tlstream_node_map) => {
                //     }
                // }
            },
            Err(err) => Err(StrErr(format!("{}.stream | Get node map - Error: {}", dbg, err))),
        }
    }
    ///
    /// Returns `Exit` signal, write true to stop reading.
    pub fn exit(&self) -> Arc<AtomicBool> {
        self.exit.clone()
    }

}
//
//
impl Drop for AcDevice {
    fn drop(&mut self) {
        unsafe {
            let err = AcErr::from(acSystemDestroyDevice(self.system, self.device));
            match err {
                AcErr::Success => {}
                AcErr::InvalidHandle(_) => if log::max_level() > log::LevelFilter::Trace {
                    log::warn!("{}.drop | Warn: {}", self.name, err);
                }
                _ => log::error!("{}.drop | Error: {}", self.name, err),
            }
        }
    }
}

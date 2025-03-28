use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use sal_core::error::Error;
use sal_sync::services::entity::name::Name;
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
    pub fn listen(&mut self, on_event: impl Fn(AcImage)) -> Result<(), Error> {
        log::debug!("{}.listen | Started", self.name);
        unsafe {
            let err = AcErr::from(acSystemCreateDevice(self.system, self.index, &mut self.device));
            match err {
                AcErr::Success => self.read(on_event),
                _ => {
                    Err(Error::new(&self.name, "listen").pass_with("CreateDevice Error", err.to_string()))
                }
            }
        }
    }
    ///
    /// Returns the already initialized DeviceNodeMap (`acNodeMap`), used to access a device's complete feature set of nodes (acNode).
    pub fn node(&self) -> Result<AcNodeMap, Error> {
        unsafe {
            let mut h_node_map: acNodeMap = std::ptr::null_mut();
            let err = AcErr::from(acDeviceGetNodeMap(self.device, &mut h_node_map));
            match err {
                AcErr::Success => Ok(AcNodeMap::new(&self.name, self.device, h_node_map, "DeviceNodeMap")),
                _ => Err(Error::new(&self.name, "node").err(err)),
            }
        }
    }
    ///
    /// Returns the already initialized TLStreamNodeMap node map (`acNodeMap`), used to access a device's complete feature set of nodes (acNode).
    pub fn tls_stream_node(&self) -> Result<AcNodeMap, Error> {
        unsafe {
            let mut h_tlstream_node_map: acNodeMap = std::ptr::null_mut();
            let err = AcErr::from(acDeviceGetTLStreamNodeMap(self.device, &mut h_tlstream_node_map));
            match err {
                AcErr::Success => Ok(AcNodeMap::new(&self.name, self.device, h_tlstream_node_map, "TLStreamNodeMap")),
                _ => Err(Error::new(&self.name, "tls_stream_node").err(err)),
            }
        }
    }
    ///
    /// Returns Device buffer
    fn get_buffer(&self) -> Result<AcBuffer, Error> {
        let mut buffer: acBuffer = std::ptr::null_mut();
        let err = AcErr::from(unsafe { acDeviceGetBuffer(self.device, self.image_timeout, &mut buffer) });
        match err {
            AcErr::Success => Ok(AcBuffer::new(&self.name, self.device, buffer, self.conf.pixel_format)),
                _ => Err(Error::new(&self.name, "get_buffer").err(err)),
        }
    }
    ///
    /// Set acquisition frame rate, FPS
    fn set_frame_rate(&self, node_map: &AcNodeMap, value: FrameRate) -> Result<(), Error> {
        let dbg = self.name.join();
        let error = Error::new(&dbg, "set_frame_rate");
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
                            Err(err) => Err(error.pass_with("Get Min Error", err)),
                        }
                        FrameRate::Max => match node.get_float_max_value() {
                            Ok(val) => Ok(val),
                            Err(err) => Err(error.pass_with("Get Max Error", err)),
                        },
                        FrameRate::Val(val) => Ok(val)
                    };
                    match val {
                        Ok(val) => match node.set_float_value(val) {
                            Ok(_) => {
                                log::debug!("{}.set_frame_rate | AcquisitionFrameRate changed to {:?} ({:.3} FPS)", dbg, value, val);
                                Ok(())
                            }
                            Err(err) => Err(error.pass_with(format!("AcquisitionFrameRate change to {:?}", value), err)),
                        }
                        Err(err) => Err(error.pass(err)),
                    }
                }
                Err(err) => Err(error.pass(err)),
            }
            Err(err) => Err(error.pass_with(format!("AcquisitionFrameRateEnable change to {:?}", acq_fr_en), err)),
        }
    }
    ///
    /// Set Exposure time
    fn set_exposure(&self, node_map: &AcNodeMap, exposure: Exposure) -> Result<(), Error> {
        let dbg = self.name.join();
        let error = Error::new(&dbg, "set_exposure");
        match node_map.set_enum_value("ExposureAuto", exposure.auto.as_str()) {
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
                            Err(err) => Err(error.pass_with(format!("Set Exposure {} us", exposure.time), err)),
                        }
                    } else {
                        Err(error.err(format!("Set Exposure {} us Error, ExposureTime Node - is not writable", exposure.time)))
                    }
                },
                Err(err) => Err(error.pass_with("Get ExposureTime Node", err)),
            }
            ExposureAuto::Continuous => Ok(()),
        }
    }
    ///
    /// Set DeviceStream Channel Packet Size
    fn set_stream_channel_packet_size(&self, node_map: &AcNodeMap, size: ChannelPacketSize) -> Result<(), Error> {
        let dbg = self.name.join();
        let error = Error::new(&dbg, "set_stream_channel_packet_size");
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
                        Err(err) => Err(error.pass_with("Get Min Error", err)),
                    }
                    ChannelPacketSize::Max => match node.get_int_max_value() {
                        Ok(val) => Ok(val),
                        Err(err) => Err(error.pass_with("Get Max Error", err)),
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
                        Err(err) => Err(error.pass_with(format!("Set ChannelPacketSize {}", val), err)),
                    },
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(error.pass_with("Get ChannelPacketSize Node", err)),
        }
    }
    ///
    /// Resets device to factory defaults.
    fn factory_reset(&self, node_map: &AcNodeMap) -> Result<(), Error> {
        let dbg = self.name.join();
        let error = Error::new(&dbg, "factory_reset");
        match node_map.get_node("DeviceFactoryReset") {
            Ok(node) => {
                log::debug!(
                    "{}.factory_reset | FactoryReset: \nis_writable: {} \n bool: {} \nint: {} \nfloat: {} \nstr: {}", dbg,
                    node.is_writable(),
                    node.get_bool_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                    node.get_int_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                    node.get_float_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                    node.get_str_value().map_or_else(|err| format!("{err}"), |v| format!("{v}")),
                );
                Ok(())
            }
            Err(err) => Err(error.pass_with("Get ChannelPacketSize Node", err)),
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
    fn read(&self, on_event: impl Fn(AcImage)) -> Result<(), Error> {
        let dbg = self.name.join();
        let error = Error::new(&dbg, "read");
        let exit = self.exit.clone();
        log::debug!("{}.read | Get node map...", dbg);
        match self.node() {
            Ok(node_map) => {
                log::debug!("{}.read | Get node map - Ok", dbg);
                // log::debug!("{}.read | DeviceFactoryReset: {}", dbg, node_map.set_value("DeviceFactoryReset", "").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                // log::debug!("{}.read | DeviceFactoryReset: {}", dbg, node_map.get_enum_value("DeviceFactoryReset").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                // if let Err(err) = self.factory_reset(&node_map) {
                //     log::warn!("{}.read | Error: {}", dbg, err)
                // }
                log::debug!("{}.read | Pixel format prev: {}", dbg, node_map.get_enum_value("PixelFormat").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                match node_map.set_enum_value("PixelFormat", &self.conf.pixel_format.format()) {
                    Ok(_) => log::debug!("{}.read | PixelFormat changed to: {}", dbg, self.conf.pixel_format.format()),
                    Err(err) => log::warn!("{}.read | Set PixelFormat Error: {}", dbg, err),
                };
                log::debug!("{}.read | Pixel format changed to: {}", dbg, node_map.get_enum_value("PixelFormat").map_or_else(|err| format!("{err}"), |v| format!("{v}")) );
                if let Err(err) = self.set_stream_channel_packet_size(&node_map, self.conf.channel_packet_size) {
                    log::warn!("{}.read | Error: {}", dbg, err);
                }
                if let Err(err) = self.set_exposure(&node_map, self.conf.exposure) {
                    log::warn!("{}.read | Error: {}", dbg, err)
                } 
                if let Err(err) = self.set_frame_rate(&node_map, self.conf.fps) {
                    log::warn!("{}.read | Error: {}", dbg, err)
                }
                match self.tls_stream_node() {
                    Ok(node) => {
                        if let Err(err) = node.set_bool_value("StreamAutoNegotiatePacketSize", self.conf.auto_packet_size){
                            log::warn!("{}.read | Set StreamAutoNegotiatePacketSize Error: {}", dbg, err);
                        }
                        if let Err(err) = node.set_bool_value("StreamPacketResendEnable", self.conf.resend_packet){
                            log::warn!("{}.read | Set StreamPacketResendEnable Error: {}", dbg, err);
                        }
                        if let Err(err) = node.set_value("StreamBufferHandlingMode", "NewestOnly"){
                            log::warn!("{}.read | Set StreamBufferHandlingMode set 'NewestOnly' Error: {}", dbg, err);
                        }
                    }
                    Err(err) => log::warn!("{}.read | Get TLS Node Error: {}", dbg, err)
                }
                let node_name = "AcquisitionMode";
                match node_map.get_value(node_name) {
                    Ok(initial_acquisition_mode) => {
                        log::debug!("{}.read | Set acquisition mode to 'Continuous'...", dbg);
                        match node_map.set_value(node_name, "Continuous") {
                            Ok(_) => {
                                let result = match node_map.get_access_mode("TransportStreamProtocol") {
                                    Ok(transport_stream_protocol_access_mode) => match transport_stream_protocol_access_mode {
                                        AcAccessMode::NotImplemented => Err(error.err(format!("Access denied, Mode: {}", transport_stream_protocol_access_mode))),
                                        AcAccessMode::Undefined(_) => Err(error.err(format!("Access is undefined: {}", transport_stream_protocol_access_mode))),
                                        _ => {
                                            log::debug!("{}.read | Start stream", dbg);
                                            let err = AcErr::from(unsafe { acDeviceStartStream(self.device) });
                                            match err {
                                                AcErr::Success => {
                                                    log::debug!("{}.read | Retriving images...", dbg);
                                                    loop {
                                                        log::trace!("{}.read | Read image...", dbg);
                                                        match self.get_buffer() {
                                                            Ok(buffer) => {
                                                                match buffer.get_image() {
                                                                    Ok(img) => {
                                                                        (on_event)(img)
                                                                    }
                                                                    Err(err) => log::warn!("{}.read | Error: {}", dbg, err),
                                                                }
                                                            }
                                                            Err(err) => {
                                                                log::warn!("{}.read | Error: {}", dbg, err);
                                                            }
                                                        };
                                                        if exit.load(Ordering::SeqCst) {
                                                            break;
                                                        }
                                                    }
                                                    // stop stream
                                                    log::debug!("{}.read | Stop stream...", dbg);
                                                    let err = AcErr::from(unsafe { acDeviceStopStream(self.device) });
                                                    if err != AcErr::Success {
                                                        return Err(error.pass_with("DeviceStopStream Error", err.to_string()));
                                                    }
                                                    Ok(())
                                                    // return node to its initial values
                                                    // self.set_node_value(node_map, "TransportStreamProtocol", &p_transport_stream_protocol_initial)?;
                                                }
                                                _ => Err(error.pass_with("DeviceStartStream Error", err.to_string())),
                                            }
                                        }
                                    }
                                    Err(err) => Err(error.pass_with("Get TransportStreamProtocol access mode Error", err)),
                                };
                                if let Err(err) = node_map.set_value("AcquisitionMode", &initial_acquisition_mode) {
                                    log::debug!("{}.read | Error return mode back: {}", dbg, err);
                                }
                                result
                            },
                            Err(err) => {
                                if let Err(err) = node_map.set_value("AcquisitionMode", &initial_acquisition_mode) {
                                    log::debug!("{}.read | Error return mode back: {}", dbg, err);
                                }
                                Err(error.pass_with("Set acquisition mode to 'Continuous' Error",err))
                            }
                        }
                    },
                    Err(err) => Err(error.pass_with("Get `initial_acquisition_mode` Error", err)),
                }
                // match self.tls_stream_node() {
                //     Err(err) => return Err(Error(format!("{}.read | GetTLStreamNodeMap Error: {}", dbg, err))),
                //     Ok(h_tlstream_node_map) => {
                //     }
                // }
            },
            Err(err) => Err(error.pass_with("Get node map error", err)),
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

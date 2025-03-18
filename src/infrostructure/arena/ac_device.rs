use std::sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc, RwLock};
use sal_sync::services::entity::{error::str_err::StrErr, name::Name};
use crate::infrostructure::arena::{ac_access_mode::AcAccessMode, bindings::{
    acBuffer, acDeviceGetBuffer, acDeviceGetTLStreamNodeMap, acDeviceStartStream, acDeviceStopStream,
}};

use super::{
    ac_buffer::AcBuffer, ac_err::AcErr, ac_image::AcImage, ac_node::AcNodeMap, bindings::{
        acDevice, acNodeMap, acSystem, acDeviceGetNodeMap, acSystemCreateDevice, acSystemDestroyDevice
    }, pixel_format::PixelFormat
};

///
/// Represents a Arena SDK device, used to configure and stream a device.
pub struct AcDevice {
    name: Name,
    index: usize,
    device: acDevice,
    system: acSystem,
    pixel_format: PixelFormat,
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
    pub fn new(parent: impl Into<String>, system: acSystem, index: usize, pixel_format: PixelFormat, exit: Option<Arc<AtomicBool>>) -> Self {
        let name = Name::new(parent.into(), format!("AcDevice({index})"));
        Self {
            name,
            index,
            device: std::ptr::null_mut(),
            system,
            pixel_format,
            image_timeout: 2000,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// 
    pub fn listen(&mut self, on_event: impl Fn(AcImage)) -> Result<(), StrErr> {
        unsafe {
            let err = AcErr::from(acSystemCreateDevice(self.system, self.index, &mut self.device));
            match err {
                AcErr::Success => self.read(on_event),
                _ => {
                    Err(StrErr(format!("{}.stream | Error: {}", self.name, err)))
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
            AcErr::Success => Ok(AcBuffer::new(&self.name, self.device, buffer, self.pixel_format)),
            _ => Err(StrErr(format!("{}.buffer | Error: {}", self.name, err))),
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
                match self.tls_stream_node() {
                    Err(err) => return Err(StrErr(format!("{}.stream | GetTLStreamNodeMap Error: {}", dbg, err))),
                    Ok(h_tlstream_node_map) => {
                        log::debug!("{}.stream | Get node map - Ok", dbg);
                        match node_map.set_enumeration_value("PixelFormat", &self.pixel_format.format()) {
                            Ok(_) => log::debug!("{}.stream | PixelFormat: {}", dbg, self.pixel_format.format()),
                            Err(err) => log::warn!("{}.stream | Set PixelFormat Error: {}", dbg, err),
                        };
                        let node_name = "AcquisitionMode";
                        match node_map.get_value(node_name) {
                            Ok(initial_acquisition_mode) => {
                                // set acquisition mode
                                log::debug!("{}.stream | Set acquisition mode to 'Continuous'...", dbg);
                                match node_map.set_value(node_name, "Continuous") {
                                    Ok(_) => {
                                        // set buffer handling mode
                                        log::debug!("{}.stream | Set buffer handling mode to 'NewestOnly'...", dbg);
                                        if let Err(err) = h_tlstream_node_map.set_value("StreamBufferHandlingMode", "NewestOnly"){
                                            log::warn!("{}.stream | StreamBufferHandlingMode set 'NewestOnly' Error: {}", dbg, err);
                                        }
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
                    }
                }
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

use std::{ffi::CString, str::FromStr};
use sal_sync::services::entity::{error::str_err::StrErr, name::Name};
use crate::infrostructure::arena::bindings::{
    acBuffer, acNode,
    acBufferGetSizeFilled, acDeviceGetBuffer, acDeviceGetTLStreamNodeMap, acDeviceRequeueBuffer, acDeviceStartStream,
    acDeviceStopStream, acImageGetData, acImageGetHeight, acImageGetTimestampNs, acImageGetWidth,
    AC_ACCESS_MODE_NI,
};

use super::{
    ac_buffer::AcBuffer, ac_err::AcErr, ac_node::AcNodeMap, bindings::{
        acDevice, acDeviceGetNodeMap, acNodeMap, acNodeMapGetNodeAndAccessMode, acSystem, acSystemCreateDevice, acSystemDestroyDevice
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
    image_timeout: u64
}
//
//
impl AcDevice {
    ///
    /// 
    pub fn new(parent: impl Into<String>, system: acSystem, index: usize, pixel_format: PixelFormat) -> Self {
        let name = Name::new(parent.into(), format!("AcDevice({index})"));
        Self {
            name,
            index,
            device: std::ptr::null_mut(),
            system,
            pixel_format,
            image_timeout: 2000,
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> Result<(), StrErr> {
        unsafe {
            let err = AcErr::from(acSystemCreateDevice(self.system, self.index, &mut self.device));
            match err {
                AcErr::Success => {
                    match self.acquire_images(100000) {
                        Ok(_) => {
                            log::debug!("{}.run | Image received", self.name);

                        }
                        Err(err) => {
                            log::debug!("{}.run | Image receiv Error: {}", self.name, err);
                        }
                    }
                    Ok(())
                }
                _ => {
                    Err(StrErr(format!("{}.run | Error: {}", self.name, err)))
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
    fn buffer(&self) -> Result<AcBuffer, StrErr> {
        let mut buffer: acBuffer = std::ptr::null_mut();
        let err = AcErr::from(unsafe { acDeviceGetBuffer(self.device, self.image_timeout, &mut buffer) });
        match err {
            AcErr::Success => Ok(AcBuffer::new(&self.name, self.device, buffer, self.pixel_format)),
            _ => Err(StrErr(format!("{}.buffer | Error: {}", self.name, err))),
        }
    }
    ///
    /// demonstrates acquisition
    /// (1) sets acquisition mode
    /// (2) sets buffer handling mode
    /// (3) set transport stream protocol to TCP
    /// (4) starts the stream
    /// (5) gets a number of images
    /// (6) prints information from images
    /// (7) requeues buffers
    /// (8) stops the stream
    fn acquire_images(&self, images: usize) -> Result<(), StrErr> {
        unsafe {
            log::debug!("{}.acquire_images | Get node map...", self.name);
            match self.node() {
                Ok(node_map) => {
                    log::debug!("{}.acquire_images | Get node map - Ok", self.name);
                    match node_map.set_enumeration_value("PixelFormat", &self.pixel_format.format()) {
                        Ok(_) => log::debug!("{}.acquire_images | PixelFormat: {}", self.name, self.pixel_format.format()),
                        Err(err) => return Err(StrErr(format!("{}.acquire_images | Set PixelFormat Error: {}", self.name, err))),
                    };
                    let node_name = "AcquisitionMode";
                    match node_map.get_value(node_name) {
                        Ok(initial_acquisition_mode) => {
                            // set acquisition mode
                            log::debug!("{}.acquire_images | Set acquisition mode to 'Continuous'...", self.name);
                            match node_map.set_value(node_name, "Continuous") {
                                Ok(_) => {
                                    // get TL Stream node map
                                    let h_tlstream_node_map = match self.tls_stream_node() {
                                        Err(err) => return Err(StrErr(format!("{}.acquire_images | GetTLStreamNodeMap Error: {}", self.name, err))),
                                        Ok(node) => node,
                                    };
                                    // let err = AcErr::from(acDeviceGetTLStreamNodeMap(self.device, &mut h_tlstream_node_map.map));
                                    // if err != AcErr::Success {
                                    //     return Err(StrErr(format!("{}.acquire_images | GetTLStreamNodeMap Error: {}", self.name, err)));
                                    // };
                                    // set buffer handling mode
                                    log::debug!("{}.acquire_images | Set buffer handling mode to 'NewestOnly'...", self.name);
                                    h_tlstream_node_map.set_value("StreamBufferHandlingMode", "NewestOnly")?;
                                    // self.set_node_value(h_tlstream_node_map, "StreamBufferHandlingMode", "NewestOnly")?;
                                    // The TransportStreamProtocol node can tell the camera to use the TCP datastream engine. When
                                    //    set to TCP - Arena will switch to using the TCP datastream engine. 
                                    //    There is no further necessary configuration, though to achieve maximum throughput 
                                    //    users may want to set the "DeviceLinkThroughputReserve" to 0 and 
                                    //    also set the stream channel packet delay "GevSCPD" to 0.
                                    let mut h_transport_stream_protocol_node: acNode = std::ptr::null_mut();
                                    let mut access_mode_transport_stream_protocol = 0;
                                    let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                                        node_map.map,
                                        CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                                        &mut h_transport_stream_protocol_node,
                                        &mut access_mode_transport_stream_protocol,
                                    ));
                                    if err != AcErr::Success {
                                        return Err(StrErr(format!("{}.acquire_images | GetNodeAndAccessMode Error: {}", self.name, err)));
                                    };
                                    if access_mode_transport_stream_protocol != AC_ACCESS_MODE_NI {
                                        // get node value
                                        // let p_transport_stream_protocol_initial = node_map.get_value("TransportStreamProtocol")?;
                                        // log::debug!("{}.acquire_images | Set Transport Stream Protocol to TCP", self.name);
                                        // let err = AcErr::from(acNodeMapSetEnumerationValue(
                                        //     node_map,
                                        //     CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                                        //     CString::from_str("TCP").unwrap().as_ptr(),
                                        // ));
                                        // if err != AcErr::Success {
                                        //     return Err(StrErr(format!("{}.acquire_images | Set Transport Stream Protocol to TCP Error: {}", self.name, err)));
                                        // };
                                        // start stream
                                        log::debug!("{}.acquire_images | Start stream", self.name);
                                        let err = AcErr::from(acDeviceStartStream(self.device));
                                        if err != AcErr::Success {
                                            return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                        };
                                        let window = "Retrived";
                                        if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
                                            return Err(StrErr(format!("{}.acquire_images | Create Window Error: {}", self.name, err)));
                                        }
                                        // let img = opencv::imgcodecs::imread("/home/lobanov/Pictures/Sub-Issue-Bind.png", opencv::imgcodecs::IMREAD_COLOR).unwrap();
                                        // if let Err(err) = opencv::highgui::imshow(window, &img) {
                                        //     log::warn!("{}.acquire_images | Display img error: {:?}", self.name, err);
                                        // }
                                        opencv::highgui::wait_key(10).unwrap();
                                        // let mut cam = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap(); // 0 is the default camera
                                        // if ! cam.is_opened().unwrap() {
                                        //     log::warn!("{}.acquire_images | Cam isn't opened", self.name);
                                        // }
                                        // get images
                                        log::debug!("{}.acquire_images | Getting {} images", self.name, images);
                                        for i in 0..images {
                                            // get image
                                            log::debug!("{}.acquire_images | Getting {} image...", self.name, i);
                                            match self.buffer() {
                                                Ok(buffer) => {
                                                    match buffer.get_image() {
                                                        Ok(img) => {
                                                            if let Err(err) = opencv::highgui::imshow(window, &img.mat) {
                                                                log::warn!("{}.acquire_images | Display img error: {:?}", self.name, err);
                                                            };
                                                            opencv::highgui::wait_key(1).unwrap();
                                                        }
                                                        Err(_) => todo!(),
                                                    }
                                                }
                                                Err(err) => {
                                                    log::warn!("{}.acquire_images | Get buffer error: {}", self.name, err);
                                                    return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                                }
                                            };
                                            // // get image information
                                            // log::debug!(" (");
                                            // // get and display size filled
                                            // let mut size_filled = 0;
                                            // let err = AcErr::from(acBufferGetSizeFilled(h_buffer, &mut size_filled));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                            // log::debug!("{} bytes; ", size_filled);
                                            // // get and display width
                                            // let mut width = 0;
                                            // let err = AcErr::from(acImageGetWidth(h_buffer, &mut width));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                            // log::debug!("{}", width);
                                            // // get and display height
                                            // let mut height = 0;
                                            // let err = AcErr::from(acImageGetHeight(h_buffer, &mut height));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                            // log::debug!("{}; ", height);
                                            // // get and display timestamp
                                            // let mut timestamp_ns = 0;
                                            // let err = AcErr::from(acImageGetTimestampNs(h_buffer, &mut timestamp_ns));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                            // log::debug!("{}.acquire_images | timestamp (ns): {})", self.name, timestamp_ns);
                                            // let mut buf = Vec::with_capacity(size_filled);
                                            // let mut p_input  = buf.as_mut_ptr();
                                            // let err = AcErr::from(acImageGetData(h_buffer, &mut p_input));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                            // let mat = opencv::core::Mat::new_rows_cols_with_data_unsafe(
                                            //     height as i32,
                                            //     width as i32,
                                            //     self.pixel_format.cv_format(),
                                            //     p_input as *mut std::ffi::c_void,
                                            //     opencv::core::Mat_AUTO_STEP,
                                            // ).unwrap();
                                            // let mut mat = opencv::core::Mat::default();
                                            // if let Err(err) = cam.read(&mut mat) {
                                            //     log::warn!("{}.acquire_images | Cam read error: {:?}", self.name, err);
                                            // }
                                            // if let Err(err) = opencv::highgui::imshow(window, &mat) {
                                            //     log::warn!("{}.acquire_images | Display img error: {:?}", self.name, err);
                                            // };
                                            // opencv::highgui::wait_key(1).unwrap();
                                            // requeue image buffer
                                            // log::debug!("{}.acquire_images | and requeue", self.name);
                                            // let err = AcErr::from(acDeviceRequeueBuffer(self.device, h_buffer));
                                            // if err != AcErr::Success {
                                            //     return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            // };
                                        }
                                        // stop stream
                                        log::debug!("{}.acquire_images | Stop stream", self.name);
                                        let err = AcErr::from(acDeviceStopStream(self.device));
                                        match err {
                                            AcErr::Success => (),
                                            _ => return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err))),
                                        };
                                        // return node to its initial values
                                        // self.set_node_value(node_map, "TransportStreamProtocol", &p_transport_stream_protocol_initial)?;
                                    } else {
                                        log::warn!("{}.acquire_images | Connected camera does not support TCP stream", self.name);
                                    }
                                    // return node to its initial values
                                    node_map.set_value("AcquisitionMode", &initial_acquisition_mode)?;
                                    Ok(())
                                },
                                Err(err) => {
                                    if let Err(err) = node_map.set_value("AcquisitionMode", &initial_acquisition_mode) {
                                        log::debug!("{}.acquire_images | Error return mode back: {}", self.name, err);
                                    }
                                    Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)))
                                }
                            }
                        },
                        Err(err) => Err(StrErr(format!("{}.acquire_images | Get `initial_acquisition_mode` Error: {}", self.name, err))),
                    }
                },
                Err(err) => Err(StrErr(format!("{}.acquire_images | Get node map - Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// Cleans up the system (acSystem) and deinitializes the Arena SDK, deallocating all memory.
    pub fn close(&self) -> Result<(), StrErr> {
        unsafe {
            let err = AcErr::from(acSystemDestroyDevice(self.system, self.device));
            match err {
                AcErr::Success => Ok(()),
                _ => Err(StrErr(format!("{}.close | Error: {}", self.name, err))),
            }
        }
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

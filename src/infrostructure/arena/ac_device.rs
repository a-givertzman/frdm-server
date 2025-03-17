use std::{ffi::CString, str::FromStr};

use opencv::{core::MatTrait, traits::Boxed};
use sal_sync::services::entity::{error::str_err::StrErr, name::Name};

use crate::infrostructure::arena::bindings::{acBuffer, acBufferGetSizeFilled, acDeviceGetBuffer, acDeviceGetTLStreamNodeMap, acDeviceRequeueBuffer, acDeviceStartStream, acDeviceStopStream, acImageGetHeight, acImageGetTimestampNs, acImageGetWidth, acNode, acNodeMapSetEnumerationValue, AC_ACCESS_MODE_NI};

use super::{
    ac_access_mode::AcAccessMode, ac_err::AcErr,
    bindings::{
        acDevice, acDeviceGetNodeMap, acNodeMap, acNodeMapGetNodeAndAccessMode, acSystem, acSystemCreateDevice, acSystemDestroyDevice, acValueFromString, acValueToString
    }, ffi_str::FfiStr
};

///
/// Represents a Arena SDK device, used to configure and stream a device.
pub struct AcDevice {
    name: Name,
    index: usize,
    device: acDevice,
    system: acSystem,
    // Maximum time to wait for an image buffer
    image_timeout: u64
}
//
//
impl AcDevice {
    ///
    /// 
    pub fn new(parent: impl Into<String>, system: acSystem, index: usize) -> Self {
        let name = Name::new(parent.into(), format!("AcDevice({index})"));
        Self {
            name,
            index,
            device: std::ptr::null_mut(),
            system,
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
                    match self.acquire_images(100) {
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
    /// Returns the already initialized main node map (acNodeMap), used to access a device's complete feature set of nodes (acNode).
    /// - `device` - A device
    pub fn node_map(&self, device: acDevice) -> Result<acNodeMap, StrErr> {
        unsafe {
            let mut h_node_map: acNodeMap = std::ptr::null_mut();
            let err = AcErr::from(acDeviceGetNodeMap(device, &mut h_node_map));
            match err {
                AcErr::Success => Ok(h_node_map),
                _ => Err(StrErr(format!("{}.node_map | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// gets node value
    /// (1) gets node
    /// (2) checks access mode
    /// (3) gets value
    pub fn get_node_value(&self, h_node_map: acNodeMap, node_name: &str) -> Result<String, StrErr> {
        unsafe {
            // get node
            let mut h_node: super::bindings::acNode = std::ptr::null_mut();
            let mut access_mode = 0;
            let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                h_node_map,
                CString::new(node_name).unwrap().as_ptr(),
                &mut h_node,
                &mut access_mode,
            ));
            match err {
                AcErr::Success => {
                    let access_mode = AcAccessMode::from(access_mode);
                    match access_mode {
                        AcAccessMode::ReadWrite | AcAccessMode::ReadOnly => {
                            let mut result = FfiStr::<1024>::new();
                            let err = AcErr::from(acValueToString(h_node, result.as_mut_ptr(), &mut result.len));
                            match err {
                                AcErr::Success => Ok(result.to_string()),
                                _ => Err(StrErr(format!("{}.get_node_value | ValueToString Error: {}", self.name, err))),
                            }
                        },
                        _ => Err(StrErr(format!("{}.get_node_value | Access denied, current mode is '{}'", self.name, access_mode))),
                    }
                },
                _ => Err(StrErr(format!("{}.get_node_value | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// sets node value
    /// (1) gets node
    /// (2) checks access mode
    /// (3) gets value
    pub fn set_node_value(&self, h_node_map: acNodeMap, node_name: &str, value: &str) -> Result<(), StrErr>{
        unsafe {
            // get node
            let mut h_node: super::bindings::acNode = std::ptr::null_mut();
            let mut access_mode = 0;
            let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                h_node_map,
                CString::new(node_name).unwrap().as_ptr(),
                &mut h_node,
                &mut access_mode,
            ));
            match err {
                AcErr::Success => {
                    let access_mode = AcAccessMode::from(access_mode);
                    match access_mode {
                        AcAccessMode::ReadWrite | AcAccessMode::WriteOnly => {
                            let err = AcErr::from(acValueFromString(h_node, CString::new(value).unwrap().as_ptr()));
                            match err {
                                AcErr::Success => Ok(()),
                                _ => Err(StrErr(format!("{}.set_node_value | ValueFromString Error: {}", self.name, err))),
                            }
                        },
                        _ => return Err(StrErr(format!("{}.set_node_value | Access denied, current mode is '{}'", self.name, access_mode))),
                    }
                },
                _ => Err(StrErr(format!("{}.set_node_value | Error: {}", self.name, err))),
            }
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
            match self.node_map(self.device) {
                Ok(node_map) => {
                    log::debug!("{}.acquire_images | Get node map - Ok", self.name);
                    // get node values that will be changed in order to return their values at
                    // the end of the example
                    let node_name = "AcquisitionMode";
                    match self.get_node_value(node_map, node_name) {
                        Ok(initial_acquisition_mode) => {
                            // set acquisition mode
                            log::debug!("{}.acquire_images | Set acquisition mode to 'Continuous'...", self.name);
                            match self.set_node_value(node_map, node_name, "Continuous") {
                                Ok(_) => {
                                    // set buffer handling mode
                                    log::debug!("{}.acquire_images | Set buffer handling mode to 'NewestOnly'...", self.name);
                                    // get stream node map
                                    let mut h_tlstream_node_map: acNodeMap = std::ptr::null_mut();
                                    let err = AcErr::from(acDeviceGetTLStreamNodeMap(self.device, &mut h_tlstream_node_map));
                                    if err != AcErr::Success {
                                        return Err(StrErr(format!("{}.acquire_images | GetTLStreamNodeMap Error: {}", self.name, err)));
                                    };
                                    self.set_node_value(h_tlstream_node_map, "StreamBufferHandlingMode", "NewestOnly")?;
                                    // self.set_node_value(h_tlstream_node_map, "StreamBufferHandlingMode", "NewestOnly")?;
                                    // The TransportStreamProtocol node can tell the camera to use the TCP datastream engine. When
                                    //    set to TCP - Arena will switch to using the TCP datastream engine. 
                                    //    There is no further necessary configuration, though to achieve maximum throughput 
                                    //    users may want to set the "DeviceLinkThroughputReserve" to 0 and 
                                    //    also set the stream channel packet delay "GevSCPD" to 0.
                                    let mut h_transport_stream_protocol_node: acNode = std::ptr::null_mut();
                                    let mut access_mode_transport_stream_protocol = 0;
                                    let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                                        node_map,
                                        CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                                        &mut h_transport_stream_protocol_node,
                                        &mut access_mode_transport_stream_protocol,
                                    ));
                                    if err != AcErr::Success {
                                        return Err(StrErr(format!("{}.acquire_images | GetNodeAndAccessMode Error: {}", self.name, err)));
                                    };
                                    if access_mode_transport_stream_protocol != AC_ACCESS_MODE_NI {
                                        // get node value
                                        let p_transport_stream_protocol_initial = self.get_node_value(
                                            node_map,
                                            "TransportStreamProtocol",
                                        )?;
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
                                        if let Err(err) = opencv::highgui::named_window(window, 1) {
                                            return Err(StrErr(format!("{}.acquire_images | Create Window Error: {}", self.name, err)));
                                        }
                                        let img = opencv::imgcodecs::imread("/home/lobanov/Pictures/Sub-Issue-Bind.png", opencv::imgcodecs::IMREAD_COLOR).unwrap();
                                        if let Err(err) = opencv::highgui::imshow(window, &img) {
                                            log::warn!("{}.acquire_images | Display img error: {:?}", self.name, err);
                                        }
                                        opencv::highgui::wait_key(0).unwrap();
                                        // get images
                                        log::debug!("{}.acquire_images | Getting {} images", self.name, images);
                                        for i in 0..images {
                                            // get image
                                            log::debug!("{}.acquire_images | Getting {} image...", self.name, i);
                                            let mut h_buffer: acBuffer = std::ptr::null_mut();
                                            let err = AcErr::from(acDeviceGetBuffer(self.device, self.image_timeout, &mut h_buffer));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
                                            // get image information
                                            log::debug!(" (");
                                            // get and display size filled
                                            let mut size_filled = 0;
                                            let err = AcErr::from(acBufferGetSizeFilled(h_buffer, &mut size_filled));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
                                            log::debug!("{} bytes; ", size_filled);
                                            // get and display width
                                            let mut width = 0;
                                            let err = AcErr::from(acImageGetWidth(h_buffer, &mut width));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
                                            log::debug!("{}", width);
                                            // get and display height
                                            let mut height = 0;
                                            let err = AcErr::from(acImageGetHeight(h_buffer, &mut height));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
                                            log::debug!("{}; ", height);
                                            // get and display timestamp
                                            let mut timestamp_ns = 0;
                                            let err = AcErr::from(acImageGetTimestampNs(h_buffer, &mut timestamp_ns));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
                                            const PRIu64: &str = "'l' 'u'";
                                            log::debug!("{}.acquire_images | timestamp (ns): {} {} )", self.name, timestamp_ns, PRIu64);
                                            let mut mat = opencv::core::Mat::new_rows_cols_with_data_unsafe(
                                                width as i32,
                                                height as i32,
                                                opencv::core::CV_8UC1,
                                                h_buffer,
                                                opencv::core::Mat_AUTO_STEP,
                                            ).unwrap();
                                            if let Err(err) = opencv::highgui::imshow(window, &img) {
                                                log::warn!("{}.acquire_images | Display img error: {:?}", self.name, err);
                                            };
                                            // requeue image buffer
                                            log::debug!("{}.acquire_images | and requeue", self.name);
                                            let err = AcErr::from(acDeviceRequeueBuffer(self.device, h_buffer));
                                            if err != AcErr::Success {
                                                return Err(StrErr(format!("{}.acquire_images | Error: {}", self.name, err)));
                                            };
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
                                    self.set_node_value(node_map, "AcquisitionMode", &initial_acquisition_mode)?;
                                    Ok(())
                                },
                                Err(err) => {
                                    if let Err(err) = self.set_node_value(node_map, "AcquisitionMode", &initial_acquisition_mode) {
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

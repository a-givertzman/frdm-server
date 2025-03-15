#[cfg(test)]

mod tests {
    use std::{ffi::CString, str::FromStr, sync::Once, time::{Duration, Instant}};
    use crate::infrostructure::arena::{bindings::{
        acBuffer, acBufferGetSizeFilled, acCloseSystem, acDevice, acDeviceGetBuffer, acDeviceGetNodeMap, acDeviceGetTLStreamNodeMap, acDeviceRequeueBuffer, acDeviceStartStream, acDeviceStopStream, acImageGetHeight, acImageGetTimestampNs, acImageGetWidth, acNode, acNodeMap, acNodeMapGetNodeAndAccessMode, acNodeMapSetEnumerationValue, acOpenSystem, acSystemCreateDevice, acSystemDestroyDevice, acSystemGetDeviceIpAddressStr, acSystemGetDeviceModel, acSystemGetDeviceSerial, acSystemGetNumDevices, acSystemUpdateDevices, acValueFromString, acValueToString, AC_ACCESS_MODE, AC_ACCESS_MODE_NI, AC_ACCESS_MODE_RO, AC_ACCESS_MODE_RW, AC_ACCESS_MODE_WO, AC_ERROR, AC_ERROR_LIST_AC_ERR_SUCCESS
    }, AC_ERR_ERROR, AC_ERR_SUCCESS};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::domain::dbg::dbgid::DbgId;
    ///
    ///
    static INIT: Once = Once::new();
    const MAX_BUF: usize = 1024;
    const IMAGE_TIMEOUT: u64 = 2000;
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing such functionality / behavior
    #[test]
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = DbgId::root("test");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        unsafe {
            let mut h_system = std::ptr::null_mut();

            let err = acOpenSystem(&mut h_system);
            log_error("acOpenSystem", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let mut devices: usize = 0;

            let err = acSystemUpdateDevices(h_system, 200);
            log_error("acSystemUpdateDevices", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);

            let err = acSystemGetNumDevices(h_system, &mut devices);
            log_error("acSystemGetNumDevices", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);
            if devices == 0 {
                log::warn!("No devices detected");
            } else {
                log::debug!("Devices detected: {}", devices);

            }


            let max_buf = 1024;
            for dev in 0..devices {
                log::debug!("Retriving Device {}...", dev);
                
                // get device model
                let p_device_model = std::ptr::null_mut();
                let mut p_device_model_len = max_buf;
                let err = acSystemGetDeviceModel(h_system, dev, p_device_model, &mut p_device_model_len);
                log_error("acSystemGetDeviceModel", err);
                if err != AC_ERR_SUCCESS {
                    return;
                }
        
                // get device serial
                let p_device_serial = std::ptr::null_mut();
                let mut p_device_serial_len = max_buf;
                let err = acSystemGetDeviceSerial(h_system, dev, p_device_serial, &mut p_device_serial_len);
                log_error("acSystemGetDeviceSerial", err);
                if err != AC_ERR_SUCCESS {
                    return;
                }

                // get device IP address
                let p_ip_address_str= std::ptr::null_mut();
                let mut p_ip_address_str_buf_len = max_buf;
                let err = acSystemGetDeviceIpAddressStr(h_system, dev, p_ip_address_str, &mut p_ip_address_str_buf_len);
                log_error("acSystemGetDeviceIpAddressStr", err);
                if err != AC_ERR_SUCCESS {
                    return;
                }
        
                println!("Device {}: {:?} | {:?} | {:?}", dev, p_device_model, p_device_serial, p_ip_address_str);
            }



            let selection = 0;
            // err = SelectDevice(hSystem, &numDevices, &selection);
            
            let mut h_device: acDevice = std::ptr::null_mut();
            let err = acSystemCreateDevice(h_system, selection, &mut h_device);
            log_error("acSystemCreateDevice", err);
        
            // run
            // printf("Commence example\n\n");
            acquire_images(h_device, 25).unwrap();
            // printf("\nExample complete\n");
        
            // clean up
            let err = acSystemDestroyDevice(h_system, h_device);
            log_error("acSystemDestroyDevice", err);


            let err = acCloseSystem(h_system);
            log_error("acCloseSystem", err);
            assert!(err == AC_ERROR_LIST_AC_ERR_SUCCESS);
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// gets node value
    /// (1) gets node
    /// (2) checks access mode
    /// (3) gets value
    fn get_node_value(h_node_map: acNodeMap, node_name: *const ::std::os::raw::c_char, p_value: *mut i8, p_len: *mut usize) -> Result<(), AC_ERROR> {
        unsafe {
            // get node
            let mut h_node: acNode = std::ptr::null_mut();
            let mut access_mode: AC_ACCESS_MODE = 0;
            let err = acNodeMapGetNodeAndAccessMode(h_node_map, node_name, &mut h_node, &mut access_mode);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
            // check access mode
            if access_mode != AC_ACCESS_MODE_RO && access_mode != AC_ACCESS_MODE_RW {
                return Err(AC_ERR_ERROR);
            }
            // get value
            let err = acValueToString(h_node, p_value, p_len);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
        }
        Ok(())
    }
    ///
    /// sets node value
    /// (1) gets node
    /// (2) checks access mode
    /// (3) gets value
    fn set_node_value(h_node_map: acNodeMap, node_name: *const ::std::os::raw::c_char, p_value: *const ::std::os::raw::c_char) -> Result<(), AC_ERROR>{
        unsafe {
            // get node
            let mut h_node: acNode = std::ptr::null_mut();
            let mut access_mode = 0;
            let err = acNodeMapGetNodeAndAccessMode(h_node_map, node_name, &mut h_node, &mut access_mode);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
            // check access mode
            if access_mode != AC_ACCESS_MODE_WO && access_mode != AC_ACCESS_MODE_RW {
                return Err(AC_ERR_ERROR);
            }
            // get value
            let err = acValueFromString(h_node, p_value);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
        }
        Ok(())
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
    fn acquire_images(h_device: acDevice, images: usize) -> Result<(), AC_ERROR> {
        unsafe {
            // get node map
            let mut h_node_map: acNodeMap = std::ptr::null_mut();
            let err = acDeviceGetNodeMap(h_device, &mut h_node_map);
            log_error("acSystemGetDeviceModel", err);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
            // get node values that will be changed in order to return their values at
            // the end of the example
            let p_acquisition_mode_initial = std::ptr::null_mut();
            let mut len = MAX_BUF;
            let node_name = CString::from_str("AcquisitionMode").unwrap();
            get_node_value(h_node_map, node_name.as_ptr(), p_acquisition_mode_initial, &mut len)?;
            // set acquisition mode
            log::debug!(".acquire_images | Set acquisition mode to 'Continuous'...");
            set_node_value(
                h_node_map,
                node_name.as_ptr(),
                CString::from_str("Continuous").unwrap().as_ptr()
            )?;
            // set buffer handling mode
            log::debug!(".acquire_images | Set buffer handling mode to 'NewestOnly'");
            // get stream node map
            let mut h_tlstream_node_map: acNodeMap = std::ptr::null_mut();
            let err = acDeviceGetTLStreamNodeMap(h_device, &mut h_tlstream_node_map);
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
            set_node_value(
                h_tlstream_node_map,
                CString::from_str("StreamBufferHandlingMode").unwrap().as_ptr(),
                CString::from_str("NewestOnly").unwrap().as_ptr(),
            )?;
            // The TransportStreamProtocol node can tell the camera to use the TCP datastream engine. When
            //    set to TCP - Arena will switch to using the TCP datastream engine. 
            //    There is no further necessary configuration, though to achieve maximum throughput 
            //    users may want to set the "DeviceLinkThroughputReserve" to 0 and 
            //    also set the stream channel packet delay "GevSCPD" to 0.
            let mut h_transport_stream_protocol_node: acNode = std::ptr::null_mut();
            let mut access_mode_transport_stream_protocol = 0;
            let err = acNodeMapGetNodeAndAccessMode(
                h_node_map,
                CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                &mut h_transport_stream_protocol_node,
                &mut access_mode_transport_stream_protocol,
            );
            if err != AC_ERR_SUCCESS {
                return Err(err);
            }
            if access_mode_transport_stream_protocol != AC_ACCESS_MODE_NI {
                // get node value
                let p_transport_stream_protocol_initial = std::ptr::null_mut();
                let mut len = MAX_BUF;
                get_node_value(
                    h_node_map,
                    CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                    p_transport_stream_protocol_initial,
                    &mut len,
                )?;
                log::debug!(".acquire_images | Set Transport Stream Protocol to TCP");
                let err = acNodeMapSetEnumerationValue(
                    h_node_map,
                    CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                    CString::from_str("TCP").unwrap().as_ptr(),
                );
                if err != AC_ERR_SUCCESS {
                    return Err(err);
                }
                // start stream
                log::debug!(".acquire_images | Start stream");
                let err = acDeviceStartStream(h_device);
                if err != AC_ERR_SUCCESS {
                    return Err(err);
                }
                // get images
                log::debug!(".acquire_images | Getting {} images", images);
                for i in 0..images {
                    // get image
                    log::debug!(".acquire_images | Getting {} image...", i);
                    let mut h_buffer: acBuffer = std::ptr::null_mut();
                    let err = acDeviceGetBuffer(h_device, IMAGE_TIMEOUT, &mut h_buffer);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                    // get image information
                    log::debug!(" (");
                    // get and display size filled
                    let mut size_filled = 0;
                    let err = acBufferGetSizeFilled(h_buffer, &mut size_filled);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                    log::debug!("{} bytes; ", size_filled);
                    // get and display width
                    let mut width = 0;
                    let err = acImageGetWidth(h_buffer, &mut width);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                    log::debug!("{}", width);
                    // get and display height
                    let mut height = 0;
                    let err = acImageGetHeight(h_buffer, &mut height);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                    log::debug!("{}; ", height);
                    // get and display timestamp
                    let mut timestamp_ns = 0;
                    let err = acImageGetTimestampNs(h_buffer, &mut timestamp_ns);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                    const PRIu64: &str = "'l' 'u'";
                    log::debug!(".acquire_images | timestamp (ns): {} {} )", timestamp_ns, PRIu64);
                    // requeue image buffer
                    log::debug!(".acquire_images |  and requeue");
                    let err = acDeviceRequeueBuffer(h_device, h_buffer);
                    if err != AC_ERR_SUCCESS {
                        return Err(err);
                    }
                }
                // stop stream
                log::debug!(".acquire_images | Stop stream");
                let err = acDeviceStopStream(h_device);
                if err != AC_ERR_SUCCESS {
                    return Err(err);
                }
                // return node to its initial values
                set_node_value(
                    h_node_map,
                    CString::from_str("TransportStreamProtocol").unwrap().as_ptr(),
                    p_transport_stream_protocol_initial,
                )?;
            } else {
                log::warn!(".acquire_images | Connected camera does not support TCP stream");
            }
            // return node to its initial values
            set_node_value(
                h_node_map,
                CString::from_str("AcquisitionMode").unwrap().as_ptr(),
                p_acquisition_mode_initial,
            )?;
        }
        Ok(())
    }
    ///
    /// 
    fn log_error(dbg: &str, err: i32) {
        if err > 0 {
            log::error!("{} | err: {}", dbg, err);
        } else {
            log::debug!("{} | Ok", dbg);
        }
    }
}

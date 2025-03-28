use sal_core::error::Error;
use sal_sync::services::entity::name::Name;

use crate::infrostructure::arena::bindings::acSystemUpdateDevices;

use super::{ac_err::AcErr, bindings::{acCloseSystem, acOpenSystem, acSystem, acSystemGetNumDevices}, ffi_str::FfiStr};

///
/// Representation of the system object, the entry point into Arena SDK.
pub struct AcSystem {
    name: Name,
    pub system: acSystem,
    devices: Option<usize>,
    // timeout for detecting camera devices (in milliseconds).
    update_devices_timeout: u64
}
//
//
impl AcSystem {
    ///
    /// 
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            name: Name::new(parent.into(), "AcSystem"),
            system: std::ptr::null_mut(),
            devices: None,
            update_devices_timeout: 200,
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> Result<(), Error> {
        let error = Error::new(&self.name, "run");
        unsafe {
            let err = AcErr::from(acOpenSystem(&mut self.system));
            match err {
                AcErr::Success => {
                    let err = AcErr::from(acSystemUpdateDevices(self.system, self.update_devices_timeout));
                    match err {
                        AcErr::Success => {
                            let mut devices = 0;
                            let err = AcErr::from(acSystemGetNumDevices(self.system, &mut devices));
                            match err {
                                AcErr::Success => {
                                    self.devices = Some(devices);
                                    Ok(())
                                }
                                _ => Err(error.pass_with("acSystemGetNumDevices", err.to_string())),
                            }
                        }
                        _ => Err(error.pass_with("acSystemUpdateDevices", err.to_string())),
                    }
                }
                _ => Err(error.pass_with("acOpenSystem", err.to_string())),
            }
        }
    }
    ///
    /// Returns number of devices if was updated, call run first
    pub fn devices(&self) -> Option<usize> {
        self.devices
    }
    ///
    /// 
    // err = acSystemGetDeviceVendor(hSystem, i, pBuf, &len);
    ///
    /// Returns the Vendor name of a device
    /// - `h_system` - The acSystem object
    /// - `dev` - Index of the device
    pub fn device_vendor(&self, dev: usize) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            log::trace!("{}.device_vendor | Device {}...", self.name, dev);
            let err = AcErr::from(super::bindings::acSystemGetDeviceVendor(self.system, dev, result.as_mut_ptr() as *mut i8, &mut result.len));
            let result = result.to_string();
            log::trace!("{}.device_vendor | Device {} Model: {:?}", self.name, dev, result);
            match err {
                AcErr::Success => Ok(result),
                _ => Err(Error::new(&self.name, "device_vendor").err(err)),
            }
        }
    }
    ///
    /// Returns the Model name of a device
    /// - `h_system` - The acSystem object
    /// - `dev` - Index of the device
    pub fn device_model(&self, dev: usize) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            log::trace!("{}.device_model | Device {}...", self.name, dev);
            let err = AcErr::from(super::bindings::acSystemGetDeviceModel(self.system, dev, result.as_mut_ptr() as *mut i8, &mut result.len));
            let result = result.to_string();
            log::trace!("{}.device_model | Device {} Model: {:?}", self.name, dev, result);
            match err {
                AcErr::Success => Ok(result),
                _ => Err(Error::new(&self.name, "device_model").err(err)),
            }
        }
    }
    ///
    /// Returns the Serial number of a device.
    /// A serial number differentiates between devices. Each LUCID device has a unique serial
    /// number. LUCID serial numbers are numeric, but the serial numbers of other
    /// vendors may be alphanumeric.
    /// - `h_system` - The acSystem object
    /// - `dev` - Index of the device
    pub fn device_serial(&self, dev: usize) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            let err = AcErr::from(super::bindings::acSystemGetDeviceSerial(self.system, dev, result.as_mut_ptr(), &mut result.len));
            let result = result.to_string();
            log::trace!("{}.device_serial | Device {} Serial: {:?}", self.name, dev, result);
            match err {
                AcErr::Success => Ok(result),
                _ => Err(Error::new(&self.name, "device_serial").err(err)),
            }
        }
    }
    ///
    /// 
    // err = acSystemGetDeviceMacAddressStr(hSystem, i, pBuf, &len);
    ///
    /// Returns the MAC address of a device on the network, returning it as a string.
    /// - `h_system` - The acSystem object
    /// - `dev` - Index of the device
    pub fn device_mac(&self, dev: usize) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            let err = AcErr::from(super::bindings::acSystemGetDeviceMacAddressStr(self.system, dev, result.as_mut_ptr(), &mut result.len));
            let result = result.to_string();
            log::trace!("{}.device_mac | Device {} MAC: {:?}", self.name, dev, result);
            match err {
                AcErr::Success => Ok(result),
                _ => Err(Error::new(&self.name, "device_mac").err(err)),
            }
        }
    }
    ///
    /// Returns the IP address of a device on the network, returning it as a string.
    /// - `h_system` - The acSystem object
    /// - `dev` - Index of the device
    pub fn device_ip(&self, dev: usize) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            let err = AcErr::from(super::bindings::acSystemGetDeviceIpAddressStr(self.system, dev, result.as_mut_ptr(), &mut result.len));
            let result = result.to_string();
            log::trace!("{}.device_ip | Device {} IP: {:?}", self.name, dev, result);
            match err {
                AcErr::Success => Ok(result),
                _ => Err(Error::new(&self.name, "device_ip").err(err)),
            }
        }
    }
    // ///
    // /// Cleans up the system (acSystem) and deinitializes the Arena SDK, deallocating all memory.
    // pub fn close(&self) -> Result<(), Error> {
    //     let err = AcErr::from(unsafe { acCloseSystem(self.system) });
    //     match err {
    //         AcErr::Success => Ok(()),
    //         _ => Err(Error(format!("{}.close | Error: {}", self.name, err))),
    //     }
    // }
}
//
//
impl Drop for AcSystem {
    fn drop(&mut self) {
        unsafe {
            let err = AcErr::from(acCloseSystem(self.system));
            if err != AcErr::Success {
                log::error!("{}.drop | Error: {}", self.name, err)
            }
        }
    }
}

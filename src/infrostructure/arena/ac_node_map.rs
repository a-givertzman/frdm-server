use std::{ffi::CString, str::FromStr};

use sal_core::error::Error;
use sal_sync::services::entity::name::Name;

use super::{
    ac_access_mode::AcAccessMode, ac_err::AcErr, ac_node::AcNode, bindings::{acDevice, acNode, acNodeMap, acNodeMapGetEnumerationValue, acNodeMapGetNode, acNodeMapGetNodeAndAccessMode, acNodeMapSetBooleanValue, acNodeMapSetEnumerationValue, acValueFromString, acValueToString}, ffi_str::FfiStr
};

///
/// Represents `acNodeMap`, supports:
/// - Device NodeMap
/// - Device TL Stream NodeMap
pub struct AcNodeMap {
    name: Name,
    kind: String,
    device: acDevice,
    pub map: acNodeMap
}
impl AcNodeMap {
    ///
    /// Returns new instance of the Device Node Map of kind:
    /// - `DeviceNodeMap`
    /// - TLStreamNodeMap
    pub fn new(parent: impl Into<String>, device: acDevice, map: acNodeMap, kind: impl Into<String>) -> Self {
        let kind = kind.into();
        let name = Name::new(parent.into(), format!("AcNodeMap({})", kind));
        Self {
            name,
            kind,
            device,
            map,
        }
    }
    ///
    /// Get node
    pub fn get_node(&self, node_name: &str) -> Result<AcNode, Error> {
        let mut node: acNode = std::ptr::null_mut();
        let err = AcErr::from(unsafe { acNodeMapGetNode(
            self.map,
            CString::from_str(node_name).unwrap().as_ptr(),
            &mut node
        ) });
        match err {
            AcErr::Success => Ok(AcNode::new(&self.name, node, node_name)),
            _ => Err(Error::new(&self.name, "get_node").pass_with(format!("Get node '{node_name}'"),err.to_string())),
        }
    }

    ///
    /// Get Node Access mode
    /// - #[doc = "< "]
    /// - AC_ACCESS_MODE_NI = 0 - Not implemented
    /// - AC_ACCESS_MODE_NA = 1 - Not available
    /// - AC_ACCESS_MODE_WO = 2 - Write only
    /// - AC_ACCESS_MODE_RO = 3 - Read only
    /// - AC_ACCESS_MODE_RW = 4 - Read and write
    pub fn get_access_mode(&self, node_name: &str) -> Result<AcAccessMode, Error> {
        let mut h_transport_stream_protocol_node: acNode = std::ptr::null_mut();
        let mut access_mode = 0;
        let err = AcErr::from(unsafe { acNodeMapGetNodeAndAccessMode(
            self.map,
            CString::from_str(node_name).unwrap().as_ptr(),
            &mut h_transport_stream_protocol_node,
            &mut access_mode,
        ) });
        match err {
            AcErr::Success => Ok(AcAccessMode::from(access_mode)),
            _ => Err(Error::new(&self.name, "get_access_mode").err(err)),
        }
    }
    ///
    /// Gets String node value
    pub fn get_value(&self, node_name: &str) -> Result<String, Error> {
        let error= Error::new(&self.name, "get_value");
        unsafe {
            // get node
            let mut h_node: super::bindings::acNode = std::ptr::null_mut();
            let mut access_mode = 0;
            let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                self.map,
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
                                _ => Err(error.pass_with("ValueToString Error", err.to_string())),
                            }
                        },
                        _ => Err(error.err(format!("Access denied, current mode is '{access_mode}'"))),
                    }
                },
                _ => Err(error.pass(err.to_string())),
            }
        }
    }
    ///
    /// Sets String node value
    pub fn set_value(&self, node_name: &str, value: &str) -> Result<(), Error>{
        let error= Error::new(&self.name, "set_value");
        unsafe {
            let mut h_node: super::bindings::acNode = std::ptr::null_mut();
            let mut access_mode = 0;
            let err = AcErr::from(acNodeMapGetNodeAndAccessMode(
                self.map,
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
                                _ => Err(error.pass_with("ValueFromString Error", err.to_string())),
                            }
                        },
                        _ => return Err(error.err(format!("Access denied, current mode is '{access_mode}'"))),
                    }
                },
                _ => Err(error.pass(err.to_string())),
            }
        }
    }
    ///
    /// Gets Enumeration node value
    pub fn get_enum_value(&self, node_name: &str) -> Result<String, Error> {
        unsafe {
            let mut result = FfiStr::<1024>::new();
            let err = AcErr::from(acNodeMapGetEnumerationValue(
                self.map,
                CString::new(node_name).unwrap().as_ptr(),
                result.as_mut_ptr(),
                &mut result.len),
            );
            match err {
                AcErr::Success => Ok(result.to_string()),
                _ => Err(Error::new(&self.name, "get_enum_value").err(err)),
            }
        }
    }
    ///
    /// Sets Enumeration node value
    pub fn set_enum_value(&self, node_name: &str, value: &str) -> Result<(), Error>{
        let error= Error::new(&self.name, "set_enum_value");
        match self.get_access_mode(node_name) {
            Ok(access_mode) => match access_mode {
                AcAccessMode::ReadWrite | AcAccessMode::WriteOnly => {
                    let err = AcErr::from(unsafe { acNodeMapSetEnumerationValue(
                        self.map,
                        CString::from_str(node_name).unwrap().as_ptr(),
                        CString::from_str(value).unwrap().as_ptr(),
                    ) });
                    match err {
                        AcErr::Success => Ok(()),
                        _ => Err(error.err(err)),
                    }
                },
                _ => return Err(error.err(format!("Access denied, current mode is '{access_mode}'"))),
            }
            Err(err) => Err(error.pass(err)),
        }
    }
    ///
    /// Sets Bool node value
    pub fn set_bool_value(&self, node_name: &str, value: bool) -> Result<(), Error>{
        let error= Error::new(&self.name, "set_bool_value");
        match self.get_access_mode(node_name) {
            Ok(access_mode) => match access_mode {
                AcAccessMode::ReadWrite | AcAccessMode::WriteOnly => {
                    let err = AcErr::from(unsafe { acNodeMapSetBooleanValue(
                        self.map,
                        CString::from_str(node_name).unwrap().as_ptr(),
                        if value {1u8} else {0},
                    ) });
                    match err {
                        AcErr::Success => Ok(()),
                        _ => Err(error.err(err)),
                    }
                },
                _ => return Err(error.err(format!("Access denied, current mode is '{access_mode}'"))),
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

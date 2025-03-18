use std::{ffi::CString, str::FromStr};

use sal_sync::services::entity::{error::str_err::StrErr, name::Name};

use super::{
    ac_access_mode::AcAccessMode, ac_err::AcErr, ffi_str::FfiStr,
    bindings::{acDevice, acNodeMap, acNodeMapGetNodeAndAccessMode, acNodeMapSetEnumerationValue, acValueFromString, acValueToString},
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
    /// Gets String node value
    pub fn get_value(&self, node_name: &str) -> Result<String, StrErr> {
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
                                _ => Err(StrErr(format!("{}.get_value | ValueToString Error: {}", self.name, err))),
                            }
                        },
                        _ => Err(StrErr(format!("{}.get_value | Access denied, current mode is '{}'", self.name, access_mode))),
                    }
                },
                _ => Err(StrErr(format!("{}.get_value | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// Sets String node value
    pub fn set_value(&self, node_name: &str, value: &str) -> Result<(), StrErr>{
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
                        AcAccessMode::ReadWrite | AcAccessMode::WriteOnly => {
                            let err = AcErr::from(acValueFromString(h_node, CString::new(value).unwrap().as_ptr()));
                            match err {
                                AcErr::Success => Ok(()),
                                _ => Err(StrErr(format!("{}.set_value | ValueFromString Error: {}", self.name, err))),
                            }
                        },
                        _ => return Err(StrErr(format!("{}.set_value | Access denied, current mode is '{}'", self.name, access_mode))),
                    }
                },
                _ => Err(StrErr(format!("{}.set_value | Error: {}", self.name, err))),
            }
        }
    }
    ///
    /// Sets Enumeration node value
    pub fn set_enumeration_value(&self, node_name: &str, value: &str) -> Result<(), StrErr>{
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
                        AcAccessMode::ReadWrite | AcAccessMode::WriteOnly => {
                            let err = AcErr::from(acNodeMapSetEnumerationValue(
                                self.map,
                                CString::from_str(node_name).unwrap().as_ptr(),
                                CString::from_str(value).unwrap().as_ptr(),
                            ));
                            match err {
                                AcErr::Success => Ok(()),
                                _ => Err(StrErr(format!("{}.set_enumeration_value | ValueFromString Error: {}", self.name, err))),
                            }
                        },
                        _ => return Err(StrErr(format!("{}.set_enumeration_value | Access denied, current mode is '{}'", self.name, access_mode))),
                    }
                },
                _ => Err(StrErr(format!("{}.set_enumeration_value | Error: {}", self.name, err))),
            }
        }
    }

}

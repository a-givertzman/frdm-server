use std::{ffi::CString, str::FromStr};

use sal_sync::services::entity::{error::str_err::StrErr, name::Name};

use super::{ac_err::AcErr, 
    bindings::{acFloatGetMax, acFloatGetMin, acFloatGetValue, acFloatSetValue, acIsWritable, acNode}}
;

///
/// Represents `acNode`, supports:
pub struct AcNode {
    name: Name,
    node: acNode,
}
impl AcNode {
    ///
    /// Returns new instance of the Device Node Map of kind:
    /// - `DeviceNodeMap`
    /// - TLStreamNodeMap
    pub fn new(parent: impl Into<String>, node: acNode, node_name: &str) -> Self {
        let name = Name::new(parent.into(), format!("AcNode({})", node_name));
        Self {
            name,
            node
        }
    }
    ///
    /// Returns `true` if current Node accessed and writable
    pub fn is_writable(&self) -> bool {
        let mut is_writable = 0;
	    let err = AcErr::from(unsafe { acIsWritable(self.node, &mut is_writable) });
        match err {
            AcErr::Success => is_writable > 0,
            _ => {
                let err = StrErr(format!("{}.is_writable | Error: {}", self.name, err));
                log::warn!("{}", err);
                false
            }
        }
    }
    ///
    /// Gets f64 node value
    pub fn get_float_value(&self) -> Result<f64, StrErr> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetValue(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(StrErr(format!("{}.get_float_value | Error: {}", self.name, err))),
        }
    }
    ///
    /// Sets f64 node value
    pub fn set_float_value(&self, value: f64) -> Result<(), StrErr> {
        let err = AcErr::from(unsafe { acFloatSetValue(self.node, value) });
        match err {
            AcErr::Success => Ok({}),
            _ => Err(StrErr(format!("{}.get_float_value | Error: {}", self.name, err))),
        }
    }
    ///
    /// Gets Minimum f64 node value
    pub fn get_float_min_value(&self) -> Result<f64, StrErr> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetMin(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(StrErr(format!("{}.get_float_min_value | Error: {}", self.name, err))),
        }
    }
    ///
    /// Gets Maximum f64 node value
    pub fn get_float_max_value(&self) -> Result<f64, StrErr> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetMax(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(StrErr(format!("{}.get_float_max_value | Error: {}", self.name, err))),
        }
    }
}

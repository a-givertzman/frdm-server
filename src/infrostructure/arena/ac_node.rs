use sal_core::error::Error;
use sal_sync::services::entity::name::Name;

use super::{ac_err::AcErr, 
    bindings::{acBooleanGetValue, acFloatGetMax, acFloatGetMin, acFloatGetValue, acFloatSetValue, acIntegerGetMax, acIntegerGetMin, acIntegerGetValue, acIntegerSetValue, acIsWritable, acNode, acStringGetValue}, ffi_str::FfiStr}
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
                let err = Error::new(&self.name, "is_writable").err(err);
                log::warn!("{}", err);
                false
            }
        }
    }
    ///
    /// Gets bool node value
    pub fn get_bool_value(&self) -> Result<bool, Error> {
        let mut value = 0;
        let err = AcErr::from(unsafe { acBooleanGetValue(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value > 0),
            _ => Err(Error::new(&self.name, "get_bool_value").err(err)),
        }
    }
    ///
    /// Gets f64 node value
    pub fn get_float_value(&self) -> Result<f64, Error> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetValue(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_float_value").err(err)),
        }
    }
    ///
    /// Sets f64 node value
    pub fn set_float_value(&self, value: f64) -> Result<(), Error> {
        let err = AcErr::from(unsafe { acFloatSetValue(self.node, value) });
        match err {
            AcErr::Success => Ok({}),
            _ => Err(Error::new(&self.name, "set_float_value").err(err)),
        }
    }
    ///
    /// Gets Minimum f64 node value
    pub fn get_float_min_value(&self) -> Result<f64, Error> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetMin(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_float_min_value").err(err)),
        }
    }
    ///
    /// Gets Maximum f64 node value
    pub fn get_float_max_value(&self) -> Result<f64, Error> {
        let mut value = 0.0;
        let err = AcErr::from(unsafe { acFloatGetMax(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_float_max_value").err(err)),
        }
    }
    ///
    /// Gets i64 node value
    pub fn get_int_value(&self) -> Result<i64, Error> {
        let mut value = 0;
        let err = AcErr::from(unsafe { acIntegerGetValue(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_int_value").err(err)),
        }
    }
    ///
    /// Sets i64 node value
    pub fn set_int_value(&self, value: i64) -> Result<(), Error> {
        let err = AcErr::from(unsafe { acIntegerSetValue(self.node, value) });
        match err {
            AcErr::Success => Ok({}),
            _ => Err(Error::new(&self.name, "set_int_value").err(err)),
        }
    }
    ///
    /// Gets Minimum i64 node value
    pub fn get_int_min_value(&self) -> Result<i64, Error> {
        let mut value = 0;
        let err = AcErr::from(unsafe { acIntegerGetMin(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_int_min_value").err(err)),
        }
    }
    ///
    /// Gets Maximum i64 node value
    pub fn get_int_max_value(&self) -> Result<i64, Error> {
        let mut value = 0;
        let err = AcErr::from(unsafe { acIntegerGetMax(self.node, &mut value) });
        match err {
            AcErr::Success => Ok(value),
            _ => Err(Error::new(&self.name, "get_int_max_value").err(err)),
        }
    }
    ///
    /// Gets string node value
    pub fn get_str_value(&self) -> Result<String, Error> {
        let mut result = FfiStr::<1024>::new();
        let err = AcErr::from(unsafe { acStringGetValue(self.node, result.as_mut_ptr(), &mut result.len) });
        match err {
            AcErr::Success => Ok(result.to_string()),
            _ => Err(Error::new(&self.name, "get_str_value").err(err)),
        }
    }
}

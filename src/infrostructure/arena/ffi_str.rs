///
/// ```ignore
/// let mut result = FfiStr::new(1024);
/// let err = cApi::getName(result.as_mut_ptr() as *mut i8, &mut result.len));
/// let result = result.to_string();
/// log::debug!("Result: {}", result);
/// ```
pub struct FfiStr<const SIZE: usize> {
    ///
    /// The length in bytes of the data stored under the raw pointer
    pub len: usize,
    raw: [i8; SIZE],
}
//
//
impl<const SIZE: usize> FfiStr<SIZE> {
    ///
    /// 
    pub fn new() -> Self {
        Self {
            len: SIZE,
            raw: [0i8; SIZE],
        }
    }
    ///
    /// Retirns mutable raw pointer
    pub fn as_mut_ptr(&mut self) -> *mut i8 {
        self.raw.as_mut_ptr() as *mut i8
    }
    ///
    /// Returns a string of length `len` from raw pointer  
    pub fn to_string(&mut self) -> String {
        log::trace!("FfiStr.device_model | len: {}, raw: {:?}", self.len, self.raw);
        let buf = self.raw[..self.len].iter().map(|item| *item as u8).collect();
        log::trace!("FfiStr.device_model | buf: {:?}", buf);
        let mut result = String::from_utf8(buf).unwrap_or(String::new());
        if result.ends_with('\0') {
            result.pop();
        }
        result
    }
}

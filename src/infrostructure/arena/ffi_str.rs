///
/// 
pub struct FfiStr {
    pub len: usize,
    raw: [i8; 1024],
    str: String,
}
//
//
impl FfiStr {
    ///
    /// 
    pub fn new(len: usize) -> Self {
        Self {
            len,
            raw: [0i8; 1024],
            str: String::new(),
        }
    }
    pub fn as_mut_ptr(&mut self) -> *mut i8 {
        self.raw.as_mut_ptr() as *mut i8
    }
    pub fn to_string(&self) -> String {
        log::trace!("FfiStr.device_model | raw: {:?}", self.raw);
        let buf: Vec<u8> = self.raw[..self.len].iter().map(|item| *item as u8).collect();
        log::trace!("FfiStr.device_model | buf: {:?}", buf);
        let mut result = String::from_utf8(buf).unwrap();
        if result.ends_with('\0') {
            result.pop();
        }
        result
    }
}

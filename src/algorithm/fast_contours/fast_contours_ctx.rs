use crate::domain::Image;
///
/// Filtered and binarised image
#[derive(Debug, Clone)]
pub struct FastContoursCtx {
    pub result: Image,
}
//
//
impl Default for FastContoursCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}

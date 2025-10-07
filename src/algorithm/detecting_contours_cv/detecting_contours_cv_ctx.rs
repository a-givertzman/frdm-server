use crate::domain::Image;
///
/// Filtered and binarised image
#[derive(Debug, Clone)]
pub struct CvContoursCtx {
    pub result: Image,
}
//
//
impl Default for CvContoursCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}

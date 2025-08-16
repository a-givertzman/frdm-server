use crate::domain::Image;
///
/// Cropped image
#[derive(Debug, Clone)]
pub struct CroppingCtx {
    pub result: Image,
}
//
//
impl Default for CroppingCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}

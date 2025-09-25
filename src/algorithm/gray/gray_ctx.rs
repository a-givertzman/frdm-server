use crate::domain::Image;
///
/// Gray scale image
#[derive(Debug, Clone)]
pub struct GrayCtx {
    pub frame: Image,
}
//
//
impl Default for GrayCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

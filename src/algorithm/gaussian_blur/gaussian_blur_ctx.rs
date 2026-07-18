use crate::domain::Image;
///
/// Gaussian blur result image
#[derive(Debug, Clone)]
pub struct GaussianBlurCtx {
    pub frame: Image,
}
//
//
impl Default for GaussianBlurCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

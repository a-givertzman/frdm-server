use crate::domain::Image;
///
/// Cropped image
#[derive(Debug, Clone)]
pub struct BufferCtx {
    pub result: Image,
}
//
//
impl Default for BufferCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}

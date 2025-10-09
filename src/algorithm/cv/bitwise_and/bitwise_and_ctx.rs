use crate::domain::Image;
///
/// Bitwise And result image
#[derive(Debug, Clone)]
pub struct BitwiseAndCtx {
    pub frame: Image,
}
//
//
impl Default for BitwiseAndCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

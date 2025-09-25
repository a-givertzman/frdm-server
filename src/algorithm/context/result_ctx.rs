use crate::domain::Image;
///
/// Common result image from current step
#[derive(Debug, Clone)]
pub struct ResultCtx {
    pub frame: Image,
}
//
//
impl Default for ResultCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

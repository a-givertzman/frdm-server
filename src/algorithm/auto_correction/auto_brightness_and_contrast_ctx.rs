use crate::domain::Image;
///
/// Brightness and Contrast corrected [Image]
#[derive(Debug, Clone)]
pub struct AutoBrightnessAndContrastCtx {
    pub result: Image,
}
//
//
impl Default for AutoBrightnessAndContrastCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}
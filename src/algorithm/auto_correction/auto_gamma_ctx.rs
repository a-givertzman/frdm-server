use crate::domain::Image;
///
/// Gamma corrected [Image]
#[derive(Debug, Clone)]
pub struct AutoGammaCtx {
    pub result: Image,
}
//
//
impl Default for AutoGammaCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}
use crate::domain::Image;
///
/// TemporalFilter result image
#[derive(Debug, Clone)]
pub struct TemporalFilterCtx {
    pub frame: Image,
}
//
//
impl Default for TemporalFilterCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

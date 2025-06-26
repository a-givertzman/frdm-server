use crate::infrostructure::arena::Image;
///
/// Filtered and binarised image
#[derive(Debug, Clone)]
pub struct DetectingContoursCvCtx {
    pub result: Image,
}
//
//
impl Default for DetectingContoursCvCtx {
    fn default() -> Self {
        Self { 
            result: Image::default()
         }
    }
}

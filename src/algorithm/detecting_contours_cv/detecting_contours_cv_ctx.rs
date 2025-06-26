use crate::infrostructure::arena::Image;
///
/// Filtered and binarised image
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

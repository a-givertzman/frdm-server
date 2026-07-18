use crate::domain::Image;
///
/// Fine filtered and binarised image
#[derive(Debug, Clone)]
pub struct FineContoursCtx {
    pub result: Image,
}
//
//
impl Default for FineContoursCtx {
    fn default() -> Self {
        Self {
            result: Image::default(),
         }
    }
}

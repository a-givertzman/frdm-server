use crate::domain::Image;
///
/// Fine filtered and binarised image
#[derive(Debug, Clone)]
pub struct FineContoursCtx {
    pub convex: Image,
    // pub contour: Image,
    pub result: Image,
}
//
//
impl Default for FineContoursCtx {
    fn default() -> Self {
        Self {
            convex: Image::default(),
            // contour: Image::default(),
            result: Image::default(),
         }
    }
}

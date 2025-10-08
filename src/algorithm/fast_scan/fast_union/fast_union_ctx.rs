use crate::domain::Image;
///
/// `FastUnion` result contour
#[derive(Debug, Clone)]
pub struct FastUnionCtx {
    pub frame: Image,
}
//
//
impl Default for FastUnionCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

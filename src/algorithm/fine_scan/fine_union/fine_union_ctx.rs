use crate::domain::Image;
///
/// `FineUnion` result contour
#[derive(Debug, Clone)]
pub struct FineUnionCtx {
    pub frame: Image,
}
//
//
impl Default for FineUnionCtx {
    fn default() -> Self {
        Self { 
            frame: Image::default()
         }
    }
}

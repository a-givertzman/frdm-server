use crate::domain::Image;
///
/// Result convex of `FineContours` - solid contour around the rope
#[derive(Debug, Clone)]
pub struct FineConvexCtx {
    pub convex: Option<Image>,
}
//
//
impl Default for FineConvexCtx {
    fn default() -> Self {
        Self {
            convex: Default::default(),
         }
    }
}

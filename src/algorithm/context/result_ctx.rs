///
/// Common result from the step
#[derive(Debug, Clone)]
pub struct ResultCtx<T> {
    pub val: T,
}
//
//
impl<T: Default> Default for ResultCtx<T> {
    fn default() -> Self {
        Self { 
            val: T::default()
         }
    }
}

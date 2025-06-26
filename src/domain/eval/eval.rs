///
/// Trate defines common evaluation function for calculations classes
pub trait Eval<In, Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval(&self, val: In) -> Out;
}
///
/// Trate defines common mutable evaluation function for calculations classes
pub trait EvalMut<In, Out> {
    ///
    /// Performs a calculation
    /// - Returns [Out] contains results inside
    fn eval(&mut self, val: In) -> Out;
}

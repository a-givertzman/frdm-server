///
/// TODO
pub trait Eval<In, Out> {
    fn eval(&self, val: In) -> Out;
}
///
/// TODO
pub trait EvalMut<In, Out> {
    fn eval(&self, val: In) -> Out;
}

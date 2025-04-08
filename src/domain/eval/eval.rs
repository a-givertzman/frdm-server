pub trait Eval<In, Out> {
    fn eval(&mut self, val: In) -> Out;
}

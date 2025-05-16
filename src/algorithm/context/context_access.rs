use super::{context::Context};
use crate::{
    algorithm::{geometry_defect::contraciton::ContractionCtx, InitialCtx, InitialPoints}, 
    domain::Error, 
};
///
/// Provides restricted write access to the [Context] members
pub trait ContextWrite<T> {
    fn write(self, value: T) -> Result<Context, Error>;
}
///
/// Provides simple read access to the [Context] members
pub trait ContextRead<T> {
    fn read(&self) -> &T;
}
//
//
impl ContextWrite<InitialCtx> for Context {
    fn write(mut self, value: InitialCtx) -> Result<Self, Error> {
        self.initial = value;
        Result::Ok(self)
    }
}
impl ContextRead<InitialCtx> for Context {
    fn read(&self) -> &InitialCtx {
        &self.initial
    }
}
//
//
impl ContextWrite<InitialPoints<usize>> for Context {
    fn write(mut self, value: InitialPoints<usize>) -> Result<Self, Error> {
        self.initial_points = value;
        Result::Ok(self)
    }
}
impl ContextRead<InitialPoints<usize>> for Context {
    fn read(&self) -> &InitialPoints<usize> {
        &self.initial_points
    }
}
//
//
impl ContextWrite<ContractionCtx> for Context {
    fn write(mut self, value: ContractionCtx) -> Result<Self, Error> {
        self.contraction = value;
        Result::Ok(self)
    }
}
impl ContextRead<ContractionCtx> for Context {
    fn read(&self) -> &ContractionCtx {
        &self.contraction
    }
}

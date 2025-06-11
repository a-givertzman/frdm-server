use super::{context::Context};
use crate::{
    algorithm::{
        geometry_defect::GeometryDefectCtx, 
        width_emissions::WidthEmissionsCtx, 
        InitialCtx, 
        InitialPoints
    }, 
    domain::Error 
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
impl ContextWrite<WidthEmissionsCtx> for Context {
    fn write(mut self, value: WidthEmissionsCtx) -> Result<Self, Error> {
        self.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<WidthEmissionsCtx> for Context {
    fn read(&self) -> &WidthEmissionsCtx {
        &self.width_emissions
    }
}
//
//
impl ContextWrite<GeometryDefectCtx> for Context {
    fn write(mut self, value: GeometryDefectCtx) -> Result<Self, Error> {
        self.geometry_defect = value;
        Result::Ok(self)
    }
}
impl ContextRead<GeometryDefectCtx> for Context {
    fn read(&self) -> &GeometryDefectCtx {
        &self.geometry_defect
    }
}

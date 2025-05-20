use super::{context::Context};
use crate::{
    algorithm::{geometry_defect::{ContractionCtx, ExpansionCtx, GrooveCtx, MoundCtx}, width_emissions::WidthEmissionsCtx, InitialCtx, InitialPoints}, domain::Error 
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
impl ContextWrite<ExpansionCtx> for Context {
    fn write(mut self, value: ExpansionCtx) -> Result<Self, Error> {
        self.expansion = value;
        Result::Ok(self)
    }
}
impl ContextRead<ExpansionCtx> for Context {
    fn read(&self) -> &ExpansionCtx {
        &self.expansion
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
//
//
impl ContextWrite<GrooveCtx> for Context {
    fn write(mut self, value: GrooveCtx) -> Result<Self, Error> {
        self.groove = value;
        Result::Ok(self)
    }
}
impl ContextRead<GrooveCtx> for Context {
    fn read(&self) -> &GrooveCtx {
        &self.groove
    }
}
//
//
impl ContextWrite<MoundCtx> for Context {
    fn write(mut self, value: MoundCtx) -> Result<Self, Error> {
        self.mound = value;
        Result::Ok(self)
    }
}
impl ContextRead<MoundCtx> for Context {
    fn read(&self) -> &MoundCtx {
        &self.mound
    }
}

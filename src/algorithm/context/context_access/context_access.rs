//! 
//! Read / Write implementations for the root of the [Context]
//! 
use crate::{
    algorithm::{
        Context, ContextRead, ContextWrite, FastScanCtx, FineScanCtx, RopeDefectCtx, InitialCtx, NormalizedCtx, FineConvexCtx, ResultCtx
    },
    domain::{Error, Image},
};

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
impl ContextWrite<ResultCtx<Image>> for Context {
    fn write(mut self, value: ResultCtx<Image>) -> Result<Self, Error> {
        self.result = value;
        Result::Ok(self)
    }
}
impl ContextRead<ResultCtx<Image>> for Context {
    fn read(&self) -> &ResultCtx<Image> {
        &self.result
    }
}
//
//
impl ContextWrite<RopeDefectCtx<()>> for Context {
    fn write(mut self, value: RopeDefectCtx<()>) -> Result<Self, Error> {
        self.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDefectCtx<()>> for Context {
    fn read(&self) -> &RopeDefectCtx<()> {
        &self.defects
    }
}
//
//
impl ContextWrite<NormalizedCtx> for Context {
    fn write(mut self, value: NormalizedCtx) -> Result<Self, Error> {
        self.normalized = value;
        Result::Ok(self)
    }
}
impl ContextRead<NormalizedCtx> for Context {
    fn read(&self) -> &NormalizedCtx {
        &self.normalized
    }
}
//
//
impl ContextWrite<FastScanCtx> for Context {
    fn write(mut self, value: FastScanCtx) -> Result<Self, Error> {
        self.fast_scan = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx> for Context {
    fn read(&self) -> &FastScanCtx {
        &self.fast_scan
    }
}
//
//
impl ContextWrite<FineScanCtx> for Context {
    fn write(mut self, value: FineScanCtx) -> Result<Self, Error> {
        self.fine_scan = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx> for Context {
    fn read(&self) -> &FineScanCtx {
        &self.fine_scan
    }
}
//
//
impl ContextWrite<FineConvexCtx> for Context {
    fn write(mut self, value: FineConvexCtx) -> Result<Self, Error> {
        self.convex = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineConvexCtx> for Context {
    fn read(&self) -> &FineConvexCtx {
        &self.convex
    }
}

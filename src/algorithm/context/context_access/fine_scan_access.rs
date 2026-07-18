use crate::{
    algorithm::{
        FineContoursCtx, FineScanCtx, FineUnionCtx, RopeDefectCtx, RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx, FineEdgesCtx,
        Context, ContextRead, ContextWrite,
    },
    domain::Error,
};

//
//
impl ContextWrite<RopeDistortionsCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDistortionsCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDistortionsCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDistortionsCtx<FineScanCtx> {
        &self.fine_scan.width_emissions
    }
}
//
//
impl ContextWrite<RopeDefectCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDefectCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDefectCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDefectCtx<FineScanCtx> {
        &self.fine_scan.defects
    }
}
//
//
impl ContextWrite<RopeDimensionsCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDimensionsCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDimensionsCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDimensionsCtx<FineScanCtx> {
        &self.fine_scan.rope_dimensions
    }
}
//
//
impl ContextWrite<TemporalFilterCtx<FineScanCtx>> for Context {
    fn write(mut self, value: TemporalFilterCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<TemporalFilterCtx<FineScanCtx>> for Context {
    fn read(&self) -> &TemporalFilterCtx<FineScanCtx> {
        &self.fine_scan.temporal_filter
    }
}
//
//
impl ContextWrite<FineEdgesCtx> for Context {
    fn write(mut self, value: FineEdgesCtx) -> Result<Self, Error> {
        self.fine_scan.edges = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineEdgesCtx> for Context {
    fn read(&self) -> &FineEdgesCtx {
        &self.fine_scan.edges
    }
}
//
//
impl ContextWrite<FineUnionCtx> for Context {
    fn write(mut self, value: FineUnionCtx) -> Result<Self, Error> {
        self.fine_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineUnionCtx> for Context {
    fn read(&self) -> &FineUnionCtx {
        &self.fine_scan.union
    }
}
//
//
impl ContextWrite<FineContoursCtx> for Context {
    fn write(mut self, value: FineContoursCtx) -> Result<Self, Error> {
        self.fine_scan.fine_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineContoursCtx> for Context {
    fn read(&self) -> &FineContoursCtx {
        &self.fine_scan.fine_contours
    }
}

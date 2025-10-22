use crate::{
    algorithm::{
        FastContoursCtx, FastEdgesCtx, FastScanCtx, FastUnionCtx, RopeDefectCtx,
        RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx,
        Context, ContextRead, ContextWrite,
    },
    domain::Error,
};

//
//
impl ContextWrite<FastContoursCtx> for Context {
    fn write(mut self, value: FastContoursCtx) -> Result<Self, Error> {
        self.fast_scan.fast_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastContoursCtx> for Context {
    fn read(&self) -> &FastContoursCtx {
        &self.fast_scan.fast_contours
    }
}
//
//
impl ContextWrite<FastEdgesCtx> for Context {
    fn write(mut self, value: FastEdgesCtx) -> Result<Self, Error> {
        self.fast_scan.edges = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastEdgesCtx> for Context {
    fn read(&self) -> &FastEdgesCtx {
        &self.fast_scan.edges
    }
}
//
//
impl ContextWrite<RopeDistortionsCtx<FastScanCtx>> for Context {
    fn write(mut self, value: RopeDistortionsCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDistortionsCtx<FastScanCtx>> for Context {
    fn read(&self) -> &RopeDistortionsCtx<FastScanCtx> {
        &self.fast_scan.width_emissions
    }
}
//
//
impl ContextWrite<RopeDefectCtx<FastScanCtx>> for Context {
    fn write(mut self, value: RopeDefectCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDefectCtx<FastScanCtx>> for Context {
    fn read(&self) -> &RopeDefectCtx<FastScanCtx> {
        &self.fast_scan.defects
    }
}
//
//
impl ContextWrite<RopeDimensionsCtx<FastScanCtx>> for Context {
    fn write(mut self, value: RopeDimensionsCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDimensionsCtx<FastScanCtx>> for Context {
    fn read(&self) -> &RopeDimensionsCtx<FastScanCtx> {
        &self.fast_scan.rope_dimensions
    }
}
//
//
impl ContextWrite<TemporalFilterCtx<FastScanCtx>> for Context {
    fn write(mut self, value: TemporalFilterCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<TemporalFilterCtx<FastScanCtx>> for Context {
    fn read(&self) -> &TemporalFilterCtx<FastScanCtx> {
        &self.fast_scan.temporal_filter
    }
}
//
//
impl ContextWrite<FastUnionCtx> for Context {
    fn write(mut self, value: FastUnionCtx) -> Result<Self, Error> {
        self.fast_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastUnionCtx> for Context {
    fn read(&self) -> &FastUnionCtx {
        &self.fast_scan.union
    }
}

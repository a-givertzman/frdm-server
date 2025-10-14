use crate::{
    algorithm::{
        FineContoursCtx, FineScanCtx, FineUnionCtx, GeometryDefectCtx, RopeDimensionsCtx, TemporalFilterCtx, WidthEmissionsCtx,
    },
    domain::Error, Context, ContextRead, ContextWrite,
};

//
//
impl ContextWrite<FineScanCtx, WidthEmissionsCtx> for Context {
    fn write(mut self, value: WidthEmissionsCtx) -> Result<Self, Error> {
        self.fast_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx, WidthEmissionsCtx> for Context {
    fn read(&self) -> &WidthEmissionsCtx {
        &self.fast_scan.width_emissions
    }
}
//
//
impl ContextWrite<FineScanCtx, GeometryDefectCtx> for Context {
    fn write(mut self, value: GeometryDefectCtx) -> Result<Self, Error> {
        self.fine_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx, GeometryDefectCtx> for Context {
    fn read(&self) -> &GeometryDefectCtx {
        &self.fine_scan.defects
    }
}
// //
// //
// impl ContextWrite<GaussianBlurCtx> for Context {
//     fn write(mut self, value: GaussianBlurCtx) -> Result<Self, Error> {
//         self.gaussian_blur = value;
//         Result::Ok(self)
//     }
// }
// impl ContextRead<GaussianBlurCtx> for Context {
//     fn read(&self) -> &GaussianBlurCtx {
//         &self.gaussian_blur
//     }
// }
//
//
impl ContextWrite<FineScanCtx, RopeDimensionsCtx> for Context {
    fn write(mut self, value: RopeDimensionsCtx) -> Result<Self, Error> {
        self.fine_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx, RopeDimensionsCtx> for Context {
    fn read(&self) -> &RopeDimensionsCtx {
        &self.fine_scan.rope_dimensions
    }
}
//
//
impl ContextWrite<FineScanCtx, TemporalFilterCtx> for Context {
    fn write(mut self, value: TemporalFilterCtx) -> Result<Self, Error> {
        self.fine_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx, TemporalFilterCtx> for Context {
    fn read(&self) -> &TemporalFilterCtx {
        &self.fine_scan.temporal_filter
    }
}
//
//
impl ContextWrite<(), FineUnionCtx> for Context {
    fn write(mut self, value: FineUnionCtx) -> Result<Self, Error> {
        self.fine_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<(), FineUnionCtx> for Context {
    fn read(&self) -> &FineUnionCtx {
        &self.fine_scan.union
    }
}
//
//
impl ContextWrite<(), FineContoursCtx> for Context {
    fn write(mut self, value: FineContoursCtx) -> Result<Self, Error> {
        self.fine_scan.fine_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<(), FineContoursCtx> for Context {
    fn read(&self) -> &FineContoursCtx {
        &self.fine_scan.fine_contours
    }
}

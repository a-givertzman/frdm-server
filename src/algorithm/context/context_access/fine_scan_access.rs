use crate::{
    algorithm::{
        FineContoursCtx, FineScanCtx, FineUnionCtx, GeometryDefectCtx, RopeDimensionsCtx, TemporalFilterCtx, WidthEmissionsCtx, FineEdgesCtx,
        Context, ContextRead, ContextWrite,
    },
    domain::Error,
};

//
//
impl ContextWrite<WidthEmissionsCtx<FineScanCtx>> for Context {
    fn write(mut self, value: WidthEmissionsCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<WidthEmissionsCtx<FineScanCtx>> for Context {
    fn read(&self) -> &WidthEmissionsCtx<FineScanCtx> {
        &self.fine_scan.width_emissions
    }
}
//
//
impl ContextWrite<GeometryDefectCtx<FineScanCtx>> for Context {
    fn write(mut self, value: GeometryDefectCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<GeometryDefectCtx<FineScanCtx>> for Context {
    fn read(&self) -> &GeometryDefectCtx<FineScanCtx> {
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

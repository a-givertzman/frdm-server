use crate::{
    algorithm::{
        FastContoursCtx, FastEdgesCtx, FastScanCtx, FastUnionCtx, GeometryDefectCtx,
        ResultCtx, RopeDimensionsCtx, TemporalFilterCtx, WidthEmissionsCtx,
    },
    domain::{Error, Image}, Context, ContextRead, ContextWrite,
};

//
//
impl ContextWrite<(), FastContoursCtx> for Context {
    fn write(mut self, value: FastContoursCtx) -> Result<Self, Error> {
        self.fast_scan.fast_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<(), FastContoursCtx> for Context {
    fn read(&self) -> &FastContoursCtx {
        &self.fast_scan.fast_contours
    }
}
//
//
impl ContextWrite<(), FastEdgesCtx> for Context {
    fn write(mut self, value: FastEdgesCtx) -> Result<Self, Error> {
        self.fast_scan.edges = value;
        Result::Ok(self)
    }
}
impl ContextRead<(), FastEdgesCtx> for Context {
    fn read(&self) -> &FastEdgesCtx {
        &self.fast_scan.edges
    }
}
//
//
impl ContextWrite<FastScanCtx, WidthEmissionsCtx> for Context {
    fn write(mut self, value: WidthEmissionsCtx) -> Result<Self, Error> {
        self.fast_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx, WidthEmissionsCtx> for Context {
    fn read(&self) -> &WidthEmissionsCtx {
        &self.fast_scan.width_emissions
    }
}
//
//
impl ContextWrite<FastScanCtx, GeometryDefectCtx> for Context {
    fn write(mut self, value: GeometryDefectCtx) -> Result<Self, Error> {
        self.fast_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx, GeometryDefectCtx> for Context {
    fn read(&self) -> &GeometryDefectCtx {
        &self.fast_scan.defects
    }
}
//
//
impl ContextWrite<FastScanCtx, ResultCtx<Image>> for Context {
    fn write(mut self, value: ResultCtx<Image>) -> Result<Self, Error> {
        self.fast_scan.result = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx, ResultCtx<Image>> for Context {
    fn read(&self) -> &ResultCtx<Image> {
        &self.fast_scan.result
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
impl ContextWrite<FastScanCtx, RopeDimensionsCtx> for Context {
    fn write(mut self, value: RopeDimensionsCtx) -> Result<Self, Error> {
        self.fast_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx, RopeDimensionsCtx> for Context {
    fn read(&self) -> &RopeDimensionsCtx {
        &self.fast_scan.rope_dimensions
    }
}
//
//
impl ContextWrite<FastScanCtx, TemporalFilterCtx> for Context {
    fn write(mut self, value: TemporalFilterCtx) -> Result<Self, Error> {
        self.fast_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx, TemporalFilterCtx> for Context {
    fn read(&self) -> &TemporalFilterCtx {
        &self.fast_scan.temporal_filter
    }
}
//
//
impl ContextWrite<(), FastUnionCtx> for Context {
    fn write(mut self, value: FastUnionCtx) -> Result<Self, Error> {
        self.fast_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<(), FastUnionCtx> for Context {
    fn read(&self) -> &FastUnionCtx {
        &self.fast_scan.union
    }
}

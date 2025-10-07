use super::{context::Context};
use crate::{
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx},
        geometry_defect::GeometryDefectCtx, width_emissions::WidthEmissionsCtx,
        CroppingCtx, DetectingContoursCvCtx, EdgeDetectionCtx, GrayCtx, GaussianBlurCtx,
        InitialCtx, ResultCtx, RopeDimensionsCtx,
    },
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
impl ContextWrite<DetectingContoursCvCtx> for Context {
    fn write(mut self, value: DetectingContoursCvCtx) -> Result<Self, Error> {
        self.detecting_contours_cv = value;
        Result::Ok(self)
    }
}
impl ContextRead<DetectingContoursCvCtx> for Context {
    fn read(&self) -> &DetectingContoursCvCtx {
        &self.detecting_contours_cv
    }
}
//
//
impl ContextWrite<CroppingCtx> for Context {
    fn write(mut self, value: CroppingCtx) -> Result<Self, Error> {
        self.cropping = value;
        Result::Ok(self)
    }
}
impl ContextRead<CroppingCtx> for Context {
    fn read(&self) -> &CroppingCtx {
        &self.cropping
    }
}
//
//
impl ContextWrite<AutoGammaCtx> for Context {
    fn write(mut self, value: AutoGammaCtx) -> Result<Self, Error> {
        self.auto_gamma = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoGammaCtx> for Context {
    fn read(&self) -> &AutoGammaCtx {
        &self.auto_gamma
    }
}
//
//
impl ContextWrite<AutoBrightnessAndContrastCtx> for Context {
    fn write(mut self, value: AutoBrightnessAndContrastCtx) -> Result<Self, Error> {
        self.auto_brightness_and_contrast = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoBrightnessAndContrastCtx> for Context {
    fn read(&self) -> &AutoBrightnessAndContrastCtx {
        &self.auto_brightness_and_contrast
    }
}
//
//
impl ContextWrite<EdgeDetectionCtx> for Context {
    fn write(mut self, value: EdgeDetectionCtx) -> Result<Self, Error> {
        self.edge_detection = value;
        Result::Ok(self)
    }
}
impl ContextRead<EdgeDetectionCtx> for Context {
    fn read(&self) -> &EdgeDetectionCtx {
        &self.edge_detection
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
//
//
impl ContextWrite<ResultCtx> for Context {
    fn write(mut self, value: ResultCtx) -> Result<Self, Error> {
        self.result = value;
        Result::Ok(self)
    }
}
impl ContextRead<ResultCtx> for Context {
    fn read(&self) -> &ResultCtx {
        &self.result
    }
}
//
//
impl ContextWrite<GrayCtx> for Context {
    fn write(mut self, value: GrayCtx) -> Result<Self, Error> {
        self.gray = value;
        Result::Ok(self)
    }
}
impl ContextRead<GrayCtx> for Context {
    fn read(&self) -> &GrayCtx {
        &self.gray
    }
}
//
//
impl ContextWrite<GaussianBlurCtx> for Context {
    fn write(mut self, value: GaussianBlurCtx) -> Result<Self, Error> {
        self.gaussian_blur = value;
        Result::Ok(self)
    }
}
impl ContextRead<GaussianBlurCtx> for Context {
    fn read(&self) -> &GaussianBlurCtx {
        &self.gaussian_blur
    }
}
//
//
impl ContextWrite<RopeDimensionsCtx> for Context {
    fn write(mut self, value: RopeDimensionsCtx) -> Result<Self, Error> {
        self.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDimensionsCtx> for Context {
    fn read(&self) -> &RopeDimensionsCtx {
        &self.rope_dimensions
    }
}

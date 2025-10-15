use crate::{
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx}, CroppingCtx, GrayCtx,
        Context, ContextRead, ContextWrite,
    }, domain::Error,
};

//
//
impl ContextWrite<CroppingCtx> for Context {
    fn write(mut self, value: CroppingCtx) -> Result<Self, Error> {
        self.normalized.cropping = value;
        Result::Ok(self)
    }
}
impl ContextRead<CroppingCtx> for Context {
    fn read(&self) -> &CroppingCtx {
        &self.normalized.cropping
    }
}
//
//
impl ContextWrite<AutoGammaCtx> for Context {
    fn write(mut self, value: AutoGammaCtx) -> Result<Self, Error> {
        self.normalized.auto_gamma = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoGammaCtx> for Context {
    fn read(&self) -> &AutoGammaCtx {
        &self.normalized.auto_gamma
    }
}
//
//
impl ContextWrite<AutoBrightnessAndContrastCtx> for Context {
    fn write(mut self, value: AutoBrightnessAndContrastCtx) -> Result<Self, Error> {
        self.normalized.auto_brightness_and_contrast = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoBrightnessAndContrastCtx> for Context {
    fn read(&self) -> &AutoBrightnessAndContrastCtx {
        &self.normalized.auto_brightness_and_contrast
    }
}
//
//
impl ContextWrite<GrayCtx> for Context {
    fn write(mut self, value: GrayCtx) -> Result<Self, Error> {
        self.normalized.gray = value;
        Result::Ok(self)
    }
}
impl ContextRead<GrayCtx> for Context {
    fn read(&self) -> &GrayCtx {
        &self.normalized.gray
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

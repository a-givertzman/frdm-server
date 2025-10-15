use crate::{
    algorithm::{
        GeometryDefectType, InitialCtx, ResultCtx, Context, ContextRead, ContextWrite,
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
impl ContextWrite<ResultCtx<Vec<GeometryDefectType>>> for Context {
    fn write(mut self, value: ResultCtx<Vec<GeometryDefectType>>) -> Result<Self, Error> {
        self.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<ResultCtx<Vec<GeometryDefectType>>> for Context {
    fn read(&self) -> &ResultCtx<Vec<GeometryDefectType>> {
        &self.defects
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

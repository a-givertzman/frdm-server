use std::{cell::RefCell, time::Instant};
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, ResultCtx, FilterHighPass},
    domain::{Eval, Filter, Image},
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter {
    amplify_factor: f64,
    reduce_factor: f64,
    threshold: f64,
    filters: RefCell<Vec<Box<dyn Filter<Item = u8>>>>,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    pub fn new(amplify_factor: f64, reduce_factor: f64, threshold: f64, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            amplify_factor,
            reduce_factor,
            threshold,
            filters: RefCell::new(vec![]),
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for TemporalFilter {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("TemporalFilter", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                let mut result = frame.mat.data_bytes().unwrap().to_vec();
                let height = frame.mat.rows();
                let width = frame.mat.cols();
                let pixels = (width * height) as usize;
                if self.filters.borrow().is_empty() {
                    fn boxed_filter<'a, T>(f: impl Filter<Item = T> + 'a) -> Box<dyn Filter<Item = T> + 'a> {
                        Box::new(f)
                    }
                    *self.filters.borrow_mut() = (0..pixels).map(|_| {
                        boxed_filter(FilterHighPass::<u8>::new(None, 1.0))
                    }).collect();
                }
                let mat = frame.mat.data_bytes().unwrap();
                let mut filters = self.filters.borrow_mut();
                for (i, pixel) in mat.iter().enumerate() {
                    match filters.get_mut(i) {
                        Some(filter) => {
                            if let Some(value) = filter.add(*pixel) {
                                match result.get_mut(i) {
                                    Some(r) => *r = value,
                                    None => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                                }
                            }
                        }
                        None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                    }
                }
                match Mat::new_nd_with_data(&[width, height], &result) {
                    Ok(result) => {
                        let result = ResultCtx { frame: Image::with(result.clone_pointee()) };
                        log::debug!("DetectingContoursCv.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

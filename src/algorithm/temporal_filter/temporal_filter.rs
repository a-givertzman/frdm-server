use std::{cell::RefCell, fs::OpenOptions, path::Path, time::Instant};
use opencv::core::{MatTrait, MatTraitConst};
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
    filters: RefCell<Vec<FilterHighPass::<u8>>>,
    cache_path: String,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `cache` - path to the cache folder
    pub fn new(amplify_factor: f64, reduce_factor: f64, threshold: f64, cache: impl Into<String>, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            amplify_factor,
            reduce_factor,
            threshold,
            filters: RefCell::new(vec![]),
            cache_path: cache.into(),
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Wraps specified `f` into the `Box`
    fn boxed_filter<'a, T>(f: impl Filter<Item = T> + 'a) -> Box<dyn Filter<Item = T> + 'a> {
        Box::new(f)
    }
    ///
    /// Loads filters initial rates from the cache
    fn load_cache(&self) -> Option<Vec<FilterHighPass::<u8>>> {
        let f = OpenOptions::new()
            .read(true)
            .open(&self.cache_path);
        match f {
            Ok(f) => {
                match serde_json::from_reader(f) {
                    Ok(rates) => {
                        let rates: Vec<f64> = rates;
                        Some(rates.into_iter().map(|rate| {
                            FilterHighPass::<u8>::new(None, Some(rate), self.amplify_factor, self.reduce_factor, self.threshold)
                        }).collect())
                    }
                    Err(err) => {
                        log::warn!("TemporalFilter.load_cache | Can't parse rates from json: {:?}", err);
                        None
                    }
                }
            }
            Err(err) => {
                log::warn!("TemporalFilter.load_cache | Can't read rates from cache '{}': {:?}", self.cache_path, err);
                None
            }
        }
    }
    ///
    /// Stores filters initial rates from the cache
    fn store_cache(&self) {
        let path = Path::new(&self.cache_path);
        let path_exists = match path.is_dir() {
            true => true,
            false => {
                match std::fs::create_dir_all(path) {
                    Ok(_) => true,
                    Err(err) => {
                        log::warn!("TemporalFilter.load_cache | Can't create cache folder'{}', error: {:?}", self.cache_path, err);
                        false
                    },
                }
            }
        };
        if path_exists {
            let f = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path);
            match f {
                Ok(f) => {
                    let rates: Vec<f64> = self.filters.borrow().iter().map(|f| f.rate()).collect();
                    if let Err(err) = serde_json::to_writer(f, &rates) {
                        log::warn!("TemporalFilter.load_cache | Can't serialize rates[{}] {:?}..., error: {:?}", rates.len(), &rates[..4], err);
                    }
                }
                Err(err) => {
                    log::warn!("TemporalFilter.load_cache | Can't open file '{}' to write rates, error: {:?}", self.cache_path, err);
                }
            }
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
                let mut out = frame.mat.clone();
                let height = frame.mat.rows();
                let width = frame.mat.cols();
                let pixels = (width * height * frame.mat.channels()) as i32;
                log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                if self.filters.borrow().is_empty() {
                    match self.load_cache() {
                        Some(filters) => *self.filters.borrow_mut() = filters,
                        None => {
                            *self.filters.borrow_mut() = (0..pixels).map(|_| {
                                FilterHighPass::<u8>::new(None, None, self.amplify_factor, self.reduce_factor, self.threshold)
                            }).collect();
                        }
                    }
                }
                log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                let mut filters = self.filters.borrow_mut();
                for i in 0..pixels {
                    let pixel = frame.mat.at(i).unwrap();
                    match filters.get_mut(i as usize) {
                        Some(filter) => {
                            if let Some(value) = filter.add(*pixel) {
                                match out.at_mut(i) {
                                    Ok(r) => *r = value,
                                    Err(_) => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                                }
                            }
                        }
                        None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                    }
                }
                self.store_cache();
                let result = ResultCtx { frame: Image::with(out) };
                log::debug!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

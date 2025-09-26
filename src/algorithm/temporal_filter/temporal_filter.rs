use std::{cell::RefCell, time::Instant};
use opencv::core::{Mat, MatTrait, MatTraitConst, MatTraitConstManual};
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
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `cache` - path to the cache folder
    pub fn new(amplify_factor: f64, reduce_factor: f64, threshold: f64, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            amplify_factor,
            reduce_factor,
            threshold,
            filters: RefCell::new(vec![]),
            ctx: Box::new(ctx),
        }
    }
    // ///
    // /// Wraps specified `f` into the `Box`
    // fn boxed_filter<'a, T>(f: impl Filter<Item = T> + 'a) -> Box<dyn Filter<Item = T> + 'a> {
    //     Box::new(f)
    // }
    // ///
    // /// Loads filters initial rates from the cache
    // fn load_cache(&self) -> Option<Vec<FilterHighPass::<u8>>> {
    //     let path = Path::new(&self.cache_path).join("rates.json");
    //     let f = OpenOptions::new()
    //         .read(true)
    //         .open(&path);
    //     match f {
    //         Ok(f) => {
    //             match serde_json::from_reader(f) {
    //                 Ok(rates) => {
    //                     let rates: Vec<i8> = rates;
    //                     Some(rates.into_iter().map(|rate| {
    //                         FilterHighPass::<u8>::new(None, Some(rate), self.amplify_factor, self.reduce_factor, self.threshold)
    //                     }).collect())
    //                 }
    //                 Err(err) => {
    //                     log::warn!("TemporalFilter.load_cache | Can't parse rates from json: {:?}", err);
    //                     None
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             log::warn!("TemporalFilter.load_cache | Can't read rates from cache '{}': {:?}", path.display(), err);
    //             None
    //         }
    //     }
    // }
    // ///
    // /// Stores filters initial rates from the cache
    // fn store_cache(&self) {
    //     let dir = Path::new(&self.cache_path);
    //     let path = dir.join("rates.json");
    //     let path_exists = match dir.is_dir() {
    //         true => true,
    //         false => {
    //             match std::fs::create_dir_all(&dir) {
    //                 Ok(_) => true,
    //                 Err(err) => {
    //                     log::warn!("TemporalFilter.load_cache | Can't create cache folder'{}', error: {:?}", dir.display(), err);
    //                     false
    //                 },
    //             }
    //         }
    //     };
    //     if path_exists {
    //         let f = OpenOptions::new()
    //             .write(true)
    //             .create(true)
    //             .truncate(true)
    //             .open(&path);
    //         match f {
    //             Ok(f) => {
    //                 let rates: Vec<i8> = self.filters.borrow().iter().map(|f| f.rate()).collect();
    //                 if let Err(err) = serde_json::to_writer(f, &rates) {
    //                     log::warn!("TemporalFilter.load_cache | Can't serialize rates[{}] {:?}..., error: {:?}", rates.len(), &rates[..4], err);
    //                 }
    //             }
    //             Err(err) => {
    //                 log::warn!("TemporalFilter.load_cache | Can't open file '{}' to write rates, error: {:?}", path.display(), err);
    //             }
    //         }
    //     }
    // }
    fn blure(&self, mut img: Mat, width: usize, height: usize, weight: f32) -> Result<Mat, Error> {
        let error = Error::new("TemporalFilter", "blure");
        let pixels = width * height * img.channels() as usize;
        let input = img.data_bytes().unwrap().to_owned();
        let filters = self.filters.borrow();
        for i in 0..pixels {
            let y = (i as f32 / width as f32).trunc() as usize;
            let x = i - y * width;
            match filters.get(i as usize) {
                Some(filter) => {
                    if filter.rate() < 0.0 {
                        let pixel = *(input.get(i).unwrap()) as f32;
                        let pixel_tl = match x > 0 && y > 0 {
                            true => (*(input.get(i - width - 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_tp = match y > 0 {
                            true => (*(input.get(i - width).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_tr = match x < (width - 1) && y > 0 {
                            true => (*(input.get(i - width + 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_bl = match x > 0 && y < (height - 1) {
                            true => (*(input.get(width + i - 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_bm = match y < (height - 1) {
                            true => (*(input.get(width + i).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_br = match x < (width - 1) && y < (height - 1) {
                            true => (*(input.get(width + i + 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_lt = match x > 0 {
                            true => (*(input.get(i - 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let pixel_rt = match x < (width - 1) {
                            true => (*(input.get(i + 1).unwrap()) as f32) * weight,
                            false => pixel * weight,
                        };
                        let average = (pixel + pixel_tl + pixel_tp + pixel_tr + pixel_rt + pixel_br + pixel_bm + pixel_bl + pixel_lt) / (1.0 + 8.0 * weight);
                        match img.at_mut(i as i32) {
                            Ok(r) => *r = average.round() as u8,
                            Err(_) => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                        }
                    }
                }
                None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
            }
        }
        Ok(img)
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
                let input = frame.mat.data_bytes().unwrap();
                let mut out = frame.mat.clone();
                let height = frame.mat.rows() as usize;
                let width = frame.mat.cols() as usize;
                let pixels = width * height * frame.mat.channels() as usize;
                log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                if self.filters.borrow().is_empty() {
                    *self.filters.borrow_mut() = (0..pixels).map(|_| {
                        FilterHighPass::<u8>::new(None, self.amplify_factor, self.reduce_factor, self.threshold)
                    }).collect();
                }
                log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                {
                    let mut filters = self.filters.borrow_mut();
                    for i in 0..pixels {
                        let pixel = input.get(i).unwrap();
                        match filters.get_mut(i as usize) {
                            Some(filter) => {
                                if let Some(value) = filter.add(*pixel) {
                                    match out.at_mut(i as i32) {
                                        Ok(r) => *r = value,
                                        Err(_) => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                                    }
                                }
                            }
                            None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                        }
                    }
                }
                let blure_weight = 0.64;
                // let out = self.blure(out, width, height, blure_weight)?;
                let result = ResultCtx { frame: Image::with(out) };
                log::debug!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

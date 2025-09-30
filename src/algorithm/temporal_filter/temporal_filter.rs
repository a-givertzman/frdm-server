use std::{cell::RefCell, time::Instant};
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Point2i, Size2i};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, ResultCtx, FilterHighPass},
    domain::{Eval, Filter, Image},
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter {
    amplify_factor: f64,
    grow_speed: f64,
    reduce_factor: f64,
    down_speed: f64,
    threshold: f64,
    filters: RefCell<Vec<FilterHighPass::<u8>>>,
    // background: RefCell<Mat>,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `cache` - path to the cache folder
    pub fn new(amplify_factor: f64, grow_speed: f64, reduce_factor: f64, down_speed: f64, threshold: f64, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            amplify_factor,
            grow_speed,
            reduce_factor,
            down_speed,
            threshold,
            filters: RefCell::new(vec![]),
            // background: RefCell::new(Mat::default()),
            ctx: Box::new(ctx),
        }
    }
    // ///
    // /// 
    // fn blure(&self, mut img: Mat, width: usize, height: usize, weight: f32) -> Result<Mat, Error> {
    //     let error = Error::new("TemporalFilter", "blure");
    //     let pixels = width * height * img.channels() as usize;
    //     let input = img.data_bytes().unwrap().to_owned();
    //     let filters = self.filters.borrow();
    //     for i in 0..pixels {
    //         let y = (i as f32 / width as f32).trunc() as usize;
    //         let x = i - y * width;
    //         match filters.get(i as usize) {
    //             Some(filter) => {
    //                 if filter.rate() < 0.0 {
    //                     let pixel = *(input.get(i).unwrap()) as f32;
    //                     let pixel_tl = match x > 0 && y > 0 {
    //                         true => (*(input.get(i - width - 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_tp = match y > 0 {
    //                         true => (*(input.get(i - width).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_tr = match x < (width - 1) && y > 0 {
    //                         true => (*(input.get(i - width + 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_bl = match x > 0 && y < (height - 1) {
    //                         true => (*(input.get(width + i - 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_bm = match y < (height - 1) {
    //                         true => (*(input.get(width + i).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_br = match x < (width - 1) && y < (height - 1) {
    //                         true => (*(input.get(width + i + 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_lt = match x > 0 {
    //                         true => (*(input.get(i - 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let pixel_rt = match x < (width - 1) {
    //                         true => (*(input.get(i + 1).unwrap()) as f32) * weight,
    //                         false => pixel * weight,
    //                     };
    //                     let average = (pixel + pixel_tl + pixel_tp + pixel_tr + pixel_rt + pixel_br + pixel_bm + pixel_bl + pixel_lt) / (1.0 + 8.0 * weight);
    //                     match img.at_mut(i as i32) {
    //                         Ok(r) => *r = average.round() as u8,
    //                         Err(_) => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
    //                     }
    //                 }
    //             }
    //             None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
    //         }
    //     }
    //     Ok(img)
    // }
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
                match frame.mat.data_bytes() {
                    Ok(input) => {
                        let height = frame.mat.rows() as usize;
                        let width = frame.mat.cols() as usize;
                        let pixels = width * height * frame.mat.channels() as usize;
                        let mut out = vec![0u8; pixels];
                        log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                        if self.filters.borrow().is_empty() {
                            *self.filters.borrow_mut() = (0..pixels).map(|_| {
                                FilterHighPass::<u8>::new(None, self.amplify_factor, self.grow_speed, self.reduce_factor, self.down_speed, self.threshold)
                            }).collect();
                            // *self.background.borrow_mut() = unsafe { Mat::new_rows_cols(height as i32, width as i32, opencv::core::CV_8UC1).unwrap() };
                        }
                        log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                        {
                            let mut filters = self.filters.borrow_mut();
                            for i in 0..pixels {
                                match input.get(i) {
                                    Some(value) => {
                                        if let Some(filter) = filters.get_mut(i) {
                                            filter.add(*value);
                                            match out.get_mut(i) {
                                                Some(pixel) => *pixel = match filter.rate() > 0.0 {
                                                    true => 255,
                                                    false => 0,
                                                },
                                                None => return Err(error.err(format!("Out image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                                            }
                                        }
                                    }
                                    None => return Err(error.err(format!("Input image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                                }
                            }
                        }
                        // let blure_weight = 0.64;
                        // let out = self.blure(out, width, height, blure_weight)?;
                        let out = unsafe { Mat::new_rows_cols_with_data_unsafe(
                            height as i32,
                            width as i32,
                            opencv::core::CV_8UC1,
                            out.as_ptr() as *mut std::ffi::c_void,
                            opencv::core::Mat_AUTO_STEP,
                        ) }.map_err(|err| error.pass(err.to_string()))?;
                        let kernel = opencv::imgproc::get_structuring_element(opencv::imgproc::MORPH_ELLIPSE, Size2i::new(3, 3), Point2i::new(-1, -1)).unwrap();
                        let mut dst = Mat::default();
                        opencv::imgproc::morphology_ex(
                            &out,
                            &mut dst,
                            opencv::imgproc::MORPH_OPEN,
                            &kernel,
                            Point2i::new(-1, -1),
                            1,
                            opencv::core::BORDER_CONSTANT,
                            opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                        ).map_err(|err| error.pass(err.to_string()))?;
                        let result = ResultCtx { frame: Image::with(dst) };
                        log::debug!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
                // {
                //     let mut filters = self.filters.borrow_mut();
                //     let mut background = self.background.borrow_mut();
                //     for i in 0..pixels {
                //         let pixel = background.at_mut(i as i32).unwrap();
                //         let value: &u8 = frame.mat.at(i as i32).unwrap();
                //         if let Some(filter) = filters.get_mut(i) {
                //             _ = filter.add(*value);
                //             *pixel = ((*pixel as f32 + *value as f32) * 0.5 * filter.rate() * 6.0).round() as u8;
                //         }
                //     }
                // }
                // opencv::core::subtract(&frame.mat, &*self.background.borrow(), &mut out, &Vector::<u8>::new(), -1).unwrap();
                // {
                //     let mut filters = self.filters.borrow_mut();
                //     for i in 0..pixels {
                //         let pixel = input.get(i).unwrap();
                //         match filters.get_mut(i as usize) {
                //             Some(filter) => {
                //                 if let Some(value) = filter.add(*pixel) {
                //                     match out.at_mut(i as i32) {
                //                         Ok(r) => *r = value,
                //                         Err(_) => return Err(error.err(format!("Output image format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                //                     }
                //                 }
                //             }
                //             None => return Err(error.err(format!("Filters matrix format error, index [{i}] out of range {width} x {height} = {pixels}"))),
                //         }
                //     }
                // }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}

use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;
use opencv::core::Mat;
use opencv::core::MatTraitConst;
use opencv::core::Point;
use opencv::core::Point2f;
use opencv::core::Point2i;
use opencv::core::Point2l;
use opencv::core::Size2i;
use opencv::core::VectorToVec;
use opencv::imgproc;
use opencv::core;
use opencv::imgproc::LineTypes;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite, ContextRead,
    CvContoursCtx, FineContoursCtx,
    GrayCtx, EvalResult, ResultCtx,
};
use crate::conf::DetectingContoursConf;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return filtered and binarised [Image] with contours detected
pub struct FineContours {
    conf: DetectingContoursConf,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl FineContours {
    ///
    /// Returns [FineContours] new instance
    /// - `ctx` - Prevouse step returns [Image] in [Context]
    /// - `conf` - Configuration for `Contour dectection` algorithm:
    ///     - gausian:
    ///         - `kernel` - Gausian blur kernel size
    ///         - `sigma_x` - Standard deviation in X direction
    ///         - `sigma_y` - Standard deviation in Y direction
    ///     - sobel:
    ///         - `kernel_size` - Sobel kernel size
    ///         - `scale` - Scale factor for computed derivative values
    ///         - `delta` - Delta values added to results
    ///     - overlay:
    ///         - `src1-weight` - Weight for X gradient
    ///         - `src1-weight` - Weight for Y gradient
    ///         - `gamma` - Scalar added to weighted sum
    pub fn new(conf: DetectingContoursConf, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self { 
            conf,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Increments `i` upwars to `len - 1`, then returns to `0`
    fn inc(i: usize, len: usize) -> usize {
        if i >= (len - 1) {
            0
        } else {
            i + 1
        }
    }
    ///
    /// Decrements `i` downwards to `0`, then returns to `len - 1`
    fn dec(i: usize, len: usize) -> usize {
        if i == 0 {
            len - 1
        } else {
            i - 1
        }
    }
    ///
    /// Returns distance between points
    fn distance(pt1: &Point, pt2: &Point) -> f32 {
        ((pt2.x - pt1.x).pow(2) as f32 + (pt2.y - pt1.y).pow(2)  as f32).sqrt()
    }
    ///
    /// Returns minimum distance between contours
    fn min_distance(contour1: &core::Vector<Point>, contour2: &core::Vector<Point>, threshold: f32) -> Result<core::Vector<Point>, Error> {
        let error = Error::new("FineContours", "min_distance");
        if contour1.len() < 4 || contour2.len() < 4 {
            log::warn!("FineContours.min_distance | c1[{}], c2[{}]", contour1.len(), contour2.len());
            return Err(error.err("c1 and c2 can't be length < 3"))
        }
        // let mut dst = Mat::default();
        let mut found = false;
        let mut min = (0, 0, threshold);
        for (i1, pt1) in contour1.iter().enumerate() {
            for (i2, pt2) in contour2.iter().enumerate() {
                let distance = Self::distance(&pt1, &pt2);
                if distance <= min.2 {
                    min = (i1, i2, distance);
                    // imgproc::cvt_color(img, &mut dst, imgproc::COLOR_GRAY2BGRA, 4)
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::line(&mut dst, pt1, pt2, core::Vec4d::from_array([0.0, 0.0, 128.0, 128.0]), 1, LineTypes::LINE_8 as i32, 0)
                    // // imgproc::polylines(&dst, &hull, true, core::Vec4d::from_array([128.0, 128.0, 128.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // opencv::highgui::imshow("Contours", &dst).unwrap();
                    // opencv::highgui::wait_key(1).unwrap();
                    // imgproc::fill_convex_poly(&mut thresh, &hull, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0)
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // log::debug!("FineContours.eval | contour: {:?}", contour);
                    found = true;
                }
            }
        }
        if !found {
            return Err(error.err("Can't merge"));
        }
        let mut c1_remove = std::collections::HashSet::new();
        let mut c2_remove = std::collections::HashSet::new();
        let mut i1 = min.0;
        let mut i2 = min.1;
        // log::debug!("FineContours.min_distance | pair: {i1}, {i2}");
        let mut count = [contour1.len(), contour2.len()].iter().min().map(|m| *m).unwrap_or(contour1.len()) / 4;
        while count > 0 {
            // log::debug!("FineContours.min_distance | pair: {i1}, {i2}");
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance(&pt1, &pt2) <= threshold {
                // imgproc::cvt_color(img, &mut dst, imgproc::COLOR_GRAY2BGRA, 4)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // imgproc::line(&mut dst, pt1, pt2, core::Vec4d::from_array([0.0, 255.0, 0.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::highgui::imshow("Contours", &dst).unwrap();
                // opencv::highgui::wait_key(1).unwrap();
                c1_remove.insert(i1);
                c2_remove.insert(i2);
                i1 = Self::dec(i1, contour1.len());
                i2 = Self::inc(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_end = Self::dec(i1, contour1.len());
        let c2_start = Self::inc(i2, contour2.len());
        // let mut ins = Self::inc(i1, contour1.len());
        // let mut get = i2;
        let mut i1 = min.0;
        let mut i2 = min.1;
        // log::debug!("FineContours.min_distance | i1_remove[{}]: {:?}", i1_remove.len(), i1_remove);
        // log::debug!("FineContours.min_distance | i2_remove[{}]: {:?}", i2_remove.len(), i2_remove);
        let mut count = [contour1.len(), contour2.len()].iter().min().map(|m| *m).unwrap_or(contour1.len()) / 4;
        while count > 0 {
            // log::debug!("FineContours.min_distance | pair: {i1}, {i2}");
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance(&pt1, &pt2) <= threshold {
                // imgproc::cvt_color(img, &mut dst, imgproc::COLOR_GRAY2BGRA, 4)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // imgproc::line(&mut dst, pt1, pt2, core::Vec4d::from_array([0.0, 255.0, 0.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::highgui::imshow("Contours", &dst).unwrap();
                // opencv::highgui::wait_key(1).unwrap();
                c1_remove.insert(i1);
                c2_remove.insert(i2);
                i1 = Self::inc(i1, contour1.len());
                i2 = Self::dec(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_start = Self::inc(i1, contour1.len());
        let c2_end = Self::dec(i2, contour2.len());

        // log::debug!("FineContours.min_distance | i1_remove[{}]: {:?}", i1_remove.len(), i1_remove);
        // log::debug!("FineContours.min_distance | i2_remove[{}]: {:?}", i2_remove.len(), i2_remove);
        // log::debug!("FineContours.min_distance | c1: {},  i1_remove: {}", c1.len(), i1_remove.len());
        // log::debug!("FineContours.min_distance | ins: {ins}");
        let mut c: core::Vector<Point> = core::Vector::default();
        let mut i = c1_start;
        while i != c1_end {
            c.push(contour1.get(i).unwrap());
            i = Self::inc(i, contour1.len());
        }
        let mut i = c2_start;
        while i != c2_end {
            c.push(contour2.get(i).unwrap());
            i = Self::inc(i, contour2.len());
        }
        Ok(c)
    }
}
//
//
impl Eval<Image, EvalResult> for FineContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineContours", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                // let result: &ResultCtx = ctx.read();
                let result: &GrayCtx = ctx.read();
                let frame = &result.frame;
                // let mut dst = Mat::default();
                // opencv::imgproc::sobel(&frame.mat, &mut sobelx, core::CV_8U, 1, 0, 3, 1.0, 0.0, core::BORDER_DEFAULT)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::imgproc::sobel(&frame.mat, &mut sobely, core::CV_8U, 1, 0, 3, 1.0, 0.0, core::BORDER_DEFAULT)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::core::add_weighted_def(&sobelx, 0.99, &sobely, 0.99, 0.0, &mut dst)
                //     .map_err(|err| error.pass(err.to_string()))?;
                let mut dst = Mat::default();
                imgproc::gaussian_blur(&frame.mat, &mut dst, Size2i::new(11, 11), 0.0, 0.0, opencv::core::BORDER_DEFAULT)
                    .map_err(|err| error.pass(err.to_string()))?;
                opencv::imgproc::laplacian(&dst.clone(), &mut dst, opencv::core::CV_8UC1, 5, 1.0, 0.0, opencv::core::BorderTypes::BORDER_REFLECT_101 as i32)
                    .map_err(|err| error.pass(err.to_string()))?;
                imgproc::gaussian_blur(&dst.clone(), &mut dst, Size2i::new(13, 13), 0.0, 0.0, opencv::core::BORDER_DEFAULT)
                    .map_err(|err| error.pass(err.to_string()))?;

                let kernel = opencv::imgproc::get_structuring_element(opencv::imgproc::MORPH_ELLIPSE, core::Size2i::new(5, 5), core::Point2i::new(-1, -1)).unwrap();
                let mut deleted = core::Mat::default();
                opencv::imgproc::morphology_ex(
                    &dst,
                    &mut deleted,
                    opencv::imgproc::MORPH_OPEN,
                    &kernel,
                    core::Point2i::new(-1, -1),
                    2,
                    opencv::core::BORDER_CONSTANT,
                    opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                ).map_err(|err| error.pass(err.to_string()))?;
                let mut thresh = core::Mat::default();
                let threshold = opencv::imgproc::threshold(&deleted, &mut thresh, 8.0, 255.0, opencv::imgproc::ThresholdTypes::THRESH_OTSU as i32)
                    .map_err(|err| error.pass(err.to_string()))?;
                opencv::imgproc::threshold(&deleted, &mut thresh, threshold * 0.4, 255.0, opencv::imgproc::ThresholdTypes::THRESH_BINARY as i32)
                    .map_err(|err| error.pass(err.to_string()))?;
                let mut contours: core::Vector<core::Vector<Point>> = core::Vector::default();
                log::debug!("FineContours.eval | contours...");
                imgproc::find_contours(
                    &thresh,
                    &mut contours,
                    imgproc::RetrievalModes::RETR_EXTERNAL as i32,
                    imgproc::ContourApproximationModes::CHAIN_APPROX_SIMPLE as i32,
                    core::Point2i::new(0, 0),
                ).map_err(|err| error.pass(err.to_string()))?;
                // contours = core::Vector::from_iter([
                //     core::Vector::from_iter([Point2i::new(10, 10), Point2i::new(500, 10), Point2i::new(500, 500), Point2i::new(10, 500)]),
                // ]);
                let mut contours = VecDeque::from_iter(contours.iter().filter(|c| c.len() > 3));
                let mut count = 1;
                while count > 0 {
                    let mut found = None;
                    'contour1: for (i1, contour1) in contours.iter().enumerate() {
                        for (i2, contour2) in contours.iter().enumerate().filter(|(i, _)| i1 != *i) {
                            // log::debug!("FineContours.eval | search nierby segments...");
                            if let Ok(hull) = Self::min_distance(&contour1, &contour2, 24.0) {
                                if hull.len() > 0 {
                                //             let mut hull: core::Vector<Point> = core::Vector::default();
                                            // imgproc::approx_poly_dp(&hull.clone(), &mut hull, 0.4, true)
                                            //     .map_err(|err| error.pass(err.to_string()))?;
                                //             // imgproc::convex_hull(&points, &mut hull, true, true)
                                //             //     .map_err(|err| error.pass(err.to_string()))?;
                                //             // log::debug!("FineContours.eval | hull: {:?}", hull);
                                            // imgproc::polylines(&mut thresh, &hull, true, core::Vec4d::from_array([128.0, 128.0, 128.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                                            //     .map_err(|err| error.pass(err.to_string()))?;
                                            // imgproc::fill_convex_poly(&mut thresh, &hull, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0)
                                            // imgproc::fill_poly(&mut thresh, &hull, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                                            //     .map_err(|err| error.pass(err.to_string()))?;
                                            // opencv::highgui::imshow("Contours", &thresh).unwrap();
                                            // opencv::highgui::wait_key(0).unwrap();
                                //             // log::debug!("FineContours.eval | contour: {:?}", contour);
                                            found = Some((i1, i2, hull));
                                            break 'contour1;
                                }
                            }
                        }
                    }
                    // std::thread::sleep(Duration::from_millis(100));
                    match found {
                        Some((i1, i2, hull)) => {
                            // log::debug!("FineContours.eval | found: {i1}, {i2}");
                            contours = contours.into_iter().enumerate().filter_map(|(i, h)| {
                                match [i1, i2].contains(&i) {
                                    true => None,
                                    false => Some(h),
                                }
                            }).collect();
                            contours.push_front(hull);
                        }
                        None => {
                            count -= 1;
                        }
                    }
                    // log::debug!("FineContours.eval | count: {}", count);
                    // log::debug!("FineContours.eval | contours: {}", contours.len());
                }
                log::debug!("FineContours.eval | contours: {}", contours.len());
                let contour = contours.into_iter().max_by(|c1, c2| {
                    let area1 = imgproc::contour_area(c1, false).ok();
                    let area2 = imgproc::contour_area(c2, false).ok();
                    // log::debug!("FineContours.eval | area1: {:?},  area2: {:?}", area1, area2);
                    match (area1, area2) {
                        (None, None) => std::cmp::Ordering::Equal,
                        (None, Some(_)) => std::cmp::Ordering::Equal,
                        (Some(_), None) => std::cmp::Ordering::Equal,
                        (Some(area1), Some(area2)) => match area1.partial_cmp(&area2) {
                            Some(cmp) => cmp,
                            None => std::cmp::Ordering::Equal,
                        }
                    }
                });
                let mut dst = Mat::default();
                let mut contour_fill = Mat::default();
                let mut convex = Mat::new_nd_vec_with_default(&core::Vector::from_slice(&[thresh.rows(), thresh.cols()]), core::CV_8UC1, core::Vec4d::from_array([0.0, 0.0, 0.0, 0.0]))
                    .map_err(|err| error.pass(err.to_string()))?;
                if let Some(contour) = contour {
                    let mut hull: core::Vector<Point> = core::Vector::default();
                    imgproc::convex_hull(&contour, &mut hull, true, true)
                        .map_err(|err| error.pass(err.to_string()))?;
                    imgproc::fill_poly(&mut convex, &hull, core::Vec4d::from_array([255.0, 255.0, 255.0, 255.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                        .map_err(|err| error.pass(err.to_string()))?;

                    core::bitwise_and(&thresh, &convex, &mut contour_fill, &core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::approx_poly_dp(&contours.clone(), &mut contours, 24.0, true)
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::fill_convex_poly(&mut thresh, contours, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0)
                    imgproc::fill_poly(&mut dst, &contour_fill, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                        .map_err(|err| error.pass(err.to_string()))?;
                }
                // for contour in contours {
                //     log::debug!("FineContours.eval | contour: {:?}", contour);
                //     // imgproc::polylines(&mut thresh, &contour, true, core::Vec4d::from_array([128.0, 128.0, 128.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                //     //     .map_err(|err| error.pass(err.to_string()))?;
                //     imgproc::fill_convex_poly(&mut thresh, &contour, core::Vec4d::from_array([128.0, 128.0, 128.0, 255.0]), LineTypes::LINE_8 as i32, 0)
                //         .map_err(|err| error.pass(err.to_string()))?;
                // }
                let frame = Image::with(dst);
                let result = FineContoursCtx {
                    convex: Image::with(convex),
                    contour: Image::with(contour_fill),
                    result: frame.clone() };
                let ctx = ctx.write(result)?;
                let result = ResultCtx { frame };
                log::debug!("FineContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
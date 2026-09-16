use std::collections::VecDeque;
use std::time::Instant;
use opencv::{
    core, imgproc,
    core::{Mat, MatTraitConst, Point, Point2i},
    imgproc::{LineTypes ,ThresholdTypes},
};
use sal_core::error::Error;
use crate::{algorithm::{
    cv, ContextRead, ContextWrite, EvalResult, FineContoursConf, FineContoursCtx, FineConvexCtx, ResultCtx,
}};
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return filtered and binarised [Image] with contours detected
pub struct FineContours {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    thresh_ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
    conf: FineContoursConf,
    debug: bool,
}
//
//
impl FineContours {
    ///
    /// Returns [FineContours] new instance
    /// - `ctx` - Prevouse step returns [Image] in [Context]
    /// - `conf` - Configuration for `Fine Contour dectection` algorithm:
    ///     - otsu-tune: 0.40 - Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
    ///     - merge-distance: 24.0 - Maximum distance between contours to be merged
    #[allow(unused)]
    pub fn new(conf: FineContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self { 
            // Первая версия подготовки изображения
            ctx: Box::new(ctx),
            // thresh_ctx: Box::new(
            //     cv::AutoThreshold::new(
            //         conf.otsu_tune,
            //         255.0,
            //         ThresholdTypes::THRESH_BINARY,
            //         cv::Morphology::open(
            //             &[5, 5],
            //             cv::GaussianBlur::new(
            //                 &[13, 13],
            //                 cv::Laplacian::new(
            //                     5,
            //                     cv::GaussianBlur::new(
            //                         &[11, 11],
            //                         PassCvMat::new(),
            //                     ),
            //                 ),
            //             ),
            //         ),
            //     )
            // ),
            // Улучшенная версия подготовки изображения
            thresh_ctx: Box::new(
                // 3. Замыкаем возможные разрывы пунктирных контуров
                cv::Morphology::close(
                    &[9, 9], 
                    // Ищем границы с автоматической адаптацией (Otsu Threshold)
                    cv::AutoCanny::new(
                        conf.otsu_tune,
                        // Обязательно глушим цифровой шум матрицы перед детектором
                        cv::GaussianBlur::new(
                            &[5, 5],
                            PassCvMat::new(),
                        ),
                    ),
                )
            ),
            conf,
            debug,
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
    /// Returns squared distance to avoid expensive sqrt calculation
    fn distance_sq(pt1: &Point, pt2: &Point) -> f32 {
        (pt2.x - pt1.x).pow(2) as f32 + (pt2.y - pt1.y).pow(2) as f32
    }
    // ///
    // /// Returns distance between points
    // fn distance(pt1: &Point, pt2: &Point) -> f32 {
    //     ((pt2.x - pt1.x).pow(2) as f32 + (pt2.y - pt1.y).pow(2)  as f32).sqrt()
    // }
    ///
    /// Returns merged contour if distance between passed contours less then `threshold`
    fn merge(contour1: &core::Vector<Point>, contour2: &core::Vector<Point>, threshold: f32) -> Result<core::Vector<Point>, Error> {
        let error = Error::new("FineContours", "merge");
        if contour1.len() < 4 || contour2.len() < 4 {
            // log::warn!("FineContours.merge | c1[{}], c2[{}]", contour1.len(), contour2.len());
            return Err(error.err("c1 and c2 can't be length < 3"))
        }
        // let mut dst = Mat::default();
        let threshold_sq = threshold * threshold;
        let mut found = false;
        let mut min = (0, 0, threshold_sq);
        for (i1, pt1) in contour1.iter().enumerate() {
            for (i2, pt2) in contour2.iter().enumerate() {
                let distance = Self::distance_sq(&pt1, &pt2);
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
        // log::debug!("FineContours.merge | pair: {i1}, {i2}");
        let min_points = [contour1.len(), contour2.len()].iter().min().map(|m| *m).unwrap_or(contour1.len());
        let steps = (min_points >= 8).then(|| min_points / 4).unwrap_or(min_points / 2);
        let mut i1 = min.0;
        let mut i2 = min.1;
        let mut count = steps;
        while count > 0 {
            // log::debug!("FineContours.merge | pair: {i1}, {i2}");
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance_sq(&pt1, &pt2) <= threshold_sq {
                // imgproc::cvt_color(img, &mut dst, imgproc::COLOR_GRAY2BGRA, 4)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // imgproc::line(&mut dst, pt1, pt2, core::Vec4d::from_array([0.0, 255.0, 0.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::highgui::imshow("Contours", &dst).unwrap();
                // opencv::highgui::wait_key(1).unwrap();
                i1 = Self::dec(i1, contour1.len());
                i2 = Self::inc(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_end = i1; // Self::dec(i1, contour1.len());
        let c2_start = i2; // Self::inc(i2, contour2.len());
        let mut i1 = min.0;
        let mut i2 = min.1;
        // log::debug!("FineContours.merge | i1_remove[{}]: {:?}", i1_remove.len(), i1_remove);
        // log::debug!("FineContours.merge | i2_remove[{}]: {:?}", i2_remove.len(), i2_remove);
        let mut count = steps;
        while count > 0 {
            // log::debug!("FineContours.merge | pair: {i1}, {i2}");
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance_sq(&pt1, &pt2) <= threshold_sq {
                // imgproc::cvt_color(img, &mut dst, imgproc::COLOR_GRAY2BGRA, 4)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // imgproc::line(&mut dst, pt1, pt2, core::Vec4d::from_array([0.0, 255.0, 0.0, 255.0]), 1, LineTypes::LINE_8 as i32, 0)
                //     .map_err(|err| error.pass(err.to_string()))?;
                // opencv::highgui::imshow("Contours", &dst).unwrap();
                // opencv::highgui::wait_key(1).unwrap();
                i1 = Self::inc(i1, contour1.len());
                i2 = Self::dec(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_start = i1; //Self::inc(i1, contour1.len());
        let c2_end = i2; //Self::dec(i2, contour2.len());
        // log::debug!("FineContours.merge | i1_remove[{}]: {:?}", i1_remove.len(), i1_remove);
        // log::debug!("FineContours.merge | i2_remove[{}]: {:?}", i2_remove.len(), i2_remove);
        // log::debug!("FineContours.merge | c1: {},  i1_remove: {}", c1.len(), i1_remove.len());
        // log::debug!("FineContours.merge | ins: {ins}");
        let mut c: core::Vector<Point> = core::Vector::default();
        let mut i = c1_start;
        loop {
            c.push(contour1.get(i).unwrap());
            if i == c1_end { break; }
            i = Self::inc(i, contour1.len());
        }
        let mut i = c2_start;
        loop {
            c.push(contour2.get(i).unwrap());
            if i == c2_end { break; }
            i = Self::inc(i, contour2.len());
        }
        Ok(c)
    }
    ///
    /// Returns single merged contour
    /// 
    /// Merging done for nierby contours with distance between less then `threshold`
    fn contour(image: &Mat, threshold: f64) -> Result<core::Vector<Point>, Error> {
        let error = Error::new("FineContours", "contour");
        let mut contours: core::Vector<core::Vector<Point>> = core::Vector::default();
        log::trace!("FineContours.eval | contours...");
        imgproc::find_contours(
            image,
            &mut contours,
            imgproc::RetrievalModes::RETR_EXTERNAL as i32,
            imgproc::ContourApproximationModes::CHAIN_APPROX_SIMPLE as i32,
            core::Point2i::new(0, 0),
        ).map_err(|err| error.pass(err.to_string()))?;
        let mut contours = VecDeque::from_iter(contours.iter().filter(|c| c.len() > 3));
        let mut count = 1;
        while count > 0 {
            let mut found = None;
            'contour1: for (i1, contour1) in contours.iter().enumerate() {
                for (i2, contour2) in contours.iter().enumerate().filter(|(i, _)| i1 != *i) {
                    // log::debug!("FineContours.eval | search nierby segments...");
                    if let Ok(hull) = Self::merge(&contour1, &contour2, threshold as f32) {
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
        log::trace!("FineContours.eval | contours: {}", contours.len());
        let contour = contours.into_iter().max_by(|c1, c2| {
            let area1 = imgproc::contour_area(c1, false).ok();
            let area2 = imgproc::contour_area(c2, false).ok();
            // log::debug!("FineContours.eval | area1: {:?},  area2: {:?}", area1, area2);
            match (area1, area2) {
                (Some(area1), Some(area2)) => area1.partial_cmp(&area2).unwrap_or(std::cmp::Ordering::Equal),
                _ => std::cmp::Ordering::Equal,
            }
        });
        match contour {
            Some(contour) => Ok(contour),
            None => Err(error.err("Max contour isn't found")),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FineContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineContours", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                // let result: &ResultCtx = ctx.read();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let thresh = self.thresh_ctx.eval(frame.mat.clone()).map_err(|err| error.pass(err))?;
                let mut dst = Mat::default();
                let contour = Self::contour(&thresh, self.conf.merge_distance).map_err(|err| error.pass(err))?;
                // let mut contour_fill = Mat::default();
                let mut convex = Mat::new_rows_cols_with_default(
                    thresh.rows(), 
                    thresh.cols(), 
                    core::CV_8UC1, 
                    core::Scalar::all(0.0)
                ).map_err(|err| error.pass(err.to_string()))?;
                if !contour.is_empty() {
                    let mut convex_contour: core::Vector<Point> = core::Vector::default();
                    imgproc::convex_hull(&contour, &mut convex_contour, true, true)
                        .map_err(|err| error.pass(err.to_string()))?;
                    imgproc::fill_poly(&mut convex, &convex_contour, core::Vec4d::from_array([255.0, 255.0, 255.0, 255.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                        .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::fill_poly(&mut dst, &contour, core::Vec4d::from_array([255.0, 255.0, 255.0, 255.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // opencv::highgui::imshow("Fine Contours", &thresh).unwrap();
                    // opencv::highgui::wait_key(0).unwrap();
                    // opencv::highgui::imshow("Fine Contours", &convex).unwrap();
                    // opencv::highgui::wait_key(0).unwrap();
                    core::bitwise_and(&thresh, &convex, &mut dst, &core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::approx_poly_dp(&contours.clone(), &mut contours, 24.0, true)
                    //     .map_err(|err| error.pass(err.to_string()))?;
                    // imgproc::fill_convex_poly(&mut thresh, contours, core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]), LineTypes::LINE_8 as i32, 0)
                    // imgproc::fill_poly(&mut dst, &contour_fill,
                    //     core::Vec4d::from_array([128.0, 128.0, 128.0, 64.0]),
                    //     LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0),
                    // )
                    //     .unwrap();
                        // .map_err(|err| error.pass(err.to_string()))?;
                }
                let frame = Image::from(dst, meta);
                let ctx = match self.debug {
                    true => ctx.write(FineContoursCtx { result: frame.clone() }).map_err(|err| error.pass(err))?,
                    false => ctx,
                };
                let ctx = ctx.write(FineConvexCtx { convex: Some(Image::from(convex, meta)) }).map_err(|err| error.pass(err))?;
                let result = ResultCtx { val: frame };
                log::debug!("FineContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// Closes calculation sequence, passing input `Mat`
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}
///
/// Basic tests
#[cfg(test)]
mod tests {
    use super::*;
    use opencv::core::{Point, Vector};
    // Фабрика для удобного конструирования контуров
    fn create_contour(points: &[(i32, i32)]) -> Vector<Point> {
        let mut v = Vector::default();
        for &(x, y) in points {
            v.push(Point::new(x, y));
        }
        v
    }
    #[test]
    fn rejects_contours_with_insufficient_points() {
        let c1 = create_contour(&[(0, 0), (10, 0), (10, 10)]); // Длина 3, должно отбить
        let c2 = create_contour(&[(0, 0), (10, 0), (10, 10), (0, 10)]); // Длина 4
        let result = FineContours::merge(&c1, &c2, 5.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::new("FineContours", "merge").err("c1 and c2 can't be length < 3"));
    }
    #[test]
    fn rejects_merge_when_distance_exceeds_threshold() {
        // Контуры разнесены далеко друг от друга
        let c1 = create_contour(&[(0, 0), (10, 0), (10, 10), (0, 10)]);
        let c2 = create_contour(&[(100, 100), (110, 100), (110, 110), (100, 110)]);
        let result = FineContours::merge(&c1, &c2, 10.0); // Порог 10.0, а расстояние ~100
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::new("FineContours", "merge").err("Can't merge"));
    }
    #[test]
    fn merges_adjacent_contours_successfully() {
        // Два квадрата, соприкасающиеся по одной грани
        let c1 = create_contour(&[(0, 0), (10, 0), (10, 10), (0, 10)]);
        let c2 = create_contour(&[(10, 0), (20, 0), (20, 10), (10, 10)]);
        let threshold = 2.0;
        let result = FineContours::merge(&c1, &c2, threshold);
        assert!(result.is_ok());
        let merged = result.unwrap();
        // В обоих контурах по 4 точки. count = 4 / 4 = 1.
        // Ожидаем, что итоговый контур соберется корректно без паники
        assert!(merged.len() >= 4);
    }
    #[test]
    fn handles_ring_buffer_wrap_around() {
        // Смещаем порядок точек так, чтобы минимальная дистанция выпала
        // ровно на стык начала и конца массива (индексы 0 и len-1)
        let c1 = create_contour(&[(10, 10), (0, 10), (0, 0), (10, 0)]);
        let c2 = create_contour(&[(10, 0), (20, 0), (20, 10), (10, 10)]);
        let threshold = 2.0;
        let result = FineContours::merge(&c1, &c2, threshold);
        assert!(result.is_ok());
        let merged = result.unwrap();
        // Убеждаемся, что inc и dec правильно прокрутили индексы и не потеряли точки
        assert!(merged.len() >= 4);
    }
    #[test]
    fn strict_geometry_merge_retains_corners() {
        // Квадрат 1: левый
        let c1 = create_contour(&[(0, 0), (10, 0), (10, 10), (0, 10)]);
        // Квадрат 2: правый, прилегает по грани X=10
        let c2 = create_contour(&[(10, 0), (20, 0), (20, 10), (10, 10)]);
        
        let merged = FineContours::merge(&c1, &c2, 2.0).expect("Must merge");
        
        // Вытащим координаты для проверки
        let points: Vec<(i32, i32)> = merged.iter().map(|p| (p.x, p.y)).collect();
        println!("{:?}", points);
        // Проверяем наличие всех 4-х внешних углов итогового прямоугольника:
        assert!(points.contains(&(0, 0)), "Lost corner (0,0)");
        assert!(points.contains(&(0, 10)), "Lost corner (0,10)");
        assert!(points.contains(&(20, 0)), "Lost corner (20,0)");
        assert!(points.contains(&(20, 10)), "Lost corner (20,10)");
    }
}
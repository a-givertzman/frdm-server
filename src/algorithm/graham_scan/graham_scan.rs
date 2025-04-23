use photon_rs::PhotonImage;
use crate::domain::eval::eval::Eval;
use super::graham_scan_ctx::GrahamScanCtx;
///
/// [Graham scan](https://en.wikipedia.org/wiki/Graham_scan)
pub struct GrahamScan {
    contour: PhotonImage,
    result: Option<GrahamScanCtx>,
}
///
/// Represents point in image
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub rgba: [u8; 4],
    pub x: u32,
    pub y: u32,
}
//
//
impl Point {
    fn new(rgba: [u8; 4], x: u32, y: u32) -> Self {
        Self { rgba, x, y }
    }
}
//
//
impl GrahamScan {
    ///
    /// New instance [GrahamScan]
    pub fn new(contour: PhotonImage) -> Self {
        Self {
            contour,
            result: None,
        }
    }
    ///
    /// Find all contour points in the image
    fn find_points(&self) -> Vec<Point> {
        let input_raw_pixels = self.contour.get_raw_pixels();
        let input_width = self.contour.get_width();
        let input_height = self.contour.get_height();
        let mut points = Vec::new();
        for y in 0..input_height {
            for x in 0..input_width {
                let index = (y * input_width * 4 + x * 4) as usize;
                let rgba = [
                    input_raw_pixels[index],
                    input_raw_pixels[index + 1],
                    input_raw_pixels[index + 2],
                    input_raw_pixels[index + 3],
                ];
                if rgba == [255, 255, 255, 255] {
                    points.push(Point::new(rgba, x, y));
                }
            }
        }
        points
    }
    ///
    /// Find the lowest y-coordinate and leftmost point, called P0
    fn find_p0(points: &[Point]) -> Point {
        let mut p0 = points[0];
        for point in points {
            if point.y > p0.y || (point.y == p0.y && point.x < p0.x) {
                p0 = *point;
            }
        }
        p0
    }
    ///
    /// Polar angle between p0 and p1
    fn polar_angle(p0: Point, p1: Point) -> f64 {
        let dx = p1.x as f64 - p0.x as f64;
        let dy = p1.y as f64 - p0.y as f64;
        dy.atan2(dx)
    }
    ///
    /// Squared distance between two points
    fn distance_sq(p1: Point, p2: Point) -> u32 {
        let dx = p1.x as i32 - p2.x as i32;
        let dy = p1.y as i32 - p2.y as i32;
        (dx * dx + dy * dy) as u32
    }
    ///
    /// Orientation of three points
    fn orientation(p1: Point, p2: Point, p3: Point) -> i32 {
        let val = (p2.y as i32 - p1.y as i32) * (p3.x as i32 - p2.x as i32) 
                - (p2.x as i32 - p1.x as i32) * (p3.y as i32 - p2.y as i32);
        if val == 0 { 0 } // colinear
        else if val > 0 { 1 } // clockwise
        else { 2 } // counter-clockwise
    }
}

impl Eval<(), GrahamScanCtx> for GrahamScan {
    ///
    /// Finding convex hull by Graham scan
    fn eval(&mut self, _: ()) -> GrahamScanCtx {
        let points = self.find_points();
        let p0 = Self::find_p0(&points);
        let mut sorted_points = points.clone();
        // sorting by polar angle based on P0
        sorted_points.sort_by(|a, b| {
            let angle_a = Self::polar_angle(p0, *a);
            let angle_b = Self::polar_angle(p0, *b);
            // sorting by rising polar angle
            if angle_a < angle_b {
                std::cmp::Ordering::Less
            } else if angle_a > angle_b {
                std::cmp::Ordering::Greater
            } else { // if polar angle's are equals we take farthest
                let dist_a = Self::distance_sq(p0, *a);
                let dist_b = Self::distance_sq(p0, *b);
                dist_a.cmp(&dist_b)
            }
        });
        sorted_points.insert(0, p0);
        // Initialize stack
        let mut stack: Vec<Point> = Vec::new();
        for i in 0..sorted_points.len() {   
            while stack.len() > 1 {
                let top = stack[stack.len() - 1];
                let next_to_top = stack[stack.len() - 2];
                
                if Self::orientation(next_to_top, top, sorted_points[i]) != 2 {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(sorted_points[i]);
        }
        GrahamScanCtx { 
            result: stack,
        }
    }
}
use photon_rs::PhotonImage;
use sal_core::dbg::Dbg;
use crate::domain::eval::eval::Eval;
use super::{entities::point::Point, graham_ctx::GrahamCtx};
///
/// [Graham Scan](https://en.wikipedia.org/wiki/Graham_scan)
pub struct Graham {
    dbg: Dbg,
    // contour of rope
    contour: PhotonImage,
    // points of clear contour
    points: Vec<Point>,
    // detected perimeter
    result: Option<GrahamCtx>
}
//
//
impl Graham {
    ///
    /// New instance [Graham]
    pub fn new(contour: PhotonImage) -> Self {
        Self {
            dbg: Dbg::own("Graham"),
            contour,
            points: Vec::new(),
            result: None,
        }
    }
    ///
    /// Find the lowest y-coordinate and leftmost point, called P0
    fn P0(&mut self, raw_pixels: Vec<u8>, width: u32, height: u32) -> Point {
        let mut result: Point = Point::new();
        for i in (0..height).rev() {
            for j in (4..width).rev().step_by(4) {
                let slice = j as usize;
                if &raw_pixels[slice-4..slice] == [255,255,255,0] {
                    println!("P0 {:?}",Point {
                        rgba: raw_pixels[slice-4..slice].to_vec(),
                        x: j,
                        y: i,
                    });

                    return Point {
                        rgba: raw_pixels[slice-4..slice].to_vec(),
                        x: j,
                        y: i,
                    }
                }
            }   
        }
        result
    }
}
//
//
impl Eval<(), GrahamCtx> for Graham {
    ///
    /// Detecting rope contours
    fn eval(&mut self, _: ()) -> GrahamCtx {
        println!("Width:{:?}",self.contour.get_width());
        println!("Height:{:?}",self.contour.get_height());

        self.P0(self.contour.get_raw_pixels(), self.contour.get_width(), self.contour.get_height());
        GrahamCtx { 
            result: PhotonImage::new(Vec::new(), 0, 0)
        }
    }
}
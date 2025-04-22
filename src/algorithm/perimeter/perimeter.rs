use photon_rs::{native::save_image, PhotonImage};
use sal_core::dbg::Dbg;
use crate::domain::eval::eval::Eval;
use super::{entities::point::Point, perimeter_ctx::PerimeterCtx};
///
/// [Graham Scan](https://en.wikipedia.org/wiki/Graham_scan)
pub struct Perimeter {
    dbg: Dbg,
    // contour of rope
    contour: PhotonImage,
    // points of clear contour, where first point is the lowest y-coordinate and rightmost point 
    points: Vec<Point>,
    // vector of RGBA
    raw_pixels: Vec<u8>,
    // image of finded perimeter
    result: Option<PerimeterCtx>
}
//
//
impl Perimeter {
    ///
    /// New instance [Perimeter]
    pub fn new(contour: PhotonImage) -> Self {
        Self {
            dbg: Dbg::own("Perimeter"),
            contour,
            points: Vec::new(),
            raw_pixels: Vec::new(),
            result: None,
        }
    }
    ///
    /// Find raw pixels of rope perimeter
    fn perimeter(&mut self, raw_pixels: Vec<u8>, width: u32, height: u32) {
        // finding left side of rope perimeter
        for y in (0..height).rev() {
            for x in 0..width {
                let index = (y * width * 4 + x * 4) as usize;
                let rgba = &raw_pixels[index..index + 4];                
                if rgba == [255, 255, 255, 255] {
                    self.raw_pixels[index..index + 4].copy_from_slice(rgba);
                    break; // to not take point inside large contour
                }
            }
        }
        // finding right side of rope perimeter
        for y in (0..height).rev() {
            for x in (0..width).rev() {
                let index = (y * width * 4 + x * 4) as usize;
                let rgba = &raw_pixels[index..index + 4];                
                if rgba == [255, 255, 255, 255] {
                    self.raw_pixels[index..index + 4].copy_from_slice(rgba);
                    break; // to not take point inside large contour
                }
            }   
        }
    }
}
//
//
impl Eval<(), PerimeterCtx> for Perimeter {
    ///
    /// Detecting rope contours
    fn eval(&mut self, _: ()) -> PerimeterCtx {
        let input_raw_pixels = self.contour.get_raw_pixels();
        let input_width = self.contour.get_width();
        let input_height = self.contour.get_height();
        self.raw_pixels.resize((input_height*input_width*4).try_into().unwrap(), 0);
        self.perimeter(input_raw_pixels, input_width, input_height);
        let result = PhotonImage::new(self.raw_pixels.clone(), input_width, input_height);
        let _ = save_image(result, "src/algorithm/perimeter/out_1.jpeg");
        PerimeterCtx { 
            result: PhotonImage::new(Vec::new(), 0, 0)
        }
    }
}
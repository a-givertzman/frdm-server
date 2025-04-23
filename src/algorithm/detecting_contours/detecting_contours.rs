use photon_rs::PhotonImage;
use sal_core::dbg::Dbg;
use crate::domain::eval::eval::Eval;
use super::detecting_contours_ctx::DetectingContoursCtx;
use photon_rs::monochrome::grayscale;
use photon_rs::monochrome::threshold;
use photon_rs::conv::{
    noise_reduction,
    gaussian_blur,
    sobel_vertical,
};
///
/// Algorithm of finding rope contours on image
pub struct DetectingContours {
    dbg: Dbg,
    // rope frame to detecting
    input_frame: PhotonImage,
    // detected contours
    result: Option<DetectingContoursCtx>
}
//
//
impl DetectingContours {
    ///
    /// New instance [DetectingContours]
    pub fn new(input_frame: PhotonImage) -> Self {
        Self {
            dbg: Dbg::own("DetectingContours"),
            input_frame,
            result: None,
        }
    }
    ///
    /// Detecting rope contours
    fn get_contours(&self) -> PhotonImage {
        let mut result = self.input_frame.clone();
        grayscale(&mut result);
        gaussian_blur(&mut result, 4_i32);
        sobel_vertical(&mut result);
        noise_reduction(&mut result);
        threshold(&mut result, 7_u32);
        result
    }
}
//
//
impl Eval<(), DetectingContoursCtx> for DetectingContours {
    ///
    /// Detecting rope contours
    fn eval(&mut self, _: ()) -> DetectingContoursCtx {
        DetectingContoursCtx { 
            result: self.get_contours() 
        }
    }
}
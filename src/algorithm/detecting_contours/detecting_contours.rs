use photon_rs::PhotonImage;
use crate::domain::{dbg::dbgid::DbgId, eval::eval::Eval};
use super::detecting_contours_ctx::DetectingContoursCtx;
use photon_rs::monochrome::grayscale;
use photon_rs::monochrome::threshold;
use photon_rs::conv::detect_45_deg_lines;
use photon_rs::conv::gaussian_blur;
///
/// Algorithm of finding rope contours on image
pub struct DetectingContours {
    dbgid: DbgId,
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
            dbgid: DbgId::root("DetectingContours"),
            input_frame,
            result: None,
        }
    }
    ///
    /// Detecting rope contours
    fn get_contours(&self) -> PhotonImage {
        let mut result = self.input_frame.clone();
        grayscale(&mut result);
        gaussian_blur(&mut result, 1_i32);
        detect_45_deg_lines(&mut result);
        threshold(&mut result, 4_u32);
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
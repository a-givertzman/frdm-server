use photon_rs::PhotonImage;
///
/// Context store of [DetectingContours](src/algorithm/detecting_contours/detecting_contours.rs)
/// - `result` - rope contours
pub struct DetectingContoursCtx {
    pub result: PhotonImage,
}
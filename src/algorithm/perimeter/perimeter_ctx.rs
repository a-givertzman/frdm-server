use photon_rs::PhotonImage;
///
/// Context store of [Perimeter](src/algorithm/perimeter/perimeter.rs)
/// - `result` - perimeter of rope contours
pub struct PerimeterCtx {
    pub result: PhotonImage,
}
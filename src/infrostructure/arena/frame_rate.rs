use serde::{de, Deserialize};

///
/// Represents a setting for the camera
/// - `Min` - Minimum supported
/// - `Max` - Maximum supported
/// - `30.0` - User specified value
/// 
/// Set `AcquisitionFrameRateEnable` to `true` - this is required to change the `AcquisitionFrameRate`
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum FrameRate {
    Min,
    Max,
    #[serde(untagged)]
    Val(f64),
}

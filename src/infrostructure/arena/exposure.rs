use serde::Deserialize;

///
/// Exposure settings for the camera
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct Exposure {
	/// Disable automatic exposure before setting an exposure time.
    /// 
    /// Automatic exposure controls whether the exposure time is set manually or
	/// automatically by the device. Setting automatic exposure to 'Off' stops
	/// the device from automatically updating the exposure time while
	/// streaming.
    pub auto: ExposureAuto,
    ///
    /// Exposure Time in microseconds
    /// - Opproximate range (exact range to be read from device):
    ///     - Normal mode: 20.5 μs to 10 s
    ///     - Short mode : 1 μs to 5 μs
	/// - Before setting the exposure time:
    ///     - Disable automatic exposure
    ///     - Check that the new exposure time is not
	///       outside of the exposure time's acceptable range. If the value is above the
	///       maximum or below the minimum, update the value to be within range. Lastly,
	///       set new the new exposure time.
    pub time: f64,
}
//
//
impl Exposure {
    ///
    /// Returns [Exposure] new instance
    pub fn new(auto: ExposureAuto, time: f64) -> Self {
        Self {
            auto,
            time,
        }
    }
}
///
/// 
#[derive(Clone, Copy, Deserialize, PartialEq)]
pub enum ExposureAuto {
    Off,
    // On,
    Continuous,
}
impl ExposureAuto {
    /// Returns &str representation of the [ExposureAuto] variant, used in the `AcDevice` raw setting
    pub fn as_str(&self) -> &str {
        match self {
            ExposureAuto::Continuous => "Continuous",
            ExposureAuto::Off => "Off",
            // ExposureAuto::On => "On",
        }
    }
}
impl std::fmt::Display for ExposureAuto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl std::fmt::Debug for ExposureAuto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
//
//
impl Default for Exposure {
    fn default() -> Self {
        Self { auto: ExposureAuto::Continuous, time: 0.0 }
    }
}
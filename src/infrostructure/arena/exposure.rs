///
/// Exposure Time 	20.5 μs to 10 s (Normal) / 1 μs to 5 μs (Short Mode)
pub struct Exposure {
    pub auto: ExposureAuto,
    ///
    /// Exposure Time in microseconds
    pub time: f64,
}
//
//
impl Exposure {
    pub fn new(auto: ExposureAuto, time: f64) -> Self {
        Self {
            auto,
            time,
        }
    }
}
///
/// 
pub enum ExposureAuto {
    Off,
    // On,
    Continuous,
}
impl AsRef<str> for ExposureAuto {
    fn as_ref(&self) -> &str {
        match self {
            ExposureAuto::Continuous => "Continuous",
            ExposureAuto::Off => "Off",
            // ExposureAuto::On => "On",
        }
    }
}
impl std::fmt::Display for ExposureAuto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl std::fmt::Debug for ExposureAuto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
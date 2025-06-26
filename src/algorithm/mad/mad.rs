use crate::domain::Eval;
use super::MadCtx;
///
/// Median Absolute Deviation
pub struct Mad;
//
//
impl Mad {
    ///
    /// New instance [Mad]
    pub fn new() -> Self {
        Self {}
    }
    ///
    /// Calculate median
    fn median(points: &[usize]) -> f64 {
        let mut values: Vec<f64> = points
            .iter()
            .map(|point| *point as f64)
        .collect();
        values
            .sort_by(|a, b| a.partial_cmp(b)
        .unwrap());
        let len = values.len();
        if len % 2 == 1 {
            values[len / 2]
        } else {
            (values[len / 2 - 1] + values[len / 2]) / 2.0
        }
    }
    ///
    /// Calculate Median Absolute Deviation
    fn mad(sample: &[usize], median: f64) -> f64 {
        let mut deviations: Vec<f64> = sample.iter()
            .map(|point| (*point as f64 - median).abs())
        .collect();
        deviations
            .sort_by(|a, b| a.partial_cmp(b)
        .unwrap());
        let len = deviations.len();
        if len % 2 == 1 {
            deviations[len / 2]
        } else {
            (deviations[len / 2 - 1] + deviations[len / 2]) / 2.0
        }
    }
}
//
//
impl Eval<Vec<usize>, MadCtx> for Mad {
    fn eval(&self, sample: Vec<usize>) -> MadCtx {
        let median = Self::median(&sample);
        let mad = Self::mad(&sample, median);
        MadCtx { median, mad }
    }
}

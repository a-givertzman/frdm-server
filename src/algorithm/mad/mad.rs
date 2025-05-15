use crate::domain::eval::eval::Eval;
///
/// Median Absolute Deviation
pub struct MAD {
    sample: Vec<i16>,
    result: Option<MADCtx>,
}
//
//
impl MAD {
    ///
    /// New instance [MAD]
    pub fn new(sample: Vec<i16>) -> Self {
        Self {
            sample,
            result: None,
        }
    }
    ///
    /// Calculate median
    fn median(points: &[i16]) -> f32 {
        let mut values: Vec<f32> = points.iter().map(|point| *point as f32).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();
        if len % 2 == 1 {
            values[len / 2]
        } else {
            (values[len / 2 - 1] + values[len / 2]) / 2.0
        }
    }
    ///
    /// Calculate Median Absolute Deviation
    fn mad(sample: &[i16], median: f32) -> f32 {
        let mut deviations: Vec<f32> = sample.iter()
            .map(|point| (*point as f32 - median).abs())
            .collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
impl Eval<(), MADCtx> for MAD {
    fn eval(&mut self, _: ()) -> MADCtx {
        let median = Self::median(&self.sample);
        let mad = Self::mad(&self.sample, median);
        let result = MADCtx { median, mad };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of algorithm [MAD]
#[derive(Debug, Clone, PartialEq)]
pub struct MADCtx {
    pub median: f32,
    pub mad: f32,
}
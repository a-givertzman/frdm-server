use sal_core::error::Error;

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
    fn median(points: &[usize]) -> Result<f64, Error> {
        let mut values: Vec<f64> = points
            .iter()
            .map(|point| *point as f64)
        .collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if !values.is_empty() {
            let len = values.len();
            if len % 2 == 1 {
                Ok(values[len / 2])
            } else {
                Ok((values[len / 2 - 1] + values[len / 2]) / 2.0)
            }
        } else {
            Err(Error::new("Mad", "median").err("Input sequence is empty"))
        }
    }
    ///
    /// Calculate Median Absolute Deviation
    fn mad(sample: &[usize], median: f64) -> Result<f64, Error> {
        let mut deviations: Vec<f64> = sample.iter()
            .map(|point| (*point as f64 - median).abs())
            .collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if !deviations.is_empty() {
            let len = deviations.len();
            if len % 2 == 1 {
                Ok(deviations[len / 2])
            } else {
                Ok((deviations[len / 2 - 1] + deviations[len / 2]) / 2.0)
            }
        } else {
            Err(Error::new("Mad", "median").err("Input sequence is empty"))
        }
    }
}
//
//
impl Eval<Vec<usize>, Result<MadCtx, Error>> for Mad {
    fn eval(&self, sample: Vec<usize>) -> Result<MadCtx, Error> {
        match Self::median(&sample) {
            Ok(median) => Ok(MadCtx {
                median,
                mad: Self::mad(&sample, median).map_err(|err| Error::new("Mad", "eval").pass(err))?,
            }),
            Err(err) => Err(Error::new("Mad", "eval").pass(err)),
        }
    }
}

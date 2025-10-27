use std::time::Instant;
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
    pub fn median(sample: &[usize]) -> Result<usize, Error> {
        let len = sample.len();
        match len {
            0 => Err(Error::new("Mad", "median").err("Input sequence is empty")),
            1 => Ok(sample[0]),
            _ => {
                // log::debug!("Mad.median | values: {:?}", values);
                let half_len = len / 2;
                if len % 2 == 1 {
                    Ok(sample[half_len])
                } else {
                    Ok(((sample[half_len - 1] + sample[half_len]) as f64 * 0.5).round() as usize)
                }
            }
        }
    }
    ///
    /// Calculate Median Absolute Deviation
    fn mad(sample: &[usize], median: usize) -> Result<usize, Error> {
        let mut sample: Vec<usize> = sample.iter().map(|v| (*v as isize - median as isize).abs() as usize).collect();
        sample.sort_by(|a, b| a.cmp(b));
        Self::median(&sample)
    }
}
//
//
impl Eval<Vec<usize>, Result<MadCtx, Error>> for Mad {
    fn eval(&self, mut sample: Vec<usize>) -> Result<MadCtx, Error> {
        let t = Instant::now();
        sample.sort_by(|a, b| a.cmp(b));
        match Self::median(&sample) {
            Ok(median) => Ok(MadCtx {
                median: median as f64,
                mad: {
                    let mad = Self::mad(&sample, median).map_err(|err| Error::new("Mad", "eval").pass(err))? as f64;
                    // log::debug!("Mad.eval | Elapsed: {:?}", t.elapsed());
                    mad
                }
            }),
            Err(err) => Err(Error::new("Mad", "eval").pass(err)),
        }
    }
}

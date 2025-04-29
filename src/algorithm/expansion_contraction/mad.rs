use crate::domain::{eval::eval::Eval, graham::dot::Dot};
use super::bond::Bond;
///
/// Median Absolute Deviation
pub struct MAD {
    upper_points: Vec<Dot<u16>>,
    lower_points: Vec<Dot<u16>>,
    result: Option<MADCtx>,
}
//
//
impl MAD {
    ///
    /// New instance [MAD]
    pub fn new(upper_points: Vec<Dot<u16>>, lower_points: Vec<Dot<u16>>) -> Self {
        Self {
            upper_points,
            lower_points,
            result: None,
        }
    }
    ///
    /// Calculate median of y-coordinates
    fn median_y(points: &[Dot<u16>]) -> f32 {
        let mut y_values: Vec<f32> = points.iter().map(|point| point.y as f32).collect();
        y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = y_values.len();
        if len % 2 == 1 {
            y_values[len / 2]
        } else {
            (y_values[len / 2 - 1] + y_values[len / 2]) / 2.0
        }
    }
    ///
    /// Calculate Median Absolute Deviation
    fn mad(points: &[Dot<u16>], median: f32) -> f32 {
        let mut deviations: Vec<f32> = points.iter()
            .map(|point| (point.y as f32 - median).abs())
            .collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = deviations.len();
        if len % 2 == 1 {
            deviations[len / 2]
        } else {
            (deviations[len / 2 - 1] + deviations[len / 2]) / 2.0
        }
    }
    ///
    /// Find emision's
    fn emissions(&self, points: &[Dot<u16>], median: f32, mad: f32, threshold: f32) -> Vec<Bond<u16>> {
        points.iter()
            .filter(|point| {
                let deviation = (point.y as f32 - median).abs();
                deviation > threshold * mad
            })
            .map(|point| Bond {x: point.x, y: point.y})
            .collect()
    }
}
//
//
impl Eval<(), MADCtx> for MAD {
    fn eval(&mut self, _: ()) -> MADCtx {
        let median_upper = Self::median_y(&self.upper_points);
        let median_lower = Self::median_y(&self.lower_points);
        let mad_upper = Self::mad(&self.upper_points, median_upper);
        let mad_lower = Self::mad(&self.lower_points, median_lower);
        let threshold = 1.1;
        let bond_up = self.emissions(&self.upper_points, median_upper, mad_upper, threshold);
        let bond_low = self.emissions(&self.lower_points, median_lower, mad_lower, threshold);
        let result = MADCtx { bond_up, bond_low };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of algorithm [MAD]
#[derive(Debug, Clone)]
pub struct MADCtx {
    pub bond_up: Vec<Bond<u16>>,
    pub bond_low: Vec<Bond<u16>>,
}
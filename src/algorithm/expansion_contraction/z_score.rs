use crate::domain::{eval::eval::Eval, graham::dot::Dot};
use super::bond::Bond;
///
/// Z score
pub struct ZScore {
    upper_points: Vec<Dot<u16>>,
    lower_points: Vec<Dot<u16>>,
    result: Option<ZScoreCtx>,
}
//
//
impl ZScore {
    ///
    /// New instance [ZScore]
    pub fn new(upper_points: Vec<Dot<u16>>, lower_points: Vec<Dot<u16>>) -> Self {
        Self {
            upper_points,
            lower_points,
            result: None,
        }
    }
    ///
    /// Average of height coordinate
    fn average_y(&mut self, points: Vec<Dot<u16>>) -> f32 {
        let sum: u32 = points.iter().map(|point| point.y as u32).sum();
        sum as f32 / points.len() as f32
    }
    ///
    /// standard deviation of height coordinate
    fn standard_deviation_y(&mut self, points: Vec<Dot<u16>>, average: f32) -> f32 {
        let mut sum_squared_diff = 0.0;
        for point in points.clone() {
            let diff = point.y as f32 - average;
            sum_squared_diff += diff * diff;
        }
        let variance = sum_squared_diff / points.len() as f32;
        variance.sqrt()
    }
    ///
    /// Find emision's
    fn emissions(&mut self, points: Vec<Dot<u16>>, average: f32, standard_deviation: f32) -> Vec<Bond<u16>> {
        let threshold = 1.0;
        let mut result = Vec::new();
        for point in points {
            let ratio = (((point.y as f32) - average) / standard_deviation).abs();
            if ratio > threshold {
                result.push(Bond { x: point.x, y: point.y })
            }
        }
        result
    }
}
//
//
impl Eval<(), ZScoreCtx> for ZScore {
    fn eval(&mut self, _: ()) -> ZScoreCtx {
        let avg_up = self.average_y(self.upper_points.clone());
        let avg_low = self.average_y(self.lower_points.clone());
        let standard_deviation_up = self.standard_deviation_y(self.upper_points.clone(), avg_up);
        let standard_deviation_low = self.standard_deviation_y(self.lower_points.clone(), avg_low);
        ZScoreCtx { 
            bond_up: self.emissions(self.upper_points.clone(), avg_up, standard_deviation_up), 
            bond_low: self.emissions(self.lower_points.clone(), avg_low, standard_deviation_low), 
        }
    }
}
///
/// Store result of algorithm [ZScore]
#[derive(Debug, Clone)]
pub struct ZScoreCtx {
    pub bond_up: Vec<Bond<u16>>,
    pub bond_low: Vec<Bond<u16>>,
}
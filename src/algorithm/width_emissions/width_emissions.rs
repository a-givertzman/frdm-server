use sal_core::dbg::Dbg;
use crate::{algorithm::{geometry_defect::Threshold, mad::{bond::Bond, mad::Mad}}, domain::{Eval, graham::dot::Dot}};
///
/// Finding width emissions the rope
pub struct WidthEmissions {
    dbg: Dbg,
    threshold: Threshold,
    initial_points_upper: Vec<Dot<usize>>,
    initial_points_lower: Vec<Dot<usize>>,
    result: Option<WidthEmissionsCtx>
}
//
//
impl WidthEmissions {
    ///
    /// New instance [WidthEmissions]
    pub fn new(threshold: Threshold, initial_points_upper: Vec<Dot<usize>>, initial_points_lower: Vec<Dot<usize>>) -> Self {
        Self {
            dbg: Dbg::own("WidthEmissions"),
            threshold,
            initial_points_upper,
            initial_points_lower,
            result: None, 
        }
    }
    ///
    /// Compute width between initial dots
    fn points_width(&self) -> Vec<usize> {
        let mut dots_width = Vec::new();
        for i in 0..self.initial_points_lower.len() { // `for` only for one vector cause they must be same length
            let width = self.initial_points_upper[i].y - self.initial_points_lower[i].y;
            dots_width.push(width);
        };
        dots_width
    }
    ///
    /// Find emissions
    fn emissions(&self, median: f64, mad: f64, threshold: f64) -> Vec<Bond<usize>> {
        let mut emissions = Vec::new();
        for i in 0..self.initial_points_lower.len() { // `for` only for one vector cause they must be same length
            let deviation = ((self.initial_points_upper[i].y - self.initial_points_lower[i].y) as f64 - median).abs();
            if deviation > threshold * mad {
                emissions.push(
                    Bond {
                        x: self.initial_points_upper[i].x,
                        y: self.initial_points_upper[i].y,
                    }
                );
                emissions.push(
                    Bond {
                        x: self.initial_points_lower[i].x,
                        y: self.initial_points_lower[i].y,
                    }
                );
            }
        };
        emissions
    }
}
//
//
impl Eval<(), WidthEmissionsCtx> for WidthEmissions {
    fn eval(&self, _: ()) -> WidthEmissionsCtx {
        let mad_result = Mad::new(self.points_width()).eval(());
        let result = WidthEmissionsCtx {
            result: self.emissions(mad_result.median, mad_result.mad, self.threshold.0),
        };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of [WidthEmissions]
#[derive(Clone)]
pub struct WidthEmissionsCtx {
    pub result: Vec<Bond<usize>>
}
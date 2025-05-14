use sal_core::dbg::Dbg;
use crate::{algorithm::mad::{bond::Bond, mad::MAD}, domain::{eval::eval::Eval, graham::dot::Dot}};
///
/// Finding width emissions the rope
pub struct WidthEmissions {
    initial_points_upper: Vec<Dot<u16>>,
    initial_points_lower: Vec<Dot<u16>>,
    dbg: Dbg,
    result: Option<WidthEmissionsCtx>
}
//
//
impl WidthEmissions {
    ///
    /// New instance [WidthEmissions]
    pub fn new(initial_points_upper: Vec<Dot<u16>>, initial_points_lower: Vec<Dot<u16>>) -> Self {
        Self {
            initial_points_upper,
            initial_points_lower,
            dbg: Dbg::own("WidthEmissions"),
            result: None, 
        }
    }
    ///
    /// Compute width of initial dots
    fn points_width(&self) -> Vec<u16> {
        let mut dots_width = Vec::new();
        for i in 0..self.initial_points_lower.len() { // `for` only for one vector cause they must be same length
            let width = self.initial_points_upper[i].y - self.initial_points_lower[i].y;
            dots_width.push(width);
        };
        dots_width
    }
    ///
    /// Find emissions
    fn emissions(&self, median: f32, mad: f32, threshold: f32) -> Vec<Bond<u16>> {
        let mut emissions = Vec::new();
        for i in 0..self.initial_points_lower.len() { // `for` only for one vector cause they must be same length
            let deviation = ((self.initial_points_upper[i].y - self.initial_points_lower[i].y) as f32 - median).abs();
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
    fn eval(&mut self, _: ()) -> WidthEmissionsCtx {
        let threshold = 1.1;
        let mad_result = MAD::new(self.points_width()).eval(());
        let result = WidthEmissionsCtx {
            result: self.emissions(mad_result.median, mad_result.mad,threshold),
            median: mad_result.median,
            mad: mad_result.mad
        };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of [WidthEmissions]
#[derive(Clone)]
pub struct WidthEmissionsCtx {
    pub result: Vec<Bond<u16>>,
    pub median: f32,
    pub mad: f32
}
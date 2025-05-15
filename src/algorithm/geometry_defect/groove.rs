use sal_core::dbg::Dbg;
use crate::{algorithm::{mad::{bond::Bond, mad::MAD}, width_emissions::width_emissions::WidthEmissions}, domain::{eval::eval::Eval, graham::dot::Dot}};
///
/// Detecting groove on the rope
pub struct Groove {
    initial_points_upper: Vec<Dot<u16>>,
    initial_points_lower: Vec<Dot<u16>>,
    dbg: Dbg,
    result: Option<GrooveCtx>
}
//
//
impl Groove {
    ///
    /// New instance [Groove]
    pub fn new(initial_points_upper: Vec<Dot<u16>>, initial_points_lower: Vec<Dot<u16>>) -> Self {
        Self {
            initial_points_upper,
            initial_points_lower,
            dbg: Dbg::own("Groove"),
            result: None,
        }
    }
}
//
//
impl Eval<(), GrooveCtx> for Groove {
    fn eval(&mut self, _: ()) -> GrooveCtx {
        let width_emissions_result = WidthEmissions::new(
            self.initial_points_upper.clone(),
            self.initial_points_lower.clone()
        ).eval(());
        let mad_of_upper_points = MAD::new(
            self.initial_points_upper
                .iter()
                .map(|dot| dot.y as i16)
                .collect()
        ).eval(());
        let mad_of_lower_points = MAD::new(
            self.initial_points_lower
                .iter()
                .map(|dot| dot.y as i16)
                .collect()
        ).eval(());
        let mut result: Vec<Bond<u16>> = Vec::new();
        let threshold = 1.1;
        for i in (0..width_emissions_result.result.len()-1).step_by(2) {
            let upper_point = width_emissions_result.result[i];
            let lower_point = width_emissions_result.result[i+1];
            let deviation_upper = upper_point.y as f32 - mad_of_upper_points.median;
            let deviation_lower = lower_point.y as f32 - mad_of_lower_points.median;
            // checking groove on lower points
            if (deviation_upper.abs() < threshold * mad_of_upper_points.mad) &&
               (deviation_lower > threshold * mad_of_lower_points.mad) {
                result.push(
                    lower_point
                );
            }
            // checking groove on upper points
            else if (deviation_upper > threshold * mad_of_upper_points.mad) &&
               (deviation_lower.abs() < threshold * mad_of_lower_points.mad) {
                result.push(
                    upper_point
                );
            }
        }
        let result = GrooveCtx {
            result: result,
        };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of [Groove]
#[derive(Clone)]
pub struct GrooveCtx {
    pub result: Vec<Bond<u16>>
}
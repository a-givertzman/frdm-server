use sal_core::dbg::Dbg;
use crate::{algorithm::{mad::{bond::Bond, mad::MAD}, width_emissions::width_emissions::WidthEmissions}, domain::{eval::eval::Eval, graham::dot::Dot}};
///
/// Detecting contraction on the rope
pub struct Contraction {
    initial_points_upper: Vec<Dot<u16>>,
    initial_points_lower: Vec<Dot<u16>>,
    dbg: Dbg,
    result: Option<ContractionCtx>
}
//
//
impl Contraction {
    ///
    /// New instance [Contraction]
    pub fn new(initial_points_upper: Vec<Dot<u16>>, initial_points_lower: Vec<Dot<u16>>) -> Self {
        Self {
            initial_points_upper,
            initial_points_lower,
            dbg: Dbg::own("Contraction"),
            result: None,
        }
    }
}
//
//
impl Eval<(), ContractionCtx> for Contraction {
    fn eval(&mut self, _: ()) -> ContractionCtx {
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
            if (deviation_upper < -threshold * mad_of_upper_points.mad) &&
               (deviation_lower > threshold * mad_of_lower_points.mad) {
                result.push(
                    upper_point
                );
                result.push(
                    lower_point
                );
            }
        }
        let result = ContractionCtx {
            result: result,
        };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of [Contraction]
#[derive(Clone)]
pub struct ContractionCtx {
    pub result: Vec<Bond<u16>>
}
use sal_core::dbg::Dbg;
use crate::{algorithm::{mad::{Bond, Mad}, width_emissions::width_emissions::WidthEmissions}, domain::{Eval, graham::dot::Dot}};

use super::Threshold;
///
/// Detecting expansion on the rope
pub struct Expansion {
    threshold: Threshold,
    initial_points_upper: Vec<Dot<usize>>,
    initial_points_lower: Vec<Dot<usize>>,
    dbg: Dbg,
    result: Option<ExpansionCtx>
}
//
//
impl Expansion {
    ///
    /// New instance [Expansion]
    pub fn new(threshold: Threshold, initial_points_upper: Vec<Dot<usize>>, initial_points_lower: Vec<Dot<usize>>) -> Self {
        Self {
            threshold,
            initial_points_upper,
            initial_points_lower,
            dbg: Dbg::own("Expansion"),
            result: None,
        }
    }
}
//
//
impl Eval<(), ExpansionCtx> for Expansion {
    fn eval(&self, _: ()) -> ExpansionCtx {
        let width_emissions_result = WidthEmissions::new(
            self.threshold,
            self.initial_points_upper.clone(),
            self.initial_points_lower.clone()
        ).eval(());
        let mad_of_upper_points = Mad::new(
            self.initial_points_upper
                .iter()
                .map(|dot| dot.y)
                .collect()
        ).eval(());
        let mad_of_lower_points = Mad::new(
            self.initial_points_lower
                .iter()
                .map(|dot| dot.y)
                .collect()
        ).eval(());
        let mut result: Vec<Bond<usize>> = Vec::new();
        let threshold = 1.1;
        for i in (0..width_emissions_result.result.len()-1).step_by(2) {
            let upper_point = width_emissions_result.result[i];
            let lower_point = width_emissions_result.result[i+1];
            if (((upper_point.y as f64 - mad_of_upper_points.median)) > threshold * mad_of_upper_points.mad) &&
               (((lower_point.y as f64 - mad_of_lower_points.median)) < -threshold * mad_of_lower_points.mad) {
                result.push(
                    upper_point
                );
                result.push(
                    lower_point
                );
            }
        }
        let result = ExpansionCtx {
            result: result,
        };
        self.result = Some(result.clone());
        result
    }
}
///
/// Store result of [Expansion]
#[derive(Clone)]
pub struct ExpansionCtx {
    pub result: Vec<Bond<usize>>
}
use sal_core::dbg::Dbg;
use crate::{algorithm::{mad::{bond::Bond, mad::MAD}, width_emissions::width_emissions::WidthEmissions}, domain::{eval::eval::Eval, graham::dot::Dot}};
///
/// Detecting expansion on the rope
pub struct Expansion {
    initial_points_upper: Vec<Dot<u16>>,
    initial_points_lower: Vec<Dot<u16>>,
    dbg: Dbg,
    result: Option<ExpansionCtx>
}
//
//
impl Expansion {
    ///
    /// New instance [Expansion]
    pub fn new(initial_points_upper: Vec<Dot<u16>>, initial_points_lower: Vec<Dot<u16>>) -> Self {
        Self {
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
    fn eval(&mut self, _: ()) -> ExpansionCtx {
        let width_emissions_result = WidthEmissions::new(
            self.initial_points_upper.clone(),
            self.initial_points_lower.clone()
        ).eval(());
        let mad_of_upper_points = MAD::new(
            self.initial_points_upper
                .iter()
                .map(|dot| dot.y)
                .collect()
        ).eval(());
        let mad_of_lower_points = MAD::new(
            self.initial_points_lower
                .iter()
                .map(|dot| dot.y)
                .collect()
        ).eval(());
        let mut result: Vec<Bond<u16>> = Vec::new();
        let threshold = 1.1;
        for mut i in 0..width_emissions_result.result.len()-1 {
            let upper_point = width_emissions_result.result[i];
            let lower_point = width_emissions_result.result[i+1];
            if (((upper_point.y as f32 - mad_of_upper_points.median)) > threshold * mad_of_upper_points.mad) &&
               (((lower_point.y as f32 - mad_of_lower_points.median)) < -threshold * mad_of_lower_points.mad) {
                result.push(
                    upper_point
                );
                result.push(
                    lower_point
                );
            }
            i+=1;
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
    pub result: Vec<Bond<u16>>
}
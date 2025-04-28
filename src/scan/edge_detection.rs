use opencv::core::MatTraitConst;

use crate::{domain::graham::dot::Dot, infrostructure::camera::pimage::PImage};

pub struct EdgeDetection{
    pub upper_edge: Vec<Dot<isize>>,
    pub lower_edge: Vec<Dot<isize>>,
}
//
//
impl EdgeDetection{
    pub fn new(frame: PImage) -> Self {
        let rows = frame.frame.rows();
        let cols = frame.frame.cols();
        let mut upper_edge = Vec::new();
        let mut lower_edge = Vec::new();
        for col in 0..cols {
            for row in 0..rows {
                if *frame.frame.at_2d::<u8>(row, col).unwrap() >= 100 {
                    upper_edge.push(Dot {x: col as isize, y: row as isize});
                    break;
                }
            }
            for row in (0..rows).rev() {
                if *frame.frame.at_2d::<u8>(row, col).unwrap() >= 100 {
                    lower_edge.push(Dot {x: col as isize, y: row as isize});
                    break;
                }
            }
        }
        Self { upper_edge, lower_edge }
    }
}
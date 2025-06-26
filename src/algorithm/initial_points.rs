use indexmap::IndexMap;
use crate::domain::Dot;
///
/// Storing points of ropes side's
#[derive(Debug, Clone)]
pub struct InitialPoints<T> {
    sides: IndexMap<Side, Vec<Dot<T>>>,
}
impl<T: Copy> InitialPoints<T> {
    ///
    /// 
    pub fn new(upper: Vec<Dot<T>>, lower: Vec<Dot<T>>) -> Self {
        Self {
            sides: IndexMap::from([
                (Side::Upper, upper),
                (Side::Lower, lower),
            ])
        }
    }
    pub fn get(&self, side: Side) -> Vec<Dot<T>> {
        match self.sides.get(&side) {
            Some(side) => side.to_vec(),
            None => vec![],
        }
    }
}
impl<T> Default for InitialPoints<T> {
    fn default() -> Self {
        Self { sides: IndexMap::<Side, Vec<Dot<T>>>::new() }
    }
}
impl<T: PartialEq> PartialEq for InitialPoints<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sides == other.sides
    }
}
///
/// Side of the rope
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Upper,
    Lower,
}
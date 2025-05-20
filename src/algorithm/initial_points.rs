use indexmap::IndexMap;
use crate::domain::graham::dot::Dot;

///
/// TODO
#[derive(Debug, Clone)]
pub struct InitialPoints<T> {
    pub sides: IndexMap<Side, Vec<Dot<T>>>,
}
impl<T: Copy> InitialPoints<T> {
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
///
/// TODO
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Upper,
    Lower,
}
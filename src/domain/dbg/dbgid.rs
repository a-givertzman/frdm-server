use std::fmt::Display;

pub struct DbgId {
    val: String, 
    pub parent: Box<Self>,
}
//
//
impl DbgId {
    pub fn new(parent: Self, me: impl Into<String>) -> Self {
        Self {
            val: format!("{}/{}", parent, me.into()),
            parent: Box::new(parent),
        }
    }
}
//
//
impl Display for DbgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
use std::fmt::Display;

pub struct DbgId {
    val: String, 
    // pub parent: Box<Self>,
}
//
//
impl DbgId {
    ///
    /// Returns [DbgId] new instance without parent
    pub fn root(me: impl Into<String>) -> Self {
        Self {
            val: me.into(),
            // parent: Box::new(parent.clone),
        }
    }
    /// Returns [DbgId] new instance with parent
    pub fn new(parent: &Self, me: impl Into<String>) -> Self {
        Self {
            val: format!("{}/{}", parent, me.into()),
            // parent: Box::new(parent.clone),
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
//
//
impl From<DbgId> for String {
    fn from(value: DbgId) -> Self {
        value.val.clone()
    }
}
//
//
impl From<&DbgId> for String {
    fn from(value: &DbgId) -> String {
        value.val.clone()
    }
}
use std::fmt::Display;
use strum_macros::{AsRefStr, EnumIter};
///
/// The defination of all diagnosis signals
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DiagKeywd {
    Status,
    Connection,
}
//
//
impl DiagKeywd {
    ///
    /// Creates new DiagKeywd from it string representation
    pub fn new(value: &str) -> Self {
        let value = value.to_lowercase();
        match "" {
            _ if value.ends_with(Self::Status.as_str())       => Self::Status,
            _ if value.ends_with(Self::Connection.as_str())   => Self::Connection,
            _ => panic!("DiagKeywd.from_str | Diagnosis point '{}' - does not supported", value)
        }
    }
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
//
//
impl Display for DiagKeywd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
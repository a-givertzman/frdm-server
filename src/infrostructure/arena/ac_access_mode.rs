use std::fmt::Display;

use super::bindings::AC_ACCESS_MODE;


pub enum AcAccessMode {
    NotImplemented,
    NotAvailable,
    WriteOnly,
    ReadOnly,
    ReadWrite,
    Undefined(i32),
}
//
//
impl AcAccessMode {
    #[doc = "< Not implemented"]
    const AC_ACCESS_MODE_NI: AC_ACCESS_MODE = 0;
    #[doc = "< Not available"]
    const AC_ACCESS_MODE_NA: AC_ACCESS_MODE = 1;
    #[doc = "< Write only"]
    const AC_ACCESS_MODE_WO: AC_ACCESS_MODE = 2;
    #[doc = "< Read only"]
    const AC_ACCESS_MODE_RO: AC_ACCESS_MODE = 3;
    #[doc = "< Read and write"]
    const AC_ACCESS_MODE_RW: AC_ACCESS_MODE = 4;
}
//
//
impl From<i32> for AcAccessMode {
    fn from(value: i32) -> Self {
        match value {
            Self::AC_ACCESS_MODE_NI => Self::NotImplemented,
            Self::AC_ACCESS_MODE_NA => Self::NotAvailable,
            Self::AC_ACCESS_MODE_WO => Self::WriteOnly,
            Self::AC_ACCESS_MODE_RO => Self::ReadOnly,
            Self::AC_ACCESS_MODE_RW => Self::ReadWrite,
            _ => Self::Undefined(value)
        }
    }
}
//
//
impl std::fmt::Display for AcAccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcAccessMode::NotImplemented => write!(f, "AcAccessMode::NotImplemented"),
            AcAccessMode::NotAvailable => write!(f, "AcAccessMode::NotAvailable"),
            AcAccessMode::WriteOnly => write!(f, "AcAccessMode::WriteOnly"),
            AcAccessMode::ReadOnly => write!(f, "AcAccessMode::ReadOnly"),
            AcAccessMode::ReadWrite => write!(f, "AcAccessMode::ReadWrite"),
            AcAccessMode::Undefined(val) => write!(f, "AcAccessMode::Undefined({})", val),
        }
    }
}
//
//
impl std::fmt::Debug for AcAccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
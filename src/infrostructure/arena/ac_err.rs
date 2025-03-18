use std::fmt::{Debug, Display};

///
/// Represents AC_ERROR; i32
/// - Contains a string information about error comong from C API
#[derive(PartialEq)]
pub enum AcErr {
    Success,
    Error(String),
    NotInitialized(String),
    NotImplemented(String),
    ResourceInUse(String),
    AccessDenied(String),
    InvalidHandle(String),
    InvalidId(String),
    NoData(String),
    InvalidParameter(String),
    Io(String),
    Timeout(String),
    Abort(String),
    InvalidBuffer(String),
    NotAvailable(String),
    InvalidAddress(String),
    BufferTooSmall(String),
    InvalidIndex(String),
    ParsingChunkData(String),
    InvalidValue(String),
    ResourceExhausted(String),
    OutOfMemory(String),
    Busy(String),
    Custom(String),
    Undefined(isize),
}
//
//
impl AcErr {
    const SUCCESS: &str             = "0: Success, no error";			            // i32 = 0
    const ERROR: &str               = "1001: Generic error";	                    // i32 = -1001
    const NOT_INITIALIZED: &str     = "1002: Arena SDK not initialized";	        // i32 = -1002
    const NOT_IMPLEMENTED: &str     = "1003: Function not implemented";	            // i32 = -1003
    const RESOURCE_IN_USE: &str     = "1004: Resource already in use";	            // i32 = -1004
    const ACCESS_DENIED: &str       = "1005: Incorrect access";	                    // i32 = -1005
    const INVALID_HANDLE: &str      = "1006: Null/incorrect handle";	            // i32 = -1006
    const INVALID_ID: &str          = "1007: Incorrect ID";		                    // i32 = -1007
    const NO_DATA: &str             = "1008: No data available";			        // i32 = -1008
    const INVALID_PARAMETER: &str   = "1009: Null/incorrect parameter";             // i32 = -1009
    const IO: &str                  = "1010: Input/output error";				    // i32 = -1010
    const TIMEOUT: &str             = "1011: Timed out";			                // i32 = -1011
    const ABORT: &str               = "1012: Function aborted";			            // i32 = -1012
    const INVALID_BUFFER: &str      = "1013: Invalid buffer";	                    // i32 = -1013
    const NOT_AVAILABLE: &str       = "1014: Function not available";	            // i32 = -1014
    const INVALID_ADDRESS: &str     = "1015: Invalid register address";	            // i32 = -1015
    const BUFFER_TOO_SMALL: &str    = "1016: Buffer too small";                     // i32 = -1016
    const INVALID_INDEX: &str       = "1017: Invalid index";	                    // i32 = -1017
    const PARSING_CHUNK_DATA: &str  = "1018: Error parsing chunk data";             // i32 = -1018
    const INVALID_VALUE: &str       = "1019: Invalid value";	                    // i32 = -1019
    const RESOURCE_EXHAUSTED: &str  = "1020 Resource cannot perform more actions";  // i32 = -1020
    const OUT_OF_MEMORY: &str       = "1021 Not enough memory";	                    // i32 = -1021
    const BUSY: &str                = "1022 Busy on anothe process";			    // i32 = -1022
    const CUSTOM: &str              = "10000 Start adding custom error LIST here";  // i32 = -10000
}
//
//
impl From<i32> for AcErr {
        fn from(val: i32) -> AcErr {
        match val {
            0      => AcErr::Success,
            -1001  => AcErr::Error(String::from(AcErr::ERROR)),
            -1002  => AcErr::NotInitialized(String::from(AcErr::NOT_INITIALIZED)),
            -1003  => AcErr::NotImplemented(String::from(AcErr::NOT_IMPLEMENTED)),
            -1004  => AcErr::ResourceInUse(String::from(AcErr::RESOURCE_IN_USE)),
            -1005  => AcErr::AccessDenied(String::from(AcErr::ACCESS_DENIED)),
            -1006  => AcErr::InvalidHandle(String::from(AcErr::INVALID_HANDLE)),
            -1007  => AcErr::InvalidId(String::from(AcErr::INVALID_ID)),
            -1008  => AcErr::NoData(String::from(AcErr::NO_DATA)),
            -1009  => AcErr::InvalidParameter(String::from(AcErr::INVALID_PARAMETER)),
            -1010  => AcErr::Io(String::from(AcErr::IO)),
            -1011  => AcErr::Timeout(String::from(AcErr::TIMEOUT)),
            -1012  => AcErr::Abort(String::from(AcErr::ABORT)),
            -1013  => AcErr::InvalidBuffer(String::from(AcErr::INVALID_BUFFER)),
            -1014  => AcErr::NotAvailable(String::from(AcErr::NOT_AVAILABLE)),
            -1015  => AcErr::InvalidAddress(String::from(AcErr::INVALID_ADDRESS)),
            -1016  => AcErr::BufferTooSmall(String::from(AcErr::BUFFER_TOO_SMALL)),
            -1017  => AcErr::InvalidIndex(String::from(AcErr::INVALID_INDEX)),
            -1018  => AcErr::ParsingChunkData(String::from(AcErr::PARSING_CHUNK_DATA)),
            -1019  => AcErr::InvalidValue(String::from(AcErr::INVALID_VALUE)),
            -1020  => AcErr::ResourceExhausted(String::from(AcErr::RESOURCE_EXHAUSTED)),
            -1021  => AcErr::OutOfMemory(String::from(AcErr::OUT_OF_MEMORY)),
            -1022  => AcErr::Busy(String::from(AcErr::BUSY)),
            -10000 => AcErr::Custom(String::from(AcErr::CUSTOM)),
            _      => AcErr::Undefined(val as isize),
        }
    }
}
//
//
impl From<AcErr> for String {
    fn from(value: AcErr) -> Self {
        match value {
            AcErr::Success => format!("AcErr::Success"),
            AcErr::Error(val) => format!("AcErr::Error({})", val),
            AcErr::NotInitialized(val) => format!("AcErr::NotInitialized({})", val),
            AcErr::NotImplemented(val) => format!("AcErr::NotImplemented({})", val),
            AcErr::ResourceInUse(val) => format!("AcErr::ResourceInUse({})", val),
            AcErr::AccessDenied(val) => format!("AcErr::AccessDenied({})", val),
            AcErr::InvalidHandle(val) => format!("AcErr::InvalidHandle({})", val),
            AcErr::InvalidId(val) => format!("AcErr::InvalidId({})", val),
            AcErr::NoData(val) => format!("AcErr::NoData({})", val),
            AcErr::InvalidParameter(val) => format!("AcErr::InvalidParameter({})", val),
            AcErr::Io(val) => format!("AcErr::Io({})", val),
            AcErr::Timeout(val) => format!("AcErr::Timeout({})", val),
            AcErr::Abort(val) => format!("AcErr::Abort({})", val),
            AcErr::InvalidBuffer(val) => format!("AcErr::InvalidBuffer({})", val),
            AcErr::NotAvailable(val) => format!("AcErr::NotAvailable({})", val),
            AcErr::InvalidAddress(val) => format!("AcErr::InvalidAddress({})", val),
            AcErr::BufferTooSmall(val) => format!("AcErr::BufferTooSmall({})", val),
            AcErr::InvalidIndex(val) => format!("AcErr::InvalidIndex({})", val),
            AcErr::ParsingChunkData(val) => format!("AcErr::ParsingChunkData({})", val),
            AcErr::InvalidValue(val) => format!("AcErr::InvalidValue({})", val),
            AcErr::ResourceExhausted(val) => format!("AcErr::ResourceExhausted({})", val),
            AcErr::OutOfMemory(val) => format!("AcErr::OutOfMemory({})", val),
            AcErr::Busy(val) => format!("AcErr::Busy({})", val),
            AcErr::Custom(val) => format!("AcErr::Custom({})", val),
            AcErr::Undefined(val) => format!("AcErr::Undefined({})", val),
        }
    }
}
//
//
impl Display for AcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
//
//
impl Debug for AcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.debug_tuple("Success").finish(),
            Self::Error(val) => f.debug_tuple("Error").field(val).finish(),
            Self::NotInitialized(val) => f.debug_tuple("NotInitialized").field(val).finish(),
            Self::NotImplemented(val) => f.debug_tuple("NotImplemented").field(val).finish(),
            Self::ResourceInUse(val) => f.debug_tuple("ResourceInUse").field(val).finish(),
            Self::AccessDenied(val) => f.debug_tuple("AccessDenied").field(val).finish(),
            Self::InvalidHandle(val) => f.debug_tuple("InvalidHandle").field(val).finish(),
            Self::InvalidId(val) => f.debug_tuple("InvalidId").field(val).finish(),
            Self::NoData(val) => f.debug_tuple("NoData").field(val).finish(),
            Self::InvalidParameter(val) => f.debug_tuple("InvalidParameter").field(val).finish(),
            Self::Io(val) => f.debug_tuple("Io").field(val).finish(),
            Self::Timeout(val) => f.debug_tuple("Timeout").field(val).finish(),
            Self::Abort(val) => f.debug_tuple("Abort").field(val).finish(),
            Self::InvalidBuffer(val) => f.debug_tuple("InvalidBuffer").field(val).finish(),
            Self::NotAvailable(val) => f.debug_tuple("NotAvailable").field(val).finish(),
            Self::InvalidAddress(val) => f.debug_tuple("InvalidAddress").field(val).finish(),
            Self::BufferTooSmall(val) => f.debug_tuple("BufferTooSmall").field(val).finish(),
            Self::InvalidIndex(val) => f.debug_tuple("InvalidIndex").field(val).finish(),
            Self::ParsingChunkData(val) => f.debug_tuple("ParsingChunkData").field(val).finish(),
            Self::InvalidValue(val) => f.debug_tuple("InvalidValue").field(val).finish(),
            Self::ResourceExhausted(val) => f.debug_tuple("ResourceExhausted").field(val).finish(),
            Self::OutOfMemory(val) => f.debug_tuple("OutOfMemory").field(val).finish(),
            Self::Busy(val) => f.debug_tuple("Busy").field(val).finish(),
            Self::Custom(val) => f.debug_tuple("Custom").field(val).finish(),
            Self::Undefined(val) => f.debug_tuple("Undefined").field(val).finish(),
        }
    }
}

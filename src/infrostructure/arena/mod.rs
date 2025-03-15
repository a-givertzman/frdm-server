// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
pub mod bindings;

pub const AC_ERR_SUCCESS: i32 = 0;				        // Success, no error
pub const AC_ERR_ERROR: i32 = -1001;			        // Generic error
pub const AC_ERR_NOT_INITIALIZED: i32 = -1002;	        // Arena SDK not initialized
pub const AC_ERR_NOT_IMPLEMENTED: i32 = -1003;	        // Function not implemented
pub const AC_ERR_RESOURCE_IN_USE: i32 = -1004;	        // Resource already in use
pub const AC_ERR_ACCESS_DENIED: i32 = -1005;	        // Incorrect access
pub const AC_ERR_INVALID_HANDLE: i32 = -1006;	        // Null/incorrect handle
pub const AC_ERR_INVALID_ID: i32 = -1007;		        // Incorrect ID
pub const AC_ERR_NO_DATA: i32 = -1008;			        // No data available
pub const AC_ERR_INVALID_PARAMETER: i32 = -1009;          // Null/incorrect parameter
pub const AC_ERR_IO: i32 = -1010;				        // Input/output error
pub const AC_ERR_TIMEOUT: i32 = -1011;			        // Timed out
pub const AC_ERR_ABORT: i32 = -1012;			        // Function aborted
pub const AC_ERR_INVALID_BUFFER: i32 = -1013;	        // Invalid buffer
pub const AC_ERR_NOT_AVAILABLE: i32 = -1014;	        // Function not available
pub const AC_ERR_INVALID_ADDRESS: i32 = -1015;	        // Invalid register address
pub const AC_ERR_BUFFER_TOO_SMALL: i32 = -1016;           // Buffer too small
pub const AC_ERR_INVALID_INDEX: i32 = -1017;	        // Invalid index
pub const AC_ERR_PARSING_CHUNK_DATA: i32 = -1018;         // Error parsing chunk data
pub const AC_ERR_INVALID_VALUE: i32 = -1019;	        // Invalid value
pub const AC_ERR_RESOURCE_EXHAUSTED: i32 = -1020;         // Resource cannot perform more actions
pub const AC_ERR_OUT_OF_MEMORY: i32 = -1021;	        // Not enough memory
pub const AC_ERR_BUSY: i32 = -1022;			        // Busy on anothe process
pub const AC_ERR_CUSTOM: i32 = -10000;		        // Start adding custom error LIST here

// #[doc = " @typedef AC_ERROR;\n\n Integer representation of the error enum (AC_ERROR_LIST)."]
// pub type AC_ERROR = i32;

// #[doc = " @enum AC_ERROR_LIST\n\n This enum represents the different errors that a function might return.\n\n @warning\n  - Use AC_ERROR integer values in place of AC_ERROR_LIST enum values"]
// pub type AC_ERROR_LIST = ::std::os::raw::c_int;


// #[doc = " @typedef AC_ERROR;\n\n Success, no error"]
// pub const AC_ERR_SUCCESS: AC_ERROR = 0;

// #[doc = "< Success, no error"]
// pub const AC_ERROR_LIST_AC_ERR_SUCCESS: AC_ERROR_LIST = 0;

// #[doc = " @typedef acSystem;\n\n Representation of the system object, the entry point into Arena SDK."]
// pub type acSystem = *mut ::std::os::raw::c_void;
// #[doc = " @typedef acDevice;\n\n Represents a device, used to configure and stream a device."]
// pub type acDevice = *mut ::std::os::raw::c_void;
// //
// //
// unsafe extern "C" {
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acOpenSystem(acSystem* phSystem)\n\n @param phSystem\n  - Type: acSystem*\n  - [Out] parameter\n  - The system object\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acOpenSystem </B> initializes the Arena SDK and retrieves the system\n object (acSystem). The system must be closed, or memory will leak.\n\n @see\n  - acSystem"]
//     pub fn acOpenSystem(phSystem: *mut acSystem) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acCloseSystem(acSystem hSystem)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acCloseSystem </B> cleans up the system (acSystem) and deinitializes the\n Arena SDK, deallocating all memory.\n\n @see\n  - acSystem"]
//     pub fn acCloseSystem(hSystem: acSystem) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemUpdateDevices(acSystem hSystem, uint64_t timeout)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param timeout\n  - Type: uint64_t\n  - [In] parameter\n  - Time to wait for connected devices to respond\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemUpdateDevices </B> updates the internal list of devices, (along\n with their relevant interfaces). It must be called before retrieving the\n number of devices (acSystemGetNumDevices) or any time that an updated device\n list might be necessary.\n\n @see\n  - acSystemGetNumDevices"]
//     pub fn acSystemUpdateDevices(hSystem: acSystem, timeout: u64) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemGetNumDevices(acSystem hSystem, size_t* pNumDevices)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param pNumDevices\n  - Type: size_t*\n  - [Out] parameter\n  - The number of discovered devices\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemGetNumDevices </B> retrieves the number of discovered devices. It\n must be called after updating the internal list of devices\n (acSystemUpdateDevices).\n\n @see\n  - acSystemUpdateDevices"]
//     pub fn acSystemGetNumDevices(hSystem: acSystem, pNumDevices: *mut usize) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemGetDeviceModel(acSystem hSystem, size_t index, char* pModelNameBuf, size_t* pBufLen)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param index\n  - Type: size_t\n  - [In] parameter\n  - Index of the device\n\n @param pModelNameBuf\n  - Type: char*\n  - [Out] parameter\n  - Accepts null\n  - Model name of the device\n\n @param pBufLen\n  - Type: size_t*\n  - [In/out] parameter\n  - (In) Length of the buffer\n  - (Out) Length of the value\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemGetDeviceModel </B> gets the model name of a device."]
//     pub fn acSystemGetDeviceModel(
//         hSystem: acSystem,
//         index: usize,
//         pModelNameBuf: *mut ::std::os::raw::c_char,
//         pBufLen: *mut usize,
//     ) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemGetDeviceSerial(acSystem hSystem, size_t index, char* pSerialNumberBuf, size_t* pBufLen)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param index\n  - Type: size_t\n  - [In] parameter\n  - Index of the device\n\n @param pSerialNumberBuf\n  - Type: char*\n  - [Out] parameter\n  - Serial number of the device\n\n @param pBufLen\n  - Type: size_t*\n  - [In/out] parameter\n  - (In) Length of the buffer\n  - (Out) Length of the value\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acDeviceGetDeviceSerial </B> gets the serial number of a device. A serial\n number differentiates between devices. Each LUCID device has a unique serial\n number. LUCID serial numbers are numeric, but the serial numbers of other\n vendors may be alphanumeric."]
//     pub fn acSystemGetDeviceSerial(
//         hSystem: acSystem,
//         index: usize,
//         pSerialNumberBuf: *mut ::std::os::raw::c_char,
//         pBufLen: *mut usize,
//     ) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemGetDeviceIpAddressStr(acSystem hSystem, size_t index, char* pIpAddressStr, size_t* pBufLen)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param index\n  - Type: size_t\n  - [In] parameter\n  - Index of the device\n\n @param pIpAddressStr\n  - Type: char*\n  - [Out] parameter\n  - Accepts null\n  - IP address as a dot-separated string\n\n @param pBufLen\n  - Type: size_t*\n  - [In/out] parameter\n  - (In) Length of the buffer\n  - (Out) Length of the value\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemGetDeviceIpAddressStr </B> gets the IP address of a device on the\n network, returning it as a string."]
//     pub fn acSystemGetDeviceIpAddressStr(
//         hSystem: acSystem,
//         index: usize,
//         pIpAddressStr: *mut ::std::os::raw::c_char,
//         pBufLen: *mut usize,
//     ) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemCreateDevice(acSystem hSystem, size_t index, acDevice* phDevice)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param index\n  - Type: size_t\n  - [In] parameter\n  - Index of the device\n\n @param phDevice\n  - Type: acDevice*\n  - [Out] parameter\n  - Initialized, ready-to-use device\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemCreateDevice </B> creates and initializes a device. It must be\n called after the device list has been updated (acSystemUpdateDevices). The\n device must be destroyed (acSystemDestroyDevice) when no longer needed.\n\n @see\n  - acSystemUpdateDevices\n  - acSystemDestroyDevice"]
//     pub fn acSystemCreateDevice(
//         hSystem: acSystem,
//         index: usize,
//         phDevice: *mut acDevice,
//     ) -> AC_ERROR;
//     //
//     //
//     #[doc = " @fn AC_ERROR AC_API acSystemDestroyDevice(acSystem hSystem, acDevice hDevice)\n\n @param hSystem\n  - Type: acSystem\n  - [In] parameter\n  - The system object\n\n @param hDevice\n  - Type: acDevice\n  - [In] parameter\n  - Device to destroy\n\n @return\n  - Type: AC_ERROR\n  - Error code for the function\n  - Returns AC_ERR_SUCCESS (0) on success\n\n <B> acSystemDestroyDevice </B> destroys and cleans up the internal memory of a\n device (acDevice). Devices that have been created (acSystemCreateDevice) must\n be destroyed.\n\n @see\n  - acDevice\n  - acSystemCreateDevice"]
//     pub fn acSystemDestroyDevice(hSystem: acSystem, hDevice: acDevice) -> AC_ERROR;

// }

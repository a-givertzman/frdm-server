// #![allow(non_upper_case_globals)]
// #![allow(non_snake_case)]
#![allow(non_camel_case_types)]
pub mod bindings;
pub mod ac_device;
pub mod ac_system;
pub mod ac_err;
pub mod ac_access_mode;








    // pub struct AcNode {
    //     pub node: super::bindings::acNode,
    //     pub access: AcAccessMode, 
    // }
    // ///
    // /// Returns a node with its access mode..
    // /// - `node_map` - A node map
    // /// - `node_name` - Name of the node to retrieve
    // pub fn acNodeMapGetNodeAndAccessMode(node_map: super::bindings::acNodeMap, node_name: &str) -> Result<AcNode, AcErr> {
    //     unsafe {
    //         let mut h_node: super::bindings::acNode = std::ptr::null_mut();
    //         let mut access_mode: super::bindings::AC_ACCESS_MODE = 0;
    //         let node_name = CString::new(node_name).unwrap();
    //         let err = super::bindings::acNodeMapGetNodeAndAccessMode(
    //             node_map,
    //             node_name.as_ptr(),
    //             &mut h_node,
    //             &mut access_mode,
    //         );
    //         match err {
    //             0 => Ok(AcNode {
    //                 node: h_node,
    //                 access: AcAccessMode::from(access_mode),
    //             }),
    //             _ => Err(AcErr::from(err)),
    //         }
    //     }
    // }
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::{AddWeightedConf, BitwiseAndConf};

///
/// ## Configuration for fast-edges algorithm
/// 
/// Specify one of `add-weighted` or `bitwise-and` two calculate union of two images
/// 
/// ### Example:
/// ```yaml
/// union:
///     add-weighted:           # Weighted sum of two images to be calculated
///         weight1: 1.0            # Weight of the first array elements.
///         weight2: 1.0            # Weight of the second array elements.
///         gamma: 0.0              Scalar added to the result, default 0.0
///     bitwise-and:            # Bitwise AND of two images to be calculated
///         no-params: no parameters required
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnionConf {
    /// Specify if weighted sum of two images have to be calculated
    pub add_weighted: Option<AddWeightedConf>,
    /// Specify if bitwise AND operation of two images have to be calculated
    pub bitwise_and: Option<BitwiseAndConf>,
}
//
// 
impl UnionConf {
    ///
    /// Returns [UnionConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "UnionConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let add_weighted = conf.get("add-weighted").map(|conf| AddWeightedConf::new(&name, conf));
        log::trace!("{dbg}.new | add-weighted: {:#?}", add_weighted);
        let bitwise_and = conf.get("bitwise-and").map(|conf| BitwiseAndConf::new(&name, conf));
        log::trace!("{dbg}.new | bitwise-and: {:#?}", bitwise_and);
        match (add_weighted, bitwise_and) {
            (None, None) => panic!("{dbg}.new | One of 'add-weighted' / `bitwise-and` - have to be specified"),
            (Some(_), Some(_)) => panic!("{dbg}.new | Both: 'add-weighted' and `bitwise-and` - are specified, please use one of"),
            _ => {},
        }
        Self {
            add_weighted,
            bitwise_and,
        }
    }
}
//
//
impl Default for UnionConf {
    fn default() -> Self {
        Self {
            add_weighted: None,
            bitwise_and: Some(BitwiseAndConf::default()),
        }
    }
}

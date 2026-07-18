use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::{AddWeightedConf, BitwiseAndConf, BitwiseOrConf};

///
/// ## Configuration for fast-edges algorithm
/// 
/// Specify one of `add-weighted` or `bitwise-and` two calculate union of two images
/// 
/// ### Example:
/// ```yaml
/// union:  # use just one of option
///     add-weighted:           # Weighted sum of two images to be calculated
///         weight1: 1.0            # Weight of the first array elements.
///         weight2: 1.0            # Weight of the second array elements.
///         gamma: 0.0              Scalar added to the result, default 0.0
///     bitwise-and:            # Bitwise AND of two images to be calculated
///         no-params: no parameters required
///     bitwise-or:            # Bitwise OR of two images to be calculated
///         no-params: no parameters required
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnionConf {
    /// Specify a way to compose of two images
    pub kind: UnionKindConf,
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
        let bitwise_or = conf.get("bitwise-or").map(|conf| BitwiseOrConf::new(&name, conf));
        log::trace!("{dbg}.new | bitwise-or: {:#?}", bitwise_or);
        let kind = match (add_weighted, bitwise_and, bitwise_or) {
            (None, None, None) => UnionKindConf::BitwiseOr(BitwiseOrConf::default()),
            (None, None, Some(conf)) => UnionKindConf::BitwiseOr(conf),
            (None, Some(conf), None) => UnionKindConf::BitwiseAnd(conf),
            (None, Some(_), Some(conf)) => {
                log::warn!("{dbg}.new | Bitwise-And and Bitwise-Or - both specified, by default Bitwise-Or used");
                UnionKindConf::BitwiseOr(conf)
            }
            (Some(conf), None, None) => UnionKindConf::AddWeighted(conf),
            (Some(_), None, Some(conf)) => {
                log::warn!("{dbg}.new | Add-Weighted and Bitwise-Or - both specified, by default Bitwise-Or used");
                UnionKindConf::BitwiseOr(conf)
            }
            (Some(conf), Some(_), None) => {
                log::warn!("{dbg}.new | Add-Weighted and Bitwise-And - both specified, by default Add-Weighted used");
                UnionKindConf::AddWeighted(conf)
            }
            (Some(_), Some(_), Some(conf)) => {
                log::warn!("{dbg}.new | Add-Weighted, Bitwise-And, Bitwise-Or - all specified, by default Bitwise-Or used");
                UnionKindConf::BitwiseOr(conf)
            }
        };
        Self {
            kind,
        }
    }
}
//
//
impl Default for UnionConf {
    fn default() -> Self {
        Self {
            kind: UnionKindConf::BitwiseOr(BitwiseOrConf::default()),
        }
    }
}
///
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnionKindConf {
    /// Specify if weighted sum of two images have to be calculated
    AddWeighted(AddWeightedConf),
    /// Specify if bitwise AND operation of two images have to be calculated
    BitwiseAnd(BitwiseAndConf),
    /// Specify if bitwise OR operation of two images have to be calculated
    BitwiseOr(BitwiseOrConf),
}
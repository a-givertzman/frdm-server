use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};

///
/// ## Configuration for `AddWeighted` algorithm
///
/// `AddWeighted` Calculates the weighted sum of two arrays.
/// The function addWeighted calculates the weighted sum of two arrays as follows: block formula where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each channel is processed independently. The function can be replaced with a matrix expression:
///   dst = src1*alpha + src2*beta + gamma;
/// Note: Saturation is not applied when the output array has the depth CV_32S. You may even get result of an incorrect sign in the case of overflow.
///
/// ### Example:
/// ```yaml
/// add-weighted:
///     weight1: 1.0            # Weight of the first array elements.
///     weight2: 1.0            # Weight of the second array elements.
///     gamma: 0.0
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AddWeightedConf {
    /// Weight of the first array elements
    pub weight1: f64,
    /// Weight of the second array elements.
    pub weight2: f64,
    /// Scalar added to the result, default 0.0
    pub gamma: f64,
}
//
// 
impl AddWeightedConf {
    ///
    /// Returns [AddWeightedConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "AddWeightedConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let weight1 = conf.get("weight1").expect(&format!("{dbg}.new | 'weight1' - not found or wrong configuration"));
        log::trace!("{dbg}.new | weight1: {:#?}", weight1);
        let weight2 = conf.get("weight2").expect(&format!("{dbg}.new | 'weight2' - not found or wrong configuration"));
        log::trace!("{dbg}.new | weight2: {:#?}", weight1);
        let gamma = conf.get("gamma").unwrap_or(0.0);
        log::trace!("{dbg}.new | gamma: {:#?}", gamma);
        Self {
            weight1,
            weight2,
            gamma,
        }
    }
}
//
//
impl Default for AddWeightedConf {
    fn default() -> Self {
        Self {
            weight1: 1.0,
            weight2: 1.0,
            gamma: 0.0,
        }
    }
}

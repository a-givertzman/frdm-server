use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{FastContoursConf, FastEdgesConf, RopeDimensionsConf, TemporalFilterConf, Threshold}, conf::UnionConf};

///
/// `FastScan` algorithm configuration
/// 
/// ### Example
/// ```yaml
/// fast-scan:
///     fast-contours:
///         otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
///     temporal-filter:
///         gaussian:
///             kernel: [11, 11]    # Gausian blur kernel size, must be odd
///             sigma: [0.0, 0.0]   # Standard deviation in [X, Y] direction, The higher the value, the more pixels are used to count each pixel and the smoother blur will be
///         open-kernel: [3, 3]     # Morphology open operation kernel size [w, h], default [5, 5]
///         erode-kernel: [3, 3]    # Morphology erode operation kernel size [w, h], default [5, 5]
///         threshold: 12.0         # Threshold to detect the pixel whas changed or not in the each next frame
///     fast-edges:
///         otsu-tune: 1.40         # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
///         # threshold: 128        # 0...255, used if otsu-tune is not specified
///         smooth: 36              # Smoothing of edge line factor. The higher the factor the smoother the line.
///     union:
///         add-weighted:               # Combine two images
///             weight1: 1.0            # Weight of the first array elements.
///             weight2: 1.0            # Weight of the second array elements.
///             gamma: 0.0              # Scalar added to the result, default 0.0
///     rope-dimensions:        # Verifaing the rope dimensions 
///         rope-width: 380               # Standart rope width, px
///         width-tolerance: 25.0         # Tolerance for rope width, %
///         square-tolerance: 100.0       # Tolerance for rope square, %
//      distortion-threshold: 1.0    # 1.0..1.5, threshold to detect the rope distortions
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FastScanConf {
    pub fast_contours: FastContoursConf,
    /// Configuration for `Temporal Filter`
    pub temporal_filter: TemporalFilterConf,
    pub fast_edges: FastEdgesConf,
    pub union: UnionConf,
    pub rope_dimensions: RopeDimensionsConf,
    pub distortion_threshold: Threshold,
}
impl FastScanConf {
    ///
    /// Returns [FastScanConf] built from `ConfTree`:
    #[allow(unused)]
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FastScanConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fast_contours = conf.get("fast-contours").expect(&format!("{dbg}.new | 'fast-contours' - not found or wrong configuration"));
        let fast_contours = FastContoursConf::new(&name, fast_contours);
        log::trace!("{dbg}.new | fast-contours: {:#?}", fast_contours);
        let temporal_filter = conf.get("temporal-filter").expect(&format!("{dbg}.new | 'temporal-filter' - not found or wrong configuration"));
        let temporal_filter = TemporalFilterConf::new(&name, temporal_filter);
        log::trace!("{dbg}.new | temporal-filter: {:#?}", temporal_filter);
        let fast_edges = conf.get("fast-edges").expect(&format!("{dbg}.new | 'fast-edges' - not found or wrong configuration"));
        let fast_edges = FastEdgesConf::new(&name, fast_edges);
        log::trace!("{dbg}.new | fast-edges: {:#?}", fast_edges);
        let union = conf.get("union").expect(&format!("{dbg}.new | 'union' - not found or wrong configuration"));
        let union = UnionConf::new(&name, union);
        log::trace!("{dbg}.new | union: {:#?}", union);
        let rope_dimensions = conf.get("rope-dimensions").expect(&format!("{dbg}.new | 'rope-dimensions' - not found or wrong configuration"));
        let rope_dimensions = RopeDimensionsConf::new(&name, rope_dimensions);
        log::trace!("{dbg}.new | rope-dimensions: {:#?}", rope_dimensions);
        let distortion_threshold = conf.get("distortion-threshold").expect(&format!("{dbg}.new | 'distortion-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | distortion-threshold: {:?}", distortion_threshold);
        Self {
            fast_contours,
            temporal_filter,
            fast_edges,
            union,
            rope_dimensions,
            distortion_threshold: Threshold(distortion_threshold),
        }
    }
}
//
//
impl Default for FastScanConf {
    fn default() -> Self {
        Self {
            fast_contours: Default::default(),
            temporal_filter: Default::default(),
            fast_edges: Default::default(),
            union: UnionConf::default(),
            rope_dimensions: Default::default(),
            distortion_threshold: Default::default(),
        }
    }
}
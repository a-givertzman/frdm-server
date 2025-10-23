use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{
    FineContoursConf, FineEdgesConf, RopeDimensionsConf, TemporalFilterConf, Threshold
}, conf::UnionConf};

///
/// `FineScan` algorithm configuration
/// 
/// ### Example
/// ```yaml
/// fine-scan:
///     fine-contours:
///         otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
///         merge-distance: 24.0    # Maximum distance between contours to be merged
///     temporal-filter:
///         gaussian:
///             kernel: [11, 11]    # Gausian blur kernel size, must be odd
///             sigma: [0.0, 0.0]   # Standard deviation in [X, Y] direction, The higher the value, the more pixels are used to count each pixel and the smoother blur will be
///         open-kernel: [3, 3]     # Morphology open operation kernel size [w, h], default [5, 5]
///         erode-kernel: [3, 3]    # Morphology erode operation kernel size [w, h], default [5, 5]
///         threshold: 12.0         # Threshold to detect the pixel whas changed or not in the each next frame
///     fine-edges:
///         otsu-tune: 1.40             # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
///         # threshold: 128            # 0...255, used if otsu-tune is not specified
///         smooth: 36                  # Smoothing of edge line factor. The higher the factor the smoother the line.
///     union:
///         add-weighted:               # Combine two images
///             weight1: 1.0            # Weight of the first array elements.
///             weight2: 1.0            # Weight of the second array elements.
///             gamma: 0.0              # Scalar added to the result, default 0.0
///     rope-dimensions:            # Verifaing the rope dimensions 
///         rope-width: 380               # Standart rope width, px
///         width-tolerance: 25.0         # Tolerance for rope width, %
///         square-tolerance: 100.0       # Tolerance for rope square, %
///     distortion-threshold: 1.2      # 1.1..1.3, threshold to detect the rope distortions
///     defect-threshold: 1.2          # 1.1..1.3, threshold to detect rope geometry defects
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FineScanConf {
    pub fine_contours: FineContoursConf,
    pub temporal_filter: TemporalFilterConf,
    pub fine_edges: FineEdgesConf,
    pub union: UnionConf,
    pub rope_dimensions: RopeDimensionsConf,
    pub distortion_threshold: Threshold,
    pub defect_threshold: Threshold,
}
//
//
impl FineScanConf {
    ///
    /// Returns [FineScanConf] built from `ConfTree`:
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FineScanConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fine_contours = conf.get("fine-contours").expect(&format!("{dbg}.new | 'fine-contours' - not found or wrong configuration"));
        let fine_contours = FineContoursConf::new(&name, fine_contours);
        log::trace!("{dbg}.new | fine-contours: {:?}", fine_contours);
        let temporal_filter = conf.get("temporal-filter").expect(&format!("{dbg}.new | 'temporal-filter' - not found or wrong configuration"));
        let temporal_filter = TemporalFilterConf::new(&name, temporal_filter);
        log::trace!("{dbg}.new | temporal-filter: {:#?}", temporal_filter);
        let fine_edges = conf.get("fine-edges").expect(&format!("{dbg}.new | 'fine-edges' - not found or wrong configuration"));
        let fine_edges = FineEdgesConf::new(&name, fine_edges);
        log::trace!("{dbg}.new | fine-edges: {:#?}", fine_edges);
        let union = conf.get("union").expect(&format!("{dbg}.new | 'union' - not found or wrong configuration"));
        let union = UnionConf::new(&name, union);
        log::trace!("{dbg}.new | union: {:#?}", union);
        let rope_dimensions = conf.get("rope-dimensions").expect(&format!("{dbg}.new | 'rope-dimensions' - not found or wrong configuration"));
        let rope_dimensions = RopeDimensionsConf::new(&name, rope_dimensions);
        log::trace!("{dbg}.new | rope-dimensions: {:#?}", rope_dimensions);
        let distortion_threshold = conf.get("distortion-threshold").expect(&format!("{dbg}.new | 'distortion-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | distortion-threshold: {:?}", distortion_threshold);
        let defect_threshold = conf.get("defect-threshold").expect(&format!("{dbg}.new | 'defect-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | defect-threshold: {:?}", defect_threshold);
        Self {
            fine_contours,
            temporal_filter,
            fine_edges,
            union,
            rope_dimensions,
            distortion_threshold: Threshold(distortion_threshold),
            defect_threshold: Threshold(defect_threshold),
        }
    }
}
//
//
impl Default for FineScanConf {
    fn default() -> Self {
        Self {
            fine_contours: FineContoursConf::default(),
            temporal_filter: TemporalFilterConf::default(),
            fine_edges: FineEdgesConf::default(),
            union: UnionConf::default(),
            rope_dimensions: RopeDimensionsConf::default(),
            distortion_threshold: Threshold::default(),
            defect_threshold: Threshold::default(),
        }
    }
}

use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{FastContoursConf, RopeDimensionsConf, TemporalFilterConf, Threshold}, conf::EdgeDetectionConf};

///
/// The application configuration
/// 
/// ### Example
/// ```yaml
/// fast-scan:
///     union:
///         add-weighted:               # Combine two images
///             weight1: 1.0            # Weight of the first array elements.
///             weight2: 1.0            # Weight of the second array elements.
///             gamma: 0.0
///         fast-contours:
///             cropping:
///                 x: 230              # New left edge
///                 y: 300              # New top edge
///                 width: 1410         # New image width
///                 height: 1000        # New image height
///             gamma:
///                 factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
///             otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
///         temporal-filter:
///             open-kernel: [3, 3]     # Morphology open operation kernel size [w, h], default [5, 5]
///             erode-kernel: [3, 3]    # Morphology erode operation kernel size [w, h], default [5, 5]
///             threshold: 12.0         # Threshold to detect the pixel whas changed or not in the each next frame
///     edge-detection:
///         otsu-tune: 1.40             # Multiplier to otsu auto threshold, 1.0 - do nothing, just use otsu auto threshold, default 1.0
///         # threshold: 128            # 0...255, used if otsu-tune is not specified
///         smooth: 36                  # Smoothing of edge line factor. The higher the factor the smoother the line.
///     rope-dimensions:            # Verifaing the rope dimensions 
///         rope-width: 380               # Standart rope width, px
///         width-tolerance: 25.0         # Tolerance for rope width, %
///         square-tolerance: 100.0       # Tolerance for rope square, %
///     geometry-defect-threshold: 1.0    # 1.1..1.3, absolute threshold to detect the geometry deffects
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FastScanConf {
    pub fast_contours: FastContoursConf,
    /// Configuration for `Temporal Filter`
    pub temporal_filter: TemporalFilterConf,
    pub edge_detection: EdgeDetectionConf,
    pub rope_dimensions: RopeDimensionsConf,
    pub geometry_defect_threshold: Threshold,
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
        let edge_detection = conf.get("edge-detection").expect(&format!("{dbg}.new | 'edge-detection' - not found or wrong configuration"));
        let edge_detection = EdgeDetectionConf::new(&name, edge_detection);
        log::trace!("{dbg}.new | edge-detection: {:#?}", edge_detection);
        let rope_dimensions = conf.get("rope-dimensions").expect(&format!("{dbg}.new | 'rope-dimensions' - not found or wrong configuration"));
        let rope_dimensions = RopeDimensionsConf::new(&name, rope_dimensions);
        log::trace!("{dbg}.new | rope-dimensions: {:#?}", rope_dimensions);
        let geometry_defect_threshold = conf.get("geometry-defect-threshold").unwrap();
        let geometry_defect_threshold = Threshold(geometry_defect_threshold);
        log::trace!("{dbg}.new | geometry-defect-threshold: {:?}", geometry_defect_threshold);
        Self {
            fast_contours,
            temporal_filter,
            edge_detection,
            rope_dimensions,
            geometry_defect_threshold,
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
            edge_detection: Default::default(),
            rope_dimensions: Default::default(),
            geometry_defect_threshold: Default::default(),
        }
    }
}
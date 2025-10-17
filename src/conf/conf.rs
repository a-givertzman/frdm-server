use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{FastScanConf, FineScanConf}, conf::NormalizeConf};

///
/// The application configuration
/// 
/// ### Example
/// ```yaml
/// normalize:
///     cropping:
///         x: 230           # New left edge
///         y: 300           # New top edge
///         width: 1410      # New image width
///         height: 1000     # New image height
///     gamma:
///         factor: 120.0    # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
/// fast-scan:
///     add-weighted:               # Combine two images
///         weight1: 1.0            # Weight of the first array elements.
///         weight2: 1.0            # Weight of the second array elements.
///         gamma: 0.0
///     fast-contours:
///         cropping:
///             x: 230              # New left edge
///             y: 300              # New top edge
///             width: 1410         # New image width
///             height: 1000        # New image height
///         gamma:
///             factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
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
///     rope-dimensions:        # Verifaing the rope dimensions 
///         rope-width: 380               # Standart rope width, px
///         width-tolerance: 25.0         # Tolerance for rope width, %
///         square-tolerance: 100.0       # Tolerance for rope square, %
///     geometry-defect-threshold: 1.0    # 1.1..1.3, absolute threshold to detect the geometry deffects
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
///     rope-dimensions:            # Verifaing the rope dimensions 
///         rope-width: 380               # Standart rope width, px
///         width-tolerance: 25.0         # Tolerance for rope width, %
///         square-tolerance: 100.0       # Tolerance for rope square, %
///     geometry-defect-threshold: 1.0    # 1.1..1.3, absolute threshold to detect the geometry deffects
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Conf {
    pub normalize: NormalizeConf,
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
impl Conf {
    ///
    /// Returns [Conf] built from `ConfTree`:
    #[allow(unused)]
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "Conf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fast_scan = conf.get("fast-scan").expect(&format!("{dbg}.new | 'fast-scan' - not found or wrong configuration"));
        let fast_scan = FastScanConf::new(&name, fast_scan);
        log::trace!("{dbg}.new | fast-scan: {:#?}", fast_scan);
        let fine_scan = conf.get("fine-scan").expect(&format!("{dbg}.new | 'fine-scan' - not found or wrong configuration"));
        let fine_scan = FineScanConf::new(&name, fine_scan);
        log::trace!("{dbg}.new | fine-scan: {:#?}", fine_scan);
        let normalize = conf.get("normalize").expect(&format!("{dbg}.new | 'normalize' - not found or wrong configuration"));
        let normalize = NormalizeConf::new(&name, normalize);
        log::trace!("{dbg}.new | normalize: {:#?}", normalize);
        Self {
            normalize,
            fast_scan,
            fine_scan,
        }
    }
}

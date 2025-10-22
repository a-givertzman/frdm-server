use crate::algorithm::{
    FastEdgesCtx, FastContoursCtx, FastUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx, RopeDefectCtx,
};
///
/// `FastScan` algorithm results
#[derive(Debug, Clone)]
pub struct FastScanCtx {
    /// Filtered and binarised image
    pub fast_contours: FastContoursCtx,
    /// `TemporalFilter` result
    pub temporal_filter: TemporalFilterCtx<FastScanCtx>,
    /// `FastUnion` result contour
    pub union: FastUnionCtx,
    /// Points of rope perimeter
    pub edges: FastEdgesCtx,
    /// Rope calculated dimensions
    pub rope_dimensions: RopeDimensionsCtx<FastScanCtx>,
    /// Result of rope `WidthEmissions`
    pub width_emissions: RopeDistortionsCtx<FastScanCtx>,
    /// Rope geometry defects
    pub defects: RopeDefectCtx<FastScanCtx>,

}
//
//
impl Default for FastScanCtx {
    fn default() -> Self {
        Self {
            fast_contours: FastContoursCtx::default(),
            temporal_filter: TemporalFilterCtx::default(),
            union: FastUnionCtx::default(),
            edges: FastEdgesCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            width_emissions: RopeDistortionsCtx::default(),
            defects: RopeDefectCtx::default(),
        }
    }
}

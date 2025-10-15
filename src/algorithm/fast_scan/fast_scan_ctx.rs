use crate::algorithm::{
    FastEdgesCtx, FastContoursCtx, FastUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, WidthEmissionsCtx, GeometryDefectCtx,
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
    pub width_emissions: WidthEmissionsCtx<FastScanCtx>,
    /// Rope geometry defects
    pub defects: GeometryDefectCtx<FastScanCtx>,

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
            width_emissions: WidthEmissionsCtx::default(),
            defects: GeometryDefectCtx::default(),
        }
    }
}

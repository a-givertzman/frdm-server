use crate::{
    algorithm::{FastEdgesCtx, FastContoursCtx, FastUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, WidthEmissionsCtx, GeometryDefectCtx, ResultCtx},
    domain::Image,
};
///
/// `FastScan` algorithm results
#[derive(Debug, Clone)]
pub struct FastScanCtx {
    /// Result of `FastScan` algorithm
    pub result: ResultCtx<Image>,
    /// Filtered and binarised image
    pub fast_contours: FastContoursCtx,
    /// `TemporalFilter` result
    pub temporal_filter: TemporalFilterCtx,
    /// `FastUnion` result contour
    pub union: FastUnionCtx,
    /// Points of rope perimeter
    pub edges: FastEdgesCtx,
    /// Rope calculated dimensions
    pub rope_dimensions: RopeDimensionsCtx,
    /// Result of rope `WidthEmissions`
    pub width_emissions: WidthEmissionsCtx,
    /// Rope geometry defects
    pub defects: GeometryDefectCtx,

}
//
//
impl Default for FastScanCtx {
    fn default() -> Self {
        Self {
            result: ResultCtx::default(),
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

use crate::algorithm::{
    FineEdgesCtx, FineContoursCtx, FineUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, GeometryDefectCtx, WidthEmissionsCtx,
};

///
/// `FineScan` algorithm results
#[derive(Debug, Clone)]
pub struct FineScanCtx {
    /// Fine filtered and binarised image
    pub fine_contours: FineContoursCtx,
    /// `TemporalFilter` result
    pub temporal_filter: TemporalFilterCtx<FineScanCtx>,
    /// `FastUnion` result contour
    pub union: FineUnionCtx,
    /// Points of rope edges
    pub edges: FineEdgesCtx,
    /// Rope calculated dimensions
    pub rope_dimensions: RopeDimensionsCtx<FineScanCtx>,
    /// Result of rope `WidthEmissions`
    pub width_emissions: WidthEmissionsCtx<FineScanCtx>,
    /// Rope geometry defects
    pub defects: GeometryDefectCtx<FineScanCtx>,
}
//
//
impl Default for FineScanCtx {
    fn default() -> Self {
        Self {
            fine_contours: FineContoursCtx::default(),
            temporal_filter: TemporalFilterCtx::default(),
            union: FineUnionCtx::default(),
            edges: FineEdgesCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            width_emissions: WidthEmissionsCtx::default(),
            defects: GeometryDefectCtx::default(),
        }
    }
}

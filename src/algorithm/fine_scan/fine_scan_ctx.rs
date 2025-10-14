use crate::{
    algorithm::{FineEdgesCtx, FineContoursCtx, FineUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, ResultCtx},
    domain::Image, GeometryDefectCtx, WidthEmissionsCtx,
};

///
/// `FineScan` algorithm results
#[derive(Debug, Clone)]
pub struct FineScanCtx {
    /// Result of `FineScan` algorithm
    pub result: ResultCtx<Image>,
    /// Fine filtered and binarised image
    pub fine_contours: FineContoursCtx,
    /// `TemporalFilter` result
    pub temporal_filter: TemporalFilterCtx,
    /// `FastUnion` result contour
    pub union: FineUnionCtx,
    /// Points of rope edges
    pub edges: FineEdgesCtx,
    /// Rope calculated dimensions
    pub rope_dimensions: RopeDimensionsCtx,
    /// Result of rope `WidthEmissions`
    pub width_emission: WidthEmissionsCtx,
    /// Rope geometry defects
    pub defects: GeometryDefectCtx,
}
//
//
impl Default for FineScanCtx {
    fn default() -> Self {
        Self {
            result: ResultCtx::default(),
            fine_contours: FineContoursCtx::default(),
            temporal_filter: TemporalFilterCtx::default(),
            union: FineUnionCtx::default(),
            edges: FineEdgesCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            width_emission: WidthEmissionsCtx::default(),
            defects: GeometryDefectCtx::default(),
        }
    }
}

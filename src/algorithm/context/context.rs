use crate::{
    algorithm::{
        FastScanCtx, FineConvexCtx, FineScanCtx, InitialCtx, NormalizedCtx, ResultCtx, TestingCtx,
    },
    domain::Image,
};
///
/// Meta information used for identification
pub type MetaCtx = usize;
///
/// # Calculation context
/// - Provides read/write access to initial
/// - R/W access to the isoleted data of each step of computations
#[derive(Debug, Clone)]
pub struct Context {
    /// Some identification info
    pub(super) meta: MetaCtx,
    /// where store source frame
    pub(super) initial: InitialCtx,
    /// Result of last evaluated step
    pub(super) result: ResultCtx<Image>,
    /// Normalize algorithms results, cropp, auto gamma, brightness, contast, gray etc...
    pub(super) normalized: NormalizedCtx,
    /// `FastScan` algorithm results
    pub(super) fast_scan: FastScanCtx,
    /// `FineScan` algorithm results
    pub(super) fine_scan: FineScanCtx,
    /// Result of `FineScan` convex - solid contour
    pub(super) convex: FineConvexCtx,
    ///
    /// Used for testing only
    #[allow(dead_code)]
    pub testing: Option<TestingCtx>,
}
//
//
impl Context {
    ///
    /// New instance [Context]
    /// - 'initial' - [InitialCtx] instance, where store initial data
    pub fn new(initial: InitialCtx) -> Self {
        Self {
            meta: 0,
            initial,
            result: ResultCtx::default(),
            normalized: NormalizedCtx::default(),
            fast_scan: FastScanCtx::default(),
            fine_scan: FineScanCtx::default(),
            convex: FineConvexCtx::default(),
            testing: None,
        }
    }
}
    
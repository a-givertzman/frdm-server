use super::graham_scan::Point;
///
/// Context store of [GrahamScan](src/algorithm/graham_scan/graham_scan.rs)
/// - `result` - rope perimeter
pub struct GrahamScanCtx {
    pub result: Vec<Point>,
}
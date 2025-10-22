///
/// Enum of [geometry defect kinds](design/theory/geometry_rope_defects.md)
/// containing the position of defect withing a frame
#[derive(Debug, Clone, PartialEq)]
pub enum RopeDefectKind {
    /// Detecting both sides width growing
    Expansion,
    /// Detecting both sides width reduction
    Compressing,
    /// Detecting one side raising
    Hill,
    /// Detecting one side drooping
    Pit,
}
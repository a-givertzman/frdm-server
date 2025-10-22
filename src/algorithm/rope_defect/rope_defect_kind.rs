///
/// Enum of [geometry defect kinds](design/theory/geometry_rope_defects.md)
/// 
/// Contains defect start, end positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RopeDefectKind {
    /// Detecting both sides width growing
    Expansion(usize, usize),
    /// Detecting both sides width reduction
    Compressing(usize, usize),
    /// Detecting one side raising
    Hill(usize, usize),
    /// Detecting one side drooping
    Pit(usize, usize),
}
//
//
impl RopeDefectKind {
    ///
    /// Returns `true` if `self` and `other` has same kind, but can contains different values
    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (RopeDefectKind::Expansion(_, _), RopeDefectKind::Expansion(_, _)) => true,
            (RopeDefectKind::Compressing(_, _), RopeDefectKind::Compressing(_, _)) => true,
            (RopeDefectKind::Hill(_, _), RopeDefectKind::Hill(_, _)) => true,
            (RopeDefectKind::Pit(_, _), RopeDefectKind::Pit(_, _)) => true,
            _ => false,
        }
    }
    ///
    /// Returns position whwre defect is starts
    pub fn start(&self) -> usize {
        match self {
            RopeDefectKind::Expansion(start, _) => *start,
            RopeDefectKind::Compressing(start, _) => *start,
            RopeDefectKind::Hill(start, _) => *start,
            RopeDefectKind::Pit(start, _) => *start,
        }
    }
    ///
    /// Returns position whwre defect is ends
    pub fn end(&self) -> usize {
        match self {
            RopeDefectKind::Expansion(_, end) => *end,
            RopeDefectKind::Compressing(_, end) => *end,
            RopeDefectKind::Hill(_, end) => *end,
            RopeDefectKind::Pit(_, end) => *end,
        }
    }
    ///
    /// Update position whwre defect is starts
    pub fn start_with(&self, start: usize) -> RopeDefectKind {
        match self {
            RopeDefectKind::Expansion(_, end) => RopeDefectKind::Expansion(start, *end),
            RopeDefectKind::Compressing(_, end) => RopeDefectKind::Compressing(start, *end),
            RopeDefectKind::Hill(_, end) => RopeDefectKind::Hill(start, *end),
            RopeDefectKind::Pit(_, end) => RopeDefectKind::Pit(start, *end),
        }
    }
    ///
    /// Update position whwre defect is ends
    pub fn end_with(&self, end: usize) -> RopeDefectKind {
        match self {
            RopeDefectKind::Expansion(start, _) => RopeDefectKind::Expansion(*start, end),
            RopeDefectKind::Compressing(start, _) => RopeDefectKind::Compressing(*start, end),
            RopeDefectKind::Hill(start, _) => RopeDefectKind::Hill(*start, end),
            RopeDefectKind::Pit(start, _) => RopeDefectKind::Pit(*start, end),
        }
    }
}
/// The 18 standard moves of a 3x3 Rubik's cube (HTM - Half Turn Metric).
/// Each face can rotate 90° clockwise (prime = false), 90° counter-clockwise (prime = true),
/// or 180° (half = true).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    U, // Up
    D, // Down
    F, // Front
    B, // Back
    L, // Left
    R, // Right
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub face: Face,
    pub turns: u8, // 1 = CW 90°, 2 = 180°, 3 = CCW 90°
}

impl Move {
    pub const fn new(face: Face, turns: u8) -> Self {
        Self { face, turns }
    }

    /// Returns the inverse of this move.
    #[allow(dead_code)]
    pub fn inverse(self) -> Self {
        Self {
            face: self.face,
            turns: (4 - self.turns) % 4,
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = match self.face {
            Face::U => "U",
            Face::D => "D",
            Face::F => "F",
            Face::B => "B",
            Face::L => "L",
            Face::R => "R",
        };
        let suffix = match self.turns {
            1 => "",
            2 => "2",
            3 => "'",
            _ => "?",
        };
        write!(f, "{}{}", face, suffix)
    }
}

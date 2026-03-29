use super::Cube;

/// High-level state derived from the cube for UI display.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CubeState {
    Solved,
    Unsolved,
}

#[allow(dead_code)]
impl CubeState {
    pub fn from_cube(cube: &Cube) -> Self {
        if cube.is_solved() {
            CubeState::Solved
        } else {
            CubeState::Unsolved
        }
    }

    pub fn is_solved(&self) -> bool {
        *self == CubeState::Solved
    }
}

/// 54-facelet color representation for rendering.
/// Faces in order: U(0..9), R(9..18), F(18..27), D(27..36), L(36..45), B(45..54)
/// Within each face, facelets are row-major top-left to bottom-right.
#[derive(Debug, Clone)]
pub struct Facelets {
    pub data: [u8; 54], // 0=U(white) 1=R(red) 2=F(green) 3=D(yellow) 4=L(orange) 5=B(blue)
}

impl Facelets {
    /// Convert cubie representation to facelets for rendering.
    /// Convert from cube state representation
    pub fn from_cube(cube: &Cube) -> Self {
        Self { data: cube.facelets }
    }
}

use super::moves::{Face, Move};

/// Standar Kociemba 54-facelet representation.
/// Faces: U=0, R=1, F=2, D=3, L=4, B=5.
/// Each face has 9 facelets row-major (0=top-left, 8=bottom-right).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    pub facelets: [u8; 54],
}

impl Cube {
    /// Returns a solved cube.
    pub fn solved() -> Self {
        let mut facelets = [0; 54];
        for i in 0..6 {
            for j in 0..9 {
                facelets[i * 9 + j] = i as u8;
            }
        }
        Self { facelets }
    }

    pub fn is_solved(&self) -> bool {
        self.facelets == Self::solved().facelets
    }

    pub fn apply(&mut self, m: Move) {
        for _ in 0..m.turns {
            self.apply_once(m.face);
        }
    }

    fn apply_once(&mut self, face: Face) {
        let face_off = match face {
            Face::U => 0 * 9,
            Face::R => 1 * 9,
            Face::F => 2 * 9,
            Face::D => 3 * 9,
            Face::L => 4 * 9,
            Face::B => 5 * 9,
        };
        let mut f = self.facelets; // mutate copy
        
        // Face internal cycle
        Self::cycle4(&mut f, face_off + 0, face_off + 2, face_off + 8, face_off + 6);
        Self::cycle4(&mut f, face_off + 1, face_off + 5, face_off + 7, face_off + 3);

        // Adjacent panels
        match face {
            Face::U => {
                Self::cycle4(&mut f, 45+0, 9+0, 18+0, 36+0);
                Self::cycle4(&mut f, 45+1, 9+1, 18+1, 36+1);
                Self::cycle4(&mut f, 45+2, 9+2, 18+2, 36+2);
            }
            Face::D => {
                Self::cycle4(&mut f, 18+6, 9+6, 45+6, 36+6);
                Self::cycle4(&mut f, 18+7, 9+7, 45+7, 36+7);
                Self::cycle4(&mut f, 18+8, 9+8, 45+8, 36+8);
            }
            Face::F => {
                Self::cycle4(&mut f, 6, 9+0, 27+2, 36+8);
                Self::cycle4(&mut f, 7, 9+3, 27+1, 36+5);
                Self::cycle4(&mut f, 8, 9+6, 27+0, 36+2);
            }
            Face::B => {
                Self::cycle4(&mut f, 0, 36+0, 27+8, 9+8);
                Self::cycle4(&mut f, 1, 36+3, 27+7, 9+5);
                Self::cycle4(&mut f, 2, 36+6, 27+6, 9+2);
            }
            Face::L => {
                Self::cycle4(&mut f, 0, 18+0, 27+0, 45+8);
                Self::cycle4(&mut f, 3, 18+3, 27+3, 45+5);
                Self::cycle4(&mut f, 6, 18+6, 27+6, 45+2);
            }
            Face::R => {
                Self::cycle4(&mut f, 2, 45+6, 27+2, 18+2);
                Self::cycle4(&mut f, 5, 45+3, 27+5, 18+5);
                Self::cycle4(&mut f, 8, 45+0, 27+8, 18+8);
            }
        }
        self.facelets = f;
    }

    fn cycle4(f: &mut [u8; 54], a: usize, b: usize, c: usize, d: usize) {
        let tmp = f[d];
        f[d] = f[c];
        f[c] = f[b];
        f[b] = f[a];
        f[a] = tmp;
    }
}

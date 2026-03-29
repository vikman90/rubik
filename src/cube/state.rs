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
    pub fn from_cube(cube: &Cube) -> Self {
        let mut data = [0u8; 54];

        // Centers are fixed
        data[4] = 0;  // U center
        data[13] = 1; // R center
        data[22] = 2; // F center
        data[31] = 3; // D center
        data[40] = 4; // L center
        data[49] = 5; // B center

        // Corner facelets:
        // Each corner has 3 facelets. Table: [corner_id] -> [(face_idx, facelet_within_face) x3]
        // in order matching orientation 0,1,2 of that corner type.
        //
        // Corners: 0=URF 1=UFL 2=ULB 3=UBR 4=DFR 5=DLF 6=DBL 7=DRB
        // Faces: U=0,R=1,F=2,D=3,L=4,B=5
        // Facelet index within face (0..9), row-major:
        //   0 1 2
        //   3 4 5
        //   6 7 8
        const CORNER_FACELETS: [[(usize, usize); 3]; 8] = [
            [(0, 8), (1, 0), (2, 2)], // URF: U8, R0, F2
            [(0, 6), (2, 0), (4, 2)], // UFL: U6, F0, L2
            [(0, 0), (4, 0), (5, 2)], // ULB: U0, L0, B2
            [(0, 2), (5, 0), (1, 2)], // UBR: U2, B0, R2
            [(3, 2), (1, 6), (2, 8)], // DFR: D2, R6, F8  -- wait, need to recheck
            [(3, 0), (2, 6), (4, 8)], // DLF: D0, F6, L8
            [(3, 6), (4, 6), (5, 8)], // DBL: D6, L6, B8
            [(3, 8), (5, 6), (1, 8)], // DRB: D8, B6, R8
        ];

        const CORNER_COLORS: [[u8; 3]; 8] = [
            [0, 1, 2], // URF: U,R,F
            [0, 2, 4], // UFL: U,F,L
            [0, 4, 5], // ULB: U,L,B
            [0, 5, 1], // UBR: U,B,R
            [3, 1, 2], // DFR: D,R,F  -- wait orientation: D,F,R or D,R,F?
            [3, 2, 4], // DLF: D,F,L
            [3, 4, 5], // DBL: D,L,B
            [3, 5, 1], // DRB: D,B,R
        ];

        for slot in 0..8usize {
            let piece = cube.corner_pos[slot] as usize;
            let ori = cube.corner_ori[slot] as usize;
            let facelets = &CORNER_FACELETS[slot];
            let colors = &CORNER_COLORS[piece];
            for k in 0..3 {
                let color = colors[(k + ori) % 3];
                let (face, within) = facelets[k];
                data[face * 9 + within] = color;
            }
        }

        // Edge facelets:
        // Edges: 0=UR 1=UF 2=UL 3=UB 4=DR 5=DF 6=DL 7=DB 8=FR 9=FL 10=BL 11=BR
        const EDGE_FACELETS: [[(usize, usize); 2]; 12] = [
            [(0, 5), (1, 1)], // UR: U5, R1
            [(0, 7), (2, 1)], // UF: U7, F1
            [(0, 3), (4, 1)], // UL: U3, L1
            [(0, 1), (5, 1)], // UB: U1, B1
            [(3, 5), (1, 7)], // DR: D5, R7
            [(3, 7), (2, 7)], // DF: D7, F7
            [(3, 3), (4, 7)], // DL: D3, L7
            [(3, 1), (5, 7)], // DB: D1, B7
            [(2, 5), (1, 3)], // FR: F5, R3
            [(2, 3), (4, 5)], // FL: F3, L5
            [(5, 5), (4, 3)], // BL: B5, L3
            [(5, 3), (1, 5)], // BR: B3, R5
        ];

        const EDGE_COLORS: [[u8; 2]; 12] = [
            [0, 1], // UR: U,R
            [0, 2], // UF: U,F
            [0, 4], // UL: U,L
            [0, 5], // UB: U,B
            [3, 1], // DR: D,R
            [3, 2], // DF: D,F
            [3, 4], // DL: D,L
            [3, 5], // DB: D,B
            [2, 1], // FR: F,R
            [2, 4], // FL: F,L
            [5, 4], // BL: B,L
            [5, 1], // BR: B,R
        ];

        for slot in 0..12usize {
            let piece = cube.edge_pos[slot] as usize;
            let ori = cube.edge_ori[slot] as usize;
            let facelets = &EDGE_FACELETS[slot];
            let colors = &EDGE_COLORS[piece];
            for k in 0..2 {
                let color = colors[(k + ori) % 2];
                let (face, within) = facelets[k];
                data[face * 9 + within] = color;
            }
        }

        Self { data }
    }
}

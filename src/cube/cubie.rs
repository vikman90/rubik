use super::moves::{Face, Move};

/// Corner pieces: 8 corners, each with position (0..8) and orientation (0..3).
/// Orientation 0 = solved, 1 = CW twist, 2 = CCW twist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    /// corner_pos[i] = which corner is at slot i (0..8)
    pub corner_pos: [u8; 8],
    /// corner_ori[i] = orientation of corner at slot i (0..3)
    pub corner_ori: [u8; 8],
    /// edge_pos[i] = which edge is at slot i (0..12)
    pub edge_pos: [u8; 12],
    /// edge_ori[i] = orientation of edge at slot i (0..2)
    pub edge_ori: [u8; 12],
}

/// Corner slot indices (cubie positions around the cube):
/// 0=URF 1=UFL 2=ULB 3=UBR 4=DFR 5=DLF 6=DBL 7=DRB
///
/// Edge slot indices:
/// 0=UR 1=UF 2=UL 3=UB 4=DR 5=DF 6=DL 7=DB 8=FR 9=FL 10=BL 11=BR

impl Cube {
    /// Returns a solved cube.
    pub fn solved() -> Self {
        Self {
            corner_pos: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_ori: [0; 8],
            edge_pos: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            edge_ori: [0; 12],
        }
    }

    pub fn is_solved(&self) -> bool {
        self.corner_pos == [0, 1, 2, 3, 4, 5, 6, 7]
            && self.corner_ori == [0; 8]
            && self.edge_pos == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
            && self.edge_ori == [0; 12]
    }

    pub fn apply(&mut self, m: Move) {
        for _ in 0..m.turns {
            self.apply_once(m.face);
        }
    }

    pub fn apply_sequence(&mut self, moves: &[Move]) {
        for &m in moves {
            self.apply(m);
        }
    }

    fn apply_once(&mut self, face: Face) {
        match face {
            Face::U => self.move_u(),
            Face::D => self.move_d(),
            Face::F => self.move_f(),
            Face::B => self.move_b(),
            Face::L => self.move_l(),
            Face::R => self.move_r(),
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn cycle4_corners(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let (p, o) = (&mut self.corner_pos, &mut self.corner_ori);
        let tmp_p = p[d];
        let tmp_o = o[d];
        p[d] = p[c]; o[d] = o[c];
        p[c] = p[b]; o[c] = o[b];
        p[b] = p[a]; o[b] = o[a];
        p[a] = tmp_p; o[a] = tmp_o;
    }

    fn cycle4_edges(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let (p, o) = (&mut self.edge_pos, &mut self.edge_ori);
        let tmp_p = p[d];
        let tmp_o = o[d];
        p[d] = p[c]; o[d] = o[c];
        p[c] = p[b]; o[c] = o[b];
        p[b] = p[a]; o[b] = o[a];
        p[a] = tmp_p; o[a] = tmp_o;
    }

    fn twist_corners(&mut self, slots: &[usize], deltas: &[u8]) {
        for (&slot, &delta) in slots.iter().zip(deltas.iter()) {
            self.corner_ori[slot] = (self.corner_ori[slot] + delta) % 3;
        }
    }

    fn flip_edges(&mut self, slots: &[usize]) {
        for &slot in slots {
            self.edge_ori[slot] ^= 1;
        }
    }

    // ── Face moves ────────────────────────────────────────────────────────────
    // Corner slots:  0=URF 1=UFL 2=ULB 3=UBR 4=DFR 5=DLF 6=DBL 7=DRB
    // Edge slots:    0=UR  1=UF  2=UL  3=UB  4=DR  5=DF  6=DL  7=DB  8=FR 9=FL 10=BL 11=BR

    fn move_u(&mut self) {
        self.cycle4_corners(0, 3, 2, 1); // URF UBR ULB UFL
        self.cycle4_edges(0, 3, 2, 1);   // UR  UB  UL  UF
        // U moves don't change orientations
    }

    fn move_d(&mut self) {
        self.cycle4_corners(4, 5, 6, 7); // DFR DLF DBL DRB
        self.cycle4_edges(4, 6, 7, 5);   // DR  DL  DB  DF  (note order: CW when looking from below)
        // D moves don't change orientations
    }

    fn move_f(&mut self) {
        self.cycle4_corners(0, 1, 5, 4); // URF UFL DLF DFR
        self.cycle4_edges(1, 9, 5, 8);   // UF  FL  DF  FR
        self.twist_corners(&[0, 1, 5, 4], &[2, 1, 2, 1]);
        self.flip_edges(&[1, 9, 5, 8]);
    }

    fn move_b(&mut self) {
        self.cycle4_corners(2, 3, 7, 6); // ULB UBR DRB DBL
        self.cycle4_edges(3, 11, 7, 10); // UB  BR  DB  BL
        self.twist_corners(&[2, 3, 7, 6], &[1, 2, 1, 2]);
        self.flip_edges(&[3, 11, 7, 10]);
    }

    fn move_l(&mut self) {
        self.cycle4_corners(1, 2, 6, 5); // UFL ULB DBL DLF
        self.cycle4_edges(2, 10, 6, 9);  // UL  BL  DL  FL
        self.twist_corners(&[1, 2, 6, 5], &[2, 1, 2, 1]);
        // L moves don't flip edges
    }

    fn move_r(&mut self) {
        self.cycle4_corners(3, 0, 4, 7); // UBR URF DFR DRB
        self.cycle4_edges(0, 8, 4, 11);  // UR  FR  DR  BR
        self.twist_corners(&[3, 0, 4, 7], &[1, 2, 1, 2]);
        // R moves don't flip edges
    }
}

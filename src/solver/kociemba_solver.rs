use crate::cube::{Cube, Move};
use crate::cube::moves::Face;
use kewb::{FaceCube, CubieCube, Solver, DataTable};
use std::convert::TryFrom;
use std::sync::OnceLock;

static DATA_TABLE: OnceLock<DataTable> = OnceLock::new();

pub fn solve(cube: &Cube) -> Option<Vec<Move>> {
    if cube.is_solved() {
        return Some(vec![]);
    }

    let facelets_str = to_kociemba_string(cube);

    // Parse facelet string into kewb's FaceCube
    let face_cube = match FaceCube::try_from(facelets_str.as_str()) {
        Ok(fc) => fc,
        Err(_) => return None,
    };

    // Convert FaceCube to CubieCube
    let state = match CubieCube::try_from(&face_cube) {
        Ok(s) => s,
        Err(_) => return None,
    };

    let table = DATA_TABLE.get_or_init(DataTable::default);

    // In test builds, limit depth to 20 (plenty for short scrambles) to reduce
    // recursive call-stack depth. In release builds, use 28 for near-optimal solutions.
    #[cfg(test)]
    let max_length: u8 = 20;
    #[cfg(not(test))]
    let max_length: u8 = 28;

    // Use a generous 30-second timeout during tests so the solver returns the
    // best solution found without relying on None-timeout infinite recursion,
    // which can overflow the stack on Windows (1 MB default vs 8 MB on Unix).
    // In production use a tighter 5-second limit.
    #[cfg(test)]
    let timeout_secs: f32 = 30.0;
    #[cfg(not(test))]
    let timeout_secs: f32 = 5.0;

    // Spawn solver on a thread with an explicit 8 MB stack to avoid stack
    // overflows on Windows (which defaults to a 1 MB thread stack).
    let result = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let mut solver = Solver::new(table, max_length, Some(timeout_secs));
            solver.solve(state)
        })
        .ok()?
        .join()
        .ok()??;

    Some(parse_moves(&result.to_string()))
}

fn to_kociemba_string(cube: &Cube) -> String {
    // Kociemba facelet order: U=0, R=1, F=2, D=3, L=4, B=5
    let chars = ['U', 'R', 'F', 'D', 'L', 'B'];
    let mut s = String::with_capacity(54);
    for &f in &cube.facelets {
        s.push(chars[f as usize]);
    }
    s
}

fn parse_moves(s: &str) -> Vec<Move> {
    s.split_whitespace().filter_map(|m| {
        let mut chars = m.chars();
        let face = match chars.next()? {
            'U' => Face::U,
            'D' => Face::D,
            'F' => Face::F,
            'B' => Face::B,
            'L' => Face::L,
            'R' => Face::R,
            _ => return None,
        };
        let turns = match chars.next() {
            Some('2') => 2,
            Some('\'') => 3,
            _ => 1,
        };
        Some(Move::new(face, turns))
    }).collect()
}

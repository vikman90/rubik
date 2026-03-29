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

    // Convert FaceCube to State (CubieCube equivalent)
    let state = match CubieCube::try_from(&face_cube) {
        Ok(s) => s,
        Err(_) => return None,
    };

    let table = DATA_TABLE.get_or_init(|| DataTable::default());
    // Max 32 moves ensures phase 1 + phase 2 completes in < 1ms usually.
    let mut solver = Solver::new(table, 32, Some(5.0));

    // Solve the state
    let solution = solver.solve(state)?;

    Some(parse_moves(&solution.to_string()))
}

fn to_kociemba_string(cube: &Cube) -> String {
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Cube;
    use crate::cube::scrambler::scramble;

    #[test]
    fn test_solve_random_combinations() {
        for seed in 0..1000 {
            let mut cube = Cube::solved();
            let mut history = vec![];
            let moves = scramble(20);
            for m in moves {
                cube.apply(m);
                history.push(m);
                
                let s = solve(&cube);
                if s.is_none() {
                    panic!("FAILED ON SEED: {}\nHISTORY: {:?}", seed, history);
                }
            }
        }
    }
}

use crate::cube::{Cube, Move};
use crate::cube::moves::Face;
use kewb::{FaceCube, CubieCube, Solver, DataTable};
use std::convert::TryFrom;
use std::sync::OnceLock;
use std::path::PathBuf;

static DATA_TABLE: OnceLock<DataTable> = OnceLock::new();

/// Get the path to the cached DataTable file.
/// Uses a platform-appropriate cache directory.
fn get_cache_path() -> PathBuf {
    let cache_dir = std::env::var("XDG_CACHE_HOME")
        .ok()
        .and_then(|p| if p.is_empty() { None } else { Some(PathBuf::from(p)) })
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(".cache"))
        })
        .unwrap_or_else(|| PathBuf::from("."));

    cache_dir.join("rubik_solver_table.bin")
}

/// Load or generate the DataTable.
/// First tries to load from cache, falls back to generating (which is slow).
fn load_or_generate_table() -> DataTable {
    let cache_path = get_cache_path();

    // Try to load from cache
    if let Ok(table) = kewb::fs::read_table(&cache_path) {
        return table;
    }

    // Generate new table (slow - only happens once)
    let table = DataTable::default();

    // Try to save for next time (ignore errors)
    let _ = kewb::fs::write_table(&cache_path);

    table
}

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

    let table = DATA_TABLE.get_or_init(load_or_generate_table);

    // In test builds, limit depth to 20 (plenty for short scrambles) to reduce
    // recursive call-stack depth. In release builds, use 28 for near-optimal solutions.
    #[cfg(test)]
    let max_length: u8 = 20;
    #[cfg(not(test))]
    let max_length: u8 = 28;

    // Use None as timeout to make the solver return immediately upon finding
    // the first solution within max_length. The kewb solver with a timeout
    // continues searching for better solutions until the timeout expires.
    // Without a timeout, it returns the first valid solution found, which is
    // much faster.
    let timeout: Option<f32> = None;

    // Spawn solver on a thread with an explicit 8 MB stack to avoid stack
    // overflows on Windows (which defaults to a 1 MB thread stack).
    let result = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            let mut solver = Solver::new(table, max_length, timeout);
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

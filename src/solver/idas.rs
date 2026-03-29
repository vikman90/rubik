/// IDA* solver for the Rubik's cube.
///
/// Uses a simple admissible heuristic: number of misplaced corners / 4
/// (since each move can fix at most 4 corners). This is fast enough for
/// scrambles up to ~8 moves and gives optimal solutions for those depths.
/// For deeper scrambles, use with a depth cap and fall back to Kociemba externally.
use crate::cube::{Cube, Move};
use crate::cube::moves::Face;

const MAX_DEPTH: u8 = 20;

pub fn solve(cube: &Cube) -> Option<Vec<Move>> {
    if cube.is_solved() {
        return Some(vec![]);
    }

    let mut path = Vec::new();
    let mut threshold = heuristic(cube);

    loop {
        match search(cube, 0, threshold, None, &mut path) {
            SearchResult::Found => return Some(path),
            SearchResult::NotFound(next_threshold) => {
                if next_threshold > MAX_DEPTH {
                    return None;
                }
                threshold = next_threshold;
                path.clear();
            }
        }
    }
}

enum SearchResult {
    Found,
    NotFound(u8),
}

fn search(
    cube: &Cube,
    g: u8,
    threshold: u8,
    last_face: Option<Face>,
    path: &mut Vec<Move>,
) -> SearchResult {
    let h = heuristic(cube);
    let f = g + h;

    if f > threshold {
        return SearchResult::NotFound(f);
    }

    if cube.is_solved() {
        return SearchResult::Found;
    }

    if g >= MAX_DEPTH {
        return SearchResult::NotFound(u8::MAX);
    }

    let mut min_t = u8::MAX;

    for m in Move::all() {
        // Prune: don't repeat the same face (e.g. U then U again — already covered by turns=2)
        // and avoid redundant opposite face sequences
        if Some(m.face) == last_face {
            continue;
        }
        if is_redundant(last_face, m.face) {
            continue;
        }

        let mut next = cube.clone();
        next.apply(m);
        path.push(m);

        match search(&next, g + 1, threshold, Some(m.face), path) {
            SearchResult::Found => return SearchResult::Found,
            SearchResult::NotFound(t) => {
                if t < min_t {
                    min_t = t;
                }
            }
        }

        path.pop();
    }

    SearchResult::NotFound(min_t)
}

/// Admissible heuristic: misplaced corners / 4.
fn heuristic(cube: &Cube) -> u8 {
    let misplaced_corners = cube
        .corner_pos
        .iter()
        .enumerate()
        .filter(|(i, &p)| p != *i as u8 || cube.corner_ori[*i] != 0)
        .count() as u8;

    let misplaced_edges = cube
        .edge_pos
        .iter()
        .enumerate()
        .filter(|(i, &p)| p != *i as u8 || cube.edge_ori[*i] != 0)
        .count() as u8;

    // Each move can affect at most 4 corners and 4 edges
    (misplaced_corners / 4).max(misplaced_edges / 4)
}

/// Avoid redundant opposite-face sequences (e.g. R then L is equivalent to L then R).
fn is_redundant(last: Option<Face>, current: Face) -> bool {
    match (last, current) {
        (Some(Face::D), Face::U) => true,
        (Some(Face::B), Face::F) => true,
        (Some(Face::L), Face::R) => true,
        _ => false,
    }
}

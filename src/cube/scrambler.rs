use rand::Rng;
use super::moves::{Face, Move};

/// Generate a random scramble sequence of `length` moves,
/// avoiding consecutive moves on the same face.
pub fn scramble(length: usize) -> Vec<Move> {
    let faces = [Face::U, Face::D, Face::F, Face::B, Face::L, Face::R];
    let mut rng = rand::thread_rng();
    let mut moves = Vec::with_capacity(length);
    let mut last_face: Option<Face> = None;

    while moves.len() < length {
        let face = faces[rng.gen_range(0..6)];
        if Some(face) == last_face {
            continue;
        }
        let turns = rng.gen_range(1u8..=3);
        moves.push(Move::new(face, turns));
        last_face = Some(face);
    }

    moves
}

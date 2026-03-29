use rubik::cube::{Cube, Move, moves::Face};
use rubik::cube::scrambler::scramble;
use rubik::solver::solve;

#[test]
fn test_four_turns_identity() {
    let faces = [Face::U, Face::D, Face::F, Face::B, Face::L, Face::R];
    for &face in &faces {
        // Option A: Apply 4 sequentially
        let mut cube = Cube::solved();
        for _ in 0..4 {
            cube.apply(Move::new(face, 1));
        }
        assert!(cube.is_solved(), "4 sequential CW turns of {:?} should return to identity", face);

        // Option B: Apply 1 move of magnitude 4
        let mut cube2 = Cube::solved();
        cube2.apply(Move::new(face, 4));
        assert!(cube2.is_solved(), "Move({:?}, 4) should return to identity", face);
    }
}

#[test]
fn test_reverse_moves() {
    let mut cube = Cube::solved();
    let moves = scramble(20);
    
    // Scramble the cube
    for &m in &moves {
        cube.apply(m);
    }
    
    // Incredibly low probability it scrambled back to solved, but possible.
    // Generally, it shouldn't be solved after 20 moves.
    assert!(!cube.is_solved(), "Cube should be scrambled after 20 random moves");
    
    // Apply moves in reverse order and direction
    for &m in moves.iter().rev() {
        // Inverting turns: 1 CW -> 3 CW (equivalent to 1 CCW)
        // 2 CW -> 2 CW
        // 3 CW -> 1 CW
        let rev_turns = (4 - (m.turns % 4)) % 4;
        let rev_move = Move::new(m.face, rev_turns);
        cube.apply(rev_move);
    }
    
    assert!(cube.is_solved(), "Applying the inverse moves should always restore the cube");
}

#[test]
fn test_solver_integration() {
    // We will test 3 random seeds of 5 moves to ensure solver correctness and physical validity.
    // Deep scrambles take too long in unoptimized debug tests, causing false-positive timeouts.
    for _ in 0..3 {
        let mut cube = Cube::solved();
        
        // 5 random moves
        let moves = scramble(5);
        for &m in &moves {
            cube.apply(m);
        }
        
        // Ensure kocieamba's async-friendly solver engine retrieves a clean solution
        let solution = solve(&cube).expect("Kociemba solver failed to find a valid solution");
        
        // Apply the solution mathematically to the scrambled cube
        for m in solution {
            cube.apply(m);
        }
        
        // Validate if perfectly solved
        assert!(cube.is_solved(), "The solution provided by Kociemba didn't correctly solve the cube physically");
    }
}

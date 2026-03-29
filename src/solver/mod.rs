// idas is deprecated
pub mod stats;
pub mod kociemba_solver;

pub use kociemba_solver::solve;
pub use stats::{SolveStats, SolveRecord};

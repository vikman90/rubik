use bevy::prelude::*;
use crate::cube::Move;
use crate::cube::moves::Face;

/// Current state of the rotation animation.
#[derive(Resource, Default)]
pub struct AnimationState {
    pub active: bool,
    pub elapsed: f32,
    pub duration: f32,
    pub pending_moves: Vec<Move>,
    /// Move currently being animated
    pub current_move: Option<Move>,
}

impl AnimationState {
    pub fn queue(&mut self, moves: Vec<Move>, duration_per_move: f32) {
        self.pending_moves = moves;
        self.duration = duration_per_move;
        self.active = true;
        self.elapsed = 0.0;
        self.current_move = None;
    }

    pub fn is_busy(&self) -> bool {
        self.active || !self.pending_moves.is_empty()
    }
}

/// Component tag for cubelets currently participating in a face rotation.
#[allow(dead_code)]
#[derive(Component)]
pub struct RotationAnimation {
    pub axis: Vec3,
    pub total_angle: f32,
    pub elapsed: f32,
    pub duration: f32,
}

/// Returns the rotation axis and sign for a face move.
#[allow(dead_code)]
pub fn face_axis(face: Face) -> Vec3 {
    match face {
        Face::U => Vec3::Y,
        Face::D => Vec3::NEG_Y,
        Face::F => Vec3::NEG_Z,
        Face::B => Vec3::Z,
        Face::R => Vec3::NEG_X,
        Face::L => Vec3::X,
    }
}

/// Returns the grid coordinate that is fixed for cubelets on this face.
#[allow(dead_code)]
pub fn face_fixed_coord(face: Face) -> (Axis, i8) {
    match face {
        Face::U => (Axis::Y, 1),
        Face::D => (Axis::Y, -1),
        Face::F => (Axis::Z, 1),
        Face::B => (Axis::Z, -1),
        Face::R => (Axis::X, 1),
        Face::L => (Axis::X, -1),
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Axis { X, Y, Z }

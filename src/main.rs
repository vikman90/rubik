mod cube;
mod solver;
mod render;
mod ui;

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

use cube::{Cube, Move};
use cube::moves::Face;
use cube::scrambler::scramble;
use cube::state::Facelets;
use render::{CubeletMarker, spawn_cube};
use render::animation::AnimationState;
use solver::{solve, SolveStats, SolveTimer};
use ui::hud::{spawn_hud, StatusText, StatsText};

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct CubeResource(Cube);

#[derive(Resource)]
struct Stats(SolveStats);

#[derive(Resource)]
struct OrbitCamera {
    yaw: f32,
    pitch: f32,
    radius: f32,
    dragging: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: 0.4,
            radius: 8.0,
            dragging: false,
        }
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rubik's Cube Solver".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(CubeResource(Cube::solved()))
        .insert_resource(Stats(SolveStats::new()))
        .insert_resource(AnimationState::default())
        .insert_resource(OrbitCamera::default())
        .insert_resource(ClearColor(Color::srgb(0.12, 0.12, 0.18)))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            keyboard_input,
            orbit_camera,
            update_hud,
            tick_animation,
        ))
        .run();
}

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_res: Res<CubeResource>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.5, 0.0)),
    ));
    commands.spawn((
        PointLight {
            intensity: 4000.0,
            ..default()
        },
        Transform::from_xyz(-4.0, 6.0, 4.0),
    ));

    let facelets = Facelets::from_cube(&cube_res.0);
    spawn_cube(&mut commands, &mut meshes, &mut materials, &facelets);

    spawn_hud(&mut commands);
}

// ── Keyboard input ────────────────────────────────────────────────────────────

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut stats: ResMut<Stats>,
    mut anim: ResMut<AnimationState>,
    cube_res: Res<CubeResource>,
) {
    if anim.is_busy() {
        return;
    }

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let turns = if shift { 3u8 } else { 1u8 };

    let maybe_face: Option<Face> = if keys.just_pressed(KeyCode::KeyU) { Some(Face::U) }
        else if keys.just_pressed(KeyCode::KeyD) { Some(Face::D) }
        else if keys.just_pressed(KeyCode::KeyF) { Some(Face::F) }
        else if keys.just_pressed(KeyCode::KeyB) { Some(Face::B) }
        else if keys.just_pressed(KeyCode::KeyL) { Some(Face::L) }
        else if keys.just_pressed(KeyCode::KeyR) { Some(Face::R) }
        else { None };

    if let Some(face) = maybe_face {
        let m = Move::new(face, turns);
        anim.queue(vec![m], 0.3);
        return;
    }

    if keys.just_pressed(KeyCode::KeyS) {
        let moves = scramble(20);
        anim.queue(moves, 0.12);
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        if cube_res.0.is_solved() {
            return;
        }
        let timer = SolveTimer::start();
        if let Some(solution) = solve(&cube_res.0) {
            let record = timer.finish(solution.len());
            stats.0.record(record.steps, record.duration);
            anim.queue(solution, 0.25);
        }
    }
}

fn rebuild_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cubelet_query: &Query<(Entity, &CubeletMarker, &mut Transform, Option<&mut render::animation::RotationAnimation>)>,
    cube: &Cube,
) {
    for (entity, _, _, _) in cubelet_query.iter() {
        commands.entity(entity).despawn();
    }
    let facelets = Facelets::from_cube(cube);
    spawn_cube(commands, meshes, materials, &facelets);
}

// ── Camera orbit ──────────────────────────────────────────────────────────────

fn orbit_camera(
    mut orbit: ResMut<OrbitCamera>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        orbit.dragging = true;
    }
    if mouse_buttons.just_released(MouseButton::Left) {
        orbit.dragging = false;
    }

    if orbit.dragging {
        for ev in mouse_motion.read() {
            orbit.yaw -= ev.delta.x * 0.005;
            orbit.pitch = (orbit.pitch - ev.delta.y * 0.005)
                .clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
        }
    } else {
        mouse_motion.clear();
    }

    for ev in scroll.read() {
        orbit.radius = (orbit.radius - ev.y * 0.5).clamp(3.0, 20.0);
    }

    let x = orbit.radius * orbit.pitch.cos() * orbit.yaw.sin();
    let y = orbit.radius * orbit.pitch.sin();
    let z = orbit.radius * orbit.pitch.cos() * orbit.yaw.cos();

    for mut transform in camera_query.iter_mut() {
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

// ── HUD ───────────────────────────────────────────────────────────────────────

fn update_hud(
    cube_res: Res<CubeResource>,
    stats: Res<Stats>,
    mut status_query: Query<(&mut Text, &mut TextColor), (With<StatusText>, Without<StatsText>)>,
    mut stats_query: Query<&mut Text, (With<StatsText>, Without<StatusText>)>,
) {
    let solved = cube_res.0.is_solved();

    for (mut text, mut color) in status_query.iter_mut() {
        if solved {
            text.0 = "● SOLVED".to_string();
            color.0 = Color::srgb(0.2, 1.0, 0.4);
        } else {
            text.0 = "● UNSOLVED".to_string();
            color.0 = Color::srgb(1.0, 0.3, 0.3);
        }
    }

    let s = &stats.0;
    for mut text in stats_query.iter_mut() {
        if s.count() == 0 {
            text.0 = "No solves yet".to_string();
        } else {
            let last = s.last().unwrap();
            text.0 = format!(
                "Solves: {}   Last: {} steps / {:.1} ms   Avg: {:.1} steps / {:.1} ms   Speed: {:.2} steps/ms",
                s.count(),
                last.steps,
                last.duration.as_secs_f64() * 1000.0,
                s.avg_steps(),
                s.avg_duration_ms(),
                s.avg_steps_per_ms(),
            );
        }
    }
}

// ── Animation ─────────────────────────────────────────────────────────────────

fn tick_animation(
    mut anim: ResMut<AnimationState>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cubelet_query: Query<(Entity, &CubeletMarker, &mut Transform, Option<&mut render::animation::RotationAnimation>)>,
    mut cube_res: ResMut<CubeResource>,
) {
    if !anim.active { return; }

    if anim.current_move.is_none() {
        if anim.pending_moves.is_empty() {
            anim.active = false;
            return;
        }
        let m = anim.pending_moves.remove(0);
        anim.current_move = Some(m);
        anim.elapsed = 0.0;
        
        let (axis, fixed_val) = render::animation::face_fixed_coord(m.face);
        let rotation_axis = render::animation::face_axis(m.face);
        let total_angle = render::animation::turn_angle(m.turns);

        for (entity, marker, _, _) in cubelet_query.iter() {
            let matches = match axis {
                render::animation::Axis::X => marker.grid.0 == fixed_val,
                render::animation::Axis::Y => marker.grid.1 == fixed_val,
                render::animation::Axis::Z => marker.grid.2 == fixed_val,
            };
            if matches {
                commands.entity(entity).insert(render::animation::RotationAnimation {
                    axis: rotation_axis,
                    total_angle,
                    elapsed: 0.0,
                    duration: anim.duration,
                });
            }
        }
        return;
    }

    // Animate
    let dt = time.delta_secs();
    anim.elapsed += dt;
    let mut finished = false;

    for (entity, _, mut transform, mut opt_rot) in cubelet_query.iter_mut() {
        if let Some(ref mut rot) = opt_rot {
            let old_frac = (rot.elapsed / rot.duration).clamp(0.0, 1.0);
            rot.elapsed += dt;
            let new_frac = (rot.elapsed / rot.duration).clamp(0.0, 1.0);
            
            let delta_angle = (new_frac - old_frac) * rot.total_angle;
            
            // Rotate around origin (0, 0, 0)
            let quat = Quat::from_axis_angle(rot.axis, delta_angle);
            transform.translation = quat * transform.translation;
            transform.rotation = quat * transform.rotation;

            if rot.elapsed >= rot.duration {
                commands.entity(entity).remove::<render::animation::RotationAnimation>();
                finished = true;
            }
        }
    }

    // Usually duration ends at the same time for all entities, catching one true is enough.
    if finished {
        if let Some(m) = anim.current_move.take() {
            // Commit move logically
            cube_res.0.apply(m);
            // Snap mesh exactly to geometric boundaries
            rebuild_mesh(&mut commands, &mut meshes, &mut materials, &cubelet_query, &cube_res.0);
        }
    }
}

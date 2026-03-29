use bevy::prelude::*;
use crate::cube::state::Facelets;

/// Face color palette (sRGB).
pub const FACE_COLORS: [Color; 6] = [
    Color::srgb(1.0, 1.0, 1.0),  // U = white
    Color::srgb(0.8, 0.1, 0.1),  // R = red
    Color::srgb(0.0, 0.6, 0.2),  // F = green
    Color::srgb(1.0, 0.85, 0.0), // D = yellow
    Color::srgb(0.9, 0.4, 0.0),  // L = orange
    Color::srgb(0.0, 0.2, 0.8),  // B = blue
];

pub const CUBELET_SIZE: f32 = 0.95;
pub const GAP: f32 = 1.0;

/// Marker component for cubelet mesh entities.
#[allow(dead_code)]
#[derive(Component)]
pub struct CubeletMarker {
    /// (x, y, z) grid position in -1..=1
    pub grid: (i8, i8, i8),
    /// which of the 6 faces this sticker belongs to (None = inner black face)
    pub face: Option<usize>,
}

/// Index into the 54-facelet array for this sticker.
#[allow(dead_code)]
#[derive(Component)]
pub struct FaceletIndex(pub usize);

/// Spawn all 26 visible cubelets (3x3x3 minus center hidden).
pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    facelets: &Facelets,
) {
    let cubelet_mesh = meshes.add(Cuboid::new(CUBELET_SIZE, CUBELET_SIZE, CUBELET_SIZE));
    let black_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.05, 0.05),
        ..default()
    });

    // Facelet layout per face (face_index -> grid positions of its 9 stickers, row-major)
    // U face (y=+1): stickers on top (+Y)
    // D face (y=-1): stickers on bottom (-Y)
    // F face (z=+1): stickers on front (+Z)
    // B face (z=-1): stickers on back (-Z)
    // R face (x=+1): stickers on right (+X)
    // L face (x=-1): stickers on left (-X)

    for gx in -1i8..=1 {
        for gy in -1i8..=1 {
            for gz in -1i8..=1 {
                if gx == 0 && gy == 0 && gz == 0 {
                    continue; // skip internal center
                }

                let pos = Vec3::new(gx as f32 * GAP, gy as f32 * GAP, gz as f32 * GAP);

                commands.spawn((
                    Mesh3d(cubelet_mesh.clone()),
                    MeshMaterial3d(black_mat.clone()),
                    Transform::from_translation(pos),
                    CubeletMarker { grid: (gx, gy, gz), face: None },
                ));

                // Spawn sticker quads on visible faces
                spawn_stickers(commands, meshes, materials, facelets, gx, gy, gz, pos);
            }
        }
    }
}

fn spawn_stickers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    facelets: &Facelets,
    gx: i8, gy: i8, gz: i8,
    cubelet_pos: Vec3,
) {
    let sticker_size = CUBELET_SIZE * 0.85;
    let sticker_mesh = meshes.add(Rectangle::new(sticker_size, sticker_size));
    let offset = CUBELET_SIZE / 2.0 + 0.01;

    // Map grid position to facelet index for each face direction
    struct StickerDef {
        face: usize,        // which face (0=U,1=R,2=F,3=D,4=L,5=B)
        facelet: usize,     // index within 0..54
        local_offset: Vec3,
        rotation: Quat,
    }

    let mut stickers: Vec<StickerDef> = Vec::new();

    // U face (gy == 1)
    if gy == 1 {
        let col = (gx + 1) as usize;
        let row = (gz + 1) as usize;   // Kociemba: gz=-1(Back) -> row0, gz=+1(Front) -> row2
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 0, facelet: 0 * 9 + fi,
            local_offset: Vec3::new(0.0, offset, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        });
    }
    // D face (gy == -1)
    if gy == -1 {
        let col = (gx + 1) as usize;
        let row = (1 - gz) as usize;   // Kociemba: gz=+1(Front) -> row0, gz=-1(Back) -> row2
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 3, facelet: 3 * 9 + fi,
            local_offset: Vec3::new(0.0, -offset, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        });
    }
    // R face (gx == 1)
    if gx == 1 {
        let col = (1 - gz) as usize;   // gz=+1->col0, gz=-1->col2
        let row = (1 - gy) as usize;
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 1, facelet: 1 * 9 + fi,
            local_offset: Vec3::new(offset, 0.0, 0.0),
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        });
    }
    // L face (gx == -1)
    if gx == -1 {
        let col = (gz + 1) as usize;
        let row = (1 - gy) as usize;
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 4, facelet: 4 * 9 + fi,
            local_offset: Vec3::new(-offset, 0.0, 0.0),
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        });
    }
    // F face (gz == 1)
    if gz == 1 {
        let col = (gx + 1) as usize;
        let row = (1 - gy) as usize;
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 2, facelet: 2 * 9 + fi,
            local_offset: Vec3::new(0.0, 0.0, offset),
            rotation: Quat::IDENTITY,
        });
    }
    // B face (gz == -1)
    if gz == -1 {
        let col = (1 - gx) as usize;
        let row = (1 - gy) as usize;
        let fi = row * 3 + col;
        stickers.push(StickerDef {
            face: 5, facelet: 5 * 9 + fi,
            local_offset: Vec3::new(0.0, 0.0, -offset),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
        });
    }

    for s in stickers {
        let color = FACE_COLORS[facelets.data[s.facelet] as usize];
        let mat = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.6,
            metallic: 0.1,
            ..default()
        });
        commands.spawn((
            Mesh3d(sticker_mesh.clone()),
            MeshMaterial3d(mat),
            Transform {
                translation: cubelet_pos + s.local_offset,
                rotation: s.rotation,
                ..default()
            },
            CubeletMarker { grid: (gx, gy, gz), face: Some(s.face) },
            FaceletIndex(s.facelet),
        ));
    }
}

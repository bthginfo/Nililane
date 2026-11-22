use bevy::{pbr::NotShadowCaster, prelude::*};

use super::{
    LOT_STEP, landscape,
    mesh::{Batch, Tint, v},
};
use crate::{campaign::Region, layout::Layout};

struct Lamp {
    fixture: Mesh,
    flame: Mesh,
    height: f32,
    warmth: Color,
}

pub(super) fn place(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    matte: &Handle<StandardMaterial>,
    layout: Layout,
    region: Region,
) {
    let Lamp {
        fixture,
        flame,
        height,
        warmth,
    } = lamp(region);
    let fixture = meshes.add(fixture);
    let flame = meshes.add(flame);
    let light = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });
    let edge = (layout.width() as f32 - 1.0) * LOT_STEP * 0.5;
    let mut row = |x: f32, y: f32, z: f32, side: f32| {
        parent
            .spawn((
                Mesh3d(fixture.clone()),
                MeshMaterial3d(matte.clone()),
                Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_y(side)),
            ))
            .with_children(|post| {
                post.spawn((
                    Mesh3d(flame.clone()),
                    MeshMaterial3d(light.clone()),
                    NotShadowCaster,
                ));
                post.spawn((
                    PointLight {
                        color: warmth,
                        intensity: 8_800.0,
                        range: 4.35,
                        radius: 0.40,
                        ..default()
                    },
                    Transform::from_xyz(0.0, height + 0.15, 0.19),
                ));
            });
    };
    for i in (0..layout.width()).step_by(2) {
        let x = i as f32 * LOT_STEP - edge;
        row(x - 1.23, 0.07, -1.88, 0.0);
        if i + 1 < layout.width() {
            row(x + LOT_STEP - 1.20, 0.07, 1.91, std::f32::consts::PI);
        }
    }
    let pier = landscape::quay(layout);
    row(pier + 0.38, 0.134, 4.79, -1.28);
}

fn lamp(region: Region) -> Lamp {
    let mut work = Batch::default();
    let mut glow = Batch::default();
    let limestone = [203, 171, 119];
    let iron = [64, 58, 59];
    let amber = [255, 190, 100];
    let height = match region {
        Region::Greece => {
            let y = 1.27;
            work.frustum(Vec3::ZERO, 0.21, 0.16, 0.15, 10, limestone);
            work.frustum(v(0.0, 0.14, 0.0), 0.095, 0.069, 0.98, 10, limestone);
            work.frustum(v(0.0, 1.11, 0.0), 0.16, 0.17, 0.071, 10, [110, 99, 64]);
            work.frustum(v(0.0, 1.18, 0.0), 0.12, 0.17, 0.096, 12, [180, 113, 63]);
            work.gem(v(0.0, 1.19, 0.14), 0.063, 0.042, 0.085, 0.2, [180, 113, 63]);
            fire(&mut glow, v(0.0, y, 0.036), 0.77, amber);
            y
        }
        Region::Rome => {
            let y = 1.44;
            work.slab(v(0.0, 0.085, 0.0), v(0.37, 0.17, 0.35), [178, 135, 102]);
            work.slab(v(0.0, 0.65, 0.0), v(0.18, 1.0, 0.19), [168, 100, 74]);
            work.slab(v(0.0, 1.16, 0.0), v(0.28, 0.13, 0.27), limestone);
            work.frustum(v(0.0, 1.24, 0.0), 0.11, 0.21, 0.14, 10, iron);
            for i in 0..8 {
                let (sin, cos) = (i as f32 * std::f32::consts::TAU / 8.0).sin_cos();
                work.beam(
                    v(cos * 0.137, 1.28, sin * 0.137),
                    v(cos * 0.208, 1.43, sin * 0.208),
                    0.034,
                    [109, 80, 62],
                );
            }
            fire(&mut glow, v(0.0, y, 0.0), 1.20, amber);
            y
        }
        Region::Japan => {
            let y = 1.40;
            let cedar = [84, 55, 42];
            work.slab(
                v(-0.09, 0.063, -0.035),
                v(0.25, 0.125, 0.25),
                [135, 139, 126],
            );
            work.beam(v(-0.09, 0.12, -0.035), v(-0.09, 1.85, -0.035), 0.087, cedar);
            work.beam(v(-0.10, 1.80, -0.043), v(0.01, 1.80, 0.24), 0.06, cedar);
            work.beam(v(-0.09, 1.58, -0.02), v(-0.012, 1.78, 0.15), 0.038, cedar);
            work.beam(v(0.0, 1.79, 0.19), v(0.0, 1.60, 0.19), 0.021, iron);
            let paper = [255, 200, 137];
            glow.frustum(v(0.0, 1.21, 0.19), 0.098, 0.153, 0.17, 12, paper);
            glow.frustum(v(0.0, 1.38, 0.19), 0.153, 0.10, 0.18, 12, [255, 218, 165]);
            for (bottom, radius) in [
                (1.21, 0.106),
                (1.29, 0.127),
                (1.38, 0.155),
                (1.47, 0.13),
                (1.56, 0.108),
            ] {
                work.frustum(
                    v(0.0, bottom, 0.19),
                    radius,
                    radius,
                    0.012,
                    12,
                    [169, 83, 58],
                );
            }
            y
        }
        Region::Nordic => {
            let y = 1.42;
            let wood = [102, 67, 48];
            for (x, z, w) in [(-0.14, 0.0, 0.13), (0.14, -0.07, 0.12), (0.05, 0.14, 0.11)] {
                work.gem(v(x, 0.07, z), w, 0.10, w * 0.93, x, [115, 126, 124]);
            }
            work.frustum(v(0.0, 0.08, 0.0), 0.095, 0.083, 1.32, 8, wood);
            work.frustum(v(0.0, 1.21, 0.0), 0.086, 0.107, 0.09, 8, iron);
            work.frustum(v(0.0, 1.30, 0.0), 0.103, 0.111, 0.14, 8, [148, 96, 64]);
            work.frustum(v(0.0, 1.37, 0.0), 0.113, 0.112, 0.043, 8, iron);
            fire(&mut glow, v(0.0, y, 0.0), 1.39, amber);
            y
        }
        Region::Oasis => {
            let y = 1.35;
            let bronze = [115, 83, 61];
            work.frustum(Vec3::ZERO, 0.23, 0.16, 0.14, 8, limestone);
            work.frustum(v(0.0, 0.14, 0.0), 0.092, 0.067, 1.01, 8, bronze);
            work.frustum(v(0.0, 1.13, 0.0), 0.084, 0.18, 0.12, 8, bronze);
            glow.frustum(v(0.0, 1.23, 0.0), 0.165, 0.16, 0.26, 8, [255, 183, 111]);
            work.frustum(v(0.0, 1.48, 0.0), 0.18, 0.115, 0.078, 8, bronze);
            work.frustum(v(0.0, 1.55, 0.0), 0.115, 0.022, 0.14, 8, [79, 119, 105]);
            for i in 0..8 {
                let (sin, cos) = (i as f32 * std::f32::consts::TAU / 8.0).sin_cos();
                work.beam(
                    v(0.166 * cos, 1.23, 0.166 * sin),
                    v(0.169 * cos, 1.49, 0.169 * sin),
                    0.033,
                    bronze,
                );
                work.gem(
                    v(0.168 * cos, 1.355, 0.168 * sin),
                    0.041,
                    0.044,
                    0.041,
                    0.18,
                    bronze,
                );
            }
            y
        }
    };
    Lamp {
        fixture: work.mesh(),
        flame: glow.mesh(),
        height,
        warmth: Color::srgb_u8(255, 183, 117),
    }
}

pub(super) fn doorway(region: Region) -> Mesh {
    let mut b = Batch::default();
    let brass = [123, 92, 65];
    let light = [255, 195, 119];
    match region {
        Region::Greece | Region::Rome => {
            b.frustum(Vec3::ZERO, 0.108, 0.079, 0.17, 9, [170, 145, 103]);
            b.frustum(v(0.0, 0.17, 0.0), 0.067, 0.118, 0.05, 9, brass);
            fire(&mut b, v(0.0, 0.205, 0.0), 0.59, light);
        }
        Region::Japan | Region::Oasis => {
            b.frustum(Vec3::ZERO, 0.11, 0.13, 0.05, 9, brass);
            b.frustum(v(0.0, 0.05, 0.0), 0.10, 0.117, 0.15, 9, light);
            b.frustum(v(0.0, 0.20, 0.0), 0.118, 0.028, 0.09, 9, brass);
            b.beam(v(0.06, 0.05, 0.084), v(0.06, 0.20, 0.101), 0.020, brass);
        }
        Region::Nordic => {
            b.frustum(Vec3::ZERO, 0.12, 0.03, 0.06, 8, brass);
            b.frustum(v(0.0, 0.06, 0.0), 0.03, 0.046, 0.35, 8, [124, 78, 52]);
            b.frustum(v(0.0, 0.36, 0.0), 0.042, 0.075, 0.06, 8, brass);
            fire(&mut b, v(0.0, 0.40, 0.0), 0.73, light);
        }
    }
    b.mesh()
}

fn fire(b: &mut Batch, at: Vec3, scale: f32, amber: Tint) {
    b.gem(
        at + v(0.0, 0.063, 0.0) * scale,
        0.087 * scale,
        0.11 * scale,
        0.075 * scale,
        0.1,
        amber,
    );
    b.gem(
        at + v(0.029, 0.123, 0.01) * scale,
        0.049 * scale,
        0.114 * scale,
        0.047 * scale,
        0.8,
        [255, 220, 138],
    );
}

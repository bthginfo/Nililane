use bevy::prelude::*;

use crate::{campaign::JAPAN_START, model::Kind};

use super::mesh::{Batch, Grain, Tint, shift, v};

const TILE: Tint = [58, 72, 84];
const TIMBER: Tint = [98, 64, 48];
const CEDAR: Tint = [153, 104, 70];
const PLASTER: Tint = [239, 222, 184];
const STONE: Tint = [169, 168, 153];
const INDIGO: Tint = [62, 97, 139];
const JADE: Tint = [84, 127, 86];
const RED: Tint = [187, 83, 59];
const GOLD: Tint = [232, 174, 89];
const PAPER: Tint = [251, 205, 136];

pub const BELL_HOOK: Vec3 = Vec3::new(0.0, 1.63, 0.75);

pub fn model(kind: Kind, level: u8) -> Mesh {
    let mut b = Batch::default();
    use Kind::*;
    match kind {
        Machiya => townhouse(&mut b, false),
        Inn => townhouse(&mut b, true),
        RicePaddy => paddy(&mut b),
        TeaGarden => tea_garden(&mut b),
        RiceKitchen | TeaHouse | PaperWorkshop => storefront(&mut b, kind),
        Well => well(&mut b),
        Temple => temple(&mut b),
        Storehouse => kura(&mut b),
        _ => unreachable!("building unavailable in Japan"),
    }
    for tier in 2..=level {
        append_upgrade(&mut b, kind, tier);
    }
    b.mesh()
}

// Faceted ceramic eaves. The lower profile sweeps back up at the corners.
pub(super) fn roof(b: &mut Batch, center: Vec3, width: f32, depth: f32, rise: f32, tile: Tint) {
    let profile = [(0.0, 1.0), (0.46, 0.44), (0.81, 0.0), (1.0, 0.06)];
    let point = |i: usize, u: f32, side: f32| {
        let (z, y) = profile[i];
        center
            + v(
                u * width * (0.79 + z * 0.21),
                rise * y + u.abs().powi(6) * z * 0.095,
                side * z * depth,
            )
    };
    for side in [-1.0, 1.0] {
        for ix in 0..12 {
            let a = ix as f32 / 6.0 - 1.0;
            let c = a + 1.0 / 6.0;
            for i in 0..3 {
                let mut panel = [
                    point(i, a, side),
                    point(i + 1, a, side),
                    point(i + 1, c, side),
                    point(i, c, side),
                ];
                if side < 0.0 {
                    panel.reverse();
                }
                b.face(
                    &panel,
                    shift(tile, (ix * 11 % 17) as i16 - 5 + i as i16 * 3),
                );
                b.beam(
                    panel[0] + v(0.0, 0.012, 0.0),
                    panel[1] + v(0.0, 0.012, 0.0),
                    0.033,
                    shift(tile, 13),
                );
            }
            let a = point(3, a, side);
            let c = point(3, c, side);
            b.beam(a, c, 0.068, shift(tile, -12));
        }
    }
    for side in [-1.0, 1.0] {
        for i in 0..3 {
            for z in [-1.0, 1.0] {
                b.beam(
                    point(i, side, z),
                    point(i + 1, side, z),
                    0.061,
                    shift(tile, 10),
                );
                let mut gable = [
                    center + v(side * width * 0.79, -0.064, z * depth * profile[i].0),
                    center + v(side * width * 0.79, -0.064, z * depth * profile[i + 1].0),
                    point(i + 1, side, z),
                    point(i, side, z),
                ];
                if side * z < 0.0 {
                    gable.reverse();
                }
                b.face(&gable, TIMBER);
            }
        }
    }
    b.beam(
        center + v(-width * 0.84, rise, 0.0),
        center + v(width * 0.84, rise, 0.0),
        0.112,
        shift(tile, 9),
    );
    for x in [-width * 0.84, width * 0.84] {
        b.gem(
            center + v(x, rise + 0.055, 0.0),
            0.076,
            0.095,
            0.055,
            0.0,
            shift(tile, 21),
        );
    }
}

fn lattice(b: &mut Batch, at: Vec3, width: f32, height: f32, panel: Tint) {
    b.slab(at, v(width, height, 0.06), panel);
    for i in 0..=4 {
        let x = (i as f32 / 4.0 - 0.5) * width;
        b.slab(
            at + v(x, 0.0, 0.044),
            v(0.031, height + 0.025, 0.027),
            TIMBER,
        );
    }
    for i in [-0.5, -0.17, 0.17, 0.5] {
        b.slab(
            at + v(0.0, i * height, 0.044),
            v(width, 0.034, 0.028),
            TIMBER,
        );
    }
}

fn lantern(b: &mut Batch, at: Vec3, color: Tint, scale: f32) {
    let size = scale * 0.106;
    b.gem(at, size, size * 1.35, size, 0.5, color);
    for y in [-0.11, 0.11] {
        b.frustum(
            at + v(0.0, y * scale, 0.0),
            size * 0.53,
            size * 0.49,
            0.029 * scale,
            8,
            TIMBER,
        );
    }
    b.beam(
        at + v(0.0, scale * 0.12, 0.0),
        at + v(0.0, scale * 0.28, 0.0),
        0.016,
        TIMBER,
    );
}

fn foundation(b: &mut Batch, width: f32, depth: f32) {
    b.slab(v(0.0, 0.12, 0.02), v(width, 0.22, depth), STONE);
    let span = width / 6.0;
    for i in 0..6 {
        let x = (i as f32 - 2.5) * span;
        b.slab(
            v(x, 0.12, depth * 0.5 + 0.035),
            v(span - 0.02, 0.16, 0.08),
            shift(STONE, (i * 11 % 21) as i16 - 8),
        );
    }
}

fn townhouse(b: &mut Batch, inn: bool) {
    let scale = if inn { 1.12 } else { 0.91 };
    let upper = if inn { 1.98 } else { 1.8 };
    let width = 2.23 * scale;
    foundation(b, width, 1.66);
    b.slab(v(0.0, 0.95, 0.0), v(width - 0.1, 1.54, 1.50), PLASTER);
    for x in [-1.0, -0.38, 0.38, 1.0] {
        b.slab(v(x * scale, 0.97, 0.757), v(0.091, 1.54, 0.114), TIMBER);
    }
    for y in [0.33, 1.04, 1.65] {
        b.slab(v(0.0, y, 0.79), v(width, 0.091, 0.114), TIMBER);
    }
    for side in [-1.0, 1.0] {
        lattice(
            b,
            v(side * 0.72 * scale, 0.65, 0.77),
            0.57 * scale,
            0.65,
            CEDAR,
        );
        lattice(
            b,
            v(side * 0.52 * scale, 1.38, 0.77),
            0.63 * scale,
            0.41,
            [235, 193, 132],
        );
    }
    lattice(b, v(0.0, 0.64, 0.779), 0.67, 0.80, shift(CEDAR, -14));
    roof(b, v(0.0, upper, -0.02), 1.39 * scale, 1.08, 0.63, TILE);
    b.beam(
        v(-1.08 * scale, 1.15, 1.05),
        v(1.08 * scale, 1.15, 1.05),
        0.054,
        TIMBER,
    );
    for n in -4..=4 {
        b.slab(
            v(n as f32 * scale * 0.24, 1.08, 1.05),
            v(0.032, 0.22, 0.042),
            CEDAR,
        );
    }
    if inn {
        b.slab(v(0.0, 1.32, 1.11), v(0.72, 0.18, 0.064), INDIGO);
        b.slab(v(0.0, 1.33, 1.15), v(0.39, 0.026, 0.026), PLASTER);
        for x in [-0.84, 0.84] {
            lantern(b, v(x, 0.89, 1.07), PAPER, 0.91);
        }
    } else {
        b.slab(v(-0.89, 0.35, 1.09), v(0.44, 0.22, 0.20), TIMBER);
        b.gem(v(-0.89, 0.51, 1.09), 0.30, 0.11, 0.17, 0.4, JADE);
    }
}

fn storefront(b: &mut Batch, kind: Kind) {
    let (fabric, walls, y) = match kind {
        Kind::RiceKitchen => (INDIGO, [222, 189, 142], 1.24),
        Kind::TeaHouse => (JADE, [205, 157, 97], 1.11),
        Kind::PaperWorkshop => (RED, [166, 104, 72], 1.47),
        _ => unreachable!(),
    };
    foundation(b, 2.44, 1.74);
    b.slab(v(0.0, 0.22 + y * 0.5, -0.23), v(2.11, y, 1.11), walls);
    for x in [-1.01, -0.36, 0.36, 1.01] {
        b.slab(v(x, 0.20 + y * 0.5, 0.70), v(0.089, y, 0.096), TIMBER);
    }
    b.slab(v(0.0, y + 0.19, 0.68), v(2.35, 0.127, 0.139), CEDAR);
    roof(
        b,
        v(0.0, y + 0.24, -0.12),
        1.39,
        1.11,
        if kind == Kind::PaperWorkshop {
            0.66
        } else {
            0.38
        },
        if kind == Kind::TeaHouse {
            [83, 99, 63]
        } else {
            TILE
        },
    );
    lattice(b, v(0.57, 0.75, 0.35), 0.68, 0.62, shift(walls, 29));
    b.slab(v(0.0, 0.62, 0.74), v(1.72, 0.108, 0.54), TIMBER);
    for (i, x) in [-0.83, -0.42, -0.01, 0.40, 0.81].into_iter().enumerate() {
        let hem = 0.91 + f32::from(kind == Kind::PaperWorkshop) * 0.16;
        b.block(
            v(x, hem + 0.14, 0.96),
            v(0.383, 0.31, 0.048),
            Quat::from_rotation_x(-0.11),
            shift(fabric, i as i16 * 4 - 7),
        );
        b.slab(v(x, hem + 0.038, 0.995), v(0.315, 0.026, 0.018), PAPER);
    }
    match kind {
        Kind::RiceKitchen => {
            for i in 0..3 {
                let p = v(-0.56 + i as f32 * 0.44, 0.685, 0.87);
                b.frustum(p, 0.065, 0.139, 0.083, 9, shift(INDIGO, i as i16 * 8));
                b.gem(p + v(0.0, 0.087, 0.0), 0.108, 0.03, 0.092, 0.2, PLASTER);
            }
        }
        Kind::TeaHouse => {
            b.slab(v(-0.49, 0.31, 1.04), v(0.87, 0.117, 0.42), CEDAR);
            b.slab(v(-0.49, 0.38, 1.04), v(0.72, 0.037, 0.35), [168, 155, 86]);
            for x in [-0.59, -0.32, 0.47] {
                b.frustum(v(x, 0.68, 0.85), 0.046, 0.066, 0.080, 8, shift(JADE, 28));
            }
            bamboo(b, v(1.05, 0.095, 0.99), 0.57);
        }
        Kind::PaperWorkshop => {
            for (i, x) in [-1.08, -0.55, -0.03, 0.60, 1.08].into_iter().enumerate() {
                lantern(
                    b,
                    v(x, 1.19 + 0.08 * (i % 2) as f32, 1.05),
                    [PAPER, RED, [222, 139, 130]][i % 3],
                    1.23,
                );
            }
            lantern(b, v(0.65, 0.87, 0.79), PAPER, 1.25);
        }
        _ => unreachable!(),
    }
}

fn paddy(b: &mut Batch) {
    for (row, z) in [-0.57, 0.49].into_iter().enumerate() {
        let y = 0.12 + (1 - row) as f32 * 0.10;
        b.slab(v(0.0, y, z), v(2.47, 0.13, 0.98), [127, 124, 79]);
        b.slab(v(0.0, y + 0.070, z), v(2.21, 0.006, 0.73), [99, 150, 150]);
        for x in [-1.18, 1.18] {
            b.slab(v(x, y + 0.049, z), v(0.12, 0.084, 0.97), [167, 153, 91]);
        }
        for dz in [-0.42, 0.42] {
            b.slab(
                v(0.0, y + 0.042, z + dz),
                v(2.44, 0.087, 0.11),
                [156, 143, 89],
            );
        }
        for x in -4i32..=4 {
            for n in [-1.0, 1.0] {
                let p = v(x as f32 * 0.23, y + 0.072, z + n * 0.20);
                rice_shoot(b, p, 0.13 + (x + 7).rem_euclid(3) as f32 * 0.043, JADE);
            }
        }
    }
}

fn rice_shoot(b: &mut Batch, at: Vec3, size: f32, color: Tint) {
    for (i, side) in [-1.0, 0.27, 1.0].into_iter().enumerate() {
        b.beam(
            at + v(side * 0.022, 0.0, 0.0),
            at + v(
                side * size * 0.34,
                size * (1.08 - side.abs() * 0.28),
                side * 0.045,
            ),
            0.024,
            shift(color, i as i16 * 9),
        );
    }
}

fn tea_garden(b: &mut Batch) {
    b.slab(v(0.0, 0.13, 0.0), v(2.41, 0.21, 2.08), [123, 129, 83]);
    for row in 0..3 {
        let z = -0.66 + row as f32 * 0.62;
        b.slab(v(0.0, 0.25, z), v(2.31, 0.10, 0.42), CEDAR);
        for i in 0..7 {
            let x = -0.93 + i as f32 * 0.31;
            b.gem(
                v(x, 0.40, z),
                0.237,
                0.19,
                0.23,
                i as f32,
                shift(JADE, (i * 9 % 24) as i16),
            );
        }
    }
    stone_lantern(b, v(0.91, 0.24, 0.79), 0.50);
}

fn well(b: &mut Batch) {
    foundation(b, 1.77, 1.53);
    b.frustum(v(0.0, 0.22, 0.07), 0.57, 0.53, 0.36, 10, STONE);
    b.frustum(v(0.0, 0.57, 0.07), 0.57, 0.55, 0.075, 10, PLASTER);
    b.frustum(v(0.0, 0.652, 0.07), 0.41, 0.425, 0.006, 10, [75, 129, 128]);
    for x in [-0.66, 0.66] {
        b.slab(v(x, 0.83, -0.12), v(0.132, 1.29, 0.126), TIMBER);
    }
    roof(b, v(0.0, 1.44, -0.11), 0.96, 0.67, 0.34, TILE);
    b.beam(v(-0.64, 1.02, -0.12), v(0.72, 1.02, -0.12), 0.079, CEDAR);
    b.beam(v(0.15, 1.01, -0.12), v(0.15, 0.68, -0.12), 0.019, GOLD);
    b.frustum(v(0.15, 0.62, -0.12), 0.10, 0.118, 0.144, 8, CEDAR);
    b.frustum(v(-0.66, 0.24, 0.59), 0.15, 0.17, 0.22, 9, TIMBER);
}

fn temple(b: &mut Batch) {
    foundation(b, 2.57, 1.96);
    b.slab(v(0.0, 0.28, 0.03), v(2.37, 0.15, 1.79), CEDAR);
    for (z, y) in [(1.16, 0.12), (1.02, 0.20), (0.88, 0.29)] {
        b.slab(v(0.0, y, z), v(0.91, 0.15, 0.30), STONE);
    }
    for z in [-0.64, 0.68] {
        for x in [-1.02, 1.02] {
            b.slab(v(x, 0.96, z), v(0.16, 1.32, 0.17), RED);
            b.slab(v(x, 0.39, z), v(0.24, 0.15, 0.24), GOLD);
            b.slab(v(x, 1.56, z), v(0.32, 0.11, 0.29), CEDAR);
        }
    }
    b.slab(v(0.0, 0.94, -0.56), v(1.91, 1.14, 0.19), TIMBER);
    lattice(b, v(0.0, 0.94, -0.44), 1.02, 0.86, PAPER);
    for x in [-1.03, 1.03] {
        b.slab(v(x, 0.67, 0.08), v(0.09, 0.08, 1.22), RED);
    }
    b.slab(v(0.0, 1.57, 0.71), v(2.37, 0.13, 0.22), RED);
    roof(b, v(0.0, 1.69, -0.03), 1.52, 1.15, 0.59, TILE);
    b.slab(v(0.0, 2.19, -0.07), v(1.15, 0.24, 0.42), RED);
    roof(b, v(0.0, 2.31, -0.07), 0.70, 0.49, 0.31, shift(TILE, 10));
    for x in [-0.83, 0.83] {
        lantern(b, v(x, 1.23, 0.73), GOLD, 0.85);
    }
}

pub fn bell() -> Mesh {
    let mut b = Batch::default();
    let bronze = [97, 133, 119];
    b.beam(v(0.0, -0.16, 0.0), v(0.0, 0.02, 0.0), 0.065, GOLD);
    b.frustum(
        v(0.0, -0.50, 0.0),
        0.217,
        0.239,
        0.044,
        12,
        shift(bronze, -17),
    );
    b.frustum(v(0.0, -0.456, 0.0), 0.239, 0.165, 0.111, 12, bronze);
    b.frustum(
        v(0.0, -0.345, 0.0),
        0.165,
        0.123,
        0.182,
        12,
        shift(bronze, 11),
    );
    b.frustum(v(0.0, -0.183, 0.0), 0.14, 0.133, 0.036, 12, GOLD);
    b.beam(v(0.31, -0.37, 0.16), v(0.31, -0.83, 0.18), 0.028, PAPER);
    b.gem(v(0.31, -0.84, 0.181), 0.047, 0.061, 0.036, 0.1, GOLD);
    b.mesh()
}

fn kura(b: &mut Batch) {
    foundation(b, 2.08, 1.72);
    b.slab(v(0.0, 0.97, -0.02), v(1.91, 1.60, 1.52), PLASTER);
    b.slab(v(0.0, 0.50, 0.77), v(1.92, 0.54, 0.062), TILE);
    for i in -4..=4 {
        let x = i as f32 * 0.21;
        let y = 0.47;
        let corners = [
            v(x - 0.104, y, 0.819),
            v(x, y - 0.18, 0.819),
            v(x + 0.104, y, 0.819),
            v(x, y + 0.18, 0.819),
        ];
        for (a, c) in corners.into_iter().zip(corners.into_iter().cycle().skip(1)) {
            b.beam(a, c, 0.028, PLASTER);
        }
    }
    lattice(b, v(0.0, 0.65, 0.864), 0.61, 0.85, TIMBER);
    b.slab(v(0.0, 1.42, 0.762), v(0.39, 0.21, 0.042), shift(TILE, -25));
    for x in [-0.13, 0.0, 0.13] {
        b.slab(v(x, 1.42, 0.80), v(0.026, 0.19, 0.028), STONE);
    }
    roof(b, v(0.0, 1.72, -0.06), 1.33, 0.99, 0.55, TILE);
    crates(b, v(0.90, 0.13, 0.94), 0.35);
}

fn crates(b: &mut Batch, at: Vec3, size: f32) {
    b.slab(at + Vec3::Y * size * 0.5, Vec3::splat(size), CEDAR);
    for y in [0.15, 0.79] {
        b.slab(
            at + v(0.0, y * size, 0.51 * size),
            v(size * 1.025, size * 0.11, size * 0.045),
            TIMBER,
        );
    }
}

pub fn append_upgrade(b: &mut Batch, kind: Kind, tier: u8) {
    use Kind::*;
    match kind {
        RicePaddy => {
            let x = if tier.is_multiple_of(2) { 0.81 } else { -0.81 };
            let z = -0.89 + f32::from(tier - 2) * 0.46;
            if tier == 4 {
                for x in [-1.11, 1.11] {
                    b.beam(v(x, 0.315, -0.99), v(x, 0.21, 0.93), 0.084, STONE);
                }
                b.slab(v(0.0, 0.218, -0.02), v(2.45, 0.041, 0.10), [117, 177, 179]);
            } else {
                for i in -1..=1 {
                    rice_shoot(
                        b,
                        v(x + i as f32 * 0.17, 0.28, z),
                        0.25,
                        [193, 173 + tier * 5, 81],
                    );
                }
            }
            if tier == 6 {
                b.slab(v(-0.15, 0.39, -1.0), v(1.33, 0.07, 0.27), CEDAR);
                for x in [-0.64, -0.18, 0.31] {
                    b.gem(v(x, 0.48, -1.0), 0.16, 0.103, 0.135, x, GOLD);
                }
            }
        }
        TeaGarden => {
            let x = if tier.is_multiple_of(2) { -1.0 } else { 0.99 };
            let z = -0.95 + f32::from(tier - 2) * 0.42;
            match tier {
                2 | 4 => bamboo(b, v(x, 0.23, z), 0.62 + 0.10 * f32::from(tier)),
                3 | 5 => stone_lantern(b, v(x, 0.22, z), 0.61 + 0.06 * f32::from(tier)),
                _ => maple(b, v(-0.40, 0.25, -0.73), 0.91, RED),
            }
        }
        Temple => match tier {
            2 => {
                for x in [-0.95, 0.95] {
                    stone_lantern(b, v(x, 0.10, 1.03), 0.69);
                }
            }
            3 => {
                for x in [-0.84, 0.84] {
                    b.slab(v(x, 1.78, -0.24), v(0.34, 0.41, 0.37), RED);
                    roof(b, v(x, 2.0, -0.25), 0.32, 0.32, 0.22, shift(TILE, -5));
                }
            }
            4 => {
                b.slab(v(0.0, 2.57, -0.07), v(0.52, 0.37, 0.49), RED);
                roof(b, v(0.0, 2.77, -0.07), 0.52, 0.42, 0.26, TILE);
            }
            5 => {
                for x in [-1.15, 1.15] {
                    b.slab(v(x, 0.56, 0.96), v(0.086, 0.85, 0.08), RED);
                    lantern(b, v(x, 0.91, 1.09), RED, 1.07);
                    b.slab(v(x, 0.46, 0.84), v(0.073, 0.061, 0.52), RED);
                }
            }
            _ => {
                b.beam(v(0.0, 3.02, -0.07), v(0.0, 3.53, -0.07), 0.055, GOLD);
                for y in [3.10, 3.21, 3.32] {
                    b.frustum(v(0.0, y, -0.07), 0.14, 0.073, 0.052, 8, GOLD);
                }
                b.gem(v(0.0, 3.56, -0.07), 0.078, 0.087, 0.078, 0.2, GOLD);
            }
        },
        Machiya | Inn => match tier {
            2 => {
                b.slab(v(0.0, 0.28, 1.06), v(1.37, 0.13, 0.42), CEDAR);
                for x in [-0.68, 0.68] {
                    lantern(b, v(x, 1.0, 1.13), PAPER, 0.80);
                }
            }
            3 => {
                // A small gabled dormer follows the main roof's silhouette.
                let y = if kind == Inn { 2.17 } else { 1.99 };
                b.slab(v(0.0, y + 0.10, 0.61), v(0.87, 0.49, 0.43), PLASTER);
                lattice(b, v(0.0, y + 0.11, 0.84), 0.57, 0.30, CEDAR);
                roof(b, v(0.0, y + 0.32, 0.59), 0.55, 0.35, 0.29, TILE);
            }
            4 => veranda(b, 1.12, if kind == Inn { INDIGO } else { CEDAR }),
            5 => {
                for x in [-0.91, 0.91] {
                    lattice(b, v(x, 1.39, 0.854), 0.27, 0.46, PAPER);
                }
                b.slab(v(0.0, 1.10, 1.09), v(1.74, 0.12, 0.31), CEDAR);
            }
            _ => {
                let y = if kind == Inn { 2.59 } else { 2.41 };
                b.slab(v(0.0, y - 0.02, -0.13), v(0.83, 0.37, 0.45), CEDAR);
                lattice(b, v(0.0, y, 0.12), 0.55, 0.24, PAPER);
                roof(b, v(0.0, y + 0.17, -0.13), 0.59, 0.42, 0.29, TILE);
            }
        },
        RiceKitchen | TeaHouse | PaperWorkshop => {
            let color = match kind {
                RiceKitchen => INDIGO,
                TeaHouse => JADE,
                _ => RED,
            };
            match tier {
                2 => {
                    for x in [-0.81, 0.81] {
                        crates(b, v(x, 0.18, 1.01), 0.26);
                    }
                }
                3 => {
                    let y = if kind == PaperWorkshop { 1.48 } else { 1.22 };
                    veranda(b, y, color);
                }
                4 => {
                    for (x, s) in [(-0.75, 0.88), (0.05, 1.10), (0.72, 0.80)] {
                        lantern(
                            b,
                            v(x, 0.86 + f32::from(kind == PaperWorkshop) * 0.21, 1.25),
                            shift(color, 46),
                            s,
                        );
                    }
                }
                5 => {
                    let y = if kind == PaperWorkshop { 1.95 } else { 1.59 };
                    b.slab(v(0.0, y, -0.43), v(1.15, 0.76, 0.64), PLASTER);
                    lattice(b, v(0.0, y + 0.10, -0.10), 0.74, 0.35, color);
                    roof(b, v(0.0, y + 0.41, -0.36), 0.78, 0.48, 0.34, TILE);
                }
                _ => {
                    for x in [-0.93, 0.93] {
                        b.beam(v(x, 0.11, 1.19), v(x, 0.70, 1.19), 0.063, CEDAR);
                        let y = if kind == PaperWorkshop { 1.75 } else { 1.49 };
                        b.slab(v(x, y, 0.85), v(0.34, 0.26, 0.085), color);
                        b.slab(v(x, y, 0.90), v(0.21, 0.037, 0.021), GOLD);
                    }
                }
            }
        }
        Well | Storehouse => match tier {
            2 => {
                for x in [-0.83, 0.83] {
                    crates(b, v(x, 0.17, 0.94), 0.31);
                }
            }
            3 => veranda(b, if kind == Well { 1.07 } else { 1.52 }, INDIGO),
            4 => {
                for x in [-0.89, 0.89] {
                    lantern(b, v(x, 0.87, 1.28), PAPER, 0.81);
                }
            }
            5 => {
                let y = if kind == Well { 1.63 } else { 1.97 };
                b.slab(v(0.0, y, -0.33), v(0.94, 0.62, 0.51), PLASTER);
                lattice(b, v(0.0, y + 0.05, -0.066), 0.48, 0.24, CEDAR);
                roof(b, v(0.0, y + 0.34, -0.28), 0.67, 0.42, 0.31, TILE);
            }
            _ => {
                for x in [-0.98, 0.98] {
                    stone_lantern(b, v(x, 0.11, 1.17), 0.63);
                }
            }
        },
        _ => unreachable!("building unavailable in Japan"),
    }
}

fn veranda(b: &mut Batch, y: f32, color: Tint) {
    for x in [-1.09, 1.09] {
        b.slab(v(x, 0.12 + y * 0.5, 1.19), v(0.084, y, 0.094), TIMBER);
    }
    for i in 0..9 {
        let x = (i as f32 - 4.0) * 0.254;
        b.block(
            v(x, y + 0.116, 1.09),
            v(0.253, 0.051, 0.48),
            Quat::from_rotation_x(0.20),
            shift(color, (i * 11 % 18) as i16 - 6),
        );
    }
    b.beam(
        v(-1.17, y + 0.061, 1.31),
        v(1.17, y + 0.061, 1.31),
        0.060,
        TIMBER,
    );
}

fn bamboo(b: &mut Batch, at: Vec3, scale: f32) {
    for (n, x) in [-0.14, 0.03, 0.16].into_iter().enumerate() {
        let foot = at + v(x * scale, 0.0, 0.0);
        let top = foot + v(x * scale * 0.47, scale * (0.93 + n as f32 * 0.18), 0.04);
        b.beam(foot, top, scale * 0.047, [141, 163, 91]);
        for i in 1..4 {
            let joint = foot.lerp(top, i as f32 / 4.0);
            b.frustum(
                joint,
                scale * 0.035,
                scale * 0.027,
                scale * 0.038,
                6,
                [190, 193, 106],
            );
            let side = if i % 2 == 0 { 1.0 } else { -1.0 };
            b.beam(
                joint,
                joint + v(side * scale * 0.21, scale * 0.14, 0.0),
                scale * 0.023,
                JADE,
            );
            b.gem(
                joint + v(side * scale * 0.25, scale * 0.15, 0.0),
                scale * 0.14,
                scale * 0.036,
                scale * 0.079,
                i as f32,
                shift(JADE, i * 8),
            );
        }
    }
}

fn maple(b: &mut Batch, at: Vec3, scale: f32, foliage: Tint) {
    let mid = at + v(-0.09, 0.71, 0.02) * scale;
    b.beam(at, mid, 0.13 * scale, TIMBER);
    for (i, (x, y, z, size)) in [
        (-0.35, 0.96, 0.14, 0.36),
        (0.36, 1.1, 0.04, 0.42),
        (0.06, 1.29, -0.18, 0.49),
        (-0.20, 1.40, 0.07, 0.38),
        (0.14, 0.97, 0.27, 0.32),
    ]
    .into_iter()
    .enumerate()
    {
        let p = at + v(x, y, z) * scale;
        b.beam(mid, p - Vec3::Y * size * scale * 0.27, 0.055 * scale, CEDAR);
        b.gem(
            p,
            size * scale,
            size * 0.51 * scale,
            size * 0.83 * scale,
            i as f32,
            shift(foliage, i as i16 * 7 - 17),
        );
    }
}

fn cedar(b: &mut Batch, at: Vec3, size: f32, color: Tint) {
    b.frustum(at, size * 0.08, size * 0.035, size * 1.19, 7, TIMBER);
    for n in 0..4 {
        let tier = n as f32;
        b.frustum(
            at + Vec3::Y * size * (0.31 + tier * 0.25),
            size * (0.41 - tier * 0.072),
            size * 0.035,
            size * (0.44 - tier * 0.023),
            7,
            shift(color, n * 9),
        );
    }
}

fn stone_lantern(b: &mut Batch, at: Vec3, scale: f32) {
    b.frustum(at, 0.23 * scale, 0.18 * scale, 0.13 * scale, 6, STONE);
    b.frustum(
        at + Vec3::Y * 0.13 * scale,
        0.104 * scale,
        0.081 * scale,
        0.29 * scale,
        6,
        shift(STONE, -12),
    );
    b.slab(
        at + Vec3::Y * 0.46 * scale,
        v(0.33, 0.09, 0.33) * scale,
        STONE,
    );
    b.slab(
        at + Vec3::Y * 0.55 * scale,
        Vec3::splat(0.19 * scale),
        PAPER,
    );
    for x in [-0.11, 0.11] {
        for z in [-0.11, 0.11] {
            b.slab(
                at + v(x, 0.55, z) * scale,
                v(0.069, 0.22, 0.067) * scale,
                STONE,
            );
        }
    }
    b.frustum(
        at + Vec3::Y * 0.65 * scale,
        0.27 * scale,
        0.07 * scale,
        0.16 * scale,
        6,
        shift(STONE, -31),
    );
    b.gem(
        at + Vec3::Y * 0.83 * scale,
        0.064 * scale,
        0.079 * scale,
        0.064 * scale,
        0.2,
        STONE,
    );
}

pub fn paving(i: usize, j: usize, chapter: usize) -> Tint {
    let ground = [
        [202, 186, 155],
        [191, 184, 159],
        [189, 182, 156],
        [204, 177, 148],
        [187, 177, 166],
        [183, 181, 157],
    ][(chapter - JAPAN_START) % 6];
    let edge = if (j == 0 || j == 9) && i % 7 < 4 {
        -20
    } else {
        0
    };
    shift(ground, (i * 19 + j * 23) as i16 % 21 - 10 + edge)
}

pub fn garden(b: &mut Batch, at: Vec3, chapter: usize, grain: &mut Grain) {
    let tints = [
        [191, 130, 137],
        [209, 133, 102],
        [139, 158, 91],
        [200, 155, 92],
    ];
    let foliage = tints[((chapter - JAPAN_START) / 3) % tints.len()];
    maple(
        b,
        at + v(-0.46, 0.01, 0.13),
        grain.span(0.73, 1.01),
        foliage,
    );
    bamboo(b, at + v(0.65, 0.02, 0.46), grain.span(0.69, 1.12));
    stone_lantern(b, at + v(-0.48, 0.03, -0.67), 0.72);
}

pub fn bank(b: &mut Batch, wide: f32, chapter: usize, grain: &mut Grain) {
    let local = chapter - JAPAN_START;
    let foliage = [
        [219, 150, 155],
        [127, 157, 101],
        [204, 124, 76],
        [180, 139, 87],
    ][local / 6];
    let hill = [
        [116, 143, 104],
        [113, 146, 108],
        [144, 144, 93],
        [121, 139, 106],
    ][local / 6];
    for i in 0..11 {
        let x = -wide + 1.00 + i as f32 * (wide * 2.0 - 2.4) / 10.0;
        let z = grain.span(-6.1, -5.01);
        let p = v(x, -0.29, z);
        let h = grain.span(0.50, 1.32) + (i % 4) as f32 * 0.13;
        b.gem(
            p,
            1.96,
            h,
            1.46,
            grain.span(0.0, 3.0),
            shift(hill, i as i16 % 3 * 6),
        );
        for n in 0..3 {
            let tree = p + v(grain.span(-0.48, 0.48), h * 0.57, grain.span(-0.39, 0.43));
            let size = grain.span(0.59, 1.42);
            if (i + n + local).is_multiple_of(5) {
                maple(b, tree, size, foliage);
            } else {
                cedar(b, tree, size, shift(hill, -22 + n as i16 * 8));
            }
        }
    }

    let x = if local.is_multiple_of(2) {
        wide - 0.92
    } else {
        -wide + 0.94
    };
    let z = if local % 4 < 2 { 3.79 } else { -4.2 };
    match local % 4 {
        0 => {
            // Stone harbor lantern, shaped after the lights of the Inland Sea.
            stone_lantern(b, v(x, 0.02, z), 2.37);
            b.slab(v(x, 0.08, z), v(1.36, 0.14, 1.11), STONE);
        }
        1 => {
            // Vermilion waterfront torii stands apart from the Buddhist temple.
            for dx in [-0.74, 0.74] {
                b.frustum(v(x + dx, -0.10, z), 0.16, 0.112, 1.81, 9, RED);
                b.frustum(v(x + dx, -0.12, z), 0.17, 0.157, 0.29, 9, TIMBER);
            }
            b.beam(v(x - 1.02, 1.37, z), v(x + 1.02, 1.37, z), 0.117, RED);
            b.slab(v(x, 1.74, z), v(2.21, 0.141, 0.27), RED);
            for dx in [-1.0, 1.0] {
                b.beam(
                    v(x + dx * 0.79, 1.81, z),
                    v(x + dx * 1.17, 1.93, z),
                    0.13,
                    TILE,
                );
            }
            b.slab(v(x, 1.85, z), v(1.72, 0.095, 0.29), TILE);
        }
        2 => {
            for i in 0..6 {
                let w = 1.23 - i as f32 * 0.10;
                b.slab(
                    v(x, 0.07 + i as f32 * 0.125, z + 0.57 - i as f32 * 0.21),
                    v(w, 0.148, 0.53),
                    shift(STONE, i as i16 * 3),
                );
            }
            stone_lantern(b, v(x - 0.64, 0.15, z - 0.60), 0.87);
            maple(b, v(x + 0.60, 0.12, z - 0.61), 0.89, foliage);
        }
        _ => {
            for i in -1..=1 {
                bamboo(b, v(x + i as f32 * 0.43, 0.0, z + 0.20), 1.27);
            }
            stone_lantern(b, v(x, 0.04, z - 0.52), 1.19);
        }
    }
    for i in 0..5 {
        let x = -wide + 1.6 + i as f32 * (wide * 2.0 - 3.20) / 4.0;
        if (i + local) % 3 != 2 {
            let at = v(x, -0.23, 5.0 + grain.span(-0.2, 0.26));
            b.gem(
                at,
                grain.span(0.29, 0.73),
                0.35,
                0.32,
                i as f32,
                shift(STONE, -14),
            );
        }
    }
}

pub fn distant(chapter: usize) -> Mesh {
    let mut b = Batch::default();
    let mut grain = Grain::new(chapter as u32 * 2129 + 91);
    for i in 0..7 {
        let x = -33.0 + i as f32 * 11.7;
        let z = -14.5 - (i % 3) as f32 * 7.7 + grain.span(-1.3, 1.3);
        for j in 0..4 {
            let at = v(x + j as f32 * 2.2, -0.7, z + grain.span(-1.7, 1.7));
            let color = shift([105, 148, 135], (j * 11 + i * 7) as i16 % 31 - 12);
            b.gem(
                at,
                grain.span(3.0, 5.5),
                grain.span(1.6, 4.7),
                grain.span(2.2, 4.2),
                0.7 * j as f32,
                color,
            );
        }
    }
    b.mesh()
}

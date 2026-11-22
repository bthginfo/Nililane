use bevy::prelude::*;

use super::{
    mesh::{Batch, Tint, shift, v},
    upgrades::{bench, canopy, crane, crate_box, rail},
};
use crate::model::Kind;

pub const IVORY: Tint = [239, 211, 165];
pub const MARBLE: Tint = [246, 225, 188];
pub const CREAM: Tint = [221, 182, 124];
pub const TERRACOTTA: Tint = [182, 82, 54];
pub const TEAL: Tint = [45, 113, 113];
pub const BLUE: Tint = [61, 96, 137];
pub const OAK: Tint = [99, 63, 48];
pub const HERB: Tint = [100, 127, 62];
pub const GOLD: Tint = [225, 163, 75];

pub fn model(kind: Kind, level: u8) -> Mesh {
    let mut b = Batch::default();
    match kind {
        Kind::House => yard(&mut b),
        Kind::Well => well(&mut b),
        Kind::Grove => grove(&mut b),
        Kind::Stoa => stoa(&mut b),
        Kind::Bakery => bakehouse(&mut b, false),
        Kind::Taverna => tavern(&mut b, false),
        Kind::Potter | Kind::Inn => shop(&mut b, kind),
        Kind::HerbGarden => herb_garden(&mut b, false),
        Kind::Storehouse => storage(&mut b),
        Kind::Press => press(&mut b),
        Kind::Shrine => sanctuary(&mut b),
        Kind::Aqueduct | Kind::Baths | Kind::Forum | Kind::Villa => {
            unreachable!("Roman buildings have their own meshes")
        }
        Kind::RicePaddy
        | Kind::RiceKitchen
        | Kind::TeaGarden
        | Kind::TeaHouse
        | Kind::PaperWorkshop
        | Kind::Temple
        | Kind::Machiya
        | Kind::Longhouse
        | Kind::Fishery
        | Kind::Smokehouse
        | Kind::Woodlot
        | Kind::Boatyard
        | Kind::Weaver
        | Kind::FeastHall => unreachable!("buildings from other regions have their own meshes"),
    }
    for tier in 2..=level {
        append_upgrade(&mut b, kind, tier);
    }
    b.mesh()
}

pub fn append_upgrade(b: &mut Batch, kind: Kind, tier: u8) {
    if tier >= 5 {
        super::late_upgrades::add(b, kind, tier, false);
        return;
    }

    let wood = [155, 111, 75];
    match (kind, tier) {
        (Kind::HerbGarden, _) => garden_upgrade(b, tier, false),
        (Kind::House, 2) => {
            b.slab(v(0.0, 1.02, 1.10), v(1.96, 0.13, 0.62), IVORY);
            rail(b, v(0.0, 1.08, 1.38), 1.86, TEAL);
            for x in [-0.72, 0.72] {
                flowerpot(b, v(x, 1.10, 1.18), 0.52, [214, 120, 139]);
            }
        }
        (Kind::House, 3) => {
            b.slab(v(0.91, 0.56, -0.51), v(0.65, 1.07, 0.88), IVORY);
            b.slab(v(0.91, 1.11, -0.51), v(0.79, 0.12, 1.00), BLUE);
            b.slab(v(0.94, 0.74, -0.051), v(0.25, 0.32, 0.060), [61, 78, 78]);
            b.slab(v(0.94, 0.553, 0.001), v(0.32, 0.063, 0.15), IVORY);
            for n in 0..5 {
                let height = 0.065 + n as f32 * 0.116;
                b.slab(
                    v(0.94, height * 0.5, 0.83 - n as f32 * 0.153),
                    v(0.39, height, 0.19),
                    IVORY,
                );
            }
            flowerpot(b, v(1.18, 0.07, 0.77), 0.51, [218, 122, 141]);
        }
        (Kind::House, 4) => {
            awning(b, 0.45, 1.75, 2.17, BLUE);
            b.slab(v(0.0, 0.103, 0.91), v(2.51, 0.13, 0.61), IVORY);
            b.slab(v(0.0, 0.03, 1.15), v(1.12, 0.09, 0.22), CREAM);
            for x in [-1.13, 1.13] {
                cypress(b, v(x, 0.08, 0.45), 0.58);
            }
        }
        (Kind::Well, 2) => {
            canopy(b, v(0.0, 0.0, 0.01), 1.83, 1.32, 1.60, TEAL, true);
            b.frustum(v(-0.8, 1.03, 0.01), 0.18, 0.18, 0.12, 10, OAK);
        }
        (Kind::Well, 3) => {
            b.slab(v(0.0, 0.21, 0.79), v(1.31, 0.21, 0.49), MARBLE);
            b.slab(v(0.0, 0.319, 0.79), v(1.14, 0.016, 0.36), [66, 157, 167]);
            b.beam(v(0.0, 0.49, 0.41), v(0.0, 0.487, 0.75), 0.096, CREAM);
            b.slab(v(0.0, 0.39, 0.78), v(0.065, 0.16, 0.046), [103, 183, 182]);
        }
        (Kind::Well, 4) => {
            b.slab(v(0.0, 1.78, -0.28), v(0.98, 0.13, 0.68), MARBLE);
            b.frustum(v(0.0, 1.83, -0.28), 0.46, 0.50, 0.32, 10, BLUE);
            b.frustum(v(0.0, 2.15, -0.28), 0.54, 0.49, 0.12, 10, IVORY);
            b.frustum(v(0.0, 2.27, -0.28), 0.38, 0.08, 0.21, 10, TEAL);
            for x in [-0.97, 0.97] {
                b.slab(v(x, 0.40, -0.36), v(0.17, 0.79, 0.22), CREAM);
            }
        }
        (Kind::Grove, 2) => {
            olive(b, v(0.0, 0.073, -0.68), 1.23);
            b.slab(v(0.0, 0.15, -0.76), v(2.42, 0.25, 0.19), CREAM);
        }
        (Kind::Grove, 3) => {
            canopy(b, v(0.0, 0.02, 0.63), 2.50, 0.91, 1.15, HERB, false);
            bench(b, v(0.0, 0.04, 0.76), 1.53);
        }
        (Kind::Grove, 4) => {
            for i in 0..7 {
                let x = i as f32 * 0.33 - 0.99;
                b.gem(
                    v(x, 1.32 + (x * 6.0).sin() * 0.09, 0.95),
                    0.26,
                    0.19,
                    0.29,
                    x + 0.9,
                    shift(HERB, 8 + (i * 9 % 26) as i16),
                );
                b.gem(v(x, 1.16, 1.11), 0.062, 0.073, 0.052, x, [128, 79, 121]);
            }
            b.slab(v(0.0, 0.08, 1.06), v(2.30, 0.15, 0.22), IVORY);
            amphora(b, v(1.22, 0.02, 0.85), 0.92, BLUE);
        }
        (Kind::Bakery, 2) => {
            oven(b, v(0.83, 0.07, 0.81), TERRACOTTA);
            b.slab(v(0.86, 0.16, 0.87), v(0.90, 0.18, 0.70), CREAM);
        }
        (Kind::Bakery, 3) => bake_shelf(b, v(-0.55, 0.125, 0.94), TEAL),
        (Kind::Bakery, 4) => {
            b.slab(v(-0.62, 1.95, -0.39), v(0.65, 0.56, 0.69), IVORY);
            b.slab(v(-0.62, 2.27, -0.39), v(0.83, 0.16, 0.84), CREAM);
            b.slab(v(-0.62, 2.48, -0.41), v(0.44, 0.34, 0.47), TERRACOTTA);
            b.frustum(v(-0.62, 2.58, -0.41), 0.30, 0.12, 0.25, 10, MARBLE);
        }
        (Kind::Potter, 2) => oven(b, v(0.78, 0.09, 0.81), [151, 90, 72]),
        (Kind::Potter, 3) => {
            shelf(b, v(-0.72, 0.12, 0.88), TEAL);
        }
        (Kind::Potter, 4) => {
            canopy(
                b,
                v(0.0, 1.93, -0.24),
                1.58,
                0.96,
                0.56,
                [198, 99, 77],
                true,
            );
            b.slab(v(0.0, 1.97, -0.27), v(1.69, 0.14, 1.02), CREAM);
            for i in 0..4 {
                amphora(
                    b,
                    v(-0.54 + i as f32 * 0.36, 2.05, -0.16),
                    0.69,
                    if i % 2 == 0 { BLUE } else { TERRACOTTA },
                );
            }
        }
        (Kind::Stoa, 2) => {
            for x in [-0.81, 0.81] {
                bench(b, v(x, 0.21, 0.44), 0.76);
            }
            b.slab(v(0.0, 0.085, 1.02), v(2.71, 0.16, 0.35), IVORY);
        }
        (Kind::Stoa, 3) => {
            for x in [-1.15, 1.15] {
                column(b, v(x, 0.18, 0.97), 1.51, 0.111);
            }
            b.slab(v(0.0, 1.79, 1.0), v(2.64, 0.22, 0.44), MARBLE);
            b.slab(v(0.0, 1.92, 1.01), v(2.68, 0.066, 0.51), BLUE);
        }
        (Kind::Stoa, 4) | (Kind::Shrine, 3) => {
            let y = if kind == Kind::Stoa { 1.96 } else { 1.98 };
            b.face(
                &[v(-1.2, y, 0.93), v(1.2, y, 0.93), v(0.0, y + 0.54, 0.93)],
                MARBLE,
            );
            for side in [-1.0, 1.0] {
                b.beam(v(1.22 * side, y, 0.96), v(0.0, y + 0.56, 0.96), 0.076, BLUE);
            }
            b.gem(v(0.0, y + 0.22, 0.961), 0.17, 0.14, 0.039, 0.29, GOLD);
        }
        (Kind::Taverna, 2) => {
            for x in [-0.82, 0.82] {
                bench(b, v(x, 0.10, 1.02), 0.73);
            }
            b.slab(v(0.0, 0.097, 0.90), v(2.72, 0.19, 0.61), CREAM);
        }
        (Kind::Taverna, 3) => {
            canopy(b, v(0.0, 0.08, 0.97), 2.54, 0.64, 1.56, BLUE, false);
        }
        (Kind::Taverna, 4) => {
            b.slab(v(0.53, 2.14, -0.37), v(0.91, 0.42, 0.80), IVORY);
            b.slab(v(0.53, 2.39, -0.37), v(1.06, 0.15, 0.86), CREAM);
            canopy(b, v(0.53, 2.40, -0.31), 1.10, 0.95, 0.51, BLUE, true);
            for x in [0.18, 0.87] {
                amphora(b, v(x, 2.50, -0.30), 0.68, TERRACOTTA);
            }
        }
        (Kind::Inn, 2) => {
            b.slab(v(0.0, 1.44, 0.74), v(2.31, 0.14, 0.55), CREAM);
            rail(b, v(0.0, 1.48, 0.99), 2.19, TEAL);
            for x in [-0.79, 0.79] {
                flowerpot(b, v(x, 1.51, 0.85), 0.60, [223, 130, 154]);
            }
        }
        (Kind::Inn, 3) => {
            canopy(b, v(0.0, 1.48, 0.78), 2.32, 0.68, 0.93, BLUE, false);
            b.slab(v(0.0, 2.39, 0.79), v(2.39, 0.072, 0.73), GOLD);
        }
        (Kind::Inn, 4) => rooftop(b, v(0.10, 2.59, -0.27), 1.30, BLUE),
        (Kind::Storehouse, 2) => {
            for x in [-0.99, 0.98] {
                crate_box(b, v(x, 0.12, 0.91), v(0.50, 0.41, 0.41), wood);
            }
            awning(b, 0.62, 1.24, 2.46, BLUE);
        }
        (Kind::Storehouse, 3) => crane(b, v(1.01, 0.15, 0.10), 2.16),
        (Kind::Storehouse, 4) => rooftop(b, v(-0.40, 1.76, -0.21), 1.46, [141, 107, 78]),
        (Kind::Press, 2) => canopy(b, v(0.0, 0.10, 0.09), 2.48, 1.70, 1.50, HERB, true),
        (Kind::Press, 3) => {
            b.slab(v(0.0, 0.19, 0.92), v(2.43, 0.28, 0.44), CREAM);
            for x in [-0.83, -0.29, 0.29, 0.83] {
                amphora(
                    b,
                    v(x, 0.33, 0.94),
                    0.95,
                    if x.abs() > 0.5 { TERRACOTTA } else { GOLD },
                );
            }
        }
        (Kind::Press, 4) => {
            b.frustum(v(0.0, 1.63, -0.3), 0.49, 0.55, 0.54, 11, CREAM);
            b.frustum(v(0.0, 2.17, -0.3), 0.56, 0.49, 0.11, 11, GOLD);
            b.frustum(v(0.0, 2.28, -0.3), 0.49, 0.12, 0.22, 11, HERB);
        }
        (Kind::Shrine, 2) => {
            b.slab(v(0.0, 0.08, 1.08), v(2.35, 0.13, 0.29), CREAM);
            b.slab(v(0.0, 0.19, 0.97), v(2.13, 0.13, 0.31), IVORY);
            b.slab(v(0.0, 0.47, 0.90), v(0.69, 0.44, 0.39), MARBLE);
            b.slab(v(0.0, 0.68, 0.91), v(0.83, 0.08, 0.43), GOLD);
        }
        (Kind::Shrine, 4) => {
            for x in [-1.23, 1.23] {
                column(b, v(x, 0.09, 0.94), 0.79, 0.12);
                b.frustum(v(x, 0.87, 0.94), 0.083, 0.24, 0.16, 9, OAK);
                b.gem(v(x, 1.13, 0.94), 0.12, 0.16, 0.13, x, GOLD);
                b.gem(
                    v(x - 0.034, 1.21, 0.94),
                    0.048,
                    0.13,
                    0.065,
                    x,
                    [234, 130, 62],
                );
            }
        }
        _ => {}
    }
}

pub(super) fn oven(b: &mut Batch, p: Vec3, brick: Tint) {
    b.frustum(p, 0.40, 0.43, 0.35, 12, brick);
    b.frustum(p + Vec3::Y * 0.35, 0.43, 0.31, 0.26, 12, shift(brick, 12));
    b.frustum(p + Vec3::Y * 0.61, 0.31, 0.085, 0.18, 12, shift(brick, 21));
    b.slab(p + v(0.0, 0.29, 0.395), v(0.30, 0.26, 0.047), [69, 48, 44]);
    b.gem(
        p + v(0.019, 0.265, 0.427),
        0.069,
        0.083,
        0.016,
        0.3,
        [246, 155, 72],
    );
    b.slab(p + v(0.0, 0.425, 0.403), v(0.36, 0.067, 0.083), CREAM);
}

pub(super) fn bake_shelf(b: &mut Batch, p: Vec3, tint: Tint) {
    for x in [-0.46, 0.46] {
        b.slab(p + v(x, 0.44, 0.0), v(0.064, 0.88, 0.36), tint);
    }
    for y in [0.40, 0.74] {
        b.slab(p + v(0.0, y, 0.0), v(0.99, 0.067, 0.37), OAK);
        for i in 0..4 {
            b.gem(
                p + v(i as f32 * 0.215 - 0.32, y + 0.12, 0.03),
                0.097,
                0.087,
                0.12,
                i as f32 * 0.11,
                shift(GOLD, i * 6),
            );
        }
    }
}

pub(super) fn shelf(b: &mut Batch, p: Vec3, tint: Tint) {
    for x in [-0.43, 0.43] {
        b.slab(p + v(x, 0.49, 0.0), v(0.064, 0.98, 0.34), OAK);
    }
    for y in [0.33, 0.72] {
        b.slab(p + v(0.0, y, 0.0), v(0.95, 0.064, 0.35), OAK);
        for i in 0..3 {
            amphora(
                b,
                p + v(i as f32 * 0.25 - 0.25, y + 0.034, 0.056),
                0.53,
                if i % 2 == 0 { tint } else { TERRACOTTA },
            );
        }
    }
}

fn rooftop(b: &mut Batch, p: Vec3, width: f32, tile: Tint) {
    b.slab(p + v(0.0, 0.23, 0.0), v(width, 0.57, 0.99), IVORY);
    b.slab(p + v(0.0, 0.52, 0.0), v(width + 0.12, 0.083, 1.07), CREAM);
    window(b, p.x, p.y + 0.29, p.z + 0.51, TEAL);
    b.slab(p + v(0.0, 0.58, 0.0), v(width + 0.17, 0.065, 1.11), tile);
    canopy(b, p + Vec3::Y * 0.61, width, 0.93, 0.25, tile, true);
}

fn stone_base(b: &mut Batch, x: f32, z: f32) {
    b.slab(v(0.0, 0.045, 0.0), v(x + 0.13, 0.10, z + 0.13), CREAM);
    b.slab(v(0.0, 0.117, 0.0), v(x, 0.054, z), IVORY);
}

fn yard(b: &mut Batch) {
    b.slab(v(0.0, 0.04, 0.85), v(1.56, 0.08, 0.42), IVORY);
    amphora(b, v(-1.24, 0.0, 0.68), 0.78, TERRACOTTA);
    flowerpot(b, v(1.24, 0.0, 0.68), 0.69, [206, 102, 131]);
}

pub(super) fn roof(b: &mut Batch, x: f32, z: f32, y: f32, h: f32, color: Tint) {
    let (left, right, back, front) = (-x / 2.0, x / 2.0, -z / 2.0, z / 2.0);
    let a = v(left, y, front);
    let c = v(right, y, front);
    let ridge = v(0.0, y + h, front);
    let a_back = v(left, y, back);
    let c_back = v(right, y, back);
    let ridge_back = v(0.0, y + h, back);
    b.face(&[a, c, ridge], shift(color, -16));
    b.face(&[c_back, a_back, ridge_back], shift(color, -16));
    b.face(&[a_back, a, ridge, ridge_back], color);
    b.face(&[c, c_back, ridge_back, ridge], shift(color, 10));
    b.beam(
        ridge + Vec3::Y * 0.025,
        ridge_back + Vec3::Y * 0.025,
        0.085,
        shift(color, 15),
    );
    for side in [-1.0, 1.0] {
        b.beam(
            v(side * left, y, back),
            v(side * left, y, front),
            0.095,
            shift(color, -22),
        );
        for i in 0..(z / 0.16) as usize {
            let zz = back + 0.07 + i as f32 * 0.16;
            b.beam(
                v(side * right, y + 0.025, zz),
                v(0.0, y + h + 0.025, zz),
                0.043,
                shift(color, ((i * 13) % 22) as i16 - 7),
            );
        }
    }
}

fn window(b: &mut Batch, x: f32, y: f32, z: f32, color: Tint) {
    b.slab(v(x, y, z), v(0.34, 0.44, 0.034), shift(TEAL, -31));
    for side in [-1.0, 1.0] {
        b.slab(
            v(x + side * 0.184, y, z + 0.025),
            v(0.093, 0.46, 0.06),
            color,
        );
    }
    b.slab(v(x, y - 0.23, z + 0.047), v(0.45, 0.059, 0.13), MARBLE);
    b.slab(v(x, y + 0.23, z + 0.018), v(0.40, 0.062, 0.066), MARBLE);
}

pub(super) fn awning(b: &mut Batch, z: f32, y: f32, width: f32, color: Tint) {
    for x in [-width * 0.46, width * 0.46] {
        b.slab(v(x, y * 0.49, z + 0.54), v(0.045, y * 0.98, 0.045), OAK);
        b.beam(
            v(x, y + 0.08, z - 0.05),
            v(x, y - 0.10, z + 0.60),
            0.04,
            OAK,
        );
    }
    let w = width / 9.0;
    for i in 0..9 {
        let a = -width * 0.5 + i as f32 * w;
        let paint = if i % 2 == 0 { color } else { MARBLE };
        b.face(
            &[
                v(a, y + 0.11, z),
                v(a, y - 0.09, z + 0.62),
                v(a + w, y - 0.09, z + 0.62),
                v(a + w, y + 0.11, z),
            ],
            paint,
        );
        b.face(
            &[
                v(a, y - 0.09, z + 0.62),
                v(a, y - 0.24, z + 0.62),
                v(a + w * 0.5, y - 0.27, z + 0.62),
                v(a + w, y - 0.23, z + 0.62),
                v(a + w, y - 0.09, z + 0.62),
            ],
            shift(paint, -10),
        );
    }
}

pub(super) fn bakehouse(b: &mut Batch, roman: bool) {
    let plaster = if roman {
        [233, 168, 109]
    } else {
        [249, 223, 167]
    };
    let tiles = if roman { [170, 74, 53] } else { [226, 141, 72] };
    let brick = if roman { [162, 85, 68] } else { [187, 104, 65] };
    stone_base(b, 2.58, 1.91);
    b.slab(v(-0.1, 0.73, -0.19), v(2.16, 1.33, 1.39), plaster);
    roof(b, 2.32, 1.82, 1.44, 0.51, tiles);
    b.slab(v(0.65, 1.47, -0.51), v(0.48, 1.70, 0.39), brick);
    for y in [0.93, 1.54, 2.23] {
        b.slab(v(0.65, y, -0.51), v(0.56, 0.083, 0.45), shift(brick, 26));
    }
    b.slab(v(0.65, 2.29, -0.51), v(0.25, 0.028, 0.19), [79, 48, 41]);
    b.slab(v(0.36, 0.57, 0.525), v(0.49, 0.92, 0.075), shift(OAK, -30));
    b.slab(v(0.36, 1.04, 0.563), v(0.58, 0.13, 0.13), MARBLE);
    b.slab(v(-0.56, 0.75, 0.528), v(0.93, 0.53, 0.088), [88, 60, 42]);
    b.slab(v(-0.56, 0.48, 0.72), v(1.06, 0.12, 0.48), shift(OAK, 13));
    for i in 0..5 {
        let x = -0.95 + i as f32 * 0.18;
        let z = 0.73 + (i % 2) as f32 * 0.093;
        b.gem(
            v(x, 0.573, z),
            0.113,
            0.066,
            0.081,
            i as f32 * 0.66,
            shift(GOLD, -7 + i * 6),
        );
        b.beam(
            v(x - 0.042, 0.621, z - 0.037),
            v(x + 0.015, 0.629, z + 0.028),
            0.019,
            CREAM,
        );
    }
    awning(
        b,
        0.52,
        1.27,
        1.95,
        if roman { [169, 65, 57] } else { [226, 169, 68] },
    );
    b.block(
        v(-0.79, 1.40, 0.71),
        v(0.39, 0.23, 0.042),
        Quat::from_rotation_z(-0.09),
        OAK,
    );
    b.gem(v(-0.79, 1.40, 0.749), 0.155, 0.065, 0.022, 0.25, GOLD);
    if roman {
        for x in [-1.04, 0.85] {
            b.slab(v(x, 0.74, 0.539), v(0.13, 1.23, 0.15), brick);
        }
    }
}

pub(super) fn tavern(b: &mut Batch, roman: bool) {
    let plaster = if roman { [212, 166, 124] } else { IVORY };
    let wood = if roman { [111, 76, 64] } else { [57, 105, 113] };
    let cloth = if roman { [122, 39, 78] } else { [53, 128, 149] };
    let leaf = if roman { [100, 139, 76] } else { [79, 145, 91] };
    stone_base(b, 2.66, 2.00);
    b.slab(v(0.0, 0.84, -0.71), v(2.40, 1.51, 0.40), plaster);
    b.slab(v(0.0, 1.66, -0.55), v(2.49, 0.15, 0.79), MARBLE);
    b.slab(v(0.0, 1.77, -0.55), v(2.55, 0.09, 0.86), cloth);
    b.slab(v(0.0, 0.80, -0.47), v(1.44, 1.03, 0.115), shift(wood, -16));
    for x in [-1.07, 1.07] {
        b.slab(v(x, 0.84, 0.70), v(0.119, 1.46, 0.122), wood);
        b.slab(v(x, 0.19, 0.70), v(0.24, 0.16, 0.24), plaster);
        b.beam(v(x, 1.55, 0.91), v(x, 1.55, -0.66), 0.11, wood);
    }
    for i in 0..9 {
        let z = -0.57 + i as f32 * 0.173;
        b.slab(
            v(0.0, 1.57, z),
            v(2.43, 0.071, 0.074),
            shift(wood, i * 3 - 12),
        );
        if i < 7 {
            b.slab(
                v(-0.11, 1.62 + (i % 2) as f32 * 0.018, z),
                v(1.83, 0.06, 0.18),
                shift(cloth, i * 4 - 8),
            );
        }
    }
    for x in [-0.63, 0.61] {
        b.frustum(v(x, 0.12, 0.61), 0.068, 0.047, 0.39, 8, wood);
        b.frustum(v(x, 0.51, 0.61), 0.30, 0.30, 0.048, 10, cloth);
        b.frustum(v(x - 0.05, 0.56, 0.67), 0.13, 0.16, 0.055, 9, MARBLE);
        for i in 0..4 {
            b.gem(
                v(x - 0.14 + i as f32 * 0.054, 0.635, 0.67),
                0.031,
                0.03,
                0.029,
                i as f32,
                shift([192, 150, 66], i * 10),
            );
        }
        bench(b, v(x, 0.12, 0.99), 0.41);
    }
    for i in 0..7 {
        let at = v(
            1.05 - i as f32 * 0.23,
            1.58 + 0.045 * (i as f32).sin(),
            0.73,
        );
        b.gem(at, 0.31, 0.105, 0.144, i as f32, shift(leaf, i * 5 - 16));
        if i % 2 == 0 {
            b.gem(
                at + v(0.085, -0.115, 0.09),
                0.08,
                0.10,
                0.073,
                0.3,
                [135, 74, 120],
            );
        }
    }
    amphora(b, v(-1.04, 0.125, 0.32), 1.05, cloth);
}

pub(super) fn herb_garden(b: &mut Batch, roman: bool) {
    let border = if roman {
        [168, 108, 80]
    } else {
        [198, 168, 119]
    };
    b.slab(v(0.0, 0.05, 0.0), v(2.56, 0.095, 1.84), shift(border, 15));
    for (i, x) in [-0.79, 0.0, 0.79].into_iter().enumerate() {
        b.slab(v(x, 0.12, 0.01), v(0.65, 0.13, 1.43), border);
        b.slab(v(x, 0.191, 0.01), v(0.57, 0.03, 1.34), [112, 77, 53]);
        for j in 0..7 {
            let at = v(
                x + (j % 2) as f32 * 0.18 - 0.08,
                0.32,
                -0.52 + j as f32 * 0.18,
            );
            let color = [[104, 158, 113], [84, 139, 70], [138, 142, 101]][i];
            b.gem(
                at,
                0.16,
                0.16,
                0.13,
                j as f32 * 0.73,
                shift(color, j * 4 - 8),
            );
            if i == 2 {
                b.gem(
                    at + v(0.037, 0.148, -0.028),
                    0.063,
                    0.058,
                    0.061,
                    0.2,
                    [170, 124, 192],
                );
            }
        }
    }
    amphora(
        b,
        v(-1.09, 0.103, 0.84),
        0.71,
        if roman { [158, 76, 62] } else { TEAL },
    );
    b.beam(v(1.09, 0.14, 0.77), v(1.09, 0.63, 0.77), 0.048, OAK);
    b.block(
        v(1.09, 0.56, 0.77),
        v(0.29, 0.16, 0.049),
        Quat::from_rotation_z(0.13),
        shift(OAK, 26),
    );
}

pub(super) fn garden_upgrade(b: &mut Batch, tier: u8, roman: bool) {
    let stone = if roman { [187, 135, 99] } else { CREAM };
    match tier {
        2 => {
            b.slab(v(0.0, 0.25, -0.25), v(0.70, 0.21, 0.75), stone);
            b.slab(v(0.0, 0.376, -0.25), v(0.61, 0.045, 0.67), [113, 85, 59]);
            flowerpot(b, v(-0.16, 0.39, -0.14), 0.72, [228, 164, 159]);
        }
        3 => {
            for x in [-1.07, 1.07] {
                b.slab(v(x, 0.72, -0.65), v(0.082, 1.34, 0.084), OAK);
            }
            for i in 0..8 {
                let x = -1.02 + i as f32 * 0.29;
                b.beam(v(x, 1.33, -0.87), v(x, 1.33, -0.31), 0.062, OAK);
                b.gem(
                    v(x, 1.38, -0.46),
                    0.24,
                    0.14,
                    0.24,
                    i as f32 * 0.4,
                    shift([103, 153, 87], i * 5 - 16),
                );
            }
            b.beam(v(-1.22, 1.33, -0.65), v(1.22, 1.33, -0.65), 0.085, OAK);
        }
        _ => {
            awning(
                b,
                0.15,
                1.05,
                1.27,
                if roman { [150, 65, 68] } else { [221, 173, 83] },
            );
            bake_shelf(b, v(0.55, 0.09, 0.39), stone);
        }
    }
}

fn shop(b: &mut Batch, kind: Kind) {
    let (color, roof_color, fabric, height) = match kind {
        Kind::Bakery => (IVORY, TERRACOTTA, TEAL, 1.55),
        Kind::Potter => ([223, 178, 145], [77, 125, 129], [201, 107, 85], 1.58),
        Kind::Taverna => ([238, 201, 132], TERRACOTTA, BLUE, 1.69),
        _ => (MARBLE, BLUE, [202, 139, 76], 2.12),
    };
    let back = -0.30;
    let z = 0.35;
    stone_base(b, 2.46, 1.69);
    b.slab(
        v(0.0, height / 2.0 + 0.12, back),
        v(2.29, height, 1.30),
        color,
    );
    for i in 0..3 {
        b.slab(
            v(0.0, 0.18 + i as f32 * 0.11, z + 0.016),
            v(2.3, 0.043, 0.05),
            shift(CREAM, (i * 4) as i16),
        );
    }
    b.slab(
        v(0.0, height + 0.08, back),
        v(2.36, 0.105, 1.36),
        shift(color, -25),
    );
    b.slab(v(0.0, height + 0.14, back), v(2.39, 0.047, 1.40), MARBLE);
    roof(b, 2.55, 1.97, height + 0.16, 0.47, roof_color);
    b.slab(
        v(0.0, 0.62, z + 0.021),
        v(0.55, 0.97, 0.055),
        shift(OAK, -10),
    );
    b.slab(v(0.0, 1.11, z + 0.04), v(0.68, 0.053, 0.079), MARBLE);
    window(b, -0.76, 0.86, z, fabric);
    window(b, 0.76, 0.86, z, fabric);
    if kind == Kind::Inn {
        window(b, -0.59, 1.69, z, TEAL);
        window(b, 0.59, 1.69, z, TEAL);
        b.slab(v(0.0, 1.42, z + 0.17), v(1.68, 0.087, 0.45), OAK);
        for i in 0..8 {
            b.slab(
                v((i as f32 - 3.5) * 0.21, 1.55, z + 0.38),
                v(0.035, 0.26, 0.035),
                TEAL,
            );
        }
        b.slab(v(0.0, 1.68, z + 0.38), v(1.60, 0.065, 0.06), TEAL);
        flowerpot(b, v(0.89, 0.14, 0.81), 0.86, [209, 107, 146]);
        amphora(b, v(-0.9, 0.16, 0.76), 0.84, TERRACOTTA);
    } else {
        awning(b, z + 0.06, 1.28, 2.24, fabric);
        b.slab(v(-0.73, 0.53, 0.77), v(0.68, 0.14, 0.35), OAK);
        for i in 0..3 {
            let x = -0.95 + 0.20 * i as f32;
            if kind == Kind::Potter {
                amphora(
                    b,
                    v(x, 0.60, 0.78),
                    0.46,
                    if i == 1 { TEAL } else { TERRACOTTA },
                );
            } else {
                b.gem(v(x, 0.67, 0.79), 0.11, 0.065, 0.075, 0.25, GOLD);
            }
        }
        if kind == Kind::Taverna {
            b.frustum(v(0.81, 0.12, 0.83), 0.056, 0.049, 0.44, 8, OAK);
            b.frustum(v(0.81, 0.56, 0.83), 0.27, 0.27, 0.065, 10, TEAL);
            amphora(b, v(0.81, 0.63, 0.83), 0.41, MARBLE);
        } else {
            amphora(b, v(0.90, 0.15, 0.77), 0.82, TERRACOTTA);
        }
        if kind == Kind::Bakery || kind == Kind::Potter {
            b.slab(
                v(0.78, height + 0.33, -0.63),
                v(0.26, 0.54, 0.26),
                shift(color, -31),
            );
            b.slab(
                v(0.78, height + 0.64, -0.63),
                v(0.33, 0.08, 0.33),
                TERRACOTTA,
            );
        }
    }
}

pub fn amphora(b: &mut Batch, p: Vec3, s: f32, color: Tint) {
    b.frustum(p, s * 0.09, s * 0.16, s * 0.18, 9, shift(color, -12));
    b.frustum(
        p + v(0.0, s * 0.18, 0.0),
        s * 0.16,
        s * 0.08,
        s * 0.18,
        9,
        color,
    );
    b.frustum(
        p + v(0.0, s * 0.36, 0.0),
        s * 0.074,
        s * 0.086,
        s * 0.075,
        9,
        shift(color, 13),
    );
    for sign in [-1.0, 1.0] {
        b.beam(
            p + v(sign * s * 0.079, s * 0.36, 0.0),
            p + v(sign * s * 0.176, s * 0.29, 0.0),
            s * 0.036,
            color,
        );
        b.beam(
            p + v(sign * s * 0.176, s * 0.29, 0.0),
            p + v(sign * s * 0.145, s * 0.17, 0.0),
            s * 0.031,
            color,
        );
    }
}

pub fn flowerpot(b: &mut Batch, p: Vec3, s: f32, bloom: Tint) {
    b.frustum(p, s * 0.13, s * 0.18, s * 0.20, 8, TERRACOTTA);
    b.gem(
        p + v(0.0, s * 0.26, 0.0),
        s * 0.24,
        s * 0.17,
        s * 0.21,
        0.0,
        HERB,
    );
    for (x, y, z) in [
        (0.07, 0.38, 0.11),
        (-0.12, 0.38, 0.08),
        (0.10, 0.43, -0.03),
        (-0.04, 0.49, 0.0),
    ] {
        b.gem(
            p + v(x, y, z) * s,
            s * 0.073,
            s * 0.054,
            s * 0.07,
            0.33,
            bloom,
        );
    }
}

pub fn column(b: &mut Batch, p: Vec3, h: f32, radius: f32) {
    b.slab(
        p + v(0.0, 0.047, 0.0),
        v(radius * 2.65, 0.09, radius * 2.65),
        MARBLE,
    );
    b.frustum(
        p + v(0.0, 0.08, 0.0),
        radius * 1.13,
        radius,
        0.11,
        10,
        IVORY,
    );
    b.frustum(
        p + v(0.0, 0.16, 0.0),
        radius,
        radius * 0.86,
        h - 0.29,
        12,
        MARBLE,
    );
    b.frustum(
        p + v(0.0, h - 0.14, 0.0),
        radius * 0.94,
        radius * 1.22,
        0.11,
        10,
        IVORY,
    );
    b.slab(
        p + v(0.0, h, 0.0),
        v(radius * 2.58, 0.085, radius * 2.58),
        MARBLE,
    );
}

fn well(b: &mut Batch) {
    for i in 0..14 {
        let a = i as f32 / 14.0 * std::f32::consts::TAU;
        let (s, c) = a.sin_cos();
        b.block(
            v(c * 0.84, 0.053, s * 0.84),
            v(0.20, 0.084, 0.26),
            Quat::from_rotation_y(-a),
            if i % 2 == 0 { TEAL } else { IVORY },
        );
    }
    b.frustum(Vec3::ZERO, 0.65, 0.67, 0.17, 12, CREAM);
    for i in 0..12 {
        let a = i as f32 / 12.0 * std::f32::consts::TAU;
        let (s, c) = a.sin_cos();
        b.block(
            v(c * 0.52, 0.28, s * 0.52),
            v(0.21, 0.26, 0.285),
            Quat::from_rotation_y(-a),
            shift(MARBLE, (i * 5 % 17) as i16 - 8),
        );
    }
    b.frustum(v(0.0, 0.11, 0.0), 0.46, 0.46, 0.11, 12, [72, 163, 170]);
    for side in [-1.0, 1.0] {
        column(b, v(0.56 * side, 0.14, 0.0), 1.1, 0.083);
    }
    b.slab(v(0.0, 1.27, 0.0), v(1.41, 0.136, 0.19), OAK);
    b.beam(v(0.0, 1.24, 0.0), v(0.0, 0.53, 0.0), 0.024, CREAM);
    amphora(b, v(0.0, 0.43, 0.0), 0.61, OAK);
    flowerpot(b, v(-0.95, 0.0, 0.40), 0.72, [240, 205, 123]);
    amphora(b, v(0.98, 0.0, 0.36), 0.92, TERRACOTTA);
}

fn stoa(b: &mut Batch) {
    stone_base(b, 2.64, 1.72);
    b.slab(v(0.0, 0.17, 0.03), v(2.49, 0.09, 1.65), MARBLE);
    for x in [-1.08, -0.36, 0.36, 1.08] {
        for z in [-0.58, 0.57] {
            column(b, v(x, 0.22, z), 1.43, 0.102);
        }
    }
    b.slab(v(0.0, 1.74, 0.0), v(2.53, 0.18, 1.38), MARBLE);
    b.slab(v(0.0, 1.79, 0.70), v(2.54, 0.096, 0.04), BLUE);
    for i in 0..15 {
        b.slab(
            v((i as f32 - 7.0) * 0.16, 1.79, 0.733),
            v(0.07, 0.058, 0.032),
            if i % 3 == 0 { GOLD } else { IVORY },
        );
    }
    roof(b, 2.68, 1.76, 1.85, 0.41, TERRACOTTA);
    b.slab(v(0.0, 0.47, -0.40), v(1.63, 0.13, 0.29), OAK);
    for x in [-0.65, 0.65] {
        b.slab(v(x, 0.32, -0.40), v(0.08, 0.30, 0.22), CREAM);
    }
    flowerpot(b, v(-1.27, 0.02, 0.77), 0.80, [178, 105, 158]);
    flowerpot(b, v(1.27, 0.02, 0.77), 0.80, [215, 111, 136]);
}

fn grove(b: &mut Batch) {
    let earth = [155, 126, 77];
    b.slab(v(0.0, 0.049, 0.0), v(2.5, 0.08, 1.65), earth);
    for x in [-1.20, 1.20] {
        b.slab(v(x, 0.16, 0.0), v(0.09, 0.28, 1.66), CREAM);
        b.slab(v(x, 0.31, 0.0), v(0.14, 0.06, 1.72), IVORY);
    }
    olive(b, v(-0.61, 0.10, -0.24), 1.02);
    olive(b, v(0.62, 0.10, -0.34), 0.94);
    flowerpot(b, v(-0.57, 0.10, 0.62), 0.53, [231, 192, 93]);
    amphora(b, v(0.68, 0.10, 0.64), 0.59, CREAM);
}

pub fn olive(b: &mut Batch, p: Vec3, s: f32) {
    b.frustum(p, 0.12 * s, 0.06 * s, s * 0.83, 7, shift(OAK, 20));
    for (x, y, z, shade) in [
        (-0.37, 1.08, -0.08, -10),
        (0.30, 1.28, -0.07, 11),
        (0.07, 1.49, -0.07, 6),
        (0.06, 1.12, 0.25, -1),
    ] {
        b.beam(
            p + v(0.0, 0.56, 0.0) * s,
            p + v(x, y, z) * s,
            s * 0.072,
            OAK,
        );
        b.gem(
            p + v(x, y, z) * s,
            s * 0.47,
            s * 0.39,
            s * 0.43,
            x,
            shift([97, 136, 89], shade),
        );
    }
}

pub fn cypress(b: &mut Batch, p: Vec3, s: f32) {
    b.frustum(p, s * 0.10, s * 0.07, s * 0.53, 7, OAK);
    for (y, h, w, shade) in [
        (0.92, 0.76, 0.30, -4),
        (1.42, 0.83, 0.32, 2),
        (1.94, 0.60, 0.24, 11),
    ] {
        b.gem(
            p + v(0.0, y * s, 0.0),
            w * s,
            h * s,
            w * s,
            0.29,
            shift([68, 114, 85], shade),
        );
    }
}

fn storage(b: &mut Batch) {
    stone_base(b, 2.58, 1.69);
    b.slab(v(0.0, 0.71, -0.16), v(2.35, 1.25, 1.33), [226, 193, 142]);
    let front = 0.524;
    for n in 0..13 {
        let x = (n as f32 - 6.0) * 0.105;
        b.slab(
            v(x, 0.63, front),
            v(0.096, 1.02, 0.037),
            shift(OAK, (n * 7 % 37) as i16),
        );
    }
    for y in [0.38, 0.84] {
        b.slab(v(0.0, y, front + 0.03), v(1.45, 0.12, 0.067), BLUE);
    }
    roof(b, 2.55, 1.75, 1.36, 0.40, [141, 107, 78]);
    for (x, s, color) in [
        (-0.98, 0.75, BLUE),
        (-1.20, 0.89, TERRACOTTA),
        (1.15, 0.89, IVORY),
    ] {
        amphora(b, v(x, 0.14, 0.75), s, color);
    }
}

fn press(b: &mut Batch) {
    stone_base(b, 2.51, 1.61);
    for x in [-1.0, 1.0] {
        b.slab(v(x, 0.77, -0.45), v(0.11, 1.35, 0.11), OAK);
    }
    b.slab(v(0.0, 1.53, -0.40), v(2.31, 0.14, 0.96), HERB);
    b.frustum(v(0.0, 0.15, 0.33), 0.57, 0.62, 0.21, 12, CREAM);
    b.frustum(v(0.0, 0.33, 0.33), 0.48, 0.52, 0.055, 12, [153, 150, 101]);
    b.frustum(v(0.0, 0.34, 0.33), 0.09, 0.09, 0.75, 8, OAK);
    b.beam(v(-0.48, 0.72, -0.02), v(0.82, 0.72, 0.67), 0.09, OAK);
    b.gem(v(-0.38, 0.59, 0.06), 0.28, 0.25, 0.19, 0.12, MARBLE);
    amphora(b, v(0.87, 0.14, 0.64), 1.12, GOLD);
    amphora(b, v(-0.94, 0.14, 0.70), 0.87, TERRACOTTA);
    for n in 0..8 {
        b.gem(
            v(
                -0.35 + n as f32 * 0.089,
                0.40,
                0.51 + (n as f32 * 1.9).sin() * 0.10,
            ),
            0.06,
            0.04,
            0.06,
            n as f32,
            [84, 102, 54],
        );
    }
}

fn sanctuary(b: &mut Batch) {
    stone_base(b, 2.56, 1.83);
    b.slab(v(0.0, 0.25, 0.0), v(2.28, 0.23, 1.61), MARBLE);
    for x in [-0.95, -0.30, 0.30, 0.95] {
        column(b, v(x, 0.36, 0.50), 1.31, 0.090);
        column(b, v(x, 0.36, -0.50), 1.31, 0.090);
    }
    b.slab(v(0.0, 1.79, 0.0), v(2.35, 0.25, 1.47), IVORY);
    roof(b, 2.52, 1.67, 1.94, 0.42, BLUE);
    b.frustum(v(0.0, 0.38, 0.07), 0.31, 0.28, 0.53, 9, CREAM);
    b.frustum(v(0.0, 0.91, 0.07), 0.34, 0.38, 0.083, 10, GOLD);
    for x in [-0.75, 0.75] {
        flowerpot(b, v(x, 0.17, 0.93), 0.74, [227, 132, 138]);
    }
}

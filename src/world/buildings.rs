use bevy::prelude::*;

use super::{
    architecture::{gable, panel_roof, plinth, stoop, tank},
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
        Kind::House => yard(&mut b, level),
        Kind::Well => well(&mut b, level),
        Kind::Grove => grove(&mut b, level),
        Kind::Stoa => colonnade(&mut b, level, false),
        Kind::Bakery => bakehouse(&mut b, false, level),
        Kind::Taverna => tavern(&mut b, false, level),
        Kind::Potter => pottery(&mut b, level),
        Kind::Inn => inn(&mut b, level),
        Kind::HerbGarden => herb_garden(&mut b, false, level),
        Kind::Storehouse => storage(&mut b, level),
        Kind::Press => press(&mut b, level),
        Kind::Shrine => colonnade(&mut b, level, true),
        Kind::Apiary | Kind::Dyer | Kind::FerryOffice => {
            super::market_buildings::greek(&mut b, kind, level)
        }
        _ => unreachable!("buildings from other regions have their own meshes"),
    }
    b.mesh()
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

fn stone_base(b: &mut Batch, x: f32, z: f32) {
    b.slab(v(0.0, 0.045, 0.0), v(x + 0.13, 0.10, z + 0.13), CREAM);
    b.slab(v(0.0, 0.117, 0.0), v(x, 0.054, z), IVORY);
}

pub(super) fn house_transform(level: u8) -> Transform {
    let x = if level < 3 { 0.0 } else { -0.28 };
    let scale = if level < 3 { 0.73 } else { 0.67 };
    Transform::from_xyz(x, 0.05, -0.30).with_scale(Vec3::splat(scale))
}

fn yard(b: &mut Batch, level: u8) {
    plinth(
        b,
        v(0.0, 0.0, -0.025),
        2.56 + f32::from(level) * 0.034,
        2.22,
        0.10,
        IVORY,
    );
    stoop(
        b,
        v(-0.14, 0.0, 0.92),
        if level < 4 { 0.94 } else { 1.78 },
        0.10,
        0.28,
        2,
        IVORY,
    );
    amphora(b, v(-1.24, 0.10, 0.72), 0.84, TERRACOTTA);
    flowerpot(b, v(1.17, 0.10, 0.88), 0.73, [221, 107, 136]);
    if level >= 2 {
        for x in [-1.03, 1.03] {
            b.slab(v(x, 0.18, 0.87), v(0.56, 0.17, 0.36), CREAM);
            flowerpot(b, v(x, 0.268, 0.92), 0.65, [228, 145, 155]);
        }
    }
    if level >= 3 {
        let wing = if level >= 6 {
            1.49
        } else {
            0.98 + f32::from(level - 3) * 0.07
        };
        b.slab(
            v(0.87, 0.10 + wing / 2.0, -0.51),
            v(1.04, wing, 1.08),
            IVORY,
        );
        b.slab(v(0.86, 0.25, 0.039), v(0.98, 0.26, 0.038), CREAM);
        panel_roof(b, v(0.84, 0.172 + wing, -0.49), 1.16, 1.20, 0.14, BLUE);
        window(b, 0.94, wing * 0.65 + 0.17, 0.057, TEAL);
    }
    if level >= 4 {
        let width = if level < 5 { 1.74 } else { 2.47 };
        let center = -0.15;
        for side in [-1.0, 1.0] {
            column(
                b,
                v(center + side * (width / 2.0 - 0.10), 0.11, 1.03),
                1.08,
                0.064,
            );
        }
        panel_roof(b, v(center, 1.26, 0.87), width, 0.53, 0.14, BLUE);
        bench(b, v(-0.89, 0.10, 0.82), 0.56);
    }
    if level >= 5 {
        for n in 0..11 {
            let x = (n as f32 - 5.0) * 0.19;
            b.slab(
                v(x, 0.104, 0.999),
                v(0.12, 0.012, 0.14),
                if n % 2 == 0 { BLUE } else { GOLD },
            );
        }
        cypress(b, v(1.22, 0.11, -0.77), 0.49);
    }
}

pub(super) fn roof(b: &mut Batch, x: f32, z: f32, y: f32, h: f32, color: Tint) {
    super::architecture::gable(b, v(0.0, y, 0.0), x, z, h, color);
}

pub(super) fn window(b: &mut Batch, x: f32, y: f32, z: f32, color: Tint) {
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

pub(super) fn bakehouse(b: &mut Batch, roman: bool, level: u8) {
    let plaster = if roman {
        [233, 168, 109]
    } else {
        [249, 223, 167]
    };
    let tiles = if roman { [170, 74, 53] } else { [226, 141, 72] };
    let brick = if roman { [162, 85, 68] } else { [187, 104, 65] };
    stone_base(b, 2.58, 1.91);
    b.slab(v(-0.1, 0.73, -0.19), v(2.16, 1.33, 1.39), plaster);
    let loft = if level >= 6 { 0.57 } else { 0.0 };
    if level >= 6 {
        b.slab(
            v(-0.06, 1.40 + loft * 0.5, -0.18),
            v(2.21, loft, 1.43),
            plaster,
        );
        for x in [-0.71, 0.08] {
            window(b, x, 1.66, 0.547, if roman { TERRACOTTA } else { GOLD });
        }
        b.slab(v(0.0, 1.41, 0.534), v(2.26, 0.066, 0.098), CREAM);
    }
    roof(b, 2.36, 1.86, 1.42 + loft, 0.51, tiles);
    let stack = if level >= 4 { 2.63 } else { 2.25 } + loft;
    let width = if level >= 4 { 0.53 } else { 0.39 };
    b.slab(
        v(0.68, (stack + 0.42) / 2.0, -0.53),
        v(width, stack - 0.42, 0.41),
        brick,
    );
    for y in [stack - 0.40, stack - 0.06] {
        b.slab(
            v(0.68, y, -0.53),
            v(width + 0.091, 0.082, 0.48),
            shift(brick, 26),
        );
    }
    b.slab(
        v(0.68, stack + 0.003, -0.53),
        v(0.25, 0.028, 0.19),
        [79, 48, 41],
    );
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
    let cloth = if roman { [169, 65, 57] } else { [226, 169, 68] };
    if level < 5 {
        awning(b, 0.52, 1.27, 1.95, cloth);
    } else {
        for x in [-1.16, 1.16] {
            b.beam(v(x, 0.15, 1.06), v(x, 1.37, 1.06), 0.074, OAK);
        }
        panel_roof(b, v(0.0, 1.40, 0.81), 2.52, 0.60, 0.21, tiles);
    }
    if level >= 2 {
        oven(b, v(0.77, 0.15, 0.76), brick);
    }
    if level >= 3 {
        bake_shelf(b, v(-0.64, 0.15, 0.88), cloth);
    }
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

pub(super) fn tavern(b: &mut Batch, roman: bool, level: u8) {
    let plaster = if roman { [212, 166, 124] } else { IVORY };
    let wood = if roman { [111, 76, 64] } else { [57, 105, 113] };
    let cloth = if roman { [122, 39, 78] } else { [53, 128, 149] };
    let leaf = if roman { [100, 139, 76] } else { [79, 145, 91] };
    stone_base(b, 2.66, if level >= 2 { 2.16 } else { 1.97 });
    if level >= 2 {
        let count = if level >= 4 { 15 } else { 9 };
        for i in 0..count {
            let x = (i as f32 - (count - 1) as f32 * 0.5) * 0.16;
            b.slab(
                v(x, 0.15, 1.00),
                v(0.135, 0.013, 0.14),
                if i % 3 == 0 { GOLD } else { cloth },
            );
        }
    }
    b.slab(v(0.0, 0.84, -0.71), v(2.40, 1.51, 0.40), plaster);
    b.slab(v(0.0, 1.66, -0.55), v(2.49, 0.15, 0.79), MARBLE);
    if level >= 5 {
        let rear_height = if level == 5 { 0.59 } else { 0.87 };
        b.slab(
            v(0.0, 1.72 + rear_height * 0.5, -0.69),
            v(2.12, rear_height, 0.50),
            plaster,
        );
        let x = if level == 5 { 0.61 } else { 0.67 };
        for xx in [-x, x] {
            b.slab(
                v(xx, 1.79 + rear_height * 0.47, -0.433),
                v(0.33, 0.23, 0.024),
                wood,
            );
        }
        gable(
            b,
            v(0.0, 1.72 + rear_height, -0.66),
            2.32,
            0.86,
            0.32,
            cloth,
        );
    } else {
        b.slab(v(0.0, 1.77, -0.55), v(2.55, 0.09, 0.86), cloth);
    }
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
        if i < if level >= 3 { 9 } else { 7 } {
            b.slab(
                v(-0.11, 1.62 + (i % 2) as f32 * 0.018, z),
                v(if level >= 3 { 2.17 } else { 1.83 }, 0.06, 0.179),
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
    if level >= 4 {
        for x in [-1.03, 1.03] {
            b.beam(v(x, 1.58, 0.95), v(x, 1.36, 0.95), 0.023, OAK);
            b.gem(v(x, 1.30, 0.95), 0.086, 0.102, 0.078, x, GOLD);
        }
        b.slab(v(0.03, 0.60, -0.32), v(1.52, 0.12, 0.33), wood);
    }
}

pub(super) fn herb_garden(b: &mut Batch, roman: bool, level: u8) {
    let border = if roman {
        [168, 108, 80]
    } else {
        [198, 168, 119]
    };
    b.slab(v(0.0, 0.05, 0.0), v(2.56, 0.095, 1.84), shift(border, 15));
    let bed_height = if level >= 5 { 0.23 } else { 0.13 };
    let plants_y = 0.185 + bed_height;
    let count = if level >= 2 { 8 } else { 6 };
    for (i, x) in [-0.79, 0.0, 0.79].into_iter().enumerate() {
        b.slab(
            v(x, 0.055 + bed_height * 0.5, 0.01),
            v(0.65, bed_height, 1.43),
            border,
        );
        b.slab(
            v(x, 0.063 + bed_height, 0.01),
            v(0.57, 0.031, 1.34),
            [112, 77, 53],
        );
        for j in 0..count {
            let at = v(
                x + (j % 2) as f32 * 0.18 - 0.08,
                plants_y,
                -0.53 + j as f32 * 1.04 / (count - 1) as f32,
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
    if level >= 3 {
        let depth = if level >= 6 { 0.81 } else { 0.52 };
        for x in [-1.12, 1.12] {
            b.beam(v(x, 0.10, -0.79), v(x, 1.33, -0.79), 0.079, OAK);
            if level >= 6 {
                b.beam(
                    v(x, 0.10, -0.79 + depth),
                    v(x, 1.33, -0.79 + depth),
                    0.074,
                    OAK,
                );
            }
        }
        b.beam(v(-1.27, 1.32, -0.79), v(1.27, 1.32, -0.79), 0.070, OAK);
        for i in 0..9 {
            let x = -1.10 + i as f32 * 0.275;
            b.beam(v(x, 1.33, -0.89), v(x, 1.33, -0.77 + depth), 0.046, OAK);
            b.gem(
                v(x, 1.37, -0.73 + (i % 2) as f32 * 0.16),
                0.21,
                0.116,
                depth * 0.55,
                x,
                shift(HERB, 13 + (i * 12 % 33) as i16),
            );
        }
    }
    if level >= 4 {
        let crop = v(0.0, 0.083, 1.018);
        tank(
            b,
            crop,
            1.30,
            0.34,
            if level >= 5 { 0.14 } else { 0.11 },
            border,
            [96, 163, 156],
        );
    }
}

fn pottery(b: &mut Batch, level: u8) {
    let tile = [69, 125, 131];
    let brick = [186, 104, 82];
    let kiln_height = 0.70 + f32::from(level) * 0.13;
    plinth(b, Vec3::ZERO, 2.71, 2.15, 0.14, IVORY);
    let wall = 1.32 + if level >= 5 { 0.36 } else { 0.0 };
    b.slab(
        v(-0.33, 0.14 + wall / 2.0, -0.40),
        v(1.97, wall, 1.18),
        [218, 172, 145],
    );
    gable(b, v(-0.33, wall + 0.14, -0.40), 2.14, 1.34, 0.40, tile);
    b.slab(v(-0.33, 0.69, 0.22), v(0.82, 0.90, 0.063), OAK);
    b.slab(v(-0.33, 0.58, 0.41), v(1.48, 0.10, 0.48), CREAM);
    for i in 0..4 {
        amphora(
            b,
            v(-0.87 + i as f32 * 0.34, 0.63, 0.52),
            0.6 + i as f32 * 0.04,
            if i % 2 == 0 { TEAL } else { TERRACOTTA },
        );
    }
    let kiln = v(0.91, 0.14, 0.43);
    b.frustum(kiln, 0.35, 0.42, kiln_height * 0.51, 12, brick);
    b.frustum(
        kiln + Vec3::Y * (kiln_height * 0.51),
        0.42,
        0.19,
        kiln_height * 0.47,
        12,
        shift(brick, 15),
    );
    b.frustum(
        kiln + Vec3::Y * (kiln_height * 0.98),
        0.16,
        0.21,
        0.12,
        10,
        TERRACOTTA,
    );
    b.slab(kiln + v(0.0, 0.34, 0.39), v(0.24, 0.22, 0.042), OAK);
    b.gem(kiln + v(0.0, 0.32, 0.419), 0.075, 0.061, 0.018, 0.27, GOLD);
    if level >= 2 {
        shelf(b, v(-0.80, 0.15, 0.84), TEAL);
    }
    if level >= 3 {
        b.beam(v(-1.24, 0.14, 1.08), v(-1.24, 1.32, 1.08), 0.079, OAK);
        b.beam(v(0.43, 0.14, 1.08), v(0.43, 1.32, 1.08), 0.079, OAK);
        panel_roof(b, v(-0.40, 1.36, 0.65), 1.86, 0.93, 0.15, tile);
    }
    if level >= 4 {
        for i in 0..6 {
            let y = 0.98 + i as f32 * 0.032;
            b.beam(v(-1.20, y, 1.09), v(0.26, y, 1.09), 0.023, CREAM);
        }
        amphora(b, v(1.14, 0.15, 0.92), 0.84, IVORY);
    }
    if level >= 5 {
        window(b, -0.35, 1.58, 0.204, BLUE);
    }
    if level >= 6 {
        b.frustum(
            kiln + Vec3::Y * (kiln_height + 0.08),
            0.177,
            0.164,
            0.36,
            10,
            shift(TERRACOTTA, 11),
        );
        b.frustum(
            kiln + Vec3::Y * (kiln_height + 0.42),
            0.22,
            0.21,
            0.10,
            10,
            CREAM,
        );
        b.beam(v(-1.21, 1.31, 1.08), v(0.41, 1.31, 1.08), 0.063, BLUE);
    }
}

fn inn(b: &mut Batch, level: u8) {
    let floor = if level < 5 {
        2.15
    } else {
        2.26 + f32::from(level - 5) * 0.27
    };
    plinth(b, v(0.0, 0.0, -0.06), 2.72, 2.20, 0.16, MARBLE);
    b.slab(
        v(0.0, 0.16 + floor / 2.0, -0.36),
        v(2.29, floor, 1.38),
        [232, 208, 170],
    );
    gable(b, v(0.0, floor + 0.16, -0.36), 2.52, 1.61, 0.47, BLUE);
    let front = 0.34;
    b.slab(v(0.0, 0.73, front), v(0.58, 1.08, 0.07), OAK);
    b.slab(v(0.0, 1.29, front + 0.03), v(0.79, 0.092, 0.13), MARBLE);
    for x in [-0.74, 0.74] {
        window(b, x, 0.79, front, TEAL);
        window(b, x, 1.79, front, BLUE);
    }
    let width = 1.52 + f32::from(level.min(3) - 1) * 0.41;
    b.slab(v(0.0, 1.39, 0.57), v(width, 0.13, 0.55), IVORY);
    rail(b, v(0.0, 1.455, 0.826), width - 0.10, BLUE);
    for x in [-1.06, 1.06] {
        flowerpot(b, v(x, 0.16, 0.83), 0.77, [219, 106, 137]);
        if level >= 3 {
            column(b, v(x, 0.16, 0.81), 1.17, 0.077);
        }
        if level >= 4 {
            flowerpot(b, v(x * 0.88, 1.46, 0.64), 0.54, [222, 142, 157]);
        }
    }
    if level >= 4 {
        stoop(b, v(0.0, 0.0, 0.92), 1.55, 0.16, 0.30, 2, MARBLE);
    }
    if level >= 5 {
        b.slab(v(0.0, floor - 0.06, front), v(2.32, 0.082, 0.11), GOLD);
        b.slab(v(0.0, 1.85, front), v(0.38, 0.31, 0.082), TEAL);
        b.gem(v(0.0, 1.86, 0.398), 0.081, 0.096, 0.024, 0.10, GOLD);
    }
    if level >= 6 {
        panel_roof(b, v(0.0, 2.39, 0.56), 2.38, 0.61, 0.15, BLUE);
        for x in [-1.05, 1.05] {
            b.beam(v(x, 1.48, 0.83), v(x, 2.34, 0.83), 0.066, TEAL);
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

fn well(b: &mut Batch, level: u8) {
    let at = v(0.0, 0.0, if level >= 3 { -0.30 } else { 0.0 });
    let water = [68, 162, 181];
    plinth(b, Vec3::ZERO, 2.54, 2.20, 0.10, IVORY);
    b.frustum(at + v(0.0, 0.10, 0.0), 0.70, 0.68, 0.154, 14, CREAM);
    for i in 0..14 {
        let a = i as f32 / 14.0 * std::f32::consts::TAU;
        let (s, c) = a.sin_cos();
        b.block(
            at + v(c * 0.55, 0.39, s * 0.55),
            v(0.19, 0.27, 0.264),
            Quat::from_rotation_y(-a),
            shift(MARBLE, (i * 5 % 17) as i16 - 8),
        );
    }
    b.frustum(at + v(0.0, 0.254, 0.0), 0.49, 0.49, 0.059, 14, water);
    for side in [-1.0, 1.0] {
        column(b, at + v(0.71 * side, 0.10, 0.0), 1.29, 0.082);
    }
    b.slab(at + v(0.0, 1.40, 0.0), v(1.66, 0.10, 0.16), OAK);
    b.beam(at + v(0.0, 1.36, 0.0), at + v(0.0, 0.59, 0.0), 0.025, CREAM);
    amphora(b, at + v(0.0, 0.48, 0.0), 0.56, OAK);
    if level >= 2 {
        let width = if level >= 6 { 2.72 } else { 1.88 };
        let depth = if level >= 6 { 1.44 } else { 0.87 };
        if level >= 6 {
            for x in [-1.13, 1.13] {
                for z in [-0.59, 0.41] {
                    column(b, at + v(x, 0.10, z), 1.28, 0.079);
                }
            }
            b.slab(at + v(0.0, 1.50, 0.43), v(2.61, 0.14, 0.19), MARBLE);
        }
        gable(b, at + v(0.0, 1.50, 0.0), width, depth, 0.31, TEAL);
    }
    if level >= 3 {
        tank(
            b,
            v(0.0, 0.10, 0.75),
            1.72 + f32::from(level - 3) * 0.18,
            0.66,
            0.26,
            MARBLE,
            water,
        );
        let spouts: &[f32] = if level < 5 { &[0.0] } else { &[-0.55, 0.55] };
        for &x in spouts {
            b.slab(v(x, 0.36, 0.38), v(0.24, 0.52, 0.24), IVORY);
            b.beam(v(x, 0.61, 0.40), v(x, 0.58, 0.71), 0.076, GOLD);
            b.beam(v(x, 0.57, 0.71), v(x, 0.34, 0.72), 0.037, water);
        }
    }
    if level >= 4 {
        for x in [-1.02, 1.02] {
            b.frustum(v(x, 0.10, -0.48), 0.21, 0.26, 0.44, 10, [136, 126, 109]);
            b.frustum(v(x, 0.54, -0.48), 0.26, 0.084, 0.18, 10, BLUE);
        }
    }
    if level >= 5 {
        for n in -5..=5 {
            b.slab(
                v(n as f32 * 0.19, 0.104, 1.113),
                v(0.135, 0.018, 0.13),
                if n % 2 == 0 { GOLD } else { TEAL },
            );
        }
    }
}

fn colonnade(b: &mut Batch, level: u8, shrine: bool) {
    let tile = if shrine { BLUE } else { TERRACOTTA };
    let height = if shrine { 1.54 } else { 1.40 } + if level >= 5 { 0.17 } else { 0.0 };
    let raised = if shrine { 0.30 } else { 0.17 };
    let depth = if level >= 3 { 1.86 } else { 1.46 };
    let columns: &[f32] = if shrine || level >= 5 {
        &[-1.00, -0.35, 0.35, 1.00]
    } else {
        &[-1.04, 0.0, 1.04]
    };
    plinth(b, v(0.0, 0.0, -0.07), 2.72, depth + 0.14, raised, MARBLE);
    for &x in columns {
        for z in [-depth * 0.38, depth * 0.39] {
            column(
                b,
                v(x, raised, z),
                height,
                if level >= 5 { 0.116 } else { 0.091 },
            );
        }
    }
    let beam_y = raised + height + 0.10;
    b.slab(v(0.0, beam_y, 0.0), v(2.48, 0.21, depth * 0.97), IVORY);
    b.slab(v(0.0, beam_y, depth * 0.49), v(2.49, 0.079, 0.05), tile);
    gable(
        b,
        v(0.0, beam_y + 0.107, 0.0),
        2.63,
        depth + 0.18,
        if level >= 4 { 0.49 } else { 0.33 },
        tile,
    );
    if shrine {
        b.frustum(v(0.0, raised, -0.09), 0.30, 0.25, 0.53, 10, CREAM);
        b.frustum(v(0.0, raised + 0.52, -0.09), 0.25, 0.35, 0.085, 10, GOLD);
    } else {
        bench(b, v(0.0, raised, -0.43), 1.55);
    }
    if level >= 2 {
        let front = if level >= 3 { 0.86 } else { 0.73 };
        stoop(
            b,
            v(0.0, 0.0, front),
            2.18,
            raised,
            1.22 - front,
            if shrine { 3 } else { 2 },
            MARBLE,
        );
        for x in [-1.13, 1.13] {
            flowerpot(
                b,
                v(x, raised, 0.48),
                0.62,
                if shrine {
                    [238, 137, 155]
                } else {
                    [197, 112, 155]
                },
            );
        }
    }
    if level >= 3 {
        for n in -6..=6 {
            let x = n as f32 * 0.17;
            b.slab(
                v(x, raised + 0.012, 0.28),
                v(0.14, 0.024, 0.18),
                if n % 2 == 0 { BLUE } else { GOLD },
            );
        }
    }
    if level >= 4 {
        b.gem(
            v(0.0, beam_y + 0.29, depth * 0.5 + 0.093),
            0.15,
            0.123,
            0.042,
            0.31,
            GOLD,
        );
        if shrine {
            for x in [-1.11, 1.11] {
                b.frustum(v(x, raised, 0.31), 0.18, 0.12, 0.56, 10, CREAM);
                b.frustum(v(x, raised + 0.56, 0.31), 0.09, 0.18, 0.10, 10, OAK);
                b.gem(v(x, raised + 0.72, 0.31), 0.086, 0.12, 0.082, x, GOLD);
            }
        }
    }
    if level >= 6 {
        for side in [-1.0, 1.0] {
            b.frustum(
                v(side * 1.11, beam_y + 0.10, depth * 0.49),
                0.115,
                0.095,
                0.135,
                9,
                MARBLE,
            );
            b.gem(
                v(side * 1.11, beam_y + 0.26, depth * 0.49),
                0.077,
                0.095,
                0.061,
                side,
                GOLD,
            );
        }
        b.slab(
            v(0.0, beam_y + 0.062, depth * 0.5 - 0.02),
            v(2.44, 0.053, 0.082),
            GOLD,
        );
    }
}

fn grove(b: &mut Batch, level: u8) {
    let earth = [155, 126, 77];
    plinth(b, Vec3::ZERO, 2.58, 2.08, 0.09, earth);
    for x in [-1.20, 1.20] {
        b.slab(v(x, 0.17, -0.10), v(0.10, 0.23, 1.65), CREAM);
        b.slab(v(x, 0.30, -0.10), v(0.15, 0.06, 1.70), IVORY);
    }
    olive(b, v(-0.63, 0.09, -0.32), 0.91 + f32::from(level) * 0.034);
    olive(b, v(0.63, 0.09, -0.23), 0.81 + f32::from(level) * 0.046);
    if level >= 2 {
        olive(b, v(0.0, 0.09, -0.65), 0.80);
    }
    flowerpot(b, v(-0.98, 0.10, 0.87), 0.62, [231, 192, 93]);
    if level >= 3 {
        let width = if level >= 5 { 2.29 } else { 1.73 };
        canopy(
            b,
            v(0.0, 0.09, 0.73),
            width,
            if level >= 5 { 0.91 } else { 0.63 },
            1.23,
            HERB,
            false,
        );
        bench(b, v(0.0, 0.10, 0.76), width * 0.67);
    }
    if level >= 4 {
        for i in 0..8 {
            let x = i as f32 * 0.23 - 0.81;
            b.gem(
                v(x, 1.38 + (x * 6.0).sin() * 0.09, 1.01),
                0.23,
                0.15,
                0.18,
                x,
                shift(HERB, 8 + (i * 9 % 26) as i16),
            );
            b.gem(v(x, 1.26, 1.105), 0.062, 0.073, 0.052, x, [128, 79, 121]);
        }
    }
    if level >= 6 {
        for x in [-1.03, 1.03] {
            b.slab(v(x, 0.18, 0.92), v(0.49, 0.19, 0.57), IVORY);
            flowerpot(b, v(x, 0.278, 0.93), 0.81, [224, 128, 167]);
        }
    }
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

fn storage(b: &mut Batch, level: u8) {
    let extra = if level >= 5 {
        0.34 + f32::from(level - 5) * 0.19
    } else {
        0.0
    };
    stone_base(b, 2.58, 1.69);
    b.slab(
        v(0.0, 0.71 + extra * 0.5, -0.16),
        v(2.35, 1.25 + extra, 1.33),
        [226, 193, 142],
    );
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
    roof(b, 2.55, 1.75, 1.36 + extra, 0.40, [141, 107, 78]);
    if level >= 2 {
        plinth(b, v(0.0, 0.0, 0.89), 2.65, 0.64, 0.13, CREAM);
        crate_box(b, v(-0.82, 0.13, 0.88), v(0.47, 0.43, 0.42), OAK);
    }
    if level >= 3 {
        crane(b, v(1.13, 0.13, 0.31), 1.76 + extra);
    }
    if level >= 4 {
        b.beam(v(-1.18, 0.13, 1.12), v(-1.18, 1.24, 1.12), 0.077, OAK);
        panel_roof(b, v(-0.36, 1.29, 0.80), 1.82, 0.66, 0.22, BLUE);
    }
    if level >= 5 {
        for x in [-0.76, 0.14] {
            window(b, x, 1.36 + extra * 0.29, front, TEAL);
        }
    }
    if level >= 6 {
        b.slab(v(0.0, 1.15, front), v(2.37, 0.11, 0.12), BLUE);
        crate_box(b, v(-1.0, 0.58, 0.92), v(0.38, 0.27, 0.32), [159, 112, 68]);
    }
    for (x, s, color) in [
        (-0.98, 0.75, BLUE),
        (-1.20, 0.89, TERRACOTTA),
        (1.15, 0.89, IVORY),
    ] {
        amphora(b, v(x, 0.14, 0.75), s, color);
    }
}

fn press(b: &mut Batch, level: u8) {
    stone_base(b, 2.51, 1.61);
    let front = if level >= 4 { 0.44 } else { -0.29 };
    let high = if level >= 6 { 1.85 } else { 1.53 };
    for x in [-1.02, 1.02] {
        b.beam(v(x, 0.12, -0.70), v(x, high, -0.70), 0.11, OAK);
        if level >= 2 {
            b.beam(v(x, 0.12, front), v(x, high - 0.10, front), 0.11, OAK);
        }
    }
    panel_roof(
        b,
        v(0.0, high + 0.03, (front - 0.70) / 2.0),
        2.36,
        front + 1.04,
        0.13,
        HERB,
    );
    if level >= 3 {
        tank(
            b,
            v(0.0, 0.13, 0.96),
            1.31,
            0.41,
            0.24,
            CREAM,
            [177, 145, 66],
        );
    }
    if level >= 5 {
        for x in [-1.02, 1.02] {
            b.frustum(v(x, 0.13, -0.70), 0.22, 0.23, 0.41, 10, CREAM);
            b.frustum(v(x, 0.54, -0.70), 0.23, 0.10, 0.12, 10, HERB);
        }
    }
    if level >= 6 {
        b.beam(
            v(-1.16, 1.56, front + 0.03),
            v(1.16, 1.56, front + 0.03),
            0.095,
            OAK,
        );
        b.gem(v(0.0, 1.43, front + 0.065), 0.128, 0.15, 0.04, 0.2, GOLD);
        for z in [-0.45, -0.20, 0.06] {
            b.beam(v(1.02, 0.94, z), v(1.02, 1.61, z), 0.063, shift(OAK, 20));
        }
    }
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

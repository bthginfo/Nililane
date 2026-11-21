use bevy::prelude::*;

use super::mesh::{Batch, Tint, shift, v};
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
        Kind::Bakery | Kind::Potter | Kind::Taverna | Kind::Inn => shop(&mut b, kind),
        Kind::Storehouse => storage(&mut b),
        Kind::Press => press(&mut b),
        Kind::Shrine => sanctuary(&mut b),
        Kind::Aqueduct | Kind::Baths | Kind::Forum | Kind::Villa => {
            unreachable!("Roman buildings have their own meshes")
        }
    }
    for n in 1..level {
        let x = [-1.09, 1.09, 0.64][usize::from(n - 1)];
        if kind == Kind::Grove {
            olive(&mut b, v(x, 0.0, -0.66), 0.72);
        } else {
            amphora(&mut b, v(x, 0.04, 0.87), 0.72, BLUE);
            b.slab(v(x, 0.37, 0.86), v(0.048, 0.65, 0.048), OAK);
            b.face(
                &[v(x, 0.51, 0.84), v(x + 0.27, 0.57, 0.86), v(x, 0.68, 0.84)],
                GOLD,
            );
        }
    }
    b.mesh()
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

fn shop(b: &mut Batch, kind: Kind) {
    let (color, roof_color, fabric, height) = match kind {
        Kind::Bakery => (IVORY, TERRACOTTA, TEAL, 1.55),
        Kind::Potter => ([230, 182, 138], [173, 100, 67], [198, 99, 77], 1.43),
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

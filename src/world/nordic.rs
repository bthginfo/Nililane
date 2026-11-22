use bevy::prelude::*;

use crate::{campaign::NORDIC_START, model::Kind};

use super::mesh::{Batch, Grain, Tint, shift, v};

const OAK: Tint = [99, 71, 52];
const PINE: Tint = [151, 114, 76];
const DARK: Tint = [71, 57, 49];
const MOSS: Tint = [111, 129, 76];
const STONE: Tint = [145, 154, 150];
const RED: Tint = [164, 84, 60];
const GOLD: Tint = [219, 172, 102];
const LINEN: Tint = [232, 211, 174];
const BLUE: Tint = [71, 112, 141];

pub fn model(kind: Kind, level: u8) -> Mesh {
    let mut b = Batch::default();
    use Kind::*;
    match kind {
        Longhouse => hall(&mut b, false),
        FeastHall => hall(&mut b, true),
        Fishery => fishery(&mut b),
        Smokehouse => smoker(&mut b),
        Woodlot => woodlot(&mut b),
        Boatyard => yard(&mut b),
        Weaver => weaver(&mut b),
        Well => well(&mut b),
        Storehouse => store(&mut b),
        _ => unreachable!("building unavailable in the Nordic campaign"),
    }
    for tier in 2..=level {
        append_upgrade(&mut b, kind, tier);
    }
    b.mesh()
}

fn wall(b: &mut Batch, center: Vec3, w: f32, height: f32, depth: f32, wood: Tint) {
    b.slab(center, v(w, height, depth), shift(wood, -12));
    let planks = (w / 0.16).ceil() as usize;
    for i in 0..planks {
        let x = ((i as f32 + 0.5) / planks as f32 - 0.5) * w;
        b.slab(
            center + v(x, 0.0, depth * 0.5 + 0.015),
            v(w / planks as f32 * 0.96, height, 0.035),
            shift(wood, (i * 23 % 29) as i16 - 10),
        );
    }
    for x in [-0.48, 0.48] {
        b.slab(
            center + v(x * w, 0.0, depth * 0.51),
            v(0.098, height + 0.12, 0.137),
            DARK,
        );
    }
}

fn turf(b: &mut Batch, at: Vec3, width: f32, depth: f32, rise: f32, color: Tint) {
    let border = |x: f32, z: f32| {
        let side = z.abs();
        at + v(
            width * x * (1.0 - side * 0.05),
            rise * (1.0 - side.powf(1.32)) + 0.05 * x * x,
            z * depth * (1.0 - x * x * 0.09),
        )
    };
    for side in [-1.0, 1.0] {
        for n in 0..8 {
            let x = n as f32 * 0.25 - 1.0;
            for i in 0..3 {
                let z = i as f32 / 3.0 * side;
                let next = (i + 1) as f32 / 3.0 * side;
                let mut face = [
                    border(x, z),
                    border(x, next),
                    border(x + 0.25, next),
                    border(x + 0.25, z),
                ];
                if side < 0.0 {
                    face.reverse();
                }
                b.face(&face, shift(color, (n * 13 + i * 11) as i16 % 30 - 10));
            }
            b.beam(border(x, side), border(x + 0.25, side), 0.097, DARK);
            if (n + u32::from(side > 0.0)).is_multiple_of(3) {
                let tip = border(x + 0.1, side * 0.86);
                b.gem(
                    tip + v(0.0, 0.025, 0.0),
                    0.105,
                    0.047,
                    0.10,
                    x,
                    shift(color, 24),
                );
            }
        }
    }
    for side in [-1.0, 1.0] {
        let outer = side * 1.0;
        for n in 0..6 {
            let a = border(outer, n as f32 / 3.0 - 1.0);
            let c = border(outer, (n + 1) as f32 / 3.0 - 1.0);
            b.beam(a, c, 0.092, OAK);
            let mut face = [a, c, c.with_y(at.y - 0.11), a.with_y(at.y - 0.11)];
            if side > 0.0 {
                face.reverse();
            }
            b.face(&face, PINE);
        }
        b.beam(
            at + v(side * width * 0.9, rise - 0.01, 0.0),
            at + v(side * width * 1.03, rise + 0.16, 0.0),
            0.09,
            OAK,
        );
    }
    b.beam(
        border(-1.0, 0.0),
        border(1.0, 0.0),
        0.067,
        shift(color, -14),
    );
}

fn shingle(b: &mut Batch, at: Vec3, half: f32, dept: f32, high: f32, color: Tint) {
    for side in [-1.0, 1.0] {
        for i in 0..9 {
            let x = half * (i as f32 / 4.5 - 1.0);
            let dx = half * (2.0 / 9.0);
            for row in 0..4 {
                let z = row as f32 / 4.0;
                let p = at + v(x + dx * 0.5, high * (1.0 - z), side * dept * z);
                b.block(
                    p + v(0.0, -0.12 * high, side * dept * 0.125),
                    v(dx + 0.014, 0.072, (dept * dept + high * high).sqrt() * 0.27),
                    Quat::from_rotation_x(side * high.atan2(dept)),
                    shift(color, (i * 17 + row * 7) as i16 % 29 - 8),
                );
            }
        }
        b.beam(
            at + v(-half, high, 0.0),
            at + v(-half, 0.0, side * dept),
            0.073,
            DARK,
        );
        b.beam(
            at + v(half, high, 0.0),
            at + v(half, 0.0, side * dept),
            0.073,
            DARK,
        );
    }
    b.beam(
        at + v(-half - 0.02, high + 0.035, 0.0),
        at + v(half + 0.02, high + 0.035, 0.0),
        0.083,
        OAK,
    );
}

fn footing(b: &mut Batch, width: f32, depth: f32) {
    b.slab(
        v(0.0, 0.095, 0.0),
        v(width, 0.136, depth),
        shift(STONE, -16),
    );
    for i in 0..8 {
        b.gem(
            v((i as f32 / 7.0 - 0.5) * width * 0.9, 0.118, depth * 0.49),
            width * 0.077,
            0.112,
            0.142,
            i as f32 * 1.7,
            shift(STONE, (i * 13 % 23) as i16 - 9),
        );
    }
}

fn hall(b: &mut Batch, feast: bool) {
    let roof_y = if feast { 1.47 } else { 1.18 };
    let width = if feast { 2.70 } else { 2.48 };
    footing(b, width, 1.69);
    wall(
        b,
        v(0.0, roof_y * 0.55, -0.05),
        width - 0.18,
        roof_y - 0.12,
        1.51,
        if feast { RED } else { PINE },
    );
    turf(
        b,
        v(0.0, roof_y, -0.12),
        width * 0.55,
        1.12,
        if feast { 0.74 } else { 0.64 },
        if feast { shift(MOSS, 10) } else { MOSS },
    );
    let door_x = if feast { 0.0 } else { -0.24 };
    b.slab(
        v(door_x, 0.609, 0.80),
        v(if feast { 0.78 } else { 0.57 }, 0.91, 0.119),
        DARK,
    );
    for x in [door_x - 0.43, door_x + 0.43] {
        b.beam(
            v(x, 0.15, 0.87),
            v(x * 0.91, roof_y + 0.15, 0.87),
            0.105,
            shift(OAK, 9),
        );
    }
    b.beam(
        v(door_x - 0.48, roof_y - 0.24, 0.91),
        v(door_x + 0.48, roof_y - 0.24, 0.91),
        0.110,
        OAK,
    );
    for x in [-0.74, 0.70] {
        b.slab(v(x, 0.752, 0.750), v(0.27, 0.26, 0.085), DARK);
        b.slab(v(x, 0.746, 0.807), v(0.194, 0.177, 0.036), GOLD);
        b.slab(v(x, 0.747, 0.837), v(0.030, 0.223, 0.018), OAK);
    }
    let flue = v(-0.52, roof_y + 0.37, -0.28);
    b.slab(flue, v(0.25, 0.65, 0.24), STONE);
    b.slab(flue + v(0.0, 0.34, 0.0), v(0.30, 0.105, 0.30), DARK);
    if feast {
        for x in [-1.0, 1.0] {
            b.slab(v(x, 0.60, 0.885), v(0.26, 0.54, 0.08), [180, 90, 58]);
            brooch(b, v(x, 0.82, 0.935), 0.13, GOLD);
            b.slab(v(x, 0.27, 1.09), v(0.47, 0.15, 0.27), PINE);
        }
        for sign in [-1.0, 1.0] {
            let p = v(sign * width * 0.47, roof_y + 0.66, -0.12);
            b.beam(p, p + v(sign * 0.22, 0.33, 0.0), 0.098, OAK);
            b.gem(p + v(sign * 0.21, 0.36, 0.0), 0.128, 0.077, 0.064, 0.5, RED);
        }
    } else {
        b.slab(v(0.75, 0.251, 0.98), v(0.61, 0.119, 0.30), PINE);
        b.slab(v(0.75, 0.193, 0.98), v(0.48, 0.103, 0.23), OAK);
    }
}

fn brooch(b: &mut Batch, p: Vec3, size: f32, color: Tint) {
    b.block(
        p,
        v(size, size, size * 0.14),
        Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
        color,
    );
    b.gem(
        p + Vec3::Z * size * 0.13,
        size * 0.25,
        size * 0.25,
        size * 0.14,
        0.4,
        RED,
    );
}

fn fishery(b: &mut Batch) {
    for x in [-1.09, 1.08] {
        for z in [-0.76, 0.85] {
            b.slab(v(x, 0.24, z), v(0.14, 0.44, 0.14), OAK);
        }
    }
    for i in 0..11 {
        b.slab(
            v(-1.13 + i as f32 * 0.226, 0.42, 0.0),
            v(0.22, 0.117, 1.98),
            shift(PINE, (i * 17 % 27) as i16),
        );
    }
    wall(b, v(-0.57, 0.89, -0.29), 1.09, 0.83, 0.91, RED);
    shingle(b, v(-0.56, 1.27, -0.27), 0.71, 0.66, 0.51, [77, 87, 90]);
    b.slab(v(-0.59, 0.827, 0.194), v(0.36, 0.58, 0.057), DARK);
    drying_rack(b, v(0.58, 0.47, 0.63), 0.89, BLUE);
    for i in 0..8 {
        let z = -0.51 + i as f32 * 0.12;
        b.beam(v(0.45, 0.52, z), v(1.04, 0.53, z - 0.24), 0.015, LINEN);
        b.beam(
            v(0.42 + i as f32 * 0.075, 0.52, -0.65),
            v(0.428 + i as f32 * 0.075, 0.52, 0.27),
            0.017,
            LINEN,
        );
    }
}

fn drying_rack(b: &mut Batch, at: Vec3, s: f32, tint: Tint) {
    for x in [-0.58, 0.58] {
        b.beam(
            at + v(x, 0.0, 0.10) * s,
            at + v(x, 0.88, -0.04) * s,
            0.077 * s,
            OAK,
        );
    }
    b.beam(
        at + v(-0.65, 0.86, -0.01) * s,
        at + v(0.65, 0.86, -0.01) * s,
        0.063 * s,
        PINE,
    );
    for i in 0..5 {
        let p = at + v(-0.41 + i as f32 * 0.21, 0.59 + (i % 2) as f32 * 0.04, 0.014) * s;
        b.beam(
            p + v(0.0, 0.123, 0.0) * s,
            p + v(0.0, 0.25, -0.015) * s,
            0.014 * s,
            LINEN,
        );
        b.gem(
            p,
            0.055 * s,
            0.149 * s,
            0.041 * s,
            i as f32,
            shift(tint, i as i16 * 7),
        );
        b.gem(
            p + v(0.0, -0.114, 0.0) * s,
            0.061 * s,
            0.048 * s,
            0.035 * s,
            0.2,
            shift(tint, 22),
        );
    }
}

fn smoker(b: &mut Batch) {
    footing(b, 2.35, 1.83);
    wall(b, v(-0.26, 0.87, -0.29), 1.74, 1.44, 1.17, shift(OAK, 3));
    shingle(b, v(-0.26, 1.53, -0.20), 1.07, 0.8, 0.58, [83, 89, 83]);
    b.slab(v(-0.36, 0.76, 0.354), v(0.70, 1.02, 0.104), DARK);
    b.slab(v(-0.36, 0.49, 0.422), v(0.52, 0.39, 0.075), [150, 84, 48]);
    b.slab(v(-0.36, 0.43, 0.476), v(0.38, 0.18, 0.025), [227, 127, 59]);
    b.slab(
        v(-0.70, 1.77, -0.33),
        v(0.35, 1.14, 0.37),
        shift(STONE, -28),
    );
    for n in 0..5 {
        b.slab(
            v(-0.70, 1.52 + n as f32 * 0.18, -0.33),
            v(0.37, 0.028, 0.388),
            shift(STONE, -8 - n * 5),
        );
    }
    b.slab(v(-0.70, 2.35, -0.33), v(0.43, 0.09, 0.43), DARK);
    drying_rack(b, v(0.35, 0.16, 0.89), 0.84, [191, 127, 80]);
}

fn log(b: &mut Batch, at: Vec3, length: f32, radius: f32, bark: Tint) {
    let rim = |end: f32, i: usize| {
        let a = i as f32 * std::f32::consts::TAU / 8.0;
        at + v(radius * a.cos(), radius * a.sin(), end * length * 0.5)
    };
    for i in 0..8 {
        b.face(
            &[rim(-1.0, i + 1), rim(1.0, i + 1), rim(1.0, i), rim(-1.0, i)],
            shift(bark, (i * 7 % 22) as i16),
        );
    }
    let end = (0..8).map(|i| rim(1.0, i)).collect::<Vec<_>>();
    b.face(&end, PINE);
    b.gem(
        at + v(0.0, 0.0, length * 0.5 + 0.004),
        radius * 0.67,
        radius * 0.67,
        0.009,
        0.3,
        GOLD,
    );
}

fn woodlot(b: &mut Batch) {
    footing(b, 2.45, 1.87);
    for x in [-1.0, 0.05, 1.05] {
        b.beam(v(x, 0.09, -0.30), v(x, 1.04, -0.47), 0.10, OAK);
        b.beam(v(x, 0.10, 0.09), v(x, 0.85, 0.12), 0.072, OAK);
    }
    for i in 0..12 {
        b.block(
            v(-1.14 + i as f32 * 0.202, 1.0, -0.34),
            v(0.211, 0.076, 1.16),
            Quat::from_rotation_x(0.24),
            shift(MOSS, (i * 13 % 25) as i16),
        );
    }
    for n in 0..12 {
        let x = -0.91 + (n % 5) as f32 * 0.405 + (n / 5) as f32 * 0.09;
        let y = 0.264 + (n / 5) as f32 * 0.175;
        log(b, v(x, y, 0.26 - (n % 3) as f32 * 0.057), 1.30, 0.13, OAK);
    }
    log(b, v(-1.00, 0.183, 0.91), 0.51, 0.112, PINE);
    b.frustum(v(0.87, 0.168, 0.88), 0.21, 0.166, 0.23, 8, OAK);
    b.block(
        v(0.87, 0.482, 0.90),
        v(0.069, 0.31, 0.063),
        Quat::from_rotation_z(-0.41),
        PINE,
    );
    b.block(
        v(0.80, 0.552, 0.90),
        v(0.18, 0.10, 0.044),
        Quat::from_rotation_z(-0.4),
        STONE,
    );
}

fn skiff(b: &mut Batch, at: Vec3, length: f32, beam: f32, stripe: Tint) {
    let point = |i: usize, j: usize, sign: f32| {
        let u = i as f32 / 4.0 - 1.0;
        let edge = 1.0 - u * u;
        let height = 0.045 + j as f32 * 0.069 + u.abs().powi(4) * 0.27;
        at + v(
            u * length * (0.90 + j as f32 * 0.031),
            height,
            sign * beam * edge * (0.37 + j as f32 * 0.21),
        )
    };
    for n in 0..8 {
        for j in 0..3 {
            for sign in [-1.0, 1.0] {
                let mut face = [
                    point(n, j, sign),
                    point(n + 1, j, sign),
                    point(n + 1, j + 1, sign),
                    point(n, j + 1, sign),
                ];
                if sign < 0.0 {
                    face.reverse();
                }
                b.face(
                    &face,
                    if j == 2 {
                        stripe
                    } else {
                        shift(OAK, (j * 13 + n) as i16)
                    },
                );
                b.beam(
                    point(n, j + 1, sign),
                    point(n + 1, j + 1, sign),
                    0.021,
                    DARK,
                );
            }
        }
    }
    for x in [-0.43, 0.0, 0.43] {
        b.slab(
            at + v(length * x, 0.252, 0.0),
            v(0.14, 0.071, beam * 1.6),
            PINE,
        );
    }
}

fn yard(b: &mut Batch) {
    footing(b, 2.68, 1.75);
    for x in [-1.13, 1.13] {
        for z in [-0.69, 0.58] {
            b.slab(v(x, 0.82, z), v(0.14, 1.34, 0.15), OAK);
            b.beam(v(x, 1.06, z), v(x * 0.74, 1.48, z), 0.086, PINE);
        }
        b.slab(v(x, 1.48, -0.06), v(0.19, 0.14, 1.54), DARK);
    }
    for x in [-0.69, 0.69] {
        b.beam(v(x, 0.19, 1.33), v(x, 0.35, -0.64), 0.107, PINE);
    }
    shingle(b, v(0.0, 1.49, -0.13), 1.34, 0.95, 0.69, [64, 95, 113]);
    skiff(b, v(0.0, 0.30, 0.37), 1.12, 0.58, [99, 136, 144]);
    b.beam(v(-0.76, 0.15, 1.06), v(-0.75, 0.68, 1.07), 0.060, PINE);
    b.beam(v(-0.75, 0.68, 1.07), v(0.70, 0.82, 0.85), 0.068, OAK);
    b.gem(v(0.68, 0.77, 0.86), 0.10, 0.11, 0.047, 0.17, GOLD);
}

fn weaving_rack(b: &mut Batch, at: Vec3, size: f32) {
    for x in [-0.58, 0.58] {
        b.beam(
            at + v(x, 0.0, 0.02) * size,
            at + v(x, 0.95, 0.02) * size,
            0.095 * size,
            PINE,
        );
    }
    b.beam(
        at + v(-0.66, 0.95, 0.02) * size,
        at + v(0.66, 0.95, 0.02) * size,
        0.067 * size,
        OAK,
    );
    for (i, color) in [RED, BLUE, [121, 130, 83]].into_iter().enumerate() {
        let x = -0.36 + i as f32 * 0.36;
        let y = 0.59 + i as f32 * 0.032;
        b.slab(
            at + v(x, y, 0.05) * size,
            v(0.297, 0.64, 0.041) * size,
            color,
        );
        for edge in [-0.104, 0.104] {
            b.slab(
                at + v(x + edge, y, 0.073) * size,
                v(0.031, 0.623, 0.013) * size,
                LINEN,
            );
        }
        for n in 0..3 {
            brooch(
                b,
                at + v(x, y + n as f32 * 0.155 - 0.15, 0.082) * size,
                0.092 * size,
                GOLD,
            );
        }
    }
}

fn weaver(b: &mut Batch) {
    footing(b, 2.36, 1.59);
    wall(b, v(0.0, 0.822, -0.16), 2.14, 1.39, 1.40, [156, 124, 88]);
    shingle(b, v(0.0, 1.52, -0.15), 1.23, 0.893, 0.60, BLUE);
    b.slab(v(0.0, 0.77, 0.59), v(1.29, 0.70, 0.058), DARK);
    weaving_rack(b, v(0.0, 0.21, 0.86), 1.02);
    for x in [-0.85, 0.91] {
        b.gem(v(x, 0.35, 0.93), 0.18, 0.16, 0.155, 0.54, LINEN);
        b.frustum(v(x, 0.15, 0.93), 0.123, 0.169, 0.145, 8, PINE);
    }
}

fn well(b: &mut Batch) {
    footing(b, 1.96, 1.63);
    b.frustum(v(0.0, 0.171, 0.10), 0.56, 0.55, 0.39, 9, STONE);
    for n in 0..10 {
        let t = n as f32 * std::f32::consts::TAU / 10.0;
        b.gem(
            v(t.cos() * 0.48, 0.545, t.sin() * 0.48 + 0.10),
            0.183,
            0.13,
            0.162,
            t,
            shift(STONE, n as i16 * 3 - 14),
        );
    }
    b.frustum(v(0.0, 0.55, 0.10), 0.44, 0.429, 0.026, 12, BLUE);
    for x in [-0.69, 0.69] {
        b.beam(v(x, 0.14, -0.38), v(x, 1.38, -0.36), 0.125, OAK);
    }
    turf(b, v(0.0, 1.27, -0.39), 0.89, 0.64, 0.41, shift(MOSS, -6));
    b.beam(v(-0.74, 0.976, -0.30), v(0.78, 0.976, -0.30), 0.080, PINE);
    b.beam(v(0.21, 0.99, -0.30), v(0.23, 0.69, -0.28), 0.029, LINEN);
    b.frustum(v(0.23, 0.54, -0.28), 0.120, 0.157, 0.162, 9, OAK);
}

fn store(b: &mut Batch) {
    for x in [-0.82, 0.82] {
        for z in [-0.64, 0.64] {
            b.slab(v(x, 0.28, z), v(0.16, 0.52, 0.16), OAK);
            b.slab(v(x, 0.389, z), v(0.28, 0.081, 0.27), STONE);
        }
    }
    b.slab(v(0.0, 0.48, 0.0), v(2.02, 0.147, 1.50), PINE);
    wall(b, v(0.0, 1.027, -0.10), 1.93, 1.05, 1.42, [104, 121, 121]);
    shingle(b, v(0.0, 1.55, -0.11), 1.14, 0.95, 0.70, [103, 79, 66]);
    b.slab(v(0.0, 0.95, 0.645), v(0.65, 0.86, 0.082), OAK);
    for side in [-1.0, 1.0] {
        b.beam(
            v(-side * 0.29, 0.62, 0.694),
            v(side * 0.29, 1.27, 0.694),
            0.059,
            PINE,
        );
    }
    for i in 0..5 {
        let step = i as f32;
        b.slab(
            v(0.0, 0.16 + step * 0.075, 1.13 - step * 0.11),
            v(0.61, 0.078, 0.24),
            PINE,
        );
    }
}

pub fn append_upgrade(b: &mut Batch, kind: Kind, tier: u8) {
    use Kind::*;
    match kind {
        Fishery => match tier {
            2 => drying_rack(b, v(0.62, 0.48, -0.43), 0.82, BLUE),
            3 => skiff(b, v(-0.45, 0.12, 1.02), 0.61, 0.29, RED),
            4 => shed(b, v(0.68, 0.42, -0.46), 0.55, 0.86, BLUE),
            5 => {
                b.slab(v(0.0, 0.39, 1.10), v(2.40, 0.13, 0.35), PINE);
                wooden_rail(b, v(0.0, 0.45, 1.19), 2.26);
            }
            _ => {
                b.beam(v(0.98, 0.53, -0.45), v(0.98, 1.81, -0.45), 0.085, OAK);
                b.beam(v(0.98, 1.78, -0.45), v(0.41, 1.78, -0.39), 0.066, PINE);
                b.gem(v(0.45, 1.68, -0.36), 0.145, 0.11, 0.13, 0.45, GOLD);
            }
        },
        Woodlot => match tier {
            2 | 4 => {
                let x = if tier == 2 { -0.71 } else { 0.71 };
                for i in 0..4 {
                    log(
                        b,
                        v(
                            x + (i % 2) as f32 * 0.22,
                            0.19 + (i / 2) as f32 * 0.17,
                            0.97,
                        ),
                        0.55,
                        0.12,
                        OAK,
                    );
                }
            }
            3 => shed(b, v(-0.60, 0.13, -0.66), 0.61, 1.20, MOSS),
            5 => wooden_rail(b, v(0.0, 0.13, 1.20), 2.50),
            _ => {
                for x in [-1.12, 1.13] {
                    b.beam(v(x, 0.14, 0.09), v(x, 1.59, -0.55), 0.086, OAK);
                }
                b.beam(v(-1.26, 1.43, -0.49), v(1.26, 1.43, -0.49), 0.093, PINE);
                b.slab(v(0.0, 1.48, -0.61), v(2.47, 0.12, 0.38), [126, 139, 91]);
            }
        },
        Boatyard => match tier {
            2 => {
                b.beam(v(1.06, 0.10, 0.82), v(1.06, 1.09, 0.82), 0.105, OAK);
                b.beam(v(1.06, 1.09, 0.82), v(0.51, 1.09, 0.91), 0.094, PINE);
                b.gem(v(0.6, 1.04, 0.94), 0.142, 0.148, 0.036, 0.1, GOLD);
                b.beam(v(0.58, 1.07, 0.97), v(0.58, 0.37, 0.97), 0.027, LINEN);
            }
            3 => shed(b, v(-0.87, 0.17, -0.58), 0.57, 1.26, [80, 102, 114]),
            4 => {
                b.beam(v(0.44, 1.64, -0.60), v(0.44, 2.68, -0.60), 0.075, OAK);
                b.slab(v(0.19, 2.51, -0.60), v(0.53, 0.266, 0.052), [179, 87, 65]);
                b.slab(v(0.20, 2.51, -0.568), v(0.38, 0.044, 0.020), GOLD);
            }
            5 => wooden_rail(b, v(0.0, 0.24, 1.24), 2.41),
            _ => shingle(b, v(0.0, 2.15, -0.28), 0.71, 0.45, 0.31, BLUE),
        },
        Smokehouse => match tier {
            2 => drying_rack(b, v(0.89, 0.17, -0.52), 0.66, [192, 143, 84]),
            3 => shed(b, v(0.86, 0.15, -0.57), 0.50, 1.38, [91, 97, 86]),
            4 => {
                b.slab(v(-0.74, 2.33, -0.34), v(0.262, 0.52, 0.271), STONE);
                b.slab(v(-0.74, 2.63, -0.34), v(0.38, 0.101, 0.40), OAK);
            }
            5 => awning(b, [153, 97, 75], 1.10),
            _ => {
                b.slab(v(0.55, 1.87, -0.18), v(0.28, 0.80, 0.29), shift(STONE, -23));
                b.slab(v(0.55, 2.31, -0.17), v(0.36, 0.085, 0.35), DARK);
            }
        },
        Weaver => match tier {
            2 => weaving_rack(b, v(-0.77, 0.16, -0.42), 0.61),
            3 => awning(b, BLUE, 1.24),
            4 => shed(b, v(0.79, 0.13, -0.74), 0.59, 1.40, RED),
            5 => {
                b.slab(v(0.0, 1.93, 0.27), v(0.72, 0.48, 0.56), LINEN);
                b.slab(v(0.0, 1.96, 0.556), v(0.41, 0.22, 0.044), DARK);
                shingle(b, v(0.0, 2.16, 0.23), 0.53, 0.44, 0.32, BLUE);
            }
            _ => {
                for (x, color) in [(-0.89, BLUE), (0.9, RED)] {
                    b.beam(v(x, 0.13, 1.05), v(x, 1.75, 1.05), 0.087, OAK);
                    b.slab(v(x, 1.53, 1.07), v(0.29, 0.43, 0.039), color);
                    brooch(b, v(x, 1.54, 1.096), 0.105, GOLD);
                }
            }
        },
        Longhouse | FeastHall => match tier {
            2 => wooden_rail(b, v(0.22, 0.119, 1.12), 1.87),
            3 => awning(b, if kind == FeastHall { RED } else { MOSS }, 1.06),
            4 => {
                let x = if kind == FeastHall { 0.79 } else { 0.86 };
                b.slab(v(x, 0.80, -0.62), v(0.64, 1.27, 0.84), PINE);
                turf(b, v(x, 1.45, -0.58), 0.41, 0.59, 0.37, MOSS);
            }
            5 => {
                b.slab(v(0.04, 1.58, 0.47), v(0.64, 0.46, 0.43), PINE);
                b.slab(v(0.04, 1.60, 0.697), v(0.26, 0.15, 0.041), GOLD);
                shingle(b, v(0.04, 1.86, 0.43), 0.44, 0.39, 0.28, [138, 92, 60]);
            }
            _ => {
                for x in [-1.10, 1.10] {
                    b.beam(v(x, 0.06, 1.16), v(x, 1.80, 1.12), 0.089, DARK);
                    b.gem(v(x, 1.37, 1.165), 0.136, 0.175, 0.031, 0.12, RED);
                    brooch(b, v(x, 1.37, 1.196), 0.122, GOLD);
                }
            }
        },
        Storehouse | Well => match tier {
            2 => wooden_rail(b, v(-0.04, 0.17, 1.11), 1.61),
            3 => shed(b, v(0.8, 0.12, -0.64), 0.54, 1.02, MOSS),
            4 => awning(b, [101, 122, 117], 1.03),
            5 => {
                let y = if kind == Storehouse { 2.12 } else { 1.60 };
                b.slab(v(-0.33, y, -0.37), v(0.70, 0.51, 0.66), PINE);
                b.slab(v(-0.33, y, -0.025), v(0.38, 0.17, 0.067), LINEN);
                shingle(b, v(-0.33, y + 0.28, -0.37), 0.48, 0.47, 0.30, BLUE);
            }
            _ => {
                for x in [-0.91, 0.91] {
                    b.slab(v(x, 0.56, 1.10), v(0.096, 0.93, 0.097), OAK);
                    b.gem(v(x, 1.03, 1.13), 0.13, 0.16, 0.13, 0.3, GOLD);
                }
            }
        },
        _ => unreachable!("building unavailable in the Nordic campaign"),
    }
}

fn wooden_rail(b: &mut Batch, at: Vec3, width: f32) {
    for x in [-0.5, -0.21, 0.21, 0.5] {
        b.beam(
            at + v(x * width, 0.0, 0.0),
            at + v(x * width, 0.37, 0.0),
            0.076,
            OAK,
        );
    }
    b.beam(
        at + v(-width * 0.54, 0.30, 0.0),
        at + v(width * 0.54, 0.30, 0.0),
        0.071,
        PINE,
    );
}

fn shed(b: &mut Batch, at: Vec3, half: f32, h: f32, color: Tint) {
    wall(b, at + Vec3::Y * h * 0.52, half * 1.67, h, 0.43, PINE);
    shingle(b, at + Vec3::Y * h, half, 0.37, 0.27, color);
}

fn awning(b: &mut Batch, color: Tint, y: f32) {
    for x in [-1.09, 1.09] {
        b.beam(v(x, 0.16, 1.25), v(x, y, 1.25), 0.099, PINE);
    }
    b.block(
        v(0.0, y, 1.08),
        v(2.34, 0.118, 0.49),
        Quat::from_rotation_x(0.21),
        color,
    );
    b.beam(
        v(-1.20, y - 0.056, 1.30),
        v(1.20, y - 0.056, 1.30),
        0.081,
        DARK,
    );
}

fn pine(b: &mut Batch, at: Vec3, height: f32, color: Tint, snow: bool) {
    b.frustum(at, height * 0.058, height * 0.031, height * 0.88, 7, DARK);
    for n in 0..4 {
        let y = 0.22 + 0.168 * n as f32;
        let radius = (0.29 - 0.056 * n as f32) * height;
        let p = at + v(0.0, y * height, 0.0);
        b.frustum(p, radius, 0.012, height * 0.37, 7, shift(color, n * 8));
        if snow {
            b.frustum(
                p + v(0.0, height * 0.114, 0.0),
                radius * 0.72,
                0.013,
                height * 0.248,
                7,
                [192, 207, 201],
            );
        }
    }
}

pub fn paving(i: usize, row: usize, chapter: usize) -> Tint {
    let local = chapter - NORDIC_START;
    let base = [
        [187, 169, 137],
        [170, 179, 169],
        [174, 172, 152],
        [183, 167, 142],
    ][local / 6];
    let edges = if !(1..9).contains(&row) { -14 } else { 0 };
    shift(base, (i * 17 + row * 13) as i16 % 23 - 12 + edges)
}

pub fn garden(b: &mut Batch, p: Vec3, chapter: usize, grain: &mut Grain) {
    let frost = ((chapter - NORDIC_START) / 6) == 1;
    for x in [-0.66, 0.38] {
        pine(
            b,
            p + v(x, 0.01, -0.35),
            grain.span(1.13, 1.90),
            [85, 116, 97],
            frost,
        );
    }
    for (x, z) in [(-0.72, 0.61), (0.56, 0.46)] {
        b.gem(
            p + v(x, 0.16, z),
            0.36,
            0.24,
            0.27,
            x * 4.0,
            shift(STONE, -19),
        );
    }
}

fn landmark(b: &mut Batch, p: Vec3, style: usize) {
    match style {
        0 => {
            b.gem(p + v(0.0, 0.54, 0.0), 0.39, 0.75, 0.24, 0.2, STONE);
            let at = p + v(0.0, 0.55, 0.217);
            for i in 0..4 {
                let y = -0.25 + i as f32 * 0.16;
                b.beam(
                    at + v(-0.035, y, 0.0),
                    at + v(-0.035, y + 0.13, 0.0),
                    0.027,
                    RED,
                );
                b.beam(
                    at + v(-0.035, y + 0.13, 0.0),
                    at + v(0.103, y + 0.065, -0.03),
                    0.025,
                    RED,
                );
            }
        }
        1 => {
            for x in [-0.51, 0.51] {
                b.beam(p + v(x, 0.0, 0.0), p + v(x * 0.92, 1.59, 0.0), 0.129, OAK);
            }
            b.beam(
                p + v(-0.55, 1.25, 0.05),
                p + v(0.54, 1.25, 0.05),
                0.104,
                PINE,
            );
            b.slab(p + v(0.0, 1.12, 0.046), v(0.73, 0.34, 0.073), [154, 91, 63]);
            brooch(b, p + v(0.0, 1.13, 0.092), 0.20, GOLD);
            shingle(b, p + v(0.0, 1.56, 0.0), 0.73, 0.35, 0.36, [85, 100, 109]);
        }
        2 => {
            b.slab(p + v(0.0, 0.26, 0.0), v(1.6, 0.132, 0.90), DARK);
            skiff(b, p + v(0.0, 0.32, 0.03), 0.83, 0.48, RED);
        }
        _ => {
            for i in 0..4 {
                let x = i as f32 * 0.17 - 0.39;
                log(b, p + v(x, 0.17 + x * 0.15, 0.0), 0.64, 0.131, OAK);
                b.gem(
                    p + v(x, 0.44 - x.abs() * 0.27, 0.0),
                    0.093,
                    0.22,
                    0.14,
                    x,
                    [223, 144, 77],
                );
            }
            for x in [-0.52, 0.52] {
                b.gem(p + v(x, 0.16, 0.0), 0.219, 0.211, 0.27, 0.5, STONE);
            }
        }
    }
}

pub fn bank(b: &mut Batch, wide: f32, chapter: usize, grain: &mut Grain) {
    let local = chapter - NORDIC_START;
    let climate = local / 6;
    let frost = climate == 1;
    let turf_color = [
        [114, 132, 90],
        [162, 178, 170],
        [119, 137, 111],
        [125, 135, 95],
    ][climate];
    for i in 0..11 {
        let x = -wide + 0.9 + i as f32 * (2.0 * wide - 1.8) / 10.0;
        let z = grain.span(-6.36, -4.69);
        let base = v(x, -0.23, z);
        let h = grain.span(0.43, 1.65);
        b.gem(base, 1.44, h, 1.40, grain.span(0.0, 4.0), shift(STONE, -26));
        b.gem(
            base + v(0.09, h * 0.30, -0.11),
            1.22,
            h * 0.79,
            1.15,
            grain.span(0.0, 3.0),
            turf_color,
        );
        for n in 0..2 {
            pine(
                b,
                base + v(grain.span(-0.62, 0.61), h * 0.72, grain.span(-0.48, 0.43)),
                grain.span(1.05, 2.39),
                shift([77, 109, 94], i as i16 % 5 * 4 + n * 7),
                frost,
            );
        }
    }
    for i in 0..8 {
        let x = -wide + i as f32 * (2.0 * wide - 0.65) / 7.0;
        let z = 4.44 + grain.span(-0.26, 0.34);
        if (x - wide + 3.0).abs() > 1.6 {
            b.gem(
                v(x, -0.19, z + 0.62),
                grain.span(0.31, 0.77),
                grain.span(0.34, 0.65),
                0.56,
                i as f32,
                shift(STONE, -19),
            );
        }
    }
    let x = if local.is_multiple_of(2) {
        -wide + 0.9
    } else {
        wide - 0.9
    };
    landmark(b, v(x, 0.031, 3.66), local % 4);
}

pub fn distant(chapter: usize) -> Mesh {
    let local = chapter - NORDIC_START;
    let mut rng = Grain::new(chapter as u32 * 8119 + 53);
    let mut b = Batch::default();
    for i in 0..6 {
        let x = -47.0 + i as f32 * 17.3;
        let z = -23.0 - (i % 3) as f32 * 7.0 + rng.span(-1.3, 1.5);
        let high = rng.span(5.6, 8.9) + if local / 6 == 1 { 2.7 } else { 0.0 };
        let w = rng.span(6.0, 8.5);
        for ridge in 0..3 {
            let p = v(x + ridge as f32 * 3.8, -0.56, z + rng.span(-1.1, 1.7));
            let shade = shift([119, 150, 153], i as i16 * 3 - ridge as i16 * 6);
            b.gem(
                p,
                w,
                high * rng.span(0.60, 0.95),
                w * 0.58,
                ridge as f32 * 0.81,
                shade,
            );
            if local / 6 <= 1 && (i + ridge) % 3 != 0 {
                b.gem(
                    p + v(0.0, high * 0.59, 0.0),
                    w * 0.36,
                    high * 0.29,
                    w * 0.22,
                    ridge as f32 * 0.81,
                    [199, 214, 209],
                );
            }
        }
    }
    b.mesh()
}

use bevy::prelude::*;

use super::{
    architecture::{panel_roof, plinth, stoop, tank},
    buildings::{amphora, flowerpot},
    mesh::{Batch, Grain, Tint, shift, v},
    upgrades::{bench, crate_box},
};
use crate::{campaign::harbor_stage, model::Kind};

const SANDSTONE: Tint = [217, 177, 125];
const STUCCO: Tint = [243, 202, 151];
const SHALE: Tint = [153, 108, 83];
const WOOD: Tint = [111, 71, 53];
const LAPIS: Tint = [50, 115, 140];
const JADE: Tint = [63, 153, 134];
const PLUM: Tint = [132, 70, 102];
const SUN: Tint = [234, 170, 79];
const WATER: Tint = [72, 159, 157];

pub(super) fn model(kind: Kind, level: u8) -> Mesh {
    use Kind::*;
    let mut b = Batch::default();
    match kind {
        OasisHome => house(&mut b, level),
        Cistern => cistern(&mut b, level),
        DateGarden => dates(&mut b, level),
        SpiceBazaar => bazaar(&mut b, level),
        Caravanserai => caravanserai(&mut b, level),
        Qanat => qanat(&mut b, level),
        Glassworks => glassworks(&mut b, level),
        Perfumer => perfumer(&mut b, level),
        MoonGarden => moon_garden(&mut b, level),
        CarpetHall => carpet_hall(&mut b, level),
        _ => unreachable!("building unavailable in the Oases"),
    }
    b.mesh()
}

fn arch(b: &mut Batch, p: Vec3, inset: bool, post: f32, radius: f32, stone: Tint) {
    let thickness = 0.096;
    let depth = 0.14;
    let point = |i: usize, r: f32, z: f32| {
        let angle = i as f32 * std::f32::consts::PI / 14.0;
        p + v(
            angle.cos() * r,
            post + angle.sin().max(0.0).powf(0.68) * r * 1.19,
            z,
        )
    };
    for side in [-1.0, 1.0] {
        b.slab(
            p + v(side * (radius + thickness / 2.0), post / 2.0, 0.0),
            v(thickness, post, depth),
            stone,
        );
        b.slab(
            p + v(side * (radius + thickness / 2.0), 0.06, 0.0),
            v(0.15, 0.12, depth + 0.06),
            SHALE,
        );
    }
    if inset {
        b.slab(
            p + v(0.0, post / 2.0, -0.008),
            v(radius * 2.0, post, 0.05),
            shift(stone, -79),
        );
        for i in 0..14 {
            b.face(
                &[
                    p + v(0.0, post, -0.009),
                    point(i, radius, -0.009),
                    point(i + 1, radius, -0.009),
                ],
                shift(stone, -79),
            );
        }
    }
    for i in 0..14 {
        let shade = if i % 2 == 0 { stone } else { shift(stone, -26) };
        let a = point(i, radius, depth * 0.5);
        let c = point(i + 1, radius, depth * 0.5);
        let d = point(i, radius + thickness, depth * 0.5);
        let e = point(i + 1, radius + thickness, depth * 0.5);
        let back = v(0.0, 0.0, depth);
        b.face(&[a, d, e, c], shade);
        b.face(&[a - back, c - back, e - back, d - back], shade);
        b.face(&[d, d - back, e - back, e], shade);
        b.face(&[c, c - back, a - back, a], shift(shade, -12));
    }
}

fn slab_roof(b: &mut Batch, at: Vec3, width: f32, depth: f32, border: Tint) {
    b.slab(at, v(width, 0.12, depth), SANDSTONE);
    b.slab(
        at + v(0.0, 0.071, 0.0),
        v(width + 0.046, 0.045, depth + 0.046),
        STUCCO,
    );
    b.slab(
        at + v(0.0, -0.048, depth * 0.497),
        v(width, 0.062, 0.033),
        border,
    );
    for i in 0..(width / 0.18) as usize {
        let x = (i as f32 + 0.50) * 0.18 - width * 0.50;
        b.block(
            at + v(x, -0.04, depth * 0.51),
            v(0.060, 0.059, 0.022),
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
            SUN,
        );
    }
}

fn dome(b: &mut Batch, at: Vec3, radius: f32, tint: Tint) {
    let contours = [
        (0.0, 0.86),
        (0.17, 1.04),
        (0.42, 0.94),
        (0.69, 0.65),
        (0.91, 0.25),
        (0.99, 0.09),
    ];
    for pair in contours.windows(2) {
        let [(a, ra), (c, rc)] = [pair[0], pair[1]];
        b.frustum(
            at + Vec3::Y * (a * radius),
            radius * ra,
            radius * rc,
            radius * (c - a),
            16,
            shift(tint, (a * 28.0) as i16),
        );
    }
    for (y, r) in [(0.99, 0.085), (1.15, 0.060)] {
        b.frustum(
            at + Vec3::Y * (y * radius),
            radius * r,
            radius * (r * 0.63),
            0.16 * radius,
            9,
            SUN,
        );
    }
}

fn screen(b: &mut Batch, at: Vec3, width: f32, height: f32, tint: Tint) {
    b.slab(at, v(width, height, 0.055), shift(WOOD, -22));
    for y in [-height * 0.45, 0.0, height * 0.45] {
        b.slab(at + v(0.0, y, 0.031), v(width, 0.040, 0.039), tint);
    }
    for i in 0..5 {
        let x = (i as f32 - 2.0) * width * 0.217;
        b.slab(at + v(x, 0.0, 0.038), v(0.030, height, 0.042), tint);
        b.gem(
            at + v(x, height * 0.28, 0.056),
            0.043,
            0.055,
            0.020,
            0.0,
            SUN,
        );
    }
}

fn wall(b: &mut Batch, at: Vec3, size: Vec3, stone: Tint) {
    b.slab(at, size, stone);
    let front = at.z + size.z * 0.50;
    for row in 0..((size.y * 0.53) / 0.13) as usize {
        let y = at.y - size.y * 0.50 + 0.074 + row as f32 * 0.137;
        for i in 0..(size.x / 0.23).floor() as usize {
            let x = at.x - size.x * 0.5
                + 0.12
                + i as f32 * 0.23
                + if row % 2 == 0 { 0.0 } else { 0.059 };
            b.slab(
                v(x, y, front - 0.006),
                v(0.19, 0.080, 0.041),
                shift(stone, (i * 17 + row * 9) as i16 % 31 - 21),
            );
        }
    }
}

fn house(b: &mut Batch, level: u8) {
    let y = 1.37 + f32::from(level.saturating_sub(3)) * 0.20;
    plinth(b, Vec3::ZERO, 2.73, 2.25, 0.15, SANDSTONE);
    wall(b, v(-0.19, 0.15 + y / 2.0, -0.44), v(2.29, y, 1.24), STUCCO);
    slab_roof(b, v(-0.18, 0.19 + y, -0.44), 2.37, 1.41, LAPIS);
    arch(b, v(-0.41, 0.16, 0.233), true, 0.72, 0.28, SHALE);
    let shutter = if level >= 2 { LAPIS } else { WOOD };
    screen(b, v(0.53, 0.92, 0.208), 0.46, 0.48, shutter);
    if level >= 2 {
        flowerpot(b, v(-1.09, 0.16, 0.75), 0.85, [213, 126, 132]);
    }
    if level >= 3 {
        stoop(b, v(-0.39, 0.0, 0.91), 1.47, 0.15, 0.32, 2, SANDSTONE);
        bench(b, v(0.73, 0.16, 0.80), 0.65);
    }
    if level >= 4 {
        arch(b, v(0.77, 0.15, 0.88), false, 0.75, 0.36, SHALE);
        slab_roof(b, v(0.77, 1.50, 0.64), 1.12, 0.75, JADE);
        b.slab(v(0.77, 1.39, 0.89), v(1.01, 0.14, 0.16), SANDSTONE);
    }
    if level >= 5 {
        b.slab(v(-0.18, y + 0.31, -1.07), v(2.33, 0.18, 0.15), SANDSTONE);
        for (side, x) in [(-1.0, -1.26), (1.0, 0.90)] {
            b.slab(v(x, y + 0.31, -0.47), v(0.13, 0.18, 1.20), SANDSTONE);
            b.slab(
                v(x, y + 0.416, -1.05),
                v(0.19, 0.092, 0.21),
                if side > 0.0 { LAPIS } else { SHALE },
            );
        }
    }
    if level >= 5 {
        screen(b, v(-0.53, y - 0.08, 0.206), 0.66, 0.40, WOOD);
    }
    if level >= 6 {
        let core = v(-0.47, y + 0.28, -0.55);
        b.slab(core + Vec3::Y * 0.22, v(1.21, 0.45, 0.74), SANDSTONE);
        slab_roof(b, core + Vec3::Y * 0.46, 1.31, 0.85, LAPIS);
        screen(b, core + v(0.0, 0.23, 0.381), 0.55, 0.25, WOOD);
    }
}

fn cistern(b: &mut Batch, level: u8) {
    let h = if level >= 4 { 0.41 } else { 0.29 };
    plinth(b, Vec3::ZERO, 2.69, 2.21, 0.15, SANDSTONE);
    tank(
        b,
        v(0.0, 0.15, 0.40),
        if level >= 4 { 1.86 } else { 1.37 },
        1.15,
        h,
        STUCCO,
        WATER,
    );
    for (x, z) in [(-1.10, 0.41), (1.14, 0.43), (-0.92, -0.42)]
        .into_iter()
        .take(usize::from(level.min(3)))
    {
        amphora(b, v(x, 0.15, z), 1.05, if x < 0.0 { LAPIS } else { JADE });
    }
    if level >= 3 {
        for x in [-0.85, 0.85] {
            for z in [-0.82, -0.28] {
                b.slab(v(x, 0.75, z), v(0.14, 1.20, 0.14), STUCCO);
            }
        }
        slab_roof(b, v(0.0, 1.40, -0.54), 2.0, 0.79, LAPIS);
    }
    let spouts: &[f32] = if level >= 5 { &[-0.46, 0.46] } else { &[0.0] };
    for &x in spouts {
        b.slab(v(x, 0.41, -0.31), v(0.22, 0.53, 0.28), SHALE);
        b.beam(v(x, 0.65, -0.30), v(x, 0.61, -0.02), 0.070, SUN);
        b.beam(v(x, 0.60, -0.02), v(x, h + 0.085, 0.0), 0.038, WATER);
    }
    if level >= 6 {
        b.frustum(v(0.0, 1.49, -0.54), 0.47, 0.46, 0.13, 14, STUCCO);
        dome(b, v(0.0, 1.62, -0.54), 0.59, JADE);
    }
}

pub(super) fn palm(b: &mut Batch, at: Vec3, scale: f32, seed: f32) {
    let head = at + v(0.24, 1.72, -0.14) * scale;
    let branch = |i: usize| {
        let t = i as f32 * 0.20;
        at + v(t * (0.32 - t * 0.08), 1.72 * t, (-0.07 - t * 0.07) * t) * scale
    };
    for i in 0..5 {
        b.beam(
            branch(i),
            branch(i + 1),
            (0.16 - i as f32 * 0.012) * scale,
            shift(SHALE, i as i16 * 5 - 12),
        );
    }
    for leaf in 0..8 {
        let angle = leaf as f32 * std::f32::consts::TAU / 8.0 + seed;
        let wind = v(angle.cos(), 0.0, angle.sin());
        let cross = v(-wind.z, 0.0, wind.x);
        let tip =
            head + wind * (0.83 + (leaf % 3) as f32 * 0.07) * scale + v(0.0, -0.33, 0.0) * scale;
        for (start, end, wide) in [
            (head, head + (wind * 0.32 + Vec3::Y * 0.29) * scale, 0.14),
            (head + (wind * 0.32 + Vec3::Y * 0.29) * scale, tip, 0.12),
        ] {
            for side in [-1.0, 1.0] {
                let wing = (start + end) * 0.5 + cross * (wide * side * scale) - Vec3::Y * 0.033;
                let face = [start, wing, end];
                let tint = shift(
                    [88, 138, 82],
                    (leaf * 13 % 39) as i16 + if side < 0.0 { 14 } else { 0 },
                );
                b.face(&face, tint);
                b.face(&[face[2], face[1], face[0]], shift(tint, -12));
            }
        }
    }
    for i in 0..4 {
        let x = (i as f32 - 1.5) * 0.073;
        b.gem(
            head + v(x, -0.08, 0.12) * scale,
            0.049 * scale,
            0.079 * scale,
            0.048 * scale,
            x,
            SUN,
        );
    }
}

fn dates(b: &mut Batch, level: u8) {
    plinth(b, Vec3::ZERO, 2.73, 2.21, 0.13, [166, 146, 86]);
    b.slab(v(0.0, 0.145, -0.14), v(2.32, 0.035, 1.6), [142, 150, 81]);
    palm(b, v(-0.63, 0.16, -0.27), 0.90, 1.3);
    palm(
        b,
        v(0.48, 0.16, -0.46),
        0.83 + f32::from(level) * 0.036,
        0.27,
    );
    if level >= 2 {
        palm(b, v(-0.11, 0.16, -0.64), 0.68, 1.7);
    }
    if level >= 3 {
        tank(b, v(-0.23, 0.16, 0.46), 1.71, 0.49, 0.20, SANDSTONE, WATER);
    }
    if level >= 4 {
        stoop(b, v(-0.07, 0.0, 0.90), 1.57, 0.15, 0.30, 2, SANDSTONE);
        for x in [-1.10, 1.11] {
            b.slab(v(x, 0.23, 0.16), v(0.22, 0.16, 1.29), SANDSTONE);
        }
    }
    if level >= 5 {
        let p = v(0.83, 0.15, 0.81);
        b.frustum(p, 0.18, 0.27, 0.25, 12, WOOD);
        for i in 0..7 {
            b.gem(
                p + v(
                    (i as f32 - 3.0) * 0.057,
                    0.27,
                    (i % 3) as f32 * 0.078 - 0.09,
                ),
                0.053,
                0.066,
                0.065,
                i as f32,
                SUN,
            );
        }
    }
    if level >= 6 {
        for x in [0.40, 1.21] {
            b.beam(v(x, 0.16, 0.99), v(x, 1.19, 0.99), 0.074, WOOD);
        }
        b.beam(v(1.21, 0.14, 0.41), v(1.21, 1.33, 0.41), 0.074, WOOD);
        panel_roof(b, v(0.82, 1.28, 0.67), 1.03, 0.65, 0.18, JADE);
    }
}

fn bazaar(b: &mut Batch, level: u8) {
    plinth(b, Vec3::ZERO, 2.76, 2.25, 0.14, STUCCO);
    let height = if level >= 6 { 2.02 } else { 1.46 };
    wall(
        b,
        v(0.0, 0.14 + height / 2.0, -0.63),
        v(2.30, height, 0.76),
        SANDSTONE,
    );
    slab_roof(b, v(0.0, height + 0.18, -0.63), 2.49, 0.91, PLUM);
    if level >= 6 {
        for x in [-0.73, 0.0, 0.73] {
            screen(b, v(x, 1.78, -0.212), 0.36, 0.32, PLUM);
        }
    }
    let width = if level >= 4 { 2.2 } else { 1.48 };
    let depth = if level >= 3 { 1.16 } else { 0.82 };
    b.slab(v(0.0, 0.48, 0.61), v(width, 0.12, 0.56), WOOD);
    for x in [-width * 0.42, width * 0.42] {
        b.slab(v(x, 0.28, 0.60), v(0.16, 0.29, 0.42), SHALE);
    }
    for i in 0..(3 + usize::from(level.min(5))) {
        let x = (i as f32 - (2.0 + f32::from(level.min(5))) * 0.5) * width / 8.0;
        let at = v(x, 0.55, 0.67 + (i % 2) as f32 * 0.15);
        b.frustum(at, 0.082, 0.127, 0.068, 10, STUCCO);
        b.gem(
            at + v(0.0, 0.075, 0.0),
            0.109,
            0.101,
            0.10,
            x,
            [SUN, [174, 91, 66], [137, 143, 74], PLUM][i % 4],
        );
    }
    if level >= 5 {
        for x in [-0.65, 0.65] {
            arch(b, v(x, 0.14, 0.27), false, 0.67, 0.34, STUCCO);
        }
    }
    cloth_roof(
        b,
        v(0.0, 1.44, 0.36),
        width + 0.31,
        depth,
        [PLUM, [231, 184, 132]],
    );
}

fn cloth_roof(b: &mut Batch, at: Vec3, width: f32, depth: f32, color: [Tint; 2]) {
    for side in [-1.0, 1.0] {
        let post = at + v(side * (width * 0.47), -0.18, depth * 0.5);
        b.beam(v(post.x, 0.14, post.z), post, 0.058, WOOD);
        b.beam(post, post + v(0.0, 0.31, -depth), 0.045, WOOD);
    }
    for n in 0..9 {
        let width = width / 9.0;
        let x = at.x + (n as f32 - 4.0) * width;
        let p = v(x, at.y, at.z);
        b.block(
            p,
            v(width * 0.986, 0.034, depth),
            Quat::from_rotation_x(0.265),
            color[n % 2],
        );
        b.slab(
            p + v(0.0, -0.18, depth * 0.487),
            v(width * 0.992, 0.096, 0.029),
            shift(color[n % 2], -8),
        );
    }
}

fn caravanserai(b: &mut Batch, level: u8) {
    plinth(b, Vec3::ZERO, 2.81, 2.25, 0.17, SHALE);
    let h = 1.75 + f32::from(level.saturating_sub(2)) * 0.236;
    wall(
        b,
        v(0.0, 0.17 + h / 2.0, -0.45),
        v(2.49, h, 1.33),
        SANDSTONE,
    );
    let arch_p = v(0.0, 0.17, 0.29);
    arch(b, arch_p, true, 1.03, 0.46, STUCCO);
    arch(b, arch_p + v(0.0, 0.0, 0.036), false, 1.03, 0.333, LAPIS);
    slab_roof(b, v(0.0, h + 0.21, -0.43), 2.66, 1.47, JADE);
    stoop(
        b,
        v(0.0, 0.0, 0.88),
        if level >= 2 { 1.61 } else { 0.90 },
        0.175,
        0.35,
        3,
        SANDSTONE,
    );
    for x in [-0.87, 0.87] {
        screen(b, v(x, 0.94, 0.26), 0.31, 0.53, WOOD);
        if level >= 3 {
            arch(b, v(x, 0.17, 0.88), false, 0.63, 0.23, STUCCO);
            slab_roof(b, v(x, 1.17, 0.61), 0.77, 0.67, PLUM);
        }
        if level >= 4 {
            arch(b, v(x * 0.94, h - 0.69, 0.27), true, 0.35, 0.18, STUCCO);
        }
        if level >= 6 {
            b.frustum(v(x, h + 0.30, -0.45), 0.32, 0.32, 0.12, 12, SANDSTONE);
            dome(
                b,
                v(x, h + 0.42, -0.45),
                0.43,
                if x < 0.0 { JADE } else { LAPIS },
            );
        }
    }
    if level >= 5 {
        let y = h - 0.34;
        b.slab(v(0.0, y, 0.284), v(1.00, 0.155, 0.069), LAPIS);
        for i in -3..=3 {
            b.gem(
                v(i as f32 * 0.12, y, 0.33),
                0.047,
                0.052,
                0.025,
                i as f32,
                SUN,
            );
        }
    }
}

fn qanat(b: &mut Batch, level: u8) {
    plinth(b, Vec3::ZERO, 2.70, 2.26, 0.12, SHALE);
    let h = 0.68 + f32::from(level) * 0.21;
    let shaft = v(-0.55, 0.12, -0.51);
    b.frustum(shaft, 0.70, 0.56, h, 12, SANDSTONE);
    for p in [0.13, 0.37, 0.62, 0.96] {
        b.frustum(
            shaft + Vec3::Y * (h * p),
            0.72 - p * 0.13,
            0.72 - p * 0.13,
            0.046,
            12,
            sha(p),
        );
    }
    let primary = v(-0.51, 0.14, 0.51);
    tank(b, primary, 0.76, 1.37, 0.25, STUCCO, WATER);
    b.beam(v(-0.54, 0.61, -0.24), v(-0.52, 0.50, 0.18), 0.13, LAPIS);
    b.beam(v(-0.52, 0.50, 0.18), v(-0.52, 0.35, 0.24), 0.041, WATER);
    if level >= 3 {
        tank(b, v(0.60, 0.14, 0.53), 0.85, 1.33, 0.24, STUCCO, WATER);
    }
    if level >= 4 {
        arch(b, v(0.54, 0.13, -0.49), false, 0.88, 0.48, SANDSTONE);
        slab_roof(b, v(0.59, 1.70, -0.47), 1.47, 0.71, LAPIS);
    }
    if level >= 5 {
        tank(b, v(0.55, 0.14, -0.48), 0.74, 0.61, 0.32, STUCCO, WATER);
    }
    if level >= 6 {
        b.frustum(shaft + Vec3::Y * (h + 0.02), 0.57, 0.57, 0.14, 12, STUCCO);
        dome(b, shaft + Vec3::Y * (h + 0.15), 0.51, JADE);
    }
}

fn sha(t: f32) -> Tint {
    if t > 0.8 { STUCCO } else { shift(SHALE, 24) }
}

fn glassworks(b: &mut Batch, level: u8) {
    let tint = LAPIS;
    let h = 1.33 + f32::from(level.saturating_sub(4)) * 0.27;
    plinth(b, Vec3::ZERO, 2.75, 2.24, 0.15, SANDSTONE);
    wall(
        b,
        v(-0.26, 0.15 + h / 2.0, -0.49),
        v(2.07, h, 1.11),
        SANDSTONE,
    );
    slab_roof(b, v(-0.24, h + 0.19, -0.48), 2.28, 1.29, tint);
    arch(b, v(-0.59, 0.15, 0.12), true, 0.72, 0.36, STUCCO);
    if level >= 5 {
        screen(b, v(0.36, h - 0.18, 0.095), 0.53, 0.44, tint);
    }
    let work = v(0.80, 0.15, 0.38);
    let diameter = if level >= 4 { 0.41 } else { 0.32 };
    b.frustum(work, diameter, diameter + 0.027, 0.52, 12, SHALE);
    dome(b, work + Vec3::Y * 0.51, diameter, [176, 101, 71]);
    b.slab(
        work + v(0.0, 0.32, diameter * 0.98),
        v(0.24, 0.22, 0.049),
        WOOD,
    );
    b.gem(
        work + v(0.0, 0.28, diameter + 0.028),
        0.082,
        0.093,
        0.038,
        0.0,
        SUN,
    );
    if level >= 2 {
        tank(b, v(0.55, 0.15, 0.91), 0.63, 0.54, 0.19, STUCCO, WATER);
    }
    let shelves: &[f32] = if level >= 3 { &[0.34, 0.65] } else { &[0.42] };
    for &y in shelves {
        b.slab(v(-0.62, y, 0.70), v(1.31, 0.079, 0.47), WOOD);
        for i in 0..4 {
            amphora(
                b,
                v(-1.10 + i as f32 * 0.31, y + 0.039, 0.78),
                0.48 + i as f32 * 0.055,
                [LAPIS, PLUM, JADE, SUN][i],
            );
        }
    }
    for x in [-1.13, -0.13] {
        b.beam(v(x, 0.15, 0.76), v(x, 0.48, 0.76), 0.073, WOOD);
    }
    if level >= 4 {
        let [x0, x1] = if level >= 6 {
            [-1.24, 1.19]
        } else {
            [-1.24, 0.11]
        };
        let y = if level >= 6 { 1.56 } else { 1.30 };
        b.beam(v(x0, 0.15, 1.03), v(x0, y, 1.03), 0.075, WOOD);
        b.beam(v(x1, 0.15, 1.03), v(x1, y, 1.03), 0.075, WOOD);
        panel_roof(
            b,
            v((x0 + x1) * 0.5, y + 0.066, 0.59),
            x1 - x0 + 0.12,
            0.95,
            0.16,
            tint,
        );
    }
}

fn perfumer(b: &mut Batch, level: u8) {
    let copper = [176, 111, 75];
    let rose = [218, 157, 142];
    let y = 1.41 + f32::from(level.saturating_sub(4)) * 0.22;
    plinth(b, Vec3::ZERO, 2.74, 2.24, 0.15, SANDSTONE);
    let at = v(-0.44, 0.15, -0.21);
    b.frustum(at, 0.45, 0.47, 0.61, 14, SHALE);
    b.frustum(at + Vec3::Y * 0.59, 0.47, 0.51, 0.08, 14, STUCCO);
    dome(b, at + Vec3::Y * 0.67, 0.49, copper);
    arch(b, v(-0.44, 0.17, 0.268), true, 0.22, 0.15, SHALE);
    b.beam(
        at + v(0.0, 1.05, 0.01),
        at + v(0.16, 1.36, 0.10),
        0.071,
        copper,
    );
    if level >= 2 {
        b.beam(
            at + v(0.16, 1.36, 0.10),
            at + v(0.55, 0.83, 0.27),
            0.058,
            copper,
        );
        amphora(b, at + v(0.55, 0.026, 0.37), 1.03, JADE);
    }
    b.slab(v(0.77, 0.48, 0.49), v(0.79, 0.10, 0.66), WOOD);
    for x in [0.45, 1.09] {
        b.beam(v(x, 0.15, 0.50), v(x, 0.43, 0.50), 0.095, WOOD);
    }
    for i in 0..3 + usize::from(level >= 3) + usize::from(level >= 5) {
        let x = 0.51 + (i % 3) as f32 * 0.22;
        let z = 0.40 + (i / 3) as f32 * 0.27;
        amphora(
            b,
            v(x, 0.53, z),
            0.53 + (i % 3) as f32 * 0.08,
            [PLUM, JADE, rose, SUN, LAPIS][i],
        );
    }
    if level >= 3 {
        for x in [0.56, 1.06] {
            flowerpot(b, v(x, 0.16, 0.97), 0.60, rose);
        }
    }
    if level >= 4 {
        for x in [-1.13, 1.13] {
            for z in [-0.83, -0.12] {
                b.beam(v(x, 0.15, z), v(x, y - 0.07, z), 0.113, SHALE);
            }
        }
        slab_roof(b, v(0.0, y, -0.48), 2.56, 0.98, PLUM);
    }
    if level >= 5 {
        b.frustum(v(0.0, y + 0.10, -0.48), 0.53, 0.53, 0.11, 14, SANDSTONE);
        dome(b, v(0.0, y + 0.20, -0.48), 0.60, PLUM);
    }
    if level >= 6 {
        arc_pillars(b, y, copper);
        cloth_roof(b, v(0.0, y - 0.15, 0.42), 2.38, 1.18, [PLUM, rose]);
    }
}

fn arc_pillars(b: &mut Batch, height: f32, copper: Tint) {
    for x in [-1.04, 1.04] {
        b.beam(v(x, 0.15, 0.98), v(x, height - 0.21, 0.98), 0.119, SHALE);
        b.slab(v(x, 0.24, 0.98), v(0.25, 0.18, 0.24), SANDSTONE);
        b.beam(
            v(x, height - 0.44, 0.89),
            v(x * 0.82, height - 0.26, 0.73),
            0.066,
            copper,
        );
    }
}

fn moon_garden(b: &mut Batch, level: u8) {
    plinth(b, Vec3::ZERO, 2.76, 2.23, 0.12, [158, 151, 103]);
    let w = if level >= 5 { 1.87 } else { 1.07 };
    tank(b, v(0.0, 0.12, 0.47), w, 0.80, 0.23, STUCCO, WATER);
    palm(b, v(-0.83, 0.12, -0.64), 0.68, 0.34);
    for i in 0..2 + level.min(4) {
        let x = -0.91 + f32::from(i) * 0.37;
        let z = if i % 2 == 0 { -0.54 } else { 0.09 };
        flowerpot(
            b,
            v(x, 0.12, z),
            0.70 + f32::from(level) * 0.05,
            [236, 182, 184],
        );
    }
    let y = 1.49 + f32::from(level >= 5) * 0.14;
    if level >= 3 {
        for x in [-1.05, 1.05] {
            for z in [-0.79, -0.28] {
                b.slab(
                    v(x, (y + 0.12) * 0.5, z),
                    v(0.13, y - 0.12, 0.14),
                    SANDSTONE,
                );
            }
        }
        slab_roof(b, v(0.0, y, -0.55), 2.43, 0.69, LAPIS);
        bench(b, v(0.11, 0.12, -0.48), 1.10);
    }
    if level >= 4 {
        for x in [-1.13, 1.13] {
            b.frustum(v(x, 0.12, 0.54), 0.16, 0.09, 0.48, 10, STUCCO);
            b.gem(v(x, 0.67, 0.54), 0.12, 0.16, 0.125, x, SUN);
        }
    }
    if level >= 6 {
        b.frustum(v(0.0, y + 0.10, -0.53), 0.59, 0.57, 0.12, 12, SANDSTONE);
        dome(b, v(0.0, y + 0.22, -0.53), 0.55, PLUM);
    }
}

fn carpet_hall(b: &mut Batch, level: u8) {
    let h = 1.44 + f32::from(level.saturating_sub(4)) * 0.34;
    plinth(b, Vec3::ZERO, 2.74, 2.24, 0.15, SANDSTONE);
    wall(
        b,
        v(0.0, 0.15 + h * 0.50, -0.54),
        v(2.43, h, 1.06),
        [230, 172, 134],
    );
    slab_roof(b, v(0.0, h + 0.18, -0.53), 2.60, 1.26, PLUM);
    let beam = if level >= 4 { 1.42 } else { 1.15 };
    for x in [-1.10, 1.10] {
        b.beam(v(x, 0.15, 0.74), v(x, beam, 0.74), 0.093, WOOD);
    }
    b.beam(v(-1.20, beam, 0.74), v(1.20, beam, 0.74), 0.059, WOOD);
    for i in 0..if level >= 2 { 4 } else { 3 } {
        let x = -0.77 + i as f32 * 0.48;
        let color = [PLUM, LAPIS, [161, 86, 66], JADE][i];
        let height = 0.57 + (i % 3) as f32 * 0.096;
        let at = v(x, beam - height * 0.5 - 0.015, 0.76);
        b.slab(at, v(0.391, height, 0.051), color);
        for y in [-height * 0.40, height * 0.40] {
            b.slab(at + v(0.0, y, 0.031), v(0.37, 0.038, 0.018), SUN);
        }
        for j in -1..=1 {
            b.block(
                at + v(0.0, j as f32 * 0.14, 0.034),
                v(0.10, 0.10, 0.020),
                Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                STUCCO,
            );
        }
    }
    if level >= 3 {
        cloth_roof(b, v(0.0, beam + 0.16, 0.58), 2.50, 1.04, [LAPIS, STUCCO]);
    }
    if level >= 5 {
        for x in [-0.72, 0.72] {
            screen(b, v(x, h - 0.05, 0.02), 0.50, 0.41, JADE);
        }
    }
    if level >= 6 {
        stoop(b, v(0.0, 0.0, 0.91), 1.76, 0.15, 0.32, 3, SANDSTONE);
        for x in [-1.07, 1.08] {
            b.frustum(v(x, h + 0.26, -0.53), 0.19, 0.18, 0.13, 10, PLUM);
            b.gem(v(x, h + 0.42, -0.53), 0.088, 0.164, 0.08, x, SUN);
        }
    }
}

pub(super) fn paving(i: usize, row: usize, chapter: usize) -> Tint {
    let colors = [
        [228, 186, 128],
        [216, 169, 128],
        [221, 187, 148],
        [202, 159, 114],
        [217, 176, 136],
        [225, 189, 155],
    ];
    shift(
        colors[(chapter * 7 + row / 4) % colors.len()],
        ((i * 23 + row * 19 + chapter) % 23) as i16 - 10,
    )
}

pub(super) fn garden(b: &mut Batch, at: Vec3, chapter: usize, grain: &mut Grain) {
    let cap = 0.24 + grain.span(0.04, 0.12);
    for i in 0..3 {
        let p = at + v(i as f32 * 0.18 - 0.16, 0.0, grain.span(-0.21, 0.22));
        b.gem(
            p + Vec3::Y * 0.085,
            cap * 0.70,
            cap * 0.55,
            cap * 0.64,
            i as f32,
            [158, 149, 85],
        );
        if i % 2 == chapter % 2 {
            b.gem(
                p + Vec3::Y * 0.20,
                0.048,
                0.061,
                0.056,
                i as f32,
                [205, 145, 108],
            );
        }
    }
}

pub(super) fn bank(b: &mut Batch, wide: f32, chapter: usize, grain: &mut Grain) {
    let stage = harbor_stage(chapter);
    let style = [JADE, LAPIS, PLUM, [165, 88, 66]][stage % 4];
    for i in 0..(8 + stage % 6) {
        let x = (i as f32 / (7 + stage % 6) as f32 - 0.5) * (wide * 1.78);
        let p = v(x, 0.03, grain.span(-5.72, -4.70));
        if i % 3 == stage % 3 {
            let height = grain.span(0.64, 1.25);
            wall(
                b,
                p + Vec3::Y * (height / 2.0),
                v(1.18, height, 0.83),
                paving(i, 5, chapter),
            );
            slab_roof(b, p + Vec3::Y * height, 1.30, 0.91, style);
            if (i + stage).is_multiple_of(2) {
                dome(b, p + Vec3::Y * (height + 0.09), 0.38, style);
            }
        } else {
            palm(b, p, grain.span(0.57, 1.10), i as f32);
        }
    }
    for (i, x) in [-wide + 1.05, wide - 0.85].into_iter().enumerate() {
        let y = 0.97 + 0.10 * (stage % 5) as f32;
        let at = v(x, 0.02, 3.8 + i as f32 * 0.18);
        plinth(b, at, 1.55, 0.81, 0.17, SANDSTONE);
        arch(b, at + Vec3::Y * 0.16, false, y * 0.70, 0.42, STUCCO);
        slab_roof(b, at + Vec3::Y * (y * 0.70 + 0.76), 1.34, 0.56, style);
        dome(b, at + Vec3::Y * (y * 0.70 + 0.86), 0.40, style);
    }
    if stage >= 4 {
        for (x, z) in [(-wide * 0.45, 4.40), (-wide * 0.45 + 0.76, 4.18)] {
            b.gem(v(x, 0.45, z), 0.17, 0.39, 0.46, 0.22, [166, 119, 75]);
            b.gem(
                v(x, 0.73, z - 0.41),
                0.112,
                0.24,
                0.132,
                0.31,
                [184, 132, 89],
            );
            b.gem(v(x, 0.90, z - 0.57), 0.117, 0.09, 0.21, x, [184, 134, 97]);
            for side in [-1.0, 1.0] {
                for offset in [-0.24, 0.26] {
                    b.beam(
                        v(x + side * 0.11, 0.028, z + offset),
                        v(x + side * 0.12, 0.29, z + offset),
                        0.050,
                        SHALE,
                    );
                }
            }
            b.slab(v(x, 0.56, z), v(0.38, 0.15, 0.42), style);
            crate_box(b, v(x + 0.23, 0.04, z + 0.59), v(0.34, 0.26, 0.31), WOOD);
        }
    }
}

pub(super) fn distant(chapter: usize) -> Mesh {
    let mut b = Batch::default();
    let mut grain = Grain::new(27 + chapter as u32 * 441);
    let tint = paving(4, 5, chapter);
    for n in 0..12 {
        let x = -53.0 + n as f32 * 9.7;
        let z = -21.5 - grain.span(0.0, 8.2);
        let y = grain.span(1.8, 4.9);
        b.gem(
            v(x, -1.18, z),
            grain.span(6.0, 10.8),
            y,
            grain.span(5.0, 8.1),
            n as f32,
            shift(tint, -25 + n as i16 * 2),
        );
        if n % 3 == chapter % 3 {
            palm(&mut b, v(x, -0.43, z + 6.2), 0.83, n as f32);
        }
    }
    b.mesh()
}

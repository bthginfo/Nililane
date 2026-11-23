use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::campaign::Region;

use super::{
    mesh::{Batch, Tint, shift, v},
    upgrades::crate_box,
};

const LIMESTONE: Tint = [227, 207, 167];
const BRONZE: Tint = [100, 147, 123];
const GOLD: Tint = [218, 171, 88];

pub(super) fn landmark(region: Region, tier: u8) -> Mesh {
    let mut mesh = Batch::default();
    match region {
        Region::Greece => agora(&mut mesh, tier),
        Region::Rome => amphitheater(&mut mesh, tier),
        Region::Japan => pagoda(&mut mesh, tier),
        Region::Nordic => thinghall(&mut mesh, tier),
        Region::Oasis => palace(&mut mesh, tier),
    }
    mesh.mesh()
}

fn plinth(b: &mut Batch, width: f32, depth: f32, color: Tint) {
    for i in 0..3 {
        let cut = i as f32 * 0.105;
        b.slab(
            v(0.0, 0.037 + cut * 0.53, 0.08 - cut * 0.30),
            v(width - cut * 0.95, 0.083, depth - cut * 0.90),
            shift(color, (i - 1) * 8),
        );
    }
}

fn post(b: &mut Batch, at: Vec3, radius: f32, height: f32, stem: Tint, foot: Tint) {
    b.slab(
        at + Vec3::Y * 0.047,
        v(radius * 2.9, 0.09, radius * 2.9),
        foot,
    );
    b.frustum(
        at + Vec3::Y * 0.09,
        radius,
        radius * 0.84,
        height - 0.16,
        9,
        stem,
    );
    b.frustum(
        at + Vec3::Y * (height - 0.07),
        radius * 0.84,
        radius * 1.22,
        0.055,
        9,
        shift(foot, 12),
    );
    b.slab(
        at + Vec3::Y * height,
        v(radius * 2.6, 0.062, radius * 2.6),
        foot,
    );
}

fn roof(b: &mut Batch, width: f32, depth: f32, y: f32, pitch: f32, tiles: Tint, edge: Tint) {
    for side in [-1.0, 1.0] {
        let corners = [
            v(-width, y + pitch, 0.0),
            v(-width, y, side * depth),
            v(width, y, side * depth),
            v(width, y + pitch, 0.0),
        ];
        if side > 0.0 {
            b.face(&corners, tiles);
        } else {
            b.face(
                &[corners[3], corners[2], corners[1], corners[0]],
                shift(tiles, -12),
            );
        }
        b.beam(corners[1], corners[2], 0.079, edge);
        for tile in 0..14 {
            let x = -width + (tile as f32 + 0.5) / 14.0 * width * 2.0;
            b.beam(
                v(x, y + pitch + 0.017, 0.0),
                v(x, y + 0.025, side * depth),
                0.037,
                shift(tiles, 9 + (tile % 4) as i16 * 5),
            );
        }
    }
    b.beam(
        v(-width - 0.02, y + pitch + 0.027, 0.0),
        v(width + 0.02, y + pitch + 0.027, 0.0),
        0.064,
        edge,
    );
}

fn agora(b: &mut Batch, tier: u8) {
    plinth(b, 3.34, 2.78, LIMESTONE);
    let back = -0.66;
    for i in 0..5 {
        let x = -1.24 + i as f32 * 0.62;
        post(b, v(x, 0.15, back), 0.085, 1.06, [231, 218, 188], LIMESTONE);
        if tier >= 3 && i % 2 == 0 {
            b.slab(
                v(x, 1.30, back + 0.095),
                v(0.16, 0.073, 0.045),
                [64, 135, 155],
            );
        }
    }
    b.slab(v(0.0, 1.28, back), v(2.78, 0.15, 0.40), LIMESTONE);
    for i in 0..10 {
        b.slab(
            v(-1.3 + i as f32 * 0.27, 1.325, back + 0.225),
            v(0.167, 0.081, 0.039),
            [174, 116, 86],
        );
    }
    roof(b, 1.46, 0.49, 1.37, 0.29, [179, 90, 61], [230, 153, 92]);
    let center = v(0.0, 0.16, 0.48);
    b.frustum(center, 0.50, 0.46, 0.139, 14, LIMESTONE);
    b.frustum(
        center + Vec3::Y * 0.139,
        0.401,
        0.401,
        0.016,
        14,
        [66, 159, 178],
    );
    if tier >= 2 {
        b.frustum(
            center + Vec3::Y * 0.15,
            0.19,
            0.17,
            0.26,
            9,
            shift(LIMESTONE, -10),
        );
        b.frustum(center + Vec3::Y * 0.40, 0.11, 0.16, 0.35, 8, BRONZE);
        b.gem(
            center + Vec3::Y * 0.83,
            0.105,
            0.132,
            0.095,
            0.33,
            shift(BRONZE, 10),
        );
        b.beam(
            center + v(-0.11, 0.67, 0.02),
            center + v(-0.29, 0.84, 0.05),
            0.072,
            BRONZE,
        );
        b.beam(
            center + v(0.12, 0.66, 0.02),
            center + v(0.19, 0.49, 0.07),
            0.066,
            BRONZE,
        );
    }
    for side in [-1.0, 1.0] {
        if tier >= 3 {
            b.slab(
                v(side * 1.06, 0.31, 0.46),
                v(0.68, 0.091, 0.34),
                [184, 136, 95],
            );
            b.slab(v(side * 1.06, 0.23, 0.46), v(0.51, 0.16, 0.26), LIMESTONE);
        }
        if tier >= 4 {
            b.frustum(
                v(side * 1.26, 0.15, 1.04),
                0.15,
                0.135,
                0.24,
                10,
                [200, 124, 76],
            );
            b.gem(
                v(side * 1.26, 0.51, 1.04),
                0.22,
                0.17,
                0.18,
                0.3,
                [109, 148, 89],
            );
        }
        if tier >= 5 {
            b.beam(
                v(side * 0.98, 0.19, -0.01),
                v(side * 0.98, 1.82, -0.01),
                0.039,
                GOLD,
            );
            b.face(
                &[
                    v(side * 0.98, 1.76, -0.01),
                    v(side * 1.33, 1.63, -0.045),
                    v(side * 0.98, 1.37, 0.0),
                ],
                [64, 131, 162],
            );
        }
    }
    if tier >= 6 {
        b.gem(v(0.0, 1.72, 0.08), 0.19, 0.10, 0.18, 0.58, GOLD);
    }
}

fn amphitheater(b: &mut Batch, tier: u8) {
    let sandstone = [218, 177, 128];
    plinth(b, 3.31, 2.92, sandstone);
    let circle =
        |angle: f32, radius: f32, y: f32| v(radius * angle.cos(), y, radius * angle.sin() * 0.80);
    b.frustum(v(0.0, 0.15, 0.0), 1.03, 1.03, 0.033, 24, [204, 152, 97]);
    let rows = 3 + (tier.saturating_sub(1) / 2) as usize;
    for row in 0..rows {
        let radius = 0.79 + row as f32 * 0.117;
        let height = 0.19 + row as f32 * 0.123;
        for section in 0..20 {
            let from = section as f32 / 20.0 * TAU;
            let to = (section + 1) as f32 / 20.0 * TAU;
            b.face(
                &[
                    circle(from, radius, height),
                    circle(from, radius + 0.134, height),
                    circle(to, radius + 0.134, height),
                    circle(to, radius, height),
                ],
                shift(sandstone, (row * 7) as i16),
            );
            b.face(
                &[
                    circle(to, radius, height - 0.10),
                    circle(from, radius, height - 0.10),
                    circle(from, radius, height),
                    circle(to, radius, height),
                ],
                shift(sandstone, -17),
            );
        }
    }
    let piers = 14;
    let gallery = 0.89 + f32::from(tier) * 0.055;
    for i in 0..piers {
        let a = i as f32 / piers as f32 * TAU;
        let next = (i + 1) as f32 / piers as f32 * TAU;
        let at = circle(a, 1.47, 0.15);
        post(
            b,
            at,
            0.079,
            gallery - 0.23,
            sandstone,
            shift(sandstone, 11),
        );
        let end = circle(next, 1.47, 0.15);
        let mid = at.lerp(end, 0.50) + Vec3::Y * gallery;
        for (from, to) in [(0.0, 0.23), (0.23, 0.5), (0.5, 0.77), (0.77, 1.0)] {
            let point =
                |f: f32| at.lerp(end, f) + Vec3::Y * (gallery - 0.19 + (f * PI).sin() * 0.19);
            b.beam(point(from), point(to), 0.085, shift(sandstone, 15));
        }
        b.beam(
            at + Vec3::Y * (gallery + 0.12),
            end + Vec3::Y * (gallery + 0.12),
            0.081,
            sandstone,
        );
        if tier >= 3 && i % 2 == 0 {
            let tip = circle(a + 0.18, 1.02, gallery + 0.27);
            b.face(
                &[
                    at + Vec3::Y * (gallery + 0.04),
                    tip,
                    end + Vec3::Y * (gallery + 0.04),
                ],
                if i % 4 == 0 {
                    [163, 72, 63]
                } else {
                    [237, 201, 143]
                },
            );
        }
        if tier >= 5 {
            b.gem(
                mid + v(0.0, 0.19, 0.0),
                0.084,
                0.063,
                0.086,
                a,
                [182, 112, 64],
            );
        }
    }
    if tier >= 4 {
        for x in [-0.20, 0.20] {
            post(b, v(x, 0.15, 1.19), 0.092, 0.83, sandstone, LIMESTONE);
        }
        b.slab(v(0.0, 1.01, 1.22), v(0.60, 0.125, 0.29), LIMESTONE);
    }
    if tier >= 6 {
        b.beam(v(0.0, 1.05, 1.22), v(0.0, 1.84, 1.22), 0.045, GOLD);
        b.face(
            &[v(0.0, 1.79, 1.22), v(0.40, 1.63, 1.24), v(0.0, 1.46, 1.22)],
            [151, 64, 63],
        );
    }
}

fn pagoda(b: &mut Batch, tier: u8) {
    let wood = [123, 74, 60];
    plinth(b, 3.09, 2.79, [182, 181, 155]);
    let floors = 2 + usize::from(tier >= 3) + usize::from(tier >= 5);
    for floor in 0..floors {
        let y = 0.15 + floor as f32 * 0.50;
        let wing = 0.81 - 0.115 * floor as f32;
        b.slab(
            v(0.0, y + 0.23, 0.0),
            v(wing * 1.62, 0.40, wing * 1.36),
            [224, 206, 163],
        );
        for x in [-wing * 0.78, wing * 0.78] {
            for z in [-wing * 0.64, wing * 0.64] {
                b.slab(v(x, y + 0.232, z), v(0.088, 0.456, 0.088), wood);
            }
        }
        b.slab(
            v(0.0, y + 0.21, wing * 0.685),
            v(wing * 0.40, 0.24, 0.034),
            [61, 108, 123],
        );
        for x in [-0.14, 0.0, 0.14] {
            b.slab(
                v(x * wing, y + 0.21, wing * 0.711),
                v(0.014, 0.241, 0.023),
                wood,
            );
        }
        roof(
            b,
            wing + 0.18,
            wing * 0.79 + 0.14,
            y + 0.417,
            0.22,
            [56, 82, 101],
            [102, 132, 136],
        );
        for x in [-1.0, 1.0] {
            b.beam(
                v(wing * x, y + 0.43, wing * 0.77),
                v((wing + 0.21) * x, y + 0.50, wing * 0.80),
                0.056,
                wood,
            );
        }
    }
    let stem = floors as f32 * 0.50 + 0.29;
    b.beam(
        v(0.0, stem - 0.12, 0.0),
        v(0.0, stem + 0.40, 0.0),
        0.047,
        GOLD,
    );
    for i in 0..(2 + tier / 2) {
        let y = stem + f32::from(i) * 0.074;
        b.frustum(
            v(0.0, y, 0.0),
            0.112 - f32::from(i) * 0.013,
            0.054,
            0.037,
            10,
            GOLD,
        );
    }
    if tier >= 2 {
        for side in [-1.0, 1.0] {
            let at = v(side * 1.04, 0.17, 0.91);
            post(b, at, 0.077, 0.40, [174, 171, 144], [203, 196, 167]);
            b.gem(
                at + Vec3::Y * 0.51,
                0.154,
                0.117,
                0.148,
                0.35,
                [109, 127, 119],
            );
            if tier >= 4 {
                b.frustum(v(side * 1.36, 0.14, -0.21), 0.15, 0.185, 0.18, 8, wood);
                b.beam(
                    v(side * 1.36, 0.27, -0.21),
                    v(side * 1.37, 0.80, -0.22),
                    0.058,
                    wood,
                );
                b.gem(
                    v(side * 1.31, 0.85, -0.23),
                    0.24,
                    0.20,
                    0.24,
                    0.4,
                    [199, 134, 153],
                );
            }
        }
    }
}

fn thinghall(b: &mut Batch, tier: u8) {
    let wood = [141, 91, 57];
    plinth(b, 3.26, 2.70, [163, 158, 133]);
    b.slab(v(0.0, 0.60, -0.03), v(2.65, 0.91, 1.77), [158, 104, 63]);
    for z in [-0.93, 0.88] {
        for i in 0..9 {
            let x = i as f32 * 0.296 - 1.18;
            b.slab(
                v(x, 0.61, z),
                v(0.06, 0.92, 0.067),
                shift(wood, -16 + i as i16 * 3),
            );
        }
    }
    roof(b, 1.53, 1.06, 1.07, 0.83, [67, 104, 93], [144, 111, 60]);
    b.slab(v(0.0, 0.54, 0.955), v(0.62, 0.78, 0.083), [86, 66, 54]);
    for x in [-0.40, 0.40] {
        post(b, v(x, 0.15, 1.07), 0.088, 0.83, wood, GOLD);
    }
    for side in [-1.0, 1.0] {
        let x = side * 1.22;
        if tier >= 2 {
            let colors = [[176, 117, 66], [83, 132, 133], [189, 95, 78]];
            for (i, color) in colors
                .into_iter()
                .take(usize::from(tier.min(4) - 1))
                .enumerate()
            {
                let at = v(x - side * i as f32 * 0.27, 0.76, 0.956);
                b.gem(at, 0.111, 0.111, 0.047, 0.69, color);
                b.gem(at + Vec3::Z * 0.047, 0.033, 0.033, 0.019, 0.58, GOLD);
            }
        }
        if tier >= 4 {
            b.beam(
                v(side * 1.40, 1.85, 0.0),
                v(side * 1.62, 2.08, 0.0),
                0.10,
                wood,
            );
            b.gem(v(side * 1.61, 2.12, 0.0), 0.092, 0.075, 0.082, 0.27, GOLD);
        }
        if tier >= 5 {
            b.gem(
                v(side * 0.94, 0.39, 1.11),
                0.16,
                0.23,
                0.10,
                0.27,
                [168, 178, 166],
            );
            b.beam(
                v(side * 0.94, 0.27, 1.21),
                v(side * 0.94, 0.50, 1.18),
                0.025,
                [98, 129, 125],
            );
        }
    }
    if tier >= 6 {
        b.slab(v(0.0, 0.95, 1.13), v(0.57, 0.15, 0.109), GOLD);
    }
}

fn palace(b: &mut Batch, tier: u8) {
    let sand = [218, 168, 109];
    let turquoise = [57, 141, 145];
    plinth(b, 3.31, 2.81, sand);
    b.slab(v(0.0, 0.63, -0.26), v(2.20, 0.96, 1.39), sand);
    b.slab(v(0.0, 1.10, -0.26), v(2.32, 0.16, 1.51), shift(sand, 13));
    for x in [-0.80, 0.0, 0.80] {
        for dir in [-1.0, 1.0] {
            post(
                b,
                v(x + dir * 0.24, 0.15, 0.54),
                0.079,
                0.76,
                sand,
                shift(sand, 14),
            );
        }
        for i in 0..9 {
            let theta = i as f32 * PI / 9.0;
            let next = (i + 1) as f32 * PI / 9.0;
            let point = |a: f32| v(x + a.cos() * 0.245, 0.78 + a.sin() * 0.26, 0.56);
            b.beam(
                point(theta),
                point(next),
                0.077,
                if i % 2 == 0 {
                    shift(sand, 17)
                } else {
                    [191, 117, 89]
                },
            );
        }
        b.slab(v(x, 0.49, 0.447), v(0.36, 0.65, 0.035), [119, 91, 78]);
    }
    let height = 1.18;
    for (index, (low, high, h)) in [
        (0.60, 0.68, 0.16),
        (0.68, 0.57, 0.20),
        (0.57, 0.36, 0.21),
        (0.36, 0.052, 0.16),
    ]
    .into_iter()
    .enumerate()
    {
        b.frustum(
            v(0.0, height + [0.0, 0.16, 0.36, 0.57][index], -0.26),
            low,
            high,
            h,
            12,
            shift(turquoise, index as i16 * 11),
        );
    }
    if tier >= 2 {
        b.frustum(v(0.0, height + 0.72, -0.26), 0.048, 0.048, 0.15, 8, GOLD);
        b.gem(
            v(0.0, height + 0.87, -0.26),
            0.092,
            0.101,
            0.082,
            0.62,
            GOLD,
        );
    }
    for side in [-1.0, 1.0] {
        if tier >= 3 {
            let x = side * 1.19;
            b.frustum(v(x, 0.17, 0.72), 0.21, 0.23, 0.28, 9, [169, 97, 72]);
            b.gem(v(x, 0.54, 0.73), 0.21, 0.15, 0.21, 0.42, [114, 157, 100]);
        }
        if tier >= 4 {
            post(
                b,
                v(side * 1.25, 0.15, -0.72),
                0.13,
                1.38,
                sand,
                shift(sand, 14),
            );
            b.frustum(v(side * 1.25, 1.55, -0.72), 0.19, 0.03, 0.32, 10, turquoise);
        }
    }
    if tier >= 5 {
        b.slab(v(0.0, 0.172, 0.91), v(0.83, 0.033, 0.71), [161, 79, 95]);
        for n in 0..5 {
            b.gem(
                v((n as f32 - 2.0) * 0.128, 0.198, 0.91),
                0.055,
                0.014,
                0.103,
                0.31,
                GOLD,
            );
        }
    }
    if tier >= 6 {
        b.gem(v(0.0, 1.19, 0.53), 0.143, 0.143, 0.057, 0.65, GOLD);
    }
}

pub(super) fn jetty(region: Region) -> Mesh {
    let cloth = [
        [57, 132, 156],
        [169, 72, 61],
        [77, 120, 158],
        [82, 131, 116],
        [179, 111, 142],
    ][region as usize];
    let wood = [151, 104, 66];
    let mut b = Batch::default();
    for n in 0..17 {
        let z = n as f32 * 0.37 - 1.33;
        b.slab(
            v(0.0, 0.10, z),
            v(1.67 + (n % 2) as f32 * 0.065, 0.103, 0.348),
            shift(wood, (n % 5) * 6 - 12),
        );
        if n % 4 == 0 {
            for x in [-0.72, 0.72] {
                b.frustum(v(x, -1.24, z), 0.087, 0.075, 1.69, 7, shift(wood, -21));
                b.frustum(v(x, 0.30, z), 0.090, 0.091, 0.051, 7, [200, 158, 103]);
            }
        }
    }
    for side in [-1.0, 1.0] {
        b.beam(
            v(side * 0.72, 0.18, -0.55),
            v(side * 0.72, 0.18, 3.87),
            0.065,
            shift(wood, -15),
        );
    }
    for (x, z, h) in [
        (-0.45, -0.72, 0.33),
        (0.40, -0.40, 0.32),
        (0.47, 0.00, 0.30),
    ] {
        crate_box(&mut b, v(x, h, z), v(0.40, 0.34, 0.36), shift(wood, 15));
    }
    for x in [-0.62, 0.62] {
        post(
            &mut b,
            v(x, 0.16, -1.09),
            0.075,
            1.13,
            wood,
            shift(wood, 14),
        );
    }
    b.beam(v(-0.78, 1.30, -1.09), v(0.78, 1.30, -1.09), 0.075, wood);
    for x in [-0.52, -0.17, 0.18, 0.53] {
        b.face(
            &[
                v(x - 0.16, 1.268, -1.08),
                v(x, 0.98, -0.99),
                v(x + 0.16, 1.268, -1.08),
            ],
            cloth,
        );
    }
    b.mesh()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::mesh::VertexAttributeValues;

    #[test]
    fn all_thirty_monuments_are_distinct_and_fit_adventure_plots() {
        for region in Region::ALL {
            let mut previous = Vec::new();
            for tier in 1..=6 {
                let mesh = landmark(region, tier);
                let Some(VertexAttributeValues::Float32x3(points)) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                else {
                    unreachable!()
                };
                assert_ne!(previous, *points, "{region:?} Stufe {tier}");
                for &point in points {
                    let point = Vec3::from_array(point);
                    assert!(
                        point.is_finite()
                            && point.x.abs() <= 1.75
                            && point.z.abs() <= 1.60
                            && (-0.01..3.0).contains(&point.y),
                        "{region:?} Stufe {tier} {point:?}"
                    );
                }
                previous.clone_from(points);
            }
        }
    }
}

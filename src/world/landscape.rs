use crate::campaign::Region;
use bevy::prelude::*;

use super::{
    LOT_STEP,
    buildings::{
        BLUE, CREAM, GOLD, IVORY, MARBLE, OAK, TEAL, amphora, column, cypress, flowerpot, olive,
    },
    lot_position,
    mesh::{Batch, Grain, Tint, shift, v},
};

pub fn quay(columns: usize) -> f32 {
    lot_position(columns - 1, columns).x + 1.78
}

pub fn terrain(columns: usize, chapter: usize) -> Mesh {
    let mut b = Batch::default();
    let roman = Region::of(chapter) == Region::Rome;
    let mut grain = Grain::new(401 + chapter as u32 * 4051);
    let wide = columns as f32 * LOT_STEP * 0.5 + 2.6;
    let coast = [
        (-wide - 0.7, -3.4),
        (-wide + 0.6, -5.6),
        (-wide * 0.53, -6.8),
        (wide * 0.22, -6.3),
        (wide * 0.80, -6.64),
        (wide + 0.65, -4.5),
        (wide + 0.25, 1.2),
        (wide - 0.25, 3.8),
        (wide * 0.52, 4.2),
        (-wide * 0.48, 4.55),
        (-wide + 0.33, 3.79),
        (-wide - 0.7, 1.2),
    ]
    .map(|(x, z)| (x + grain.span(-0.23, 0.24), z + grain.span(-0.16, 0.19)));
    let (earth, edge) = if roman {
        let earth = [
            [176, 151, 124],
            [163, 159, 134],
            [174, 150, 132],
            [182, 159, 143],
            [166, 148, 133],
            [188, 165, 133],
        ][chapter % 6];
        (earth, shift(earth, -39))
    } else {
        let earth = [
            [184, 168, 123],
            [164, 164, 112],
            [187, 160, 135],
            [173, 169, 140],
            [186, 174, 119],
            [177, 162, 139],
            [168, 151, 130],
            [186, 172, 145],
        ][chapter % 8];
        (earth, shift(earth, -39))
    };
    island(&mut b, &coast, earth, edge, 0.0, -0.60);
    let shore = coast.map(|(x, z)| (x * 1.032, z * 1.065));
    island(
        &mut b,
        &shore,
        [196, 191, 145],
        [133, 161, 138],
        -0.28,
        -0.64,
    );

    let ramp = lot_position(0, columns).x - 2.25;
    let steps = ((wide - ramp) / 0.52) as usize;
    for i in 0..steps {
        let x = ramp + i as f32 * 0.52;
        for row in 0..6 {
            let pz = -0.87 + row as f32 * 0.32;
            let palette = [
                [209, 177, 135],
                [224, 195, 150],
                [197, 167, 134],
                [227, 202, 163],
            ];
            b.slab(
                v(x + grain.span(-0.014, 0.014), 0.021, pz),
                v(0.508, grain.span(0.037, 0.054), 0.304),
                if roman {
                    super::roman::paving(i, row)
                } else {
                    palette[(i * 19 + row * 3) % palette.len()]
                },
            );
        }
        for sign in [-1.0, 1.0] {
            b.slab(
                v(x, 0.055, sign * 1.073),
                v(0.51, 0.09, 0.13),
                shift(CREAM, 6),
            );
        }
    }

    for row in 0..2 {
        for col in 0..columns {
            let p = lot_position(row * crate::model::COLS + col, columns);
            b.slab(p.with_y(0.021), v(2.84, 0.048, 2.44), [206, 177, 132]);
            b.slab(p.with_y(0.040), v(2.65, 0.036, 2.21), [220, 192, 151]);
            let path_z = if row == 0 { -1.04 } else { 1.08 };
            for n in 0..3 {
                b.slab(
                    v(p.x + (n as f32 - 1.0) * 0.25, 0.042, path_z),
                    v(0.225, 0.055, 0.28),
                    MARBLE,
                );
            }
            for sign in [-1.0, 1.0] {
                let x = p.x + 1.29 * sign;
                let z = p.z + 1.02;
                b.slab(v(x, 0.077, z), v(0.24, 0.037, 0.064), IVORY);
                b.slab(
                    v(x + 0.11 * sign, 0.077, z - 0.09),
                    v(0.064, 0.037, 0.225),
                    IVORY,
                );
            }
            if col + 1 < columns {
                let p = v(p.x + LOT_STEP / 2.0, 0.01, p.z + 0.57);
                meadow(&mut b, p, 0.29, &mut grain);
            }
        }
    }
    for n in 0..90 {
        let x = grain.span(-wide + 0.8, wide - 0.63);
        let z = if n % 3 == 0 {
            grain.span(3.40, 3.77)
        } else {
            grain.span(-5.16, -3.62)
        };
        meadow(&mut b, v(x, 0.018, z), grain.span(0.14, 0.49), &mut grain);
    }
    promenade_wall(&mut b, wide);
    harbor(&mut b, quay(columns));
    if roman {
        super::roman::skyline(&mut b, wide, chapter, &mut grain);
        super::roman::quay_landmark(&mut b, -wide + 1.28, chapter);
        super::roman::quay_landmark(&mut b, wide - 1.45, chapter + 1);
    } else {
        sculpt_terraces(&mut b, wide, chapter, &mut grain);
        plaza(&mut b, -wide + 1.23, chapter);
    }
    market_rope(&mut b, -wide + 1.32, 1.0, -wide + 3.5, 1.04);
    market_rope(&mut b, wide - 3.44, -1.22, wide - 1.12, -1.28);
    b.mesh()
}

fn island(b: &mut Batch, outline: &[(f32, f32)], upper: Tint, cliff: Tint, y: f32, water: f32) {
    // Outline is clockwise from above, so top triangles face the sky.
    for i in 0..outline.len() {
        let (x1, z1) = outline[i];
        let (x2, z2) = outline[(i + 1) % outline.len()];
        let a = v(x1, y, z1);
        let c = v(x2, y, z2);
        b.face(&[Vec3::Y * y, c, a], shift(upper, (i * 7 % 12) as i16 - 4));
        b.face(
            &[
                a,
                c,
                v(x2 * 1.025, water, z2 * 1.025),
                v(x1 * 1.025, water, z1 * 1.025),
            ],
            shift(cliff, (i * 13 % 21) as i16 - 8),
        );
    }
}

fn meadow(b: &mut Batch, center: Vec3, size: f32, grain: &mut Grain) {
    for j in 0..5 {
        let p = center + v(grain.span(-size, size), 0.018, grain.span(-size, size));
        let green = shift([115, 128, 72], grain.span(-16.0, 18.0) as i16);
        let h = grain.span(0.12, 0.26);
        let tilt = grain.span(-0.12, 0.12);
        let left = p + v(-0.041, 0.0, 0.0);
        let right = p + v(0.041, 0.0, 0.0);
        let tip = p + v(tilt, h, 0.02);
        b.face(&[left, right, tip], green);
        b.face(&[right, left, tip], green);
        if j == 2 {
            b.gem(
                p + v(tilt, h, 0.02),
                0.031,
                0.028,
                0.031,
                0.0,
                [[247, 222, 141], [227, 148, 145], [224, 207, 184]][(grain.get() * 3.0) as usize],
            );
        }
    }
}

fn promenade_wall(b: &mut Batch, wide: f32) {
    let count = ((wide - 1.9) * 2.0 / 0.43) as usize;
    for i in 0..count {
        let x = -wide + 1.23 + i as f32 * 0.43;
        let z = 3.67 + (x * 0.15).cos() * 0.22;
        b.slab(
            v(x, 0.12, z),
            v(0.422, 0.27, 0.37),
            shift(CREAM, (i * 11 % 27) as i16 - 10),
        );
        b.slab(v(x + 0.01, 0.271, z), v(0.429, 0.075, 0.41), MARBLE);
    }
    for i in 0..10 {
        let x = (i as f32 / 9.0 - 0.5) * (wide * 1.65);
        b.slab(
            v(x, 0.31, 3.73 + (x * 0.15).cos() * 0.22),
            v(0.18, 0.29, 0.42),
            IVORY,
        );
        b.slab(
            v(x, 0.47, 3.73 + (x * 0.15).cos() * 0.22),
            v(0.23, 0.062, 0.47),
            CREAM,
        );
    }
}

fn sculpt_terraces(b: &mut Batch, wide: f32, chapter: usize, grain: &mut Grain) {
    let cliff = [[156, 145, 108], [143, 147, 107], [153, 137, 118]][chapter % 3];
    for n in 0..17 {
        let x = -wide + 0.35 + n as f32 * (wide * 2.0 - 0.9) / 17.0;
        let z = grain.span(-5.72, -4.85);
        let rise = grain.span(0.38, 0.89);
        b.gem(
            v(x, 0.13, z),
            grain.span(0.93, 1.63),
            rise,
            grain.span(0.8, 1.26),
            grain.span(-1.0, 1.0),
            shift(cliff, grain.span(-11.0, 12.0) as i16),
        );
        if n % 3 == 1 {
            cypress(
                b,
                v(x + 0.13, rise * 0.26, z + 0.11),
                grain.span(0.78, 1.32),
            );
        } else if n % 2 == 0 {
            olive(b, v(x, rise * 0.34, z), grain.span(0.72, 1.22));
        }
    }
    let x = -wide * 0.38;
    b.slab(v(x, 0.49, -4.20), v(3.58, 0.58, 0.77), CREAM);
    b.slab(v(x, 0.81, -4.27), v(3.67, 0.10, 0.85), IVORY);
    for i in 0..7 {
        let x = x - 1.45 + i as f32 * 0.48;
        let h = if (i + chapter) % 3 == 2 {
            0.43
        } else {
            1.30 + 0.12 * (chapter % 3) as f32
        };
        column(b, v(x, 0.86, -4.27), h, 0.115);
        if h > 1.0 && i != 6 {
            b.slab(v(x + 0.13, 0.96 + h, -4.27), v(0.61, 0.15, 0.28), MARBLE);
        }
    }
}

fn harbor(b: &mut Batch, x: f32) {
    for side in [-1.0, 1.0] {
        for n in 0..4 {
            let z = 3.2 + n as f32 * 1.08;
            let p = v(x + 0.62 * side, -0.64, z);
            b.frustum(p, 0.11, 0.095, 1.02, 8, OAK);
            b.frustum(p + Vec3::Y, 0.115, 0.115, 0.069, 8, CREAM);
            if n > 0 {
                let before = p + v(0.0, 0.93, -1.08);
                let middle = p + v(0.0, 0.72, -0.54);
                b.beam(before, middle, 0.038, IVORY);
                b.beam(middle, p + v(0.0, 0.93, 0.0), 0.038, IVORY);
            }
        }
    }
    for n in 0..23 {
        b.slab(
            v(x, 0.064, 3.05 + n as f32 * 0.175),
            v(1.39, 0.14, 0.166),
            shift([148, 101, 70], (n * 7 % 38) as i16 - 17),
        );
    }
    b.slab(v(x, 0.057, 6.95), v(2.10, 0.15, 0.65), OAK);
    for i in 0..3 {
        amphora(
            b,
            v(x - 0.49, 0.133, 3.7 + i as f32 * 0.32),
            0.75,
            if i == 1 { TEAL } else { [187, 107, 79] },
        );
    }
    b.beam(v(x - 0.74, 0.1, 3.00), v(x - 0.74, 2.10, 3.00), 0.081, OAK);
    b.face(
        &[
            v(x - 0.73, 1.97, 3.012),
            v(x - 0.73, 1.60, 3.012),
            v(x - 0.05, 1.79, 3.012),
        ],
        [212, 119, 78],
    );
}

fn plaza(b: &mut Batch, x: f32, chapter: usize) {
    let z = -1.94;
    b.frustum(v(x, 0.03, z), 0.89, 0.89, 0.052, 16, CREAM);
    b.frustum(v(x, 0.076, z), 0.67, 0.67, 0.018, 16, TEAL);
    for i in 0..12 {
        let angle = i as f32 / 12.0 * std::f32::consts::TAU;
        b.block(
            v(x + angle.cos() * 0.67, 0.10, z + angle.sin() * 0.67),
            v(0.16, 0.03, 0.25),
            Quat::from_rotation_y(-angle),
            if i % 3 != 0 { MARBLE } else { BLUE },
        );
    }
    b.frustum(v(x, 0.10, z), 0.22, 0.22, 0.081, 10, IVORY);
    b.frustum(v(x, 0.18, z), 0.14, 0.155, 0.54, 10, MARBLE);
    b.frustum(v(x, 0.58, z), 0.38, 0.49, 0.10, 12, TEAL);
    b.frustum(v(x, 0.655, z), 0.38, 0.41, 0.042, 12, [103, 192, 199]);
    flowerpot(
        b,
        v(x + 0.76, 0.012, z - 0.58),
        0.94,
        if chapter.is_multiple_of(2) {
            [222, 122, 142]
        } else {
            [180, 124, 176]
        },
    );
}

fn market_rope(b: &mut Batch, x1: f32, z1: f32, x2: f32, z2: f32) {
    for (x, z) in [(x1, z1), (x2, z2)] {
        b.slab(v(x, 1.13, z), v(0.074, 2.25, 0.074), OAK);
        b.gem(v(x, 2.27, z), 0.079, 0.08, 0.08, 0.4, GOLD);
    }
    let at = |t: f32| {
        v(
            x1 + (x2 - x1) * t,
            2.21 - t * (1.0 - t) * 0.86,
            z1 + (z2 - z1) * t,
        )
    };
    for n in 0..10 {
        let a = at(n as f32 * 0.1);
        let c = at((n + 1) as f32 * 0.1);
        b.beam(a, c, 0.022, OAK);
        let mid = (a + c) * 0.5;
        b.face(
            &[a, mid - v(0.0, 0.20, -0.022), c],
            [TEAL, MARBLE, GOLD, [175, 79, 61]][n % 4],
        );
    }
}

pub fn water(columns: usize, region: Region) -> Mesh {
    let mut b = Batch::default();
    let wide = columns as f32 * LOT_STEP * 0.5 + 2.6;
    let mut grain = Grain::new(59);
    for zi in -23..=22 {
        for xi in -25..=24 {
            let z = zi as f32 * 2.8;
            let x = xi as f32 * 2.8;
            let depth = ((x.abs() - wide + 1.0).max(0.0) + (z.abs() - 2.8).max(0.0)) / 16.0;
            let depth = depth.clamp(0.0, 1.0);
            let mix = |shallow: f32, deep: f32| (shallow + (deep - shallow) * depth) as u8;
            let tint = if region == Region::Rome {
                [mix(125.0, 49.0), mix(170.0, 111.0), mix(151.0, 124.0)]
            } else {
                [mix(100.0, 36.0), mix(177.0, 112.0), mix(166.0, 142.0)]
            };
            let point = |x, z| v(x, -0.442, z);
            let a = point(x, z);
            let c = point(x + 2.8, z + 2.8);
            b.face(
                &[a, point(x, z + 2.8), c],
                shift(tint, grain.span(-3.5, 3.5) as i16),
            );
            b.face(
                &[a, c, point(x + 2.8, z)],
                shift(tint, grain.span(-3.5, 3.5) as i16),
            );
        }
    }
    b.mesh()
}

pub fn foam(columns: usize, lane: usize) -> Mesh {
    let mut b = Batch::default();
    let mut grain = Grain::new(238921 + lane as u32 * 717);
    let wide = columns as f32 * LOT_STEP * 0.5 + 2.6;
    for i in 0..95 {
        let x = grain.span(-wide - 10.0, wide + 10.0);
        let z = if i % 6 == 0 {
            grain.span(-13.0, -6.4)
        } else {
            grain.span(4.25, 15.7)
        };
        let len = grain.span(0.14, 1.46);
        let depth = grain.span(0.019, 0.063);
        b.face(
            &[
                v(x - len * 0.51, 0.0, z),
                v(x, 0.0, z + depth),
                v(x + len * 0.51, 0.0, z - 0.025),
                v(x, 0.0, z - depth),
            ],
            [116, 192, 185],
        );
    }
    for i in 0..34 {
        let x = -wide + (i as f32 / 33.0) * wide * 2.0;
        let z = 4.19 + (x * 0.28).cos() * 0.29 + lane as f32 * 0.18 + grain.span(-0.11, 0.16);
        let len = grain.span(0.17, 0.68);
        b.face(
            &[
                v(x - len, 0.0, z),
                v(x, 0.0, z + 0.075),
                v(x + len, 0.0, z),
                v(x, 0.0, z - 0.026),
            ],
            if lane == 0 {
                [182, 220, 200]
            } else {
                [134, 205, 191]
            },
        );
    }
    b.mesh()
}

pub fn islands(chapter: usize) -> Mesh {
    if Region::of(chapter) == Region::Rome {
        return super::roman::far_bank(chapter);
    }
    let mut b = Batch::default();
    let mut grain = Grain::new(715 + chapter as u32 * 123);
    for (i, (x, z, w)) in [
        (-21.0, -17.0, 7.5),
        (2.0, -20.0, 8.0),
        (18.0, -16.0, 8.2),
        (32.0, -11.0, 5.4),
        (-5.0, -31.0, 7.2),
    ]
    .into_iter()
    .enumerate()
    {
        for j in 0..6 {
            b.gem(
                v(
                    x + (j as f32 - 2.0) * (w * 0.44),
                    -0.56,
                    z + grain.span(-2.0, 2.0),
                ),
                w * grain.span(0.38, 0.72),
                w * grain.span(0.18, 0.38),
                w * 0.43,
                grain.span(0.0, 3.0),
                shift([116, 154, 152], 4 * i as i16 - j as i16),
            );
        }
    }
    b.mesh()
}

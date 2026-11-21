use bevy::prelude::*;

use crate::campaign::Region;

use super::{
    buildings::{BLUE, CREAM, GOLD, MARBLE, OAK, TEAL, amphora},
    mesh::{Batch, Tint, shift, v},
};

const OCHRE: Tint = [170, 111, 65];
const PITCH: Tint = [74, 48, 39];
const WINE: Tint = [169, 69, 52];
const LENGTHS: [f32; 3] = [2.25, 1.19, 1.52];
const WIDTHS: [f32; 3] = [0.63, 0.44, 0.54];

fn paint(region: Region, class: usize) -> Tint {
    match region {
        Region::Greece => [TEAL, BLUE, GOLD][class],
        Region::Rome => [WINE, [63, 112, 99], [172, 127, 68]][class],
    }
}

pub fn hull(region: Region, class: usize) -> Mesh {
    let mut b = Batch::default();
    let large = class == 0;
    let roman = region == Region::Rome;
    let length = LENGTHS[class];
    let width = WIDTHS[class];
    let tint = paint(region, class);
    let stations = [
        -1.0, -0.91, -0.73, -0.49, -0.23, 0.03, 0.30, 0.55, 0.76, 0.91, 1.0,
    ];
    let sections = [
        (-0.19, 0.38, PITCH),
        (-0.04, 0.73, shift(OAK, 12)),
        (0.10, 0.90, OCHRE),
        (0.23, 0.98, shift(OCHRE, 10)),
        (0.31, 1.0, tint),
        (0.395, 0.98, shift(OAK, 20)),
    ];
    let point = |station: usize, strake: usize, side: f32| {
        let t: f32 = stations[station];
        let fullness = (1.0f32 - t.abs().powf(if roman { 1.95 } else { 1.62 })).powf(0.60);
        let (level, breadth, _) = sections[strake];
        let ends = t.abs().powi(5) * if roman { 0.29 } else { 0.22 };
        v(
            t * length * (0.88 + 0.025 * strake as f32),
            level + ends,
            side * width * fullness * breadth,
        )
    };
    for station in 0..stations.len() - 1 {
        for (band, section) in sections.iter().enumerate().take(sections.len() - 1) {
            for side in [-1.0, 1.0] {
                let mut planks = [
                    point(station, band, side),
                    point(station + 1, band, side),
                    point(station + 1, band + 1, side),
                    point(station, band + 1, side),
                ];
                if side < 0.0 {
                    planks.reverse();
                }
                let color = shift(section.2, (station * 19 % 22) as i16 - 9);
                b.face(&planks, color);
            }
        }
        let edge = sections.len() - 1;
        let at = [
            point(station, edge, -1.0),
            point(station, edge, 1.0),
            point(station + 1, edge, 1.0),
            point(station + 1, edge, -1.0),
        ];
        b.face(
            &at.map(|p| p - Vec3::Y * 0.041),
            shift(OCHRE, (station * 7 % 19) as i16),
        );
        for side in [-1.0, 1.0] {
            b.beam(
                point(station, edge, side),
                point(station + 1, edge, side),
                0.049,
                PITCH,
            );
            let a = point(station, edge - 1, side);
            let c = point(station + 1, edge - 1, side);
            b.beam(
                a + Vec3::Z * side * 0.005,
                c + Vec3::Z * side * 0.005,
                0.021,
                GOLD,
            );
        }
    }
    b.beam(
        v(-length * 0.85, -0.18, 0.0),
        v(length * 0.85, -0.18, 0.0),
        0.11,
        PITCH,
    );
    b.frustum(
        v(-0.20, 0.36, 0.0),
        0.078,
        0.058,
        if large { 2.08 } else { 1.60 },
        10,
        OAK,
    );

    let mast_top = if large { 2.30 } else { 1.89 };
    for side in [-1.0, 1.0] {
        b.beam(
            v(-0.2, mast_top, 0.0),
            v(-0.96, 0.46, side * width * 0.80),
            0.022,
            CREAM,
        );
        b.beam(
            v(-0.2, mast_top - 0.07, 0.0),
            v(0.76, 0.43, side * width * 0.78),
            0.017,
            CREAM,
        );
        for i in 0..if large { 4 } else { 2 } {
            let x = -0.77 + i as f32 * 0.41;
            let at = v(x, 0.40, side * width * 0.90);
            let tip = v(
                x - 0.30,
                -0.08 + (i % 2) as f32 * 0.042,
                side * (width + 0.34),
            );
            b.beam(at, tip, 0.036, shift(OAK, 37));
            b.block(
                tip,
                v(0.11, 0.046, 0.29),
                Quat::from_rotation_y(-side * 0.63),
                OCHRE,
            );
        }
    }
    let stern = -length * 0.93;
    let prow = length * 0.95;
    if roman {
        b.beam(
            v(stern + 0.32, 0.48, 0.0),
            v(stern + 0.015, 0.81, 0.0),
            0.105,
            PITCH,
        );
        b.beam(
            v(stern + 0.015, 0.81, 0.0),
            v(stern + 0.095, 1.16, 0.0),
            0.076,
            OCHRE,
        );
        b.gem(v(stern + 0.09, 1.18, 0.0), 0.077, 0.091, 0.071, 0.25, GOLD);
        if large {
            b.slab(
                v(-length * 0.52, 0.64, 0.0),
                v(0.63, 0.51, 0.78),
                TRAVELLER_CABIN,
            );
            b.slab(v(-length * 0.52, 0.92, 0.0), v(0.75, 0.11, 0.89), WINE);
            b.slab(v(-length * 0.52, 0.68, 0.397), v(0.28, 0.21, 0.05), PITCH);
        }
    } else {
        b.beam(
            v(prow - 0.31, 0.43, 0.0),
            v(prow + 0.12, 0.61, 0.0),
            0.111,
            PITCH,
        );
        b.beam(
            v(prow + 0.12, 0.61, 0.0),
            v(prow + 0.053, 0.94, 0.0),
            0.081,
            OCHRE,
        );
        b.beam(
            v(prow + 0.053, 0.94, 0.0),
            v(prow - 0.10, 1.07, 0.0),
            0.059,
            GOLD,
        );
        for side in [-1.0, 1.0] {
            b.gem(
                v(prow - 0.25, 0.49, side * 0.22),
                0.12,
                0.054,
                0.035,
                0.0,
                MARBLE,
            );
            b.gem(
                v(prow - 0.23, 0.49, side * 0.242),
                0.041,
                0.038,
                0.024,
                0.0,
                PITCH,
            );
        }
    }
    for (x, z) in [(-0.54, 0.22), (0.38, 0.19), (0.72, -0.15)] {
        b.slab(v(x, 0.43, z), v(0.37, 0.17, 0.29), OAK);
        if large {
            amphora(&mut b, v(x, 0.50, z), 0.65, if roman { WINE } else { tint });
        }
    }
    b.beam(
        v(stern + 0.20, 0.50, 0.20),
        v(stern - 0.26, -0.04, 0.62),
        0.060,
        OAK,
    );
    b.mesh()
}

const TRAVELLER_CABIN: Tint = [177, 130, 83];

pub fn sail(region: Region, class: usize) -> Mesh {
    let mut b = Batch::default();
    let large = class == 0;
    let width = if large { 1.84 } else { 1.34 };
    let y = if large { 2.11 } else { 1.73 };
    let height = if large { 1.10 } else { 0.87 };
    let tint = paint(region, class);
    let pinned = |u: f32, v_: f32| {
        let billow = (std::f32::consts::PI * u).sin() * (std::f32::consts::PI * v_ * 0.87).sin();
        let x = (u - 0.5) * width * (1.0 - 0.17 * v_) - 0.21 + (v_ * v_) * 0.18;
        v(
            x,
            y - v_ * height - (1.0 - billow) * 0.042,
            0.031 + billow * if large { 0.31 } else { 0.21 },
        )
    };
    for row in 0..7 {
        for col in 0..12 {
            let u = col as f32 / 12.0;
            let vs = row as f32 / 7.0;
            let color = if row == 5 || (region == Region::Rome && col == 5) {
                tint
            } else {
                shift(MARBLE, (col * 8 % 17) as i16 - 9)
            };
            let corners = [
                pinned(u, vs),
                pinned(u, vs + 1.0 / 7.0),
                pinned(u + 1.0 / 12.0, vs + 1.0 / 7.0),
                pinned(u + 1.0 / 12.0, vs),
            ];
            b.face(&corners, color);
            b.face(
                &corners
                    .map(|point| point - Vec3::Z * 0.015)
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>(),
                shift(color, -8),
            );
        }
    }
    b.beam(
        pinned(0.0, 0.0) + v(-0.088, 0.06, -0.06),
        pinned(1.0, 0.0) + v(0.088, 0.06, -0.06),
        0.061,
        OAK,
    );
    b.beam(pinned(0.0, 1.0), v(-0.79, 0.46, 0.07), 0.018, CREAM);
    b.beam(pinned(1.0, 1.0), v(1.18, 0.41, -0.07), 0.018, CREAM);
    b.face(
        &[
            v(-0.2, y + 0.18, 0.003),
            v(-0.2, y + 0.34, 0.012),
            v(0.41, y + 0.23, 0.025),
        ],
        tint,
    );
    b.mesh()
}

pub fn wake(class: usize) -> Mesh {
    let mut b = Batch::default();
    let length = LENGTHS[class];
    let width = WIDTHS[class];
    for side in [-1.0, 1.0] {
        for i in 0..9 {
            let x = length * 0.82 - i as f32 * length * 0.25;
            let z = side * (width + 0.13 + i as f32 * 0.091);
            let len = 0.33 + (i % 4) as f32 * 0.18;
            b.face(
                &[
                    v(x - len, 0.0, z - 0.051),
                    v(x - 0.21, 0.0, z + 0.035),
                    v(x + len * 0.32, 0.0, z + 0.005),
                    v(x - 0.16, 0.0, z - 0.076),
                ],
                if i % 3 == 0 {
                    [196, 224, 212]
                } else {
                    [137, 189, 179]
                },
            );
        }
    }
    b.mesh()
}

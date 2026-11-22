use bevy::prelude::*;

use crate::model::Kind;

use super::{
    buildings::{CREAM, GOLD, IVORY, OAK, amphora, flowerpot, olive},
    mesh::{Batch, shift, v},
    upgrades::{bench, crate_box, rail},
};

pub(super) fn add(b: &mut Batch, kind: Kind, level: u8, roman: bool) {
    let stone = if roman { [213, 171, 131] } else { IVORY };
    let clay = if roman { [164, 82, 59] } else { [207, 125, 76] };
    let teal = if roman {
        [77, 118, 126]
    } else {
        [64, 136, 148]
    };
    let fifth = level == 5;
    match kind {
        Kind::House | Kind::Inn | Kind::Villa => {
            if fifth {
                // A shaded porch supported all the way down to the existing yard.
                let height = if roman { 1.29 } else { 0.98 };
                for x in [-1.12, 1.12] {
                    b.slab(
                        v(x, height / 2.0 + 0.12, 1.075),
                        v(0.14, height, 0.17),
                        stone,
                    );
                    b.slab(v(x, 0.15, 1.075), v(0.22, 0.10, 0.24), CREAM);
                }
                b.slab(v(0.0, height + 0.12, 1.06), v(2.58, 0.115, 0.46), stone);
                b.slab(v(0.0, height + 0.19, 1.06), v(2.62, 0.069, 0.49), teal);
                flowerpot(b, v(-0.80, 0.11, 1.19), 0.61, [204, 119, 151]);
                bench(b, v(0.37, 0.095, 1.12), 0.77);
            } else {
                let y = if kind == Kind::Villa || roman {
                    2.46
                } else {
                    2.11
                };
                b.block(
                    v(0.72, y - 0.13, -0.52),
                    v(0.79, 1.05, 0.82),
                    Quat::IDENTITY,
                    stone,
                );
                b.slab(v(0.72, y + 0.44, -0.52), v(0.91, 0.14, 0.94), teal);
                b.slab(
                    v(0.72, y + 0.58, -0.52),
                    v(0.71, 0.16, 0.73),
                    shift(teal, 14),
                );
                b.slab(
                    v(0.72, y + 0.08, -0.099),
                    v(0.35, 0.41, 0.045),
                    [73, 73, 66],
                );
                for x in [0.57, 0.72, 0.87] {
                    b.slab(v(x, y + 0.077, -0.071), v(0.051, 0.36, 0.048), clay);
                }
                b.slab(v(0.72, y - 0.163, -0.062), v(0.40, 0.069, 0.15), CREAM);
            }
        }
        Kind::Well | Kind::Aqueduct => {
            let p = v(if fifth { -0.91 } else { 0.94 }, 0.108, 0.88);
            b.frustum(p, 0.25, 0.29, 0.23, 10, stone);
            b.frustum(p + Vec3::Y * 0.23, 0.275, 0.249, 0.035, 10, teal);
            b.frustum(p + Vec3::Y * 0.26, 0.291, 0.30, 0.036, 11, CREAM);
            if fifth {
                b.slab(p + v(0.0, 0.43, -0.16), v(0.16, 0.60, 0.17), stone);
                b.beam(p + v(0.0, 0.55, -0.17), p + v(0.0, 0.52, 0.03), 0.055, GOLD);
                b.beam(
                    p + v(0.0, 0.51, 0.03),
                    p + v(0.0, 0.29, 0.03),
                    0.026,
                    [113, 188, 192],
                );
            } else {
                let y = if kind == Kind::Aqueduct { 2.78 } else { 2.19 };
                b.slab(v(0.0, y, -0.30), v(1.54, 0.15, 0.68), stone);
                b.frustum(v(0.0, y + 0.08, -0.30), 0.48, 0.47, 0.42, 12, teal);
                b.frustum(
                    v(0.0, y + 0.50, -0.30),
                    0.48,
                    0.36,
                    0.17,
                    12,
                    shift(teal, 19),
                );
                for n in 0..3 {
                    let x = -0.49 + n as f32 * 0.49;
                    b.beam(v(x, y, -0.05), v(x, y - 0.73, -0.05), 0.044, clay);
                }
            }
        }
        Kind::HerbGarden | Kind::Grove => {
            if fifth {
                for x in [-0.94, 0.12, 1.02] {
                    b.slab(v(x, 0.165, 0.97), v(0.62, 0.22, 0.35), stone);
                    for n in 0..3 {
                        let at = v(x - 0.19 + n as f32 * 0.19, 0.30, 0.99);
                        b.gem(at, 0.16, 0.079, 0.139, n as f32 * 0.66, [96, 147, 75]);
                        b.gem(
                            at + Vec3::Y * 0.084,
                            0.049,
                            0.053,
                            0.037,
                            n as f32 * 0.28,
                            [175, 125, 183],
                        );
                    }
                }
            } else {
                let y = if kind == Kind::HerbGarden { 1.25 } else { 1.69 };
                for x in [-1.06, 1.06] {
                    b.beam(v(x, 0.12, -0.85), v(x, y, -0.85), 0.072, OAK);
                }
                b.beam(v(-1.20, y, -0.85), v(1.20, y, -0.85), 0.089, OAK);
                for n in 0..9 {
                    let x = -1.11 + n as f32 * 0.275;
                    b.gem(
                        v(x, y - 0.03, -0.76),
                        0.24,
                        0.16,
                        0.17,
                        n as f32,
                        shift([98, 159, 90], n * 5 - 22),
                    );
                }
                if kind == Kind::Grove {
                    olive(b, v(-1.13, 0.12, -0.87), 0.64);
                }
            }
        }
        Kind::Bakery | Kind::Potter | Kind::Storehouse | Kind::Press => {
            if fifth {
                let p = v(1.02, 0.11, 0.62);
                b.slab(p + Vec3::Y * 0.39, v(0.51, 0.77, 0.70), stone);
                b.block(
                    p + v(0.0, 0.80, 0.03),
                    v(0.66, 0.11, 0.82),
                    Quat::from_rotation_x(-0.13),
                    clay,
                );
                let tint = if kind == Kind::Potter { teal } else { clay };
                if matches!(kind, Kind::Potter | Kind::Press) {
                    amphora(b, p + v(0.04, 0.11, 0.44), 0.76, tint);
                } else {
                    crate_box(b, p + v(-0.16, 0.16, 0.33), v(0.38, 0.32, 0.34), OAK);
                }
            } else {
                let y = if roman { 2.44 } else { 2.16 };
                b.slab(v(-0.78, y + 0.075, -0.49), v(0.76, 0.76, 0.69), stone);
                b.slab(
                    v(-0.78, y + 0.465, -0.49),
                    v(0.89, 0.14, 0.78),
                    if kind == Kind::Potter { teal } else { clay },
                );
                b.slab(
                    v(-0.78, y + 0.13, -0.124),
                    v(0.31, 0.30, 0.074),
                    [89, 62, 51],
                );
                b.slab(v(-0.78, y + 0.10, -0.062), v(0.37, 0.049, 0.17), GOLD);
                for x in [-0.96, -0.60] {
                    b.beam(v(x, y + 0.22, -0.08), v(x, y - 0.18, -0.08), 0.044, OAK);
                }
            }
        }
        Kind::Taverna | Kind::Stoa | Kind::Baths | Kind::Shrine | Kind::Forum => {
            let y = match kind {
                Kind::Baths | Kind::Forum => 2.95,
                Kind::Shrine => 2.11,
                _ => 1.91,
            };
            if fifth {
                let tint = if kind == Kind::Shrine { clay } else { teal };
                b.slab(v(0.0, 0.115, 1.12), v(2.50, 0.103, 0.34), stone);
                for n in 0..11 {
                    let x = -1.06 + n as f32 * 0.212;
                    b.block(
                        v(x, 0.174, 1.12),
                        v(0.139, 0.014, 0.139),
                        Quat::from_rotation_y(0.785),
                        tint,
                    );
                }
                for x in [-1.06, 1.06] {
                    b.frustum(v(x, 0.18, 1.07), 0.18, 0.12, 0.21, 8, clay);
                    flowerpot(b, v(x, 0.371, 1.07), 0.57, [221, 137, 130]);
                }
            } else {
                b.slab(v(0.33, y + 0.055, 0.60), v(1.79, 0.14, 0.72), stone);
                rail(b, v(0.33, y + 0.17, 0.91), 1.67, clay);
                for x in [-0.44, 1.10] {
                    b.frustum(v(x, y - 0.40, 0.66), 0.070, 0.066, 0.61, 8, stone);
                    b.frustum(v(x, y + 0.18, 0.89), 0.08, 0.044, 0.34, 8, stone);
                    b.gem(v(x, y + 0.57, 0.89), 0.10, 0.091, 0.098, 0.3, GOLD);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        campaign::{CHAPTERS, Region},
        model::MAX_LEVEL,
    };

    #[test]
    fn every_offered_building_has_geometry_at_each_upgrade() {
        for chapter in &CHAPTERS {
            for &kind in chapter.allowed() {
                let mut prior = 0;
                for level in 1..=MAX_LEVEL {
                    let mesh = match chapter.region {
                        Region::Greece => super::super::buildings::model(kind, level),
                        Region::Rome => super::super::roman::model(kind, level),
                    };
                    assert!(
                        mesh.count_vertices() > prior,
                        "{:?} {kind:?} {level}",
                        chapter.region
                    );
                    prior = mesh.count_vertices();
                }
            }
        }
    }
}

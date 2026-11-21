use bevy::prelude::*;

use super::{
    buildings::{CREAM, GOLD, MARBLE, OAK, TEAL},
    mesh::{Batch, shift, v},
};

pub fn hippo_limb(side: f32, leg: bool) -> (Vec3, Mesh) {
    let mut b = Batch::default();
    let hide = [141, 130, 141];
    let pivot = if leg {
        v(side * 0.204, 0.44, 0.0)
    } else {
        v(side * 0.405, 0.89, 0.03)
    };
    let local = |x, y, z| v(x, y, z) - pivot;
    if leg {
        b.gem(
            local(side * 0.204, 0.243, 0.0),
            0.144,
            0.277,
            0.167,
            side,
            hide,
        );
        b.gem(
            local(side * 0.21, 0.118, 0.085),
            0.164,
            0.107,
            0.219,
            side,
            shift(hide, -7),
        );
        for i in 0..3 {
            b.gem(
                local(side * 0.21 + (i as f32 - 1.0) * 0.07, 0.099, 0.228),
                0.039,
                0.031,
                0.049,
                0.0,
                [177, 154, 148],
            );
        }
    } else {
        b.gem(
            local(side * 0.405, 0.737, 0.03),
            0.131,
            0.225,
            0.13,
            side,
            hide,
        );
        b.gem(
            local(side * 0.441, 0.598, 0.074),
            0.118,
            0.107,
            0.129,
            side,
            shift(hide, 7),
        );
    }
    (pivot, b.mesh())
}

pub fn bird() -> Mesh {
    let mut b = Batch::default();
    for sign in [-1.0, 1.0] {
        let wing = [
            v(0.0, 0.0, 0.0),
            v(0.25 * sign, 0.072, -0.13),
            v(0.49 * sign, 0.121, -0.15),
            v(0.33 * sign, -0.015, -0.21),
            v(0.13 * sign, -0.004, -0.10),
        ];
        b.face(&wing, MARBLE);
        b.face(&wing.into_iter().rev().collect::<Vec<_>>(), MARBLE);
    }
    b.mesh()
}

pub fn response_marker(satisfied: bool) -> Mesh {
    let mut b = Batch::default();
    let color = if satisfied {
        [49, 131, 105]
    } else {
        [170, 86, 60]
    };
    // A small dark-backed sign keeps the shape readable against pale paving.
    b.block(
        Vec3::ZERO,
        v(0.25, 0.23, 0.065),
        Quat::from_rotation_z(0.12),
        color,
    );
    let z = 0.039;
    if satisfied {
        b.beam(v(-0.070, -0.009, z), v(-0.019, -0.046, z), 0.032, MARBLE);
        b.beam(v(-0.019, -0.046, z), v(0.087, 0.063, z), 0.032, MARBLE);
    } else {
        b.beam(v(0.0, 0.057, z), v(0.0, -0.027, z), 0.036, MARBLE);
        b.slab(v(0.0, -0.068, z), v(0.035, 0.033, 0.03), MARBLE);
    }
    b.mesh()
}

pub fn pack(job: crate::errands::Job) -> Mesh {
    use super::{
        buildings::{TERRACOTTA, amphora},
        upgrades::crate_box,
    };
    use crate::errands::Job;

    let mut b = Batch::default();
    let point = v(0.0, 0.88, -0.48);
    match job {
        Job::Freight => {
            crate_box(&mut b, point, v(0.47, 0.35, 0.33), [178, 125, 76]);
            b.beam(
                point + v(-0.24, 0.18, 0.18),
                point + v(0.23, 0.18, 0.18),
                0.038,
                CREAM,
            );
        }
        Job::Water => {
            for x in [-0.15, 0.15] {
                amphora(
                    &mut b,
                    point + v(x, -0.05, 0.015),
                    0.71,
                    if x < 0.0 { TEAL } else { TERRACOTTA },
                );
            }
            b.beam(
                point + v(-0.27, 0.08, 0.0),
                point + v(0.27, 0.08, 0.0),
                0.069,
                OAK,
            );
        }
        Job::Shade => {
            let leaf = [106, 143, 82];
            for i in 0..5 {
                let x = -0.24 + i as f32 * 0.125;
                b.beam(
                    point + v(x * 0.5, -0.035, 0.04),
                    point + v(x, 0.46, 0.03),
                    0.032,
                    OAK,
                );
                b.gem(
                    point + v(x, 0.37, -0.04),
                    0.15,
                    0.21,
                    0.105,
                    x * 4.0,
                    shift(leaf, i as i16 * 6 - 11),
                );
            }
        }
        Job::Quay => {
            b.frustum(point, 0.13, 0.183, 0.34, 8, [158, 97, 62]);
            for y in [0.068, 0.24, 0.29] {
                b.frustum(point + Vec3::Y * y, 0.176, 0.178, 0.028, 8, CREAM);
            }
            for side in [-1.0, 1.0] {
                b.beam(
                    point + v(side * 0.17, -0.03, 0.03),
                    point + v(side * 0.24, 0.52, -0.04),
                    0.066,
                    [151, 87, 53],
                );
            }
        }
        Job::Welcome => {
            b.beam(
                point + v(0.27, 0.04, 0.073),
                point + v(0.28, 0.81, 0.08),
                0.041,
                OAK,
            );
            b.face(
                &[
                    point + v(0.28, 0.80, 0.08),
                    point + v(0.61, 0.67, 0.073),
                    point + v(0.28, 0.54, 0.08),
                ],
                GOLD,
            );
            b.face(
                &[
                    point + v(0.28, 0.54, 0.07),
                    point + v(0.61, 0.67, 0.063),
                    point + v(0.28, 0.80, 0.07),
                ],
                GOLD,
            );
        }
    }
    b.mesh()
}

pub fn destination() -> Mesh {
    let mut b = Batch::default();
    for i in 0..18 {
        let a = i as f32 / 18.0 * std::f32::consts::TAU;
        let p = v(a.cos() * 0.54, 0.025, a.sin() * 0.39);
        b.block(
            p,
            v(0.092, 0.027, 0.12),
            Quat::from_rotation_y(-a),
            if i % 3 == 0 { MARBLE } else { GOLD },
        );
    }
    b.gem(v(0.0, 1.56, 0.01), 0.168, 0.225, 0.124, 0.26, GOLD);
    b.gem(
        v(0.0, 1.59, 0.093),
        0.079,
        0.117,
        0.035,
        0.22,
        [64, 120, 112],
    );
    b.mesh()
}

use bevy::prelude::*;

use super::{
    buildings::{BLUE, CREAM, GOLD, MARBLE, OAK, TEAL},
    mesh::{Batch, shift, v},
};

pub fn resident(index: usize, region: crate::campaign::Region) -> Mesh {
    use crate::campaign::Region;
    let mut b = Batch::default();
    let styled = index % 8;
    let roman = region == Region::Rome;
    let cloth = if roman {
        [
            [220, 192, 151],
            [163, 70, 69],
            [168, 124, 96],
            [116, 123, 107],
            [160, 114, 139],
            [72, 117, 126],
            [196, 161, 97],
            [196, 187, 161],
        ][styled]
    } else {
        [
            TEAL,
            [178, 93, 78],
            [170, 132, 70],
            BLUE,
            [163, 109, 152],
            [104, 129, 66],
            [214, 147, 78],
            [189, 170, 135],
        ][styled]
    };
    let skin = [
        [225, 170, 130],
        [177, 117, 83],
        [242, 201, 158],
        [151, 100, 76],
        [200, 145, 106],
        [116, 82, 66],
    ][(index * 7 + index / 3) % 6];
    let hair = [
        [73, 52, 42],
        [108, 71, 51],
        [53, 43, 38],
        [145, 100, 66],
        [198, 159, 112],
        [165, 157, 145],
    ][(index * 3 + index / 2) % 6];
    let body = 0.156 + (index % 5) as f32 * 0.013;
    let robe = matches!(styled, 1 | 4 | 6 | 7);
    let hem = if robe { 0.115 } else { 0.218 };
    let sash = if roman && styled % 3 != 1 {
        MARBLE
    } else {
        shift(cloth, 32)
    };
    b.frustum(
        v(0.0, hem, 0.0),
        body + 0.046,
        body * 0.84,
        0.49 - hem,
        9,
        cloth,
    );
    b.frustum(
        v(0.0, hem, 0.0),
        body + 0.048,
        body + 0.041,
        0.037,
        9,
        shift(cloth, -23),
    );
    for i in 0..5 {
        let x = (i as f32 - 2.0) * (body * 0.34);
        let front = body * 0.90 - x.abs() * 0.25;
        b.face(
            &[
                v(x - 0.037, hem + 0.04, front + 0.056),
                v(x + 0.028, hem + 0.04, front + 0.051),
                v(x * 0.72, 0.46, front - 0.013),
            ],
            shift(cloth, (i * 11 % 29) as i16 - 11),
        );
    }
    b.gem(v(0.0, 0.55, 0.0), body * 1.19, 0.141, 0.133, 0.13, cloth);
    b.frustum(
        v(0.0, 0.46, 0.0),
        body * 0.94,
        body * 0.90,
        0.070,
        10,
        shift(cloth, -27),
    );
    if roman || styled.is_multiple_of(3) {
        let shoulder = -body * 0.88;
        b.face(
            &[
                v(shoulder, 0.65, 0.092),
                v(shoulder - 0.04, 0.49, 0.155),
                v(body * 0.86, 0.29, 0.192),
                v(body * 0.90, 0.44, 0.148),
            ],
            sash,
        );
        b.face(
            &[
                v(shoulder, 0.65, 0.095),
                v(body * 0.89, 0.43, 0.149),
                v(body * 0.53, 0.52, 0.128),
                v(shoulder + 0.06, 0.66, 0.095),
            ],
            shift(sash, 14),
        );
        b.gem(
            v(shoulder + 0.043, 0.62, 0.116),
            0.031,
            0.03,
            0.025,
            0.3,
            GOLD,
        );
    }
    b.frustum(v(0.0, 0.59, 0.0), 0.068, 0.065, 0.123, 7, skin);
    let head_y = 0.769 + (index % 3) as f32 * 0.012;
    let head_width = 0.117 + (index % 4) as f32 * 0.011;
    b.gem(v(0.0, head_y, 0.027), head_width, 0.151, 0.123, 0.31, skin);
    b.gem(
        v(0.0, head_y - 0.011, 0.137),
        0.032,
        0.036,
        0.046,
        0.38,
        shift(skin, 11),
    );
    b.gem(
        v(0.0, head_y - 0.078, 0.119),
        0.046,
        0.012,
        0.020,
        0.2,
        shift(skin, -29),
    );
    b.gem(
        v(0.0, head_y + 0.079, -0.017),
        head_width + 0.015,
        0.091,
        0.135,
        0.49,
        hair,
    );
    if matches!(styled, 1 | 6) {
        b.gem(
            v(0.0, head_y + 0.066, -0.133),
            0.092,
            0.083,
            0.075,
            0.15,
            hair,
        );
    }
    if styled == 3 || styled == 5 {
        b.gem(
            v(0.0, head_y - 0.088, 0.076),
            0.095,
            0.063,
            0.087,
            0.74,
            hair,
        );
    }
    if styled == 2 && !roman {
        b.frustum(v(0.0, head_y + 0.085, 0.009), 0.214, 0.12, 0.059, 9, CREAM);
        b.frustum(v(0.0, head_y + 0.143, 0.009), 0.12, 0.089, 0.082, 8, GOLD);
    } else if styled == 4 || styled == 7 {
        for n in 0..6 {
            let a = 2.1 + n as f32 * 0.48;
            b.gem(
                v(a.cos() * 0.128, head_y + 0.048, a.sin() * 0.109),
                0.049,
                0.050,
                0.049,
                a,
                shift(hair, n as i16),
            );
        }
    }
    for side in [-1.0, 1.0] {
        b.gem(
            v(side * head_width * 0.52, head_y + 0.007, 0.133),
            0.019,
            0.017,
            0.010,
            0.0,
            [48, 40, 37],
        );
        b.gem(
            v(side * head_width * 0.50, head_y + 0.012, 0.140),
            0.006,
            0.006,
            0.005,
            0.0,
            MARBLE,
        );
        b.gem(
            v(side * head_width * 0.81, head_y - 0.027, 0.107),
            0.023,
            0.018,
            0.012,
            0.2,
            shift(skin, -12),
        );
        b.gem(
            v(side * head_width, head_y - 0.015, 0.026),
            0.032,
            0.044,
            0.029,
            0.0,
            skin,
        );
        if matches!(styled, 1 | 4 | 7) {
            b.gem(
                v(side * head_width, head_y - 0.049, 0.048),
                0.018,
                0.025,
                0.018,
                0.2,
                GOLD,
            );
        }
        b.gem(
            v(side * body * 1.10, 0.491, 0.012),
            0.066,
            0.103,
            0.061,
            0.17,
            cloth,
        );
        b.gem(
            v(side * body * 1.20, 0.372, 0.047),
            0.040,
            0.112,
            0.047,
            0.15,
            skin,
        );
        b.gem(
            v(side * body * 1.18, 0.285, 0.076),
            0.041,
            0.047,
            0.042,
            0.40,
            skin,
        );
        b.frustum(
            v(side * body * 0.53, 0.056, 0.015),
            0.039,
            0.048,
            hem,
            6,
            skin,
        );
        b.gem(
            v(side * body * 0.53, 0.048, 0.059),
            0.052,
            0.047,
            0.082,
            0.12,
            OAK,
        );
        b.beam(
            v(side * body * 0.53 - 0.04, 0.071, 0.076),
            v(side * body * 0.53 + 0.04, 0.071, 0.076),
            0.024,
            shift(OAK, 31),
        );
    }
    if styled == 0 {
        b.gem(v(body * 1.39, 0.28, 0.14), 0.093, 0.105, 0.078, 0.35, CREAM);
    } else if styled == 6 {
        b.slab(v(body * 1.38, 0.32, 0.12), v(0.096, 0.15, 0.056), GOLD);
    }
    b.mesh()
}

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

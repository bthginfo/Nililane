use bevy::prelude::*;

use super::{
    buildings::{CREAM, GOLD, MARBLE, OAK},
    mesh::{Batch, Tint, shift, v},
};

pub fn canopy(b: &mut Batch, p: Vec3, width: f32, depth: f32, h: f32, tint: Tint, tiled: bool) {
    let roof_y = p.y + h;
    for x in [-1.0, 1.0] {
        for z in [-1.0, 1.0] {
            let at = p + v(x * width * 0.45, 0.0, z * depth * 0.42);
            b.frustum(at, 0.081, 0.074, h, 7, OAK);
            b.slab(at + v(0.0, 0.040, 0.0), v(0.20, 0.083, 0.19), CREAM);
            b.slab(at + v(0.0, h - 0.055, 0.0), v(0.19, 0.105, 0.16), OAK);
        }
        let z = p.z + x * depth * 0.43;
        b.slab(v(p.x, roof_y, z), v(width, 0.10, 0.13), OAK);
    }
    for i in 0..12 {
        let x = p.x + ((i as f32 + 0.5) / 12.0 - 0.5) * width;
        if tiled {
            b.block(
                v(x, roof_y + 0.093, p.z),
                v(width / 12.0 * 0.94, 0.056, depth * 1.08),
                Quat::from_rotation_x(0.095),
                shift(tint, (i * 7 % 26) as i16 - 10),
            );
        } else {
            b.slab(v(x, roof_y + 0.073, p.z), v(0.063, 0.07, depth * 1.10), OAK);
            if i % 4 < 3 {
                b.slab(
                    v(x, roof_y + 0.13, p.z + 0.04),
                    v(width / 12.0, 0.045, depth * 0.88),
                    shift(tint, (i * 8 % 24) as i16 - 8),
                );
            }
        }
    }
}

pub fn rail(b: &mut Batch, p: Vec3, width: f32, tint: Tint) {
    for i in 0..10 {
        let x = p.x + (i as f32 / 9.0 - 0.5) * width;
        b.slab(v(x, p.y + 0.16, p.z), v(0.042, 0.32, 0.048), tint);
    }
    b.slab(
        p + v(0.0, 0.31, 0.0),
        v(width + 0.09, 0.061, 0.078),
        shift(tint, 12),
    );
}

pub fn bench(b: &mut Batch, p: Vec3, width: f32) {
    for x in [-1.0, 1.0] {
        b.slab(
            p + v(width * 0.38 * x, 0.102, 0.0),
            v(0.09, 0.205, 0.29),
            CREAM,
        );
    }
    b.slab(p + v(0.0, 0.23, 0.015), v(width, 0.069, 0.33), OAK);
}

pub fn crate_box(b: &mut Batch, p: Vec3, size: Vec3, tint: Tint) {
    b.slab(p + Vec3::Y * size.y * 0.5, size, tint);
    for x in [-0.43, 0.43] {
        b.slab(
            p + v(x * size.x, size.y * 0.50, size.z * 0.51),
            v(0.045, size.y * 1.05, 0.026),
            OAK,
        );
    }
    b.slab(
        p + v(0.0, size.y * 0.84, size.z * 0.515),
        v(size.x, 0.044, 0.029),
        shift(tint, 21),
    );
}

pub fn crane(b: &mut Batch, p: Vec3, h: f32) {
    let top = p + Vec3::Y * h;
    let end = top + v(0.0, 0.03, 0.85);
    b.slab(p + v(0.0, h / 2.0, 0.0), v(0.15, h, 0.14), OAK);
    b.beam(top - v(0.0, 0.04, 0.24), end, 0.125, OAK);
    b.beam(
        top - Vec3::Y * 0.68,
        top + v(0.0, 0.0, 0.59),
        0.078,
        shift(OAK, 21),
    );
    b.beam(end, end - Vec3::Y * 0.64, 0.027, CREAM);
    b.gem(end - Vec3::Y * 0.48, 0.08, 0.08, 0.08, 0.0, GOLD);
    crate_box(b, end - Vec3::Y * 0.89, v(0.32, 0.24, 0.30), [174, 123, 76]);
}

pub fn scaffold() -> Mesh {
    let mut b = Batch::default();
    let h = 1.54;
    for z in [-0.98, 1.14] {
        for x in [-1.39, 1.39] {
            b.slab(v(x, h / 2.0, z), v(0.066, h, 0.061), shift(OAK, 30));
            b.slab(v(x, 0.035, z), v(0.207, 0.070, 0.172), CREAM);
        }
        b.beam(v(-1.38, 0.28, z), v(1.38, 1.38, z), 0.041, OAK);
        b.beam(v(-1.38, 1.38, z), v(1.38, 0.24, z), 0.039, OAK);
        for y in [0.73, 1.45] {
            b.slab(v(0.0, y, z), v(2.84, 0.060, 0.085), [171, 115, 72]);
        }
    }
    for x in [-1.39, 1.39] {
        b.beam(v(x, 1.46, -1.04), v(x, 1.46, 1.18), 0.068, OAK);
        for i in 0..5 {
            b.slab(
                v(x, 1.507, -0.84 + i as f32 * 0.36),
                v(0.27, 0.04, 0.27),
                shift(CREAM, (i * 4) as i16),
            );
        }
    }
    for n in 0..5 {
        let x = n as f32 * 0.39 - 0.78;
        b.face(
            &[
                v(x, 1.45, 1.16),
                v(x + 0.14, 1.26, 1.19),
                v(x + 0.28, 1.45, 1.16),
            ],
            if n % 2 == 0 { GOLD } else { MARBLE },
        );
    }
    b.mesh()
}

pub fn chip() -> Mesh {
    let mut b = Batch::default();
    b.gem(Vec3::ZERO, 0.081, 0.067, 0.096, 0.18, [217, 182, 137]);
    b.mesh()
}

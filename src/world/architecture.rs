use bevy::prelude::*;

use super::mesh::{Batch, Tint, shift, v};

pub(super) fn plinth(b: &mut Batch, at: Vec3, width: f32, depth: f32, height: f32, stone: Tint) {
    b.slab(
        at + v(0.0, height * 0.38, 0.0),
        v(width, height * 0.76, depth),
        shift(stone, -19),
    );
    b.slab(
        at + v(0.0, height * 0.87, 0.0),
        v(width - 0.07, height * 0.26, depth - 0.07),
        stone,
    );
}

// Each tread runs up to the next; the front edge stays within the plot.
pub(super) fn stoop(
    b: &mut Batch,
    at: Vec3,
    width: f32,
    height: f32,
    run: f32,
    steps: u8,
    stone: Tint,
) {
    for i in 0..steps {
        let fraction = f32::from(i + 1) / f32::from(steps);
        let y = height * fraction;
        let tread = run / f32::from(steps);
        b.slab(
            at + v(0.0, y * 0.5, run - (f32::from(i) + 0.5) * tread),
            v(width - f32::from(i) * 0.048, y, tread + 0.011),
            shift(stone, i16::from(i) * 3 - 5),
        );
    }
}

// The ridge cap and ceramic courses follow the roof all the way to its eaves.
pub(super) fn gable(b: &mut Batch, at: Vec3, width: f32, depth: f32, rise: f32, tile: Tint) {
    let (x, z) = (width * 0.5, depth * 0.5);
    let a = |x, y, z| at + v(x, y, z);
    b.face(
        &[a(-x, 0.0, z), a(x, 0.0, z), a(0.0, rise, z)],
        shift(tile, -23),
    );
    b.face(
        &[a(x, 0.0, -z), a(-x, 0.0, -z), a(0.0, rise, -z)],
        shift(tile, -19),
    );
    let rows = ((x / 0.17).round() as usize).max(2);
    let tiles = ((depth / 0.19).round() as usize).max(4);
    for side in [-1.0, 1.0] {
        let point = |across: f32, along: f32, lift: f32| {
            a(side * across * x, rise * (1.0 - across) + lift, along)
        };
        for row in 0..rows {
            let p = row as f32 / rows as f32;
            let q = (row + 1) as f32 / rows as f32;
            for n in 0..tiles {
                let spacing = depth / tiles as f32;
                let zig = if row % 2 == 0 { 0.0 } else { spacing * 0.39 };
                let z0 = (-z + n as f32 * spacing - zig).max(-z);
                let z1 = if n == tiles - 1 {
                    z
                } else {
                    (-z + (n + 1) as f32 * spacing - zig).min(z)
                };
                let middle = (z0 + z1) * 0.5;
                for (from, to, shade) in [(z0, middle, 0), (middle, z1, 11)] {
                    let lip = if from == z0 { 0.0 } else { 0.016 };
                    let ridge = if to == z1 { 0.0 } else { 0.016 };
                    let mut face = [
                        point(p, from, lip),
                        point(p, to, ridge),
                        point(q, to, ridge - 0.01),
                        point(q, from, lip - 0.01),
                    ];
                    if side < 0.0 {
                        face.reverse();
                    }
                    b.face(
                        &face,
                        shift(tile, (n * 19 + row * 11) as i16 % 22 - 9 + shade),
                    );
                }
            }
            b.beam(
                point(p, z, 0.009),
                point(q, z, 0.009),
                0.047,
                shift(tile, 14),
            );
        }
        b.beam(
            point(1.0, -z, -0.017),
            point(1.0, z, -0.017),
            0.071,
            shift(tile, -29),
        );
    }
    b.beam(
        a(0.0, rise + 0.019, -z - 0.023),
        a(0.0, rise + 0.019, z + 0.023),
        0.092,
        shift(tile, 16),
    );
}

pub(super) fn panel_roof(b: &mut Batch, at: Vec3, width: f32, depth: f32, pitch: f32, tile: Tint) {
    let (sin, cos) = pitch.sin_cos();
    let slope = Quat::from_rotation_x(pitch);
    let n = (width / 0.17).round().max(3.0) as usize;
    for i in 0..n {
        let x = ((i as f32 + 0.5) / n as f32 - 0.5) * width;
        b.block(
            at + v(x, 0.0, 0.0),
            v(width / n as f32 * 0.985, 0.075, depth),
            slope,
            shift(tile, (i * 17 % 25) as i16 - 9),
        );
    }
    let edge = at + v(0.0, -sin * depth * 0.5, cos * depth * 0.5);
    b.beam(
        edge + v(-width * 0.503, -0.011, 0.0),
        edge + v(width * 0.503, -0.011, 0.0),
        0.061,
        shift(tile, -25),
    );
}

pub(super) fn tank(
    b: &mut Batch,
    at: Vec3,
    width: f32,
    depth: f32,
    height: f32,
    stone: Tint,
    water: Tint,
) {
    let x = width * 0.5 - 0.056;
    let z = depth * 0.5 - 0.056;
    b.slab(
        at + v(0.0, height * 0.42, 0.0),
        v(width - 0.12, height * 0.84, depth - 0.12),
        shift(stone, -14),
    );
    for side in [-1.0, 1.0] {
        b.slab(
            at + v(x * side, height * 0.5, 0.0),
            v(0.11, height, depth),
            stone,
        );
        b.slab(
            at + v(0.0, height * 0.53, z * side),
            v(width - 0.226, height * 0.94, 0.11),
            shift(stone, 8),
        );
    }
    b.slab(
        at + v(0.0, height * 0.86, 0.0),
        v(width - 0.219, 0.012, depth - 0.219),
        water,
    );
    for (i, span) in [0.35, 0.22, 0.28].into_iter().enumerate() {
        let xx = (i as f32 - 1.0) * (width * 0.18);
        b.slab(
            at + v(
                xx,
                height * 0.86 + 0.0061,
                (i as f32 - 1.0) * (depth * 0.15),
            ),
            v((width - 0.22) * span, 0.003, 0.016),
            shift(water, 36),
        );
    }
}

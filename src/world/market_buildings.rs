use bevy::prelude::*;

use super::{
    architecture::{gable, panel_roof, plinth, stoop, tank},
    buildings::{
        BLUE, CREAM, GOLD, HERB, IVORY, OAK, TEAL, TERRACOTTA, amphora, awning, column, flowerpot,
        olive, window,
    },
    japan,
    mesh::{Batch, shift, v},
    nordic, roman,
    upgrades::{bench, crate_box, rail},
};
use crate::model::Kind;

pub(super) fn greek(b: &mut Batch, kind: Kind, level: u8) {
    match kind {
        Kind::Apiary => {
            let honey = [211, 151, 61];
            plinth(b, Vec3::ZERO, 2.69, 2.19, 0.13, CREAM);
            let hives = [(-0.73, 0.43), (0.66, 0.45), (-0.80, -0.54), (0.48, -0.51)];
            for (i, (x, z)) in hives
                .into_iter()
                .enumerate()
                .take(2 + usize::from(level >= 2) + usize::from(level >= 4))
            {
                let base = v(x, 0.13, z);
                let height = 0.43 + (i % 2) as f32 * 0.11 + f32::from(level >= 5) * 0.16;
                b.frustum(base, 0.28, 0.31, height * 0.68, 12, honey);
                b.frustum(
                    base + Vec3::Y * (height * 0.68),
                    0.31,
                    0.12,
                    height * 0.32,
                    12,
                    [227, 179, 88],
                );
                for j in 0..4 {
                    let y = height * (0.12 + j as f32 * 0.15);
                    b.frustum(
                        base + Vec3::Y * y,
                        0.292 + j as f32 * 0.008,
                        0.293 + j as f32 * 0.008,
                        0.023,
                        12,
                        [164, 111, 58],
                    );
                }
                b.slab(base + v(0.0, 0.051, 0.301), v(0.12, 0.073, 0.07), OAK);
                flowerpot(
                    b,
                    base + v(0.36, 0.01, 0.17),
                    0.57,
                    [215, 141 + i as u8 * 13, 158],
                );
            }
            if level >= 3 {
                for x in [-1.14, 1.13] {
                    b.beam(v(x, 0.13, -0.75), v(x, 1.37, -0.75), 0.078, OAK);
                }
                panel_roof(b, v(0.0, 1.42, -0.66), 2.52, 0.72, 0.22, HERB);
            }
            if level >= 5 {
                for j in -2..=2 {
                    flowerpot(b, v(j as f32 * 0.42, 0.13, 0.91), 0.58, [199, 125, 183]);
                }
            }
            if level >= 6 {
                bench(b, v(0.03, 0.13, -0.21), 1.18);
                for x in [-0.37, 0.0, 0.37] {
                    amphora(b, v(x, 0.43, -0.22), 0.59, GOLD);
                }
                b.beam(v(-1.12, 1.37, -0.52), v(1.11, 1.37, -0.52), 0.065, OAK);
            }
        }
        Kind::Dyer => {
            let dye = [139, 70, 122];
            let vats = [[141, 69, 115], [70, 116, 154], [205, 125, 65]];
            plinth(b, Vec3::ZERO, 2.69, 2.18, 0.12, CREAM);
            let frame = if level >= 5 { 1.78 } else { 1.38 };
            for x in [-1.09, 1.09] {
                for z in [-0.76, -0.17] {
                    column(b, v(x, 0.12, z), frame - 0.10, 0.065);
                }
            }
            panel_roof(b, v(0.0, frame + 0.16, -0.47), 2.56, 0.94, 0.17, dye);
            b.beam(v(-1.12, frame, -0.126), v(1.12, frame, -0.126), 0.063, OAK);
            let colors = if level >= 4 { 3 } else { 2 };
            for (i, &color) in vats.iter().take(colors).enumerate() {
                let x = (i as f32 - (colors - 1) as f32 * 0.5) * 0.76;
                let h = if level >= 6 {
                    0.43
                } else {
                    0.31 + f32::from(level) * 0.015
                };
                tank(b, v(x, 0.12, 0.67), 0.61, 0.68, h, IVORY, color);
                if level >= 2 {
                    b.slab(
                        v(x, frame - 0.27, -0.136),
                        v(0.54, 0.43 + f32::from(level) * 0.025, 0.047),
                        color,
                    );
                    b.slab(
                        v(x, frame - 0.45, -0.103),
                        v(0.54, 0.042, 0.018),
                        shift(color, 29),
                    );
                }
            }
            if level >= 3 {
                b.beam(
                    v(-1.08, 0.84, -0.76),
                    v(-1.08, frame - 0.05, -0.16),
                    0.059,
                    OAK,
                );
                amphora(b, v(-1.15, 0.13, 0.14), 0.75, dye);
            }
            if level >= 6 {
                for x in [-1.20, 1.20] {
                    b.beam(v(x, 0.12, 0.99), v(x, 1.22, 0.99), 0.066, OAK);
                }
                b.beam(v(-1.22, 1.21, 0.99), v(1.22, 1.21, 0.99), 0.047, OAK);
                for (x, t) in [(-0.81, BLUE), (0.80, dye)] {
                    b.block(
                        v(x, 0.91, 0.99),
                        v(0.41, 0.58, 0.035),
                        Quat::from_rotation_x(-0.16),
                        t,
                    );
                }
            }
        }
        Kind::FerryOffice => {
            let h = 1.14 + f32::from(level - 1) * 0.15;
            let flag = [199, 111, 59];
            plinth(b, Vec3::ZERO, 2.70, 2.23, 0.18, IVORY);
            b.slab(
                v(-0.26, 0.18 + h * 0.5, -0.33),
                v(1.78, h, 1.21),
                [230, 214, 180],
            );
            gable(b, v(-0.26, 0.18 + h, -0.33), 2.0, 1.47, 0.46, BLUE);
            let z = 0.295;
            window(b, -0.56, 0.79, z, BLUE);
            b.slab(v(-0.56, 0.48, 0.54), v(0.86, 0.10, 0.44), OAK);
            for i in 0..level.min(4) {
                let x = -1.02 + i as f32 * 0.26;
                b.slab(
                    v(x, 0.27, 0.315),
                    v(0.14, 0.17, 0.034),
                    if i % 2 == 0 { BLUE } else { flag },
                );
            }
            let mast = v(1.00, 0.18, -0.32);
            b.beam(mast, mast + Vec3::Y * (h + 0.97), 0.083, OAK);
            b.gem(mast + Vec3::Y * (h + 1.00), 0.087, 0.09, 0.085, 0.1, GOLD);
            for i in 0..1 + level / 2 {
                b.slab(
                    mast + v(0.0, h + 0.74 - f32::from(i) * 0.23, 0.19),
                    v(0.053, 0.19, 0.43),
                    if i % 2 == 0 { flag } else { TEAL },
                );
            }
            if level >= 3 {
                awning(b, 0.38, 1.18, 2.21, BLUE);
                bench(b, v(0.70, 0.18, 0.68), 0.59);
            }
            if level >= 5 {
                window(b, -0.04, h - 0.16, z, flag);
                rail(b, v(0.64, 0.18, 1.05), 0.93, BLUE);
            }
            if level >= 6 {
                stoop(b, v(-0.43, 0.0, 1.0), 1.10, 0.18, 0.25, 3, CREAM);
                column(b, v(0.91, 0.18, 0.34), 1.12, 0.072);
                b.gem(v(0.91, 1.39, 0.34), 0.11, 0.13, 0.096, 0.23, GOLD);
            }
        }
        _ => unreachable!(),
    }
}

pub(super) fn roman(b: &mut Batch, kind: Kind, level: u8) {
    let brick = [175, 94, 69];
    let travertine = [223, 193, 145];
    plinth(b, Vec3::ZERO, 2.74, 2.29, 0.14, travertine);
    match kind {
        Kind::Mill => {
            let diameter = 0.34 + f32::from(level) * 0.035;
            let q = v(-0.47, 0.14, 0.46);
            b.frustum(q, diameter + 0.12, diameter, 0.22, 13, [143, 126, 112]);
            b.frustum(
                q + Vec3::Y * 0.22,
                diameter,
                diameter * 0.43,
                0.29,
                13,
                travertine,
            );
            b.frustum(
                q + Vec3::Y * 0.51,
                diameter * 0.35,
                diameter * 0.80,
                0.28,
                13,
                [178, 148, 114],
            );
            b.beam(
                q + v(-0.70, 0.58, 0.016),
                q + v(0.51, 0.58, 0.016),
                0.087,
                OAK,
            );
            let height = 0.83 + f32::from(level) * 0.13;
            b.slab(
                v(0.47, 0.14 + height / 2.0, -0.56),
                v(1.46, height, 0.87),
                brick,
            );
            gable(
                b,
                v(0.47, 0.14 + height, -0.56),
                1.68,
                1.04,
                0.39,
                TERRACOTTA,
            );
            roman::arch(
                b,
                v(0.42, 0.14, -0.09),
                0.56,
                height * 0.63,
                0.091,
                0.11,
                travertine,
            );
            b.slab(v(0.42, 0.48, -0.094), v(0.54, 0.64, 0.052), OAK);
            for i in 0..level.min(4) {
                let x = 0.43 + f32::from(i % 2) * 0.39;
                let z = 0.54 + f32::from(i / 2) * 0.36;
                b.gem(v(x, 0.31, z), 0.20, 0.20, 0.165, z, [207, 167, 110]);
            }
            if level >= 3 {
                crate_box(b, v(0.98, 0.14, 0.11), v(0.49, 0.39, 0.42), OAK);
            }
            if level >= 5 {
                for x in [-1.13, 0.16] {
                    b.beam(v(x, 0.14, 1.03), v(x, 1.47, 1.03), 0.08, OAK);
                }
                panel_roof(b, v(-0.49, 1.53, 0.57), 1.57, 1.02, 0.20, [154, 84, 60]);
            }
            if level >= 6 {
                roman::mosaic(b, v(0.56, 0.16, 1.0), 7, 2, 0.13);
                window(b, 0.67, height - 0.03, -0.111, brick);
            }
        }
        Kind::Scribe => {
            let h = 1.34 + f32::from(level >= 6) * 0.54;
            let ink = [121, 64, 102];
            b.slab(
                v(0.0, 0.14 + h * 0.5, -0.48),
                v(2.33, h, 1.03),
                [217, 170, 133],
            );
            roman::masonry(b, v(0.0, 0.14, 0.051), 2.31, h, brick);
            gable(b, v(0.0, h + 0.14, -0.48), 2.49, 1.26, 0.39, TERRACOTTA);
            b.slab(v(0.0, 0.55, 0.47), v(1.80, 0.16, 0.61), OAK);
            for x in [-0.73, 0.73] {
                b.slab(v(x, 0.32, 0.47), v(0.115, 0.36, 0.41), OAK);
            }
            for i in 0..2 + level.min(5) {
                let x = (f32::from(i) - f32::from(1 + level.min(5)) * 0.5) * 0.22;
                b.block(
                    v(x, 0.65, 0.58 + (x * 9.0).sin() * 0.08),
                    v(0.18, 0.048, 0.25),
                    Quat::from_rotation_y(x * 0.35),
                    travertine,
                );
                b.slab(v(x, 0.676, 0.57), v(0.092, 0.008, 0.027), ink);
            }
            if level >= 2 {
                roman::mosaic(b, v(0.0, 0.16, 0.92), 13, 3, 0.145);
            }
            if level >= 3 {
                for x in [-1.09, 1.09] {
                    column(b, v(x, 0.14, 0.90), 1.16, 0.065);
                }
                panel_roof(b, v(0.0, 1.41, 0.55), 2.51, 0.98, 0.16, ink);
            }
            if level >= 4 {
                for x in [-0.75, 0.75] {
                    roman::arch(b, v(x, 0.14, 0.133), 0.53, 0.72, 0.10, 0.15, travertine);
                }
            }
            if level >= 5 {
                b.slab(v(0.0, 1.33, 1.03), v(1.48, 0.11, 0.071), GOLD);
                b.gem(v(0.0, 1.23, 1.07), 0.15, 0.16, 0.049, 0.43, ink);
            }
            if level >= 6 {
                window(b, 0.0, 1.76, 0.064, ink);
                for x in [-0.86, 0.86] {
                    window(b, x, 1.76, 0.064, ink);
                }
            }
        }
        Kind::GardenCourt => {
            let pool = [64, 151, 157];
            let width = 0.90 + f32::from(level) * 0.19;
            tank(b, v(0.0, 0.14, 0.33), width, 0.84, 0.26, travertine, pool);
            roman::mosaic(b, v(0.0, 0.16, 0.94), 15, 3, 0.155);
            for side in [-1.0, 1.0] {
                olive(
                    b,
                    v(0.69 * side, 0.14, -0.68),
                    0.70 + f32::from(level) * 0.026,
                );
                flowerpot(b, v(1.12 * side, 0.14, 0.69), 0.75, [211, 115, 144]);
            }
            if level >= 2 {
                b.frustum(v(0.0, 0.31, 0.29), 0.16, 0.11, 0.27, 10, travertine);
                b.frustum(v(0.0, 0.57, 0.29), 0.11, 0.25, 0.092, 10, travertine);
                b.gem(v(0.0, 0.72, 0.29), 0.058, 0.15, 0.050, 0.1, pool);
            }
            if level >= 3 {
                let y = if level >= 5 { 1.03 } else { 0.80 };
                for x in [-0.69, 0.69] {
                    roman::arch(
                        b,
                        v(x, 0.14, -0.43),
                        0.79,
                        y,
                        0.14,
                        0.33,
                        if level >= 5 { travertine } else { brick },
                    );
                }
                b.slab(v(0.0, y + 0.72, -0.43), v(2.66, 0.18, 0.45), travertine);
                panel_roof(
                    b,
                    v(0.0, y + 0.85, -0.43),
                    2.71,
                    0.58,
                    0.15,
                    if level >= 6 { TEAL } else { TERRACOTTA },
                );
            }
            if level >= 4 {
                for x in [-1.03, 1.03] {
                    bench(b, v(x, 0.14, -0.12), 0.51);
                }
            }
            if level >= 6 {
                for i in 0..10 {
                    let x = -1.04 + i as f32 * 0.23;
                    b.gem(
                        v(x, 2.00, -0.22),
                        0.20,
                        0.11,
                        0.17,
                        x,
                        shift(HERB, 13 + i * 2),
                    );
                }
            }
        }
        _ => unreachable!(),
    }
}

pub(super) fn japan(b: &mut Batch, kind: Kind, level: u8) {
    let cedar = [127, 84, 54];
    let paper = [239, 212, 158];
    let charcoal = [58, 70, 81];
    let stone = [168, 160, 142];
    plinth(b, Vec3::ZERO, 2.76, 2.23, 0.17, stone);
    match kind {
        Kind::MarketGarden => {
            let hue = [
                [114, 149, 91],
                [84, 133, 109],
                [181, 105, 66],
                [134, 134, 71],
            ];
            let mut rows = vec![(-0.70, -0.25), (0.0, -0.25), (0.70, -0.25)];
            if level >= 2 {
                rows.push((0.0, 0.73));
            }
            for (i, (x, z)) in rows.into_iter().enumerate() {
                let h = if level >= 4 { 0.30 } else { 0.18 };
                let depth = if i == 3 { 0.56 } else { 1.22 };
                plinth(b, v(x, 0.17, z), 0.63, depth, h, cedar);
                b.slab(
                    v(x, 0.176 + h, z),
                    v(0.51, 0.021, depth - 0.12),
                    [91, 69, 51],
                );
                for n in 0..if i == 3 { 3 } else { 6 } {
                    let dz = (n as f32 / if i == 3 { 2.0 } else { 5.0 } - 0.5) * (depth - 0.26);
                    let at = v(x + (n % 2) as f32 * 0.11 - 0.055, 0.27 + h, z + dz);
                    b.gem(
                        at,
                        0.145,
                        0.14 + f32::from(level) * 0.014,
                        0.14,
                        0.9 * n as f32,
                        shift(hue[i], (n * 6 % 18) as i16),
                    );
                }
            }
            if level >= 3 {
                tank(
                    b,
                    v(-0.81, 0.17, 0.79),
                    0.69,
                    0.51,
                    0.22,
                    stone,
                    [90, 156, 153],
                );
            }
            if level >= 5 {
                for x in [-1.22, 1.22] {
                    b.beam(v(x, 0.17, -0.72), v(x, 1.31, -0.72), 0.072, cedar);
                }
                b.beam(v(-1.29, 1.29, -0.72), v(1.29, 1.29, -0.72), 0.069, cedar);
                for i in 0..9 {
                    let x = -1.12 + i as f32 * 0.28;
                    b.beam(v(x, 1.31, -0.96), v(x, 1.31, -0.44), 0.039, cedar);
                    b.gem(
                        v(x, 1.38, -0.65),
                        0.22,
                        0.124,
                        0.25,
                        x,
                        shift(hue[0], i * 4),
                    );
                }
            }
            if level >= 6 {
                japan::stone_lantern(b, v(0.93, 0.17, 0.81), 0.80);
                japan::bamboo(b, v(-1.02, 0.17, -0.84), 0.63);
            }
        }
        Kind::CeramicStudio => {
            let h = 1.18 + f32::from(level >= 5) * 0.42;
            let glaze = [75, 122, 127];
            b.slab(v(-0.29, 0.17 + h * 0.5, -0.41), v(1.90, h, 1.18), paper);
            japan::roof(b, v(-0.28, h + 0.19, -0.39), 1.08, 0.78, 0.34, glaze);
            japan::lattice(b, v(-0.48, 0.75, 0.196), 0.84, 0.61, cedar);
            if level >= 5 {
                japan::lattice(b, v(-0.48, h - 0.02, 0.196), 0.77, 0.28, paper);
            }
            let kiln = v(0.87, 0.17, 0.51);
            let oven = 0.39 + f32::from(level) * 0.085;
            b.frustum(kiln, 0.37, 0.38, oven * 0.52, 12, [149, 89, 72]);
            b.frustum(
                kiln + Vec3::Y * (oven * 0.51),
                0.38,
                0.18,
                oven * 0.49,
                12,
                [183, 113, 79],
            );
            b.frustum(
                kiln + Vec3::Y * oven,
                0.17,
                0.17,
                if level >= 6 { 0.39 } else { 0.12 },
                12,
                charcoal,
            );
            b.slab(kiln + v(0.0, 0.27, 0.367), v(0.26, 0.19, 0.05), cedar);
            b.gem(kiln + v(0.0, 0.25, 0.404), 0.09, 0.068, 0.022, 0.1, GOLD);
            let count = usize::from(level.min(4)) + 2;
            b.slab(v(-0.53, 0.48, 0.70), v(1.40, 0.11, 0.55), cedar);
            for i in 0..count {
                let x = -1.05 + (i % 3) as f32 * 0.42;
                let z = 0.59 + (i / 3) as f32 * 0.26;
                b.frustum(
                    v(x, 0.535, z),
                    0.079,
                    0.132,
                    0.128,
                    10,
                    [glaze, [187, 112, 90], paper][i % 3],
                );
                b.frustum(v(x, 0.664, z), 0.113, 0.106, 0.012, 10, charcoal);
            }
            if level >= 3 {
                for x in [-1.24, 0.26] {
                    b.beam(v(x, 0.17, 1.02), v(x, 1.24, 1.02), 0.067, cedar);
                }
                panel_roof(b, v(-0.47, 1.31, 0.72), 1.78, 0.77, 0.19, glaze);
            }
            if level >= 4 {
                japan::lantern(b, v(-1.15, 1.01, 1.07), paper, 0.88);
            }
            if level >= 6 {
                for i in 0..3 {
                    amphora(b, v(0.62 + i as f32 * 0.24, 0.17, 1.02), 0.47, glaze);
                }
            }
        }
        Kind::SilkShop => {
            let plum = [131, 71, 105];
            let h = 1.45 + f32::from(level.saturating_sub(3)) * 0.18;
            b.slab(v(0.0, (h + 0.34) / 2.0, -0.43), v(2.24, h, 1.15), paper);
            for x in [-1.05, 1.05] {
                b.beam(v(x, 0.17, 0.165), v(x, h + 0.17, 0.165), 0.092, cedar);
            }
            japan::roof(
                b,
                v(0.0, h + 0.18, -0.44),
                1.23,
                0.73,
                0.45,
                if level >= 5 { plum } else { charcoal },
            );
            japan::lattice(b, v(0.0, 0.82, 0.16), 1.66, 1.03, cedar);
            let beam = if level >= 4 { 1.50 } else { 1.27 };
            for x in [-1.05, 1.05] {
                b.beam(v(x, 0.17, 0.88), v(x, beam + 0.02, 0.88), 0.079, cedar);
            }
            b.beam(v(-1.11, beam, 0.89), v(1.11, beam, 0.89), 0.064, cedar);
            for i in 0..3 + usize::from(level >= 2) + usize::from(level >= 5) {
                let x = -0.88 + i as f32 * 0.37;
                let hue = [plum, [87, 114, 153], [176, 86, 72], [101, 133, 97], GOLD][i];
                let drop = 0.51 + (i % 2) as f32 * 0.13;
                b.block(
                    v(x, beam - drop * 0.49, 0.88),
                    v(0.29, drop, 0.048),
                    Quat::from_rotation_x(-0.075),
                    hue,
                );
                b.slab(
                    v(x, beam - drop + 0.04, 0.928),
                    v(0.26, 0.033, 0.016),
                    paper,
                );
            }
            if level >= 3 {
                panel_roof(b, v(0.0, beam + 0.078, 0.62), 2.48, 0.88, 0.16, plum);
            }
            if level >= 4 {
                b.slab(v(0.0, 0.23, 0.77), v(2.13, 0.13, 0.60), cedar);
            }
            if level >= 6 {
                japan::lattice(b, v(0.0, h - 0.03, 0.166), 1.51, 0.33, paper);
                for x in [-1.15, 1.15] {
                    japan::lantern(b, v(x, 1.18, 1.06), paper, 0.85);
                }
            }
        }
        _ => unreachable!(),
    }
}

pub(super) fn nordic(b: &mut Batch, kind: Kind, level: u8) {
    let fir = [120, 80, 55];
    let iron = [79, 92, 97];
    let pale = [207, 171, 117];
    let grass = [123, 139, 85];
    let brick = [162, 88, 61];
    let baseline = if kind == Kind::Sheepfold {
        grass
    } else {
        [143, 153, 143]
    };
    plinth(b, Vec3::ZERO, 2.76, 2.25, 0.13, baseline);
    match kind {
        Kind::Sheepfold => {
            let sides = if level >= 4 { 0.49 } else { 0.33 };
            for x in [-1.18, 1.18] {
                for z in [-0.87, -0.11, 0.68] {
                    b.beam(v(x, 0.13, z), v(x, sides + 0.18, z), 0.080, fir);
                }
                for y in [0.27, sides + 0.09] {
                    b.beam(v(x, y, -0.88), v(x, y, 0.70), 0.056, pale);
                }
            }
            for y in [0.27, sides + 0.09] {
                b.beam(v(-1.18, y, -0.87), v(1.18, y, -0.87), 0.056, pale);
            }
            let flock = [
                (-0.55, 0.53, 1.0),
                (0.43, 0.26, 0.86),
                (0.44, -0.53, 0.79),
                (-0.59, -0.36, 0.69),
            ];
            let count = 2 + usize::from(level >= 2) + usize::from(level >= 5);
            for (i, (x, z, scale)) in flock.into_iter().take(count).enumerate() {
                let at = v(x, 0.13, z);
                let fleece = if i == 2 {
                    [160, 133, 112]
                } else {
                    [231, 217, 183]
                };
                for dx in [-0.12, 0.12] {
                    for dz in [-0.14, 0.13] {
                        b.slab(
                            at + v(dx, 0.12, dz) * scale,
                            v(0.049, 0.24, 0.052) * scale,
                            fir,
                        );
                    }
                }
                b.gem(
                    at + v(0.0, 0.34, -0.018) * scale,
                    0.25 * scale,
                    0.20 * scale,
                    0.32 * scale,
                    x,
                    fleece,
                );
                b.gem(
                    at + v(0.0, 0.36, 0.24) * scale,
                    0.132 * scale,
                    0.122 * scale,
                    0.154 * scale,
                    x,
                    [126, 99, 77],
                );
                for side in [-1.0, 1.0] {
                    b.gem(
                        at + v(side * 0.12, 0.429, 0.255) * scale,
                        0.087 * scale,
                        0.048 * scale,
                        0.072 * scale,
                        side,
                        fleece,
                    );
                    b.gem(
                        at + v(side * 0.064, 0.39, 0.335) * scale,
                        0.027 * scale,
                        0.028 * scale,
                        0.015 * scale,
                        0.1,
                        iron,
                    );
                }
            }
            if level >= 3 {
                tank(
                    b,
                    v(0.85, 0.13, 0.70),
                    0.54,
                    0.65,
                    0.21,
                    fir,
                    [97, 143, 152],
                );
            }
            if level >= 5 {
                nordic::shed(
                    b,
                    v(-0.69, 0.13, -0.68),
                    0.43,
                    if level >= 6 { 1.24 } else { 0.97 },
                    grass,
                );
            }
            if level >= 6 {
                nordic::wooden_rail(b, v(0.73, 0.13, 1.05), 0.89);
                for x in [-1.12, 1.08] {
                    b.gem(v(x, 0.24, 1.00), 0.16, 0.17, 0.15, x, [206, 164, 83]);
                }
            }
        }
        Kind::Cooperage => {
            let h = 1.30 + f32::from(level.saturating_sub(4)) * 0.18;
            nordic::wall(
                b,
                v(-0.16, 0.13 + h / 2.0, -0.48),
                1.89,
                h,
                0.93,
                [162, 118, 79],
            );
            nordic::shingle(
                b,
                v(-0.16, h + 0.13, -0.47),
                1.15,
                0.68,
                0.48,
                if level >= 5 { iron } else { brick },
            );
            b.slab(v(-0.32, 0.66, 0.013), v(0.82, 0.87, 0.055), fir);
            let casks = [(-0.86, 0.66), (0.76, 0.79), (0.68, 0.19), (-0.25, 0.83)];
            for (i, (x, z)) in casks
                .into_iter()
                .take(2 + usize::from(level >= 2) + usize::from(level >= 4))
                .enumerate()
            {
                let r = 0.235 + (i % 2) as f32 * 0.06;
                let p = v(x, 0.13, z);
                let y = 0.55 + (i % 2) as f32 * 0.17;
                b.frustum(p, r * 0.82, r, y * 0.53, 12, fir);
                b.frustum(
                    p + Vec3::Y * (y * 0.53),
                    r,
                    r * 0.79,
                    y * 0.47,
                    12,
                    [171, 119, 75],
                );
                for fraction in [0.19, 0.51, 0.86] {
                    let radius = r * (1.01 - (fraction - 0.51_f32).abs() * 0.52);
                    b.frustum(
                        p + Vec3::Y * (y * fraction),
                        radius + 0.013,
                        radius + 0.013,
                        0.032,
                        12,
                        iron,
                    );
                }
                b.frustum(
                    p + v(0.0, y + 0.004, 0.0),
                    r * 0.68,
                    r * 0.69,
                    0.017,
                    12,
                    pale,
                );
            }
            if level >= 3 {
                nordic::awning(b, brick, 1.21);
            }
            if level >= 5 {
                for x in [-0.69, 0.40] {
                    b.slab(v(x, h - 0.05, 0.01), v(0.23, 0.28, 0.069), iron);
                }
            }
            if level >= 6 {
                b.slab(v(0.0, 1.39, 1.08), v(0.65, 0.23, 0.094), brick);
                nordic::brooch(b, v(0.0, 1.39, 1.134), 0.13, GOLD);
            }
        }
        Kind::SignalTower => {
            let height = 1.35 + f32::from(level - 1) * 0.23;
            for x in [-0.53, 0.53] {
                for z in [-0.54, 0.47] {
                    b.beam(v(x * 1.23, 0.13, z * 1.23), v(x, height, z), 0.121, fir);
                    b.slab(v(x, 0.21, z), v(0.29, 0.16, 0.28), iron);
                }
                b.beam(v(x, 0.27, -0.63), v(x, height - 0.12, 0.41), 0.079, pale);
            }
            b.slab(v(0.0, height, -0.06), v(1.34, 0.18, 1.29), fir);
            for depth in [-0.61, 0.49] {
                nordic::wooden_rail(b, v(0.0, height + 0.08, depth), 1.28);
            }
            let crown = v(0.0, height + 0.12, -0.12);
            b.frustum(crown, 0.19, 0.35, 0.24, 12, iron);
            b.frustum(crown + Vec3::Y * 0.23, 0.35, 0.36, 0.05, 12, [98, 106, 104]);
            b.gem(
                crown + Vec3::Y * 0.35,
                0.22,
                0.21,
                0.20,
                0.24,
                [234, 129, 54],
            );
            b.gem(
                crown + v(0.05, 0.47, 0.026),
                0.103,
                0.18,
                0.091,
                0.5,
                [254, 189, 87],
            );
            for x in [-0.17, 0.17] {
                b.beam(v(x, 0.15, 1.05), v(x, height - 0.013, 0.49), 0.06, fir);
            }
            for i in 0..level + 3 {
                let fraction = f32::from(i + 1) / f32::from(level + 4);
                let at = v(
                    0.0,
                    0.15 + (height - 0.163) * fraction,
                    1.05 - 0.56 * fraction,
                );
                b.beam(at + v(-0.21, 0.0, 0.0), at + v(0.21, 0.0, 0.0), 0.043, pale);
            }
            if level >= 3 {
                nordic::shed(b, v(-0.90, 0.13, -0.56), 0.39, 0.84, grass);
            }
            if level >= 4 {
                b.beam(
                    v(0.61, height - 0.1, 0.02),
                    v(0.61, height + 0.91, 0.02),
                    0.053,
                    fir,
                );
                b.slab(v(0.86, height + 0.70, 0.02), v(0.46, 0.21, 0.041), brick);
            }
            if level >= 5 {
                b.slab(v(0.02, height + 0.14, 0.590), v(0.32, 0.35, 0.074), brick);
                nordic::brooch(b, v(0.02, height + 0.14, 0.645), 0.13, GOLD);
            }
            if level >= 6 {
                nordic::wooden_rail(b, v(0.0, 0.13, 1.06), 1.55);
                crate_box(b, v(0.83, 0.13, 0.77), v(0.62, 0.37, 0.54), fir);
            }
        }
        _ => unreachable!(),
    }
}

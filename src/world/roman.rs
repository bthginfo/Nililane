use bevy::prelude::*;

use super::{
    architecture::{gable, panel_roof, stoop, tank},
    buildings::{
        BLUE, GOLD, MARBLE, OAK, TEAL, amphora, awning, cypress, flowerpot, olive, oven, roof,
        shelf,
    },
    mesh::{Batch, Grain, Tint, shift, v},
    upgrades::{bench, crane, crate_box, rail},
};
use crate::model::Kind;

const BRICK: Tint = [166, 81, 57];
const TILE: Tint = [164, 69, 49];
const TRAVERTINE: Tint = [217, 189, 153];
const SANDSTONE: Tint = [185, 157, 130];
const PLASTER: Tint = [220, 148, 105];
const WINE: Tint = [143, 53, 62];
const BASALT: Tint = [109, 113, 115];
const POOL: Tint = [74, 152, 167];

pub fn model(kind: Kind, level: u8) -> Mesh {
    let mut b = Batch::default();
    match kind {
        Kind::House => insula(&mut b, false, level),
        Kind::Villa => insula(&mut b, true, level),
        Kind::Well => cistern(&mut b, level),
        Kind::Grove => garden(&mut b, level),
        Kind::Stoa => portico(&mut b, level),
        Kind::Storehouse => horreum(&mut b, level),
        Kind::Aqueduct => pipes(&mut b, level),
        Kind::Baths => baths(&mut b, level),
        Kind::Forum => forum(&mut b, level),
        Kind::Bakery => super::buildings::bakehouse(&mut b, true, level),
        Kind::Taverna => super::buildings::tavern(&mut b, true, level),
        Kind::Potter | Kind::Inn => shop(&mut b, kind, level),
        Kind::HerbGarden => super::buildings::herb_garden(&mut b, true, level),
        Kind::Mill | Kind::Scribe | Kind::GardenCourt => {
            super::market_buildings::roman(&mut b, kind, level)
        }
        _ => unreachable!("building unavailable in Rome: {kind:?}"),
    }
    b.mesh()
}

fn arcade(b: &mut Batch, p: Vec3, spring: f32, depth: f32, color: Tint) {
    for x in [-0.68, 0.68] {
        arch(b, p + v(x, 0.0, 0.0), 0.88, spring, 0.145, depth, color);
    }
    let y = spring + 0.58;
    b.slab(
        p + v(0.0, y + 0.054, 0.0),
        v(2.67, 0.13, depth + 0.11),
        TRAVERTINE,
    );
    b.slab(
        p + v(0.0, y + 0.144, 0.0),
        v(2.70, 0.078, depth + 0.16),
        TILE,
    );
}

fn foundation(b: &mut Batch, depth: f32) {
    b.slab(v(0.0, 0.045, 0.0), v(2.66, 0.09, depth), BASALT);
    b.slab(v(0.0, 0.12, 0.0), v(2.52, 0.067, depth - 0.1), TRAVERTINE);
}

pub(super) fn masonry(b: &mut Batch, p: Vec3, width: f32, height: f32, color: Tint) {
    for n in 0..(height / 0.118) as usize {
        let y = p.y + 0.06 + n as f32 * 0.118;
        let step = if n % 2 == 0 { 0.0 } else { 0.106 };
        let count = (width / 0.212) as usize;
        for i in 0..count {
            let x = p.x - width / 2.0 + 0.106 + i as f32 * 0.212 + step;
            if x + 0.09 > p.x + width / 2.0 {
                continue;
            }
            b.slab(
                v(x, y, p.z),
                v(0.197, 0.088, 0.034),
                shift(color, ((n * 19 + i * 7) % 27) as i16 - 11),
            );
        }
    }
}

pub(super) fn arch(
    b: &mut Batch,
    p: Vec3,
    width: f32,
    leg: f32,
    thick: f32,
    depth: f32,
    color: Tint,
) {
    let r = width / 2.0;
    for side in [-1.0, 1.0] {
        b.slab(
            p + v(side * (r + thick / 2.0), leg / 2.0, 0.0),
            v(thick, leg, depth),
            color,
        );
        b.slab(
            p + v(side * (r + thick / 2.0), leg - 0.055, 0.0),
            v(thick + 0.055, 0.10, depth + 0.06),
            TRAVERTINE,
        );
    }
    let point = |a: f32, radius, z| p + v(a.cos() * radius, leg + a.sin() * radius, z);
    for i in 0..12 {
        let a = i as f32 * std::f32::consts::PI / 12.0;
        let c = (i + 1) as f32 * std::f32::consts::PI / 12.0;
        let tint = shift(
            if i % 4 == 1 { TRAVERTINE } else { color },
            (i * 11 % 17) as i16 - 7,
        );
        let front = [
            point(a, r, depth / 2.0),
            point(a, r + thick, depth / 2.0),
            point(c, r + thick, depth / 2.0),
            point(c, r, depth / 2.0),
        ];
        let back = front.map(|p| p - Vec3::Z * depth);
        b.face(&front, tint);
        b.face(&[back[3], back[2], back[1], back[0]], tint);
        b.face(&[front[3], back[3], back[0], front[0]], shift(tint, -18));
        b.face(&[front[1], back[1], back[2], front[2]], tint);
    }
}

fn window(b: &mut Batch, p: Vec3, width: f32, height: f32) {
    b.slab(p, v(width, height, 0.043), [65, 59, 55]);
    b.slab(
        p + v(0.0, -height * 0.50, 0.025),
        v(width + 0.12, 0.075, 0.107),
        TRAVERTINE,
    );
    for x in [-width * 0.32, 0.0, width * 0.32] {
        b.slab(p + v(x, 0.0, 0.027), v(0.024, height - 0.04, 0.024), OAK);
    }
}

fn insula(b: &mut Batch, villa: bool, level: u8) {
    foundation(b, if level >= 4 { 2.27 } else { 2.06 });
    let base = if villa { 1.44 } else { 2.04 };
    let upper = f32::from(level.saturating_sub(4)) * 0.25 + f32::from(villa && level >= 4) * 0.34;
    let h = base + upper;
    let color = if villa { PLASTER } else { [194, 111, 79] };
    b.slab(v(0.0, 0.14 + h / 2.0, -0.40), v(2.32, h, 1.22), color);
    masonry(b, v(0.0, 0.15, 0.233), 2.27, h, shift(color, 8));
    let door = if villa { 0.72 } else { 0.53 };
    b.slab(v(0.0, 0.67, 0.232), v(door, 1.05, 0.06), OAK);
    arch(b, v(0.0, 0.17, 0.255), door, 0.70, 0.093, 0.065, TRAVERTINE);
    for x in [-0.72, 0.72] {
        window(b, v(x, 0.91, 0.245), 0.31, 0.47);
        if !villa {
            window(b, v(x, 1.66, 0.245), 0.37, 0.48);
        }
    }
    if !villa {
        window(b, v(0.0, 1.66, 0.245), 0.40, 0.43);
    }
    if upper >= 0.30 {
        for x in [-0.71, 0.0, 0.71] {
            window(b, v(x, h - 0.12, 0.245), 0.30, 0.24);
        }
        b.slab(v(0.0, h - 0.28, 0.26), v(2.40, 0.073, 0.10), TRAVERTINE);
    }
    b.slab(v(0.0, h + 0.105, -0.36), v(2.42, 0.14, 1.38), TRAVERTINE);
    roof(b, 2.56, 1.89, h + 0.166, 0.38, TILE);
    if villa {
        let front = if level >= 3 { 0.86 } else { 0.68 };
        for x in [-1.02, 1.02] {
            b.frustum(v(x, 0.14, front), 0.102, 0.084, 1.24, 10, TRAVERTINE);
        }
        b.slab(
            v(0.0, 1.43, (front + 0.26) / 2.0),
            v(2.22, 0.14, front - 0.11),
            TRAVERTINE,
        );
        panel_roof(
            b,
            v(0.0, 1.55, (front + 0.25) / 2.0),
            2.38,
            front + 0.17,
            0.17,
            if level >= 5 { TEAL } else { TILE },
        );
        mosaic(
            b,
            v(0.0, 0.16, 0.56),
            if level >= 2 { 14 } else { 9 },
            5,
            0.15,
        );
        pool(
            b,
            v(0.0, 0.184, front - 0.11),
            0.78 + f32::from(level - 1) * 0.16,
            0.38,
        );
        if level >= 2 {
            for x in [-0.95, 0.95] {
                flowerpot(b, v(x, 0.15, 1.01), 0.64, [223, 122, 132]);
            }
        }
        if level >= 4 {
            stoop(b, v(0.0, 0.0, 1.01), 1.13, 0.15, 0.25, 2, TRAVERTINE);
        }
        if level >= 6 {
            for x in [-1.18, 1.18] {
                b.frustum(v(x, 0.15, 0.68), 0.18, 0.15, 0.34, 10, BASALT);
                cypress(b, v(x, 0.47, 0.68), 0.48);
            }
        }
    } else {
        let balcony = if level >= 2 { 2.49 } else { 1.79 };
        let extension = if level >= 2 { 0.66 } else { 0.52 };
        b.slab(
            v(0.0, 1.39, (extension + 0.24) * 0.5),
            v(balcony + 0.11, 0.115, extension - 0.21),
            TRAVERTINE,
        );
        rail(b, v(0.0, 1.45, extension), balcony, WINE);
        if level >= 2 {
            for x in [-0.92, 0.92] {
                flowerpot(b, v(x, 1.45, extension - 0.075), 0.49, [220, 112, 138]);
            }
        }
        if level >= 3 {
            for x in [-0.94, 0.94] {
                b.beam(v(x, 0.72, 0.27), v(x, 1.34, extension - 0.014), 0.081, OAK);
            }
            b.slab(v(0.0, 0.11, 0.85), v(2.45, 0.17, 0.50), TRAVERTINE);
        }
        if level >= 4 {
            for x in [-0.64, 0.64] {
                arch(
                    b,
                    v(x, 0.15, extension - 0.035),
                    0.75,
                    0.73,
                    0.135,
                    0.14,
                    BRICK,
                );
            }
        }
        if level >= 5 {
            panel_roof(
                b,
                v(0.0, 2.01, 0.53),
                2.45,
                0.51,
                0.20,
                if level >= 6 { TILE } else { TEAL },
            );
            for x in [-1.10, 1.10] {
                b.beam(v(x, 1.45, extension), v(x, 1.98, extension), 0.055, OAK);
            }
        }
    }
    flowerpot(b, v(-1.09, 0.13, 0.86), 0.69, [208, 108, 125]);
}

fn shop(b: &mut Batch, kind: Kind, level: u8) {
    let inn = kind == Kind::Inn;
    let wall = if inn { TRAVERTINE } else { [183, 113, 82] };
    let h = if inn { 2.05 } else { 1.40 } + f32::from(level.saturating_sub(4)) * 0.23;
    foundation(b, if level >= 4 { 2.27 } else { 1.95 });
    b.slab(v(0.0, 0.15 + h / 2.0, -0.33), v(2.31, h, 1.28), wall);
    masonry(b, v(0.0, 0.14, 0.327), 2.27, h, shift(wall, 3));
    b.slab(v(0.0, h + 0.119, -0.33), v(2.41, 0.12, 1.42), TRAVERTINE);
    if inn {
        gable(b, v(0.0, h + 0.177, -0.39), 2.56, 1.73, 0.37, TILE);
    } else {
        let kiln = v(0.65, 0.16, -0.40);
        b.slab(
            v(0.0, h + 0.184, -0.34),
            v(2.49, 0.068, 1.54),
            [88, 125, 130],
        );
        b.frustum(
            kiln + Vec3::Y * h,
            0.31,
            0.18,
            0.41 + f32::from(level) * 0.038,
            12,
            BRICK,
        );
        b.frustum(
            kiln + Vec3::Y * (h + 0.41 + f32::from(level) * 0.038),
            0.23,
            0.20,
            0.073,
            12,
            TRAVERTINE,
        );
    }
    for x in [-0.62, 0.62] {
        b.slab(v(x, 0.61, 0.343), v(0.55, 0.86, 0.053), [52, 49, 44]);
        arch(b, v(x, 0.15, 0.39), 0.57, 0.66, 0.105, 0.12, BRICK);
        if inn {
            window(b, v(x, 1.78 + (h - 2.05) * 0.64, 0.352), 0.36, 0.42);
        }
    }
    if level < 4 {
        awning(
            b,
            0.51,
            if inn { 1.30 } else { 1.25 },
            2.19,
            if inn { WINE } else { TEAL },
        );
    } else if inn {
        for x in [-1.08, 1.08] {
            super::buildings::column(b, v(x, 0.14, 0.95), 1.18, 0.077);
        }
        b.slab(v(0.0, 1.37, 0.60), v(2.50, 0.12, 0.79), TRAVERTINE);
        rail(b, v(0.0, 1.43, 0.98), 2.35, WINE);
    } else {
        for x in [-1.06, 1.06] {
            b.beam(v(x, 0.15, 1.05), v(x, 1.30, 1.05), 0.082, OAK);
        }
        panel_roof(b, v(0.0, 1.37, 0.81), 2.49, 0.70, 0.19, TEAL);
    }
    if inn {
        if (2..4).contains(&level) {
            b.slab(v(0.0, 1.46, 0.49), v(1.86, 0.085, 0.34), TRAVERTINE);
            rail(b, v(0.0, 1.50, 0.65), 1.76, WINE);
        }
        if level >= 3 {
            mosaic(b, v(0.0, 0.16, 0.90), 14, 3, 0.143);
            flowerpot(b, v(-1.07, 0.16, 0.82), 0.73, [218, 103, 129]);
        }
        if level >= 5 {
            window(b, v(0.0, h - 0.15, 0.352), 0.29, 0.28);
        }
        if level >= 6 {
            b.slab(v(0.0, h + 0.43, 0.482), v(0.72, 0.18, 0.08), TRAVERTINE);
            b.gem(v(0.0, h + 0.43, 0.536), 0.15, 0.085, 0.06, 0.45, GOLD);
        }
    } else {
        if level >= 2 {
            oven(b, v(0.83, 0.15, 0.69), BRICK);
        } else {
            amphora(b, v(0.81, 0.15, 0.73), 1.08, BRICK);
        }
        if level >= 3 {
            shelf(b, v(-0.72, 0.15, 0.86), TEAL);
        } else {
            b.slab(v(-0.63, 0.50, 0.72), v(0.69, 0.114, 0.37), OAK);
            for x in [-0.85, -0.60, -0.33] {
                amphora(b, v(x, 0.556, 0.77), 0.55, TEAL);
            }
        }
        if level >= 5 {
            for x in [-0.77, -0.38, 0.01] {
                window(b, v(x, h - 0.08, 0.352), 0.22, 0.23);
            }
        }
        if level >= 6 {
            b.slab(v(0.0, 1.46, 1.10), v(0.53, 0.19, 0.08), GOLD);
            b.gem(v(0.0, 1.44, 1.15), 0.095, 0.082, 0.028, 0.1, BLUE);
        }
    }
}

pub(super) fn mosaic(b: &mut Batch, p: Vec3, cols: usize, rows: usize, step: f32) {
    let chalk = [225, 204, 166];
    let ink = [90, 100, 113];
    for i in 0..cols {
        for j in 0..rows {
            let color = if (i + j) % 5 == 0 {
                WINE
            } else if j == 0 || j + 1 == rows {
                ink
            } else {
                shift(chalk, ((i * 7 + j) % 13) as i16 - 6)
            };
            b.slab(
                p + v(
                    (i as f32 - (cols - 1) as f32 / 2.0) * step,
                    0.0,
                    (j as f32 - (rows - 1) as f32 / 2.0) * step,
                ),
                v(step * 0.96, 0.025, step * 0.96),
                color,
            );
        }
    }
}

fn pool(b: &mut Batch, p: Vec3, w: f32, d: f32) {
    b.slab(p + Vec3::Y * 0.052, v(w, 0.08, d), POOL);
    for side in [-1.0, 1.0] {
        b.slab(
            p + v(side * w / 2.0, 0.12, 0.0),
            v(0.12, 0.13, d + 0.14),
            TRAVERTINE,
        );
        b.slab(
            p + v(0.0, 0.12, side * d / 2.0),
            v(w, 0.13, 0.11),
            TRAVERTINE,
        );
    }
    for i in 0..3 {
        b.slab(
            p + v(
                (i as f32 - 1.0) * w * 0.22,
                0.095,
                (i as f32 - 1.0) * d * 0.16,
            ),
            v(w * 0.19, 0.006, 0.023),
            [150, 201, 195],
        );
    }
}

fn cistern(b: &mut Batch, level: u8) {
    foundation(b, if level >= 3 { 2.14 } else { 1.65 });
    mosaic(b, v(0.0, 0.164, 0.04), 14, 10, 0.14);
    let width = if level >= 3 { 2.14 } else { 1.31 };
    let z = if level >= 3 { 0.60 } else { 0.28 };
    tank(
        b,
        v(0.0, 0.17, z),
        width,
        0.72,
        if level >= 5 { 0.32 } else { 0.24 },
        TRAVERTINE,
        POOL,
    );
    let spring = if level >= 6 { 1.02 } else { 0.70 };
    if level >= 2 {
        arcade(
            b,
            v(0.0, 0.14, -0.46),
            spring,
            if level >= 4 { 0.49 } else { 0.30 },
            BRICK,
        );
    } else {
        arch(b, v(0.0, 0.14, -0.41), 0.78, 0.59, 0.17, 0.27, BRICK);
        b.slab(v(0.0, 1.284, -0.41), v(1.25, 0.15, 0.39), TRAVERTINE);
    }
    if level >= 3 {
        for x in [-0.57, 0.57] {
            b.frustum(v(x, 0.17, 0.10), 0.13, 0.11, 0.56, 10, TRAVERTINE);
            b.beam(v(x, 0.71, 0.10), v(x, 0.68, 0.43), 0.083, GOLD);
            b.beam(v(x, 0.65, 0.43), v(x, 0.38, 0.47), 0.044, POOL);
        }
    }
    if level >= 4 {
        for x in [-0.71, 0.71] {
            b.slab(
                v(x, 0.75, -0.65),
                v(0.73, 1.20 + f32::from(level >= 6) * 0.28, 0.12),
                PLASTER,
            );
            window(b, v(x, 0.85, -0.581), 0.28, 0.33);
        }
    }
    if level >= 6 {
        panel_roof(b, v(0.0, 1.94, -0.44), 2.73, 0.76, 0.18, TEAL);
    }
    amphora(b, v(1.21, 0.15, 0.15), 0.70, BRICK);
}

fn pipes(b: &mut Batch, level: u8) {
    foundation(b, if level >= 3 { 2.12 } else { 1.65 });
    for x in [-0.75, 0.75] {
        arch(b, v(x, 0.14, -0.35), 0.94, 1.28, 0.20, 0.62, BRICK);
    }
    b.slab(v(0.0, 2.00, -0.35), v(2.76, 0.20, 0.77), TRAVERTINE);
    b.slab(v(0.0, 2.14, -0.35), v(2.70, 0.104, 0.71), BRICK);
    let shoulder = if level > 1 { 0.74 } else { 0.0 };
    if level >= 2 {
        for x in [-0.84, 0.0, 0.84] {
            arch(b, v(x, 2.19, -0.35), 0.59, 0.29, 0.108, 0.41, BRICK);
        }
    }
    let y = 2.197 + shoulder;
    tank(
        b,
        v(0.0, y - 0.11, -0.35),
        2.74,
        0.61,
        0.23,
        TRAVERTINE,
        POOL,
    );
    if level >= 3 {
        tank(b, v(0.0, 0.15, 0.66), 2.26, 0.74, 0.245, TRAVERTINE, POOL);
        for x in [-0.75, 0.75] {
            b.beam(v(x, 1.05, -0.04), v(x, 0.99, 0.59), 0.108, TRAVERTINE);
            b.beam(v(x, 0.96, 0.60), v(x, 0.35, 0.62), 0.048, POOL);
        }
    } else {
        tank(b, v(0.0, 0.15, 0.53), 1.29, 0.55, 0.18, TRAVERTINE, POOL);
    }
    if level >= 4 {
        for x in [-1.18, 1.18] {
            b.slab(v(x, y - 0.02, -0.35), v(0.35, 0.22, 0.70), BRICK);
            b.slab(v(x, y + 0.13, -0.35), v(0.42, 0.10, 0.74), TILE);
        }
    }
    if level >= 5 {
        mosaic(b, v(0.0, 0.16, 1.09), 15, 2, 0.145);
        for x in [-1.189, 1.189] {
            b.frustum(v(x, 0.15, 0.53), 0.139, 0.109, 0.74, 12, TRAVERTINE);
            b.frustum(v(x, 0.89, 0.53), 0.14, 0.092, 0.11, 12, GOLD);
        }
    }
    if level >= 6 {
        tank(
            b,
            v(0.0, y + 0.14, -0.36),
            1.48,
            0.75,
            0.30,
            TRAVERTINE,
            POOL,
        );
        b.frustum(v(0.0, y + 0.39, -0.36), 0.18, 0.13, 0.16, 10, TRAVERTINE);
        b.gem(v(0.0, y + 0.57, -0.36), 0.095, 0.10, 0.08, 0.1, GOLD);
    }
}

fn baths(b: &mut Batch, level: u8) {
    foundation(b, 2.23);
    let h = if level >= 6 { 1.69 } else { 1.34 };
    b.slab(
        v(0.0, (h + 0.14) / 2.0, -0.59),
        v(2.42, h - 0.14, 0.84),
        BRICK,
    );
    masonry(b, v(0.0, 0.15, -0.159), 2.39, h - 0.16, BRICK);
    for x in [-0.68, 0.68] {
        arch(b, v(x, 0.14, -0.10), 0.56, 0.69, 0.127, 0.16, TRAVERTINE);
        window(b, v(x, 0.67, -0.123), 0.39, 0.57);
    }
    b.slab(v(0.0, h + 0.045, -0.59), v(2.54, 0.10, 0.90), TRAVERTINE);
    let z = -0.54;
    dome(b, v(0.0, h + 0.10, z), if level >= 6 { 0.65 } else { 0.58 });
    if level >= 4 {
        for x in [-0.84, 0.84] {
            dome(b, v(x, h + 0.096, z - 0.07), 0.32);
        }
    }
    mosaic(b, v(0.0, 0.164, 0.57), 14, 7, 0.15);
    pool(
        b,
        v(0.0, 0.187, 0.53),
        if level >= 2 { 2.10 } else { 1.62 },
        if level >= 2 { 0.79 } else { 0.63 },
    );
    if level >= 3 {
        arcade(
            b,
            v(0.0, 0.14, 0.95),
            if level >= 6 { 0.81 } else { 0.70 },
            0.31,
            TRAVERTINE,
        );
    }
    if level >= 5 {
        for x in [-1.08, 1.08] {
            flowerpot(b, v(x, 0.16, 1.04), 0.74, [205, 134, 156]);
        }
        stoop(b, v(0.0, 0.0, 1.06), 0.94, 0.17, 0.20, 2, TRAVERTINE);
    }
}

fn dome(b: &mut Batch, at: Vec3, r: f32) {
    b.frustum(at, r, r * 0.89, r * 0.37, 14, TILE);
    b.frustum(
        at + Vec3::Y * (r * 0.37),
        r * 0.89,
        r * 0.52,
        r * 0.39,
        14,
        shift(TILE, 12),
    );
    b.frustum(
        at + Vec3::Y * (r * 0.76),
        r * 0.52,
        r * 0.16,
        r * 0.25,
        14,
        shift(TILE, 23),
    );
    b.frustum(
        at + Vec3::Y * (r * 1.01),
        r * 0.15,
        r * 0.13,
        r * 0.18,
        10,
        GOLD,
    );
}

fn portico(b: &mut Batch, level: u8) {
    foundation(b, if level >= 3 { 2.14 } else { 1.75 });
    let front = if level >= 3 { 0.73 } else { 0.40 };
    let spring = if level >= 5 { 1.14 } else { 0.94 };
    arcade(b, v(0.0, 0.14, front), spring, 0.34, BRICK);
    arcade(b, v(0.0, 0.14, -0.59), spring, 0.27, BRICK);
    let roof_y = spring + 0.93;
    if level >= 4 {
        gable(
            b,
            v(0.0, roof_y, (front - 0.59) / 2.0),
            2.73,
            front + 0.99,
            if level >= 6 { 0.46 } else { 0.34 },
            TILE,
        );
    } else {
        b.slab(
            v(0.0, spring + 0.86, (front - 0.59) / 2.0),
            v(2.72, 0.126, front + 0.99),
            TILE,
        );
    }
    bench(b, v(0.0, 0.15, -0.49), 1.74);
    if level >= 2 {
        mosaic(b, v(0.0, 0.16, 0.45), 14, 7, 0.145);
    }
    if level >= 5 {
        flowerpot(b, v(-1.21, 0.17, 0.97), 0.57, [213, 131, 152]);
    }
    if level >= 6 {
        b.gem(
            v(0.0, roof_y + 0.21, front + 0.224),
            0.17,
            0.111,
            0.042,
            0.55,
            GOLD,
        );
        b.slab(
            v(0.0, roof_y - 0.081, front + 0.19),
            v(2.62, 0.037, 0.034),
            GOLD,
        );
    }
}

fn forum(b: &mut Batch, level: u8) {
    foundation(b, 2.22);
    mosaic(b, v(0.0, 0.165, 0.25), 15, 12, 0.15);
    for (i, x) in [-0.80, 0.0, 0.80].into_iter().enumerate() {
        arch(b, v(x, 0.14, -0.44), 0.57, 1.25, 0.144, 0.29, BRICK);
        b.slab(v(x, 0.51, -0.08), v(0.59, 0.13, 0.41), OAK);
        amphora(b, v(x, 0.57, 0.03), 0.68, [WINE, TEAL, GOLD][i]);
        if level >= 2 {
            crate_box(b, v(x, 0.16, 0.72), v(0.42, 0.34, 0.39), PLASTER);
            b.gem(
                v(x, 0.57, 0.72),
                0.12,
                0.094,
                0.12,
                x,
                [WINE, TEAL, GOLD][i],
            );
        }
    }
    b.slab(v(0.0, 1.88, -0.44), v(2.70, 0.18, 0.61), TRAVERTINE);
    if level < 3 {
        b.slab(v(0.0, 2.017, -0.44), v(2.75, 0.094, 0.73), TILE);
    } else {
        let top = if level >= 5 { 0.39 } else { 0.24 };
        arcade(b, v(0.0, 1.97, -0.44), top, 0.41, BRICK);
        if level >= 4 {
            let y = 2.728 + top;
            gable(
                b,
                v(0.0, y, -0.44),
                2.76,
                0.76,
                if level >= 6 { 0.44 } else { 0.31 },
                TILE,
            );
            b.gem(v(0.0, y + 0.15, -0.043), 0.15, 0.094, 0.035, 0.38, GOLD);
        }
    }
    if level >= 5 {
        for x in [-1.16, 1.16] {
            b.beam(v(x, 0.16, 0.93), v(x, 1.42, 0.93), 0.075, OAK);
        }
        panel_roof(b, v(0.0, 1.47, 0.43), 2.50, 1.29, 0.14, WINE);
    } else {
        awning(b, 0.30, 1.48, 2.40, WINE);
    }
    if level >= 6 {
        stoop(b, v(0.0, 0.0, 1.04), 1.28, 0.16, 0.26, 2, TRAVERTINE);
    }
}

fn garden(b: &mut Batch, level: u8) {
    foundation(b, 2.14);
    pool(b, v(0.0, 0.145, 0.61), 0.85 + f32::from(level) * 0.15, 0.35);
    for x in [-0.79, 0.79] {
        olive(b, v(x, 0.15, -0.42), 0.85 + f32::from(level - 1) * 0.028);
        flowerpot(b, v(x * 1.47, 0.16, 0.73), 0.71, [213, 111, 144]);
    }
    mosaic(
        b,
        v(0.0, 0.17, -0.26),
        if level >= 2 { 8 } else { 5 },
        6,
        0.15,
    );
    if level >= 3 {
        arcade(
            b,
            v(0.0, 0.14, 0.06),
            if level >= 5 { 0.95 } else { 0.73 },
            0.36,
            TRAVERTINE,
        );
        bench(b, v(0.0, 0.16, 0.03), 1.02);
    }
    if level >= 4 {
        let y = if level >= 5 { 1.90 } else { 1.68 };
        for i in 0..9 {
            let x = -1.04 + i as f32 * 0.26;
            b.gem(
                v(x, y, 0.19),
                0.23,
                0.133,
                0.16,
                x,
                shift([94, 128, 79], (i * 7 % 17) as i16),
            );
        }
    }
    if level >= 6 {
        for x in [-1.03, 1.03] {
            b.frustum(v(x, 0.15, 0.93), 0.19, 0.17, 0.32, 10, TRAVERTINE);
            flowerpot(b, v(x, 0.46, 0.93), 0.76, [221, 135, 170]);
        }
    }
}

fn horreum(b: &mut Batch, level: u8) {
    let height = 1.42 + f32::from(level.saturating_sub(4)) * 0.27;
    foundation(b, if level >= 2 { 2.23 } else { 1.91 });
    b.slab(
        v(0.0, 0.14 + height * 0.5, -0.37),
        v(2.42, height, 1.31),
        BRICK,
    );
    masonry(b, v(0.0, 0.15, 0.30), 2.35, height - 0.08, BRICK);
    b.slab(v(0.0, 0.73, 0.34), v(0.91, 1.17, 0.08), [78, 59, 46]);
    arch(b, v(0.0, 0.14, 0.38), 0.96, 0.78, 0.14, 0.18, TRAVERTINE);
    gable(b, v(0.0, 0.16 + height, -0.34), 2.64, 1.73, 0.39, TILE);
    if level >= 2 {
        for x in [-0.86, 0.89] {
            crate_box(b, v(x, 0.15, 0.82), v(0.51, 0.35, 0.42), [150, 102, 69]);
            if level >= 4 {
                crate_box(b, v(x, 0.50, 0.81), v(0.40, 0.25, 0.32), [158, 112, 73]);
            }
        }
    } else {
        amphora(b, v(0.91, 0.13, 0.83), 0.87, BRICK);
    }
    if level >= 3 {
        crane(b, v(1.16, 0.15, 0.28), 1.82 + f32::from(level - 3) * 0.14);
    }
    if level >= 4 {
        b.beam(v(-1.12, 0.16, 1.06), v(-1.12, 1.31, 1.06), 0.071, OAK);
        panel_roof(b, v(-0.45, 1.38, 0.78), 1.58, 0.68, 0.18, TILE);
    }
    if level >= 5 {
        for x in [-0.72, 0.72] {
            window(b, v(x, height - 0.02, 0.338), 0.34, 0.22);
        }
    }
    if level >= 6 {
        b.slab(v(0.0, 1.48, 0.33), v(2.44, 0.10, 0.10), TRAVERTINE);
        stoop(b, v(0.0, 0.0, 0.94), 1.16, 0.16, 0.30, 2, TRAVERTINE);
    }
}

pub fn skyline(b: &mut Batch, wide: f32, chapter: usize, grain: &mut Grain) {
    let z = -4.91 - (chapter % 3) as f32 * 0.11;
    if chapter % 7 == 3 || chapter % 7 == 4 {
        for i in 0..8 {
            let x = (i as f32 - 3.5) * 2.25;
            let h = grain.span(1.06, 1.87);
            b.slab(
                v(x, h * 0.5, z),
                v(2.04, h, 1.12),
                shift(PLASTER, i * 3 - 8),
            );
            arch(b, v(x, 0.0, z + 0.57), 0.69, 0.60, 0.14, 0.13, BRICK);
            b.slab(v(x, h + 0.04, z), v(2.15, 0.19, 1.26), [152, 77, 55]);
            if i % 2 == 0 {
                cypress(b, v(x + 1.0, 0.04, z + 0.71), 0.9);
            }
        }
        return;
    }
    let n = ((wide * 1.82) / 1.84) as usize;
    let start = -(n as f32 - 1.0) * 0.92;
    for i in 0..n {
        let x = start + 1.84 * i as f32;
        let h = 1.65 + (i as f32 * 0.39 + chapter as f32).cos() * 0.16;
        let rise = h + 0.85;
        arch(
            b,
            v(x, 0.075, z),
            1.37,
            h,
            0.19,
            0.66,
            shift(BRICK, (i * 9 % 25) as i16 - 11),
        );
        b.slab(v(x, rise, z), v(1.86, 0.19, 0.75), TRAVERTINE);
        b.slab(v(x, rise + 0.17, z), v(1.86, 0.17, 0.68), BRICK);
        b.slab(v(x, rise + 0.29, z), v(1.86, 0.09, 0.73), TRAVERTINE);
        if i % 3 == chapter % 3 {
            let scale = grain.span(1.0, 1.23);
            cypress(b, v(x - 0.56, 0.0, z + 0.66), scale);
        }
    }
    for n in 0..5 {
        let x = (n as f32 - 2.0) * wide * 0.38;
        let level = grain.span(0.75, 1.72);
        b.slab(
            v(x, 0.78, -6.03),
            v(1.56, level, 1.21),
            shift(PLASTER, n as i16 * 3),
        );
        b.block(
            v(x, 0.77 + level * 0.5 + 0.14, -6.03),
            v(1.77, 0.23, 1.35),
            Quat::from_rotation_y(0.07),
            TILE,
        );
    }
}

pub fn quay_landmark(b: &mut Batch, x: f32, chapter: usize) {
    let z = -2.06 + (chapter % 3) as f32 * 0.16;
    b.slab(v(x, 0.055, z), v(2.24, 0.10, 2.12), TRAVERTINE);
    mosaic(b, v(x, 0.115, z), 12, 13, 0.15);
    if chapter.is_multiple_of(2) {
        pool(b, v(x, 0.123, z - 0.21), 1.04, 0.81);
        b.frustum(v(x, 0.27, z - 0.22), 0.15, 0.12, 0.73, 9, MARBLE);
        b.frustum(v(x, 1.0, z - 0.22), 0.31, 0.35, 0.11, 11, TRAVERTINE);
        b.frustum(v(x, 1.1, z - 0.22), 0.28, 0.29, 0.045, 11, POOL);
    } else {
        arch(b, v(x, 0.135, z), 0.97, 1.38, 0.25, 0.47, BRICK);
        b.slab(v(x, 2.28, z), v(1.54, 0.18, 0.55), TRAVERTINE);
        flowerpot(b, v(x - 0.84, 0.12, z + 0.47), 0.75, [194, 99, 119]);
    }
}

pub fn paving(n: usize, row: usize) -> Tint {
    [BASALT, [128, 130, 129], SANDSTONE, [146, 146, 140]][(n * 11 + row * 3) % 4]
}

pub fn far_bank(chapter: usize) -> Mesh {
    let mut b = Batch::default();
    let mut grain = Grain::new(1713 + chapter as u32 * 391);
    let z = -16.8;
    for i in -10i32..=10 {
        let x = i as f32 * 2.18;
        let bank = 0.50 + (x * 0.19).cos() * 0.11;
        let tint = shift([116, 152, 132], grain.span(-9.0, 8.0) as i16);
        b.slab(v(x, bank - 0.42, z - 3.0), v(2.19, 1.25, 6.25), tint);
        b.slab(
            v(x, bank * 0.49, z + 0.18),
            v(2.18, 0.70, 0.38),
            shift([151, 159, 136], (i * 3).rem_euclid(9) as i16),
        );
        b.slab(
            v(x, bank + 0.31, z + 0.17),
            v(2.19, 0.15, 0.48),
            [181, 184, 157],
        );
        let depth = grain.span(-3.0, -1.2);
        if i % 3 == 0 {
            b.gem(
                v(x, 1.05, z + depth),
                0.39,
                1.06,
                0.35,
                0.28,
                [86, 132, 113],
            );
            continue;
        }
        let h = grain.span(0.76, 1.58);
        let d = grain.span(0.58, 0.95);
        b.slab(
            v(x, bank + 0.18 + h / 2.0, z + depth),
            v(1.69, h, d),
            shift([190, 161, 135], i as i16),
        );
        let edge = z + depth + d * 0.5 + 0.01;
        for offset in [-0.36, 0.37] {
            b.slab(
                v(x + offset, bank + h * 0.71, edge),
                v(0.22, 0.30, 0.05),
                [112, 118, 104],
            );
        }
        let left = v(x - 0.92, bank + h + 0.24, edge);
        let right = v(x + 0.92, bank + h + 0.24, edge);
        let ridge = v(x, bank + h + 0.59, edge);
        b.face(&[left, right, ridge], [168, 117, 100]);
        b.face(
            &[
                left - Vec3::Z * (d + 0.16),
                left,
                ridge,
                ridge - Vec3::Z * (d + 0.16),
            ],
            [172, 124, 108],
        );
        b.face(
            &[
                right,
                right - Vec3::Z * (d + 0.16),
                ridge - Vec3::Z * (d + 0.16),
                ridge,
            ],
            [177, 129, 111],
        );
    }
    b.mesh()
}

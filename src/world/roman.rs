use bevy::prelude::*;

use super::{
    buildings::{
        BLUE, GOLD, MARBLE, OAK, TEAL, amphora, awning, bake_shelf, cypress, flowerpot, olive,
        oven, roof, shelf,
    },
    mesh::{Batch, Grain, Tint, shift, v},
    upgrades::{bench, canopy, crane, crate_box, rail},
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
        Kind::House => insula(&mut b, false),
        Kind::Villa => insula(&mut b, true),
        Kind::Well => cistern(&mut b),
        Kind::Grove => garden(&mut b),
        Kind::Stoa => portico(&mut b),
        Kind::Storehouse => horreum(&mut b),
        Kind::Aqueduct => pipes(&mut b),
        Kind::Baths => baths(&mut b),
        Kind::Forum => forum(&mut b),
        Kind::Bakery | Kind::Potter | Kind::Taverna | Kind::Inn => shop(&mut b, kind),
        Kind::Press | Kind::Shrine => unreachable!("Roman districts use separate amenities"),
    }
    for tier in 2..=level {
        append_upgrade(&mut b, kind, tier);
    }
    b.mesh()
}

pub fn append_upgrade(b: &mut Batch, kind: Kind, tier: u8) {
    match (kind, tier) {
        (Kind::House, 2) | (Kind::Inn, 2) => {
            b.slab(v(0.0, 1.41, 0.67), v(2.44, 0.13, 0.58), TRAVERTINE);
            rail(b, v(0.0, 1.46, 0.94), 2.26, WINE);
            for x in [-0.83, 0.83] {
                flowerpot(b, v(x, 1.48, 0.85), 0.59, [209, 108, 120]);
            }
        }
        (Kind::House, 3) => tower(b, v(-0.49, 2.18, -0.43), 1.04, 0.60, PLASTER),
        (Kind::House, 4) => {
            for x in [-0.70, 0.70] {
                arch(b, v(x, 0.13, 0.72), 0.75, 0.69, 0.14, 0.17, BRICK);
            }
            b.slab(v(0.0, 1.31, 0.72), v(2.69, 0.13, 0.42), TRAVERTINE);
            b.slab(v(0.0, 0.067, 1.07), v(2.43, 0.13, 0.31), BASALT);
        }
        (Kind::Villa, 2) => {
            mosaic(b, v(0.0, 0.184, 0.79), 16, 4, 0.15);
            for x in [-0.88, 0.88] {
                b.frustum(v(x, 0.19, 0.60), 0.23, 0.16, 0.13, 10, TRAVERTINE);
                cypress(b, v(x, 0.30, 0.75), 0.60);
            }
        }
        (Kind::Villa, 3) => {
            b.slab(v(0.0, 1.49, 0.65), v(2.40, 0.15, 0.88), TRAVERTINE);
            rail(b, v(0.0, 1.56, 1.06), 2.25, BRICK);
            canopy(b, v(0.0, 1.55, 0.62), 2.32, 0.73, 0.92, WINE, false);
        }
        (Kind::Villa, 4) => tower(b, v(-0.49, 1.84, -0.50), 1.24, 0.96, PLASTER),
        (Kind::Well, 2) => {
            for x in [-0.77, 0.77] {
                arch(b, v(x, 0.15, -0.38), 0.52, 0.62, 0.137, 0.24, TRAVERTINE);
            }
            b.slab(v(0.0, 1.43, -0.39), v(2.40, 0.14, 0.54), TRAVERTINE);
            b.slab(v(0.0, 1.54, -0.36), v(2.48, 0.10, 0.61), TILE);
        }
        (Kind::Well, 3) => {
            pool(b, v(0.0, 0.146, 0.82), 2.28, 0.52);
            for x in [-0.52, 0.52] {
                b.frustum(v(x, 0.17, 0.58), 0.094, 0.094, 0.58, 9, TRAVERTINE);
                b.beam(v(x, 0.70, 0.58), v(x, 0.70, 0.80), 0.082, GOLD);
                b.slab(v(x, 0.47, 0.81), v(0.038, 0.39, 0.037), POOL);
            }
        }
        (Kind::Well, 4) => tower(b, v(0.0, 1.56, -0.39), 1.56, 0.75, TRAVERTINE),
        (Kind::Grove, 2) => {
            olive(b, v(0.0, 0.153, -0.52), 1.16);
            mosaic(b, v(0.0, 0.18, 0.08), 14, 3, 0.16);
        }
        (Kind::Grove, 3) => {
            canal_gallery(b, v(0.0, 0.12, 0.74), 1.12, 0.53);
            bench(b, v(0.0, 0.15, 0.51), 1.36);
        }
        (Kind::Grove, 4) => {
            for i in 0..9 {
                let x = i as f32 * 0.26 - 1.04;
                b.gem(
                    v(x, 1.79 + (x * 4.0).cos() * 0.071, 0.86),
                    0.24,
                    0.16,
                    0.26,
                    x,
                    shift([94, 128, 79], (i * 7 % 17) as i16),
                );
                b.gem(v(x, 1.61, 0.96), 0.061, 0.065, 0.069, x, [146, 74, 119]);
            }
            for x in [-1.16, 1.16] {
                flowerpot(b, v(x, 0.14, 1.00), 0.98, [202, 131, 154]);
            }
        }
        (Kind::Bakery, 2) => oven(b, v(0.83, 0.115, 0.79), BRICK),
        (Kind::Bakery, 3) => bake_shelf(b, v(-0.66, 0.132, 0.91), WINE),
        (Kind::Bakery, 4) => {
            b.slab(v(0.60, 2.05, -0.49), v(0.58, 0.69, 0.61), BRICK);
            masonry(b, v(0.60, 1.75, -0.18), 0.57, 0.55, BRICK);
            b.slab(v(0.60, 2.43, -0.49), v(0.78, 0.14, 0.77), TRAVERTINE);
            b.frustum(v(0.60, 2.49, -0.49), 0.31, 0.11, 0.26, 9, TILE);
        }
        (Kind::Potter, 2) => oven(b, v(0.80, 0.117, 0.78), [185, 98, 64]),
        (Kind::Potter, 3) => shelf(b, v(-0.71, 0.12, 0.87), [89, 113, 124]),
        (Kind::Potter, 4) => {
            tower(b, v(-0.57, 1.82, -0.40), 1.11, 0.66, BRICK);
            for x in [-0.94, -0.52, -0.11] {
                amphora(b, v(x, 2.04, 0.16), 0.61, TRAVERTINE);
            }
        }
        (Kind::Taverna, 2) => {
            for x in [-0.73, 0.73] {
                bench(b, v(x, 0.073, 1.03), 0.81);
            }
            mosaic(b, v(0.0, 0.17, 0.84), 16, 4, 0.15);
        }
        (Kind::Taverna, 3) => {
            canopy(b, v(0.0, 0.12, 0.80), 2.52, 0.81, 1.54, WINE, false);
            b.slab(v(0.0, 0.59, 0.72), v(1.36, 0.12, 0.38), OAK);
        }
        (Kind::Taverna, 4) => tower(b, v(0.40, 1.82, -0.48), 1.34, 0.60, PLASTER),
        (Kind::Inn, 3) => {
            canopy(b, v(0.0, 1.47, 0.67), 2.30, 0.68, 0.87, WINE, true);
            b.slab(v(0.0, 2.39, 0.68), v(2.38, 0.091, 0.71), TRAVERTINE);
        }
        (Kind::Inn, 4) => tower(b, v(0.04, 2.30, -0.40), 1.42, 0.97, [224, 176, 129]),
        (Kind::Storehouse, 2) => {
            for x in [-0.87, 0.78] {
                crate_box(b, v(x, 0.15, 0.91), v(0.54, 0.41, 0.48), [155, 106, 71]);
                crate_box(b, v(x, 0.56, 0.87), v(0.38, 0.27, 0.37), [170, 121, 85]);
            }
            b.slab(v(0.0, 0.09, 1.01), v(2.50, 0.17, 0.52), BASALT);
        }
        (Kind::Storehouse, 3) => crane(b, v(0.87, 0.14, 0.10), 2.33),
        (Kind::Storehouse, 4) => tower(b, v(-0.46, 2.08, -0.41), 1.49, 0.65, BRICK),
        (Kind::Stoa, 2) => {
            for x in [-0.70, 0.70] {
                bench(b, v(x, 0.15, 0.72), 0.90);
            }
            mosaic(b, v(0.0, 0.16, 0.66), 15, 5, 0.15);
        }
        (Kind::Stoa, 3) => canal_gallery(b, v(0.0, 0.15, 0.7), 0.97, 0.43),
        (Kind::Stoa, 4) => {
            rail(b, v(0.0, 1.82, 0.30), 2.35, TRAVERTINE);
            canopy(b, v(0.0, 1.83, -0.16), 1.95, 1.04, 0.54, TILE, false);
        }
        (Kind::Aqueduct, 2) => {
            for x in [-0.83, 0.0, 0.83] {
                arch(b, v(x, 2.27, -0.09), 0.62, 0.30, 0.110, 0.36, BRICK);
            }
            b.slab(v(0.0, 2.99, -0.10), v(2.68, 0.13, 0.58), TRAVERTINE);
            b.slab(v(0.0, 3.09, -0.10), v(2.53, 0.084, 0.48), TILE);
        }
        (Kind::Aqueduct, 3) => {
            pool(b, v(0.0, 0.16, 0.83), 2.52, 0.55);
            for x in [-0.74, 0.74] {
                b.beam(v(x, 1.05, 0.09), v(x, 0.92, 0.68), 0.10, TRAVERTINE);
                b.slab(v(x, 0.58, 0.69), v(0.049, 0.66, 0.040), POOL);
            }
        }
        (Kind::Aqueduct, 4) => {
            for x in [-1.01, 1.01] {
                b.slab(v(x, 3.36, -0.12), v(0.61, 0.60, 0.57), BRICK);
                b.frustum(v(x, 3.68, -0.12), 0.46, 0.02, 0.24, 4, TILE);
            }
            b.slab(v(0.0, 3.22, -0.11), v(2.74, 0.12, 0.56), TRAVERTINE);
        }
        (Kind::Baths, 2) => {
            for x in [-0.70, 0.70] {
                b.frustum(v(x, 0.17, 0.70), 0.40, 0.43, 0.17, 12, TRAVERTINE);
                b.frustum(v(x, 0.34, 0.70), 0.34, 0.34, 0.027, 12, POOL);
            }
        }
        (Kind::Baths, 3) => {
            canal_gallery(b, v(0.0, 0.17, 0.76), 1.21, 0.45);
            b.slab(v(0.0, 0.077, 1.10), v(2.55, 0.15, 0.30), TRAVERTINE);
        }
        (Kind::Baths, 4) => {
            for x in [-0.92, 0.92] {
                b.frustum(v(x, 1.40, -0.53), 0.36, 0.35, 0.21, 12, TRAVERTINE);
                b.frustum(v(x, 1.60, -0.53), 0.37, 0.24, 0.26, 12, TILE);
                b.frustum(v(x, 1.85, -0.53), 0.24, 0.06, 0.16, 12, shift(TILE, 12));
                b.gem(v(x, 2.05, -0.53), 0.068, 0.102, 0.067, x, GOLD);
            }
            b.frustum(v(0.0, 1.95, -0.60), 0.19, 0.19, 0.19, 10, TRAVERTINE);
            b.frustum(v(0.0, 2.14, -0.60), 0.216, 0.032, 0.17, 10, TILE);
        }
        (Kind::Forum, 2) => {
            canopy(b, v(0.0, 0.15, 0.83), 2.62, 0.76, 1.38, WINE, false);
            for x in [-0.82, 0.82] {
                crate_box(b, v(x, 0.17, 0.83), v(0.40, 0.37, 0.34), PLASTER);
            }
        }
        (Kind::Forum, 3) => {
            for x in [-0.64, 0.64] {
                arch(b, v(x, 2.03, -0.4), 0.73, 0.37, 0.12, 0.44, BRICK);
            }
            b.slab(v(0.0, 2.83, -0.41), v(2.73, 0.14, 0.63), TRAVERTINE);
        }
        (Kind::Forum, 4) => {
            b.slab(v(0.0, 2.97, -0.38), v(2.71, 0.11, 0.73), TILE);
            b.face(
                &[
                    v(-1.32, 3.01, 0.02),
                    v(1.32, 3.01, 0.02),
                    v(0.0, 3.46, 0.02),
                ],
                TRAVERTINE,
            );
            b.gem(v(0.0, 3.17, 0.075), 0.17, 0.11, 0.037, 0.31, GOLD);
            for side in [-1.0, 1.0] {
                b.beam(
                    v(side * 1.34, 3.01, 0.034),
                    v(0.0, 3.47, 0.034),
                    0.086,
                    TILE,
                );
            }
        }
        _ => {}
    }
}

fn canal_gallery(b: &mut Batch, p: Vec3, height: f32, depth: f32) {
    for x in [-0.73, 0.73] {
        arch(
            b,
            p + v(x, 0.0, 0.0),
            0.87,
            height,
            0.137,
            depth,
            TRAVERTINE,
        );
    }
    let y = height + 0.43;
    b.slab(p + v(0.0, y, 0.0), v(2.69, 0.12, depth + 0.09), TRAVERTINE);
    b.slab(p + v(0.0, y + 0.10, 0.0), v(2.72, 0.11, depth + 0.15), TILE);
}

fn tower(b: &mut Batch, p: Vec3, width: f32, height: f32, wall: Tint) {
    b.slab(p + v(0.0, height * 0.38, 0.0), v(width, height, 0.84), wall);
    let roof_y = p.y + height * 0.88;
    b.slab(v(p.x, roof_y, p.z), v(width + 0.17, 0.17, 0.97), TRAVERTINE);
    for x in [-0.23, 0.23] {
        window(b, p + v(x, height * 0.46, 0.433), 0.24, height * 0.45);
    }
    for n in 0..10 {
        let x = p.x + (n as f32 / 9.0 - 0.5) * width;
        b.block(
            v(x, roof_y + 0.14, p.z),
            v(width / 9.4, 0.085, 1.04),
            Quat::from_rotation_x(0.13),
            shift(TILE, (n * 7 % 24) as i16 - 11),
        );
    }
}

fn foundation(b: &mut Batch, depth: f32) {
    b.slab(v(0.0, 0.045, 0.0), v(2.66, 0.09, depth), BASALT);
    b.slab(v(0.0, 0.12, 0.0), v(2.52, 0.067, depth - 0.1), TRAVERTINE);
}

fn masonry(b: &mut Batch, p: Vec3, width: f32, height: f32, color: Tint) {
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

fn arch(b: &mut Batch, p: Vec3, width: f32, leg: f32, thick: f32, depth: f32, color: Tint) {
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

fn insula(b: &mut Batch, villa: bool) {
    foundation(b, 2.06);
    let h = if villa { 1.54 } else { 2.03 };
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
    b.slab(v(0.0, h + 0.105, -0.36), v(2.42, 0.14, 1.38), TRAVERTINE);
    roof(b, 2.56, 1.89, h + 0.166, 0.38, TILE);
    if villa {
        for x in [-0.97, 0.97] {
            b.frustum(v(x, 0.135, 0.76), 0.102, 0.084, 1.22, 10, TRAVERTINE);
        }
        b.slab(v(0.0, 1.39, 0.66), v(2.20, 0.14, 0.73), TRAVERTINE);
        b.slab(v(0.0, 1.48, 0.67), v(2.30, 0.09, 0.77), TILE);
        mosaic(b, v(0.0, 0.157, 0.59), 9, 4, 0.16);
        pool(b, v(0.0, 0.188, 0.67), 0.83, 0.40);
    } else {
        b.slab(v(0.0, 1.37, 0.35), v(1.92, 0.08, 0.37), OAK);
        for n in 0..10 {
            b.slab(
                v((n as f32 - 4.5) * 0.18, 1.54, 0.53),
                v(0.032, 0.29, 0.032),
                WINE,
            );
        }
        b.slab(v(0.0, 1.69, 0.53), v(1.79, 0.062, 0.049), OAK);
    }
    flowerpot(b, v(-1.07, 0.13, 0.85), 0.75, [208, 108, 125]);
}

fn shop(b: &mut Batch, kind: Kind) {
    let (front, fabric, h) = match kind {
        Kind::Bakery => (PLASTER, WINE, 1.49),
        Kind::Potter => ([183, 113, 82], [91, 115, 122], 1.45),
        Kind::Taverna => ([213, 164, 107], [142, 77, 101], 1.55),
        _ => (TRAVERTINE, [173, 75, 57], 2.10),
    };
    foundation(b, 1.95);
    b.slab(v(0.0, 0.15 + h / 2.0, -0.33), v(2.31, h, 1.28), front);
    masonry(b, v(0.0, 0.14, 0.327), 2.27, h, shift(front, 3));
    b.slab(v(0.0, h + 0.119, -0.33), v(2.41, 0.12, 1.42), TRAVERTINE);
    roof(b, 2.53, 1.89, h + 0.19, 0.30, TILE);
    for x in [-0.59, 0.59] {
        b.slab(v(x, 0.58, 0.352), v(0.57, 0.83, 0.045), [52, 49, 44]);
        arch(b, v(x, 0.15, 0.39), 0.57, 0.66, 0.105, 0.12, BRICK);
        if kind == Kind::Inn {
            window(b, v(x, 1.78, 0.352), 0.37, 0.45);
        }
    }
    awning(
        b,
        0.51,
        if kind == Kind::Inn { 1.43 } else { 1.30 },
        2.19,
        fabric,
    );
    b.slab(v(-0.60, 0.49, 0.69), v(0.63, 0.13, 0.44), OAK);
    for i in 0..3 {
        let x = -0.79 + i as f32 * 0.20;
        if kind == Kind::Bakery {
            b.gem(v(x, 0.58, 0.73), 0.10, 0.07, 0.087, 0.28, GOLD);
        } else {
            amphora(
                b,
                v(x, 0.55, 0.73),
                0.51,
                if i % 2 == 0 { BRICK } else { BLUE },
            );
        }
    }
    amphora(b, v(0.83, 0.12, 0.79), 1.0, BRICK);
}

fn mosaic(b: &mut Batch, p: Vec3, cols: usize, rows: usize, step: f32) {
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

fn cistern(b: &mut Batch) {
    foundation(b, 1.56);
    mosaic(b, v(0.0, 0.164, 0.0), 13, 9, 0.15);
    pool(b, v(0.0, 0.18, 0.24), 1.29, 0.63);
    arch(b, v(0.0, 0.14, -0.41), 0.78, 0.58, 0.17, 0.27, BRICK);
    b.slab(v(0.0, 1.28, -0.41), v(1.31, 0.14, 0.36), TRAVERTINE);
    amphora(b, v(0.96, 0.13, 0.63), 0.82, BRICK);
}

fn pipes(b: &mut Batch) {
    foundation(b, 1.37);
    for x in [-0.75, 0.75] {
        arch(b, v(x, 0.12, -0.10), 0.91, 1.28, 0.20, 0.58, BRICK);
    }
    b.slab(v(0.0, 1.98, -0.10), v(2.79, 0.19, 0.69), TRAVERTINE);
    b.slab(v(0.0, 2.12, -0.10), v(2.65, 0.16, 0.58), BRICK);
    b.slab(v(0.0, 2.23, -0.10), v(2.59, 0.12, 0.61), TRAVERTINE);
    pool(b, v(0.0, 0.143, 0.55), 1.36, 0.41);
}

fn baths(b: &mut Batch) {
    foundation(b, 2.0);
    b.slab(v(0.0, 0.73, -0.63), v(2.40, 1.15, 0.72), BRICK);
    masonry(b, v(0.0, 0.14, -0.248), 2.37, 1.10, BRICK);
    for x in [-0.68, 0.68] {
        arch(b, v(x, 0.14, -0.20), 0.56, 0.74, 0.128, 0.16, TRAVERTINE);
        window(b, v(x, 0.68, -0.224), 0.39, 0.59);
    }
    b.slab(v(0.0, 1.36, -0.64), v(2.51, 0.11, 0.87), TRAVERTINE);
    b.frustum(v(0.0, 1.29, -0.60), 0.62, 0.47, 0.33, 14, TILE);
    b.frustum(v(0.0, 1.60, -0.60), 0.46, 0.17, 0.26, 14, shift(TILE, 9));
    b.frustum(v(0.0, 1.85, -0.60), 0.16, 0.14, 0.12, 10, TRAVERTINE);
    mosaic(b, v(0.0, 0.164, 0.47), 13, 7, 0.16);
    pool(b, v(0.0, 0.185, 0.48), 1.70, 0.64);
    flowerpot(b, v(1.16, 0.17, 0.59), 0.69, [199, 132, 150]);
}

fn portico(b: &mut Batch) {
    foundation(b, 1.73);
    for x in [-0.75, 0.75] {
        arch(b, v(x, 0.142, 0.24), 0.82, 0.91, 0.18, 0.40, BRICK);
        arch(b, v(x, 0.142, -0.46), 0.82, 0.91, 0.18, 0.24, BRICK);
    }
    b.slab(v(0.0, 1.62, -0.16), v(2.60, 0.16, 1.26), TRAVERTINE);
    b.slab(v(0.0, 1.75, -0.16), v(2.65, 0.12, 1.36), TILE);
    b.slab(v(0.0, 0.39, -0.37), v(1.88, 0.115, 0.36), OAK);
}

fn forum(b: &mut Batch) {
    foundation(b, 2.02);
    mosaic(b, v(0.0, 0.165, 0.16), 15, 11, 0.15);
    for x in [-0.80, 0.0, 0.80] {
        arch(b, v(x, 0.14, -0.38), 0.57, 1.17, 0.14, 0.26, BRICK);
        b.slab(v(x, 0.51, 0.13), v(0.59, 0.13, 0.41), OAK);
        amphora(
            b,
            v(x, 0.57, 0.15),
            0.61,
            [WINE, TEAL, GOLD][((x + 0.81) * 1.26) as usize],
        );
    }
    b.slab(v(0.0, 1.82, -0.38), v(2.70, 0.17, 0.55), TRAVERTINE);
    b.slab(v(0.0, 1.97, -0.38), v(2.77, 0.13, 0.63), TILE);
    awning(b, 0.0, 1.48, 2.40, WINE);
}

fn garden(b: &mut Batch) {
    foundation(b, 1.90);
    pool(b, v(0.0, 0.145, 0.58), 0.88, 0.31);
    for x in [-0.84, 0.84] {
        olive(b, v(x, 0.15, -0.42), 0.82);
        flowerpot(b, v(x, 0.16, 0.73), 0.73, [213, 111, 144]);
    }
    mosaic(b, v(0.0, 0.168, -0.26), 5, 6, 0.15);
}

fn horreum(b: &mut Batch) {
    foundation(b, 1.90);
    b.slab(v(0.0, 0.89, -0.37), v(2.42, 1.49, 1.31), BRICK);
    masonry(b, v(0.0, 0.16, 0.30), 2.35, 1.40, BRICK);
    b.slab(v(0.0, 0.74, 0.34), v(0.91, 1.17, 0.08), [78, 59, 46]);
    arch(b, v(0.0, 0.14, 0.38), 0.96, 0.79, 0.14, 0.18, TRAVERTINE);
    roof(b, 2.64, 2.01, 1.68, 0.29, TILE);
    for (x, col) in [(-0.94, BLUE), (0.91, BRICK), (1.14, TRAVERTINE)] {
        amphora(b, v(x, 0.12, 0.81), 1.0, col);
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

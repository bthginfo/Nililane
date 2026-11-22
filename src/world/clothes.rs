use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{cosmetics::Cosmetic, save::Campaign};

use super::{
    buildings::{CREAM, GOLD},
    mesh::{Batch, Tint, shift, v},
};

#[derive(Component)]
pub struct Costume(Cosmetic);

pub(super) fn spawn(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    for item in Cosmetic::ALL {
        parent.spawn((
            Costume(item),
            Mesh3d(meshes.add(model(item))),
            MeshMaterial3d(material.clone()),
            Transform::default(),
            Visibility::Hidden,
        ));
    }
}

pub fn dress(save: Res<Campaign>, mut clothes: Query<(&Costume, &mut Visibility)>) {
    for (item, mut visible) in &mut clothes {
        let next = if save.wardrobe.wearing(item.0) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if next != *visible {
            *visible = next;
        }
    }
}

fn model(item: Cosmetic) -> Mesh {
    let mut fabric = Batch::default();
    match item {
        Cosmetic::Sea => garment(&mut fabric, [53, 127, 155], [193, 224, 210], false),
        Cosmetic::Saffron => garment(&mut fabric, [213, 146, 66], [241, 201, 117], false),
        Cosmetic::Toga => garment(&mut fabric, [165, 78, 121], [241, 213, 178], true),
        Cosmetic::Olive => garment(&mut fabric, [105, 139, 71], [215, 199, 125], false),
        Cosmetic::Linen => garment(&mut fabric, [235, 220, 177], [139, 63, 84], true),
        Cosmetic::Coral => garment(&mut fabric, [207, 105, 104], [249, 191, 137], true),
        Cosmetic::Night => garment(&mut fabric, [47, 59, 114], [155, 179, 211], false),
        Cosmetic::Mosaic => garment(&mut fabric, [61, 144, 135], [235, 166, 95], true),
        Cosmetic::Straw => hat(&mut fabric, [203, 155, 85], [55, 119, 121], true),
        Cosmetic::Petasos => hat(&mut fabric, [164, 72, 64], [232, 188, 127], false),
        Cosmetic::Sunbonnet => hat(&mut fabric, [229, 194, 124], [203, 105, 123], true),
        Cosmetic::Indigo => {
            let p = v(0.0, 1.58, -0.055);
            fabric.frustum(p, 0.36, 0.38, 0.065, 12, [55, 68, 131]);
            fabric.frustum(p + Vec3::Y * 0.065, 0.38, 0.19, 0.18, 12, [84, 105, 174]);
            fabric.frustum(p + Vec3::Y * 0.245, 0.19, 0.05, 0.073, 12, [100, 127, 189]);
            fabric.block(
                v(0.0, 1.602, 0.32),
                v(0.47, 0.044, 0.30),
                Quat::from_rotation_x(0.17),
                [50, 62, 119],
            );
        }
        Cosmetic::Diadem | Cosmetic::Headband => {
            let metal = if item == Cosmetic::Diadem {
                GOLD
            } else {
                [206, 106, 103]
            };
            let at = |i: usize| {
                let angle = i as f32 * TAU / 18.0;
                v(
                    angle.cos() * 0.351,
                    1.53 + angle.sin() * 0.038,
                    angle.sin() * 0.317 + 0.024,
                )
            };
            for i in 0..18 {
                fabric.beam(
                    at(i),
                    at(i + 1),
                    0.034,
                    shift(metal, (i % 3) as i16 * 9 - 7),
                );
            }
            if item == Cosmetic::Diadem {
                fabric.gem(v(0.0, 1.603, 0.35), 0.076, 0.082, 0.03, 0.5, GOLD);
                fabric.gem(
                    v(0.0, 1.609, 0.377),
                    0.047,
                    0.058,
                    0.023,
                    0.6,
                    [68, 152, 147],
                );
            }
        }
        Cosmetic::Laurel | Cosmetic::Wreath => {
            for i in 0..14 {
                let angle = i as f32 / 14.0 * TAU;
                let at = v(
                    angle.cos() * 0.337,
                    1.53 + angle.sin() * 0.024,
                    angle.sin() * 0.306 + 0.02,
                );
                let next = angle + TAU / 14.0;
                let reach = v(
                    next.cos() * 0.337,
                    1.53 + next.sin() * 0.024,
                    next.sin() * 0.306 + 0.02,
                );
                fabric.beam(at, reach, 0.025, [86, 139, 74]);
                fabric.block(
                    at + Vec3::Y * 0.045,
                    v(0.064, 0.122, 0.033),
                    Quat::from_rotation_y(-angle) * Quat::from_rotation_z(0.55),
                    shift([101, 171, 92], (i % 4) as i16 * 11 - 12),
                );
                if item == Cosmetic::Wreath && i % 2 == 0 {
                    let petals = [[237, 151, 156], [240, 202, 129], [154, 142, 212]];
                    fabric.gem(
                        at + Vec3::Y * 0.061,
                        0.081,
                        0.049,
                        0.064,
                        angle,
                        petals[i % 3],
                    );
                    fabric.gem(at + Vec3::Y * 0.102, 0.034, 0.021, 0.032, angle, CREAM);
                }
            }
            fabric.gem(v(0.0, 1.555, 0.337), 0.054, 0.042, 0.023, 0.31, GOLD);
        }
        _ => accessory(&mut fabric, item),
    }
    fabric.mesh()
}

fn accessory(b: &mut Batch, item: Cosmetic) {
    match item {
        Cosmetic::Satchel | Cosmetic::Basket => {
            let strap = [139, 88, 59];
            b.beam(v(-0.30, 0.915, 0.29), v(0.37, 0.62, 0.347), 0.042, strap);
            b.beam(v(-0.30, 0.915, -0.28), v(0.36, 0.61, -0.338), 0.042, strap);
            if item == Cosmetic::Satchel {
                b.block(
                    v(0.396, 0.60, 0.19),
                    v(0.26, 0.273, 0.26),
                    Quat::from_rotation_z(0.19),
                    strap,
                );
                b.slab(v(0.397, 0.69, 0.254), v(0.27, 0.096, 0.181), [174, 119, 73]);
                b.gem(v(0.401, 0.64, 0.333), 0.044, 0.047, 0.025, 0.5, GOLD);
            } else {
                let at = v(0.404, 0.432, 0.192);
                for i in 0..5 {
                    let side = i as f32 * 0.012;
                    b.frustum(
                        at + Vec3::Y * (i as f32 * 0.040),
                        0.094 + side,
                        0.103 + side,
                        0.041,
                        12,
                        shift([191, 137, 78], i as i16 * 5 - 8),
                    );
                }
                b.gem(
                    at + v(0.02, 0.194, 0.03),
                    0.13,
                    0.069,
                    0.10,
                    0.1,
                    [115, 159, 80],
                );
            }
        }
        Cosmetic::Shell | Cosmetic::Pearls | Cosmetic::Amulet => {
            let bead = if item == Cosmetic::Pearls {
                [251, 230, 198]
            } else {
                [181, 120, 70]
            };
            for i in 0..13 {
                let x = (i as f32 - 6.0) / 6.0;
                b.gem(
                    v(x * 0.329, 0.855 + x * x * 0.161, 0.391 - x * x * 0.131),
                    0.022,
                    0.025,
                    0.023,
                    i as f32,
                    shift(bead, i % 4 * 6),
                );
            }
            let color = if item == Cosmetic::Amulet {
                [222, 144, 66]
            } else {
                [236, 184, 148]
            };
            b.gem(v(0.0, 0.786, 0.408), 0.071, 0.078, 0.032, 0.38, color);
            if item == Cosmetic::Shell {
                for x in [-0.042, 0.0, 0.042] {
                    b.beam(v(0.0, 0.814, 0.438), v(x, 0.736, 0.435), 0.013, CREAM);
                }
            }
        }
        Cosmetic::Brooch => {
            b.gem(v(0.28, 0.92, 0.323), 0.092, 0.086, 0.036, 0.23, GOLD);
            b.gem(
                v(0.28, 0.922, 0.350),
                0.063,
                0.058,
                0.038,
                0.31,
                [80, 168, 133],
            );
        }
        Cosmetic::Scarf => {
            b.beam(
                v(-0.29, 0.961, 0.27),
                v(0.285, 0.925, 0.35),
                0.10,
                [190, 80, 104],
            );
            b.gem(
                v(0.16, 0.925, 0.371),
                0.084,
                0.066,
                0.031,
                0.5,
                [222, 106, 132],
            );
            b.block(
                v(0.22, 0.813, 0.396),
                v(0.11, 0.236, 0.047),
                Quat::from_rotation_z(0.40),
                [190, 80, 104],
            );
        }
        Cosmetic::Belt => {
            let point = |i: usize| {
                let a = i as f32 * TAU / 18.0;
                v(a.cos() * 0.425, 0.59, a.sin() * 0.383 - 0.017)
            };
            for i in 0..18 {
                b.beam(
                    point(i),
                    point(i + 1),
                    0.067,
                    shift([190, 138, 71], i as i16 % 3 * 10),
                );
            }
            b.gem(v(0.04, 0.595, 0.387), 0.079, 0.064, 0.026, 0.4, GOLD);
        }
        _ => unreachable!("only accessories use these meshes"),
    }
}

fn garment(m: &mut Batch, color: Tint, edge: Tint, mantle: bool) {
    let rings = [
        (0.44, 0.364, 0.352),
        (0.50, 0.407, 0.379),
        (0.72, 0.454, 0.389),
        (0.84, 0.456, 0.385),
        (0.935, 0.387, 0.329),
    ];
    let point = |ring: usize, i: usize| {
        let (y, rx, rz) = rings[ring];
        let a = i as f32 / 18.0 * TAU;
        let fold = 0.013 * (a * 9.0).cos();
        let diagonal = a.cos() * if mantle { 0.064 } else { 0.021 };
        v(
            a.cos() * (rx + fold),
            y + diagonal * (0.4 + ring as f32 * 0.19),
            a.sin() * (rz + fold) - 0.024,
        )
    };
    for ring in 0..rings.len() - 1 {
        for i in 0..18 {
            let trim = ring == 0 || mantle && matches!(i, 3 | 4 | 13 | 14) && ring >= 2;
            m.face(
                &[
                    point(ring, i),
                    point(ring + 1, i),
                    point(ring + 1, i + 1),
                    point(ring, i + 1),
                ],
                shift(if trim { edge } else { color }, (i * 9 % 17) as i16 - 9),
            );
        }
    }
    m.face(
        &[
            v(-0.31, 0.61, 0.315),
            v(-0.27, 0.51, 0.342),
            v(0.07, 0.66, 0.409),
            v(0.025, 0.73, 0.406),
        ],
        edge,
    );
    m.face(
        &[
            v(0.025, 0.73, 0.406),
            v(0.07, 0.66, 0.409),
            v(0.37, 0.815, 0.279),
            v(0.29, 0.872, 0.268),
        ],
        shift(edge, 6),
    );
    m.gem(v(-0.311, 0.857, 0.258), 0.069, 0.065, 0.032, 0.4, GOLD);
    m.gem(
        v(-0.312, 0.856, 0.287),
        0.035,
        0.035,
        0.013,
        0.4,
        if mantle { edge } else { CREAM },
    );
}

fn hat(m: &mut Batch, color: Tint, band: Tint, straw: bool) {
    let center = v(0.038, 1.59, -0.019);
    m.frustum(
        center,
        0.42,
        if straw { 0.49 } else { 0.44 },
        0.044,
        14,
        shift(color, -14),
    );
    m.frustum(
        center + Vec3::Y * 0.044,
        if straw { 0.49 } else { 0.44 },
        0.298,
        0.071,
        14,
        color,
    );
    m.frustum(center + Vec3::Y * 0.105, 0.298, 0.277, 0.061, 14, band);
    m.frustum(
        center + Vec3::Y * 0.157,
        0.278,
        if straw { 0.185 } else { 0.147 },
        0.145,
        14,
        shift(color, 13),
    );
    m.gem(
        center + v(0.153, 0.139, 0.229),
        0.044,
        0.046,
        0.022,
        0.4,
        GOLD,
    );
}

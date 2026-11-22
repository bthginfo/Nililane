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
        Cosmetic::Straw => hat(&mut fabric, [203, 155, 85], [55, 119, 121], true),
        Cosmetic::Petasos => hat(&mut fabric, [164, 72, 64], [232, 188, 127], false),
        Cosmetic::Laurel => {
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
            }
            fabric.gem(v(0.0, 1.555, 0.337), 0.054, 0.042, 0.023, 0.31, GOLD);
        }
    }
    fabric.mesh()
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

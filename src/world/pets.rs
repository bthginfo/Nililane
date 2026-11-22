use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::{
    campaign::CHAPTERS,
    game::{Game, Panel, View},
    save::Campaign,
    street::{self, Activity, Spot},
};

use super::mesh::{Batch, Tint, shift, v};

#[derive(Component)]
pub(crate) struct StreetProp {
    slot: usize,
    activity: Activity,
}

#[derive(Component)]
pub(crate) struct PetJoint {
    slot: usize,
    activity: Activity,
    part: Part,
}

#[derive(Clone, Copy)]
enum Part {
    Head,
    Tail,
    Leg(usize),
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        perceptual_roughness: 0.84,
        ..default()
    });
    let debris = [Activity::Leaves, Activity::Scraps].map(|activity| meshes.add(litter(activity)));
    for slot in [0, 2] {
        for (activity, mesh) in [Activity::Leaves, Activity::Scraps]
            .into_iter()
            .zip(&debris)
        {
            commands.spawn((
                StreetProp { slot, activity },
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::default(),
                Visibility::Hidden,
            ));
        }
    }
    for activity in [Activity::Cat, Activity::Dog] {
        let slot = 1;
        let cat = activity == Activity::Cat;
        let coat = if cat { [210, 132, 65] } else { [167, 110, 78] };
        let fur = if cat {
            [251, 212, 148]
        } else {
            [236, 210, 173]
        };
        commands
            .spawn((
                StreetProp { slot, activity },
                Mesh3d(meshes.add(torso(cat, coat, fur))),
                MeshMaterial3d(material.clone()),
                Transform::default(),
                Visibility::Hidden,
            ))
            .with_children(|pet| {
                let mut joint = |part, rest, mesh| {
                    pet.spawn((
                        PetJoint {
                            slot,
                            activity,
                            part,
                        },
                        Mesh3d(meshes.add(mesh)),
                        MeshMaterial3d(material.clone()),
                        Transform::from_translation(rest),
                    ));
                };
                joint(Part::Head, v(0.0, 0.50, 0.255), head(cat, coat, fur));
                joint(Part::Tail, v(0.0, 0.432, -0.313), tail(cat, coat, fur));
                for n in 0..4 {
                    let rest = v(
                        if n % 2 == 0 { -0.114 } else { 0.114 },
                        0.30,
                        if n / 2 == 0 { -0.19 } else { 0.20 },
                    );
                    joint(Part::Leg(n), rest, leg(cat, coat, fur));
                }
            });
    }
}

fn torso(cat: bool, coat: Tint, fur: Tint) -> Mesh {
    let mut mesh = Batch::default();
    let width = if cat { 0.156 } else { 0.194 };
    mesh.gem(v(0.0, 0.356, -0.055), width, 0.169, 0.337, 0.18, coat);
    mesh.gem(
        v(0.0, 0.356, -0.205),
        width * 1.07,
        0.168,
        0.176,
        0.4,
        shift(coat, -9),
    );
    mesh.gem(v(0.0, 0.395, 0.179), width * 0.94, 0.159, 0.172, 0.34, coat);
    mesh.gem(v(0.0, 0.348, 0.259), width * 0.68, 0.119, 0.097, 0.44, fur);
    if cat {
        for i in 0..4 {
            let z = -0.22 + i as f32 * 0.083;
            for sign in [-1.0, 1.0] {
                mesh.face(
                    &[
                        v(0.04 * sign, 0.513, z),
                        v(0.15 * sign, 0.402, z - 0.051),
                        v(0.148 * sign, 0.413, z - 0.009),
                        v(0.038 * sign, 0.514, z + 0.032),
                    ],
                    [168, 83, 52],
                );
            }
        }
    } else {
        mesh.gem(
            v(-0.114, 0.403, -0.115),
            0.102,
            0.116,
            0.185,
            0.26,
            [135, 83, 63],
        );
        mesh.gem(v(0.103, 0.458, -0.177), 0.125, 0.078, 0.109, 0.43, fur);
        for i in 0..12 {
            let point = |j: usize| {
                let a = j as f32 / 12.0 * TAU;
                v(
                    a.cos() * 0.139,
                    0.426 + a.sin() * 0.103,
                    0.266 - a.sin() * 0.037,
                )
            };
            mesh.beam(point(i), point(i + 1), 0.023, [66, 138, 136]);
        }
        mesh.gem(
            v(0.0, 0.32, 0.292),
            0.041,
            0.043,
            0.019,
            0.3,
            [242, 190, 95],
        );
    }
    mesh.mesh()
}

fn head(cat: bool, coat: Tint, fur: Tint) -> Mesh {
    let mut mesh = Batch::default();
    let width = if cat { 0.152 } else { 0.178 };
    mesh.gem(v(0.0, 0.05, 0.051), width, 0.142, 0.135, 0.26, coat);
    mesh.gem(
        v(0.0, 0.047, 0.164),
        width * 0.61,
        0.078,
        if cat { 0.066 } else { 0.108 },
        0.2,
        fur,
    );
    for side in [-1.0, 1.0] {
        mesh.gem(v(side * 0.072, 0.032, 0.162), 0.075, 0.068, 0.068, 0.3, fur);
        let eye = v(side * 0.087, 0.106, if cat { 0.153 } else { 0.163 });
        mesh.gem(eye, 0.027, 0.032, 0.016, 0.0, [55, 42, 44]);
        if cat {
            mesh.gem(
                eye + v(0.0, 0.0, 0.013),
                0.021,
                0.024,
                0.007,
                0.0,
                [129, 161, 76],
            );
            mesh.gem(
                eye + v(0.0, 0.0, 0.020),
                0.009,
                0.022,
                0.006,
                0.0,
                [42, 43, 41],
            );
        }
        mesh.gem(
            eye + v(0.007, 0.012, 0.020),
            0.009,
            0.010,
            0.007,
            0.45,
            [250, 240, 223],
        );
        if cat {
            let base = [v(side * 0.058, 0.135, 0.092), v(side * 0.148, 0.113, 0.051)];
            let tip = v(side * 0.147, 0.303, 0.018);
            mesh.face(&[base[0], base[1], tip], coat);
            mesh.face(&[base[1], base[0], tip], shift(coat, -14));
            let inner = [
                v(side * 0.079, 0.152, 0.089),
                v(side * 0.135, 0.139, 0.072),
                v(side * 0.139, 0.258, 0.052),
            ];
            mesh.face(&inner, [218, 145, 148]);
            mesh.face(&[inner[1], inner[0], inner[2]], [218, 145, 148]);
            for n in 0..3 {
                mesh.beam(
                    v(side * 0.084, 0.035 + n as f32 * 0.012, 0.211),
                    v(side * 0.232, 0.015 + n as f32 * 0.042, 0.177),
                    0.007,
                    [236, 216, 169],
                );
            }
        } else {
            mesh.gem(
                v(side * 0.165, 0.076, 0.028),
                0.069,
                0.079,
                0.078,
                0.4,
                shift(coat, -25),
            );
            mesh.gem(
                v(side * 0.189, -0.007, 0.081),
                0.058,
                0.113,
                0.064,
                0.2,
                shift(coat, -38),
            );
            mesh.gem(v(side * 0.097, 0.146, 0.139), 0.045, 0.021, 0.020, 0.2, fur);
        }
    }
    mesh.gem(
        v(0.0, 0.064, if cat { 0.227 } else { 0.267 }),
        0.038,
        0.031,
        0.025,
        0.4,
        if cat { [176, 102, 113] } else { [59, 43, 44] },
    );
    mesh.mesh()
}

fn tail(cat: bool, coat: Tint, fur: Tint) -> Mesh {
    let mut mesh = Batch::default();
    let points = if cat {
        [
            v(0.0, 0.0, 0.0),
            v(0.0, 0.09, -0.125),
            v(0.0, 0.251, -0.173),
            v(0.027, 0.384, -0.15),
            v(0.083, 0.422, -0.097),
        ]
    } else {
        [
            v(0.0, 0.0, 0.0),
            v(0.0, 0.075, -0.113),
            v(0.0, 0.168, -0.235),
            v(0.027, 0.205, -0.295),
            v(0.04, 0.207, -0.345),
        ]
    };
    for (i, line) in points.windows(2).enumerate() {
        let p = line[0];
        let q = line[1];
        let width = 0.060 - i as f32 * 0.008;
        let color = if !cat && i == 3 {
            fur
        } else {
            shift(coat, if i % 2 == 0 { -9 } else { 6 })
        };
        mesh.beam(p, q, width, color);
        mesh.gem(q, width * 0.56, width * 0.57, width * 0.61, 0.32, color);
    }
    mesh.mesh()
}

fn leg(cat: bool, coat: Tint, fur: Tint) -> Mesh {
    let mut mesh = Batch::default();
    let width = if cat { 0.050 } else { 0.060 };
    mesh.gem(v(0.0, -0.060, 0.0), width, 0.112, width * 1.25, 0.4, coat);
    mesh.gem(
        v(0.0, -0.181, 0.008),
        width * 0.76,
        0.089,
        width,
        0.2,
        shift(coat, 9),
    );
    mesh.gem(
        v(0.0, -0.253, 0.031),
        width * 1.01,
        0.041,
        width * 1.42,
        0.2,
        fur,
    );
    mesh.mesh()
}

fn litter(activity: Activity) -> Mesh {
    let mut mesh = Batch::default();
    if activity == Activity::Leaves {
        for i in 0..6 {
            let angle = i as f32 * 2.399;
            let p = v(angle.cos() * 0.14, 0.023, angle.sin() * 0.15);
            let q = |x, y, z| p + Quat::from_rotation_y(angle) * v(x, y, z);
            let tone = [[199, 127, 62], [172, 142, 58], [217, 163, 75]][i % 3];
            for face in [
                [
                    q(-0.082, 0.004, 0.0),
                    q(0.0, 0.032, 0.0),
                    q(0.045, 0.018, 0.062),
                ],
                [
                    q(0.0, 0.032, 0.0),
                    q(0.12, 0.004, 0.0),
                    q(0.045, 0.018, 0.062),
                ],
                [
                    q(-0.082, 0.004, 0.0),
                    q(0.045, 0.006, -0.063),
                    q(0.0, 0.032, 0.0),
                ],
                [
                    q(0.0, 0.032, 0.0),
                    q(0.045, 0.006, -0.063),
                    q(0.12, 0.004, 0.0),
                ],
            ] {
                mesh.face(&face, tone);
            }
        }
    } else {
        mesh.gem(
            v(0.065, 0.059, -0.02),
            0.14,
            0.064,
            0.095,
            0.22,
            [233, 218, 170],
        );
        mesh.gem(
            v(-0.092, 0.032, 0.071),
            0.081,
            0.045,
            0.083,
            0.70,
            [197, 159, 111],
        );
        mesh.block(
            v(0.060, 0.025, -0.16),
            v(0.12, 0.025, 0.069),
            Quat::from_rotation_y(-0.54),
            [191, 100, 70],
        );
    }
    mesh.mesh()
}

type Roots<'w, 's> = Query<
    'w,
    's,
    (
        &'static StreetProp,
        &'static mut Transform,
        &'static mut Visibility,
    ),
    Without<PetJoint>,
>;
type Joints<'w, 's> =
    Query<'w, 's, (&'static PetJoint, &'static mut Transform), Without<StreetProp>>;

pub fn animate(game: Res<Game>, save: Res<Campaign>, mut props: Roots, mut joints: Joints) {
    let town = game.visible_town(&save);
    let slots = if game.view == View::Street && game.panel != Panel::Wardrobe {
        street::spots(town, &CHAPTERS[town.chapter])
    } else {
        [None; street::SPOTS]
    };
    let reaction = |slot: usize, spot: Spot| {
        game.street_action
            .filter(|(id, _, _)| *id == slot && spot.claimed)
            .map_or(0.0, |(_, _, start)| {
                (1.0 - (game.clock - start) / 1.15).clamp(0.0, 1.0)
            })
    };
    for (prop, mut transform, mut visible) in &mut props {
        let Some(spot) = slots[prop.slot].filter(|spot| spot.activity == prop.activity) else {
            *visible = Visibility::Hidden;
            continue;
        };
        let t = game.clock;
        let pulse = reaction(prop.slot, spot);
        let [x, z] = spot.position;
        *visible = Visibility::Inherited;
        let hop = if spot.walking {
            (t * 10.0).sin().abs() * 0.022
        } else {
            0.0
        };
        transform.translation = v(x, 0.058 + hop + (PI * pulse).sin() * 0.065, z);
        transform.rotation = Quat::from_rotation_y(spot.yaw + 0.18 * pulse);
        transform.scale = Vec3::splat(if prop.activity == Activity::Dog {
            1.24
        } else {
            1.0
        });
    }
    for (joint, mut transform) in &mut joints {
        let Some(spot) = slots[joint.slot].filter(|spot| spot.activity == joint.activity) else {
            continue;
        };
        let t = game.clock;
        let pulse = reaction(joint.slot, spot);
        transform.rotation = match joint.part {
            Part::Leg(leg) if spot.walking => {
                let side = if leg == 0 || leg == 3 { 1.0 } else { -1.0 };
                Quat::from_rotation_x((t * 10.0).sin() * side * 0.46)
            }
            Part::Leg(_) => Quat::IDENTITY,
            Part::Tail => {
                Quat::from_rotation_y((t * (3.0 + pulse * 11.0)).sin() * (0.23 + pulse * 0.54))
            }
            Part::Head => {
                Quat::from_rotation_z((t * 1.4).sin() * 0.12 + pulse * 0.2)
                    * Quat::from_rotation_x(pulse * -0.28 + f32::from(spot.remaining < 2.6) * 0.14)
            }
        };
    }
}

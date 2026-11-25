use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::game::{BUILD_TIME, Construction, Game};

use super::{
    mesh::{Batch, Tint, v},
    upgrades,
};

const REPLACE_AT: f32 = 0.44;

#[derive(Component)]
pub struct Assembly {
    pub work: Construction,
    pub at: Vec3,
    pub part: Part,
}

pub enum Part {
    Prior,
    Structure,
    Scaffold,
    Dust(usize),
    Confetti(usize),
}

pub const CONFETTI: usize = 28;
const CONFETTI_LIFE: f32 = 0.95;

fn confetti_piece(tint: Tint) -> Mesh {
    let mut b = Batch::default();
    b.slab(Vec3::ZERO, v(0.13, 0.022, 0.085), tint);
    b.mesh()
}

/// Where a confetti piece flies `t` seconds after a building is finished.
/// Returns `None` before the burst and after the pieces have settled.
pub fn confetti_pose(i: usize, t: f32) -> Option<Transform> {
    if !(0.0..CONFETTI_LIFE).contains(&t) {
        return None;
    }
    let (sin, cos) = (i as f32 * 2.399).sin_cos();
    let spread = 1.0 + (i % 5) as f32 * 0.32;
    let lift = 4.2 + (i % 3) as f32 * 0.8;
    let translation = v(
        cos * spread * 1.5 * t,
        1.4 + lift * t - 7.5 * t * t,
        sin * spread * 1.5 * t * 0.8,
    );
    let shrink = ((CONFETTI_LIFE - t) / 0.3).clamp(0.0, 1.0);
    Some(
        Transform::from_translation(translation)
            .with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                t * (5.0 + (i % 4) as f32),
                t * 3.0 + i as f32,
                t * (4.0 + (i % 3) as f32),
            ))
            .with_scale(Vec3::splat(shrink)),
    )
}

pub struct Parts {
    pub frame: Handle<Mesh>,
    pub chip: Handle<Mesh>,
    pub dust: Handle<StandardMaterial>,
    pub confetti: [Handle<Mesh>; 5],
}

impl Parts {
    pub fn prepare(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            frame: meshes.add(upgrades::scaffold()),
            chip: meshes.add(upgrades::chip()),
            dust: materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::srgba(0.93, 0.82, 0.67, 0.48),
                unlit: true,
                ..default()
            }),
            confetti: [
                [255, 214, 110],
                [240, 120, 96],
                [84, 190, 178],
                [250, 244, 226],
                [226, 132, 170],
            ]
            .map(|tint| meshes.add(confetti_piece(tint))),
        }
    }

    pub fn spawn(
        &self,
        parent: &mut ChildSpawnerCommands,
        at: Vec3,
        work: Construction,
        matte: &Handle<StandardMaterial>,
    ) {
        parent.spawn((
            Mesh3d(self.frame.clone()),
            MeshMaterial3d(matte.clone()),
            Transform::from_translation(at),
            Assembly {
                at,
                work,
                part: Part::Scaffold,
            },
            NotShadowCaster,
        ));
        for i in 0..10 {
            parent.spawn((
                Mesh3d(self.chip.clone()),
                MeshMaterial3d(self.dust.clone()),
                Transform::from_translation(at),
                Assembly {
                    at,
                    work,
                    part: Part::Dust(i),
                },
                NotShadowCaster,
            ));
        }
        for i in 0..CONFETTI {
            parent.spawn((
                Mesh3d(self.confetti[i % self.confetti.len()].clone()),
                MeshMaterial3d(matte.clone()),
                Transform::from_translation(at),
                Visibility::Hidden,
                Assembly {
                    at,
                    work,
                    part: Part::Confetti(i),
                },
                NotShadowCaster,
            ));
        }
    }
}

pub fn animate(game: Res<Game>, mut sites: Query<(&Assembly, &mut Transform, &mut Visibility)>) {
    for (site, mut at, mut visibility) in &mut sites {
        let age = game.clock - site.work.since;
        match site.part {
            Part::Prior => {
                let lower = ((age - 0.31) / (REPLACE_AT - 0.31)).clamp(0.0, 1.0);
                at.translation.y = site.at.y - 0.05 * lower;
                at.scale.y = 1.0 - 0.12 * lower;
                *visibility = if age < REPLACE_AT {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            Part::Structure => {
                let building = site.work.from == 0;
                let (start, initial_scale) = if building {
                    (0.14, 0.02)
                } else {
                    (REPLACE_AT, 0.76)
                };
                let t = ((age - start) / 0.73).clamp(0.0, 1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                let settle = (t * std::f32::consts::PI).sin() * t * 0.041;
                at.scale.y = initial_scale + (1.0 - initial_scale) * eased + settle;
                at.translation.y = site.at.y - 0.057 * (1.0 - eased);
                *visibility = if age < start {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
            }
            Part::Scaffold => {
                let lift = (age * 6.5).clamp(0.02, 1.0);
                let take_down = ((BUILD_TIME - age) * 3.7).clamp(0.017, 1.0);
                at.scale.y = lift.min(take_down);
            }
            Part::Confetti(i) => {
                // Only a new building is celebrated, not every upgrade.
                let pose = (site.work.from == 0)
                    .then(|| confetti_pose(i, age - 0.62))
                    .flatten();
                if let Some(pose) = pose {
                    *at = pose;
                    at.translation += site.at;
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
            Part::Dust(i) => {
                let t = (age - 0.10 - (i % 4) as f32 * 0.12) / 0.86;
                *visibility = if (0.0..1.0).contains(&t) {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *visibility != Visibility::Hidden {
                    let (sin, cos) = (i as f32 * 2.399).sin_cos();
                    let radius = 0.87 + t * (0.69 + (i % 3) as f32 * 0.17);
                    at.translation = site.at
                        + v(
                            cos * radius,
                            0.04 + 0.81 * t * (1.0 - t),
                            sin * radius * 0.76,
                        );
                    at.scale = Vec3::splat(
                        (std::f32::consts::PI * t).sin() * (0.77 + (i % 3) as f32 * 0.23),
                    );
                    at.rotation =
                        Quat::from_rotation_x(i as f32 + t * 2.4) * Quat::from_rotation_z(t * 2.1);
                }
            }
        }
    }
}

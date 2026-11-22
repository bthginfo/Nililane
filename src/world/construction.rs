use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::game::{BUILD_TIME, Construction, Game};

use super::{mesh::v, upgrades};

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
}

pub struct Parts {
    pub frame: Handle<Mesh>,
    pub chip: Handle<Mesh>,
    pub dust: Handle<StandardMaterial>,
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

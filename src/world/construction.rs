use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::{
    campaign::Region,
    game::{BUILD_TIME, Construction, Game},
    model::Kind,
};

use super::{
    buildings, japan,
    mesh::{Batch, v},
    nordic, roman, upgrades,
};

#[derive(Component)]
pub struct Assembly {
    pub work: Construction,
    pub at: Vec3,
    pub rest: f32,
    pub part: Part,
}

pub enum Part {
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
                rest: 1.0,
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
                    rest: 1.0,
                },
                NotShadowCaster,
            ));
        }
    }
}

pub fn addition(region: Region, kind: Kind, from: u8, to: u8) -> Mesh {
    let mut batch = Batch::default();
    for tier in (from + 1)..=to {
        match region {
            Region::Greece => buildings::append_upgrade(&mut batch, kind, tier),
            Region::Rome => roman::append_upgrade(&mut batch, kind, tier),
            Region::Japan => japan::append_upgrade(&mut batch, kind, tier),
            Region::Nordic => nordic::append_upgrade(&mut batch, kind, tier),
        }
    }
    batch.mesh()
}

pub fn animate(game: Res<Game>, mut sites: Query<(&Assembly, &mut Transform, &mut Visibility)>) {
    for (site, mut at, mut visibility) in &mut sites {
        let age = game.clock - site.work.since;
        match site.part {
            Part::Structure => {
                let t = ((age - 0.21) / 0.93).clamp(0.0, 1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                let settle = (t * std::f32::consts::PI).sin() * t * 0.078;
                at.scale = v(1.0, 0.018 + 0.982 * eased + settle, 1.0) * site.rest;
                at.translation.y = site.at.y - 0.057 * (1.0 - eased);
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

use bevy::prelude::*;

use crate::{
    campaign::CHAPTERS,
    game::{Game, View},
    model::Weather,
    save::Campaign,
};

use super::mesh::{Batch, Grain, shift, v};

#[derive(Component)]
pub(crate) enum Effect {
    Cloud(usize),
    Rain(usize),
    Swell(usize),
}

#[derive(Resource)]
pub(crate) struct Sky {
    cloud: Handle<StandardMaterial>,
    rain: Handle<StandardMaterial>,
    waves: Handle<StandardMaterial>,
    weather: Option<(Weather, Weather, f32)>,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cloud = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });
    let rain = materials.add(StandardMaterial {
        base_color: Color::srgba(0.78, 0.92, 0.96, 0.55),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    });
    let waves = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });
    for i in 0..5 {
        let mut b = Batch::default();
        let mut grain = Grain::new(47 + i as u32 * 4611);
        for j in 0..7 {
            let x = (j as f32 - 3.0) * 0.79;
            b.gem(
                v(x, grain.span(-0.31, 0.31), grain.span(-0.36, 0.38)),
                grain.span(0.73, 1.23),
                grain.span(0.45, 0.87),
                grain.span(0.78, 1.14),
                grain.span(0.0, 2.5),
                shift([207, 219, 216], grain.span(-23.0, 12.0) as i16),
            );
        }
        commands.spawn((
            Effect::Cloud(i),
            Mesh3d(meshes.add(b.mesh())),
            MeshMaterial3d(cloud.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    }
    let mut rng = Grain::new(781903);
    for i in 0..3 {
        let mut b = Batch::default();
        for _ in 0..130 {
            let p = v(
                rng.span(-22.0, 23.0),
                rng.span(-4.0, 6.9),
                rng.span(-8.0, 16.7),
            );
            let length = rng.span(0.23, 0.69);
            let end = p + v(-0.12, -length, 0.05);
            b.face(
                &[p, end, end + Vec3::X * 0.029, p + Vec3::X * 0.029],
                [217, 239, 244],
            );
        }
        commands.spawn((
            Effect::Rain(i),
            Mesh3d(meshes.add(b.mesh())),
            MeshMaterial3d(rain.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    }
    for i in 0..2 {
        let mut b = Batch::default();
        for z in 0..5 {
            let z = 5.05 + z as f32 * 1.22 + i as f32 * 0.61;
            for n in 0..18 {
                let x = -24.7 + n as f32 * 2.9 + rng.span(-0.73, 0.83);
                let head = v(
                    x + rng.span(-0.28, 0.32),
                    rng.span(0.030, 0.116),
                    z + rng.span(-0.39, 0.40),
                );
                let w = rng.span(0.40, 1.32);
                let a = head + v(-w, -head.y, -0.06);
                let c = head + v(w, -head.y, 0.02);
                let tip = head + v(0.07, -head.y, 0.19);
                b.face(&[a, tip, head], [105, 166, 164]);
                b.face(&[head, tip, c], [121, 182, 174]);
                let crest = head + v(0.09, 0.004, -0.031);
                b.face(
                    &[a + Vec3::Y * 0.02, head, crest, c + v(-0.18, 0.019, -0.02)],
                    [164, 204, 193],
                );
            }
        }
        commands.spawn((
            Effect::Swell(i),
            Mesh3d(meshes.add(b.mesh())),
            MeshMaterial3d(waves.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    }
    commands.insert_resource(Sky {
        cloud,
        rain,
        waves,
        weather: None,
    });
}

pub fn animate(
    mut sky: ResMut<Sky>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: Query<(&Effect, &mut Transform, &mut Visibility)>,
    game: Res<Game>,
    save: Res<Campaign>,
) {
    let town = game.visible_town(&save);
    let weather = if game.view == View::Street && game.panel != crate::game::Panel::Wardrobe {
        town.sky_blend(&CHAPTERS[town.chapter])
    } else {
        (Weather::Clear, Weather::Clear, 1.0)
    };
    let (before, after, blend) = weather;
    let w = before.swell().lerp(after.swell(), blend);
    let rain = before.rain_strength().lerp(after.rain_strength(), blend);
    let coverage = |sky| match sky {
        Weather::Storm | Weather::Rain => 5.0,
        Weather::Cloud => 4.6,
        Weather::Wind => 3.3,
        Weather::Feast => 1.8,
        _ => 1.45,
    };
    let cloud_count = coverage(before).lerp(coverage(after), blend);
    if sky.weather != Some(weather) {
        sky.weather = Some(weather);
        if let Some(mat) = materials.get_mut(&sky.cloud) {
            let shade = 1.0 - rain * 0.47 - (cloud_count - 1.0) * 0.019;
            mat.base_color = Color::srgb(shade, shade + rain * 0.065, shade + rain * 0.09);
        }
        if let Some(mat) = materials.get_mut(&sky.rain) {
            mat.base_color = Color::srgba(0.79, 0.91, 0.96, rain * 0.72);
        }
        if let Some(mat) = materials.get_mut(&sky.waves) {
            mat.base_color = Color::srgb(1.0 - w * 0.19, 1.0 - w * 0.11, 1.0);
        }
    }
    let t = game.clock;
    for (effect, mut transform, mut visible) in &mut effects {
        match *effect {
            Effect::Cloud(i) => {
                let grow = (cloud_count - i as f32).clamp(0.0, 1.0);
                *visible = if grow > 0.025 {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                let speed = 0.35 + w * 0.58;
                let x = (t * speed + i as f32 * 10.70).rem_euclid(52.0) - 22.0;
                let z = -11.35 - (i % 3) as f32 * 2.32;
                transform.translation = v(x, 4.75 + (i % 2) as f32 * 1.18, z);
                transform.scale = v(0.89 + w * 0.81, 0.47 + w * 0.34, 0.89 + w * 0.44) * grow;
            }
            Effect::Rain(i) => {
                *visible = if rain > 0.04 && (rain > 0.72 || i < 2) {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                let beat = (t * (6.5 + w * 5.0) + i as f32 * 2.84).rem_euclid(5.9);
                transform.translation = v(beat * (w - 0.27), 4.2 - beat, 0.0);
                transform.rotation = Quat::from_rotation_z(0.15 - w * 0.22);
            }
            Effect::Swell(i) => {
                *visible = if w > 0.46 {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                let beat = t * (1.17 + w) + i as f32 * std::f32::consts::PI;
                transform.translation = v(
                    beat.cos() * 0.11,
                    super::landscape::WAKE_LEVEL + 0.014 + beat.sin() * 0.008 + i as f32 * 0.004,
                    beat.cos() * 0.32,
                );
                transform.scale = v(1.0, 0.68 + w * 1.43, 1.0);
            }
        }
    }
}

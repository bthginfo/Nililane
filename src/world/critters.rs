//! Life on the islands: butterflies by day, fireflies by night, gulls over the coast.

use bevy::{pbr::NotShadowCaster, prelude::*};

use super::{
    glow::{Glow, Halo},
    mesh::{Batch, Tint, v},
};
use crate::adventure::{Adventure, Terrain, terrain::unit};

#[derive(Component)]
pub(crate) struct Firefly {
    anchor: Vec3,
    phase: f32,
}

#[derive(Component)]
pub(crate) struct Gull {
    centre: Vec3,
    radius: f32,
    speed: f32,
    phase: f32,
}

/// How dark the island is: 0 at noon, about 0.86 at midnight.
fn darkness(adventure: &Adventure) -> f32 {
    adventure.active().map_or(0.0, |island| {
        let phase = island.phase * std::f32::consts::TAU;
        ((phase.cos() + 0.09) * 0.79).clamp(0.0, 0.86)
    })
}

#[derive(Component)]
pub(crate) struct Butterfly {
    anchor: Vec3,
    phase: f32,
    roam: f32,
}

fn wings(color: Tint) -> Mesh {
    let mut b = Batch::default();
    let body = [70, 56, 52];
    b.beam(v(0.0, 0.0, -0.05), v(0.0, 0.0, 0.05), 0.018, body);
    for side in [-1.0_f32, 1.0] {
        let fore = [
            v(0.0, 0.0, -0.01),
            v(side * 0.12, 0.012, -0.09),
            v(side * 0.15, 0.012, 0.0),
        ];
        let hind = [
            v(0.0, 0.0, 0.0),
            v(side * 0.11, 0.008, 0.02),
            v(side * 0.06, 0.006, 0.09),
        ];
        for mut wing in [fore, hind] {
            if side < 0.0 {
                wing.swap(1, 2);
            }
            b.face(&wing, color);
            wing.swap(1, 2);
            b.face(&wing, color);
        }
    }
    b.mesh()
}

/// Scatters butterflies and fireflies over the meadow and sends gulls over the coast.
pub(crate) fn spawn(
    root: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    matte: &Handle<StandardMaterial>,
    glow: &Glow,
    bird: &Handle<Mesh>,
    terrain: &Terrain,
) {
    let seed = terrain.level.index as u32 * 7_717 + 3;
    let mut fireflies = 0;
    for i in 0..80_u32 {
        if fireflies >= 18 {
            break;
        }
        let a = unit(seed + i * 17) * std::f32::consts::TAU;
        let r = 5.0 + unit(seed + i * 31) * 18.0;
        let p = Vec2::new(a.cos() * r * 1.2, a.sin() * r);
        if terrain.shore(p) < 3.0 || terrain.height(p) > 3.0 {
            continue;
        }
        fireflies += 1;
        let anchor = v(p.x, terrain.height(p) + 0.5, p.y);
        root.spawn((
            Firefly {
                anchor,
                phase: unit(seed + i * 43) * 50.0,
            },
            Halo {
                strength: 0.0,
                ..Halo::new(0.34, i as f32)
            },
            Mesh3d(glow.quad.clone()),
            MeshMaterial3d(glow.firefly.clone()),
            Transform::from_translation(anchor),
            NotShadowCaster,
        ));
    }
    for i in 0..5_u32 {
        let a = unit(seed + i * 59 + 5) * std::f32::consts::TAU;
        let direction = Vec2::from_angle(a);
        let coast = direction * (terrain.shore(direction) + 1.0 - 2.5);
        root.spawn((
            Gull {
                centre: v(coast.x, 3.6 + unit(seed + i * 7) * 1.6, coast.y),
                radius: 3.5 + unit(seed + i * 11) * 3.5,
                speed: 0.30 + unit(seed + i * 13) * 0.2,
                phase: unit(seed + i * 19) * 20.0,
            },
            Mesh3d(bird.clone()),
            MeshMaterial3d(matte.clone()),
            Transform::from_translation(v(coast.x, 4.0, coast.y)),
            NotShadowCaster,
        ));
    }
    let kinds: Vec<Handle<Mesh>> = [
        [250, 246, 232],
        [247, 206, 84],
        [238, 140, 72],
        [152, 196, 236],
    ]
    .into_iter()
    .map(|color| meshes.add(wings(color)))
    .collect();
    let seed = terrain.level.index as u32 * 4_271 + 17;
    let mut placed = 0;
    for i in 0..64_u32 {
        if placed >= 12 {
            break;
        }
        let a = unit(seed + i * 13) * std::f32::consts::TAU;
        let r = 6.0 + unit(seed + i * 29) * 20.0;
        let p = Vec2::new(a.cos() * r * 1.2, a.sin() * r);
        if terrain.shore(p) < 3.0 || terrain.height(p) > 3.0 {
            continue;
        }
        placed += 1;
        root.spawn((
            Butterfly {
                anchor: v(p.x, terrain.height(p) + 0.42, p.y),
                phase: unit(seed + i * 41) * 40.0,
                roam: 1.2 + unit(seed + i * 53) * 1.4,
            },
            Mesh3d(kinds[(i as usize) % kinds.len()].clone()),
            MeshMaterial3d(matte.clone()),
            Transform::from_xyz(p.x, terrain.height(p) + 0.42, p.y),
            NotShadowCaster,
        ));
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn animate(
    time: Res<Time>,
    adventure: Res<Adventure>,
    mut butterflies: Query<(&Butterfly, &mut Transform, &mut Visibility)>,
    mut fireflies: Query<(&Firefly, &mut Halo, &mut Transform), Without<Butterfly>>,
    mut gulls: Query<(&Gull, &mut Transform), (Without<Butterfly>, Without<Firefly>)>,
) {
    let night = darkness(&adventure);
    let t = time.elapsed_secs();
    let dusk = ((night - 0.45) / 0.3).clamp(0.0, 1.0);
    for (fly, mut halo, mut transform) in &mut fireflies {
        let s = t * 0.35 + fly.phase;
        transform.translation = fly.anchor
            + v(
                (s * 1.1).sin() * 1.3 + (s * 2.9).sin() * 0.2,
                (s * 1.7).sin() * 0.25,
                (s * 0.8).cos() * 1.1,
            );
        // Each firefly glows in slow pulses with a dark pause in between.
        let pulse = ((t * 1.3 + fly.phase).sin() * 1.6 - 0.4).clamp(0.0, 1.0);
        halo.strength = dusk * pulse;
    }
    for (gull, mut transform) in &mut gulls {
        let a = t * gull.speed + gull.phase;
        transform.translation = gull.centre
            + v(
                a.cos() * gull.radius,
                (a * 2.0).sin() * 0.35,
                a.sin() * gull.radius * 0.8,
            );
        transform.rotation = Quat::from_rotation_y(-a - std::f32::consts::FRAC_PI_2)
            * Quat::from_rotation_z((t * 4.7 + gull.phase).sin() * 0.14);
        transform.scale = Vec3::splat(if night < 0.5 { 1.3 } else { 0.0 });
    }
    let shown = if night < 0.35 {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    for (fly, mut transform, mut visibility) in &mut butterflies {
        if *visibility != shown {
            *visibility = shown;
        }
        let s = t * 0.55 + fly.phase;
        let wander = v(
            (s * 0.9).sin() + (s * 2.3).sin() * 0.25,
            0.0,
            (s * 0.7).cos() + (s * 1.9).cos() * 0.25,
        ) * fly.roam;
        let bob = (t * 5.0 + fly.phase).sin() * 0.06 + (s * 1.3).sin() * 0.12;
        let next = fly.anchor + wander + Vec3::Y * bob;
        let heading = next - transform.translation;
        if heading.length_squared() > 1e-6 {
            transform.rotation = Quat::from_rotation_y(heading.x.atan2(heading.z));
        }
        transform.translation = next;
        let flap = (t * 17.0 + fly.phase).sin().abs();
        transform.scale = v(0.25 + flap * 0.85, 1.0, 1.0) * 1.6;
    }
}

//! Butterflies that drift over the island meadows by day.

use bevy::{pbr::NotShadowCaster, prelude::*};

use super::mesh::{Batch, Tint, v};
use crate::adventure::{Adventure, Terrain, terrain::unit};

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

/// Scatters a few butterflies over the meadow around the settlement.
pub(crate) fn spawn(
    root: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    matte: &Handle<StandardMaterial>,
    terrain: &Terrain,
) {
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

pub(crate) fn animate(
    time: Res<Time>,
    adventure: Res<Adventure>,
    mut butterflies: Query<(&Butterfly, &mut Transform, &mut Visibility)>,
) {
    let night = adventure.active().map_or(0.0, |island| {
        let phase = island.phase * std::f32::consts::TAU;
        ((phase.cos() + 0.09) * 0.79).clamp(0.0, 0.86)
    });
    let shown = if night < 0.35 {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    let t = time.elapsed_secs();
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

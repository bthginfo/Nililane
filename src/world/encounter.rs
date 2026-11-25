use std::f32::consts::{PI, TAU};

use bevy::{
    ecs::system::SystemParam, pbr::NotShadowCaster, prelude::*, render::mesh::skinning::SkinnedMesh,
};

use crate::{
    adventure::{Adventure, EncounterStatus, Event, Target},
    campaign::Region,
    game::{Game, Panel, View},
};

use super::{
    Art,
    adventure::position,
    buildings::amphora,
    mesh::{Batch, Tint, shift, v},
    people::{self, Cast, StationGesture, WorkerJoint},
    upgrades::crate_box,
};

#[derive(Resource, Default)]
pub(crate) struct Scene {
    signature: Option<Signature>,
    root: Option<Entity>,
}

#[derive(PartialEq)]
struct Signature {
    level: usize,
    statuses: [EncounterStatus; 3],
    sources: [Option<Vec2>; 3],
}

#[derive(Component)]
pub(crate) enum Motion {
    Fire { index: u8, slot: usize },
    Smoke { index: u8, slot: usize },
    Drops { index: u8 },
    Banner { wave: f32, at: Vec3 },
    Shine { at: Vec3 },
    Frame { slot: usize },
}

#[derive(Component)]
pub(crate) struct Visitor {
    variant: usize,
    slot: usize,
    gesture: StationGesture,
    arrival: Option<(Vec3, Vec3)>,
}

#[derive(Clone, Copy)]
struct Colorway {
    wall: Tint,
    wood: Tint,
    roof: Tint,
    cloth: Tint,
}

fn colors(region: Region) -> Colorway {
    let (wall, wood, roof, cloth) = match region {
        Region::Greece => (
            [221, 204, 167],
            [138, 94, 59],
            [172, 89, 60],
            [59, 143, 166],
        ),
        Region::Rome => ([226, 189, 145], [128, 83, 53], [169, 75, 52], [180, 77, 83]),
        Region::Japan => (
            [199, 185, 157],
            [106, 68, 50],
            [66, 91, 108],
            [164, 90, 105],
        ),
        Region::Nordic => (
            [167, 163, 142],
            [117, 77, 51],
            [83, 112, 100],
            [189, 136, 69],
        ),
        Region::Oasis => (
            [224, 172, 113],
            [139, 91, 52],
            [59, 132, 143],
            [171, 87, 114],
        ),
    };
    Colorway {
        wall,
        wood,
        roof,
        cloth,
    }
}

fn ground(b: &mut Batch, c: Colorway) {
    b.frustum(Vec3::ZERO, 1.19, 1.08, 0.087, 11, shift(c.wall, -12));
    b.slab(v(0.04, 0.076, 0.51), v(1.54, 0.071, 0.43), c.wall);
}

fn shelter(region: Region, damaged: bool) -> Mesh {
    let c = colors(region);
    let mut b = Batch::default();
    ground(&mut b, c);
    if damaged {
        for (x, z, tall) in [
            (-0.75, -0.37, 0.98),
            (0.64, -0.44, 0.43),
            (-0.66, 0.43, 0.51),
        ] {
            let p = v(x, 0.091, z);
            b.frustum(p, 0.158, 0.092, tall, 7, shift(c.wall, -20));
            b.gem(
                p + Vec3::Y * tall,
                0.086,
                0.065,
                0.09,
                0.50,
                shift(c.wall, 11),
            );
        }
        for i in 0..12 {
            let theta = i as f32 * 2.399;
            let radius = 0.27 + (i % 6) as f32 * 0.09;
            let p = v(
                radius * theta.cos(),
                0.14 + (i % 4) as f32 * 0.023,
                radius * theta.sin(),
            );
            let r = 0.13 + 0.04 * (i % 3) as f32;
            b.gem(
                p,
                r,
                r * 0.62,
                r * 0.83,
                theta,
                shift(c.wall, i as i16 * 4 - 20),
            );
        }
        b.beam(v(-0.71, 0.60, -0.52), v(0.61, 0.16, 0.22), 0.085, c.wood);
        b.beam(
            v(0.61, 0.28, -0.44),
            v(-0.39, 0.14, 0.68),
            0.093,
            shift(c.wood, -12),
        );
        b.face(
            &[
                v(-0.42, 0.49, -0.53),
                v(0.39, 0.19, -0.35),
                v(0.16, 0.14, 0.38),
                v(-0.53, 0.23, 0.17),
            ],
            c.roof,
        );
    } else {
        for x in [-0.71, 0.71] {
            for z in [-0.48, 0.48] {
                let at = v(x, 0.09, z);
                let timber = matches!(region, Region::Japan | Region::Nordic);
                b.frustum(
                    at,
                    0.114,
                    0.084,
                    0.91,
                    if timber { 4 } else { 8 },
                    if timber { c.wood } else { c.wall },
                );
                b.slab(at + Vec3::Y * 0.083, v(0.214, 0.085, 0.21), c.wall);
                b.slab(at + Vec3::Y * 0.91, v(0.22, 0.08, 0.19), shift(c.wall, -4));
            }
        }
        b.slab(v(0.0, 1.025, 0.0), v(1.70, 0.118, 1.16), c.wood);
        let peak = match region {
            Region::Nordic => 1.62,
            Region::Japan => 1.49,
            _ => 1.40,
        };
        for z in [-1.0, 1.0] {
            let tiles = [
                v(-0.969, peak, 0.0),
                v(-0.969, 1.062, z * 0.67),
                v(0.969, 1.062, z * 0.67),
                v(0.969, peak, 0.0),
            ];
            b.face(
                &if z > 0.0 {
                    tiles
                } else {
                    [tiles[3], tiles[2], tiles[1], tiles[0]]
                },
                c.roof,
            );
            for i in 0..9 {
                let x = i as f32 * 0.204 - 0.82;
                b.beam(
                    v(x, peak + 0.017, 0.0),
                    v(x, 1.092, z * 0.68),
                    0.026,
                    shift(c.roof, 12 + (i % 3) * 4),
                );
            }
        }
        b.beam(
            v(-1.00, peak + 0.022, 0.0),
            v(1.00, peak + 0.022, 0.0),
            0.067,
            shift(c.roof, 23),
        );
        for x in [-0.82, 0.82] {
            b.face(
                &[v(x, 1.060, -0.64), v(x, peak, 0.0), v(x, 1.060, 0.64)],
                c.wall,
            );
        }
        if region == Region::Oasis {
            for (radius, end, y, h) in [
                (0.23, 0.25, peak, 0.07),
                (0.25, 0.18, peak + 0.07, 0.13),
                (0.18, 0.024, peak + 0.20, 0.13),
            ] {
                b.frustum(v(0.0, y, 0.0), radius, end, h, 10, shift(c.roof, 29));
            }
        }
        b.slab(v(-0.40, 0.29, -0.36), v(0.46, 0.065, 0.29), c.wood);
        amphora(&mut b, v(0.49, 0.092, -0.41), 0.67, c.cloth);
    }
    b.mesh()
}

fn meeting(region: Region, event: Event) -> Mesh {
    let c = colors(region);
    let mut b = Batch::default();
    ground(&mut b, c);
    match event {
        Event::Fever => {
            b.slab(v(0.0, 0.232, 0.01), v(0.55, 0.12, 0.41), c.wood);
            b.slab(v(0.0, 0.301, 0.01), v(0.52, 0.055, 0.27), c.cloth);
            for x in [-0.23, 0.23] {
                b.slab(v(x, 0.15, 0.0), v(0.075, 0.23, 0.34), c.wood);
            }
            amphora(&mut b, v(-0.54, 0.085, 0.18), 0.53, [127, 158, 118]);
        }
        Event::Relic => {
            for (r, end, h, y) in [
                (0.42, 0.38, 0.13, 0.09),
                (0.27, 0.24, 0.26, 0.22),
                (0.34, 0.34, 0.09, 0.48),
            ] {
                b.frustum(v(0.0, y, -0.20), r, end, h, 10, c.wall);
            }
            b.gem(v(0.0, 0.63, -0.20), 0.099, 0.082, 0.09, 0.87, c.cloth);
        }
        Event::Festival | Event::Arrival => {
            for x in [-0.55, 0.55] {
                b.slab(v(x, 0.29, -0.11), v(0.48, 0.44, 0.36), c.wood);
                b.slab(v(x, 0.53, -0.11), v(0.60, 0.095, 0.46), shift(c.wood, 12));
                if event == Event::Festival {
                    b.slab(v(x, 0.583, -0.12), v(0.25, 0.025, 0.45), c.cloth);
                    for z in [-0.18, 0.09] {
                        b.frustum(v(x, 0.606, z), 0.09, 0.106, 0.036, 9, shift(c.wall, 27));
                        b.gem(v(x, 0.653, z), 0.071, 0.026, 0.073, 0.48, [209, 151, 76]);
                    }
                } else {
                    crate_box(
                        &mut b,
                        v(x, 0.67, -0.12),
                        v(0.26, 0.185, 0.28),
                        shift(c.wood, 14),
                    );
                }
            }
        }
        _ => {}
    }
    b.mesh()
}

fn art_part(
    root: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    handle: &Handle<StandardMaterial>,
    mesh: Mesh,
    at: Vec3,
    motion: Motion,
) {
    root.spawn((
        motion,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(handle.clone()),
        Transform::from_translation(at),
        NotShadowCaster,
    ));
}

fn flags(
    root: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    art: &Art,
    region: Region,
    span: f32,
) {
    let c = colors(region);
    let mut structure = Batch::default();
    for x in [-span, span] {
        structure.frustum(v(x, 0.01, 0.0), 0.072, 0.043, 2.12, 7, c.wood);
        structure.gem(v(x, 2.14, 0.0), 0.065, 0.081, 0.062, 0.15, [226, 182, 104]);
    }
    for i in 0..10 {
        let from = i as f32 / 10.0;
        let to = (i + 1) as f32 / 10.0;
        let height = |f: f32| 2.09 - 0.33 * (f * PI).sin();
        let a = v((from * 2.0 - 1.0) * span, height(from), 0.0);
        let b = v((to * 2.0 - 1.0) * span, height(to), 0.0);
        structure.beam(a, b, 0.016, [109, 77, 51]);
        let mut pennant = Batch::default();
        let width = (b.x - a.x) * 0.43;
        let tint = [c.cloth, c.roof, [231, 176, 90], [239, 212, 163]][i % 4];
        let left = v(-width, 0.0, 0.0);
        let right = v(width, 0.004, 0.0);
        let tip = v(0.023, -0.232, 0.014);
        pennant.face(&[left, tip, right], tint);
        pennant.face(&[right, tip, left], shift(tint, -10));
        let at = (a + b) * 0.5;
        art_part(
            root,
            meshes,
            &art.matte,
            pennant.mesh(),
            at,
            Motion::Banner {
                wave: i as f32 * 0.84,
                at,
            },
        );
    }
    root.spawn((
        Mesh3d(meshes.add(structure.mesh())),
        MeshMaterial3d(art.matte.clone()),
        Transform::default(),
    ));
}

pub(crate) fn sync(
    mut commands: Commands,
    game: Res<Game>,
    adventure: Res<Adventure>,
    art: Res<Art>,
    cast: Res<Cast>,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let state = adventure
        .terrain
        .as_ref()
        .zip(adventure.active())
        .filter(|_| game.view == View::Adventure && game.panel != Panel::Wardrobe);
    let signature = state.map(|(_, island)| Signature {
        level: island.index,
        statuses: island.encounters.visits.map(|visit| visit.status),
        sources: std::array::from_fn(|slot| {
            adventure.target_position(Target::EncounterSource(slot))
        }),
    });
    if scene.signature == signature {
        return;
    }
    scene.signature = signature;
    if let Some(root) = scene.root.take() {
        commands.entity(root).despawn();
    }
    let Some((map, island)) = state else { return };
    let region = map.level.region;
    scene.root = Some(
        commands
            .spawn((Transform::default(), Visibility::Inherited))
            .with_children(|top| {
                for slot in 0..map.level.encounter_count() {
                    let spec = island.encounter(map.level, slot).unwrap();
                    let status = island.encounters.visits[slot].status;
                    if status == EncounterStatus::Hidden {
                        continue;
                    }
                    let at = map.events[slot];
                    top.spawn((
                        Transform::from_translation(position(map, at)),
                        Visibility::Inherited,
                    ))
                    .with_children(|root| {
                        let damaged =
                            status != EncounterStatus::Finished && spec.event == Event::Ruin;
                        let base = if matches!(spec.event, Event::Ruin | Event::Fire) {
                            shelter(region, damaged)
                        } else {
                            meeting(region, spec.event)
                        };
                        root.spawn((
                            Mesh3d(meshes.add(base)),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::default(),
                        ));
                        if spec.event == Event::Ruin && status == EncounterStatus::Working {
                            let c = colors(region);
                            let mut frame = Batch::default();
                            for x in [-0.74, 0.74] {
                                frame.beam(v(x, 0.10, 0.48), v(x, 1.30, 0.48), 0.076, c.wood);
                                frame.beam(v(x, 0.10, -0.48), v(x, 1.30, -0.48), 0.076, c.wood);
                                frame.beam(v(x, 0.64, -0.48), v(x, 1.08, 0.49), 0.056, c.wood);
                            }
                            art_part(
                                root,
                                &mut meshes,
                                &art.matte,
                                frame.mesh(),
                                Vec3::ZERO,
                                Motion::Frame { slot },
                            );
                        }
                        if spec.event == Event::Fire && status != EncounterStatus::Finished {
                            for index in 0..3 {
                                let mut flame = Batch::default();
                                flame.gem(
                                    Vec3::ZERO,
                                    0.22,
                                    0.35,
                                    0.17,
                                    index as f32,
                                    [233, 105, 46],
                                );
                                flame.gem(
                                    v(0.0, -0.10, 0.04),
                                    0.14,
                                    0.21,
                                    0.12,
                                    0.3,
                                    [253, 179, 71],
                                );
                                art_part(
                                    root,
                                    &mut meshes,
                                    &art.gold,
                                    flame.mesh(),
                                    Vec3::ZERO,
                                    Motion::Fire { index, slot },
                                );
                            }
                            for index in 0..4 {
                                let mut smoke = Batch::default();
                                smoke.gem(
                                    Vec3::ZERO,
                                    0.21,
                                    0.25,
                                    0.19,
                                    index as f32 * 0.53,
                                    [122, 113, 105],
                                );
                                smoke.gem(
                                    v(0.09, -0.041, 0.035),
                                    0.155,
                                    0.14,
                                    0.12,
                                    0.81,
                                    [152, 141, 124],
                                );
                                art_part(
                                    root,
                                    &mut meshes,
                                    &art.matte,
                                    smoke.mesh(),
                                    Vec3::ZERO,
                                    Motion::Smoke { index, slot },
                                );
                            }
                            if status == EncounterStatus::Working {
                                for index in 0..5 {
                                    let mut drop = Batch::default();
                                    drop.gem(Vec3::ZERO, 0.055, 0.09, 0.055, 0.19, [84, 192, 213]);
                                    art_part(
                                        root,
                                        &mut meshes,
                                        &art.matte,
                                        drop.mesh(),
                                        Vec3::ZERO,
                                        Motion::Drops { index },
                                    );
                                }
                            }
                        }
                        if spec.event == Event::Festival {
                            flags(root, &mut meshes, &art, region, 1.68);
                        }
                        let variant = (slot * 3 + map.level.stage + region as usize) % 8;
                        let waiting =
                            spec.event == Event::Fever && status != EncounterStatus::Finished;
                        let start = v(-0.56, 0.09, 0.94);
                        let end = if waiting { v(0.0, 0.08, 0.10) } else { start };
                        let arrival = (spec.event == Event::Arrival
                            && status == EncounterStatus::Offered)
                            .then_some((start + v(-1.72, 0.0, 0.20), end));
                        let mut npc = root.spawn((
                            Visitor {
                                variant,
                                slot,
                                gesture: if waiting {
                                    StationGesture::Seated(0.22)
                                } else {
                                    StationGesture::Lift
                                },
                                arrival,
                            },
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(end)
                                .with_rotation(Quat::from_rotation_y(0.38)),
                        ));
                        people::worker_rig(&mut npc, &cast, variant, region);
                    });
                    if spec.event == Event::Festival {
                        let decoration = Vec2::new(0.14, 4.55);
                        top.spawn((
                            Transform::from_translation(position(map, decoration)),
                            Visibility::Inherited,
                        ))
                        .with_children(|root| flags(root, &mut meshes, &art, region, 2.18));
                    }
                    if spec.event == Event::Relic && status == EncounterStatus::Seeking {
                        let point = map.finds[slot];
                        top.spawn((
                            Transform::from_translation(position(map, point)),
                            Visibility::Inherited,
                        ))
                        .with_children(|root| {
                            let mut marker = Batch::default();
                            marker.frustum(Vec3::ZERO, 0.31, 0.26, 0.14, 9, colors(region).wall);
                            marker.gem(
                                v(0.0, 0.29, 0.0),
                                0.104,
                                0.15,
                                0.087,
                                slot as f32,
                                [236, 187, 84],
                            );
                            art_part(
                                root,
                                &mut meshes,
                                &art.matte,
                                marker.mesh(),
                                Vec3::ZERO,
                                Motion::Shine { at: Vec3::ZERO },
                            );
                        });
                    }
                }
            })
            .id(),
    );
}

type Effects<'w, 's> = Query<
    'w,
    's,
    (&'static Motion, &'static mut Transform),
    (Without<WorkerJoint>, Without<Visitor>),
>;
type People<'w, 's> = Query<
    'w,
    's,
    (
        &'static Visitor,
        &'static SkinnedMesh,
        &'static mut Transform,
    ),
    (Without<WorkerJoint>, Without<Motion>),
>;

#[derive(SystemParam)]
pub(crate) struct Animation<'w, 's> {
    pieces: Effects<'w, 's>,
    visitors: People<'w, 's>,
    joints: Query<'w, 's, &'static mut Transform, With<WorkerJoint>>,
}

pub(crate) fn animate(
    game: Res<Game>,
    adventure: Res<Adventure>,
    cast: Res<Cast>,
    mut animation: Animation,
) {
    let Some((map, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    if game.view != View::Adventure {
        return;
    }
    let seconds = game.clock;
    for (piece, mut t) in &mut animation.pieces {
        let active = |slot: usize| {
            if island.encounters.visits[slot].status == EncounterStatus::Working {
                1.0 - island
                    .encounters
                    .progress(slot, island.encounter(map.level, slot).unwrap())
            } else {
                1.0
            }
        };
        match *piece {
            Motion::Fire { index, slot } => {
                let pulse = (seconds * 9.4 + f32::from(index) * 1.86).sin();
                t.translation = v(
                    -0.25 + f32::from(index) * 0.25,
                    0.56 + f32::from(index) * 0.16 + pulse * 0.045,
                    0.42,
                );
                t.scale =
                    v(0.78 + pulse * 0.14, 1.05 + pulse * 0.17, 0.83) * active(slot).max(0.01);
                t.rotation = Quat::from_rotation_z(pulse * 0.17);
            }
            Motion::Smoke { index, slot } => {
                let f = (seconds * 0.63 + f32::from(index) * 0.25).fract();
                t.translation = v(0.08 + f * f * 0.53, 0.83 + f * 1.63, 0.48 - f * 0.19);
                t.scale = Vec3::splat(
                    active(slot).max(0.01) * (0.73 + f * 0.91) * (f * PI).sin().max(0.01),
                );
            }
            Motion::Drops { index } => {
                let f = (seconds * 1.9 + f32::from(index) * 0.19).fract();
                t.translation = v(
                    -0.80 + f * 0.93,
                    0.63 + (f * PI).sin() * 0.70,
                    0.98 - f * 0.52,
                );
                t.scale = Vec3::splat((f * PI).sin().max(0.02));
            }
            Motion::Banner { wave, at } => {
                t.translation = at;
                t.rotation = Quat::from_rotation_x((seconds * 3.8 + wave).sin() * 0.42);
            }
            Motion::Shine { at } => {
                t.translation = at + Vec3::Y * (0.026 + (seconds * 2.2).sin() * 0.022);
                t.rotation = Quat::from_rotation_y((seconds * 0.51).rem_euclid(TAU));
            }
            Motion::Frame { slot } => {
                let lift = island
                    .encounters
                    .progress(slot, island.encounter(map.level, slot).unwrap());
                t.scale = v(1.0, lift.max(0.05), 1.0);
            }
        }
    }
    for (actor, skin, mut t) in &mut animation.visitors {
        let pose = if let Some((from, to)) = actor.arrival {
            let delay =
                (island.encounters.elapsed - island.encounters.visits[actor.slot].since).max(0.0);
            let step = (delay / 2.6).min(1.0);
            t.translation = from.lerp(to, step);
            people::worker_pose(
                &cast,
                actor.variant,
                map.level.region,
                delay * 0.70,
                f32::from(step < 1.0),
                seconds,
                false,
            )
        } else {
            people::station_pose(
                &cast,
                actor.variant,
                map.level.region,
                actor.gesture,
                seconds + actor.slot as f32 * 0.71,
            )
        };
        for (&bone, transform) in skin.joints.iter().zip(pose) {
            if let Ok(mut joint) = animation.joints.get_mut(bone) {
                *joint = transform;
            }
        }
    }
}

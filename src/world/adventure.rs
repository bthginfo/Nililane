use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{camera::ScalingMode, mesh::skinning::SkinnedMesh},
    window::PrimaryWindow,
};

use crate::{
    adventure::{Adventure, Build, Good, Quest, Screen, Task, Terrain, terrain::unit},
    campaign::Region,
    game::{Game, Panel, View},
    model::Kind,
};

use super::{
    Art, Hippo, Limb, StreetCamera, Sun,
    architecture::{gable, panel_roof, plinth},
    buildings::{amphora, flowerpot},
    mesh::{Batch, Tint, shift, v},
    upgrades::{crate_box, rail},
};

const ORIGIN: Vec3 = Vec3::new(236.0, 0.0, 0.0);

#[derive(Component)]
pub(crate) struct Ground;

#[derive(Component)]
pub(crate) struct TownBuildings;

#[derive(Component)]
pub(crate) struct TownWorkers;

#[derive(Component)]
pub(crate) struct Worker(usize);

#[derive(Component)]
pub(crate) struct Marker;

#[derive(Component)]
pub(crate) struct Relic;

#[derive(Component)]
pub(crate) struct CivicLight;

#[derive(Component)]
pub(crate) struct BuildLift {
    when: f32,
    base: f32,
}

#[derive(Resource, Default)]
pub(crate) struct Scene {
    chapter: Option<usize>,
    buildings: Vec<Option<crate::adventure::Property>>,
    staff: usize,
    workshops: Vec<(Region, Build, u8, Handle<Mesh>)>,
}

pub(crate) fn position(terrain: &Terrain, at: Vec2) -> Vec3 {
    ORIGIN + v(at.x, terrain.height(at), at.y)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync(
    mut commands: Commands,
    game: Res<Game>,
    adventure: Res<Adventure>,
    mut art: ResMut<Art>,
    cast: Res<super::people::Cast>,
    assets: Res<AssetServer>,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ground: Query<Entity, With<Ground>>,
    buildings: Query<Entity, With<TownBuildings>>,
    workers: Query<Entity, With<TownWorkers>>,
    marker: Query<Entity, With<Marker>>,
) {
    let terrain = adventure
        .terrain
        .as_ref()
        .filter(|_| game.view == View::Adventure);
    let island = adventure.active();
    let chapter = terrain.map(|t| t.level.index);
    let switched = chapter != scene.chapter;
    if switched {
        for entity in ground.iter().chain(marker.iter()) {
            commands.entity(entity).despawn();
        }
        scene.chapter = chapter;
        scene.buildings.clear();
        scene.staff = 0;
        if let Some(terrain) = terrain {
            let earth = palette(terrain.level.region);
            let mesh = landscape(terrain, earth);
            let region = terrain.level.region;
            commands
                .spawn((
                    Ground,
                    Transform::from_translation(ORIGIN),
                    Visibility::Inherited,
                ))
                .with_children(|root| {
                    root.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(art.matte.clone())));
                    let water = materials.add(StandardMaterial {
                        base_color: Color::srgb_u8(59, 142, 159),
                        perceptual_roughness: 0.42,
                        metallic: 0.08,
                        ..default()
                    });
                    root.spawn((
                        Mesh3d(meshes.add(sea(terrain.radius))),
                        MeshMaterial3d(water),
                        NotShadowCaster,
                    ));
                    for patch in woods(terrain, earth) {
                        root.spawn((Mesh3d(meshes.add(patch)), MeshMaterial3d(art.matte.clone())));
                    }
                    let hall = civic(region);
                    root.spawn((
                        Mesh3d(meshes.add(hall)),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_xyz(0.0, terrain.height(Vec2::ZERO), 0.0)
                            .with_scale(Vec3::splat(1.4)),
                    ));
                    root.spawn((
                        CivicLight,
                        PointLight {
                            color: Color::srgb_u8(255, 192, 115),
                            intensity: 0.0,
                            range: 6.2,
                            radius: 0.45,
                            ..default()
                        },
                        Transform::from_xyz(0.0, terrain.height(Vec2::ZERO) + 1.6, 1.8),
                    ));
                    for deposit in terrain.deposits.iter() {
                        let p = deposit.at;
                        let at = v(p.x, terrain.height(p), p.y);
                        let source = match deposit.kind {
                            Good::Wood if region != Region::Oasis => Some(("wood_tree", 0.77)),
                            Good::Stone => Some(("resource_rock", 0.94)),
                            Good::Ore if matches!(region, Region::Greece | Region::Oasis) => {
                                Some(("resource_copper", 0.77))
                            }
                            Good::Ore => Some(("resource_crystal", 0.71)),
                            _ => None,
                        };
                        if let Some((filename, scale)) = source {
                            let path = format!("embedded://models/waldwinkel/{filename}.glb");
                            root.spawn((
                                SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(path))),
                                Transform::from_translation(at).with_scale(Vec3::splat(scale)),
                            ));
                        }
                        root.spawn((
                            Mesh3d(meshes.add(deposit_model(region, deposit.kind))),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(at),
                        ));
                    }
                    if terrain.level.plan().is_some() {
                        let region = terrain.level.region;
                        let marker_kind = match region {
                            Region::Greece => Kind::Shrine,
                            Region::Rome => Kind::Stoa,
                            Region::Japan => Kind::Temple,
                            Region::Nordic => Kind::FeastHall,
                            Region::Oasis => Kind::Cistern,
                        };
                        let place = terrain.shrine;
                        root.spawn((
                            Mesh3d(meshes.add(super::model(region, marker_kind, 3))),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_xyz(place.x, terrain.height(place), place.y)
                                .with_scale(Vec3::splat(1.18)),
                        ));
                        if terrain.level.expedition() {
                            let find = terrain.relic;
                            let mut altar = Batch::default();
                            altar.frustum(Vec3::ZERO, 0.47, 0.44, 0.40, 9, earth.rock);
                            altar.gem(v(0.0, 0.72, 0.0), 0.25, 0.40, 0.23, 0.4, earth.flag);
                            altar.gem(v(0.0, 0.62, 0.11), 0.14, 0.20, 0.16, 0.1, [252, 202, 105]);
                            root.spawn((
                                Relic,
                                Mesh3d(meshes.add(altar.mesh())),
                                MeshMaterial3d(art.matte.clone()),
                                Transform::from_xyz(find.x, terrain.height(find), find.y),
                            ));
                        }
                    }
                });
            let mut sign = Batch::default();
            let color = [255, 197, 96];
            for side in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let at = v(side * 1.52, 0.056, z * 1.33);
                    sign.beam(at, at - v(side * 0.29, 0.0, 0.0), 0.086, color);
                    sign.beam(at, at - v(0.0, 0.0, z * 0.29), 0.086, color);
                }
            }
            sign.gem(v(0.0, 1.98, 0.15), 0.13, 0.19, 0.11, 0.3, color);
            commands.spawn((
                Marker,
                Mesh3d(meshes.add(sign.mesh())),
                MeshMaterial3d(art.gold.clone()),
                Transform::default(),
                Visibility::Hidden,
                NotShadowCaster,
            ));
        }
    }
    let Some((terrain, island)) = terrain.zip(island) else {
        for entity in buildings.iter().chain(workers.iter()) {
            commands.entity(entity).despawn();
        }
        return;
    };
    if scene.buildings != island.lots {
        scene.buildings.clone_from(&island.lots);
        for entity in &buildings {
            commands.entity(entity).despawn();
        }
        let region = terrain.level.region;
        commands
            .spawn((TownBuildings, Transform::default(), Visibility::Inherited))
            .with_children(|root| {
                let lantern = art.doorway[region as usize].clone();
                for (lot, at) in terrain.plots.iter().copied().enumerate() {
                    let p = position(terrain, at);
                    if let Some(property) = island.lots[lot] {
                        let handle = match property.kind {
                            Build::Harbor(kind) => super::building_mesh(
                                &mut art,
                                &mut meshes,
                                region,
                                kind,
                                property.tier,
                            ),
                            kind => {
                                if let Some(entry) = scene.workshops.iter().find(|entry| {
                                    (entry.0, entry.1, entry.2) == (region, kind, property.tier)
                                }) {
                                    entry.3.clone()
                                } else {
                                    let handle = meshes.add(workshop(region, kind, property.tier));
                                    scene.workshops.push((
                                        region,
                                        kind,
                                        property.tier,
                                        handle.clone(),
                                    ));
                                    handle
                                }
                            }
                        };
                        let mut spot = root.spawn((
                            Mesh3d(handle),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(p),
                        ));
                        if let Some((_, when)) = adventure
                            .scene_change
                            .filter(|(i, when)| *i == lot && game.clock - *when < 1.2)
                        {
                            spot.insert(BuildLift { when, base: p.y });
                        }
                        spot.with_children(|model| {
                            if property.kind == Build::Harbor(Kind::House)
                                && region == Region::Greece
                            {
                                model.spawn((
                                    SceneRoot(art.house.clone()),
                                    super::buildings::house_transform(property.tier),
                                ));
                            }
                            if property.kind == Build::Harbor(Kind::Temple) {
                                model.spawn((
                                    Mesh3d(art.temple_bell.clone()),
                                    MeshMaterial3d(art.matte.clone()),
                                    Transform::from_translation(super::japan::BELL_HOOK),
                                ));
                            }
                            model.spawn((
                                Mesh3d(lantern.clone()),
                                MeshMaterial3d(art.lantern_material.clone()),
                                Transform::from_xyz(1.34, 0.08, 1.27),
                                NotShadowCaster,
                            ));
                        });
                    }
                }
            });
    }
    let population = usize::from(island.staff());
    if switched || scene.staff != population {
        scene.staff = population;
        for entity in &workers {
            commands.entity(entity).despawn();
        }
        commands
            .spawn((TownWorkers, Transform::default(), Visibility::Inherited))
            .with_children(|parent| {
                for index in 0..population {
                    let mut citizen = parent.spawn((
                        Worker(index),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::default(),
                    ));
                    super::people::worker_rig(&mut citizen, &cast, index, terrain.level.region);
                }
            });
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn animate(
    game: Res<Game>,
    adventure: Res<Adventure>,
    cast: Res<super::people::Cast>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut clear: ResMut<ClearColor>,
    mut civic_lights: Query<&mut PointLight, With<CivicLight>>,
    mut sun: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut scene: ParamSet<(
        Query<(&mut Transform, &mut Projection), (With<StreetCamera>, Without<Sun>)>,
        Query<&mut Transform, (With<Hippo>, Without<Sun>)>,
        Query<(&Limb, &mut Transform), Without<Sun>>,
        Query<(&Marker, &mut Transform, &mut Visibility), Without<Sun>>,
        Query<&mut Visibility, With<Relic>>,
        Query<(&Worker, &SkinnedMesh, &mut Transform), Without<Sun>>,
        Query<(&BuildLift, &mut Transform), Without<Sun>>,
        Query<&mut Transform, (With<super::people::WorkerJoint>, Without<Sun>)>,
    )>,
) {
    if game.view != View::Adventure {
        return;
    }
    let Some((terrain, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    let p = Vec2::from_array(island.position);
    let at = position(terrain, p);
    let phase = island.phase * std::f32::consts::TAU;
    let night = ((phase.cos() + 0.09) * 0.79).clamp(0.0, 0.86);
    if let Ok((mut camera, mut lens)) = scene.p0().single_mut() {
        let aspect = window.width() / window.height().max(1.0);
        let width = if game.panel == Panel::Wardrobe {
            4.9
        } else if adventure.screen == Screen::Map {
            44.0
        } else {
            (23.1 - adventure.map_zoom * 14.2) * (1.52 / aspect.max(1.1)).sqrt()
        };
        if let Projection::Orthographic(lens) = lens.as_mut() {
            lens.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: width,
            };
            lens.far = 260.0;
        }
        let look = v(0.18, 0.87, 1.32).normalize();
        let center = if game.panel == Panel::Wardrobe {
            at + v(-1.6, 0.67, 0.3)
        } else {
            at + v(1.48, 0.77, -0.23)
        };
        let eye = center + look * 44.0;
        let blend = if (camera.translation - eye).length() > 120.0 {
            1.0
        } else {
            1.0 - (-6.7 * time.delta_secs()).exp()
        };
        camera.translation = camera.translation.lerp(eye, blend);
        camera.rotation = camera.rotation.slerp(
            Transform::from_translation(eye)
                .looking_at(center, Vec3::Y)
                .rotation,
            blend,
        );
    }
    let mut character = scene.p1();
    if let Ok(mut nili) = character.single_mut() {
        let bob = if adventure.moved {
            adventure.gait.sin().abs() * 0.051
        } else {
            0.0
        };
        nili.translation = at + Vec3::Y * (0.026 + bob);
        nili.rotation = Quat::from_rotation_y(adventure.heading);
        nili.scale = Vec3::splat(0.94);
    }
    for (limb, mut transform) in &mut scene.p2() {
        let movement = if adventure.moved {
            (adventure.gait + limb.side * 1.16).sin() * if limb.leg { 0.54 } else { 0.38 }
        } else if !limb.leg {
            -0.44 * (adventure.work_swing * 7.1).sin()
        } else {
            0.0
        };
        transform.rotation = Quat::from_rotation_x(movement);
    }
    let current = if adventure.screen == Screen::Walk {
        adventure
            .waypoint
            .or_else(|| adventure.tutorial().map(|(_, _, target)| target))
            .or(adventure.target)
    } else {
        None
    };
    for (_, mut marker, mut visibility) in &mut scene.p3() {
        let point = current.and_then(|target| adventure.target_position(target));
        *visibility = if let Some(point) = point {
            marker.translation = position(terrain, point);
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for mut visibility in &mut scene.p4() {
        *visibility = if matches!(island.quest, Quest::Unheard | Quest::Searching) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    let homes: Vec<_> = island
        .lots
        .iter()
        .enumerate()
        .filter_map(|(lot, property)| {
            let property = (*property)?;
            let Build::Harbor(kind) = property.kind else {
                return None;
            };
            (crate::model::Building::new(kind).residents() > 0).then_some(terrain.plots[lot])
        })
        .collect();
    let mut poses = Vec::with_capacity(usize::from(island.staff()) * 9);
    for (worker, skin, mut model) in &mut scene.p5() {
        let job = island.workers.get(worker.0).copied();
        let (spot, center) = match job {
            Some(Task::Gather(node)) => {
                let deposit = terrain.deposits[node].at;
                let inward = -deposit.normalize_or_zero();
                (deposit + inward * 0.84, deposit + inward * 3.79)
            }
            Some(Task::Produce(lot)) => (
                terrain.plots[lot] + Vec2::new(1.42, 1.61),
                terrain.plots[lot] + Vec2::new(-1.39, 2.65),
            ),
            None => {
                let Some(&home) = homes.get(worker.0 % homes.len().max(1)) else {
                    continue;
                };
                let sidewalk = home - Vec2::Y * home.y.signum() * 1.82;
                (
                    sidewalk + Vec2::new(-1.14, -0.14),
                    sidewalk + Vec2::new(1.19, -0.31),
                )
            }
        };
        let elapsed = game.clock + worker.0 as f32 * 2.5;
        let cycle = elapsed.rem_euclid(7.0);
        let busy = (2.0..4.8).contains(&cycle);
        let moving = if busy { 0.0 } else { 1.0 };
        let blend = if cycle < 2.0 {
            cycle / 2.0
        } else {
            ((7.0 - cycle) / 2.2).min(1.0)
        };
        let here = center.lerp(spot, blend);
        let facing = if cycle < 4.8 {
            spot - center
        } else {
            center - spot
        };
        model.translation = position(terrain, here) + Vec3::Y * 0.028;
        model.rotation = Quat::from_rotation_y(facing.x.atan2(facing.y));
        model.scale = Vec3::splat(1.18);
        let stride = elapsed * (spot - center).length() * 0.48;
        let pose = super::people::worker_pose(
            &cast,
            worker.0,
            terrain.level.region,
            stride,
            moving,
            elapsed,
            busy && job.is_some(),
        );
        poses.extend(skin.joints.iter().copied().zip(pose));
    }
    for (entity, pose) in poses {
        if let Ok(mut transform) = scene.p7().get_mut(entity) {
            *transform = pose;
        }
    }
    for (lift, mut building) in &mut scene.p6() {
        let a = ((game.clock - lift.when - 0.10) / 0.66).clamp(0.0, 1.0);
        let ease = 1.0 - (1.0 - a).powi(3);
        building.scale.y = 0.075 + ease * 0.925;
        building.translation.y = lift.base - (1.0 - ease) * 0.10;
    }
    if let Ok((mut sun, mut light)) = sun.single_mut() {
        sun.rotation = Quat::from_euler(EulerRot::YXZ, -0.65 + phase.sin() * 0.38, -0.81, 0.0);
        light.illuminance = 12_800.0 - night * 10_300.0;
        light.color = Color::srgb(1.0 - night * 0.36, 0.93 - night * 0.18, 0.79 + night * 0.14);
    }
    ambient.brightness = 503.0 - night * 125.0;
    ambient.color = Color::srgb(0.94 - night * 0.23, 0.93 - night * 0.11, 1.0);
    clear.0 = Color::srgb(
        0.40 - night * 0.24,
        0.61 - night * 0.32,
        0.67 - night * 0.21,
    );
    for mut lamp in &mut civic_lights {
        lamp.intensity = night * 9800.0;
    }
}

#[derive(Clone, Copy)]
struct Palette {
    turf: Tint,
    coast: Tint,
    rock: Tint,
    wood: Tint,
    leaves: Tint,
    flag: Tint,
}

fn palette(region: Region) -> Palette {
    let (turf, coast, rock, wood, leaves, flag) = match region {
        Region::Greece => (
            [143, 164, 104],
            [223, 190, 135],
            [157, 151, 131],
            [123, 88, 53],
            [107, 138, 74],
            [55, 128, 147],
        ),
        Region::Rome => (
            [146, 158, 104],
            [206, 171, 125],
            [156, 133, 115],
            [130, 86, 63],
            [99, 134, 80],
            [167, 76, 57],
        ),
        Region::Japan => (
            [119, 156, 114],
            [201, 177, 134],
            [124, 138, 137],
            [110, 76, 57],
            [98, 143, 102],
            [183, 81, 66],
        ),
        Region::Nordic => (
            [115, 144, 116],
            [181, 177, 139],
            [129, 143, 145],
            [115, 81, 56],
            [76, 111, 94],
            [68, 108, 140],
        ),
        Region::Oasis => (
            [171, 156, 98],
            [231, 189, 133],
            [182, 129, 96],
            [142, 92, 61],
            [97, 132, 80],
            [54, 143, 130],
        ),
    };
    Palette {
        turf,
        coast,
        rock,
        wood,
        leaves,
        flag,
    }
}

fn landscape(terrain: &Terrain, colors: Palette) -> Mesh {
    let mut b = Batch::default();
    let step = if terrain.level.stage < 5 { 1.68 } else { 2.01 };
    let span = (terrain.radius * 1.18 / step).ceil() as i32;
    for row in -span..span {
        for col in -span..span {
            let at = Vec2::new(col as f32, row as f32) * step;
            if terrain.shore(at) < -1.6 * step {
                continue;
            }
            let corner = |dx, dz| {
                let x = (col + dx + 112) as u32;
                let z = (row + dz + 112) as u32;
                let seed = x * 2543 + z * 1531 + terrain.level.index as u32 * 121;
                let p = at
                    + (Vec2::new(dx as f32, dz as f32)
                        + (Vec2::new(unit(seed), unit(seed + 304)) - Vec2::splat(0.5)) * 0.47)
                        * step;
                v(p.x, terrain.height(p), p.y)
            };
            let [a, c, d, e] = [corner(0, 0), corner(1, 0), corner(1, 1), corner(0, 1)];
            let edge = terrain.shore(at);
            let height = (a.y + d.y) * 0.5;
            let t = unit(
                (row + 92) as u32 * 2389
                    + (col + 92) as u32 * 337
                    + terrain.level.index as u32 * 57,
            );
            let shore = edge < 4.2 + t * 2.6;
            let stone = height > 2.8 + t * 1.5;
            let snow = terrain.level.region == Region::Nordic && height > 6.7 + t * 2.8;
            let base = if snow {
                [216, 224, 219]
            } else if stone {
                colors.rock
            } else if shore {
                colors.coast
            } else {
                colors.turf
            };
            let sweep = (at.x * 0.23 + at.y * 0.14).sin() * (at.y * 0.18).cos();
            let shade = (sweep * 9.0 + (t - 0.50) * 13.0) as i16;
            b.face(&[a, e, d], shift(base, shade));
            b.face(&[a, d, c], shift(base, shade - 4));
        }
    }
    let dirt = shift(colors.coast, -9);
    // A connected yard links the plots to the council hall and keeps their margins level.
    let depth = terrain.plots.iter().fold(8.0_f32, |n, p| n.max(p.y.abs()));
    for z in -depth.ceil() as i32..=depth.ceil() as i32 {
        let path = Vec2::new(0.0, z as f32);
        b.slab(
            v(0.0, terrain.height(path) + 0.019, path.y),
            v(2.50, 0.047, 1.00),
            shift(dirt, z.rem_euclid(5) as i16 * 3 - 6),
        );
    }
    for row in [-5.7_f32, 6.4, -11.7, 12.5, -17.8, 18.6] {
        let extent = terrain
            .plots
            .iter()
            .filter(|p| (p.y - row).abs() < 0.05)
            .map(|p| p.x.abs())
            .fold(0.0_f32, f32::max);
        let z = row - row.signum() * 2.22;
        for x in -(extent as i32)..=extent as i32 {
            if extent < 1.0 || x.abs() < 2 {
                continue;
            }
            let p = Vec2::new(x as f32, z + (x as f32 * 0.34).sin() * 0.21);
            let y = terrain.height(p) + 0.024;
            b.slab(
                v(p.x, y, p.y),
                v(0.987, 0.039, 0.78),
                shift(dirt, x.rem_euclid(5) as i16 * 3 - 3),
            );
        }
    }
    for p in terrain.plots.iter().copied() {
        let y = terrain.height(p);
        plinth(
            &mut b,
            v(p.x, y + 0.008, p.y),
            3.20,
            2.77,
            0.091,
            shift(colors.coast, -9),
        );
        for border in [-1.29, 1.29] {
            b.slab(
                v(p.x, y + 0.091, p.y + border),
                v(2.86, 0.04, 0.067),
                shift(colors.rock, 5),
            );
        }
        for x in [-1.41, 1.41] {
            for z in [-1.21, 1.21] {
                b.slab(
                    v(p.x + x, y + 0.091, p.y + z),
                    v(0.16, 0.063, 0.28),
                    shift(colors.rock, 26),
                );
            }
        }
        let mid = (p.y / 2.0).trunc() * 2.0;
        for a in 0..4 {
            let x = p.x + a as f32 * 0.23 - 0.34;
            let z = if p.y > mid { p.y - 1.52 } else { p.y + 1.52 };
            b.slab(
                v(x, y + 0.03, z),
                v(0.216, 0.046, 0.32),
                shift(colors.coast, 11),
            );
        }
    }
    b.mesh()
}

fn sea(radius: f32) -> Mesh {
    let mut b = Batch::default();
    let span = (radius * 2.2 / 9.0).ceil() as i32;
    for row in -span..span {
        for col in -span..span {
            let p = v(col as f32 * 9.0, -1.18, row as f32 * 9.0);
            let t = unit((row + 100) as u32 * 29 + (col + 100) as u32 * 901);
            let color = shift([124, 193, 200], (t * 29.0) as i16 - 12);
            b.face(&[p, p + Vec3::Z * 9.0, p + v(9.0, 0.0, 9.0)], color);
            b.face(
                &[p, p + v(9.0, 0.0, 9.0), p + Vec3::X * 9.0],
                shift(color, -8),
            );
            if t > 0.48 {
                let rip = p + v(t * 7.0, 0.012, (1.0 - t) * 8.2);
                b.face(
                    &[
                        rip,
                        rip + v(-0.60, 0.0, 0.090),
                        rip + v(2.44, 0.0, 0.17),
                        rip + v(2.90, 0.0, 0.102),
                    ],
                    [152, 207, 202],
                );
            }
        }
    }
    b.mesh()
}

fn woods(terrain: &Terrain, colors: Palette) -> impl Iterator<Item = Mesh> {
    let mut patches: [Batch; 16] = std::array::from_fn(|_| Batch::default());
    let patch = |p: Vec2| {
        let tile = (p / terrain.radius * 2.0 + Vec2::splat(2.0))
            .floor()
            .clamp(Vec2::ZERO, Vec2::splat(3.0));
        tile.x as usize + tile.y as usize * 4
    };
    let fill = 360 + terrain.level.stage * 44;
    let region = terrain.level.region;
    let seed = terrain.level.index as u32 * 8763 + 431;
    for i in 0..fill as u32 {
        let x = (unit(seed + 531 * i) * 2.0 - 1.0) * terrain.radius;
        let z = (unit(seed + 711 * i + 6) * 2.0 - 1.0) * terrain.radius;
        let p = Vec2::new(x, z);
        if terrain.shore(p) < 3.3
            || p.length() < 8.0
            || terrain.plots.iter().any(|at| at.distance(p) < 3.15)
            || terrain.deposits.iter().any(|d| d.at.distance(p) < 1.75)
            || terrain.shrine.distance(p) < 4.0
            || terrain.relic.distance(p) < 2.2
        {
            continue;
        }
        let b = &mut patches[patch(p)];
        let height = terrain.height(p);
        let choice = unit(seed + i * 7 + 7);
        let size = 0.58 + unit(seed + i * 83 + 196) * 0.94;
        let at = v(x, height, z);
        if height > 3.4 || choice > 0.75 {
            b.gem(
                at + Vec3::Y * size * 0.14,
                size * 0.62,
                size * 0.37,
                size * 0.56,
                i as f32,
                shift(colors.rock, i.rem_euclid(23) as i16 - 9),
            );
        } else if choice < 0.44 {
            match region {
                Region::Oasis => super::oasis::palm(b, at, size, i as f32 * 0.61),
                Region::Greece if i % 3 == 0 => super::buildings::olive(b, at, size),
                Region::Rome if i % 3 == 0 => super::buildings::cypress(b, at, size),
                _ => conifer(
                    b,
                    at,
                    size,
                    colors,
                    region == Region::Nordic && height > 2.0,
                ),
            }
        } else {
            for j in 0..3 {
                b.gem(
                    at + v(j as f32 * 0.17, size * 0.18, (j % 2) as f32 * 0.23),
                    size * 0.27,
                    size * 0.26,
                    size * 0.24,
                    i as f32,
                    shift(colors.leaves, (i % 30) as i16 - 8),
                );
            }
        }
    }
    for i in 0..(680 + terrain.level.stage as u32 * 64) {
        let p = Vec2::new(
            unit(seed + i * 335 + 8203) * 2.0 - 1.0,
            unit(seed + i * 855 + 9272) * 2.0 - 1.0,
        ) * terrain.radius;
        if terrain.shore(p) < 3.2
            || p.x.abs() < 1.65
            || p.length() < 3.8
            || terrain.plots.iter().any(|plot| plot.distance(p) < 1.9)
        {
            continue;
        }
        let b = &mut patches[patch(p)];
        let variation = unit(seed + i * 117 + 2399);
        let height = 0.09 + variation * 0.17;
        let at = v(p.x, terrain.height(p) + 0.013, p.y);
        let tint = shift(colors.leaves, 14 + (variation * 30.0) as i16);
        for blade in 0..3 {
            let dx = blade as f32 * 0.089 - 0.092;
            b.face(
                &[
                    at + v(dx, 0.0, -0.035),
                    at + v(dx + 0.046, 0.0, 0.043),
                    at + v(dx * 1.7, height, 0.0),
                ],
                tint,
            );
        }
        if variation > 0.81 && region != Region::Nordic {
            b.gem(
                at + v(0.026, height * 0.78, 0.0),
                0.083,
                0.049,
                0.071,
                i as f32,
                colors.flag,
            );
        }
    }
    patches
        .into_iter()
        .map(Batch::mesh)
        .filter(|mesh| mesh.count_vertices() > 0)
}

fn conifer(b: &mut Batch, at: Vec3, s: f32, color: Palette, snow: bool) {
    b.frustum(at, s * 0.115, s * 0.062, s * 1.19, 7, color.wood);
    for (i, (y, r, h)) in [(0.66, 0.57, 0.86), (1.05, 0.44, 0.77), (1.42, 0.32, 0.77)]
        .into_iter()
        .enumerate()
    {
        b.frustum(
            at + Vec3::Y * s * y,
            r * s,
            0.012,
            h * s,
            7,
            shift(color.leaves, i as i16 * 9 - 11),
        );
        if snow {
            b.frustum(
                at + Vec3::Y * s * (y + h * 0.25),
                r * s * 0.77,
                0.015,
                h * s * 0.77,
                7,
                [205, 222, 212],
            );
        }
    }
}

fn deposit_model(region: Region, good: Good) -> Mesh {
    let mut b = Batch::default();
    let colors = palette(region);
    let at = v(0.0, 0.028, 0.0);
    match good {
        Good::Wood if region == Region::Oasis => super::oasis::palm(&mut b, at, 1.17, 0.32),
        Good::Food | Good::Local => {
            for (i, (x, z)) in [(-0.48, 0.14), (0.09, -0.23), (0.42, 0.29)]
                .into_iter()
                .enumerate()
            {
                b.gem(
                    v(x, 0.34, z),
                    0.45,
                    0.39,
                    0.43,
                    i as f32,
                    shift(colors.leaves, 16),
                );
                for j in 0..6 {
                    let a = j as f32 * 2.399;
                    b.gem(
                        v(
                            x + 0.25 * a.cos(),
                            0.56 + 0.06 * (j % 2) as f32,
                            z + 0.24 * a.sin(),
                        ),
                        0.078,
                        0.078,
                        0.066,
                        a,
                        if good == Good::Food {
                            [206, 105, 70]
                        } else {
                            [226, 174, 87]
                        },
                    );
                }
            }
        }
        Good::Clay | Good::Ore | Good::Stone => {
            for (i, (x, z, r)) in [(-0.45, 0.51, 0.25), (0.44, 0.18, 0.33), (0.06, -0.40, 0.42)]
                .into_iter()
                .enumerate()
            {
                b.gem(
                    v(x, r * 0.34, z),
                    r * 1.23,
                    r * 0.69,
                    r,
                    i as f32,
                    if good == Good::Clay {
                        [172, 103, 74]
                    } else {
                        shift(colors.rock, i as i16 * 9)
                    },
                );
            }
        }
        _ => {
            b.beam(
                v(-0.69, 0.124, 0.33),
                v(0.52, 0.19, 0.66),
                0.16,
                colors.wood,
            );
        }
    }
    b.mesh()
}

fn civic(region: Region) -> Mesh {
    let mut b = Batch::default();
    let c = palette(region);
    plinth(&mut b, Vec3::ZERO, 3.45, 2.72, 0.26, c.coast);
    b.slab(v(0.0, 1.05, -0.37), v(2.94, 1.52, 1.6), shift(c.coast, 6));
    match region {
        Region::Japan => {
            super::japan::roof(&mut b, v(0.0, 1.94, -0.32), 1.70, 1.17, 0.64, [63, 83, 105])
        }
        Region::Nordic => super::nordic::shingle(
            &mut b,
            v(0.0, 1.87, -0.34),
            1.65,
            1.12,
            0.63,
            [96, 124, 101],
        ),
        Region::Oasis => {
            b.slab(v(0.0, 1.86, -0.36), v(3.16, 0.14, 1.79), c.coast);
            for x in [-0.65, 0.65] {
                b.frustum(v(x, 1.91, -0.42), 0.48, 0.51, 0.19, 14, c.flag);
                b.frustum(v(x, 2.10, -0.42), 0.51, 0.24, 0.30, 14, c.flag);
                b.frustum(v(x, 2.40, -0.42), 0.24, 0.036, 0.20, 14, [221, 177, 80]);
            }
        }
        _ => gable(
            &mut b,
            v(0.0, 1.88, -0.37),
            3.24,
            1.95,
            0.43,
            if region == Region::Rome {
                [166, 74, 53]
            } else {
                c.flag
            },
        ),
    }
    for x in [-1.22, 1.22] {
        super::buildings::column(&mut b, v(x, 0.27, 0.83), 1.36, 0.091);
        b.slab(v(x, 1.30, 0.883), v(0.36, 0.52, 0.055), c.flag);
        b.gem(v(x, 1.34, 0.93), 0.105, 0.13, 0.025, 0.2, [228, 184, 93]);
    }
    b.slab(v(0.0, 0.86, 0.45), v(0.76, 1.14, 0.076), c.wood);
    b.slab(v(0.0, 1.66, 0.72), v(2.72, 0.175, 0.60), c.coast);
    super::architecture::stoop(&mut b, v(0.0, 0.0, 1.14), 1.24, 0.26, 0.39, 3, c.coast);
    b.mesh()
}

fn workshop(region: Region, kind: Build, tier: u8) -> Mesh {
    let mut b = Batch::default();
    let c = palette(region);
    let y = 1.04 + f32::from(tier) * 0.13;
    plinth(&mut b, Vec3::ZERO, 2.68, 2.25, 0.13, c.rock);
    let roof = match kind {
        Build::Sawmill => c.flag,
        Build::Brickyard => [167, 83, 67],
        Build::Forge => [94, 97, 104],
        _ => unreachable!(),
    };
    for x in [-1.08, 1.08] {
        for z in [-0.82, 0.62] {
            b.beam(v(x, 0.13, z), v(x, y, z), 0.105, c.wood);
        }
    }
    if region == Region::Japan {
        super::japan::roof(&mut b, v(0.0, y + 0.05, -0.08), 1.32, 0.97, 0.36, roof);
    } else if matches!(region, Region::Nordic | Region::Oasis) {
        panel_roof(&mut b, v(0.0, y + 0.14, -0.13), 2.63, 1.68, 0.18, roof);
    } else {
        gable(&mut b, v(0.0, y + 0.035, -0.13), 2.57, 1.93, 0.36, roof);
    }
    match kind {
        Build::Sawmill => {
            for n in 0..3 + tier {
                let t = f32::from(n);
                let x = -0.81 + f32::from(n % 3) * 0.31;
                b.slab(
                    v(x, 0.18 + f32::from(n / 3) * 0.09, 0.61),
                    v(0.24, 0.087, 0.91),
                    shift(c.wood, 26 + t as i16 * 3),
                );
            }
            b.slab(v(0.44, 0.42, -0.19), v(0.91, 0.50, 0.89), c.wood);
            b.gem(
                v(0.55, 0.73, 0.14),
                0.255,
                0.237,
                0.042,
                0.33,
                [160, 171, 168],
            );
        }
        Build::Brickyard => {
            super::buildings::oven(&mut b, v(-0.62, 0.13, 0.40), [187, 105, 76]);
            for j in 0..4 + tier {
                b.slab(
                    v(
                        0.40 + f32::from(j % 2) * 0.36,
                        0.18 + f32::from(j / 4) * 0.11,
                        0.42 + f32::from(j % 4 / 2) * 0.35,
                    ),
                    v(0.33, 0.105, 0.26),
                    shift(roof, i16::from(j) * 5),
                );
            }
        }
        Build::Forge => {
            b.slab(v(-0.61, 0.48, -0.31), v(0.79, 0.70, 0.81), c.rock);
            b.slab(v(-0.61, 1.16, -0.60), v(0.49, 1.50, 0.47), [136, 86, 68]);
            b.slab(v(-0.61, 1.92, -0.60), v(0.56, 0.095, 0.52), c.rock);
            b.gem(
                v(-0.60, 0.48, 0.115),
                0.22,
                0.136,
                0.039,
                0.12,
                [235, 132, 67],
            );
            b.frustum(v(0.58, 0.13, 0.60), 0.24, 0.24, 0.38, 9, c.wood);
            b.slab(v(0.58, 0.59, 0.60), v(0.42, 0.18, 0.25), roof);
            b.gem(v(0.82, 0.61, 0.61), 0.21, 0.068, 0.097, 0.42, roof);
        }
        _ => unreachable!(),
    }
    if tier >= 2 {
        crate_box(&mut b, v(1.05, 0.13, 0.79), v(0.42, 0.30, 0.42), c.wood);
    }
    if tier >= 3 {
        rail(&mut b, v(-0.35, 0.13, -0.87), 1.42, c.wood);
    }
    if tier >= 4 {
        amphora(&mut b, v(-1.18, 0.13, 0.95), 0.75, c.flag);
        flowerpot(&mut b, v(1.20, 0.13, -0.42), 0.75, [203, 113, 119]);
    }
    if tier >= 5 {
        b.beam(
            v(-1.09, y - 0.39, 0.64),
            v(-0.80, y - 0.03, 0.64),
            0.081,
            c.wood,
        );
        b.beam(
            v(1.09, y - 0.39, 0.64),
            v(0.80, y - 0.03, 0.64),
            0.081,
            c.wood,
        );
    }
    if tier >= 6 {
        b.slab(v(0.0, y - 0.15, 0.93), v(0.71, 0.17, 0.078), c.flag);
        b.gem(
            v(0.0, y - 0.15, 0.99),
            0.089,
            0.067,
            0.029,
            0.36,
            [229, 176, 82],
        );
    }
    b.mesh()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::mesh::VertexAttributeValues;

    #[test]
    fn all_ninety_workshop_models_have_distinct_tiers_and_clear_the_paths() {
        for region in Region::ALL {
            assert!(civic(region).count_vertices() > 200);
            for kind in [Build::Sawmill, Build::Brickyard, Build::Forge] {
                let mut previous = None;
                for tier in 1..=crate::model::MAX_LEVEL {
                    let mesh = workshop(region, kind, tier);
                    let tag = (region, kind, tier);
                    let Some(VertexAttributeValues::Float32x3(vertices)) =
                        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                    else {
                        panic!("Keine Geometrie für {tag:?}");
                    };
                    assert!(vertices.len() > 100, "{tag:?}");
                    assert!(
                        vertices.iter().all(|v| v.iter().all(|n| n.is_finite())
                            && v[0].abs() < 1.68
                            && v[2].abs() < 1.53
                            && (0.0..=3.40).contains(&v[1])),
                        "{tag:?}"
                    );
                    assert_ne!(previous.as_ref(), Some(vertices), "{tag:?}");
                    previous = Some(vertices.clone());
                }
            }
        }
    }
}

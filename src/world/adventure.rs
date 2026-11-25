use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{camera::ScalingMode, mesh::skinning::SkinnedMesh, view::NoFrustumCulling},
    window::PrimaryWindow,
};

use crate::{
    adventure::{
        Adventure, Build, DroppedLoad, Good, Quest, Screen, Target, Task, Terrain,
        terrain::{HALL_DOOR, unit},
    },
    campaign::Region,
    game::{Game, Panel, View},
    model::{Kind, Weather},
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
pub(crate) struct ConstructionModels;

#[derive(Component)]
pub(crate) struct Worker(usize);

#[derive(Component)]
pub(crate) struct Marker;

#[derive(Component, Default)]
pub(crate) struct RouteTrail(Option<(Target, Vec2)>);

#[derive(Component)]
pub(crate) struct Relic;

#[derive(Component)]
pub(crate) struct CivicLight;

#[derive(Component)]
pub(crate) struct CivicGlow;

#[derive(Component)]
pub(crate) struct ResourceModel {
    site: usize,
    alive: bool,
    falls_at: Option<f32>,
}

#[derive(Component)]
pub(crate) struct CargoModel(Option<usize>);

#[derive(Component)]
pub(crate) struct SpentResource(usize);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkTool {
    Axe,
    Pick,
    Hammer,
}

#[derive(Component)]
pub(crate) struct FishingBoat(usize);

#[derive(Component)]
pub(crate) struct FishSchool(usize);

#[derive(Component)]
pub(crate) struct MerchantShip {
    berth: Vec3,
    direction: Vec2,
}

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
    staging: Vec<Option<[u16; Good::ALL.len()]>>,
    overflow: [u16; Good::ALL.len()],
    dropped: Vec<Option<DroppedLoad>>,
    ground_loads: Option<Entity>,
    workshops: Vec<(Region, Build, u8, Handle<Mesh>)>,
    cargo: Option<Handle<Mesh>>,
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
    mut trail: Query<(&mut RouteTrail, &Mesh3d, &mut Visibility)>,
    construction: Query<Entity, With<ConstructionModels>>,
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
        scene.staging.clear();
        if let Some(pile) = scene.ground_loads.take() {
            commands.entity(pile).despawn();
        }
        scene.overflow.fill(0);
        scene.dropped.clear();
        if let Some(terrain) = terrain {
            let earth = palette(terrain.level.region);
            let mesh = landscape(terrain, earth);
            let region = terrain.level.region;
            let spent = Good::RAW.map(|kind| meshes.add(spent_model(region, kind)));
            commands
                .spawn((
                    Ground,
                    Transform::from_translation(ORIGIN),
                    Visibility::Inherited,
                ))
                .with_children(|root| {
                    root.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(art.matte.clone())));
                    root.spawn((
                        RouteTrail::default(),
                        Mesh3d(meshes.add(Batch::default().mesh())),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            unlit: true,
                            ..default()
                        })),
                        Visibility::Hidden,
                        NoFrustumCulling,
                        NotShadowCaster,
                    ));
                    root.spawn((
                        Mesh3d(meshes.add(sea(terrain))),
                        MeshMaterial3d(art.island_sea.clone()),
                        NotShadowCaster,
                    ));
                    root.spawn((
                        Mesh3d(meshes.add(surf(terrain))),
                        MeshMaterial3d(art.island_sea.clone()),
                        NotShadowCaster,
                    ));
                    if !terrain.fishing.is_empty() {
                        let boat = meshes.add(workboat());
                        let fish = meshes.add(school_model());
                        for (index, place) in terrain.fishing.iter().enumerate() {
                            root.spawn((
                                FishingBoat(index),
                                Mesh3d(boat.clone()),
                                MeshMaterial3d(art.matte.clone()),
                                Transform::default(),
                                Visibility::Hidden,
                            ));
                            root.spawn((
                                FishSchool(index),
                                Mesh3d(fish.clone()),
                                MeshMaterial3d(art.matte.clone()),
                                Transform::from_xyz(place.at.x, -1.09, place.at.y),
                                NotShadowCaster,
                            ));
                        }
                    }
                    if terrain.level.has_trading() {
                        let port = terrain.trade_pier;
                        let direction = port.normalize();
                        let shore = v(port.x, terrain.height(port) + 0.13, port.y);
                        root.spawn((
                            Mesh3d(meshes.add(super::monuments::jetty(region))),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(shore).with_rotation(
                                Quat::from_rotation_y(direction.x.atan2(direction.y)),
                            ),
                        ));
                        let side = Vec2::new(direction.y, -direction.x);
                        let moored = port + direction * 4.78 + side * 2.53;
                        let berth = v(moored.x, -1.06, moored.y);
                        root.spawn((
                            MerchantShip { berth, direction },
                            Mesh3d(meshes.add(super::ships::hull(region, 1))),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(berth),
                        ))
                        .with_children(|boat| {
                            boat.spawn((
                                Mesh3d(meshes.add(super::ships::sail(region, 1))),
                                MeshMaterial3d(art.matte.clone()),
                            ));
                        });
                    }
                    for patch in woods(terrain, earth) {
                        root.spawn((Mesh3d(meshes.add(patch)), MeshMaterial3d(art.matte.clone())));
                    }
                    let hall = super::mesh::occlude(civic(region), 0.75, 0.3);
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
                    for (x, size) in [(-0.95, 2.2), (0.95, 2.2), (0.0, 3.6)] {
                        let at = v(x, terrain.height(Vec2::ZERO) + 1.35, 1.95);
                        root.spawn((CivicGlow, super::glow::halo(&art.glow, at, size, x * 3.0)));
                    }
                    super::critters::spawn(root, &mut meshes, &art.matte, terrain);
                    for (site, deposit) in terrain.deposits.iter().enumerate() {
                        let p = deposit.at;
                        let at = v(p.x, terrain.height(p), p.y);
                        let alive = island.is_some_and(|i| i.remaining(terrain, site) > 0);
                        let spent_mesh = &spent[Good::RAW
                            .iter()
                            .position(|kind| *kind == deposit.kind)
                            .unwrap()];
                        root.spawn((
                            SpentResource(site),
                            Mesh3d(spent_mesh.clone()),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(at).with_scale(Vec3::splat(
                                deposit.scenery.map_or(1.0, |p| p.size * 0.82),
                            )),
                            if alive {
                                Visibility::Hidden
                            } else {
                                Visibility::Inherited
                            },
                        ));
                        let mut model = root.spawn((
                            ResourceModel {
                                site,
                                alive,
                                falls_at: None,
                            },
                            Transform::from_translation(at),
                            if alive {
                                Visibility::Inherited
                            } else {
                                Visibility::Hidden
                            },
                        ));
                        if let Some(scenery) = deposit.scenery {
                            model.insert((
                                Mesh3d(meshes.add(scenery_model(
                                    region,
                                    deposit.kind,
                                    scenery.seed,
                                    scenery.size,
                                    at.y,
                                    earth,
                                ))),
                                MeshMaterial3d(art.matte.clone()),
                            ));
                            continue;
                        }
                        let source = match deposit.kind {
                            Good::Wood if region != Region::Oasis => Some(("wood_tree", 0.77)),
                            Good::Stone => Some(("resource_rock", 0.94)),
                            Good::Ore if matches!(region, Region::Greece | Region::Oasis) => {
                                Some(("resource_copper", 0.77))
                            }
                            Good::Ore => Some(("resource_crystal", 0.71)),
                            _ => None,
                        };
                        model.with_children(|model| {
                            if let Some((filename, scale)) = source {
                                let path = format!("embedded://models/waldwinkel/{filename}.glb");
                                model.spawn((
                                    SceneRoot(
                                        assets.load(GltfAssetLabel::Scene(0).from_asset(path)),
                                    ),
                                    Transform::from_scale(Vec3::splat(scale)),
                                ));
                            }
                            model.spawn((
                                Mesh3d(meshes.add(deposit_model(region, deposit.kind))),
                                MeshMaterial3d(art.matte.clone()),
                            ));
                        });
                    }
                    if terrain.level.order_count() > 0 {
                        let kind = match region {
                            Region::Greece => Kind::Storehouse,
                            Region::Rome => Kind::FerryOffice,
                            Region::Japan => Kind::Machiya,
                            Region::Nordic => Kind::Longhouse,
                            Region::Oasis => Kind::Caravanserai,
                        };
                        let at = terrain.outpost;
                        root.spawn((
                            Mesh3d(meshes.add(super::model(region, kind, 2))),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_xyz(at.x, terrain.height(at), at.y)
                                .with_scale(Vec3::splat(1.18)),
                        ));
                        let mut banner = Batch::default();
                        banner.beam(Vec3::ZERO, Vec3::Y * 2.8, 0.072, earth.wood);
                        banner.face(
                            &[
                                v(0., 2.72, 0.),
                                v(0.96, 2.58, 0.03),
                                v(0.83, 1.98, 0.08),
                                v(0., 2.06, 0.),
                            ],
                            earth.flag,
                        );
                        root.spawn((
                            Mesh3d(meshes.add(banner.mesh())),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_xyz(
                                at.x + 2.3,
                                terrain.height(at + Vec2::X * 2.3),
                                at.y,
                            ),
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
            let load = scene
                .cargo
                .get_or_insert_with(|| {
                    let mut crate_mesh = Batch::default();
                    crate_box(
                        &mut crate_mesh,
                        Vec3::ZERO,
                        v(0.40, 0.24, 0.31),
                        [158, 105, 62],
                    );
                    meshes.add(crate_mesh.mesh())
                })
                .clone();
            commands.spawn((
                Ground,
                CargoModel(None),
                Mesh3d(load),
                MeshMaterial3d(art.matte.clone()),
                Transform::default(),
                Visibility::Hidden,
            ));
            for tool in [WorkTool::Pick, WorkTool::Axe, WorkTool::Hammer] {
                commands.spawn((
                    Ground,
                    tool,
                    Mesh3d(meshes.add(if tool == WorkTool::Hammer {
                        build_hammer()
                    } else {
                        harvest_tool(tool == WorkTool::Axe)
                    })),
                    MeshMaterial3d(art.matte.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                ));
            }
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
        for entity in buildings
            .iter()
            .chain(workers.iter())
            .chain(construction.iter())
        {
            commands.entity(entity).despawn();
        }
        return;
    };
    if scene.overflow != island.overflow || scene.dropped != island.dropped {
        if let Some(pile) = scene.ground_loads.take() {
            commands.entity(pile).despawn();
        }
        scene.overflow = island.overflow;
        scene.dropped.clone_from(&island.dropped);
        let mut batch = Batch::default();
        for (index, kind) in Good::ALL
            .into_iter()
            .filter(|&kind| island.overflow[kind as usize] > 0)
            .enumerate()
        {
            let at = HALL_DOOR
                + Vec2::new(
                    1.82 + (index % 4) as f32 * 0.74,
                    0.52 + (index / 4) as f32 * 0.86,
                );
            let origin = v(at.x, terrain.height(at), at.y);
            batch.slab(
                origin + Vec3::Y * 0.075,
                v(0.64, 0.10, 0.77),
                [172, 125, 78],
            );
            let count = island.overflow[kind as usize].div_ceil(2).min(5);
            material_bundles(&mut batch, origin + v(-0.06, 0.09, 0.0), kind, count);
        }
        for pile in island.dropped.iter().flatten() {
            let at = Vec2::from_array(pile.position);
            let origin = v(at.x, terrain.height(at), at.y);
            material_bundles(
                &mut batch,
                origin,
                pile.cargo.kind,
                pile.cargo.units.div_ceil(2).min(5),
            );
        }
        if batch.vertices() > 0 {
            scene.ground_loads = Some(
                commands
                    .spawn((
                        Mesh3d(meshes.add(batch.mesh())),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(ORIGIN),
                    ))
                    .id(),
            );
        }
    }
    let stages = island
        .construction
        .iter()
        .map(|site| site.as_ref().map(|site| site.materials));
    if scene.staging.iter().copied().ne(stages.clone()) {
        scene.staging = stages.collect();
        for entity in &construction {
            commands.entity(entity).despawn();
        }
        let mut batch = Batch::default();
        for (lot, materials) in scene
            .staging
            .iter()
            .enumerate()
            .filter_map(|(lot, site)| site.as_ref().map(|site| (lot, site)))
        {
            build_site(&mut batch, terrain, terrain.plots[lot], materials);
        }
        if batch.vertices() > 0 {
            commands.spawn((
                ConstructionModels,
                Mesh3d(meshes.add(batch.mesh())),
                MeshMaterial3d(art.matte.clone()),
                Transform::from_translation(ORIGIN),
            ));
        }
    }
    if !switched && let Ok((mut trail, handle, mut shown)) = trail.single_mut() {
        let active = game.panel == Panel::None && adventure.screen == Screen::Walk;
        let target = adventure.waypoint.filter(|&target| {
            active && adventure.target_position(target).is_some() && !adventure.close_enough(target)
        });
        let here = Vec2::from_array(island.position);
        if let Some(target) = target {
            if scene.buildings != island.lots
                || trail
                    .0
                    .is_none_or(|(old, from)| old != target || from.distance_squared(here) > 0.81)
            {
                trail.0 = Some((target, here));
                let path = adventure.waypoint_path().unwrap_or_default();
                let batch = trail_arrows(terrain, &path);
                *shown = if batch.vertices() == 0 {
                    Visibility::Hidden
                } else {
                    if let Some(mesh) = meshes.get_mut(&handle.0) {
                        *mesh = batch.mesh();
                    }
                    Visibility::Inherited
                };
            }
        } else if trail.0.is_some() {
            trail.0 = None;
            *shown = Visibility::Hidden;
        }
    }
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
                                    let handle = meshes.add(super::mesh::occlude(
                                        workshop(region, kind, property.tier),
                                        0.9,
                                        0.3,
                                    ));
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
                        if let Some(&(_, when)) = adventure
                            .scene_changes
                            .iter()
                            .find(|(i, when)| *i == lot && game.clock - *when < 1.2)
                        {
                            spot.insert(BuildLift { when, base: p.y });
                            spot.with_children(|spot| {
                                for piece in 0..super::construction::CONFETTI {
                                    let parts = &art.work.confetti;
                                    spot.spawn((
                                        super::construction::Confetti { piece, when },
                                        Mesh3d(parts[piece % parts.len()].clone()),
                                        MeshMaterial3d(art.matte.clone()),
                                        Transform::default(),
                                        Visibility::Hidden,
                                        NotShadowCaster,
                                    ));
                                }
                            });
                        }
                        spot.with_children(|model| {
                            if property.kind == Build::Harbor(Kind::Fishery)
                                && let Some(place) =
                                    terrain.fishing.iter().find(|place| place.lot == lot)
                            {
                                let mut dock = Batch::default();
                                let facing = (place.launch - at).normalize_or_zero();
                                let start = at + facing * 1.17;
                                for i in 0..9 {
                                    let point = start.lerp(place.launch, i as f32 / 9.0);
                                    let offset = point - at;
                                    let y = (-0.63_f32).max(terrain.height(point) + 0.061)
                                        - terrain.height(at);
                                    dock.beam(
                                        v(
                                            offset.x - facing.y * 0.72,
                                            y,
                                            offset.y + facing.x * 0.72,
                                        ),
                                        v(
                                            offset.x + facing.y * 0.72,
                                            y,
                                            offset.y - facing.x * 0.72,
                                        ),
                                        0.17,
                                        [146, 104, 70],
                                    );
                                }
                                model.spawn((
                                    Mesh3d(meshes.add(dock.mesh())),
                                    MeshMaterial3d(art.matte.clone()),
                                ));
                            }
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
    let population = usize::from(island.staff()).max(island.assigned_staff());
    if switched || scene.staff != population {
        scene.staff = population;
        for entity in &workers {
            commands.entity(entity).despawn();
        }
        let cargo = scene.cargo.clone();
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
                    if let Some(load) = &cargo {
                        parent.spawn((
                            CargoModel(Some(index)),
                            Mesh3d(load.clone()),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::default(),
                            Visibility::Hidden,
                        ));
                    }
                }
            });
    }
}

pub(crate) fn merchant_visuals(
    game: Res<Game>,
    adventure: Res<Adventure>,
    mut boats: Query<(&MerchantShip, &mut Transform)>,
) {
    let Some((map, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    for (ship, mut pose) in &mut boats {
        let phase = island.market.crossing.and_then(|crossing| {
            map.level
                .export(crossing.export)
                .map(|offer| (1.0 - crossing.remaining / offer.seconds).clamp(0.0, 1.0))
        });
        let (distance, heading) = phase.map_or((0.0, 0.0), |phase| {
            let out = (phase * std::f32::consts::PI).sin() * 26.0;
            let turn = ((phase - 0.43) / 0.14).clamp(0.0, 1.0);
            (out, turn * std::f32::consts::PI)
        });
        let yaw = (-ship.direction.y).atan2(ship.direction.x) + heading;
        let waves = (game.clock * 2.4 + distance * 0.16).sin();
        pose.translation = ship.berth
            + v(
                ship.direction.x * distance,
                waves * 0.055,
                ship.direction.y * distance,
            );
        pose.rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(waves * 0.041);
        pose.scale = Vec3::splat(0.85);
    }
}

fn trail_arrows(terrain: &Terrain, path: &[Vec2]) -> Batch {
    let mut batch = Batch::default();
    let distance = path
        .windows(2)
        .map(|points| points[0].distance(points[1]))
        .sum::<f32>();
    let mut next = 0.64f32.min(distance * 0.5);
    for edge in path.windows(2) {
        let offset = edge[1] - edge[0];
        let length = offset.length();
        let forward = offset.normalize_or_zero();
        let side = forward.perp();
        while next < length && batch.vertices() < 18 * 12 {
            let center = edge[0] + forward * next;
            for (size, lift, tint) in [(1.0, 0.24, [26, 72, 75]), (0.70, 0.26, [144, 255, 212])] {
                let point = |x: f32, y: f32| {
                    let q = center + (forward * x + side * y) * size;
                    v(q.x, terrain.height(q) + lift, q.y)
                };
                let tip = point(0.39, 0.0);
                let notch = point(0.07, 0.0);
                batch.face(&[tip, notch, point(-0.22, 0.43)], tint);
                batch.face(&[tip, point(-0.22, -0.43), notch], tint);
            }
            next += 1.12;
        }
        next -= length;
    }
    batch
}

fn build_site(batch: &mut Batch, terrain: &Terrain, at: Vec2, materials: &[u16; Good::ALL.len()]) {
    let origin = v(at.x, terrain.height(at), at.y);
    let wood = [143, 106, 73];
    let foot = |x, z| origin + v(x, 0.06, z);
    for x in [-1.56, 1.56] {
        let post = foot(x, 1.47);
        batch.frustum(post, 0.10, 0.07, 0.88, 5, wood);
        batch.slab(
            post + v(0.0, 0.16, 0.0),
            v(0.28, 0.16, 0.30),
            [169, 141, 107],
        );
        batch.beam(
            post + v(0.0, 0.31, 0.0),
            foot(x, -1.50) + v(0.0, 0.18, 0.0),
            0.05,
            [221, 176, 108],
        );
    }
    let sign = foot(-1.56, 1.47) + v(0.04, 0.76, 0.16);
    batch.slab(sign, v(0.53, 0.38, 0.12), [36, 118, 112]);
    batch.beam(
        sign + v(-0.13, -0.09, 0.073),
        sign + v(0.12, 0.10, 0.073),
        0.055,
        [249, 226, 158],
    );
    batch.beam(
        sign + v(0.04, 0.14, 0.073),
        sign + v(0.17, 0.01, 0.073),
        0.088,
        [249, 226, 158],
    );
    batch.slab(foot(0.0, -1.47), v(3.18, 0.10, 0.22), [185, 149, 109]);
    batch.slab(foot(0.0, 1.47), v(3.18, 0.10, 0.22), [185, 149, 109]);

    for (pile, kind) in Good::ALL
        .into_iter()
        .filter(|&good| materials[good as usize] > 0)
        .enumerate()
    {
        let count = materials[kind as usize].div_ceil(4).min(5);
        let p = foot(
            -1.31 + (pile % 5) as f32 * 0.66,
            2.04 + (pile / 5) as f32 * 0.66,
        );
        material_bundles(batch, p, kind, count);
    }
}

fn material_bundles(batch: &mut Batch, p: Vec3, kind: Good, count: u16) {
    for bundle in 0..count {
        let p = p + v(
            f32::from(bundle % 2) * 0.20,
            f32::from(bundle / 2) * 0.14,
            f32::from(bundle) * 0.045,
        );
        match kind {
            Good::Wood | Good::Boards => {
                let tint = if kind == Good::Wood {
                    [144, 93, 57]
                } else {
                    [202, 157, 96]
                };
                batch.slab(p + Vec3::Y * 0.12, v(0.23, 0.13, 0.77), tint);
                batch.slab(p + v(0.0, 0.197, 0.21), v(0.26, 0.025, 0.08), [98, 75, 52]);
            }
            Good::Stone | Good::Ore | Good::Clay => {
                let tint = match kind {
                    Good::Stone => [183, 174, 161],
                    Good::Ore => [104, 122, 123],
                    _ => [182, 108, 82],
                };
                batch.gem(
                    p + Vec3::Y * 0.16,
                    0.26,
                    0.16,
                    0.23,
                    f32::from(bundle) * 0.39,
                    tint,
                );
            }
            Good::Bricks => {
                batch.slab(p + Vec3::Y * 0.09, v(0.34, 0.15, 0.23), [185, 99, 69]);
                batch.slab(p + v(0.04, 0.20, 0.04), v(0.28, 0.09, 0.21), [210, 122, 88]);
            }
            Good::Water | Good::Oil | Good::Perfume => {
                amphora(batch, p + v(0.0, 0.06, 0.05), 0.48, [205, 149, 95]);
            }
            Good::Fiber | Good::Cloth | Good::Paper => {
                batch.beam(
                    p + v(0.0, 0.17, -0.23),
                    p + v(0.0, 0.17, 0.31),
                    0.19,
                    [199, 176, 120],
                );
                batch.slab(p + v(0.0, 0.245, 0.02), v(0.22, 0.04, 0.09), [65, 120, 119]);
            }
            _ => {
                crate_box(
                    batch,
                    p + Vec3::Y * 0.13,
                    v(0.27, 0.21, 0.32),
                    [165, 111, 67],
                );
                let tint = if matches!(kind, Good::Tools | Good::Craft) {
                    [160, 184, 187]
                } else {
                    [231, 186, 111]
                };
                batch.slab(p + Vec3::Y * 0.255, v(0.19, 0.051, 0.12), tint);
            }
        }
    }
}

fn build_hammer() -> Mesh {
    let mut batch = Batch::default();
    batch.beam(v(0.0, -0.17, 0.0), v(0.0, 0.26, 0.0), 0.065, [169, 118, 70]);
    batch.slab(v(0.0, 0.23, 0.0), v(0.24, 0.12, 0.13), [120, 142, 151]);
    batch.mesh()
}

#[allow(clippy::type_complexity)]
pub(crate) fn logistics_visuals(
    game: Res<Game>,
    adventure: Res<Adventure>,
    mut nodes: Query<(&mut ResourceModel, &mut Transform, &mut Visibility)>,
    mut loads: Query<
        (&CargoModel, &mut Transform, &mut Visibility),
        (Without<ResourceModel>, Without<WorkTool>),
    >,
    mut tools: Query<(&WorkTool, &mut Transform, &mut Visibility), Without<ResourceModel>>,
    mut spent: Query<
        (&SpentResource, &mut Visibility),
        (
            Without<ResourceModel>,
            Without<WorkTool>,
            Without<CargoModel>,
        ),
    >,
) {
    let Some((terrain, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    if game.view != View::Adventure {
        return;
    }
    for (model, mut shown) in &mut spent {
        let visibility = if island.remaining(terrain, model.0) == 0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *shown != visibility {
            *shown = visibility;
        }
    }
    for (mut model, mut pose, mut shown) in &mut nodes {
        let deposit = terrain.deposits[model.site];
        let striking = adventure
            .harvesting
            .filter(|work| work.site == model.site)
            .map(|work| work.duration - work.left)
            .or_else(|| {
                island
                    .hauls
                    .iter()
                    .find(|haul| haul.site == Some(model.site) && haul.work > 0.0)
                    .map(|haul| haul.work)
            });
        if model.alive && island.remaining(terrain, model.site) == 0 {
            model.alive = false;
            model.falls_at = Some(game.clock);
        }
        if let Some(since) = model.falls_at {
            let progress = ((game.clock - since) / 0.79).clamp(0.0, 1.0);
            let fall = progress * progress;
            if deposit.kind == Good::Wood {
                pose.rotation = Quat::from_rotation_z(1.51 * fall);
            } else {
                pose.scale = Vec3::splat(1.0 - 0.91 * fall);
            }
            if progress >= 1.0 {
                *shown = Visibility::Hidden;
                model.falls_at = None;
            }
        } else if model.alive {
            let hit = striking.map_or(0.0, |elapsed| (elapsed * 12.0).sin() * 0.048);
            let rotation = Quat::from_rotation_z(hit);
            if pose.rotation != rotation {
                pose.rotation = rotation;
            }
        }
    }
    for (model, mut pose, mut shown) in &mut loads {
        let actor = match model.0 {
            Some(worker) => island
                .walking_worker(worker)
                .filter(|(_, carrying)| *carrying)
                .map(|(haul, _)| (haul.position, haul.facing, haul.gait, haul.moving, 1.18)),
            None if island.has_cargo() => Some((
                island.position,
                adventure.heading,
                adventure.gait,
                adventure.moved,
                1.28,
            )),
            None => None,
        };
        if let Some((p, yaw, gait, walking, scale)) = actor {
            let rot = Quat::from_rotation_y(yaw);
            let sway = if walking { gait.sin() * 0.047 } else { 0.0 };
            pose.translation =
                position(terrain, Vec2::from_array(p)) + rot * v(0.10, 0.65 + sway, 0.48);
            pose.rotation = rot;
            pose.scale = Vec3::splat(scale);
            *shown = Visibility::Inherited;
        } else {
            *shown = Visibility::Hidden;
        }
    }
    for (tool, mut pose, mut shown) in &mut tools {
        let swing = if *tool == WorkTool::Hammer && adventure.building.is_some() {
            Some(game.clock * 9.0)
        } else {
            adventure.harvesting.and_then(|work| {
                let kind = terrain.deposits[work.site].kind;
                let work_tool = match kind {
                    Good::Wood => WorkTool::Axe,
                    Good::Stone | Good::Ore | Good::Clay => WorkTool::Pick,
                    _ => return None,
                };
                (*tool == work_tool).then_some((work.duration - work.left) * 11.0)
            })
        };
        if let Some(swing) = swing {
            let rot = Quat::from_rotation_y(adventure.heading);
            pose.translation = position(terrain, Vec2::from_array(island.position))
                + rot * v(-0.30, 0.82 + swing.sin() * 0.17, 0.45);
            pose.rotation = rot
                * Quat::from_rotation_x(0.42 + swing.cos() * 0.83)
                * Quat::from_rotation_z(-0.31);
            *shown = Visibility::Inherited;
        } else {
            *shown = Visibility::Hidden;
        }
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
    art: Res<Art>,
    mut waters: ResMut<Assets<super::water::Water>>,
    mut civic_glow: Query<&mut super::glow::Halo, With<CivicGlow>>,
    mut scene: ParamSet<(
        Query<(&mut Transform, &mut Projection), (With<StreetCamera>, Without<Sun>)>,
        Query<&mut Transform, (With<Hippo>, Without<Sun>)>,
        Query<(&Limb, &mut Transform), Without<Sun>>,
        Query<(&Marker, &mut Transform, &mut Visibility), Without<Sun>>,
        Query<&mut Visibility, With<Relic>>,
        Query<(&Worker, &SkinnedMesh, &mut Transform, &mut Visibility), Without<Sun>>,
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
        let movement = if !limb.leg
            && let Some(work) = adventure.harvesting
        {
            let beat = ((work.duration - work.left) * 10.9 + limb.side * 0.45).sin();
            -0.64 + beat * 0.80
        } else if !limb.leg && adventure.building.is_some() {
            -0.43 + (game.clock * 9.0 + limb.side * 0.18).sin() * 0.55
        } else if !limb.leg && island.has_cargo() {
            -0.77 + adventure.gait.sin() * 0.055
        } else if adventure.moved {
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
    for (worker, skin, mut model, mut visibility) in &mut scene.p5() {
        let job = island.workers.get(worker.0).copied();
        let at_bench = job.is_some_and(|job| {
            let Task::Produce(lot) = job else {
                return false;
            };
            island
                .workers
                .iter()
                .position(|&candidate| candidate == job)
                == Some(worker.0)
                && island.site(lot).is_none()
                && island.batch(lot).is_some_and(|batch| batch.working())
                && super::activity::adventure_spot(terrain, &island.lots, lot).is_some()
        });
        *visibility = if at_bench {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        if at_bench {
            continue;
        }
        if let Some((haul, carrying)) = island.walking_worker(worker.0) {
            let bob = if haul.moving {
                haul.gait.sin().abs() * 0.040
            } else {
                0.0
            };
            model.translation =
                position(terrain, Vec2::from_array(haul.position)) + Vec3::Y * (0.028 + bob);
            model.rotation = Quat::from_rotation_y(haul.facing);
            model.scale = Vec3::splat(1.18);
            let mut pose = super::people::worker_pose(
                &cast,
                worker.0,
                terrain.level.region,
                haul.gait,
                f32::from(haul.moving),
                if haul.work > 0.0 {
                    haul.work * 2.3
                } else {
                    game.clock
                },
                haul.work > 0.0,
            );
            if carrying {
                for joint in [1, 5] {
                    pose[joint].rotation *= Quat::from_rotation_x(-0.65);
                }
            }
            poses.extend(skin.joints.iter().copied().zip(pose));
            continue;
        }
        let (spot, center) = match job {
            Some(Task::Produce(lot)) => (
                terrain.plots[lot] + Vec2::new(1.42, 1.61),
                terrain.plots[lot] + Vec2::new(-1.39, 2.65),
            ),
            _ => {
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
        light.illuminance = 11_200.0 - night * 8_900.0;
        light.color = Color::srgb(1.0 - night * 0.50, 0.93 - night * 0.28, 0.79 + night * 0.21);
    }
    ambient.brightness = 360.0 - night * 70.0;
    ambient.color = Color::srgb(0.94 - night * 0.38, 0.93 - night * 0.22, 1.0);
    clear.0 = Color::srgb(
        0.40 - night * 0.24,
        0.61 - night * 0.32,
        0.67 - night * 0.21,
    );
    for mut lamp in &mut civic_lights {
        lamp.intensity = night * 9800.0;
    }
    for mut halo in &mut civic_glow {
        halo.strength = ((night - 0.25) / 0.45).clamp(0.0, 1.0);
    }
    let dusk = night / 0.86;
    let sea = super::water::Settings::calm(Vec3::ONE.lerp(v(0.44, 0.58, 0.80), dusk), 0.05)
        .weather((Weather::Clear, Weather::Clear, 1.0), dusk);
    if waters
        .get(&art.island_sea)
        .is_some_and(|water| (water.settings.tint - sea.tint).length() > 0.004)
        && let Some(water) = waters.get_mut(&art.island_sea)
    {
        water.settings = sea;
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
            [126, 159, 92],
            [223, 190, 135],
            [157, 151, 131],
            [123, 88, 53],
            [107, 138, 74],
            [55, 128, 147],
        ),
        Region::Rome => (
            [129, 153, 91],
            [206, 171, 125],
            [156, 133, 115],
            [130, 86, 63],
            [99, 134, 80],
            [167, 76, 57],
        ),
        Region::Japan => (
            [103, 150, 101],
            [201, 177, 134],
            [124, 138, 137],
            [110, 76, 57],
            [98, 143, 102],
            [183, 81, 66],
        ),
        Region::Nordic => (
            [99, 137, 106],
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

/// Smooth value noise in 0..1, identical on every platform.
fn value_noise(p: Vec2, seed: u32) -> f32 {
    let cell = p.floor();
    let f = p - cell;
    let s = f * f * (Vec2::splat(3.0) - 2.0 * f);
    let corner = |dx: i32, dz: i32| {
        let x = (cell.x as i32 + dx + 8192) as u32;
        let z = (cell.y as i32 + dz + 8192) as u32;
        unit(x.wrapping_mul(73_856_093) ^ z.wrapping_mul(19_349_663) ^ seed)
    };
    let top = corner(0, 0) + (corner(1, 0) - corner(0, 0)) * s.x;
    let bottom = corner(0, 1) + (corner(1, 1) - corner(0, 1)) * s.x;
    top + (bottom - top) * s.y
}

fn mix(a: Tint, b: Tint, t: f32) -> Tint {
    let t = t.clamp(0.0, 1.0);
    std::array::from_fn(|i| (f32::from(a[i]) + (f32::from(b[i]) - f32::from(a[i])) * t) as u8)
}

/// The walkways between the plots: the broad yard in front of the hall and the lanes.
struct Lanes {
    depth: f32,
    rows: Vec<(f32, f32)>,
}

impl Lanes {
    fn new(terrain: &Terrain) -> Self {
        let depth = terrain.plots[..terrain.level.inland_plots()]
            .iter()
            .fold(8.0_f32, |n, p| n.max(p.y.abs()));
        let rows = [-5.7_f32, 6.4, -11.7, 12.5, -17.8, 18.6]
            .into_iter()
            .filter_map(|row| {
                let extent = terrain
                    .plots
                    .iter()
                    .filter(|p| (p.y - row).abs() < 0.05)
                    .map(|p| p.x.abs())
                    .fold(0.0_f32, f32::max);
                (extent >= 1.0).then(|| (row - row.signum() * 2.22, extent))
            })
            .collect();
        Self { depth, rows }
    }

    fn lane_z(z: f32, x: f32) -> f32 {
        z + (x * 0.34).sin() * 0.21
    }

    /// Distance from the nearest walkway centre line, in world units.
    fn distance(&self, p: Vec2) -> f32 {
        let yard = if p.y.abs() <= self.depth + 0.5 {
            (p.x.abs() - 1.25).max(0.0)
        } else {
            f32::MAX
        };
        self.rows
            .iter()
            .filter(|(_, extent)| p.x.abs() <= extent + 0.6)
            .map(|&(z, _)| (p.y - Self::lane_z(z, p.x)).abs())
            .fold(yard, f32::min)
    }
}

fn landscape(terrain: &Terrain, colors: Palette) -> Mesh {
    let mut b = Batch::default();
    let step = if terrain.level.stage < 5 { 1.68 } else { 2.01 };
    let span = (terrain.radius * 1.18 / step).ceil() as i32;
    let lanes = Lanes::new(terrain);
    let seed = terrain.level.index as u32 * 7_919 + 3_571;
    // Lush hollows and sun-dried slopes keep the grass from looking like one carpet.
    let lush = shift(mix(colors.turf, colors.leaves, 0.45), -10);
    let dry = mix(colors.turf, colors.coast, 0.42);
    let earth = mix(shift(colors.coast, -30), colors.rock, 0.38);
    let worn = mix(colors.turf, colors.coast, 0.50);
    let ground = |p: Vec2, height: f32, t: f32| -> Tint {
        let edge = terrain.shore(p);
        let shore = edge < 4.2 + t * 2.6;
        let stone = height > 2.8 + t * 1.5;
        let snow = terrain.level.region == Region::Nordic && height > 6.7 + t * 2.8;
        if snow {
            return shift([216, 224, 219], (t * 8.0) as i16);
        }
        if stone {
            return shift(
                colors.rock,
                ((value_noise(p * 0.31, seed) - 0.5) * 18.0) as i16,
            );
        }
        if shore {
            let wet = (1.0 - edge / 1.6).clamp(0.0, 1.0);
            return mix(colors.coast, shift(colors.coast, -24), wet * 0.6);
        }
        let patches = value_noise(p * 0.071, seed);
        let detail = value_noise(p * 0.23, seed ^ 0x55aa);
        let mut tint = if patches < 0.5 {
            mix(lush, colors.turf, patches * 2.0)
        } else {
            mix(colors.turf, dry, (patches - 0.5) * 2.0)
        };
        if let Some(district) = terrain.district(p) {
            tint = mix(tint, district.district_color(), 0.36);
        }
        // Grass is trodden around the houses and worn bare along the walkways.
        let town = (p.x.abs() - 17.0).max(0.0) + (p.y.abs() - lanes.depth - 3.0).max(0.0);
        tint = mix(tint, worn, (1.0 - town / 6.0).clamp(0.0, 1.0) * 0.18);
        tint = mix(
            tint,
            earth,
            (1.0 - (lanes.distance(p) - 0.45) / 0.8).clamp(0.0, 1.0) * 0.62,
        );
        shift(tint, ((detail - 0.5) * 16.0) as i16)
    };
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
            let height = (a.y + d.y) * 0.5;
            let t = unit(
                (row + 92) as u32 * 2389
                    + (col + 92) as u32 * 337
                    + terrain.level.index as u32 * 57,
            );
            let sweep = (at.x * 0.23 + at.y * 0.14).sin() * (at.y * 0.18).cos();
            let shade = (sweep * 7.0 + (t - 0.50) * 9.0) as i16;
            let centre = |p: Vec3, q: Vec3, r: Vec3| {
                let m = (p + q + r) / 3.0;
                Vec2::new(m.x, m.z)
            };
            b.face(&[a, e, d], shift(ground(centre(a, e, d), height, t), shade));
            b.face(
                &[a, d, c],
                shift(ground(centre(a, d, c), height, t), shade - 5),
            );
        }
    }
    // Flagstones laid along the yard and the lanes; the earth shows between them.
    let flag = |b: &mut Batch, p: Vec2, size: Vec2, n: u32| {
        let spin = (unit(n) - 0.5) * 0.36;
        let y = terrain.height(p) + 0.022 + unit(n + 1) * 0.012;
        let tone = mix(
            shift(colors.coast, 4),
            colors.rock,
            0.35 + unit(n + 2) * 0.3,
        );
        b.block(
            v(p.x, y, p.y),
            v(size.x, 0.05, size.y),
            Quat::from_rotation_y(spin),
            shift(tone, (unit(n + 3) * 14.0) as i16 - 7),
        );
    };
    for z in -(lanes.depth.ceil() as i32)..=lanes.depth.ceil() as i32 {
        for lane in 0..3 {
            let n = (z + 500) as u32 * 31 + lane * 7 + seed;
            if unit(n + 9) < 0.08 {
                continue;
            }
            let x = (lane as f32 - 1.0) * 0.80 + (unit(n + 4) - 0.5) * 0.10;
            let p = Vec2::new(x, z as f32 + (unit(n + 5) - 0.5) * 0.12);
            flag(&mut b, p, Vec2::new(0.62 + unit(n + 6) * 0.14, 0.78), n);
        }
    }
    for (i, &(z, extent)) in lanes.rows.iter().enumerate() {
        let mut x = -extent;
        let mut n = seed + i as u32 * 9_001;
        while x < extent {
            n += 13;
            let length = 0.56 + unit(n) * 0.34;
            if x.abs() > 1.7 && unit(n + 8) > 0.06 {
                for side in [-0.21_f32, 0.21] {
                    let jitter = (unit(n + side.to_bits()) - 0.5) * 0.09;
                    let p = Vec2::new(x + length * 0.5, Lanes::lane_z(z, x) + side + jitter);
                    flag(
                        &mut b,
                        p,
                        Vec2::new(length - 0.07, 0.36),
                        n + side.to_bits(),
                    );
                }
            }
            x += length;
        }
    }
    // Plots are staked out on packed earth instead of paved over.
    let pad = mix(mix(colors.turf, earth, 0.38), worn, 0.2);
    for (index, p) in terrain.plots.iter().copied().enumerate() {
        let y = terrain.height(p);
        let n = seed + index as u32 * 977;
        plinth(&mut b, v(p.x, y + 0.004, p.y), 3.12, 2.69, 0.062, pad);
        for (x, z) in [(-1.41, -1.21), (1.41, -1.21), (-1.41, 1.21), (1.41, 1.21)] {
            let at = v(p.x + x, y + 0.05, p.y + z);
            b.frustum(
                at,
                0.05,
                0.04,
                0.30,
                5,
                shift(colors.wood, (unit(n) * 12.0) as i16),
            );
            b.gem(at + Vec3::Y * 0.31, 0.05, 0.04, 0.05, x + z, colors.flag);
        }
        for z in [-1.21, 1.21] {
            b.beam(
                v(p.x - 1.41, y + 0.24, p.y + z),
                v(p.x + 1.41, y + 0.24, p.y + z),
                0.014,
                [226, 214, 182],
            );
        }
        for i in 0..5 {
            let m = n + i * 17;
            let tuft = v(
                p.x + (unit(m) - 0.5) * 2.5,
                y + 0.06,
                p.y + (unit(m + 1) - 0.5) * 2.1,
            );
            let tint = shift(colors.leaves, 10 + (unit(m + 2) * 18.0) as i16);
            for blade in 0..3 {
                let dx = blade as f32 * 0.07 - 0.07;
                b.face(
                    &[
                        tuft + v(dx, 0.0, -0.03),
                        tuft + v(dx + 0.04, 0.0, 0.035),
                        tuft + v(dx * 1.6, 0.10 + unit(m + 3) * 0.08, 0.0),
                    ],
                    tint,
                );
            }
        }
        let mid = (p.y / 2.0).trunc() * 2.0;
        for a in 0..4 {
            let x = p.x + a as f32 * 0.23 - 0.34;
            let z = if p.y > mid { p.y - 1.46 } else { p.y + 1.46 };
            b.slab(
                v(x, y + 0.03, z),
                v(0.2, 0.04, 0.30),
                shift(mix(colors.coast, colors.rock, 0.3), 6),
            );
        }
    }
    b.mesh()
}

fn sea(terrain: &Terrain) -> Mesh {
    const CELL: f32 = 2.4;
    const SHALLOW: [f32; 3] = [102.0, 198.0, 194.0];
    const DEEP: [f32; 3] = [36.0, 108.0, 142.0];
    let mut b = Batch::default();
    let span = (terrain.radius * 2.2 / CELL).ceil() as i32;
    let tint = |q: Vec2, seed: u32| -> Tint {
        // Turquoise over the sand, darker blue where the shelf drops away.
        let depth = (-terrain.shore(q) / 15.0).clamp(0.0, 1.0).powf(0.7);
        let jitter = (unit(seed) - 0.5) * 9.0;
        std::array::from_fn(|i| (SHALLOW[i] + (DEEP[i] - SHALLOW[i]) * depth + jitter) as u8)
    };
    // Fine facets along the coast, broad ones on the open sea where they read the same.
    let quad = |b: &mut Batch, p: Vec2, size: f32, seed: u32| {
        let corner = |dx: f32, dz: f32| v(p.x + dx * size, -1.18, p.y + dz * size);
        b.face(
            &[corner(0.0, 0.0), corner(0.0, 1.0), corner(1.0, 1.0)],
            tint(p + Vec2::new(0.33, 0.66) * size, seed),
        );
        b.face(
            &[corner(0.0, 0.0), corner(1.0, 1.0), corner(1.0, 0.0)],
            tint(p + Vec2::new(0.66, 0.33) * size, seed + 1),
        );
    };
    let blocks = span / 3 + 1;
    for row in -blocks..blocks {
        for col in -blocks..blocks {
            let corner = Vec2::new(col as f32, row as f32) * CELL * 3.0;
            let seed = (row + 400) as u32 * 7919 + (col + 400) as u32 * 104_729;
            if terrain.shore(corner + Vec2::splat(CELL * 1.5)) < -(CELL * 2.2 + 13.0) {
                quad(&mut b, corner, CELL * 3.0, seed);
                continue;
            }
            for k in 0..9 {
                let p = corner + Vec2::new((k % 3) as f32, (k / 3) as f32) * CELL;
                if terrain.shore(p + Vec2::splat(CELL * 0.5)) > CELL * 1.2 {
                    continue;
                }
                quad(&mut b, p, CELL, seed + k * 2);
            }
        }
    }
    b.mesh()
}

/// Two bands of surf that follow the coastline and ride on the swell.
fn surf(terrain: &Terrain) -> Mesh {
    let mut b = Batch::default();
    let steps = (terrain.radius * 4.2) as usize;
    let rim = |angle: f32, inset: f32| {
        let direction = Vec2::from_angle(angle);
        let radius = terrain.shore(direction) + 1.0;
        let p = direction * (radius - inset);
        (p, direction)
    };
    for (lane, inset, width, tint, y) in [
        (0_u32, 0.85, 0.62, [226, 244, 236], -1.145),
        (1, -0.15, 0.24, [176, 228, 220], -1.152),
    ] {
        for i in 0..steps {
            let seed = i as u32 * 31 + lane * 977 + terrain.level.index as u32 * 13;
            if lane == 1 && unit(seed) < 0.38 {
                continue;
            }
            let a0 = i as f32 / steps as f32 * std::f32::consts::TAU;
            let a1 = (i + 1) as f32 / steps as f32 * std::f32::consts::TAU;
            let wobble = |s: u32| (unit(s) - 0.5) * 0.16;
            let (p0, d0) = rim(a0, inset + wobble(seed));
            let (p1, d1) = rim(a1, inset + wobble(seed + 1));
            let half = width * (0.75 + unit(seed + 2) * 0.5) * 0.5;
            let quad = [
                p0 - d0 * half,
                p1 - d1 * half,
                p1 + d1 * half,
                p0 + d0 * half,
            ]
            .map(|q| v(q.x, y, q.y));
            b.face(&quad, shift(tint, (unit(seed + 3) * 10.0) as i16 - 5));
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
    let region = terrain.level.region;
    let seed = terrain.level.index as u32 * 8763 + 431;
    for (i, p, choice, size) in terrain.vegetation() {
        let height = terrain.height(p);
        if height > 3.4 || !(0.44..=0.75).contains(&choice) {
            continue;
        }
        let b = &mut patches[patch(p)];
        let at = v(p.x, height, p.y);
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
    // Wildflowers, tall grass and pebbles, densest around the settlement.
    let lanes = Lanes::new(terrain);
    let blossoms: [Tint; 4] = [
        [246, 240, 222],
        [244, 205, 92],
        colors.flag,
        [226, 146, 168],
    ];
    for i in 0..(560 + terrain.level.stage as u32 * 36) {
        let n = seed ^ (i * 2_654_435_761);
        let near = unit(n) < 0.62;
        let p = if near {
            let a = unit(n + 1) * std::f32::consts::TAU;
            let r = 3.5 + unit(n + 2).sqrt() * (lanes.depth + 9.0);
            Vec2::new(a.cos() * r * 1.25, a.sin() * r)
        } else {
            Vec2::new(unit(n + 1) * 2.0 - 1.0, unit(n + 2) * 2.0 - 1.0) * terrain.radius
        };
        if terrain.shore(p) < 2.6
            || p.length() < 4.0
            || lanes.distance(p) < 0.75
            || terrain.height(p) > 3.2
            || terrain
                .plots
                .iter()
                .any(|plot| (plot.x - p.x).abs() < 1.9 && (plot.y - p.y).abs() < 1.7)
            || terrain.deposits.iter().any(|d| d.at.distance(p) < 1.0)
        {
            continue;
        }
        let b = &mut patches[patch(p)];
        let at = v(p.x, terrain.height(p) + 0.01, p.y);
        let kind = unit(n + 3);
        if kind < 0.5 {
            let color = blossoms[(unit(n + 4) * 4.0) as usize % 4];
            for k in 0..(4 + (unit(n + 5) * 4.0) as u32) {
                let m = n + 40 + k * 3;
                let q = at + v((unit(m) - 0.5) * 1.0, 0.0, (unit(m + 1) - 0.5) * 0.8);
                let h = 0.12 + unit(m + 2) * 0.14;
                // Few triangles per blossom: these fill whole meadows on phones too.
                let spin = unit(m + 3) * std::f32::consts::TAU;
                let leaf = v(spin.cos(), 0.0, spin.sin()) * 0.13;
                b.face(
                    &[q - leaf, q + leaf, q + Vec3::Y * (h * 0.8)],
                    shift(colors.leaves, -4),
                );
                // A tiny top ring keeps every side face a proper quad with a normal.
                b.frustum(
                    q + Vec3::Y * (h - 0.03),
                    0.12,
                    0.02,
                    0.08,
                    4,
                    shift(color, (k as i16) * 3),
                );
            }
        } else if kind < 0.82 {
            let tint = shift(colors.leaves, 4 + (unit(n + 4) * 20.0) as i16);
            for k in 0..6 {
                let a = k as f32 / 6.0 * std::f32::consts::TAU + unit(n + 5);
                let lean = v(a.cos() * 0.17, 0.0, a.sin() * 0.17);
                let h = 0.30 + unit(n + 6 + k) * 0.22;
                b.face(
                    &[
                        at + v(a.sin() * 0.07, 0.0, -a.cos() * 0.07),
                        at - v(a.sin() * 0.07, 0.0, -a.cos() * 0.07),
                        at + lean + Vec3::Y * h,
                    ],
                    shift(tint, k as i16 * 2),
                );
            }
        } else {
            for k in 0..(1 + (unit(n + 4) * 2.0) as u32) {
                let m = n + 70 + k * 5;
                let r = 0.10 + unit(m) * 0.13;
                b.block(
                    at + v(
                        (unit(m + 1) - 0.5) * 0.6,
                        r * 0.2,
                        (unit(m + 2) - 0.5) * 0.6,
                    ),
                    v(r * 1.3, r * 0.7, r),
                    Quat::from_euler(EulerRot::YXZ, m as f32, 0.25, 0.18),
                    shift(colors.rock, (unit(m + 3) * 24.0) as i16 - 6),
                );
            }
        }
    }
    patches
        .into_iter()
        .map(Batch::mesh)
        .filter(|mesh| mesh.count_vertices() > 0)
}

fn scenery_model(
    region: Region,
    kind: Good,
    seed: u32,
    size: f32,
    height: f32,
    colors: Palette,
) -> Mesh {
    let mut b = Batch::default();
    let at = Vec3::ZERO;
    if kind == Good::Stone {
        b.gem(
            Vec3::Y * size * 0.14,
            size * 0.62,
            size * 0.37,
            size * 0.56,
            seed as f32,
            shift(colors.rock, (seed % 23) as i16 - 9),
        );
    } else {
        match region {
            Region::Oasis => super::oasis::palm(&mut b, at, size, seed as f32 * 0.61),
            Region::Greece if seed.is_multiple_of(3) => super::buildings::olive(&mut b, at, size),
            Region::Rome if seed.is_multiple_of(3) => super::buildings::cypress(&mut b, at, size),
            _ => conifer(
                &mut b,
                at,
                size,
                colors,
                region == Region::Nordic && height > 2.0,
            ),
        }
    }
    b.mesh()
}

fn harvest_tool(axe: bool) -> Mesh {
    let mut b = Batch::default();
    b.beam(v(0., -0.27, 0.), v(0., 0.33, 0.), 0.052, [132, 86, 51]);
    if axe {
        b.slab(v(0.11, 0.28, 0.), v(0.27, 0.21, 0.066), [107, 133, 136]);
        b.slab(v(0.23, 0.28, 0.), v(0.041, 0.23, 0.070), [171, 192, 183]);
    } else {
        b.beam(
            v(-0.22, 0.26, 0.),
            v(0.16, 0.34, 0.),
            0.068,
            [110, 135, 142],
        );
        b.beam(v(0.16, 0.34, 0.), v(0.30, 0.24, 0.), 0.046, [167, 185, 182]);
    }
    b.mesh()
}

#[allow(clippy::type_complexity)]
pub(crate) fn fishing_visuals(
    game: Res<Game>,
    adventure: Res<Adventure>,
    mut vessels: Query<(&FishingBoat, &mut Transform, &mut Visibility)>,
    mut schools: Query<(&FishSchool, &mut Transform, &mut Visibility), Without<FishingBoat>>,
) {
    let Some((terrain, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    let phase = game.clock;
    for (school, mut pose, mut visible) in &mut schools {
        let left = island.fish_left.get(school.0).copied().unwrap_or(0);
        let state = if left > 0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visible != state {
            *visible = state;
        }
        if left > 0 {
            pose.rotation = Quat::from_rotation_y(phase * 0.34);
            pose.translation.y = -1.071 + (phase * 3.2).sin() * 0.045;
        }
    }
    for (vessel, mut pose, mut visible) in &mut vessels {
        let home = terrain.fishing[vessel.0];
        let batch = island
            .batch(home.lot)
            .filter(|batch| batch.fishing.is_some() && !batch.paused)
            .or_else(|| {
                island
                    .lots
                    .iter()
                    .enumerate()
                    .filter(|(lot, _)| !terrain.level.coastal(*lot))
                    .find_map(|(lot, _)| {
                        island
                            .batch(lot)
                            .filter(|batch| batch.fishing == Some(vessel.0) && !batch.paused)
                    })
            });
        let Some(batch) = batch else {
            *visible = Visibility::Hidden;
            continue;
        };
        let place = terrain.fishing[batch.fishing.unwrap()];
        let progress = batch.progress();
        let outbound = ((progress - 0.09) / 0.30).clamp(0.0, 1.0);
        let inbound = ((progress - 0.62) / 0.30).clamp(0.0, 1.0);
        let at = home.launch.lerp(place.at, outbound * (1.0 - inbound));
        let goal = if progress < 0.62 {
            place.at - home.launch
        } else {
            home.launch - place.at
        };
        pose.translation = v(at.x, -1.10 + (phase * 2.8).sin() * 0.075, at.y);
        pose.rotation = Quat::from_euler(
            EulerRot::YXZ,
            goal.x.atan2(goal.y),
            (phase * 2.1).sin() * 0.065,
            (phase * 3.0).cos() * 0.084,
        );
        *visible = Visibility::Inherited;
    }
}

fn workboat() -> Mesh {
    let mut boat = Batch::default();
    let oak = [124, 76, 47];
    let gunwale = [195, 138, 84];
    let shape = [(-1.34, 0.035), (-0.86, 0.42), (0.64, 0.45), (1.30, 0.04)];
    for pair in shape.windows(2) {
        let [(z, w), (next, wide)] = [pair[0], pair[1]];
        for side in [-1.0, 1.0] {
            let a = v(w * side, 0.23, z);
            let b = v(wide * side, 0.23, next);
            let c = v(wide * side * 0.48, 0.015, next * 0.91);
            let d = v(w * side * 0.48, 0.015, z * 0.91);
            boat.face(&[a, b, c, d], oak);
            boat.face(&[d, c, b, a], shift(oak, 12));
            boat.beam(a, b, 0.052, gunwale);
        }
    }
    boat.slab(v(0., 0.086, 0.), v(0.64, 0.067, 1.66), gunwale);
    for z in [-0.59, 0.38] {
        boat.slab(v(0., 0.223, z), v(0.76, 0.063, 0.19), [156, 98, 58]);
    }
    boat.beam(
        v(0.33, 0.19, -0.18),
        v(0.92, 0.09, -0.75),
        0.047,
        [191, 131, 76],
    );
    boat.slab(v(0.91, 0.078, -0.76), v(0.31, 0.036, 0.16), [132, 89, 60]);
    boat.frustum(v(0.08, 0.20, 0.37), 0.18, 0.23, 0.26, 9, [175, 134, 82]);
    boat.frustum(v(-0.07, 0.20, -0.33), 0.17, 0.19, 0.42, 7, [95, 127, 142]);
    boat.gem(
        v(-0.07, 0.72, -0.31),
        0.111,
        0.139,
        0.112,
        0.2,
        [219, 168, 119],
    );
    boat.frustum(v(-0.07, 0.75, -0.31), 0.128, 0.068, 0.10, 8, [132, 100, 62]);
    boat.mesh()
}

fn school_model() -> Mesh {
    let mut fish = Batch::default();
    for i in 0..7 {
        let angle = i as f32 * 2.399;
        let p = v(
            angle.cos() * (1.1 + (i % 3) as f32 * 0.16),
            (i % 3) as f32 * 0.027,
            angle.sin() * 0.91,
        );
        let tint = shift([121, 185, 186], i * 6 - 10);
        fish.gem(p, 0.14, 0.076, 0.25, angle, tint);
        let tail = p + v(angle.sin() * -0.25, 0., angle.cos() * -0.25);
        fish.face(
            &[p, tail + v(0.025, 0.068, 0.), tail - v(0.03, 0.057, 0.)],
            [192, 215, 198],
        );
        fish.beam(
            p + v(-0.28, 0.01, 0.24),
            p + v(0.27, 0.01, 0.28),
            0.021,
            [175, 222, 218],
        );
    }
    fish.mesh()
}

fn spent_model(region: Region, kind: Good) -> Mesh {
    let mut mesh = Batch::default();
    let colors = palette(region);
    match kind {
        Good::Wood => {
            mesh.frustum(Vec3::ZERO, 0.35, 0.26, 0.22, 9, colors.wood);
            mesh.frustum(v(0., 0.22, 0.), 0.245, 0.238, 0.027, 9, [202, 153, 91]);
            mesh.frustum(v(0., 0.247, 0.), 0.148, 0.143, 0.011, 9, [225, 178, 111]);
            mesh.frustum(
                v(0.012, 0.258, -0.016),
                0.065,
                0.058,
                0.008,
                8,
                [178, 123, 78],
            );
            for angle in [0.28_f32, 2.41, 4.55] {
                let root = v(angle.cos(), 0., angle.sin());
                mesh.beam(
                    root * 0.22 + v(0., 0.08, 0.),
                    root * 0.62 + v(0., 0.025, 0.),
                    0.077,
                    colors.wood,
                );
            }
        }
        Good::Stone | Good::Ore => {
            let stone = if kind == Good::Ore {
                [100, 105, 110]
            } else {
                colors.rock
            };
            for (i, (x, z, size)) in [
                (0.22, 0.00, 0.38),
                (-0.23, -0.07, 0.30),
                (0.01, 0.47, 0.13),
                (-0.44, 0.31, 0.085),
                (0.45, -0.35, 0.092),
            ]
            .into_iter()
            .enumerate()
            {
                mesh.gem(
                    v(x, size * 0.24, z),
                    size,
                    size * 0.46,
                    size * 0.68,
                    i as f32,
                    shift(stone, i as i16 * 7 - 14),
                );
            }
        }
        Good::Clay => {
            mesh.frustum(Vec3::ZERO, 0.73, 0.62, 0.052, 10, [144, 83, 64]);
            mesh.frustum(v(0., 0.052, 0.), 0.49, 0.45, 0.018, 10, [105, 64, 52]);
            for (x, z) in [(-0.51, 0.14), (0.31, -0.47), (0.56, 0.27)] {
                mesh.gem(v(x, 0.066, z), 0.17, 0.11, 0.14, x, [176, 105, 74]);
            }
        }
        Good::Local if region == Region::Japan => {
            for (x, z, h) in [
                (-0.27, 0.03, 0.19),
                (-0.06, -0.21, 0.27),
                (0.17, 0.13, 0.16),
                (0.36, -0.15, 0.14),
            ] {
                mesh.frustum(v(x, 0., z), 0.056, 0.047, h, 7, [126, 153, 86]);
                mesh.frustum(v(x, h, z), 0.037, 0.035, 0.011, 7, [218, 202, 123]);
            }
        }
        Good::Local if region == Region::Nordic => {
            mesh.frustum(Vec3::ZERO, 0.64, 0.53, 0.045, 8, [135, 123, 104]);
            for (x, z) in [(-0.43, 0.19), (0.42, 0.22), (0.2, -0.43)] {
                mesh.gem(v(x, 0.055, z), 0.12, 0.069, 0.093, z, [112, 116, 110]);
            }
        }
        _ => {
            mesh.frustum(Vec3::ZERO, 0.59, 0.55, 0.026, 9, shift(colors.wood, 14));
            for (x, z) in [(-0.31, 0.14), (0.16, -0.34), (0.32, 0.15), (-0.11, -0.17)] {
                mesh.beam(
                    v(x, 0.025, z),
                    v(x + 0.057, 0.17, z - 0.067),
                    0.035,
                    shift(colors.leaves, -13),
                );
                mesh.beam(
                    v(x, 0.034, z),
                    v(x - 0.14, 0.066, z + 0.11),
                    0.022,
                    [181, 146, 91],
                );
            }
        }
    }
    mesh.mesh()
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
        Good::Food if matches!(region, Region::Greece | Region::Rome | Region::Japan) => {
            let rice = region == Region::Japan;
            b.frustum(
                at,
                0.74,
                0.69,
                0.043,
                9,
                if rice {
                    [112, 165, 139]
                } else {
                    [159, 118, 71]
                },
            );
            for i in 0..22 {
                let angle = i as f32 * 2.399;
                let radius = 0.62 * (i as f32 / 22.0).sqrt();
                let root = v(radius * angle.cos(), 0.065, radius * angle.sin());
                let h = 0.25 + unit(i * 121 + 19) * 0.18;
                let top = root + v(-0.034, h, 0.044);
                b.beam(
                    root,
                    top,
                    0.023,
                    if rice { [113, 158, 77] } else { [169, 140, 63] },
                );
                b.gem(
                    top,
                    0.045,
                    0.082,
                    0.057,
                    angle,
                    if rice {
                        [201, 190, 106]
                    } else {
                        [226, 182, 83]
                    },
                );
            }
        }
        Good::Food if region == Region::Oasis => {
            super::oasis::palm(&mut b, at, 0.75, 0.79);
            super::oasis::palm(&mut b, v(0.46, 0.025, 0.41), 0.5, 1.44);
            for (x, z) in [(-0.20, 0.34), (0.21, 0.37), (0.40, -0.20), (-0.41, 0.06)] {
                b.gem(v(x, 0.08, z), 0.09, 0.065, 0.075, x, [149, 81, 46]);
            }
        }
        Good::Local if region == Region::Greece => {
            super::buildings::olive(&mut b, at, 0.68);
            for i in 0..16 {
                let a = i as f32 * 2.399;
                b.gem(
                    v(a.cos() * 0.31, 0.76 + unit(i * 527) * 0.25, a.sin() * 0.26),
                    0.05,
                    0.067,
                    0.041,
                    a,
                    [92, 76, 62],
                );
            }
        }
        Good::Local if region == Region::Japan => {
            for i in 0..7 {
                let a = i as f32 * 2.399;
                let base = v(a.cos() * 0.35, 0.03, a.sin() * 0.35);
                let h = 0.78 + unit(i * 621 + 45) * 0.75;
                b.frustum(base, 0.057, 0.044, h, 7, [109, 155, 82]);
                for ring in 1..4 {
                    let y = h * ring as f32 / 4.2;
                    b.frustum(base + Vec3::Y * y, 0.056, 0.053, 0.018, 7, [161, 174, 100]);
                    let p = base + Vec3::Y * y;
                    b.face(
                        &[
                            p,
                            p + v(a.cos() * 0.22, 0.15, -0.075),
                            p + v(a.cos() * 0.13, 0.042, 0.081),
                        ],
                        [101, 145, 74],
                    );
                }
            }
        }
        Good::Local if region == Region::Nordic => {
            b.frustum(at, 0.61, 0.56, 0.06, 9, [140, 133, 106]);
            for i in 0..6 {
                let a = i as f32 * 2.399;
                b.gem(
                    v(a.cos() * 0.29, 0.11, a.sin() * 0.31),
                    0.12,
                    0.083,
                    0.095,
                    a,
                    shift([214, 137, 67], i * 6 - 12),
                );
            }
        }
        Good::Local if region == Region::Oasis => {
            for i in 0..15 {
                let a = i as f32 * 2.399;
                let p = v(a.cos() * 0.49, 0.03, a.sin() * 0.49);
                let h = 0.19 + unit(i * 827 + 150) * 0.13;
                b.beam(p, p + Vec3::Y * h, 0.024, [110, 145, 73]);
                b.frustum(p + Vec3::Y * h, 0.026, 0.091, 0.071, 6, [167, 131, 180]);
                b.gem(
                    p + Vec3::Y * (h + 0.078),
                    0.04,
                    0.035,
                    0.039,
                    a,
                    [188, 88, 66],
                );
            }
        }
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
    if kind == Build::Monument {
        return super::monuments::landmark(region, tier);
    }
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
    fn island_scenery_stays_within_a_phone_vertex_budget() {
        for index in 0..crate::adventure::LEVELS {
            let terrain = Terrain::new(crate::adventure::Level::at(index));
            let colors = palette(terrain.level.region);
            let ground = landscape(&terrain, colors).count_vertices();
            let plants: usize = woods(&terrain, colors).map(|m| m.count_vertices()).sum();
            let water = sea(&terrain).count_vertices() + surf(&terrain).count_vertices();
            let total = ground + plants + water;
            assert!(
                total < 200_000,
                "{}: {ground} Boden, {plants} Pflanzen, {water} Wasser",
                terrain.level.name
            );
        }
    }

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

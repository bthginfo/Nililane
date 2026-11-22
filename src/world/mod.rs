mod actors;
mod buildings;
mod construction;
mod landscape;
mod mesh;
mod motion;
pub(crate) use motion::stop as route_position;
pub(crate) use work::animate as animate_work;
mod people;
mod roman;
mod ships;
mod sky;
mod upgrades;
mod work;

use std::collections::HashMap;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::{camera::ScalingMode, mesh::skinning::SkinnedMeshInverseBindposes},
    window::PrimaryWindow,
};

use crate::{
    campaign::{CHAPTERS, Region},
    errands::{Job, Stop},
    game::{Game, View},
    layout::{Layout, lot_col, lot_row},
    model::{Kind, Phase, Weather},
    save::Campaign,
};

use self::{
    buildings::GOLD,
    construction::{Assembly, Part, Parts},
    mesh::{Batch, v},
};

pub const LOT_STEP: f32 = 3.20;

pub fn lot_position(lot: usize, columns: usize) -> Vec3 {
    v(
        lot_col(lot) as f32 * LOT_STEP - (columns as f32 - 1.0) * LOT_STEP * 0.5,
        0.065,
        if lot_row(lot) == 0 { -2.34 } else { 2.38 },
    )
}

#[derive(Component)]
pub struct StreetCamera;
#[derive(Component)]
pub(crate) struct Sun;
#[derive(Component)]
pub(crate) struct Island;
#[derive(Component)]
pub(crate) struct Parcels;
#[derive(Component)]
pub(crate) struct Selected;
#[derive(Component)]
pub(crate) struct Hippo;
#[derive(Component)]
pub(crate) struct JobPack(Job);
#[derive(Component)]
pub(crate) struct Destination;
#[derive(Component)]
pub(crate) struct Limb {
    side: f32,
    leg: bool,
}
#[derive(Component)]
pub(crate) struct Wave(usize);
#[derive(Component)]
pub(crate) struct Boat(usize);
#[derive(Component)]
pub(crate) struct Sail(usize);
#[derive(Component)]
pub(crate) struct Wake(usize);
#[derive(Component)]
pub(crate) struct Gangway;
#[derive(Component)]
pub(crate) struct Bird(usize);

#[derive(Resource)]
pub struct Art {
    matte: Handle<StandardMaterial>,
    sea: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    sea_weather: Option<Weather>,
    gold: Handle<StandardMaterial>,
    kinds: HashMap<(Region, Kind, u8), Handle<Mesh>>,
    additions: HashMap<(Region, Kind, u8, u8), Handle<Mesh>>,
    work: Parts,
    hulls: [[Handle<Mesh>; 3]; 2],
    sails: [[Handle<Mesh>; 3]; 2],
    house: Handle<Scene>,
    store: Handle<Scene>,
    tower: Handle<Scene>,
    pine: Handle<Scene>,
    stone: Handle<Scene>,
    chapter: Option<usize>,
    world_version: Option<u32>,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    mut bind_poses: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    commands
        .spawn((
            StreetCamera,
            Camera3d::default(),
            Tonemapping::AcesFitted,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 20.0,
                },
                near: 0.1,
                far: 160.0,
                ..OrthographicProjection::default_3d()
            }),
            if cfg!(target_os = "android") {
                Msaa::Off
            } else {
                Msaa::Sample4
            },
            Transform::from_xyz(4.0, 19.0, 28.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert_if(bevy::core_pipeline::fxaa::Fxaa::default(), || {
            cfg!(target_os = "android")
        });
    commands.insert_resource(DirectionalLightShadowMap {
        size: if cfg!(target_os = "android") {
            1024
        } else {
            2048
        },
    });
    commands.spawn((
        Sun,
        DirectionalLight {
            color: Color::srgb_u8(255, 234, 197),
            illuminance: 11_500.0,
            shadows_enabled: true,
            shadow_depth_bias: 0.18,
            shadow_normal_bias: 1.0,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            minimum_distance: 18.0,
            maximum_distance: 58.0,
            ..default()
        }
        .build(),
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -0.58, -0.81, 0.0)),
    ));

    let matte = materials.add(StandardMaterial {
        perceptual_roughness: 0.91,
        ..default()
    });
    let sea = materials.add(StandardMaterial {
        unlit: true,
        ..default()
    });
    let gold = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(255, 226, 155),
        unlit: true,
        ..default()
    });
    let scene =
        |path: &'static str, index| assets.load(GltfAssetLabel::Scene(index).from_asset(path));
    let hulls = [Region::Greece, Region::Rome]
        .map(|region| std::array::from_fn(|i| meshes.add(ships::hull(region, i))));
    let sails = [Region::Greece, Region::Rome]
        .map(|region| std::array::from_fn(|i| meshes.add(ships::sail(region, i))));
    let art = Art {
        matte,
        sea,
        water: materials.add(StandardMaterial {
            unlit: true,
            ..default()
        }),
        sea_weather: None,
        gold,
        hulls,
        sails,
        kinds: HashMap::new(),
        additions: HashMap::new(),
        work: Parts::prepare(&mut meshes, &mut materials),
        house: scene("embedded://models/villages/aegean.glb", 0),
        store: scene("embedded://models/villages/aegean.glb", 1),
        tower: scene("embedded://models/works/aegean.glb", 0),
        pine: scene("embedded://models/waldwinkel/wood_tree.glb", 0),
        stone: scene("embedded://models/waldwinkel/resource_rock.glb", 0),
        chapter: None,
        world_version: None,
    };
    let mut highlight = Batch::default();
    for x in [-1.41, 1.41] {
        for z in [-1.19, 1.19] {
            let p = v(x, 0.036, z);
            highlight.slab(p, v(0.38, 0.075, 0.088), GOLD);
            highlight.slab(p, v(0.077, 0.075, 0.32), GOLD);
        }
    }
    highlight.gem(v(0.0, 0.35, 1.29), 0.13, 0.17, 0.13, 0.33, GOLD);
    commands.spawn((
        Selected,
        Mesh3d(meshes.add(highlight.mesh())),
        MeshMaterial3d(art.gold.clone()),
        Transform::default(),
        Visibility::Hidden,
        bevy::pbr::NotShadowCaster,
    ));

    commands.spawn((
        Destination,
        Mesh3d(meshes.add(actors::destination())),
        MeshMaterial3d(art.gold.clone()),
        Transform::default(),
        Visibility::Hidden,
        bevy::pbr::NotShadowCaster,
    ));

    work::spawn(&mut commands, &mut meshes, &art.gold);
    commands
        .spawn((
            Hippo,
            Transform::from_scale(Vec3::splat(0.87)),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn(SceneRoot(scene("embedded://models/hippo.glb", 0)));
            for job in Job::ALL {
                parent.spawn((
                    JobPack(job),
                    Mesh3d(meshes.add(actors::pack(job))),
                    MeshMaterial3d(art.matte.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                    bevy::pbr::NotShadowCaster,
                ));
            }
            for side in [-1.0, 1.0] {
                for leg in [false, true] {
                    let (pivot, geometry) = actors::hippo_limb(side, leg);
                    parent.spawn((
                        Limb { side, leg },
                        Mesh3d(meshes.add(geometry)),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(pivot),
                    ));
                }
            }
        });
    people::stage(
        &mut commands,
        &mut meshes,
        &mut bind_poses,
        &mut materials,
        &art.matte,
    );
    let bird = meshes.add(actors::bird());
    for i in 0..4 {
        commands.spawn((
            Bird(i),
            Mesh3d(bird.clone()),
            MeshMaterial3d(art.sea.clone()),
            Transform::default(),
            bevy::pbr::NotShadowCaster,
        ));
    }
    for i in 0..3 {
        commands
            .spawn((
                Boat(i),
                Mesh3d(art.hulls[0][i].clone()),
                MeshMaterial3d(art.matte.clone()),
                Transform::default(),
                bevy::pbr::NotShadowCaster,
            ))
            .with_children(|boat| {
                boat.spawn((
                    Sail(i),
                    Mesh3d(art.sails[0][i].clone()),
                    MeshMaterial3d(art.matte.clone()),
                    Transform::default(),
                    bevy::pbr::NotShadowCaster,
                ));
            });
        commands.spawn((
            Wake(i),
            Mesh3d(meshes.add(ships::wake(i))),
            MeshMaterial3d(art.sea.clone()),
            Transform::default(),
            bevy::pbr::NotShadowCaster,
        ));
    }
    commands.spawn((
        Gangway,
        Mesh3d(meshes.add(ships::gangway())),
        MeshMaterial3d(art.matte.clone()),
        Transform::default(),
        Visibility::Hidden,
        bevy::pbr::NotShadowCaster,
    ));
    commands.insert_resource(art);
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn sync(
    mut commands: Commands,
    game: Res<Game>,
    save: Res<Campaign>,
    mut art: ResMut<Art>,
    mut meshes: ResMut<Assets<Mesh>>,
    terrain: Query<Entity, With<Island>>,
    parcels: Query<Entity, With<Parcels>>,
    mut props: Query<(Option<&Boat>, Option<&Sail>, &mut Mesh3d), Or<(With<Boat>, With<Sail>)>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let town = game.visible_town(&save);
    let ch = &CHAPTERS[town.chapter];
    let cols = ch.layout.width();
    let region = Region::of(town.chapter);
    let weather = if game.view == View::Street {
        town.weather(ch)
    } else {
        Weather::Clear
    };
    if art.sea_weather != Some(weather) {
        art.sea_weather = Some(weather);
        if let Some(material) = materials.get_mut(&art.water) {
            material.base_color = match weather {
                Weather::Storm => Color::srgb(0.67, 0.75, 0.80),
                Weather::Rain => Color::srgb(0.78, 0.85, 0.88),
                Weather::Cloud => Color::srgb(0.91, 0.95, 0.96),
                Weather::Heat => Color::srgb(0.95, 1.0, 0.97),
                _ => Color::WHITE,
            };
        }
    }
    if art.chapter != Some(town.chapter) {
        if art
            .chapter
            .is_none_or(|chapter| Region::of(chapter) != region)
        {
            for (boat, sail, mut mesh) in &mut props {
                if let Some(boat) = boat {
                    mesh.0 = art.hulls[region as usize][boat.0].clone();
                } else if let Some(sail) = sail {
                    mesh.0 = art.sails[region as usize][sail.0].clone();
                }
            }
        }
        for entity in &terrain {
            commands.entity(entity).despawn();
        }
        let mut root = commands.spawn((Island, Transform::default(), Visibility::default()));
        root.with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(landscape::terrain(ch.layout, town.chapter))),
                MeshMaterial3d(art.matte.clone()),
            ));
            parent.spawn((
                Mesh3d(meshes.add(landscape::water(cols, region))),
                MeshMaterial3d(art.water.clone()),
                bevy::pbr::NotShadowCaster,
            ));
            parent.spawn((
                Mesh3d(meshes.add(landscape::islands(town.chapter))),
                MeshMaterial3d(art.sea.clone()),
                bevy::pbr::NotShadowCaster,
            ));
            for i in 0..2 {
                parent.spawn((
                    Wave(i),
                    Mesh3d(meshes.add(landscape::foam(cols, i))),
                    MeshMaterial3d(art.sea.clone()),
                    Transform::default(),
                    bevy::pbr::NotShadowCaster,
                ));
            }
            if region == Region::Greece {
                let bank = cols as f32 * LOT_STEP * 0.5;
                parent.spawn((
                    SceneRoot(art.tower.clone()),
                    Transform::from_xyz(bank + 0.83, 0.0, -2.97),
                ));
                for (x, z, scale) in [(-bank - 1.14, -4.49, 0.49), (bank - 0.65, -5.42, 0.42)] {
                    parent.spawn((
                        SceneRoot(art.pine.clone()),
                        Transform::from_xyz(x, 0.0, z).with_scale(Vec3::splat(scale)),
                    ));
                }
                for (x, z, scale) in [
                    (-bank - 1.32, 4.04, 0.71),
                    (-bank + 0.37, 4.54, 0.45),
                    (bank + 1.67, 3.52, 0.65),
                    (bank + 1.88, -3.3, 0.58),
                ] {
                    parent.spawn((
                        SceneRoot(art.stone.clone()),
                        Transform::from_xyz(x, -0.26, z).with_scale(Vec3::splat(scale)),
                    ));
                }
                parent.spawn((
                    SceneRoot(art.store.clone()),
                    Transform::from_xyz(-bank - 0.77, 0.06, -3.43).with_scale(Vec3::splat(0.55)),
                ));
            }
        });
        art.chapter = Some(town.chapter);
        art.world_version = None;
    }
    if art.world_version != Some(game.world_version) {
        for entity in &parcels {
            commands.entity(entity).despawn();
        }
        commands
            .spawn((Parcels, Transform::default(), Visibility::default()))
            .with_children(|parent| {
                for (i, building) in town.lots.iter().enumerate() {
                    let Some(building) = building else { continue };
                    let at = lot_position(i, cols);
                    let work = game.work[i].filter(|_| game.view == View::Street);
                    let before = work.map_or(building.level, |work| work.from);
                    let kind = building.kind;
                    if before > 0 {
                        let mesh = building_mesh(&mut art, &mut meshes, region, kind, before);
                        parent.spawn((
                            Mesh3d(mesh),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(at),
                        ));
                    }
                    if let Some(work) = work {
                        let mesh = if before == 0 {
                            building_mesh(&mut art, &mut meshes, region, kind, building.level)
                        } else {
                            art.additions
                                .entry((region, kind, before, building.level))
                                .or_insert_with(|| {
                                    meshes.add(construction::addition(
                                        region,
                                        kind,
                                        before,
                                        building.level,
                                    ))
                                })
                                .clone()
                        };
                        parent.spawn((
                            Mesh3d(mesh),
                            MeshMaterial3d(art.matte.clone()),
                            Transform::from_translation(at),
                            Assembly {
                                at,
                                work,
                                rest: 1.0,
                                part: Part::Structure,
                            },
                        ));
                        art.work.spawn(parent, at, work, &art.matte);
                    }
                    if kind == Kind::House && region == Region::Greece {
                        let at = at + v(0.0, 0.0, -0.24);
                        let mut shell = parent.spawn((
                            SceneRoot(art.house.clone()),
                            Transform::from_translation(at).with_scale(Vec3::splat(0.765)),
                        ));
                        if let Some(work) = work.filter(|w| w.from == 0) {
                            shell.insert(Assembly {
                                at,
                                work,
                                rest: 0.765,
                                part: Part::Structure,
                            });
                        }
                    }
                }
            });
        art.world_version = Some(game.world_version);
    }
}

fn building_mesh(
    art: &mut Art,
    meshes: &mut Assets<Mesh>,
    region: Region,
    kind: Kind,
    level: u8,
) -> Handle<Mesh> {
    art.kinds
        .entry((region, kind, level))
        .or_insert_with(|| {
            meshes.add(match region {
                Region::Greece => buildings::model(kind, level),
                Region::Rome => roman::model(kind, level),
            })
        })
        .clone()
}

pub use construction::animate as animate_construction;

type CameraRig<'w, 's> =
    Query<'w, 's, (&'static mut Transform, &'static mut Projection), With<StreetCamera>>;
type Extras<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Transform,
        &'static mut Visibility,
        Option<&'static Gangway>,
        Option<&'static Wave>,
        Option<&'static Boat>,
        Option<&'static Bird>,
        Option<&'static Sail>,
        Option<&'static Wake>,
    ),
    Or<(
        With<Wave>,
        With<Boat>,
        With<Bird>,
        With<Sail>,
        With<Wake>,
        With<Gangway>,
    )>,
>;

pub use people::{Reactions, animate as animate_people};
pub use sky::{animate as animate_weather, setup as setup_weather};
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn animate(
    game: Res<Game>,
    save: Res<Campaign>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut rigs: ParamSet<(
        CameraRig,
        Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
        Query<(&mut Transform, &mut Visibility), With<Selected>>,
        Query<&mut Transform, With<Hippo>>,
        Extras,
        Query<(&mut Transform, &Limb)>,
        Query<(&mut Transform, &mut Visibility), With<Destination>>,
        Query<(&JobPack, &mut Transform, &mut Visibility)>,
    )>,
) {
    let town = game.visible_town(&save);
    let chapter = &CHAPTERS[town.chapter];
    let cols = chapter.layout.width();
    let quay = landscape::quay(chapter.layout);
    let t = game.clock;
    let playing = game.view == View::Street;
    let clock = match &town.phase {
        Phase::Plan => 0.09,
        Phase::Open(day) => 0.13 + (day.seconds / chapter.day_seconds()) * 0.74,
        Phase::Report(_) => 0.87,
    };
    let sunset = if playing {
        ((clock - 0.61) * 3.65).clamp(0.0, 1.0)
    } else {
        0.22
    };
    let weather = if playing {
        town.weather(chapter)
    } else {
        Weather::Clear
    };
    let bright = match weather {
        Weather::Heat => 1.08,
        Weather::Cloud => 0.86,
        Weather::Rain => 0.69,
        Weather::Storm => 0.53,
        _ => 1.0,
    };
    let swell = weather.swell();
    let blend = 1.0 - (-5.2 * time.delta_secs()).exp();
    {
        let mut rig = rigs.p0();
        let Ok((mut camera, mut lens)) = rig.single_mut() else {
            return;
        };
        let aspect = window.width() / window.height().max(1.0);
        let dims = if playing {
            (cols as f32 * LOT_STEP + 6.8) / (aspect * 0.75).max(0.75)
        } else {
            20.4
        };
        let height = dims.max(12.9) / (1.0 + if playing { game.zoom } else { 0.0 });
        if let Projection::Orthographic(ortho) = lens.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: height,
            };
        }
        let offset = if playing { 0.12 } else { -0.095 };
        let yaw = if playing { 0.13 } else { 0.19 };
        let look = v(yaw, 0.86, 1.15).normalize();
        let right = v(look.z, 0.0, -look.x).normalize();
        let width = height * aspect;
        let pan = if playing { game.pan } else { Vec2::ZERO };
        let mut framing = width * offset + pan.x;
        if game.view == View::Title {
            framing = framing.max(ships::berth(quay).dot(right) - width * 0.32);
        } else if playing && game.zoom > 0.04 && game.pan.x.abs() < 0.12 {
            let plot = lot_position(game.selected, cols).dot(right);
            framing = framing.clamp(plot - width * 0.17, plot + width * 0.32);
        }
        let target = right * framing + v(0.0, 0.38, 0.28 + pan.y);
        let eye = target + look * 36.0;
        camera.translation = camera.translation.lerp(eye, blend);
        let rotation = Transform::from_translation(eye)
            .looking_at(target, Vec3::Y)
            .rotation;
        camera.rotation = camera.rotation.slerp(rotation, blend);
    }
    if let Ok((mut transform, mut sun)) = rigs.p1().single_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            -0.79 + clock * 0.64,
            -1.01 + clock * 0.39,
            0.0,
        );
        sun.color = Color::srgb(1.0, 0.94 - sunset * 0.19, 0.83 - sunset * 0.25);
        sun.illuminance = (12_400.0 - sunset * 4_500.0) * bright;
    }
    ambient.brightness = (510.0 - sunset * 146.0) * (0.49 + bright * 0.51);
    let p = lot_position(game.selected, cols);
    if let Ok((mut transform, mut visible)) = rigs.p2().single_mut() {
        *visible = if playing {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        transform.translation = p + Vec3::Y * (t * 3.7).sin() * 0.017;
    }
    let lives = if let Phase::Open(day) = &town.phase {
        Some(day)
    } else {
        None
    };
    let route = lives.and_then(|day| day.route.map(|route| (day.seconds, route)));
    let pending = game
        .job
        .and_then(|job| job.plan(town, chapter, game.selected).ok());
    if let Ok((mut transform, mut visible)) = rigs.p6().single_mut() {
        let pin = route.map(|(_, route)| route).or(pending);
        if let Some(pin) = pin.filter(|_| playing) {
            *visible = Visibility::Inherited;
            transform.translation = motion::stop(pin.delivery, chapter.layout);
            transform.rotation = Quat::from_rotation_y((t * 2.6).sin() * 0.24);
        } else {
            *visible = Visibility::Hidden;
        }
    }
    let pose = if let Some((seconds, route)) = route {
        let (stage, at) = route.stage(seconds);
        work::perform(route.job, stage, at)
    } else {
        game.delivery()
            .map_or_else(work::Pose::default, |done| work::celebrate(t - done.since))
    };
    for (pack, mut transform, mut visible) in &mut rigs.p7() {
        let next = if route.is_some_and(|(_, route)| pack.0 == route.job) && pose.carry {
            let tilt = pose.pack_tilt;
            let rotation = Quat::from_euler(EulerRot::XYZ, tilt.x, tilt.y, tilt.z);
            transform.rotation = rotation;
            transform.translation =
                pose.pack_shift + work::PACK_CENTER - rotation * work::PACK_CENTER;
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visible != next {
            *visible = next;
        }
    }
    let mut gait = 0.0;
    if let Ok(mut hippo) = rigs.p3().single_mut() {
        hippo.scale = v(
            1.0 - 0.5 * pose.stretch,
            1.0 + pose.stretch,
            1.0 - 0.5 * pose.stretch,
        ) * 0.87;
        if let Some((seconds, route)) = route {
            let motion = motion::job_walk(route, seconds, chapter.layout);
            let beat = motion.distance * 8.8;
            gait = if motion.moving { beat.sin() } else { 0.0 };
            hippo.translation = motion.at + Vec3::Y * (0.047 * gait.abs() + pose.height - 0.033);
            let tilt =
                Quat::from_rotation_x(pose.pitch) * Quat::from_rotation_z(pose.roll + gait * 0.045);
            hippo.rotation = hippo
                .rotation
                .slerp(Quat::from_rotation_y(motion.yaw) * tilt, blend * 1.32);
        } else {
            let home = if playing {
                if let Some(day) = lives {
                    let dock = if day.route.is_none() && day.tally.routes_today() == 0 {
                        day.favor_at.map_or(day.nili_at, Stop::Parcel)
                    } else {
                        day.nili_at
                    };
                    let stop = motion::stop(dock, chapter.layout);
                    stop + v(0.18, -0.045, 0.09)
                } else {
                    v(
                        p.x + 0.47,
                        0.056,
                        if lot_row(game.selected) == 0 {
                            -0.32
                        } else {
                            0.79
                        },
                    )
                }
            } else {
                v((t * 0.29).sin() * 3.25, 0.052, -0.02)
            };
            let walk = home - hippo.translation;
            let moving = walk.length_squared() > 0.035;
            gait = (t * if moving { 10.4 } else { 4.1 }).sin() * if moving { 1.0 } else { 0.07 };
            let bob = gait.abs() * 0.058;
            hippo.translation = hippo
                .translation
                .lerp(home + Vec3::Y * (bob + pose.height), blend * 0.62);
            if pose.height > 0.02 {
                hippo.translation.y = home.y + pose.height;
            }
            let yaw = if moving { walk.x.atan2(walk.z) } else { 0.22 };
            let rot = Quat::from_rotation_y(yaw)
                * Quat::from_rotation_x(pose.pitch)
                * Quat::from_rotation_z(pose.roll + gait * 0.038);
            hippo.rotation = hippo.rotation.slerp(rot, blend);
        }
    }
    for (mut transform, limb) in &mut rigs.p5() {
        let amount = if limb.leg { 0.40 } else { -0.34 };
        let arm = if limb.leg {
            0.0
        } else {
            pose.arms[usize::from(limb.side > 0.0)]
        };
        let spread = if limb.leg {
            0.0
        } else {
            pose.spread * limb.side
        };
        transform.rotation =
            Quat::from_rotation_x(gait * limb.side * amount + arm) * Quat::from_rotation_z(spread);
    }
    let ferry_day = match &town.phase {
        Phase::Plan => 0.0,
        Phase::Open(day) => day.seconds / chapter.day_seconds(),
        Phase::Report(_) => 1.0,
    };
    let ferry = ships::ferry(quay, ferry_day, t, swell);
    for (mut transform, mut visible, gangway, wave, boat, bird, sail, wake) in &mut rigs.p4() {
        if gangway.is_some() {
            if let Some(bridge) = ferry.gangway {
                *transform = bridge;
            }
            let next = if ferry.gangway.is_some() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            if *visible != next {
                *visible = next;
            }
        }
        if let Some(wave) = wave {
            let a = t * (0.52 + swell * 0.54) + wave.0 as f32 * 2.4;
            transform.translation = v(
                a.sin() * (0.21 + swell * 0.28),
                landscape::WATER_LEVEL + 0.012 + wave.0 as f32 * 0.006,
                a.cos() * (0.16 + swell * 0.21),
            );
        }
        if let Some(n) = boat.map(|b| b.0).or_else(|| wake.map(|w| w.0)) {
            if n == 0 {
                if wake.is_some() {
                    *transform = ferry.wake;
                    let next = if ferry.gangway.is_some() {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    };
                    if *visible != next {
                        *visible = next;
                    }
                } else {
                    *transform = ferry.hull;
                }
                continue;
            }
            let (x, z, angle) = if n == 1 {
                let w = cols as f32 * 1.6;
                (-w * 0.58 + (t * 0.14).sin() * w * 0.44, 8.65, 0.22)
            } else {
                ((t * 0.093).sin() * 9.0, -9.36, -0.16)
            };
            let beat = t * (1.1 + swell) + n as f32 * 2.9;
            if wake.is_some() {
                transform.translation = v(x, landscape::WAKE_LEVEL, z);
                transform.rotation = Quat::from_rotation_y(angle);
                transform.scale = v(
                    0.86 + beat.sin() * 0.10 + swell * 0.32,
                    1.0,
                    0.79 + swell * 0.44,
                );
            } else {
                let dip = beat.sin() * (0.048 + swell * 0.125);
                transform.translation = v(x, -0.346 + dip, z);
                transform.rotation = Quat::from_rotation_y(angle)
                    * Quat::from_rotation_z(beat.cos() * (0.03 + swell * 0.095))
                    * Quat::from_rotation_x(beat.sin() * swell * 0.095);
                transform.scale = Vec3::splat(if n == 2 { 0.76 } else { 1.0 });
            }
        }
        if let Some(sail) = sail {
            let beat = t * (1.2 + swell * 2.2) + sail.0 as f32 * 1.3;
            transform.rotation = Quat::from_rotation_y(beat.sin() * (0.035 + swell * 0.11));
            transform.scale.z = 0.86 + swell * 0.32 + beat.sin() * 0.12;
            if sail.0 == 0 {
                let unfold = ((ferry_day - 0.39) / 0.095).clamp(0.0, 1.0);
                let billow = unfold * unfold * (3.0 - 2.0 * unfold);
                transform.scale.y = 0.22 + 0.78 * billow;
                transform.translation.y = ships::FERRY_SPAR * (1.0 - transform.scale.y);
            }
        }
        if let Some(bird) = bird {
            let a = t * 0.22 + bird.0 as f32 * 0.62;
            transform.translation = v(
                a.cos() * 7.8,
                3.71 + (a * 2.0).sin() * 0.43,
                a.sin() * 5.4 - 0.4,
            );
            transform.rotation =
                Quat::from_rotation_y(-a - 1.45) * Quat::from_rotation_z((t * 5.3).sin() * 0.12);
        }
    }
}

pub fn pick(
    camera: &Camera,
    transform: &GlobalTransform,
    position: Vec2,
    layout: Layout,
) -> Option<usize> {
    let cols = layout.width();
    let size = camera.logical_viewport_size()?;
    if position.x > size.x * 0.75 || position.y > size.y * 0.86 {
        return None;
    }
    let radius = if cols > 4 { 47.0 } else { 62.0 } * (size.x / 1280.0).clamp(0.55, 2.5);
    layout
        .lots()
        .filter_map(|lot| {
            let point = camera
                .world_to_viewport(transform, lot_position(lot, cols) + Vec3::Y * 0.53)
                .ok()?;
            let d = position.distance(point);
            (d <= radius).then_some((lot, d))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(lot, _)| lot)
}

mod actors;
mod buildings;
mod landscape;
mod mesh;
mod roman;
mod ships;
mod sky;

use std::collections::HashMap;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};

use crate::{
    campaign::{CHAPTERS, Region},
    game::{Game, View},
    model::{COLS, Kind, Origin, Phase, Weather},
    save::Campaign,
};

use self::{
    buildings::GOLD,
    mesh::{Batch, v},
};

pub const LOT_STEP: f32 = 3.20;

pub fn lot_position(lot: usize, columns: usize) -> Vec3 {
    v(
        (lot % COLS) as f32 * LOT_STEP - (columns as f32 - 1.0) * LOT_STEP * 0.5,
        0.065,
        if lot < COLS { -2.34 } else { 2.38 },
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
pub(crate) struct Limb {
    side: f32,
    leg: bool,
}
#[derive(Component)]
pub(crate) struct Resident(usize);
#[derive(Component)]
pub(crate) struct Receipt(usize);
#[derive(Component)]
pub(crate) struct Wave(usize);
#[derive(Component)]
pub(crate) struct Boat(usize);
#[derive(Component)]
pub(crate) struct Sail(usize);
#[derive(Component)]
pub(crate) struct Wake(usize);
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
    guests: [[Handle<Mesh>; 24]; 2],
    receipts: [Handle<Mesh>; 2],
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
) {
    commands.spawn((
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
    ));
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
            maximum_distance: 65.0,
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
    let guests = [Region::Greece, Region::Rome]
        .map(|region| std::array::from_fn(|i| meshes.add(actors::resident(i, region))));
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
        guests,
        receipts: [false, true].map(|served| meshes.add(actors::response_marker(served))),
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

    commands
        .spawn((
            Hippo,
            Transform::from_scale(Vec3::splat(0.87)),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn(SceneRoot(scene("embedded://models/hippo.glb", 0)));
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
    for i in 0..160 {
        commands.spawn((
            Resident(i),
            Mesh3d(art.guests[0][i % 24].clone()),
            MeshMaterial3d(art.matte.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    }
    for n in 0..12 {
        commands.spawn((
            Receipt(n),
            Mesh3d(art.receipts[1].clone()),
            MeshMaterial3d(art.sea.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    }
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
    mut props: Query<
        (Option<&Resident>, Option<&Boat>, Option<&Sail>, &mut Mesh3d),
        Or<(With<Resident>, With<Boat>, With<Sail>)>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let town = game.visible_town(&save);
    let ch = &CHAPTERS[town.chapter];
    let cols = ch.columns;
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
            for (resident, boat, sail, mut mesh) in &mut props {
                if let Some(resident) = resident {
                    mesh.0 = art.guests[region as usize][resident.0 % 24].clone();
                } else if let Some(boat) = boat {
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
                Mesh3d(meshes.add(landscape::terrain(cols, town.chapter))),
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
                    Transform::from_xyz(0.0, -0.43, 0.0),
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
                    let key = (region, building.kind, building.level);
                    let mesh = art
                        .kinds
                        .entry(key)
                        .or_insert_with(|| {
                            meshes.add(match region {
                                Region::Greece => buildings::model(building.kind, building.level),
                                Region::Rome => roman::model(building.kind, building.level),
                            })
                        })
                        .clone();
                    parent.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(at),
                    ));
                    if building.kind == Kind::House && region == Region::Greece {
                        parent.spawn((
                            SceneRoot(art.house.clone()),
                            Transform::from_translation(at + v(0.0, 0.0, -0.24))
                                .with_scale(Vec3::splat(0.74 + 0.025 * f32::from(building.level))),
                        ));
                    }
                }
            });
        art.world_version = Some(game.world_version);
    }
}

type CameraRig<'w, 's> =
    Query<'w, 's, (&'static mut Transform, &'static mut Projection), With<StreetCamera>>;
type Extras<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Transform,
        Option<&'static Wave>,
        Option<&'static Boat>,
        Option<&'static Bird>,
        Option<&'static Sail>,
        Option<&'static Wake>,
    ),
    Or<(With<Wave>, With<Boat>, With<Bird>, With<Sail>, With<Wake>)>,
>;

pub use sky::{animate as animate_weather, setup as setup_weather};
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn animate(
    game: Res<Game>,
    save: Res<Campaign>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    art: Res<Art>,
    mut rigs: ParamSet<(
        CameraRig,
        Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
        Query<(&mut Transform, &mut Visibility), With<Selected>>,
        Query<&mut Transform, With<Hippo>>,
        Query<(&mut Transform, &mut Visibility, &Resident)>,
        Extras,
        Query<(&mut Transform, &Limb)>,
        Query<(&mut Transform, &mut Visibility, &Receipt, &mut Mesh3d)>,
    )>,
) {
    let town = game.visible_town(&save);
    let chapter = &CHAPTERS[town.chapter];
    let cols = chapter.columns;
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
        let yaw = if playing {
            0.13
        } else {
            0.19 + (t * 0.067).sin() * 0.09
        };
        let look = v(yaw, 0.86, 1.15).normalize();
        let right = v(look.z, 0.0, -look.x).normalize();
        let target =
            right * (height * aspect * offset + game.pan.x) + v(0.0, 0.38, 0.28 + game.pan.y);
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
    let mut gait = 0.0;
    if let Ok(mut hippo) = rigs.p3().single_mut() {
        let home = if playing {
            v(
                p.x + 0.47,
                0.06,
                if game.selected < COLS { -0.57 } else { 1.20 },
            )
        } else {
            v((t * 0.29).sin() * 3.25, 0.052, -0.02)
        };
        let moving = hippo.translation.distance_squared(home) > 0.035;
        gait = if moving {
            (t * 10.4).sin()
        } else {
            (t * 4.1).sin() * 0.12
        };
        let bob =
            (t * if moving { 10.4 } else { 4.1 }).sin().abs() * if moving { 0.086 } else { 0.025 };
        hippo.translation = hippo.translation.lerp(home + Vec3::Y * bob, blend * 0.62);
        let yaw = if moving {
            (home.x - hippo.translation.x).signum() * 0.87
        } else {
            0.13
        };
        let rot = Quat::from_rotation_y(yaw) * Quat::from_rotation_z((t * 8.4).sin() * 0.028);
        hippo.rotation = hippo.rotation.slerp(rot, blend);
    }
    for (mut transform, limb) in &mut rigs.p6() {
        let amount = if limb.leg { 0.40 } else { -0.34 };
        transform.rotation = Quat::from_rotation_x(gait * limb.side * amount);
    }
    let lives = if let Phase::Open(day) = &town.phase {
        Some(day)
    } else {
        None
    };
    for (mut transform, mut visibility, actor) in &mut rigs.p4() {
        let i = actor.0;
        let place = if let Some(day) = lives {
            day.visits.get(i).and_then(|visit| {
                let age = day.seconds - visit.at;
                if !(-6.0..5.0).contains(&age) {
                    return None;
                }
                let origin = match visit.origin {
                    Origin::Home(lot) => gate(usize::from(lot), cols),
                    Origin::Ferry => v(landscape::quay(cols), 0.133, 5.65),
                };
                let shop = if visit.resolved {
                    visit.shop.map(usize::from)
                } else {
                    town.target(&day.stock, visit, chapter)
                };
                let goal = shop
                    .map(|lot| gate(lot, cols))
                    .unwrap_or(v(0.0, 0.055, 0.34));
                let approaching = age < 0.0;
                let elapsed = if approaching {
                    (age + 6.0) / 6.0
                } else {
                    ((age - 0.85) / 4.15).clamp(0.0, 1.0)
                };
                let (from, to) = if approaching {
                    (origin, goal)
                } else {
                    (goal, origin)
                };
                Some((
                    walk(from, to, elapsed, (i % 3) as f32 * 0.23 - 0.23),
                    (to.x - from.x).signum() * 1.22,
                    age.abs() > 0.85,
                ))
            })
        } else if i < if playing { 3 } else { 9 } {
            let phase = t * 0.26 + i as f32 * 1.77;
            Some((
                v(
                    phase.sin() * cols as f32 * 1.25,
                    0.052,
                    0.38 * (i % 3) as f32 - 0.38,
                ),
                phase.cos().signum() * 1.16,
                true,
            ))
        } else {
            None
        };
        if let Some((at, yaw, moving)) = place {
            *visibility = Visibility::Inherited;
            let stride = (t * 11.0 + i as f32 * 6.6).sin() * if moving { 1.0 } else { 0.17 };
            transform.translation = at + Vec3::Y * stride.abs() * 0.067;
            transform.rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_z(stride * 0.084);
            transform.scale = Vec3::splat(0.87 + (i % 5) as f32 * 0.06);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    let mut recent = [None; 12];
    if let Some(day) = lives {
        for (slot, visit) in day
            .visits
            .iter()
            .rev()
            .filter(|v| v.resolved && (0.0..1.45).contains(&(day.seconds - v.at)))
            .enumerate()
            .take(recent.len())
        {
            recent[slot] = Some(visit);
        }
        for (mut transform, mut visible, mark, mut mesh) in &mut rigs.p7() {
            let Some(visit) = recent[mark.0] else {
                *visible = Visibility::Hidden;
                continue;
            };
            let fade = (day.seconds - visit.at) / 1.45;
            *visible = Visibility::Inherited;
            let p = visit.shop.map_or(v(0.0, 0.0, 0.36), |lot| {
                let p = lot_position(usize::from(lot), cols);
                p + v(
                    0.17,
                    0.0,
                    if usize::from(lot) < COLS { 0.95 } else { -0.94 },
                )
            });
            transform.translation = p + v((mark.0 % 3) as f32 * 0.15, 1.15 + fade * 0.50, 0.05);
            transform.scale = Vec3::splat(0.95 + (fade * std::f32::consts::PI).sin() * 0.63);
            let geometry = &art.receipts[usize::from(visit.shop.is_some())];
            if &mesh.0 != geometry {
                mesh.0 = geometry.clone();
            }
        }
    } else {
        for (_, mut visible, _, _) in &mut rigs.p7() {
            *visible = Visibility::Hidden;
        }
    }
    for (mut transform, wave, boat, bird, sail, wake) in &mut rigs.p5() {
        if let Some(wave) = wave {
            let a = t * (0.52 + swell * 0.54) + wave.0 as f32 * 2.4;
            transform.translation = v(
                a.sin() * (0.21 + swell * 0.28),
                -0.429,
                a.cos() * (0.16 + swell * 0.21),
            );
        }
        if let Some(n) = boat.map(|b| b.0).or_else(|| wake.map(|w| w.0)) {
            let (x, z, angle) = if n == 0 {
                let drift = lives.map_or(0.0, |day| {
                    let s = day.seconds / chapter.day_seconds();
                    ((s - 0.11).abs() - 0.31).max(0.0) * (15.0 + swell * 3.3)
                });
                (
                    landscape::quay(cols) + 2.18 + drift,
                    6.88 + drift * 0.25,
                    -0.14,
                )
            } else if n == 1 {
                let w = cols as f32 * 1.6;
                (-w * 0.58 + (t * 0.14).sin() * w * 0.44, 8.65, 0.22)
            } else {
                ((t * 0.093).sin() * 9.0, -9.36, -0.16)
            };
            let beat = t * (1.1 + swell) + n as f32 * 2.9;
            if wake.is_some() {
                transform.translation = v(x, -0.422, z);
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

fn gate(lot: usize, cols: usize) -> Vec3 {
    let p = lot_position(lot, cols);
    if lot < COLS {
        p + v(0.18, 0.0, 1.05)
    } else {
        p + v(-1.38, 0.0, -0.39)
    }
}

fn walk(from: Vec3, to: Vec3, t: f32, path_z: f32) -> Vec3 {
    let a = v(from.x, 0.048, path_z);
    let b = v(to.x, 0.048, path_z);
    if t < 0.27 {
        from.lerp(a, t / 0.27)
    } else if t < 0.73 {
        a.lerp(b, (t - 0.27) / 0.46)
    } else {
        b.lerp(to, (t - 0.73) / 0.27)
    }
}

pub fn pick(
    camera: &Camera,
    transform: &GlobalTransform,
    position: Vec2,
    cols: usize,
) -> Option<usize> {
    let size = camera.logical_viewport_size()?;
    if position.x > size.x * 0.75 || position.y > size.y * 0.86 {
        return None;
    }
    let radius = if cols > 4 { 47.0 } else { 62.0 } * (size.x / 1280.0).clamp(0.55, 2.5);
    (0..2)
        .flat_map(|row| (0..cols).map(move |col| row * COLS + col))
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

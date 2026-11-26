use std::{
    collections::HashMap,
    f32::consts::{PI, TAU},
};

use bevy::{
    ecs::system::SystemParam, pbr::NotShadowCaster, prelude::*, render::mesh::skinning::SkinnedMesh,
};

use crate::{
    adventure::{Adventure, Build, Good, Terrain},
    campaign::{CHAPTERS, Region},
    game::{BUILD_TIME, Game, Panel, View},
    layout::{LOTS, lot_row},
    model::{Building, Kind, Phase, Town, Weather},
    save::Campaign,
};

use super::{
    Art,
    activity_assets::{self as assets, Craft, Movement, Profile, Sign},
    glow::{Halo, halo},
    lot_position,
    people::{self, Cast, StationGesture, WorkerJoint},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Harbor(usize),
    Adventure(usize),
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Workshop {
    build: Build,
    tier: u8,
}

#[derive(Default, Resource)]
pub(crate) struct Scene {
    place: Option<Place>,
    slots: Vec<Option<(Workshop, Entity)>>,
    models: HashMap<(Region, Profile), Model>,
    signs: HashMap<Sign, Handle<Mesh>>,
}

struct Model {
    parts: Vec<(Handle<Mesh>, Vec3, Movement)>,
    person: Vec3,
    guest: Option<Vec3>,
}

#[derive(Component)]
pub(crate) struct Station {
    lot: usize,
    workshop: Workshop,
    profile: Profile,
}

/// A building whose work happens inside: it shows the work through its door and roof.
#[derive(Component)]
pub(crate) struct Interior {
    lot: usize,
    workshop: Workshop,
    profile: Profile,
}

/// One piece of smoke, steam, sparks, chips or dust leaving a busy building.
#[derive(Component)]
pub(crate) struct Puff {
    lot: usize,
    index: u8,
    sign: Sign,
    origin: Vec3,
}

/// Firelight in the doorway of a kiln, oven or forge.
#[derive(Component)]
pub(crate) struct Firelight {
    lot: usize,
}

/// Cloth, laundry or a lantern on a line, moving in the wind.
#[derive(Component)]
pub(crate) struct Flutter {
    phase: f32,
}

/// Someone who works inside and now and then steps out to the yard beside the door.
#[derive(Component)]
pub(crate) struct Errand {
    lot: usize,
    variant: usize,
    gesture: StationGesture,
    door: Vec3,
    spot: Vec3,
    face: f32,
    offset: f32,
}

/// The rhythm of an errand: inside, out of the door, at work in the yard, back in.
const ERRAND_PERIOD: f32 = 15.0;
const ERRAND_OUT: f32 = 7.0;
const ERRAND_WALK: f32 = 1.3;
const ERRAND_WORK: f32 = 3.2;

fn errand_gesture(craft: Craft) -> StationGesture {
    match craft {
        Craft::Saw => StationGesture::Saw,
        Craft::Dye | Craft::Bath | Craft::Scent => StationGesture::Pour,
        Craft::Forge | Craft::Glass => StationGesture::Tend,
        _ => StationGesture::Lift,
    }
}

#[derive(Component)]
pub(crate) struct Detail {
    lot: usize,
    at: Vec3,
    movement: Movement,
}

#[derive(Component)]
pub(crate) struct Attendant {
    lot: usize,
    variant: usize,
    gesture: StationGesture,
    guest: bool,
}

fn harbor_spot(lot: usize, columns: usize) -> Transform {
    let center = lot_position(lot, columns);
    let (front, yaw) = if lot_row(lot) == 0 {
        (-1.24, 0.0)
    } else {
        (1.24, PI)
    };
    Transform::from_xyz(center.x + 0.49, 0.092, front)
        .with_rotation(Quat::from_rotation_y(yaw))
        .with_scale(Vec3::splat(0.78))
}

pub(super) fn adventure_spot(
    terrain: &Terrain,
    lots: &[Option<crate::adventure::Property>],
    lot: usize,
) -> Option<Transform> {
    let plot = terrain.plots[lot];
    for (dx, dz, yaw) in [
        (0.61, 2.46, 0.0),
        (-0.61, -2.46, PI),
        (2.46, 0.61, PI * 0.5),
        (-2.46, -0.61, -PI * 0.5),
    ] {
        let at = plot + Vec2::new(dx, dz);
        let turn = Quat::from_rotation_y(yaw);
        let clear = [Vec2::ZERO, Vec2::new(0.52, 0.42), Vec2::new(-0.49, -0.52)]
            .into_iter()
            .all(|offset| {
                let sample = turn * Vec3::new(offset.x, 0.0, offset.y);
                terrain.walkable(at + Vec2::new(sample.x, sample.z), lots)
            });
        if clear {
            return Some(
                Transform::from_translation(
                    super::adventure::position(terrain, at) + Vec3::Y * 0.07,
                )
                .with_rotation(turn)
                .with_scale(Vec3::splat(1.04)),
            );
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn sync(
    mut commands: Commands,
    game: Res<Game>,
    campaign: Res<Campaign>,
    adventure: Res<Adventure>,
    cast: Res<Cast>,
    art: Res<Art>,
    mut cache: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let town = game.visible_town(&campaign);
    let chapter = &CHAPTERS[town.chapter];
    let land = adventure.terrain.as_ref().zip(adventure.active());
    let scene = match game.view {
        View::Street => Some((Place::Harbor(town.chapter), chapter.region, LOTS)),
        View::Adventure => land.map(|(map, island)| {
            (
                Place::Adventure(island.index),
                map.level.region,
                island.lots.len(),
            )
        }),
        _ => None,
    };
    if cache.place != scene.map(|(place, _, _)| place) {
        for &(_, entity) in cache.slots.iter().flatten() {
            commands.entity(entity).despawn();
        }
        cache.slots.clear();
        cache.place = scene.map(|(place, _, _)| place);
    }
    let Some((place, region, len)) = scene else {
        return;
    };
    cache.slots.resize(len, None);
    for lot in 0..len {
        let workshop = match place {
            Place::Harbor(_) => town.lots[lot].map(|b| Workshop {
                build: Build::Harbor(b.kind),
                tier: b.level,
            }),
            Place::Adventure(_) => {
                land.and_then(|(_, island)| island.lots[lot])
                    .map(|p| Workshop {
                        build: p.kind,
                        tier: p.tier,
                    })
            }
        };
        if cache.slots[lot].map(|(current, _)| current) == workshop {
            continue;
        }
        if let Some((_, old)) = cache.slots[lot].take() {
            commands.entity(old).despawn();
        }
        let Some(workshop) = workshop else { continue };
        let profile = Profile::of(workshop.build);
        if profile.indoors() {
            let base = match place {
                Place::Harbor(_) => Some(lot_position(lot, chapter.layout.width())),
                Place::Adventure(_) => {
                    land.map(|(map, _)| super::adventure::position(map, map.plots[lot]))
                }
            };
            let Some(base) = base else { continue };
            let roof = match workshop.build {
                Build::Harbor(kind) => art.roof_height(region, kind, workshop.tier),
                build => super::adventure::workshop_roof(region, build, workshop.tier),
            };
            let sign = profile.sign();
            let piece = cache
                .signs
                .entry(sign)
                .or_insert_with(|| meshes.add(sign.piece()))
                .clone();
            let origin = match sign {
                Sign::Chips | Sign::Dust => Vec3::new(0.0, 0.22, 1.28),
                _ => Vec3::new(0.42, (roof - 0.12).max(0.6), -0.18),
            };
            let material = if sign == Sign::Sparks {
                art.lantern_material.clone()
            } else {
                art.matte.clone()
            };
            // The yard lies where the old outdoor bench stood, beside the door.
            let outside = match place {
                Place::Harbor(_) => Some(harbor_spot(lot, chapter.layout.width())),
                Place::Adventure(_) => {
                    land.and_then(|(map, island)| adventure_spot(map, &island.lots, lot))
                }
            };
            let yard = outside.map(|spot| {
                let local = spot.translation - base;
                let out = Vec2::new(local.x, local.z).normalize_or(Vec2::Y);
                let side = Vec2::new(out.y, -out.x);
                let ground = local.y;
                let door = Vec3::new(out.x * 1.15, ground, out.y * 1.15);
                let work = Vec3::new(local.x, ground, local.z);
                let pile = work + Vec3::new(side.x, 0.0, side.y) * 0.62;
                (door, work, pile, out, spot.scale.x)
            });
            let root = commands
                .spawn((
                    Interior {
                        lot,
                        workshop,
                        profile,
                    },
                    Transform::from_translation(base),
                    Visibility::Inherited,
                ))
                .with_children(|root| {
                    for index in 0..sign.pieces() {
                        root.spawn((
                            Puff {
                                lot,
                                index,
                                sign,
                                origin,
                            },
                            Mesh3d(piece.clone()),
                            MeshMaterial3d(material.clone()),
                            Transform::from_translation(origin).with_scale(Vec3::ZERO),
                            NotShadowCaster,
                        ));
                    }
                    if sign.fire() {
                        root.spawn((
                            Firelight { lot },
                            halo(&art.glow, Vec3::new(0.0, 0.55, 1.30), 1.35, lot as f32),
                        ));
                    }
                    let Some((door, work, pile, out, scale)) = yard else {
                        return;
                    };
                    let facing = Quat::from_rotation_y(out.x.atan2(out.y));
                    let yard = assets::yard(profile.craft, region);
                    root.spawn((
                        Mesh3d(meshes.add(yard.still)),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(pile)
                            .with_rotation(facing)
                            .with_scale(Vec3::splat(scale * 1.15)),
                    ))
                    .with_children(|pile| {
                        for (i, (mesh, pivot)) in yard.hanging.into_iter().enumerate() {
                            pile.spawn((
                                Flutter {
                                    phase: lot as f32 * 0.7 + i as f32 * 1.3,
                                },
                                Mesh3d(meshes.add(mesh)),
                                MeshMaterial3d(art.matte.clone()),
                                Transform::from_translation(pivot),
                                NotShadowCaster,
                            ));
                        }
                    });
                    let variant =
                        (lot * 3 + workshop.tier as usize + profile.variation as usize) % 8;
                    let mut person = root.spawn((
                        Errand {
                            lot,
                            variant,
                            gesture: errand_gesture(profile.craft),
                            door,
                            spot: work,
                            face: (pile - work).x.atan2((pile - work).z),
                            offset: lot as f32 * 3.7,
                        },
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(door).with_scale(Vec3::splat(scale)),
                        Visibility::Hidden,
                    ));
                    people::worker_rig(&mut person, &cast, variant, region);
                })
                .id();
            cache.slots[lot] = Some((workshop, root));
            continue;
        }
        let transform = match place {
            Place::Harbor(_) => Some(harbor_spot(lot, chapter.layout.width())),
            Place::Adventure(_) => {
                land.and_then(|(map, island)| adventure_spot(map, &island.lots, lot))
            }
        };
        let Some(transform) = transform else { continue };
        let model = cache.models.entry((region, profile)).or_insert_with(|| {
            let station = assets::station(profile, region);
            Model {
                parts: station
                    .parts
                    .into_iter()
                    .map(|part| (meshes.add(part.mesh), part.at, part.movement))
                    .collect(),
                person: station.person,
                guest: station.visitor,
            }
        });
        let variant = (lot * 3 + workshop.tier as usize + profile.variation as usize) % 8;
        let root = commands
            .spawn((
                Station {
                    lot,
                    workshop,
                    profile,
                },
                transform,
                Visibility::Hidden,
            ))
            .with_children(|root| {
                for (handle, at, movement) in &model.parts {
                    let mut piece = root.spawn((
                        Detail {
                            lot,
                            at: *at,
                            movement: *movement,
                        },
                        Mesh3d(handle.clone()),
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_translation(*at),
                    ));
                    if matches!(movement, Movement::Smoke(_) | Movement::Fleck(..)) {
                        piece.insert(NotShadowCaster);
                    }
                }
                let person = |root: &mut ChildSpawnerCommands, at: Vec3, guest| {
                    let style = if guest { (variant + 5) % 8 } else { variant };
                    let gesture = if guest {
                        StationGesture::Seated(at.y)
                    } else {
                        gesture(profile.craft)
                    };
                    let mut actor = root.spawn((
                        Attendant {
                            lot,
                            variant: style,
                            gesture,
                            guest,
                        },
                        MeshMaterial3d(art.matte.clone()),
                        Transform::from_xyz(at.x, 0.008, at.z)
                            .with_rotation(Quat::from_rotation_y(if guest { PI } else { 0.0 })),
                        Visibility::Hidden,
                    ));
                    people::worker_rig(&mut actor, &cast, style, region);
                };
                person(root, model.person, false);
                if let Some(at) = model.guest {
                    person(root, at, true);
                }
            })
            .id();
        cache.slots[lot] = Some((workshop, root));
    }
}

fn gesture(craft: Craft) -> StationGesture {
    match craft {
        Craft::Saw | Craft::Barrel => StationGesture::Saw,
        Craft::Draw | Craft::Bath | Craft::Dye => StationGesture::Pour,
        Craft::Clay | Craft::Press | Craft::Mill => StationGesture::Wheel,
        Craft::Bake | Craft::Glass | Craft::Forge | Craft::Brick | Craft::Signal => {
            StationGesture::Tend
        }
        Craft::Offering => StationGesture::Bow,
        Craft::Write => StationGesture::Write,
        Craft::Weave | Craft::Lantern | Craft::Nets => StationGesture::Weave,
        Craft::Home
        | Craft::Garden
        | Craft::Store
        | Craft::Market
        | Craft::Table
        | Craft::Hive
        | Craft::Scent => StationGesture::Lift,
    }
}

#[derive(Clone, Copy, Default)]
struct Active {
    work: bool,
    seated: bool,
}

fn harbor_activity(
    town: &Town,
    game: &Game,
    (lot, workshop, profile): (usize, Workshop, Profile),
    guests: &[bool; LOTS],
) -> Active {
    let Phase::Open(day) = &town.phase else {
        return Active::default();
    };
    if game.work[lot].is_some_and(|work| game.clock - work.since < BUILD_TIME) {
        return Active::default();
    }
    let Build::Harbor(kind) = workshop.build else {
        return Active::default();
    };
    let served = guests[lot];
    let busy = if kind.service().is_some() {
        day.stock[lot] > 0 || served
    } else {
        !kind.water_source() || day.water > 0
    };
    Active {
        work: busy,
        seated: served && profile.seating(),
    }
}

fn adventure_activity(
    island: &crate::adventure::Island,
    (lot, workshop, profile): (usize, Workshop, Profile),
) -> Active {
    if island.site(lot).is_some() {
        return Active::default();
    }
    let staffed = island.staff() > 0;
    let public = match workshop.build {
        Build::Harbor(kind) => {
            Building::new(kind).residents() > 0
                || staffed
                    && matches!(
                        kind,
                        Kind::Inn
                            | Kind::GardenCourt
                            | Kind::MoonGarden
                            | Kind::Stoa
                            | Kind::Shrine
                            | Kind::Temple
                            | Kind::Forum
                            | Kind::Baths
                            | Kind::Storehouse
                            | Kind::FerryOffice
                            | Kind::SignalTower
                    )
        }
        Build::Monument => staffed,
        _ => false,
    };
    let batch = island.batch(lot).is_some_and(|batch| batch.working());
    let dinner = profile.seating()
        && staffed
        && island.amount(Good::Meals) > 0
        && (0.31..0.76).contains(&island.phase);
    let work = batch || public || dinner;
    Active {
        work,
        seated: dinner,
    }
}

#[allow(clippy::type_complexity)]
#[derive(SystemParam)]
pub(crate) struct Animation<'w, 's> {
    stations: Query<'w, 's, (&'static Station, &'static mut Visibility)>,
    parts: Query<'w, 's, (&'static Detail, &'static mut Transform), Without<WorkerJoint>>,
    actors: Query<
        'w,
        's,
        (
            &'static Attendant,
            &'static SkinnedMesh,
            &'static mut Visibility,
        ),
        Without<Station>,
    >,
    joints: Query<'w, 's, &'static mut Transform, With<WorkerJoint>>,
    interiors: Query<
        'w,
        's,
        (&'static Interior, &'static mut Visibility),
        (Without<Station>, Without<Attendant>),
    >,
    puffs: Query<
        'w,
        's,
        (&'static Puff, &'static mut Transform),
        (Without<Detail>, Without<WorkerJoint>),
    >,
    firelight: Query<'w, 's, (&'static Firelight, &'static mut Halo)>,
    flutter: Query<
        'w,
        's,
        (&'static Flutter, &'static mut Transform),
        (
            Without<Detail>,
            Without<Puff>,
            Without<WorkerJoint>,
            Without<Errand>,
        ),
    >,
    errands: Query<
        'w,
        's,
        (
            &'static Errand,
            &'static SkinnedMesh,
            &'static mut Transform,
            &'static mut Visibility,
        ),
        (
            Without<Attendant>,
            Without<Station>,
            Without<Interior>,
            Without<Detail>,
            Without<Puff>,
            Without<WorkerJoint>,
        ),
    >,
}

pub(crate) fn animate(
    game: Res<Game>,
    campaign: Res<Campaign>,
    adventure: Res<Adventure>,
    cast: Res<Cast>,
    mut visual: Animation,
) {
    if visual.stations.is_empty() && visual.interiors.is_empty() {
        return;
    }
    let town = game.visible_town(&campaign);
    let adventure_state = adventure.terrain.as_ref().zip(adventure.active());
    let (region, wind, size, night) = match game.view {
        View::Street => {
            let region = Region::of(town.chapter);
            let wind = match town.weather(&CHAPTERS[town.chapter]) {
                Weather::Wind => 0.31,
                Weather::Storm => 0.51,
                _ => 0.12,
            };
            let night = if CHAPTERS[town.chapter].shift.night() {
                1.0
            } else {
                0.0
            };
            (region, wind, LOTS, night)
        }
        View::Adventure => {
            let Some((map, island)) = adventure_state else {
                return;
            };
            let phase = island.phase * TAU;
            let night = ((phase.cos() + 0.09) * 0.79).clamp(0.0, 0.86) / 0.86;
            (map.level.region, 0.12, island.lots.len(), night)
        }
        _ => return,
    };
    let guests = match &town.phase {
        Phase::Open(day) if game.view == View::Street => {
            let mut occupied = [false; LOTS];
            for visit in &day.visits {
                if (3.7..13.4).contains(&(day.seconds - visit.at))
                    && let Some(lot) = visit.shop
                {
                    occupied[usize::from(lot)] = true;
                }
            }
            occupied
        }
        _ => [false; LOTS],
    };
    let mut live = vec![Active::default(); size];
    let state_of = |lot: usize, workshop: Workshop, profile: Profile| {
        if game.panel == Panel::Wardrobe || lot >= size {
            Active::default()
        } else if game.view == View::Street {
            harbor_activity(town, &game, (lot, workshop, profile), &guests)
        } else {
            adventure_state.map_or(Active::default(), |(_, island)| {
                adventure_activity(island, (lot, workshop, profile))
            })
        }
    };
    for (station, mut visible) in &mut visual.stations {
        let state = state_of(station.lot, station.workshop, station.profile);
        if let Some(slot) = live.get_mut(station.lot) {
            *slot = state;
        }
        set_visibility(&mut visible, state.work);
    }
    for (inside, mut visible) in &mut visual.interiors {
        let state = state_of(inside.lot, inside.workshop, inside.profile);
        if let Some(slot) = live.get_mut(inside.lot) {
            *slot = state;
        }
        // The yard stays; only the signs of work come and go.
        set_visibility(&mut visible, game.panel != Panel::Wardrobe);
    }
    for (fire, mut light) in &mut visual.firelight {
        // By night every doorway already glows; the firelight shows the work by day.
        let working = live.get(fire.lot).is_some_and(|state| state.work);
        light.strength = if working { 0.6 * (1.0 - night) } else { 0.0 };
    }
    for (puff, mut transform) in &mut visual.puffs {
        if live.get(puff.lot).is_some_and(|state| state.work) {
            *transform = puff_pose(puff, game.clock + puff.lot as f32 * 0.47, wind);
        } else if transform.scale != Vec3::ZERO {
            transform.scale = Vec3::ZERO;
        }
    }
    for (cloth, mut transform) in &mut visual.flutter {
        let t = game.clock * (1.6 + wind * 3.0) + cloth.phase;
        let gust = 0.10 + wind * 0.9;
        transform.rotation = Quat::from_rotation_x(-gust - (t.sin() * 0.5 + 0.5) * gust * 1.4)
            * Quat::from_rotation_z((t * 0.7).sin() * 0.06);
    }
    let mut errand_poses = Vec::new();
    for (errand, rig, mut transform, mut visible) in &mut visual.errands {
        let working = live.get(errand.lot).is_some_and(|state| state.work);
        let t = (game.clock + errand.offset).rem_euclid(ERRAND_PERIOD);
        let walk_out = ERRAND_OUT..ERRAND_OUT + ERRAND_WALK;
        let at_work = walk_out.end..walk_out.end + ERRAND_WORK;
        let walk_in = at_work.end..at_work.end + ERRAND_WALK;
        let outside = working && (walk_out.start..walk_in.end).contains(&t);
        set_visibility(&mut visible, outside);
        if !outside {
            continue;
        }
        let path = errand.spot - errand.door;
        let heading = |d: Vec3| d.x.atan2(d.z);
        let pose = if walk_out.contains(&t) || walk_in.contains(&t) {
            let (p, toward) = if walk_out.contains(&t) {
                ((t - walk_out.start) / ERRAND_WALK, path)
            } else {
                (1.0 - (t - walk_in.start) / ERRAND_WALK, -path)
            };
            let eased = p * p * (3.0 - 2.0 * p);
            transform.translation = errand.door + path * eased;
            transform.rotation = Quat::from_rotation_y(heading(toward));
            people::worker_pose(&cast, errand.variant, region, t * 5.2, 1.0, t, false)
        } else {
            transform.translation = errand.spot;
            transform.rotation = Quat::from_rotation_y(errand.face);
            people::station_pose(&cast, errand.variant, region, errand.gesture, t)
        };
        errand_poses.extend(rig.joints.iter().copied().zip(pose));
    }
    for (entity, pose) in errand_poses {
        if let Ok(mut joint) = visual.joints.get_mut(entity) {
            *joint = pose;
        }
    }
    for (piece, mut transform) in &mut visual.parts {
        if !live.get(piece.lot).is_some_and(|state| state.work) {
            continue;
        }
        let theta = game.clock + piece.lot as f32 * 0.47;
        match piece.movement {
            Movement::Still => {}
            Movement::Spin(rate) => transform.rotation = Quat::from_rotation_y(theta * rate),
            Movement::Swing(axis, amplitude, rate) => {
                transform.rotation = Quat::from_axis_angle(axis, amplitude * (theta * rate).sin());
            }
            Movement::Slide(distance, rate) => {
                transform.translation = piece.at + distance * (theta * rate).sin()
            }
            Movement::Smoke(index) => {
                let phase = (theta * 0.65 + f32::from(index) * 0.33).fract();
                let grow = (phase * PI).sin().max(0.02);
                transform.translation = piece.at
                    + Vec3::new(
                        wind * phase * phase,
                        phase * 0.94,
                        0.07 * (phase * TAU).sin(),
                    );
                transform.scale = Vec3::splat((0.45 + phase * 0.79) * grow);
                transform.rotation = Quat::from_rotation_y(phase * 1.8);
            }
            Movement::Fleck(index, rate) => {
                let phase = (theta * rate + f32::from(index) * 0.24).fract();
                let lateral = index as f32 * 0.19 - 0.20;
                transform.translation =
                    piece.at + Vec3::new(lateral * phase, (phase * PI).sin() * 0.19, 0.18 * phase);
                transform.scale = Vec3::splat((phase * PI).sin().max(0.01));
                transform.rotation = Quat::from_rotation_z(phase * TAU);
            }
        }
    }
    for (actor, rig, mut visible) in &mut visual.actors {
        let show = live.get(actor.lot).is_some_and(|state| {
            if actor.guest {
                state.seated
            } else {
                state.work
            }
        });
        set_visibility(&mut visible, show);
        if !show {
            continue;
        }
        let seconds = game.clock + actor.lot as f32 * 0.47;
        let pose = people::station_pose(&cast, actor.variant, region, actor.gesture, seconds);
        for (&entity, transform) in rig.joints.iter().zip(pose) {
            if let Ok(mut joint) = visual.joints.get_mut(entity) {
                *joint = transform;
            }
        }
    }
}

/// Where one piece of a work effect is at time `t`.
fn puff_pose(puff: &Puff, t: f32, wind: f32) -> Transform {
    let count = f32::from(puff.sign.pieces().max(1));
    let offset = f32::from(puff.index) / count;
    let spread = f32::from(puff.index) * 2.399;
    let at = |phase: f32, rate: f32| (phase * rate + offset).fract();
    let (translation, scale, spin) = match puff.sign {
        Sign::Hearth | Sign::Smoke | Sign::Steam => {
            let (rate, rise, size) = match puff.sign {
                Sign::Hearth => (0.22, 0.95, 0.55),
                Sign::Steam => (0.42, 0.70, 0.85),
                _ => (0.34, 1.20, 1.0),
            };
            let p = at(t, rate);
            let drift = Vec3::new(
                wind * 2.2 * p * p + 0.06 * (p * TAU + spread).sin(),
                rise * p,
                0.05 * (p * TAU + spread).cos(),
            );
            let grow = (0.45 + p * 0.9) * (p * PI).sin().max(0.0);
            (puff.origin + drift, grow * size, p * 1.8)
        }
        Sign::Sparks => {
            let p = at(t, 0.85);
            let flight = p * 0.75;
            let out = Vec3::new(spread.cos() * 0.32, 1.25, spread.sin() * 0.32);
            let fall = Vec3::Y * (-1.7 * flight * flight);
            (puff.origin + out * flight + fall, (1.0 - p) * 0.9, p * TAU)
        }
        Sign::Chips => {
            let p = at(t, 0.62);
            let flight = p * 0.7;
            let lateral = (f32::from(puff.index) - 2.0) * 0.16;
            let out = Vec3::new(lateral, 1.05, 0.75);
            let height = (out.y * flight - 2.4 * flight * flight).max(-0.18);
            (
                puff.origin + Vec3::new(out.x * flight, height, out.z * flight),
                (1.0 - p.powi(4)).max(0.0),
                p * 9.0 + spread,
            )
        }
        Sign::Dust => {
            let p = at(t, 0.38);
            let drift = Vec3::new(
                (f32::from(puff.index) - 1.5) * 0.22 * p + wind * p,
                0.22 * (p * PI).sin() + 0.12 * p,
                0.42 * p,
            );
            (puff.origin + drift, (p * PI).sin() * 1.1, p * 2.0)
        }
        Sign::Glow => (puff.origin, 0.0, 0.0),
    };
    Transform::from_translation(translation)
        .with_rotation(Quat::from_euler(EulerRot::YXZ, spin, spin * 0.6, 0.0))
        .with_scale(Vec3::splat(scale.max(0.0)))
}

fn set_visibility(out: &mut Visibility, show: bool) {
    let next = if show {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    if *out != next {
        *out = next;
    }
}

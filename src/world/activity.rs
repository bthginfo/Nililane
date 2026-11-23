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
    activity_assets::{self as assets, Craft, Movement, Profile},
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
        let transform = match place {
            Place::Harbor(_) => Some(harbor_spot(lot, chapter.layout.width())),
            Place::Adventure(_) => {
                land.and_then(|(map, island)| adventure_spot(map, &island.lots, lot))
            }
        };
        let Some(transform) = transform else { continue };
        let profile = Profile::of(workshop.build);
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

fn harbor_activity(town: &Town, game: &Game, station: &Station, guests: &[bool; LOTS]) -> Active {
    let Phase::Open(day) = &town.phase else {
        return Active::default();
    };
    if game.work[station.lot].is_some_and(|work| game.clock - work.since < BUILD_TIME) {
        return Active::default();
    }
    let Build::Harbor(kind) = station.workshop.build else {
        return Active::default();
    };
    let served = guests[station.lot];
    let busy = if kind.service().is_some() {
        day.stock[station.lot] > 0 || served
    } else {
        !kind.water_source() || day.water > 0
    };
    Active {
        work: busy,
        seated: served && station.profile.seating(),
    }
}

fn adventure_activity(island: &crate::adventure::Island, station: &Station) -> Active {
    if island.site(station.lot).is_some() {
        return Active::default();
    }
    let staffed = island.staff() > 0;
    let public = match station.workshop.build {
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
    let batch = island
        .batch(station.lot)
        .is_some_and(|batch| batch.working());
    let dinner = station.profile.seating()
        && staffed
        && island.amount(Good::Meals) > 0
        && (0.31..0.76).contains(&island.phase);
    let work = batch || public || dinner;
    Active {
        work,
        seated: dinner,
    }
}

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
}

pub(crate) fn animate(
    game: Res<Game>,
    campaign: Res<Campaign>,
    adventure: Res<Adventure>,
    cast: Res<Cast>,
    mut visual: Animation,
) {
    if visual.stations.is_empty() {
        return;
    }
    let town = game.visible_town(&campaign);
    let adventure_state = adventure.terrain.as_ref().zip(adventure.active());
    let (region, wind, size) = match game.view {
        View::Street => {
            let region = Region::of(town.chapter);
            let wind = match town.weather(&CHAPTERS[town.chapter]) {
                Weather::Wind => 0.31,
                Weather::Storm => 0.51,
                _ => 0.12,
            };
            (region, wind, LOTS)
        }
        View::Adventure => {
            let Some((map, island)) = adventure_state else {
                return;
            };
            (map.level.region, 0.12, island.lots.len())
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
    for (station, mut visible) in &mut visual.stations {
        let state = if game.panel == Panel::Wardrobe || station.lot >= size {
            Active::default()
        } else if game.view == View::Street {
            harbor_activity(town, &game, station, &guests)
        } else {
            adventure_state.map_or(Active::default(), |(_, island)| {
                adventure_activity(island, station)
            })
        };
        if let Some(slot) = live.get_mut(station.lot) {
            *slot = state;
        }
        set_visibility(&mut visible, state.work);
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

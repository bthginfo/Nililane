use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::{
    campaign::CHAPTERS,
    errands::{Job, RouteStage, Stop},
    game::{Game, Panel, View},
    model::{Favor, Phase},
    save::Campaign,
};

use super::{
    buildings::{GOLD, MARBLE},
    mesh::{Batch, v},
    motion::{Path, controlled_position, delivery_path, stop},
};

const TRAIL: usize = 15;
const SPARKS: usize = 6;
pub(super) const PACK_CENTER: Vec3 = Vec3::new(0.0, 0.87, 0.49);

#[derive(Default)]
pub(super) struct Pose {
    pub height: f32,
    pub pitch: f32,
    pub roll: f32,
    pub stretch: f32,
    pub arms: [f32; 2],
    pub spread: f32,
    pub pack_shift: Vec3,
    pub pack_tilt: Vec3,
    pub carry: bool,
}

pub(super) fn quick_action(activity: crate::street::Activity, time: f32) -> Pose {
    let bend = (time / 0.60 * PI).sin();
    let sweep = (time * 23.0).sin() * 0.22;
    Pose {
        height: -0.15 * bend,
        pitch: 0.34 * bend,
        roll: 0.10 * bend,
        arms: [
            -0.12 * bend,
            (-0.95 + sweep * f32::from(activity.animal())) * bend,
        ],
        stretch: -0.06 * bend,
        ..default()
    }
}

pub(super) fn perform(job: Job, stage: RouteStage, phase: f32) -> Pose {
    use RouteStage::*;
    let mut pose = Pose::default();
    if stage == Approach {
        return pose;
    }
    if stage == Carry {
        pose.carry = true;
        pose.arms = [-0.60, -0.60];
        pose.pitch = if job == Job::Quay { 0.12 } else { -0.025 };
        return pose;
    }

    let collect = stage == Collect;
    let weight = (phase * PI).sin();
    let sway = (phase * TAU * 1.5).sin() * weight;
    pose.carry = !collect || phase > 0.17 || job == Job::Welcome;
    let shift = if collect { 1.0 - phase } else { phase };
    pose.pack_shift = v(0.0, -0.53 * shift, 0.30 * shift);
    pose.stretch = -0.12 * weight;

    match job {
        Job::Freight => {
            pose.height = -0.13 * weight;
            pose.pitch = 0.33 * weight;
            pose.arms = [-0.55 - 0.53 * weight; 2];
            pose.pack_tilt.x = 0.24 * weight;
        }
        Job::Water => {
            pose.height = -0.07 * weight;
            pose.pitch = 0.20 * weight;
            pose.roll = -0.21 * weight;
            pose.arms = [-0.65 - 0.56 * weight, -0.62];
            pose.pack_shift.x = 0.14 * weight;
            pose.pack_tilt.z = -(if collect { 0.31 } else { 1.08 }) * weight;
        }
        Job::Welcome => {
            pose.height = 0.075 * sway.abs();
            pose.pitch = if collect {
                -0.08 * weight
            } else {
                0.16 * weight
            };
            pose.arms = [-0.27, -1.8 * weight + 0.28 * sway];
            pose.spread = 0.42 * weight;
            pose.pack_shift = Vec3::ZERO;
            pose.pack_tilt.z = 0.38 * sway;
        }
        Job::Shade => {
            pose.height = 0.03 * sway.abs();
            pose.roll = 0.20 * sway;
            pose.arms = [-0.64 + 0.68 * sway, -0.64 - 0.68 * sway];
            pose.pack_shift.x = 0.38 * sway;
            pose.pack_tilt.z = 0.89 * sway;
        }
        Job::Quay => {
            pose.height = -0.09 * weight;
            pose.pitch = 0.26 * weight;
            pose.roll = 0.18 * sway;
            pose.arms = [-0.66 - 0.60 * sway, -0.66 + 0.60 * sway];
            pose.pack_shift.x = 0.43 * sway;
            pose.pack_tilt = v(0.32 * weight, 0.48 * sway, 0.42 * sway);
        }
    }
    pose
}

pub(super) fn celebrate(age: f32) -> Pose {
    let p = (age / 0.86).clamp(0.0, 1.0);
    let spring = (p * PI).sin();
    let hop = spring * (p * TAU).sin().abs();
    Pose {
        height: 0.24 * hop,
        pitch: -0.16 * hop,
        roll: 0.18 * (p * TAU).sin() * spring,
        stretch: 0.10 * hop,
        arms: [-1.22 * spring; 2],
        spread: 0.63 * spring,
        ..default()
    }
}

#[derive(Component)]
pub(crate) enum Mark {
    Trail(usize),
    Pickup,
    Particle(Job, usize),
    Star(usize),
    Flame,
    Smoke(usize),
    Crates,
    BrokenJars,
    ShadeSail,
    LostGroup,
}

pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    glow: &Handle<StandardMaterial>,
) {
    let mut batch = Batch::default();
    batch.gem(v(0.0, 0.022, 0.0), 0.084, 0.032, 0.131, 0.0, GOLD);
    let dot = meshes.add(batch.mesh());
    let mut spawn = |kind, mesh: Handle<Mesh>| {
        commands.spawn((
            kind,
            Mesh3d(mesh),
            MeshMaterial3d(glow.clone()),
            Transform::default(),
            Visibility::Hidden,
            bevy::pbr::NotShadowCaster,
        ));
    };
    for i in 0..TRAIL {
        spawn(Mark::Trail(i), dot.clone());
    }
    let mut batch = Batch::default();
    for i in 0..12 {
        let angle = i as f32 / 12.0 * TAU;
        let p = v(angle.cos() * 0.47, 0.024, angle.sin() * 0.37);
        batch.gem(p, 0.071, 0.04, 0.09, angle, [107, 206, 187]);
    }
    batch.gem(v(0.0, 1.15, 0.0), 0.12, 0.18, 0.12, 0.51, [145, 230, 214]);
    spawn(Mark::Pickup, meshes.add(batch.mesh()));

    for (job, tint) in Job::ALL.into_iter().zip([
        [251, 193, 103],
        [83, 206, 241],
        [251, 159, 175],
        [159, 207, 113],
        [216, 176, 131],
    ]) {
        let mut batch = Batch::default();
        batch.gem(Vec3::ZERO, 0.081, 0.129, 0.082, 0.17, tint);
        let particle = meshes.add(batch.mesh());
        for i in 0..SPARKS {
            spawn(Mark::Particle(job, i), particle.clone());
        }
    }
    let mut batch = Batch::default();
    batch.gem(Vec3::ZERO, 0.10, 0.15, 0.063, 0.36, GOLD);
    batch.gem(v(0.0, 0.033, 0.039), 0.048, 0.075, 0.030, 0.31, MARBLE);
    let star = meshes.add(batch.mesh());
    for i in 0..SPARKS {
        spawn(Mark::Star(i), star.clone());
    }
    let mut fire = Batch::default();
    fire.gem(v(0.0, 0.24, 0.0), 0.30, 0.32, 0.24, 0.1, [218, 91, 41]);
    fire.gem(v(0.03, 0.20, 0.09), 0.18, 0.24, 0.14, 0.8, [251, 179, 79]);
    fire.gem(v(-0.15, 0.32, -0.02), 0.13, 0.27, 0.12, 0.4, [227, 113, 49]);
    spawn(Mark::Flame, meshes.add(fire.mesh()));
    let mut smoke = Batch::default();
    smoke.gem(Vec3::ZERO, 0.17, 0.18, 0.13, 0.12, [121, 116, 107]);
    let plume = meshes.add(smoke.mesh());
    for i in 0..4 {
        spawn(Mark::Smoke(i), plume.clone());
    }
    let mut boxes = Batch::default();
    for (point, tilt) in [(v(0.05, 0.17, 0.19), 0.2), (v(-0.28, 0.18, -0.22), -0.43)] {
        boxes.block(
            point,
            v(0.39, 0.37, 0.36),
            Quat::from_rotation_y(tilt),
            [161, 114, 74],
        );
        boxes.block(
            point + Vec3::Y * 0.14,
            v(0.39, 0.065, 0.37),
            Quat::from_rotation_y(tilt),
            [217, 168, 109],
        );
    }
    spawn(Mark::Crates, meshes.add(boxes.mesh()));
    let mut jars = Batch::default();
    for n in 0..10 {
        let angle = n as f32 * 2.399;
        let p = v(angle.cos() * 0.38, 0.013, angle.sin() * 0.24);
        let q = v(angle.sin() * 0.16, 0.075, angle.cos() * 0.14);
        jars.face(
            &[p, p + q, p + v(0.12, 0.014, 0.05)],
            [[204, 111, 75], [141, 87, 64], [82, 144, 142]][n % 3],
        );
        jars.face(&[p + v(0.12, 0.014, 0.05), p + q, p], [224, 164, 107]);
    }
    spawn(Mark::BrokenJars, meshes.add(jars.mesh()));
    let mut sail = Batch::default();
    sail.beam(
        v(-0.56, 0.04, -0.10),
        v(0.49, 0.39, 0.21),
        0.053,
        [148, 94, 65],
    );
    for n in 0..7 {
        let x = -0.51 + n as f32 * 0.156;
        let p = v(x, 0.065 + (x * 6.1).cos() * 0.033, -0.17);
        sail.face(
            &[
                p,
                p + v(0.16, 0.021, 0.03),
                p + v(0.16, 0.22, 0.43),
                p + v(0.0, 0.14, 0.49),
            ],
            if n % 2 == 0 {
                [114, 169, 172]
            } else {
                [235, 219, 166]
            },
        );
    }
    spawn(Mark::ShadeSail, meshes.add(sail.mesh()));
    let mut sign = Batch::default();
    sign.beam(v(0.0, 0.02, 0.0), v(0.0, 0.58, 0.0), 0.049, [147, 104, 68]);
    sign.block(
        v(-0.09, 0.49, 0.0),
        v(0.39, 0.18, 0.068),
        Quat::from_rotation_z(0.12),
        [91, 142, 142],
    );
    sign.gem(v(0.0, 0.49, 0.057), 0.071, 0.052, 0.029, 0.25, GOLD);
    spawn(Mark::LostGroup, meshes.add(sign.mesh()));
}

pub fn animate(
    game: Res<Game>,
    save: Res<Campaign>,
    mut marks: Query<(&Mark, &mut Transform, &mut Visibility)>,
) {
    let town = game.visible_town(&save);
    let chapter = &CHAPTERS[town.chapter];
    let layout = chapter.layout;
    let day = if let Phase::Open(day) = &town.phase {
        Some(day)
    } else {
        None
    };
    let active = day.and_then(|day| day.route_pose());
    let manual = day.and_then(|day| day.manual_route());
    let preview = game
        .job
        .and_then(|job| job.plan(town, chapter, game.selected).ok());
    let route = active.map(|(route, _)| route).or(preview);
    let stage = active.map(|(_, (stage, _))| stage);
    let track = route.map(|route| {
        if let Some((_, nili)) = manual {
            Path::road(
                controlled_position(nili),
                stop(nili.target(route), layout),
                0.34,
            )
        } else if stage == Some(RouteStage::Approach) {
            Path::road(stop(route.from, layout), stop(route.pickup, layout), 0.34)
        } else {
            delivery_path(route, layout)
        }
    });
    let burst = active.and_then(|(route, (stage, at))| {
        let place = match stage {
            RouteStage::Collect => route.pickup,
            RouteStage::Deliver => route.delivery,
            _ => return None,
        };
        let point = manual.map_or_else(
            || stop(place, layout),
            |(_, nili)| controlled_position(nili),
        );
        Some((route.job, point, at, stage))
    });
    let incident = town.incident(chapter);
    let fire = incident
        .filter(|(event, _)| *event == Favor::Fire)
        .map(|(_, lot)| {
            stop(Stop::Parcel(lot as u8), layout)
                + v(
                    0.19,
                    0.0,
                    if crate::layout::lot_row(lot) == 0 {
                        -0.29
                    } else {
                        0.33
                    },
                )
        });
    let celebration = game.delivery().filter(|_| active.is_none()).map(|done| {
        (
            stop(done.route.delivery, layout),
            (game.clock - done.since) / 1.05,
        )
    });
    for (mark, mut transform, mut visible) in &mut marks {
        let effect = if game.view == View::Street && game.panel != Panel::Wardrobe {
            match *mark {
                Mark::Trail(i) => track
                    .as_ref()
                    .filter(|p| p.length() > 0.7)
                    .and_then(|path| {
                        let spacing = (path.length() / 0.62).ceil().min(TRAIL as f32 + 1.0);
                        let step = (i as f32 + 1.0) / spacing;
                        (step < 1.0).then(|| {
                            let m = path.sample(step, 1.0, 0.0);
                            let pulse = 0.84 + 0.16 * (game.clock * 5.0 - i as f32 * 0.66).sin();
                            Transform::from_translation(m.at + Vec3::Y * 0.028)
                                .with_rotation(Quat::from_rotation_y(m.yaw))
                                .with_scale(Vec3::splat(pulse))
                        })
                    }),
                Mark::Pickup => route
                    .filter(|r| {
                        r.pickup != r.delivery
                            && !matches!(stage, Some(RouteStage::Carry | RouteStage::Deliver))
                    })
                    .map(|r| Transform::from_translation(stop(r.pickup, layout))),
                Mark::Particle(job, i) => {
                    burst
                        .filter(|(task, _, _, _)| *task == job)
                        .and_then(|(_, point, p, stage)| {
                            let start = i as f32 * 0.072;
                            let age = (p - start) * 1.42;
                            let high = if job == Job::Water && stage == RouteStage::Deliver {
                                0.66
                            } else {
                                0.42
                            };
                            particle(point, age, i, high, 0.56)
                        })
                }
                Mark::Star(i) => celebration.and_then(|(point, p)| {
                    particle(point + Vec3::Y * 0.62, p - i as f32 * 0.049, i, 0.91, 0.68)
                }),
                Mark::Flame => fire.map(|point| {
                    let flicker = 0.94 + (game.clock * 6.4).sin() * 0.08;
                    Transform::from_translation(point).with_scale(v(1.0, flicker, 0.96))
                }),
                Mark::Smoke(i) => fire.map(|point| {
                    let lift = (game.clock * 0.61 + i as f32 * 0.25).fract();
                    Transform::from_translation(
                        point + v(lift * 0.39, 0.46 + lift * 1.15, lift * -0.13),
                    )
                    .with_scale(Vec3::splat(0.62 + lift * 1.22))
                    .with_rotation(Quat::from_rotation_y(lift * 1.1 + i as f32))
                }),
                Mark::BrokenJars | Mark::ShadeSail | Mark::LostGroup => {
                    let want = match mark {
                        Mark::BrokenJars => Favor::BrokenJars,
                        Mark::ShadeSail => Favor::ShadeSail,
                        _ => Favor::LostGuests,
                    };
                    incident
                        .filter(|(favor, _)| *favor == want)
                        .map(|(_, lot)| {
                            let point = if want == Favor::LostGuests {
                                stop(Stop::Quay, layout) + v(0.59, 0.0, -0.27)
                            } else {
                                stop(Stop::Parcel(lot as u8), layout) + v(0.25, -0.027, 0.0)
                            };
                            Transform::from_translation(point).with_rotation(Quat::from_rotation_y(
                                (game.clock * 2.1).sin() * 0.06,
                            ))
                        })
                }
                Mark::Crates => incident
                    .filter(|(event, _)| *event == Favor::Crates)
                    .filter(|_| {
                        !manual.is_some_and(|(route, nili)| route.favor && !nili.stage.pickup())
                    })
                    .map(|_| Transform::from_translation(stop(Stop::Pier, layout))),
            }
        } else {
            None
        };
        let next = if let Some(pose) = effect {
            *transform = pose;
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visible != next {
            *visible = next;
        }
    }
}

fn particle(origin: Vec3, at: f32, index: usize, lift: f32, spread: f32) -> Option<Transform> {
    if !(0.0..1.0).contains(&at) {
        return None;
    }
    let angle = index as f32 / SPARKS as f32 * TAU + 0.23;
    let reach = (0.3 + at) * spread;
    let spot = origin
        + v(
            angle.cos() * reach,
            0.17 + (at * PI).sin() * lift + at * 0.27,
            angle.sin() * reach,
        );
    let size = (at * PI).sin().sqrt();
    Some(
        Transform::from_translation(spot)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, at * 3.2, angle, at * 4.1))
            .with_scale(Vec3::splat(size)),
    )
}

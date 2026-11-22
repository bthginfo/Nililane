use bevy::prelude::*;

use super::{landscape, lot_position, mesh::v};
use crate::{
    errands::{NiliControl, Route, Stop},
    layout::{Layout, lot_row},
    model::{Phase, Town},
};

const ROAD_EDGE: f32 = 1.62;
const PIER_HALF_WIDTH: f32 = 0.39;
const INTERACT_DISTANCE: f32 = 0.79;

pub fn ground(at: Vec3, layout: Layout) -> Vec2 {
    let reach = (layout.width() as f32 - 1.0) * super::LOT_STEP * 0.5;
    let quay = landscape::quay(layout);
    let x =
        at.x.clamp(-reach - 1.0, (reach + 1.0).max(quay + PIER_HALF_WIDTH));
    let far = if (quay - PIER_HALF_WIDTH..=quay + PIER_HALF_WIDTH).contains(&x) {
        6.27
    } else {
        ROAD_EDGE
    };
    Vec2::new(x, at.z.clamp(-ROAD_EDGE, far))
}

pub fn controlled_position(nili: NiliControl) -> Vec3 {
    let [x, z] = nili.position;
    let pier = ((z - 2.5) / 0.56).clamp(0.0, 1.0);
    v(x, 0.057 + pier * 0.078, z)
}

pub fn walk(nili: &mut NiliControl, delta: Vec2, layout: Layout) -> f32 {
    let prior = Vec2::from_array(nili.position);
    let x = if prior.y > ROAD_EDGE {
        let quay = landscape::quay(layout);
        (prior.x + delta.x).clamp(quay - PIER_HALF_WIDTH, quay + PIER_HALF_WIDTH)
    } else {
        prior.x + delta.x
    };
    let next = ground(v(x, 0.0, prior.y + delta.y), layout);
    let step = next - prior;
    if step.length_squared() > 0.000001 {
        nili.position = next.to_array();
        nili.yaw = step.x.atan2(step.y);
    }
    step.length()
}

pub fn can_work(nili: NiliControl, route: Route, layout: Layout) -> bool {
    nili.stage.action(route.job).is_some()
        && stop(nili.target(route), layout)
            .xz()
            .distance_squared(Vec2::from_array(nili.position))
            <= INTERACT_DISTANCE * INTERACT_DISTANCE
}

pub fn can_inspect(town: &Town, lot: usize, layout: Layout) -> bool {
    let Phase::Open(day) = &town.phase else {
        return true;
    };
    layout.contains(lot)
        && town.lots[lot].is_some()
        && day.nili.is_some_and(|nili| {
            entrance(lot, layout)
                .xz()
                .distance_squared(Vec2::from_array(nili.position))
                <= 0.96_f32.powi(2)
        })
}

pub fn nearby_lot(town: &Town, position: [f32; 2], layout: Layout) -> Option<usize> {
    let distance = |lot| {
        entrance(lot, layout)
            .xz()
            .distance_squared(Vec2::from_array(position))
    };
    layout
        .lots()
        .filter(|&lot| can_inspect(town, lot, layout))
        .min_by(|&a, &b| distance(a).total_cmp(&distance(b)))
}

pub fn entrance(lot: usize, layout: Layout) -> Vec3 {
    let lot_center = lot_position(lot, layout.width());
    v(
        lot_center.x,
        0.084,
        if lot_row(lot) == 0 { -1.86 } else { 1.89 },
    )
}

pub fn stop(at: Stop, layout: Layout) -> Vec3 {
    match at {
        Stop::Parcel(lot) => entrance(usize::from(lot), layout),
        Stop::Quay => v(landscape::quay(layout), 0.137, 3.16),
        Stop::Pier => v(landscape::quay(layout), 0.138, 6.10),
    }
}

pub struct Path {
    points: [Vec3; 4],
    distances: [f32; 4],
}

pub struct Motion {
    pub at: Vec3,
    pub yaw: f32,
    pub distance: f32,
    pub moving: bool,
}

impl Motion {
    pub fn still(at: Vec3, yaw: f32, distance: f32) -> Self {
        Self {
            at,
            yaw,
            distance,
            moving: false,
        }
    }
}

impl Path {
    pub fn road(from: Vec3, to: Vec3, lane: f32) -> Self {
        if (from.x - to.x).abs() < 0.1 {
            Self::direct(from, to)
        } else {
            Self::new([from, v(from.x, 0.074, lane), v(to.x, 0.074, lane), to])
        }
    }

    pub fn direct(from: Vec3, to: Vec3) -> Self {
        Self::new([from, from, to, to])
    }

    fn new(points: [Vec3; 4]) -> Self {
        let mut distances = [0.0; 4];
        for i in 1..4 {
            distances[i] = distances[i - 1] + points[i - 1].distance(points[i]);
        }
        Self { points, distances }
    }

    pub fn length(&self) -> f32 {
        self.distances[3]
    }

    fn point(&self, distance: f32) -> Vec3 {
        let distance = distance.clamp(0.0, self.length());
        for i in 1..3 {
            let corner = self.distances[i];
            let radius = (corner - self.distances[i - 1])
                .min(self.distances[i + 1] - corner)
                .mul_add(0.45, 0.0)
                .min(0.54);
            if radius > 0.015 && (distance - corner).abs() < radius {
                let point = self.points[i];
                let a = point.move_towards(self.points[i - 1], radius);
                let b = point.move_towards(self.points[i + 1], radius);
                let t = (distance - corner + radius) / (2.0 * radius);
                return a.lerp(point, t).lerp(point.lerp(b, t), t);
            }
        }
        for i in 1..4 {
            let width = self.distances[i] - self.distances[i - 1];
            if width > 0.0001 && distance <= self.distances[i] {
                return self.points[i - 1]
                    .lerp(self.points[i], (distance - self.distances[i - 1]) / width);
            }
        }
        self.points[3]
    }

    pub fn sample(&self, elapsed: f32, duration: f32, prior: f32) -> Motion {
        let t = (elapsed / duration.max(0.01)).clamp(0.0, 1.0);
        let distance = self.length() * t;
        let direction = self.point(distance + 0.17) - self.point(distance - 0.17);
        Motion {
            at: self.point(distance),
            yaw: direction.x.atan2(direction.z),
            distance: prior + distance,
            moving: (0.0..1.0).contains(&t) && self.length() > 0.025,
        }
    }
}

pub fn turn(from: f32, to: f32, fraction: f32) -> f32 {
    let delta =
        (to - from + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    from + delta * fraction.clamp(0.0, 1.0)
}

pub fn delivery_path(route: crate::errands::Route, layout: Layout) -> Path {
    Path::road(
        stop(route.pickup, layout),
        stop(route.delivery, layout),
        0.34,
    )
}

pub fn job_walk(route: crate::errands::Route, seconds: f32, layout: Layout) -> Motion {
    let from = stop(route.from, layout);
    let pickup = stop(route.pickup, layout);
    let reach = Path::road(from, pickup, 0.34);
    let load = delivery_path(route, layout);
    let start_yaw = reach.sample(1.0, 1.0, 0.0).yaw;
    if seconds < route.pickup_at {
        return reach.sample(
            seconds - route.started,
            route.pickup_at - route.started,
            0.0,
        );
    }
    if seconds < route.depart_at {
        let fraction =
            ((seconds - route.pickup_at) / (route.depart_at - route.pickup_at)).clamp(0.0, 1.0);
        let yaw = turn(start_yaw, load.sample(0.0, 1.0, 0.0).yaw, fraction);
        return Motion::still(pickup, yaw, reach.length());
    }
    load.sample(
        seconds - route.depart_at,
        route.arrival - route.depart_at,
        reach.length(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        campaign::CHAPTERS,
        errands::{Job, ManualStage},
    };

    #[test]
    fn every_harbor_has_walkable_entrances_and_a_safe_pier() {
        for chapter in &CHAPTERS {
            let layout = chapter.layout;
            let quay = landscape::quay(layout);
            let mut nili = NiliControl {
                position: [quay, 3.16],
                yaw: 0.0,
                stage: ManualStage::Pickup,
            };
            for _ in 0..60 {
                walk(&mut nili, Vec2::new(0.06, 0.10), layout);
            }
            assert!((nili.position[0] - quay).abs() < 0.391);
            assert!((nili.position[1] - 6.27).abs() < 0.001);
            for point in layout
                .lots()
                .map(|lot| Stop::Parcel(lot as u8))
                .chain([Stop::Quay, Stop::Pier])
            {
                let route = Route::new(
                    Job::Water,
                    false,
                    Stop::Quay,
                    point,
                    Stop::Quay,
                    0.0,
                    chapter,
                )
                .unwrap();
                nili.position = ground(stop(point, layout), layout).to_array();
                assert!(can_work(nili, route, layout));
            }
        }
    }
}

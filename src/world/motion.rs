use bevy::prelude::*;

use super::{landscape, lot_position, mesh::v};
use crate::{
    errands::Stop,
    layout::{Layout, lot_row},
};

pub fn entrance(lot: usize, layout: Layout) -> Vec3 {
    let lot_center = lot_position(lot, layout.width());
    v(
        lot_center.x,
        0.084,
        if lot_row(lot) == 0 { -1.23 } else { 1.23 },
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

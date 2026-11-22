use bevy::prelude::*;

use super::{Good, Level, Property, simulation::Deposit};

#[derive(Clone)]
pub struct Terrain {
    pub level: Level,
    pub plots: Vec<Vec2>,
    pub deposits: Vec<Deposit>,
    pub shrine: Vec2,
    pub relic: Vec2,
    pub radius: f32,
    shape: f32,
    angle: f32,
    hills: [(Vec2, f32, f32); 3],
    settlement: Vec2,
}

impl Terrain {
    pub fn new(level: Level) -> Self {
        let radius = level.diameter() * 0.5;
        let seed = level.index as u32 * 152_093 + 25_739;
        let angle = unit(seed) * 3.8 - 1.9;
        let plots: Vec<_> = (0..level.plots())
            .map(|i| {
                let width = 5 + usize::from(level.stage > 5) + usize::from(level.stage > 9);
                let row = i / width;
                let col = i % width;
                let x = (col as f32 - (width - 1) as f32 * 0.5) * 4.9;
                let z = match row {
                    0 => 6.4,
                    1 => -5.7,
                    2 => 12.5,
                    3 => -11.7,
                    4 => 18.6,
                    _ => -17.8,
                };
                Vec2::new(x + (row % 2) as f32 * 1.3, z)
            })
            .collect();
        let deposits = vec![
            Deposit {
                kind: Good::Wood,
                at: Vec2::new(-5.3, 3.1),
            },
            Deposit {
                kind: Good::Stone,
                at: Vec2::new(5.9, 2.4),
            },
            Deposit {
                kind: Good::Food,
                at: Vec2::new(-4.9, -2.2),
            },
        ];
        let peak = Vec2::new(-radius * 0.40, -radius * 0.50);
        let base = 5.6 + level.stage as f32 * 0.66;
        let hills = [
            (peak, base + unit(seed + 4) * 2.8, radius * 0.19),
            (
                Vec2::new(radius * 0.39, -radius * 0.39),
                base * 0.84,
                radius * 0.20,
            ),
            (
                Vec2::new(-radius * 0.58, radius * 0.18),
                base * 0.62,
                radius * 0.17,
            ),
        ];
        let shape = [0.12, 0.19, 0.24, 0.14][(level.stage + level.region as usize) % 4];
        let settlement = Vec2::new(
            14.7 + if level.stage > 5 { 1.2 } else { 0.0 },
            11.5 + level.stage as f32 * 0.61,
        );
        let mut map = Self {
            level,
            plots,
            deposits,
            shrine: Vec2::new(radius * 0.37, radius * 0.27),
            relic: peak,
            radius,
            shape,
            angle,
            hills,
            settlement,
        };
        for (i, kind) in [Good::Clay, Good::Ore, Good::Local].into_iter().enumerate() {
            let bearing = [0.45, -2.2, 2.73][i] + angle * 0.30;
            let first =
                radius * (0.41 + unit(seed + i as u32 * 405) * 0.06) * Vec2::from_angle(bearing)
                    + Vec2::new(2.0, -10.0);
            let positions = std::iter::once(first).chain((0..48).map(|attempt| {
                let direction = bearing + attempt as f32 * 0.79;
                let distance = radius * (0.42 + (attempt % 9) as f32 * 0.036);
                distance * Vec2::from_angle(direction)
            }));
            if let Some(at) = positions.into_iter().find(|&at| map.deposit_site(at)) {
                map.deposits.push(Deposit { kind, at });
            }
        }
        for (i, kind) in Good::RAW.into_iter().chain(Good::RAW).enumerate() {
            let a = i as f32 * 2.399 + angle;
            let distance = radius * (0.58 + unit(seed + i as u32 * 401) * 0.17);
            let at = distance * Vec2::from_angle(a);
            if map.deposit_site(at) {
                map.deposits.push(Deposit { kind, at });
            }
        }
        map
    }

    fn deposit_site(&self, at: Vec2) -> bool {
        self.shore(at) > 3.5
            && at.length() > 5.0
            && self.height(at) < 4.5
            && self.plots.iter().all(|plot| plot.distance(at) > 3.4)
            && self
                .deposits
                .iter()
                .all(|deposit| deposit.at.distance(at) > 4.0)
            && (self.level.plan().is_none() || self.shrine.distance(at) > 3.2)
    }

    pub fn radius_at(&self, angle: f32) -> f32 {
        let angle = angle + self.angle;
        self.radius
            * (0.96
                + self.shape * ((angle * 3.0).sin() * 0.44 + (angle * 5.0).cos() * 0.28)
                + 0.11 * (angle * 2.0).sin())
    }

    pub fn shore(&self, p: Vec2) -> f32 {
        let angle = p.y.atan2(p.x);
        self.radius_at(angle) - p.length()
    }

    pub fn height(&self, p: Vec2) -> f32 {
        let edge = self.shore(p);
        let base = (edge / 8.0).clamp(-1.4, 1.0) * 1.51 - 1.22;
        if edge < 0.0 {
            return base;
        }
        let fold = (p.x * 0.151 + self.angle).sin() * (p.y * 0.162).cos() * 0.42;
        let hill = self
            .hills
            .iter()
            .map(|&(at, h, w)| {
                let t = (1.0 - p.distance(at) / (w * 1.7)).max(0.0);
                h * t * t * (3.0 - 2.0 * t)
            })
            .sum::<f32>();
        let outer = (p.abs() - self.settlement).max(Vec2::ZERO).length();
        let blend = (outer * 0.155).clamp(0.0, 1.0);
        base + (fold + hill) * blend
    }

    const FOOTPRINT: Vec2 = Vec2::new(1.70, 1.51);

    pub fn inside_building(offset: Vec2) -> bool {
        offset.abs().cmple(Self::FOOTPRINT).all()
    }

    pub fn just_outside(offset: Vec2) -> Vec2 {
        let clearance = Self::FOOTPRINT - offset.abs();
        if clearance.x < clearance.y {
            Vec2::new((Self::FOOTPRINT.x + 0.07).copysign(offset.x), offset.y)
        } else {
            Vec2::new(offset.x, (Self::FOOTPRINT.y + 0.07).copysign(offset.y))
        }
    }

    pub fn walkable(&self, at: Vec2, lots: &[Option<Property>]) -> bool {
        self.accessible(at)
            && at.length_squared() > 1.55 * 1.55
            && self
                .plots
                .iter()
                .zip(lots)
                .all(|(plot, building)| building.is_none() || !Self::inside_building(at - *plot))
            && self
                .deposits
                .iter()
                .all(|deposit| deposit.at.distance_squared(at) > 0.44 * 0.44)
    }

    pub fn walk(&self, start: Vec2, delta: Vec2, lots: &[Option<Property>]) -> Vec2 {
        let steps = (delta.length() / 0.28).ceil().max(1.0) as u32;
        let stride = delta / steps as f32;
        let mut here = start;
        for _ in 0..steps {
            let candidate = [stride, Vec2::new(stride.x, 0.0), Vec2::new(0.0, stride.y)]
                .into_iter()
                .find(|&step| step != Vec2::ZERO && self.walkable(here + step, lots));
            if let Some(step) = candidate {
                here += step;
            } else {
                break;
            }
        }
        here
    }

    pub fn accessible(&self, at: Vec2) -> bool {
        self.shore(at) > 1.20 && at.is_finite()
    }
}

// Integer noise keeps islands identical on desktop and Android.
pub fn unit(seed: u32) -> f32 {
    let mut seed = seed.wrapping_add(0x9e37_79b9);
    seed ^= seed >> 16;
    seed = seed.wrapping_mul(0x21f0_aaad);
    seed ^= seed >> 15;
    seed = seed.wrapping_mul(0x735a_2d97);
    (seed >> 8) as f32 / 16_777_215.0
}

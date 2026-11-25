use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use super::{
    Good, Level, Property,
    encounters::SLOTS,
    simulation::{Deposit, Scenery},
    twists::{FERTILE_SOIL, ORE_VEIN, RICH_WOODS, Twist},
};

pub const HALL_DOOR: Vec2 = Vec2::new(0.0, 3.2);

#[derive(Clone, Copy)]
pub struct ResourceZone {
    pub kind: Good,
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Clone, Copy)]
enum Reach {
    Near,
    Middle,
    Far,
}

#[derive(Clone, Copy)]
pub struct FishingGround {
    pub at: Vec2,
    pub launch: Vec2,
    pub lot: usize,
}

#[derive(Clone)]
pub struct Terrain {
    pub level: Level,
    pub plots: Vec<Vec2>,
    pub deposits: Vec<Deposit>,
    pub zones: Vec<ResourceZone>,
    pub fishing: Vec<FishingGround>,
    pub shrine: Vec2,
    pub relic: Vec2,
    pub outpost: Vec2,
    pub events: [Vec2; SLOTS],
    pub finds: [Vec2; SLOTS],
    pub shore_intake: Vec2,
    pub trade_pier: Vec2,
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
        let plots: Vec<_> = (0..level.inland_plots())
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
            Deposit::new(Good::Wood, Vec2::new(-5.3, 3.1)),
            Deposit::new(Good::Stone, Vec2::new(5.9, 2.4)),
            Deposit::new(Good::Food, Vec2::new(-4.9, -2.2)),
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
            zones: Vec::new(),
            fishing: Vec::new(),
            shrine: Vec2::new(radius * 0.37, radius * 0.27),
            relic: peak,
            outpost: Vec2::ZERO,
            events: [Vec2::ZERO; SLOTS],
            finds: [Vec2::ZERO; SLOTS],
            shore_intake: Vec2::ZERO,
            trade_pier: Vec2::ZERO,
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
                map.deposits.push(Deposit::new(kind, at));
            }
        }
        for (i, kind) in Good::RAW.into_iter().chain(Good::RAW).enumerate() {
            let a = i as f32 * 2.399 + angle;
            let distance = radius * (0.58 + unit(seed + i as u32 * 401) * 0.17);
            let at = distance * Vec2::from_angle(a);
            if map.deposit_site(at) {
                map.deposits.push(Deposit::new(kind, at));
            }
        }
        if level.region == crate::campaign::Region::Nordic {
            for sector in [0.18, 2.37] {
                let direction = (0..32)
                    .map(|n| sector + angle * 0.28 + n as f32 * 0.23)
                    .find(|&a| {
                        let at = Vec2::from_angle(a) * (map.radius_at(a) - 3.5);
                        map.plots.iter().all(|p| p.distance(at) > 6.0)
                            && map.shrine.distance(at) > 5.0
                    })
                    .unwrap_or(sector);
                let shore = map.radius_at(direction);
                let ray = Vec2::from_angle(direction);
                let lot = map.plots.len();
                map.plots.push(ray * (shore - 3.5));
                map.fishing.push(FishingGround {
                    at: ray * (shore + 9.0 + level.stage as f32 * 0.52),
                    launch: ray * (shore + 1.65),
                    lot,
                });
            }
        }
        // Hills and the shrine move only after the node list above is settled, so the
        // node order of saved islands stays the same.
        map.vary_landmarks(seed);
        map.outpost = (0..90)
            .map(|n| {
                let direction = 0.78 + angle * 0.5 + n as f32 * 0.61;
                Vec2::from_angle(direction) * radius * (0.52 + (n % 7) as f32 * 0.027)
            })
            .find(|&at| {
                map.deposit_site(at)
                    && map.shrine.distance(at) > 10.0
                    && map.relic.distance(at) > 9.0
            })
            .unwrap_or(Vec2::new(radius * 0.57, 1.0));
        map.fill_resources();
        map.enrich();
        map.place_encounters();
        map
    }

    /// Makes nodes richer where the island's twist says so.
    fn enrich(&mut self) {
        match self.level.twist() {
            Some(Twist::FertileSoil) => {
                for deposit in self.deposits.iter_mut().filter(|d| d.kind == Good::Food) {
                    deposit.extra = (deposit.base_units() * FERTILE_SOIL).div_ceil(100);
                }
            }
            Some(Twist::OreVein) => {
                let center = self.zone(Good::Ore).center;
                if let Some(vein) = self
                    .deposits
                    .iter_mut()
                    .filter(|d| d.kind == Good::Ore)
                    .min_by(|a, b| a.at.distance(center).total_cmp(&b.at.distance(center)))
                {
                    vein.extra = ORE_VEIN;
                }
            }
            _ => {}
        }
    }

    /// Turns the hills and the shrine together around the island and varies the hills a
    /// little. Keeps the classic layout when no turn leaves the relic high, dry and free.
    fn vary_landmarks(&mut self, seed: u32) {
        let classic = (self.hills, self.shrine);
        let start = unit(seed + 913) * std::f32::consts::TAU;
        let candidates = (0..10).map(|attempt| start + attempt as f32 * 2.47);
        for turn in candidates {
            let rotation = Vec2::from_angle(turn);
            for (i, hill) in self.hills.iter_mut().enumerate() {
                let (at, height, width) = classic.0[i];
                let key = seed + 960 + i as u32 * 37;
                let reach = 0.94 + unit(key) * 0.12;
                hill.0 = rotation.rotate(at) * reach;
                let lift = if i == 0 {
                    1.0
                } else {
                    0.82 + unit(key + 5) * 0.36
                };
                hill.1 = height * lift;
                hill.2 = width * (0.9 + unit(key + 9) * 0.2);
            }
            self.shrine = rotation.rotate(classic.1);
            self.relic = self.hills[0].0;
            let clear = |at: Vec2, gap: f32| self.plots.iter().all(|p| p.distance(at) > gap);
            if self.shore(self.shrine) > 4.0
                && clear(self.shrine, 6.0)
                && self.shore(self.relic) > 5.0
                && clear(self.relic, 6.0)
                && self.height(self.relic) > 3.8
            {
                return;
            }
        }
        (self.hills, self.shrine) = classic;
        self.relic = self.hills[0].0;
    }

    fn place_encounters(&mut self) {
        let count = self.level.encounter_count();
        for slot in 0..count {
            let direction = self.angle + 0.88 + slot as f32 * 2.4;
            self.events[slot] = self.encounter_position(direction, 11.5, 4.8, &self.events[..slot]);
        }
        for slot in 0..count {
            self.finds[slot] = self.encounter_position(
                3.2 + self.angle + slot as f32 * 2.28,
                self.radius * 0.53,
                7.8,
                &self.events[..count],
            );
        }
        self.shore_intake = (0..80)
            .map(|attempt| {
                let direction = 0.92 + self.angle * 0.3 + attempt as f32 * 0.21;
                Vec2::from_angle(direction) * (self.radius_at(direction) - 2.08)
            })
            .find(|&at| {
                self.plots.iter().all(|p| p.distance(at) > 5.0)
                    && self.events[..count].iter().all(|p| p.distance(at) > 4.0)
            })
            .unwrap_or_else(|| Vec2::X * (self.radius_at(0.0) - 2.08));
        if self.level.has_trading() {
            let far_side = self.shore_intake.to_angle() + std::f32::consts::PI;
            self.trade_pier = (0..80)
                .map(|attempt| {
                    let direction = far_side + attempt as f32 * 0.23;
                    Vec2::from_angle(direction) * (self.radius_at(direction) - 2.6)
                })
                .find(|&at| {
                    self.shore_intake.distance(at) > 8.0
                        && self.plots.iter().all(|p| p.distance(at) > 5.4)
                })
                .unwrap_or_else(|| Vec2::from_angle(far_side) * (self.radius_at(far_side) - 2.6));
        }
    }

    fn encounter_position(
        &self,
        direction: f32,
        radius: f32,
        distance: f32,
        reserved: &[Vec2],
    ) -> Vec2 {
        (0..260)
            .map(|attempt| {
                let step = (attempt % 15) as f32 * 0.73;
                Vec2::from_angle(direction + attempt as f32 * 2.399) * (radius + step)
            })
            .find(|&at| {
                self.deposit_site(at)
                    && self.relic.distance(at) > 4.5
                    && reserved.iter().all(|p| p.distance(at) > distance)
            })
            .unwrap_or_else(|| Vec2::from_angle(direction) * (self.radius_at(direction) - 5.0))
    }

    fn fill_resources(&mut self) {
        let seed = self.level.index as u32 * 8763 + 431;
        let legacy = std::mem::take(&mut self.deposits);
        let layout = self.zone_layout(seed);
        let settlement = self.plots[..self.level.inland_plots()]
            .iter()
            .map(|p| p.length())
            .fold(0.0, f32::max);
        self.zones = Good::RAW
            .into_iter()
            .zip(layout)
            .map(|(kind, (angle, reach))| {
                let size = match kind {
                    Good::Wood => 0.27,
                    Good::Stone => 0.235,
                    _ => 0.195,
                };
                let shore = self.radius_at(angle);
                let distance = match reach {
                    Reach::Near => (shore * 0.45).max(settlement + self.radius * size * 0.55),
                    Reach::Middle => shore * (0.62 + unit(seed + kind as u32 * 53) * 0.08),
                    Reach::Far => shore * 0.78,
                };
                let mut center = Vec2::from_angle(angle) * distance;
                if self.level.order_count() > 0
                    && center.distance(self.outpost) < self.radius * 0.15
                {
                    center = Vec2::from_angle(angle - 0.25) * distance;
                }
                ResourceZone {
                    kind,
                    center,
                    radius: self.radius * size,
                }
            })
            .collect();
        // Keep the identities used by existing citizen assignments, even when their sites move.
        for (index, deposit) in legacy.into_iter().enumerate() {
            let at = if self.level.stage == 0 && index < 3 {
                deposit.at
            } else {
                self.zone_site(self.zone(deposit.kind), seed + index as u32 * 4903)
                    .map_or(deposit.at, |(at, _)| at)
            };
            self.deposits.push(Deposit::new(deposit.kind, at));
        }
        for zone in self.zones.clone() {
            let desired = match zone.kind {
                Good::Wood => 24 + self.level.stage * 4,
                Good::Stone => 25 + self.level.stage * 4,
                Good::Clay | Good::Ore => 13 + self.level.stage,
                _ => 13 + self.level.stage,
            };
            let present = self
                .deposits
                .iter()
                .filter(|deposit| deposit.kind == zone.kind)
                .count();
            for serial in present..desired {
                if !self.plant(zone, seed, serial) {
                    break;
                }
            }
        }
        // Extra trees go to the end of the list, so the nodes above keep their order.
        if self.level.has_twist(Twist::RichWoods) {
            let zone = self.zone(Good::Wood);
            let planted = self
                .deposits
                .iter()
                .filter(|d| d.kind == Good::Wood)
                .count();
            let more = planted * usize::from(RICH_WOODS) / 100;
            for serial in planted..planted + more {
                if !self.plant(zone, seed, serial) {
                    break;
                }
            }
        }
    }

    /// Adds the node number `serial` of a district; false when the district is full.
    fn plant(&mut self, zone: ResourceZone, seed: u32, serial: usize) -> bool {
        let key = seed + (zone.kind as u32 + 1) * 72_901 + serial as u32 * 1_013;
        let Some((at, variation)) = self.zone_site(zone, key) else {
            return false;
        };
        let scenery = matches!(zone.kind, Good::Wood | Good::Stone).then(|| Scenery {
            seed: variation,
            size: 0.70 + unit(variation + 81) * 0.96,
        });
        self.deposits.push(Deposit {
            scenery,
            ..Deposit::new(zone.kind, at)
        });
        true
    }

    /// Direction and distance of each raw material district, in [`Good::RAW`] order.
    /// Every island shuffles the six sectors; one district lies close to the
    /// settlement and one far out.
    fn zone_layout(&self, seed: u32) -> [(f32, Reach); 6] {
        use std::f32::consts::TAU;
        let mut sectors = [0, 1, 2, 3, 4, 5];
        for i in (1..sectors.len()).rev() {
            let pick = (unit(seed + 7_177 + i as u32 * 31) * (i + 1) as f32) as usize;
            sectors.swap(i, pick.min(i));
        }
        let turn = unit(seed + 6_101) * TAU;
        // The first island of a region keeps wood close and the regional goods far out.
        let (near, far) = if self.level.stage == 0 {
            (0, 5)
        } else {
            let near = (unit(seed + 6_203) * 6.0) as usize % 6;
            let far = (near + 1 + (unit(seed + 6_211) * 5.0) as usize % 5) % 6;
            (near, far)
        };
        std::array::from_fn(|i| {
            let jitter = (unit(seed + 6_301 + i as u32 * 13) - 0.5) * 0.3;
            let reach = if i == near {
                Reach::Near
            } else if i == far {
                Reach::Far
            } else {
                Reach::Middle
            };
            (turn + sectors[i] as f32 * TAU / 6.0 + jitter, reach)
        })
    }

    pub fn zone(&self, good: Good) -> ResourceZone {
        *self
            .zones
            .iter()
            .find(|zone| zone.kind == good)
            .expect("raw material has a district")
    }

    pub fn district(&self, at: Vec2) -> Option<Good> {
        self.zones
            .iter()
            .min_by(|a, b| {
                (a.center.distance(at) / a.radius).total_cmp(&(b.center.distance(at) / b.radius))
            })
            .filter(|zone| zone.center.distance(at) <= zone.radius)
            .map(|zone| zone.kind)
    }

    fn zone_site(&self, zone: ResourceZone, seed: u32) -> Option<(Vec2, u32)> {
        let spacing = if matches!(zone.kind, Good::Wood | Good::Stone) {
            1.62
        } else {
            2.10
        };
        (0..240).find_map(|attempt| {
            let key = seed + attempt * 181;
            let angle = std::f32::consts::TAU * unit(key);
            let expand = if attempt >= 140 { 1.22 } else { 1.0 };
            let at = zone.center
                + zone.radius * expand * unit(key + 71).sqrt() * Vec2::from_angle(angle);
            (self.shore(at) > 3.25
                && at.length() > 9.4
                && self.plots.iter().all(|plot| plot.distance(at) > 3.35)
                && self
                    .deposits
                    .iter()
                    .all(|deposit| deposit.at.distance(at) > spacing)
                && self.shrine.distance(at) > 4.0
                && self.relic.distance(at) > 3.0
                && (self.level.order_count() == 0 || self.outpost.distance(at) > 4.0))
                .then_some((at, key % 100_003))
        })
    }

    pub fn vegetation(&self) -> impl Iterator<Item = (u32, Vec2, f32, f32)> + '_ {
        let seed = self.level.index as u32 * 8763 + 431;
        (0..(360 + self.level.stage as u32 * 44)).filter_map(move |i| {
            let at = Vec2::new(
                unit(seed + 531 * i) * 2.0 - 1.0,
                unit(seed + 711 * i + 6) * 2.0 - 1.0,
            ) * self.radius;
            (self.shore(at) >= 3.3
                && at.length() >= 8.0
                && self.plots.iter().all(|p| p.distance(at) >= 3.15)
                && self.deposits.iter().all(|d| d.at.distance(at) >= 1.12)
                && self.shrine.distance(at) >= 4.0
                && (self.level.order_count() == 0 || self.outpost.distance(at) >= 4.0)
                && self.relic.distance(at) >= 2.2
                && (0..self.level.encounter_count()).all(|slot| {
                    self.events[slot].distance(at) > 2.6 && self.finds[slot].distance(at) > 1.2
                }))
            .then(|| {
                let choice = unit(seed + i * 7 + 7);
                let size = 0.58 + unit(seed + i * 83 + 196) * 0.94;
                (i, at, choice, size)
            })
        })
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
            && (self.level.order_count() == 0 || self.outpost.distance(at) > 3.6)
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

    /// The nearest walkable spot outside the footprint of the building on `plot`.
    pub fn way_out(&self, plot: Vec2, at: Vec2, lots: &[Option<Property>]) -> Vec2 {
        let offset = at - plot;
        let mut exits: Vec<Vec2> = [0.07, 0.5, 1.2]
            .into_iter()
            .flat_map(|margin| {
                let edge = Self::FOOTPRINT + margin;
                let near = edge * Vec2::new(1.0f32.copysign(offset.x), 1.0f32.copysign(offset.y));
                [
                    Vec2::new(near.x, offset.y),
                    Vec2::new(offset.x, near.y),
                    Vec2::new(-near.x, offset.y),
                    Vec2::new(offset.x, -near.y),
                    near,
                    Vec2::new(-near.x, near.y),
                    Vec2::new(near.x, -near.y),
                    -near,
                ]
            })
            .map(|exit| plot + exit)
            .collect();
        exits.sort_by(|a, b| a.distance_squared(at).total_cmp(&b.distance_squared(at)));
        exits
            .into_iter()
            .find(|&exit| self.walkable(exit, lots))
            .unwrap_or(HALL_DOOR)
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

    /// `at` when someone can stand there, otherwise the closest free spot around it.
    /// Saves from older layouts may place walkers inside a tree or a rock.
    pub fn free_spot(&self, at: Vec2, lots: &[Option<Property>]) -> Vec2 {
        if self.walkable(at, lots) {
            return at;
        }
        (1..=24)
            .flat_map(|ring| {
                (0..16).map(move |step| {
                    at + Vec2::from_angle(step as f32 * std::f32::consts::TAU / 16.0)
                        * (ring as f32 * 0.3)
                })
            })
            .find(|&spot| self.walkable(spot, lots))
            .unwrap_or(HALL_DOOR)
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

    pub fn route(
        &self,
        start: Vec2,
        goal: Vec2,
        reach: f32,
        lots: &[Option<Property>],
    ) -> Option<Vec<Vec2>> {
        let can_walk = |at| self.walkable(at, lots);
        let clear = |from: Vec2, to: Vec2| {
            let steps = (from.distance(to) / 0.35).ceil().max(1.0) as u32;
            (1..=steps).all(|i| can_walk(from.lerp(to, i as f32 / steps as f32)))
        };
        let distance = start.distance(goal);
        if distance <= reach {
            return Some(Vec::new());
        }
        let stop = start.lerp(goal, ((distance - reach + 0.09) / distance).min(1.0));
        if clear(start, stop) {
            return Some(vec![stop]);
        }
        let mut paths: HashMap<IVec2, Option<IVec2>> = HashMap::new();
        let mut open = VecDeque::new();
        let origin = start.round().as_ivec2();
        for y in -1..=1 {
            for x in -1..=1 {
                let at = origin + IVec2::new(x, y);
                if clear(start, at.as_vec2()) {
                    open.push_back(at);
                    paths.insert(at, None);
                }
            }
        }
        while let Some(at) = open.pop_front() {
            let point = at.as_vec2();
            if point.distance(goal) <= reach {
                let mut route = vec![point];
                let mut previous = paths[&at];
                while let Some(parent) = previous {
                    route.push(parent.as_vec2());
                    previous = paths[&parent];
                }
                return Some(route);
            }
            for direction in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
                let next = at + direction;
                if !paths.contains_key(&next) && clear(point, next.as_vec2()) {
                    paths.insert(next, Some(at));
                    open.push_back(next);
                }
            }
        }
        None
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

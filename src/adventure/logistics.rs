use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Good, Island, Task, Terrain, terrain::HALL_DOOR};

pub const NILI_CAPACITY: u16 = 10;
const CITIZEN_CAPACITY: u16 = 6;
pub const RESOURCE_REACH: f32 = 2.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cargo {
    pub kind: Good,
    pub units: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Haul {
    pub position: [f32; 2],
    pub facing: f32,
    pub site: Option<usize>,
    pub cargo: Option<Cargo>,
    pub work: f32,
    pub retiring: bool,
    #[serde(skip)]
    path: Vec<Vec2>,
    #[serde(skip)]
    retry: f32,
    #[serde(skip)]
    pub gait: f32,
    #[serde(skip)]
    pub moving: bool,
}

impl Default for Haul {
    fn default() -> Self {
        Self {
            position: HALL_DOOR.to_array(),
            facing: 0.0,
            site: None,
            cargo: None,
            work: 0.0,
            retiring: false,
            path: Vec::new(),
            retry: 0.0,
            gait: 0.0,
            moving: false,
        }
    }
}

impl Haul {
    fn face(&mut self, point: Vec2) {
        let delta = point - Vec2::from_array(self.position);
        if delta.length_squared() > 0.0001 {
            self.facing = delta.x.atan2(delta.y);
        }
    }

    fn reset_site(&mut self, site: Option<usize>) {
        self.site = site;
        self.work = 0.0;
        self.path.clear();
        self.retry = 0.0;
    }

    fn go(&mut self, island: &Island, map: &Terrain, goal: Vec2, radius: f32, dt: f32) {
        let start = Vec2::from_array(self.position);
        self.retry = (self.retry - dt).max(0.0);
        if self.path.is_empty() && self.retry == 0.0 {
            self.path = map
                .route(start, goal, radius, &island.lots)
                .unwrap_or_default();
            if self.path.is_empty() {
                self.retry = 1.2;
            }
        }
        while self.path.last().is_some_and(|p| start.distance(*p) < 0.07) {
            self.path.pop();
        }
        let Some(&waypoint) = self.path.last() else {
            return;
        };
        let speed = if self.cargo.is_some() { 3.55 } else { 4.3 };
        let delta = (waypoint - start).clamp_length_max(dt * speed);
        let next = map.walk(start, delta, &island.lots);
        let travelled = start.distance(next);
        if travelled < 0.001 {
            self.path.clear();
            self.retry = 0.35;
        } else {
            self.face(next);
            self.position = next.to_array();
            self.moving = true;
            self.gait += travelled * 5.7;
        }
    }
}

impl Island {
    pub fn prepare_logistics(&mut self, map: &Terrain) {
        self.remaining.truncate(map.deposits.len());
        self.remaining.extend(
            map.deposits[self.remaining.len()..]
                .iter()
                .map(|d| d.units()),
        );
        self.hauls.resize_with(self.workers.len(), Haul::default);
        for haul in &mut self.hauls {
            if !map.accessible(Vec2::from_array(haul.position)) {
                haul.position = HALL_DOOR.to_array();
            }
            if !haul.work.is_finite() || haul.site.is_some_and(|site| site >= map.deposits.len()) {
                haul.reset_site(None);
            }
        }
    }

    pub fn remaining(&self, map: &Terrain, site: usize) -> u16 {
        map.deposits.get(site).map_or(0, |deposit| {
            self.remaining
                .get(site)
                .copied()
                .unwrap_or_else(|| deposit.units())
        })
    }

    pub fn available_near(&self, map: &Terrain, kind: Good, from: Vec2) -> Option<usize> {
        map.deposits
            .iter()
            .enumerate()
            .filter(|(id, deposit)| deposit.kind == kind && self.remaining(map, *id) > 0)
            .min_by(|(_, a), (_, b)| {
                from.distance_squared(a.at)
                    .total_cmp(&from.distance_squared(b.at))
            })
            .map(|(id, _)| id)
    }

    fn extract(&mut self, map: &Terrain, site: usize, capacity: u16) -> Option<Cargo> {
        self.remaining.extend(
            map.deposits[self.remaining.len().min(map.deposits.len())..]
                .iter()
                .map(|d| d.units()),
        );
        let deposit = map.deposits.get(site)?;
        let left = self.remaining.get_mut(site)?;
        let units = (*left).min(capacity);
        *left -= units;
        (units > 0).then_some(Cargo {
            kind: deposit.kind,
            units,
        })
    }

    pub fn room_for(&self, kind: Good) -> u16 {
        if self.shipment.is_some() {
            return 0;
        }
        match self.cargo {
            None => NILI_CAPACITY,
            Some(load) if load.kind == kind => NILI_CAPACITY.saturating_sub(load.units),
            Some(_) => 0,
        }
    }

    pub fn carry_resource(&mut self, map: &Terrain, site: usize) -> Option<Cargo> {
        let kind = map.deposits.get(site)?.kind;
        let load = self.extract(map, site, self.room_for(kind))?;
        let total = self.cargo.map_or(0, |cargo| cargo.units) + load.units;
        self.cargo = Some(Cargo {
            units: total,
            ..load
        });
        Some(load)
    }

    pub fn unload(&mut self) -> Option<Cargo> {
        let cargo = self.cargo?;
        let units = self.add(cargo.kind, cargo.units);
        self.cargo = (units < cargo.units).then_some(Cargo {
            units: cargo.units - units,
            ..cargo
        });
        (units > 0).then_some(Cargo { units, ..cargo })
    }

    pub fn tick_hauling(&mut self, map: &Terrain, dt: f32) -> bool {
        if !self
            .workers
            .iter()
            .any(|task| matches!(task, Task::Gather(_)))
        {
            return false;
        }
        let mut updated = false;
        let mut hauls = std::mem::take(&mut self.hauls);
        hauls.resize_with(self.workers.len(), Haul::default);
        let mut reserved: Vec<_> = hauls
            .iter()
            .map(|haul| haul.site.filter(|_| haul.cargo.is_none()))
            .collect();
        let mut retired = Vec::new();
        for (index, haul) in hauls.iter_mut().enumerate() {
            let Task::Gather(anchor) = self.workers[index] else {
                continue;
            };
            let hall = HALL_DOOR + Vec2::X * ((index % 5) as f32 * 0.32 - 0.64);
            haul.moving = false;
            if let Some(load) = haul.cargo {
                if Vec2::from_array(haul.position).distance(hall) <= 0.55 {
                    let stored = self.add(load.kind, load.units);
                    updated |= stored > 0;
                    if stored == load.units {
                        haul.cargo = None;
                        haul.reset_site(None);
                        if haul.retiring {
                            retired.push(index);
                        }
                    } else {
                        haul.cargo = Some(Cargo {
                            units: load.units - stored,
                            ..load
                        });
                    }
                } else {
                    haul.go(self, map, hall, 0.51, dt);
                }
                continue;
            }
            let Some(deposit) = map.deposits.get(anchor) else {
                continue;
            };
            if haul.site.is_none() && self.free_storage() == 0 {
                continue;
            }
            if haul.site.is_none_or(|site| self.remaining(map, site) == 0) {
                let site = map
                    .deposits
                    .iter()
                    .enumerate()
                    .filter(|(site, node)| {
                        node.kind == deposit.kind
                            && self.remaining(map, *site) > 0
                            && !reserved.contains(&Some(*site))
                    })
                    .min_by(|(a_id, a), (b_id, b)| {
                        (a_id != &anchor).cmp(&(b_id != &anchor)).then_with(|| {
                            a.at.distance_squared(hall)
                                .total_cmp(&b.at.distance_squared(hall))
                        })
                    })
                    .map(|(site, _)| site);
                if haul.site != site {
                    haul.reset_site(site);
                    reserved[index] = site;
                }
            }
            let Some(site) = haul.site else {
                continue;
            };
            let node = map.deposits[site];
            if Vec2::from_array(haul.position).distance(node.at) <= RESOURCE_REACH {
                haul.face(node.at);
                haul.work += dt;
                if haul.work >= node.kind.work_seconds() {
                    haul.cargo = self.extract(map, site, CITIZEN_CAPACITY);
                    haul.reset_site(None);
                    reserved[index] = None;
                    updated |= haul.cargo.is_some();
                }
            } else {
                haul.go(self, map, node.at, RESOURCE_REACH - 0.09, dt);
            }
        }
        for index in retired.into_iter().rev() {
            self.workers.remove(index);
            hauls.remove(index);
        }
        self.hauls = hauls;
        updated
    }
}

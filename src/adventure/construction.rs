use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    Build, Good, Island, Level, Property, Terrain,
    logistics::{Cargo, Haul},
    simulation::load_goods,
    terrain::HALL_DOOR,
};

const LOAD: u16 = 6;
const SITE_REACH: f32 = 2.65;
pub const MAX_BUILD_CREW: u8 = 6;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BuildingSite {
    pub property: Property,
    #[serde(deserialize_with = "load_goods")]
    pub needed: [u16; Good::ALL.len()],
    #[serde(deserialize_with = "load_goods")]
    pub materials: [u16; Good::ALL.len()],
    pub work: f32,
    pub crew: u8,
}

impl BuildingSite {
    pub fn missing(&self, good: Good) -> u16 {
        self.needed[good as usize].saturating_sub(self.materials[good as usize])
    }

    pub fn ready(&self) -> bool {
        self.needed
            .iter()
            .zip(self.materials)
            .all(|(&need, have)| have >= need)
    }

    pub fn seconds(&self) -> f32 {
        5.0 + f32::from(self.needed.iter().sum::<u16>()) * 0.11
            + f32::from(self.property.tier.saturating_sub(1)) * 3.0
    }

    pub fn progress(&self) -> f32 {
        (self.work / self.seconds()).clamp(0.0, 1.0)
    }

    fn accept(&mut self, load: Cargo) -> u16 {
        let units = self.missing(load.kind).min(load.units);
        self.materials[load.kind as usize] += units;
        units
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Builder {
    pub lot: usize,
    pub walk: Haul,
    pub returning: bool,
}

impl Island {
    pub fn site(&self, lot: usize) -> Option<&BuildingSite> {
        self.construction.get(lot)?.as_ref()
    }

    pub fn plan_build(&mut self, level: Level, lot: usize, kind: Build) -> bool {
        let Some(existing) = self.lots.get(lot).copied() else {
            return false;
        };
        if self.site(lot).is_some()
            || existing.is_some_and(|p| p.kind != kind || p.tier >= crate::model::MAX_LEVEL)
            || (existing.is_none() && !kind.fits(level, lot))
        {
            return false;
        }
        let tier = existing.map_or(1, |p| p.tier + 1);
        if self.locked(level, kind, tier) || !Build::all(level).contains(&kind) {
            return false;
        }
        self.construction.resize_with(self.lots.len(), || None);
        let mut needed = [0; Good::ALL.len()];
        for (good, units) in kind.cost(tier) {
            needed[good as usize] += units;
        }
        self.construction[lot] = Some(BuildingSite {
            property: Property { kind, tier },
            needed,
            materials: [0; Good::ALL.len()],
            work: 0.0,
            crew: 1,
        });
        true
    }

    pub fn with_crew(&mut self, lot: usize, change: i8) -> bool {
        let Some(site) = self.construction.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        let crew = site.crew.saturating_add_signed(change).min(MAX_BUILD_CREW);
        let changed = site.crew != crew;
        site.crew = crew;
        changed
    }

    pub fn site_workers(&self, lot: usize) -> usize {
        self.builders
            .iter()
            .filter(|b| b.lot == lot && !b.returning)
            .count()
    }

    pub fn site_supply(&self, lot: usize) -> Option<Cargo> {
        let site = self.site(lot)?;
        let mut incoming = [0u16; Good::ALL.len()];
        for load in self
            .builders
            .iter()
            .filter(|b| b.lot == lot && !b.returning)
            .filter_map(|b| b.walk.cargo)
            .chain(self.cargo.filter(|_| self.building_load == Some(lot)))
        {
            incoming[load.kind as usize] = incoming[load.kind as usize].saturating_add(load.units);
        }
        Good::ALL.into_iter().find_map(|good| {
            let units = site
                .missing(good)
                .saturating_sub(incoming[good as usize])
                .min(self.amount(good));
            (units > 0).then_some(Cargo { kind: good, units })
        })
    }

    fn take_site_supply(&mut self, lot: usize, limit: u16) -> Option<Cargo> {
        let mut load = self.site_supply(lot)?;
        load.units = load.units.min(limit);
        self.stores[load.kind as usize] -= load.units;
        Some(load)
    }

    pub fn take_building_load(&mut self, lot: usize) -> Option<Cargo> {
        if self.has_cargo() {
            return None;
        }
        let load = self.take_site_supply(lot, self.capacity())?;
        self.cargo = Some(load);
        self.building_load = Some(lot);
        Some(load)
    }

    pub fn drop_building_load(&mut self, lot: usize) -> Option<Cargo> {
        let mut load = self.cargo?;
        let site = self.construction.get_mut(lot)?.as_mut()?;
        let accepted = site.accept(load);
        if accepted == 0 {
            return None;
        }
        load.units -= accepted;
        self.cargo = (load.units > 0).then_some(load);
        if self.cargo.is_none() {
            self.building_load = None;
        }
        Some(Cargo {
            units: accepted,
            ..load
        })
    }

    pub fn cancel_build(&mut self, lot: usize) -> bool {
        let Some(site) = self.site(lot) else {
            return false;
        };
        let materials = site.materials;
        let raw: u16 = Good::ALL
            .into_iter()
            .filter(|good| good.uses_storage())
            .map(|good| materials[good as usize])
            .sum();
        if raw > self.free_storage()
            || Good::ALL.into_iter().any(|good| {
                self.amount(good)
                    .checked_add(materials[good as usize])
                    .is_none()
            })
        {
            return false;
        }
        self.construction[lot] = None;
        for good in Good::ALL {
            self.add(good, materials[good as usize]);
        }
        if self.building_load == Some(lot) {
            self.building_load = None;
        }
        for builder in self.builders.iter_mut().filter(|b| b.lot == lot) {
            builder.returning = true;
            builder.walk.reset_site(None);
        }
        true
    }

    /// Moves walkers, dropped loads and Nili out of a finished building, so nobody is
    /// trapped inside walls that went up around them.
    pub(super) fn evict(&mut self, map: &Terrain, lot: usize) {
        let Some(&plot) = map.plots.get(lot) else {
            return;
        };
        if self.lots.get(lot).copied().flatten().is_none() {
            return;
        }
        let lots = self.lots.clone();
        let exit = |at: [f32; 2]| {
            let at = Vec2::from_array(at);
            Terrain::inside_building(at - plot).then(|| map.way_out(plot, at, &lots))
        };
        for walk in self
            .hauls
            .iter_mut()
            .chain(self.builders.iter_mut().map(|builder| &mut builder.walk))
            .chain(self.courier.iter_mut().map(|courier| &mut courier.walk))
        {
            if let Some(at) = exit(walk.position) {
                walk.relocate(at);
            }
        }
        for pile in self.dropped.iter_mut().flatten() {
            if let Some(at) = exit(pile.position) {
                pile.position = at.to_array();
            }
        }
        if let Some(at) = exit(self.position) {
            self.position = at.to_array();
        }
    }

    pub(super) fn prepare_construction(&mut self, map: &Terrain) {
        self.construction.resize_with(self.lots.len(), || None);
        for builder in &mut self.builders {
            if !map.accessible(Vec2::from_array(builder.walk.position)) {
                builder.walk.position = HALL_DOOR.to_array();
            }
        }
    }

    pub fn tick_construction(
        &mut self,
        map: &Terrain,
        dt: f32,
        nili: Option<usize>,
    ) -> (bool, Vec<usize>) {
        if self.builders.is_empty() && self.construction.iter().all(Option::is_none) {
            return (false, Vec::new());
        }
        let mut changed = false;
        let mut index = 0;
        while index < self.builders.len() {
            let mut builder = std::mem::take(&mut self.builders[index]);
            let already = self.builders[..index]
                .iter()
                .filter(|b| b.lot == builder.lot && !b.returning)
                .count();
            if !builder.returning
                && self
                    .site(builder.lot)
                    .is_none_or(|site| already >= usize::from(site.crew))
            {
                builder.returning = true;
                builder.walk.reset_site(None);
            }
            let (updated, released) = self.step_builder(map, &mut builder, dt);
            changed |= updated;
            if released {
                self.builders.remove(index);
            } else {
                self.builders[index] = builder;
                index += 1;
            }
        }

        let mut completed = Vec::new();
        for lot in 0..self.lots.len() {
            let Some(site) = self.site(lot) else { continue };
            let mut crew = self.site_workers(lot);
            let wanted = usize::from(site.crew);
            let ready = site.ready();
            let working = self
                .builders
                .iter()
                .filter(|b| b.lot == lot && b.walk.work > 0.0)
                .count()
                + usize::from(nili == Some(lot));
            if ready && working > 0 {
                let site = self.construction[lot].as_mut().unwrap();
                site.work += dt * (1.0 + 0.55 * (working - 1) as f32);
                if site.work >= site.seconds() {
                    self.lots[lot] = Some(site.property);
                    self.construction[lot] = None;
                    self.evict(map, lot);
                    completed.push(lot);
                    changed = true;
                    continue;
                }
            }
            if !ready && self.site_supply(lot).is_none() {
                continue;
            }
            while crew < wanted && self.available_staff() > 0 {
                self.builders.push(Builder {
                    lot,
                    ..Default::default()
                });
                crew += 1;
                changed = true;
            }
        }
        (changed, completed)
    }

    fn step_builder(&mut self, map: &Terrain, builder: &mut Builder, dt: f32) -> (bool, bool) {
        let here = Vec2::from_array(builder.walk.position);
        let hall = here.distance(HALL_DOOR) <= 0.55;
        builder.walk.moving = false;
        if builder.returning {
            builder.walk.work = 0.0;
            if !hall {
                builder.walk.go(self, map, HALL_DOOR, 0.49, dt);
                return (false, false);
            }
            if let Some(mut cargo) = builder.walk.cargo {
                let stored = self.add(cargo.kind, cargo.units);
                cargo.units -= stored;
                builder.walk.cargo = (cargo.units > 0).then_some(cargo);
                return (stored > 0, cargo.units == 0);
            }
            return (true, true);
        }

        let lot = builder.lot;
        let plot = map.plots[lot];
        if let Some(mut load) = builder.walk.cargo {
            if here.distance(plot) > SITE_REACH {
                builder.walk.go(self, map, plot, SITE_REACH - 0.12, dt);
                return (false, false);
            }
            load.units -= self.construction[lot].as_mut().unwrap().accept(load);
            builder.walk.cargo = (load.units > 0).then_some(load);
            builder.returning = load.units > 0;
            builder.walk.reset_site(None);
            return (true, false);
        }
        if self.site(lot).is_some_and(BuildingSite::ready) {
            if here.distance(plot) <= SITE_REACH {
                builder.walk.face(plot);
                builder.walk.work += dt;
            } else {
                builder.walk.go(self, map, plot, SITE_REACH - 0.12, dt);
            }
        } else if hall {
            builder.walk.cargo = self.take_site_supply(lot, LOAD);
            builder.walk.reset_site(None);
            return (true, builder.walk.cargo.is_none());
        } else {
            builder.walk.go(self, map, HALL_DOOR, 0.49, dt);
        }
        (false, false)
    }
}

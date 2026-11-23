use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::construction::{Builder, BuildingSite};
use super::levels::{Level, Requirement};
use super::logistics::{Cargo, Courier, DroppedLoad, Haul};
use crate::{
    campaign::{CHAPTERS, REGION_SIZE, Region},
    model::Kind,
};

pub const HALL_STORAGE: u16 = 72;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Good {
    Wood,
    Stone,
    Clay,
    Ore,
    Food,
    Water,
    Boards,
    Bricks,
    Tools,
    Local,
    Meals,
    Craft,
    Cloth,
    Fiber,
    Fish,
    Oil,
    Flour,
    Paper,
    Perfume,
    Masterwork,
}

impl Good {
    pub const RAW: [Self; 6] = [
        Self::Wood,
        Self::Stone,
        Self::Clay,
        Self::Ore,
        Self::Food,
        Self::Local,
    ];
    pub const ALL: [Self; 20] = [
        Self::Wood,
        Self::Stone,
        Self::Clay,
        Self::Ore,
        Self::Food,
        Self::Water,
        Self::Boards,
        Self::Bricks,
        Self::Tools,
        Self::Local,
        Self::Meals,
        Self::Craft,
        Self::Cloth,
        Self::Fiber,
        Self::Fish,
        Self::Oil,
        Self::Flour,
        Self::Paper,
        Self::Perfume,
        Self::Masterwork,
    ];

    pub fn present_in(self, region: Region) -> bool {
        match self {
            Self::Oil => region == Region::Greece,
            Self::Flour => region == Region::Rome,
            Self::Paper => region == Region::Japan,
            Self::Fish => region == Region::Nordic,
            Self::Perfume => region == Region::Oasis,
            _ => true,
        }
    }

    pub fn uses_storage(self) -> bool {
        Self::RAW.contains(&self) || self == Self::Fish
    }

    pub fn work_seconds(self) -> f32 {
        match self {
            Self::Wood => 2.35,
            Self::Stone | Self::Ore => 2.8,
            Self::Clay => 2.1,
            _ => 1.65,
        }
    }

    pub fn gather_action(self) -> &'static str {
        match self {
            Self::Wood => "Fällen",
            Self::Stone => "Stein abbauen",
            Self::Ore => "Erz abbauen",
            Self::Clay => "Lehm stechen",
            _ => "Sammeln",
        }
    }

    pub fn district_name(self) -> &'static str {
        match self {
            Self::Wood => "Wald",
            Self::Stone => "Steinbruch",
            Self::Clay => "Lehmfeld",
            Self::Ore => "Mine",
            Self::Food => "Sammelgebiet",
            _ => "Regionale Fundstelle",
        }
    }

    pub fn district_color(self) -> [u8; 3] {
        match self {
            Self::Wood => [96, 130, 79],
            Self::Stone => [167, 163, 146],
            Self::Clay => [182, 129, 91],
            Self::Ore => [142, 126, 134],
            Self::Food => [168, 171, 106],
            _ => [142, 157, 99],
        }
    }

    pub fn name(self, region: Region) -> &'static str {
        match self {
            Self::Wood if region == Region::Japan => "Zedernholz",
            Self::Wood if region == Region::Nordic => "Fichtenholz",
            Self::Wood if region == Region::Oasis => "Palmenholz",
            Self::Wood => "Holz",
            Self::Stone if region == Region::Rome => "Travertin",
            Self::Stone if region == Region::Oasis => "Sandstein",
            Self::Stone => "Stein",
            Self::Clay => "Lehm",
            Self::Ore => match region {
                Region::Greece | Region::Oasis => "Kupfererz",
                Region::Rome => "Zinnerz",
                Region::Japan | Region::Nordic => "Eisenerz",
            },
            Self::Food => match region {
                Region::Greece => "Feldfrüchte",
                Region::Rome => "Getreide",
                Region::Japan => "Wildreis",
                Region::Nordic => "Wildbeeren",
                Region::Oasis => "Datteln",
            },
            Self::Water => "Wasser",
            Self::Boards => "Bretter",
            Self::Bricks => "Ziegel",
            Self::Tools => "Werkzeug",
            Self::Local => match region {
                Region::Greece => "Oliven",
                Region::Rome => "Kräuter",
                Region::Japan => "Bambus",
                Region::Nordic => "Bernstein",
                Region::Oasis => "Safran",
            },
            Self::Meals => "Mahlzeiten",
            Self::Craft => "Handelswaren",
            Self::Cloth => "Gewebe",
            Self::Fiber => "Fasern",
            Self::Fish => "Fisch",
            Self::Oil => "Olivenöl",
            Self::Flour => "Mehl",
            Self::Paper => "Papier",
            Self::Perfume => "Duftöl",
            Self::Masterwork => match region {
                Region::Greece => "Bemalte Amphoren",
                Region::Rome => "Reliefziegel",
                Region::Japan => "Washi-Laternen",
                Region::Nordic => "Bernsteinbänder",
                Region::Oasis => "Safranduftöl",
            },
        }
    }

    pub fn icon(self, region: Region) -> &'static str {
        match self {
            Self::Wood => "adventure_wood",
            Self::Stone => "adventure_stone",
            Self::Clay => "adventure_clay",
            Self::Ore => "adventure_ore",
            Self::Food => "adventure_food",
            Self::Water => "water",
            Self::Boards => "adventure_boards",
            Self::Bricks => "adventure_bricks",
            Self::Tools => "adventure_tools",
            Self::Local => match region {
                Region::Greece => "olive",
                Region::Rome => "herb_garden",
                Region::Japan => "tea_garden",
                Region::Nordic => "wear_amber_brooch",
                Region::Oasis => "spice_bazaar",
            },
            Self::Meals => "adventure_meals",
            Self::Craft => "adventure_craft",
            Self::Cloth => "adventure_cloth",
            Self::Fiber => "woodlot",
            Self::Fish => "fishery",
            Self::Oil => "press",
            Self::Flour => "mill",
            Self::Paper => "paper_workshop",
            Self::Perfume => "perfumer",
            Self::Masterwork => match region {
                Region::Greece => "potter",
                Region::Rome => "adventure_bricks",
                Region::Japan => "paper_workshop",
                Region::Nordic => "wear_amber_brooch",
                Region::Oasis => "perfumer",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Build {
    Harbor(Kind),
    Sawmill,
    Brickyard,
    Forge,
}

impl Build {
    pub fn fits(self, level: Level, lot: usize) -> bool {
        self != Self::Harbor(Kind::Fishery) || level.coastal(lot)
    }

    pub fn name(self, region: Region) -> &'static str {
        match self {
            Self::Harbor(kind) => kind.name(),
            Self::Sawmill => {
                if region == Region::Japan {
                    "Holzwerkstatt"
                } else {
                    "Sägewerk"
                }
            }
            Self::Brickyard => {
                if region == Region::Oasis {
                    "Lehmofen"
                } else {
                    "Ziegelei"
                }
            }
            Self::Forge => {
                if region == Region::Greece {
                    "Kupferschmiede"
                } else {
                    "Schmiede"
                }
            }
        }
    }

    pub fn all(level: Level) -> Vec<Self> {
        let mut plans = vec![
            Self::Harbor(level.home()),
            Self::Sawmill,
            Self::Harbor(level.well()),
            Self::Harbor(Kind::Storehouse),
            Self::Brickyard,
            Self::Forge,
        ];
        for chapter in &CHAPTERS[level.region.start()..level.region.start() + REGION_SIZE] {
            for &kind in chapter.allowed() {
                let plan = Self::Harbor(kind);
                if !plans.contains(&plan) {
                    plans.push(plan);
                }
            }
        }
        plans
    }

    pub fn page(self) -> u8 {
        match self {
            Self::Sawmill | Self::Brickyard | Self::Forge => 0,
            Self::Harbor(Kind::Storehouse) => 0,
            Self::Harbor(k)
                if k.water_source() || crate::model::Building::new(k).residents() > 0 =>
            {
                0
            }
            Self::Harbor(
                Kind::Temple
                | Kind::Shrine
                | Kind::Baths
                | Kind::Forum
                | Kind::GardenCourt
                | Kind::FeastHall
                | Kind::SignalTower
                | Kind::MoonGarden
                | Kind::Stoa
                | Kind::FerryOffice,
            ) => 2,
            Self::Harbor(_) => 1,
        }
    }

    pub fn cost(self, tier: u8) -> Vec<(Good, u16)> {
        use Good::*;
        if tier > 1 {
            let stage = u16::from(tier);
            let mut costs = vec![(Boards, stage * 3), (Stone, stage * 2)];
            if tier >= 3 {
                costs.push((Bricks, stage * 2));
            }
            if tier >= 4 {
                costs.push((Tools, stage - 1));
            }
            if tier >= 5 {
                costs.push((Local, stage - 1));
            }
            return costs;
        }
        match self {
            Self::Sawmill => vec![(Wood, 7), (Stone, 4)],
            Self::Brickyard => vec![(Wood, 5), (Boards, 4), (Stone, 5), (Clay, 4)],
            Self::Forge => vec![(Boards, 6), (Bricks, 5), (Stone, 5)],
            Self::Harbor(Kind::Storehouse) => vec![(Wood, 6), (Stone, 5)],
            Self::Harbor(kind) if crate::model::Building::new(kind).residents() > 0 => {
                vec![(Wood, 6), (Stone, 4)]
            }
            Self::Harbor(kind) if kind.water_source() && kind.cost() <= 20 => {
                vec![(Wood, 4), (Stone, 7)]
            }
            Self::Harbor(kind) => {
                let tier = kind.cost() / 14;
                let mut costs = vec![(Wood, 4 + tier as u16), (Stone, 4 + tier as u16)];
                if tier >= 2 {
                    costs.extend([(Boards, 4), (Bricks, 3)]);
                }
                if tier >= 3 {
                    costs.push((Tools, 2));
                }
                costs
            }
        }
    }

    pub fn recipe(self, tier: u8, region: Region) -> Option<Recipe> {
        use Good::*;
        use Kind::*;
        let t = u16::from(tier);
        let (need, output) = match self {
            Self::Sawmill if tier >= 3 => (vec![(Wood, 3), (Meals, 1)], (Boards, 5 + t)),
            Self::Sawmill => (vec![(Wood, 2), (Food, 1)], (Boards, 3 + t)),
            Self::Brickyard if tier >= 3 => (
                vec![(Clay, 3), (Wood, 1), (Water, 1), (Meals, 1)],
                (Bricks, 5 + t),
            ),
            Self::Brickyard => (vec![(Clay, 3), (Wood, 1), (Water, 1)], (Bricks, 3 + t)),
            Self::Forge if tier >= 3 => (vec![(Ore, 2), (Wood, 1), (Meals, 1)], (Tools, 3 + t)),
            Self::Forge => (vec![(Ore, 2), (Wood, 1), (Food, 1)], (Tools, 2 + t)),
            Self::Harbor(Qanat | Aqueduct) => (vec![], (Water, 6 + t * 3)),
            Self::Harbor(kind) if kind.water_source() => (vec![], (Water, 3 + t * 2)),
            Self::Harbor(Fishery) => (vec![(Food, 1)], (Fish, 3 + t)),
            Self::Harbor(Smokehouse) => (vec![(Fish, 2), (Wood, 1)], (Meals, 3 + t)),
            Self::Harbor(FeastHall) => (vec![(Fish, 2), (Water, 1)], (Meals, 4 + t)),
            Self::Harbor(Bakery) if region == Region::Rome => {
                (vec![(Flour, 2), (Water, 1)], (Meals, 3 + t))
            }
            Self::Harbor(Bakery | Taverna | RiceKitchen | Caravanserai) => {
                (vec![(Food, 2), (Water, 1)], (Meals, 3 + t))
            }
            Self::Harbor(TeaHouse) => (vec![(Local, 1), (Water, 2)], (Meals, 3 + t)),
            Self::Harbor(DateGarden | RicePaddy | MarketGarden) => {
                (vec![(Water, 1)], (Food, 3 + t))
            }
            Self::Harbor(Apiary) => (vec![(Local, 1), (Water, 1)], (Food, 3 + t)),
            Self::Harbor(Grove | HerbGarden | TeaGarden) => (vec![(Water, 1)], (Local, 3 + t)),
            Self::Harbor(Sheepfold) => (vec![(Food, 1), (Water, 1)], (Fiber, 3 + t)),
            Self::Harbor(Woodlot) => (vec![(Food, 1)], (Wood, 3 + t)),
            Self::Harbor(Mill) if region == Region::Rome => (vec![(Food, 2)], (Flour, 3 + t)),
            Self::Harbor(Mill) => (vec![(Food, 2)], (Meals, 2 + t)),
            Self::Harbor(Weaver) => (vec![(Fiber, 2), (Local, 1)], (Cloth, 3 + t)),
            Self::Harbor(SilkShop | CarpetHall | Dyer) => {
                (vec![(Local, 2), (Water, 1), (Food, 1)], (Cloth, 2 + t))
            }
            Self::Harbor(Press) => (vec![(Local, 2), (Water, 1)], (Oil, 3 + t)),
            Self::Harbor(Perfumer) => (vec![(Food, 1), (Local, 2), (Water, 1)], (Perfume, 3 + t)),
            Self::Harbor(SpiceBazaar) => (vec![(Food, 1), (Local, 2)], (Craft, 3 + t)),
            Self::Harbor(PaperWorkshop) => (vec![(Local, 2), (Water, 1)], (Paper, 3 + t)),
            Self::Harbor(Scribe) => (vec![(Local, 2), (Wood, 1)], (Craft, 3 + t)),
            Self::Harbor(Potter | CeramicStudio) => (vec![(Wood, 1), (Clay, 2)], (Craft, 3 + t)),
            Self::Harbor(Cooperage) => (vec![(Boards, 2), (Tools, 1)], (Craft, 3 + t)),
            Self::Harbor(Glassworks) => (vec![(Stone, 2), (Wood, 1), (Water, 1)], (Craft, 3 + t)),
            Self::Harbor(Boatyard) => (vec![(Boards, 2), (Food, 1)], (Craft, 3 + t)),
            Self::Harbor(_) => return None,
        };
        let base = match output.0 {
            Water => 4.0,
            Food | Local | Wood | Fiber => 7.0,
            Boards => 8.0,
            Meals => 10.0,
            Flour => 9.0,
            Oil | Paper => 13.0,
            Perfume => 20.0,
            Bricks => 14.0,
            Tools | Craft => 19.0,
            Cloth => 23.0,
            Fish => 18.0,
            _ => 8.0,
        };
        Some(Recipe {
            need,
            output,
            seconds: base + f32::from(tier.saturating_sub(1)) * 0.85,
        })
    }
}

pub struct Recipe {
    pub need: Vec<(Good, u16)>,
    pub output: (Good, u16),
    pub seconds: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Production {
    pub output: Cargo,
    pub duration: f32,
    pub left: f32,
    pub fishing: Option<usize>,
}

impl Production {
    pub fn progress(self) -> f32 {
        (1.0 - self.left / self.duration).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Task {
    Gather(usize),
    Produce(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub kind: Build,
    pub tier: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Deposit {
    pub kind: Good,
    pub at: Vec2,
    pub scenery: Option<Scenery>,
}

#[derive(Clone, Copy, Debug)]
pub struct Scenery {
    pub seed: u32,
    pub size: f32,
}

impl Deposit {
    pub fn new(kind: Good, at: Vec2) -> Self {
        Self {
            kind,
            at,
            scenery: None,
        }
    }

    pub fn units(self) -> u16 {
        if let Some(scenery) = self.scenery {
            let amount = match self.kind {
                Good::Wood => 7,
                _ => 5,
            };
            return amount + (scenery.seed % 4) as u16;
        }
        match self.kind {
            Good::Stone => 8,
            Good::Ore => 16,
            Good::Clay => 18,
            _ => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quest {
    #[default]
    Unheard,
    Searching,
    Carrying,
    Offering,
    Finished,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Island {
    pub index: usize,
    pub lots: Vec<Option<Property>>,
    #[serde(deserialize_with = "load_goods")]
    pub stores: [u16; Good::ALL.len()],
    #[serde(default, deserialize_with = "load_goods")]
    pub overflow: [u16; Good::ALL.len()],
    #[serde(default)]
    pub dropped: Vec<Option<DroppedLoad>>,
    pub workers: Vec<Task>,
    pub position: [f32; 2],
    pub phase: f32,
    pub quest: Quest,
    pub completed: bool,
    #[serde(default)]
    pub remaining: Vec<u16>,
    #[serde(default)]
    pub cargo: Option<Cargo>,
    #[serde(default)]
    pub hauls: Vec<Haul>,
    #[serde(default)]
    pub orders_delivered: u8,
    #[serde(default)]
    pub shipment: Option<u8>,
    #[serde(default)]
    pub courier: Option<Courier>,
    #[serde(default)]
    pub construction: Vec<Option<BuildingSite>>,
    #[serde(default)]
    pub builders: Vec<Builder>,
    #[serde(default)]
    pub building_load: Option<usize>,
    #[serde(default)]
    pub batches: Vec<Option<Production>>,
    #[serde(default)]
    pub fish_left: Vec<u16>,
    #[serde(default)]
    pub base_crafted: u16,
    #[serde(default)]
    masterwork_lots: Vec<usize>,
}

pub(super) fn load_goods<'de, D: serde::Deserializer<'de>>(
    input: D,
) -> Result<[u16; Good::ALL.len()], D::Error> {
    let saved = Vec::<u16>::deserialize(input)?;
    let mut stores = [0; Good::ALL.len()];
    for (slot, count) in stores.iter_mut().zip(saved) {
        *slot = count;
    }
    Ok(stores)
}

impl Island {
    pub fn new(level: Level) -> Self {
        Self {
            index: level.index,
            lots: vec![None; level.plots()],
            stores: [0; Good::ALL.len()],
            overflow: [0; Good::ALL.len()],
            dropped: Vec::new(),
            workers: Vec::new(),
            position: [0.0, 3.2],
            phase: 0.36,
            quest: Quest::Unheard,
            completed: false,
            remaining: Vec::new(),
            cargo: None,
            hauls: Vec::new(),
            orders_delivered: 0,
            shipment: None,
            courier: None,
            construction: vec![None; level.plots()],
            builders: Vec::new(),
            building_load: None,
            batches: vec![None; level.plots()],
            fish_left: Vec::new(),
            base_crafted: 0,
            masterwork_lots: Vec::new(),
        }
    }

    pub fn amount(&self, kind: Good) -> u16 {
        self.stores[kind as usize]
    }

    pub fn warehouse_capacity(tier: u8) -> u16 {
        36 + u16::from(tier.saturating_sub(1)) * 24
    }

    pub fn storage_capacity(&self) -> u16 {
        HALL_STORAGE
            + self
                .lots
                .iter()
                .flatten()
                .filter(|p| p.kind == Build::Harbor(Kind::Storehouse))
                .map(|p| Self::warehouse_capacity(p.tier))
                .sum::<u16>()
    }

    pub fn raw_stock(&self) -> u16 {
        Good::ALL
            .into_iter()
            .filter(|good| good.uses_storage())
            .map(|good| self.amount(good))
            .fold(0, u16::saturating_add)
    }

    pub fn free_storage(&self) -> u16 {
        self.storage_capacity().saturating_sub(self.raw_stock())
    }

    pub fn overflow_total(&self) -> u32 {
        self.overflow.iter().copied().map(u32::from).sum()
    }

    pub fn store_overflow(&mut self, good: Good) -> u16 {
        let units = self.add(good, self.overflow[good as usize]);
        self.overflow[good as usize] -= units;
        units
    }

    pub fn discard_stock(&mut self, good: Good) -> u16 {
        if !good.uses_storage() {
            return 0;
        }
        let units = self.amount(good).min(5);
        self.stores[good as usize] -= units;
        units
    }

    pub fn staff(&self) -> u16 {
        self.lots
            .iter()
            .flatten()
            .filter_map(|property| {
                let Build::Harbor(kind) = property.kind else {
                    return None;
                };
                let residents = crate::model::Building::new(kind).residents();
                (residents > 0)
                    .then_some(residents.saturating_sub(1) + u16::from(property.tier - 1))
            })
            .sum::<u16>()
            .min(28)
    }

    pub fn ready(&self, costs: &[(Good, u16)]) -> bool {
        costs
            .iter()
            .all(|&(good, units)| self.amount(good) >= units)
    }

    pub fn locked(&self, level: Level, kind: Build, tier: u8) -> bool {
        let chain = match kind {
            Build::Brickyard => 1,
            Build::Forge => 2,
            _ if kind.page() == 2 => 1,
            _ => 0,
        };
        level.stage < chain && !level.goals().any(|goal| goal.kind == kind)
            || (Some(kind) == level.plan()
                && self.quest != Quest::Finished
                && (level.stage < 9 || tier >= 3))
    }

    pub fn turn_in(&mut self, level: Level) -> bool {
        let Some(offering) = level.offering() else {
            return false;
        };
        if self.quest != Quest::Offering || !self.ready(&[offering]) {
            return false;
        }
        self.stores[offering.0 as usize] -= offering.1;
        self.quest = Quest::Finished;
        true
    }

    pub fn pack_order(&mut self, level: Level) -> bool {
        if self.cargo.is_some() {
            return false;
        }
        let Some(order) = self.reserve_order(level) else {
            return false;
        };
        self.shipment = Some(order);
        true
    }

    pub fn order_ready(&self, level: Level) -> bool {
        self.shipment.is_none()
            && self
                .courier
                .as_ref()
                .is_none_or(|trip| trip.order.is_none())
            && level
                .order(self.orders_delivered)
                .is_some_and(|order| self.ready(&order.goods))
    }

    pub(super) fn reserve_order(&mut self, level: Level) -> Option<u8> {
        if !self.order_ready(level) {
            return None;
        }
        let order = level.order(self.orders_delivered)?;
        for (kind, units) in order.goods {
            self.stores[kind as usize] -= units;
        }
        Some(self.orders_delivered)
    }

    pub fn return_order(&mut self, level: Level) -> bool {
        let Some(order) = self.shipment.and_then(|index| level.order(index)) else {
            return false;
        };
        let space: u16 = order
            .goods
            .into_iter()
            .filter(|(good, _)| good.uses_storage())
            .map(|(_, count)| count)
            .sum();
        if space > self.free_storage() {
            return false;
        }
        self.shipment = None;
        for (kind, units) in order.goods {
            self.add(kind, units);
        }
        true
    }

    pub fn deliver_order(&mut self, level: Level) -> bool {
        if self.shipment != Some(self.orders_delivered)
            || level.order(self.orders_delivered).is_none()
        {
            return false;
        }
        self.shipment = None;
        self.orders_delivered += 1;
        true
    }

    pub fn add(&mut self, good: Good, units: u16) -> u16 {
        let space = if good.uses_storage() {
            self.free_storage()
        } else {
            u16::MAX - self.amount(good)
        };
        let stored = units.min(space);
        self.stores[good as usize] += stored;
        stored
    }

    pub fn assignment(&self, task: Task) -> usize {
        self.workers.iter().filter(|&&job| job == task).count()
    }

    pub fn assigned_staff(&self) -> usize {
        self.workers.len() + usize::from(self.courier.is_some()) + self.builders.len()
    }

    pub fn available_staff(&self) -> usize {
        usize::from(self.staff()).saturating_sub(self.assigned_staff())
    }

    pub fn assign(&mut self, task: Task, layout: &super::terrain::Terrain) -> bool {
        let valid = match task {
            Task::Gather(node) => layout.deposits.get(node).is_some_and(|deposit| {
                self.available_near(layout, deposit.kind, deposit.at)
                    .is_some()
            }),
            Task::Produce(lot) => self
                .lots
                .get(lot)
                .copied()
                .flatten()
                .is_some_and(|p| p.kind.recipe(p.tier, layout.level.region).is_some()),
        };
        if !valid || self.available_staff() == 0 || self.assignment(task) >= 3 {
            return false;
        }
        self.hauls.resize_with(self.workers.len(), Haul::default);
        self.workers.push(task);
        self.hauls.push(Haul::default());
        true
    }

    pub fn unassign(&mut self, task: Task) -> bool {
        let Some(index) = self.workers.iter().rposition(|&job| job == task) else {
            return false;
        };
        self.remove_worker(index);
        true
    }

    pub fn validate_workers(&mut self, terrain: &super::Terrain, maximum: usize) {
        for i in (0..self.workers.len()).rev() {
            let valid = match self.workers[i] {
                Task::Gather(node) => terrain.deposits.get(node).is_some(),
                Task::Produce(lot) => self
                    .lots
                    .get(lot)
                    .copied()
                    .flatten()
                    .is_some_and(|p| p.kind.recipe(p.tier, terrain.level.region).is_some()),
            };
            if !valid {
                self.remove_worker(i);
            }
        }
        let available =
            maximum.saturating_sub(usize::from(self.courier.is_some()) + self.builders.len());
        while self.workers.len() > available {
            self.remove_worker(self.workers.len() - 1);
        }
        self.hauls.truncate(available);
    }

    fn remove_worker(&mut self, index: usize) {
        self.workers.remove(index);
        if index < self.hauls.len() {
            let haul = self.hauls.remove(index);
            if let Some(cargo) = haul.cargo {
                self.drop_load(DroppedLoad {
                    cargo,
                    position: haul.position,
                });
            }
        }
    }

    pub fn prepare_production(&mut self, map: &super::Terrain) {
        self.batches.resize(self.lots.len(), None);
        self.fish_left.truncate(map.fishing.len());
        self.fish_left
            .resize(map.fishing.len(), 140 + map.level.stage as u16 * 14);
    }

    pub fn batch(&self, lot: usize) -> Option<Production> {
        self.batches.get(lot).copied().flatten()
    }

    fn fish_ground(&self, map: &super::Terrain, lot: usize) -> Option<usize> {
        map.fishing
            .iter()
            .enumerate()
            .find(|(i, place)| {
                (place.lot == lot || !map.level.coastal(lot))
                    && self.fish_left.get(*i).is_some_and(|left| *left > 0)
            })
            .map(|(i, _)| i)
    }

    pub fn fish_available(&self, map: &super::Terrain, lot: usize) -> bool {
        self.fish_ground(map, lot).is_some()
    }

    pub fn makes_masterwork(&self, lot: usize) -> bool {
        self.masterwork_lots.contains(&lot)
    }

    pub fn masterwork_unlocked(&self, level: Level, lot: usize) -> bool {
        let Some(order) = level.work_order() else {
            return false;
        };
        self.base_crafted >= order.base_needed
            && self
                .lots
                .get(lot)
                .copied()
                .flatten()
                .is_some_and(|property| {
                    property.kind == order.workshop && property.tier >= order.tier
                })
    }

    pub fn set_masterwork(&mut self, level: Level, lot: usize, enabled: bool) -> bool {
        if enabled == self.makes_masterwork(lot)
            || (enabled && !self.masterwork_unlocked(level, lot))
        {
            return false;
        }
        if enabled {
            self.masterwork_lots.push(lot);
        } else {
            self.masterwork_lots.retain(|&saved| saved != lot);
        }
        true
    }

    pub fn recipe_for(&self, level: Level, lot: usize) -> Option<Recipe> {
        let property = self.lots.get(lot).copied().flatten()?;
        if self.makes_masterwork(lot)
            && self.masterwork_unlocked(level, lot)
            && let Some(order) = level.work_order()
        {
            return Some(Recipe {
                need: vec![
                    (order.base, 2),
                    (order.ingredient, u16::from(order.tier - 1)),
                ],
                output: (Good::Masterwork, 3),
                seconds: 18.0 + f32::from(order.tier) * 3.0,
            });
        }
        property.kind.recipe(property.tier, level.region)
    }

    pub fn start_production(&mut self, map: &super::Terrain, lot: usize) -> bool {
        let Some(mut recipe) = self.recipe_for(map.level, lot) else {
            return false;
        };
        if self.batch(lot).is_some() || !self.ready(&recipe.need) {
            return false;
        }
        self.prepare_production(map);
        let (kind, mut units) = recipe.output;
        let mut school = None;
        let mut seconds = recipe.seconds;
        if kind == Good::Fish {
            if self.assignment(Task::Produce(lot)) == 0 {
                return false;
            }
            school = self.fish_ground(map, lot);
            let Some(i) = school else {
                return false;
            };
            let place = map.fishing[i];
            units = units.min(self.fish_left[i]);
            seconds +=
                place.at.distance(place.launch) * 0.38 + map.plots[place.lot].length() * 0.16;
        }
        recipe.output.1 = units;
        if !self.room_for_recipe(&recipe) {
            return false;
        }
        if let Some(i) = school {
            self.fish_left[i] -= units;
        }
        for (good, units) in recipe.need {
            self.stores[good as usize] -= units;
        }
        let crew = self.assignment(Task::Produce(lot)).max(1);
        let duration = seconds / (1.0 + 0.48 * (crew - 1) as f32);
        self.batches[lot] = Some(Production {
            output: Cargo { kind, units },
            duration,
            left: duration,
            fishing: school,
        });
        true
    }

    pub fn room_for_recipe(&self, recipe: &Recipe) -> bool {
        if !recipe.output.0.uses_storage() {
            return true;
        }
        let used: u16 = recipe
            .need
            .iter()
            .filter(|(good, _)| good.uses_storage())
            .map(|(_, units)| *units)
            .sum();
        self.raw_stock()
            .saturating_sub(used)
            .saturating_add(recipe.output.1)
            <= self.storage_capacity()
    }

    pub fn tick_production(&mut self, map: &super::Terrain, dt: f32) -> bool {
        let mut changed = false;
        let order = map.level.work_order();
        for lot in 0..self.lots.len() {
            if let Some(mut batch) = self.batch(lot) {
                batch.left = (batch.left - dt).max(0.0);
                if batch.left == 0.0 {
                    let stored = self.add(batch.output.kind, batch.output.units);
                    batch.output.units -= stored;
                    if let Some(order) = order
                        && batch.output.kind == order.base
                        && self.lots[lot].is_some_and(|property| property.kind == order.workshop)
                    {
                        self.base_crafted = self
                            .base_crafted
                            .saturating_add(stored)
                            .min(order.base_needed);
                    }
                    changed |= stored > 0;
                }
                self.batches[lot] = (batch.output.units > 0).then_some(batch);
            } else if self.assignment(Task::Produce(lot)) > 0 {
                changed |= self.start_production(map, lot);
            }
        }
        changed
    }

    pub fn progress(&self, target: Requirement) -> u8 {
        self.lots
            .iter()
            .flatten()
            .filter(|p| p.kind == target.kind && p.tier >= target.tier)
            .count() as u8
    }

    pub fn victory(&self, level: Level) -> bool {
        let (stock, count) = level.reserve();
        level.goals().all(|g| self.progress(g) >= g.count)
            && self.amount(stock) >= count
            && (level.plan().is_none() || self.quest == Quest::Finished)
            && self.orders_delivered >= level.order_count()
    }
}

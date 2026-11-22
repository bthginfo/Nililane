use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::levels::{Level, Requirement};
use crate::{
    campaign::{CHAPTERS, REGION_SIZE, Region},
    model::Kind,
};

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
    pub const ALL: [Self; 14] = [
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
    ];

    pub fn harvest(self) -> u16 {
        if matches!(self, Self::Ore | Self::Local) {
            3
        } else {
            5
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
                Region::Nordic => "Fisch und Beeren",
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

    pub fn recipe(self, tier: u8) -> Option<Recipe> {
        use Good::*;
        use Kind::*;
        let t = u16::from(tier);
        let (need, output) = match self {
            Self::Sawmill => (vec![(Wood, 2), (Food, 1)], (Boards, 3 + t)),
            Self::Brickyard => (vec![(Clay, 3), (Wood, 1), (Water, 1)], (Bricks, 3 + t)),
            Self::Forge => (vec![(Ore, 2), (Wood, 1), (Food, 1)], (Tools, 2 + t)),
            Self::Harbor(kind) if kind.water_source() => (vec![], (Water, 3 + t * 2)),
            Self::Harbor(
                Bakery | Taverna | RiceKitchen | Smokehouse | FeastHall | Caravanserai,
            ) => (vec![(Food, 2), (Water, 1)], (Meals, 3 + t)),
            Self::Harbor(TeaHouse) => (vec![(Local, 1), (Water, 2)], (Meals, 3 + t)),
            Self::Harbor(Fishery | DateGarden | RicePaddy | MarketGarden) => {
                (vec![(Water, 1)], (Food, 3 + t))
            }
            Self::Harbor(Apiary) => (vec![(Local, 1), (Water, 1)], (Food, 3 + t)),
            Self::Harbor(Grove | HerbGarden | TeaGarden) => (vec![(Water, 1)], (Local, 3 + t)),
            Self::Harbor(Sheepfold) => (vec![(Food, 1), (Water, 1)], (Fiber, 3 + t)),
            Self::Harbor(Woodlot) => (vec![(Food, 1)], (Wood, 3 + t)),
            Self::Harbor(Mill) => (vec![(Food, 2)], (Meals, 2 + t)),
            Self::Harbor(Weaver) => (vec![(Fiber, 2), (Local, 1)], (Cloth, 3 + t)),
            Self::Harbor(SilkShop | CarpetHall | Dyer) => {
                (vec![(Local, 2), (Water, 1), (Food, 1)], (Cloth, 2 + t))
            }
            Self::Harbor(Press | Perfumer) => {
                (vec![(Food, 1), (Local, 2), (Water, 1)], (Craft, 3 + t))
            }
            Self::Harbor(Scribe | PaperWorkshop) => (vec![(Local, 2), (Wood, 1)], (Craft, 3 + t)),
            Self::Harbor(Potter | CeramicStudio) => (vec![(Wood, 1), (Clay, 2)], (Craft, 3 + t)),
            Self::Harbor(Cooperage) => (vec![(Boards, 2), (Tools, 1)], (Craft, 3 + t)),
            Self::Harbor(Glassworks) => (vec![(Stone, 2), (Wood, 1), (Water, 1)], (Craft, 3 + t)),
            Self::Harbor(Boatyard) => (vec![(Boards, 2), (Food, 1)], (Craft, 3 + t)),
            Self::Harbor(_) => return None,
        };
        Some(Recipe { need, output })
    }
}

pub struct Recipe {
    pub need: Vec<(Good, u16)>,
    pub output: (Good, u16),
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
    pub stores: [u16; Good::ALL.len()],
    pub workers: Vec<Task>,
    pub position: [f32; 2],
    pub phase: f32,
    pub quest: Quest,
    pub completed: bool,
}

impl Island {
    pub fn new(level: Level) -> Self {
        Self {
            index: level.index,
            lots: vec![None; level.plots()],
            stores: [0; Good::ALL.len()],
            workers: Vec::new(),
            position: [0.0, 3.2],
            phase: 0.36,
            quest: Quest::Unheard,
            completed: false,
        }
    }

    pub fn amount(&self, kind: Good) -> u16 {
        self.stores[kind as usize]
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

    pub fn build(&mut self, level: Level, lot: usize, kind: Build) -> bool {
        let Some(prev) = self.lots.get(lot).copied() else {
            return false;
        };
        if prev.is_some_and(|property| {
            property.kind != kind || property.tier >= crate::model::MAX_LEVEL
        }) {
            return false;
        }
        let tier = prev.map_or(1, |p| p.tier + 1);
        let costs = kind.cost(tier);
        if self.locked(level, kind, tier)
            || !Build::all(level).contains(&kind)
            || !self.ready(&costs)
        {
            return false;
        }
        for (good, units) in costs {
            self.stores[good as usize] -= units;
        }
        self.lots[lot] = Some(Property { kind, tier });
        true
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

    pub fn add(&mut self, good: Good, units: u16) {
        self.stores[good as usize] = self.amount(good).saturating_add(units);
    }

    pub fn gather(&mut self, good: Good) {
        self.add(good, good.harvest());
    }

    pub fn assignment(&self, task: Task) -> usize {
        self.workers.iter().filter(|&&job| job == task).count()
    }

    pub fn assign(&mut self, task: Task, layout: &super::terrain::Terrain) -> bool {
        let valid = match task {
            Task::Gather(node) => layout.deposits.get(node).is_some(),
            Task::Produce(lot) => self
                .lots
                .get(lot)
                .copied()
                .flatten()
                .is_some_and(|p| p.kind.recipe(p.tier).is_some()),
        };
        if !valid || self.workers.len() >= usize::from(self.staff()) || self.assignment(task) >= 3 {
            return false;
        }
        self.workers.push(task);
        true
    }

    pub fn unassign(&mut self, task: Task) -> bool {
        let Some(index) = self.workers.iter().rposition(|&job| job == task) else {
            return false;
        };
        self.workers.remove(index);
        true
    }

    pub fn produce(&mut self, lot: usize) -> bool {
        let Some(recipe) = self
            .lots
            .get(lot)
            .copied()
            .flatten()
            .and_then(|p| p.kind.recipe(p.tier))
        else {
            return false;
        };
        if !self.ready(&recipe.need) {
            return false;
        }
        for (good, units) in recipe.need {
            self.stores[good as usize] -= units;
        }
        self.add(recipe.output.0, recipe.output.1);
        true
    }

    pub fn tick_work(&mut self, terrain: &super::terrain::Terrain) {
        // Collect raw materials before factories try to claim that round's inputs.
        let workers = self.workers.clone();
        for &job in &workers {
            if let Task::Gather(node) = job
                && let Some(deposit) = terrain.deposits.get(node)
            {
                self.add(deposit.kind, 2);
            }
        }
        for job in workers {
            if let Task::Produce(lot) = job {
                self.produce(lot);
            }
        }
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
    }
}

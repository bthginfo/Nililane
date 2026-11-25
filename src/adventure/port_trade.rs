use serde::{Deserialize, Serialize};

use crate::campaign::Region;

use super::logistics::Cargo;
use super::twists::{FAIR_WIND, Twist};
use super::{Good, Island, Level};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Export {
    Regular,
    Artisan,
}

impl Export {
    pub const ALL: [Self; 2] = [Self::Regular, Self::Artisan];
}

#[derive(Clone, Copy)]
pub struct Offer {
    pub good: Good,
    pub units: u16,
    pub marks: u16,
    pub seconds: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Crossing {
    pub export: Export,
    pub remaining: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Market {
    pub marks: u16,
    pub crossings: u16,
    pub packed: Option<Export>,
    pub crossing: Option<Crossing>,
    pub blueprint: bool,
}

pub const PLAN_PRICE: u16 = 5;
pub const IMPORT_PRICE: u16 = 3;
pub const IMPORT_UNITS: u16 = 4;

impl Level {
    pub fn has_trading(self) -> bool {
        self.stage >= 7
    }

    pub fn trading_neighbor(self) -> &'static str {
        const NEIGHBORS: [[&str; 5]; 5] = [
            ["Paros", "Naxos", "Aigina", "Delos", "Milos"],
            ["Ostia", "Puteoli", "Capua", "Antium", "Luna"],
            ["Onomichi", "Miyajima", "Etajima", "Awajishima", "Tomonoura"],
            ["Borg", "Ribe", "Hitra", "Hedeby", "Balestrand"],
            [
                "Qasr Safra",
                "Ain Nura",
                "Khusran",
                "Ribat Sada",
                "Hisn Qamar",
            ],
        ];
        NEIGHBORS[self.region as usize][self.stage.saturating_sub(7).min(4)]
    }

    pub fn export(self, quality: Export) -> Option<Offer> {
        use Good::*;
        if !self.has_trading() {
            return None;
        }
        let (regular, fine) = match self.region {
            Region::Greece => (Oil, Cloth),
            Region::Rome => (Bricks, Tools),
            Region::Japan => (Paper, Cloth),
            Region::Nordic => (Cloth, Craft),
            Region::Oasis => (Perfume, Craft),
        };
        let wind = if self.has_twist(Twist::FairWind) {
            FAIR_WIND
        } else {
            1.0
        };
        Some(match quality {
            Export::Regular => Offer {
                good: regular,
                units: 6,
                marks: 4,
                seconds: 21.0 * wind,
            },
            Export::Artisan => Offer {
                good: if self.work_order().is_some() {
                    Masterwork
                } else {
                    fine
                },
                units: 4,
                marks: 6,
                seconds: 27.0 * wind,
            },
        })
    }
}

pub fn monument_name(region: Region) -> &'static str {
    match region {
        Region::Greece => "Agora",
        Region::Rome => "Amphitheater",
        Region::Japan => "Pagodengarten",
        Region::Nordic => "Thinghalle",
        Region::Oasis => "Palast der Sterne",
    }
}

impl Island {
    pub fn pack_export(&mut self, level: Level, option: Export) -> bool {
        let Some(offer) = level.export(option) else {
            return false;
        };
        if self.has_cargo() || self.amount(offer.good) < offer.units {
            return false;
        }
        self.stores[offer.good as usize] -= offer.units;
        self.market.packed = Some(option);
        true
    }

    pub fn unpack_export(&mut self, level: Level) -> bool {
        let Some(offer) = self.market.packed.and_then(|packed| level.export(packed)) else {
            return false;
        };
        if self.amount(offer.good) > u16::MAX - offer.units {
            return false;
        }
        self.add(offer.good, offer.units);
        self.market.packed = None;
        true
    }

    pub fn launch_export(&mut self, level: Level) -> bool {
        if self.market.crossing.is_some() {
            return false;
        }
        let Some(export) = self.market.packed else {
            return false;
        };
        let Some(offer) = level.export(export) else {
            return false;
        };
        self.market.crossing = Some(Crossing {
            export,
            remaining: offer.seconds,
        });
        self.market.packed = None;
        true
    }

    pub fn tick_export(&mut self, level: Level, dt: f32) -> Option<u16> {
        let crossing = self.market.crossing.as_mut()?;
        crossing.remaining = (crossing.remaining - dt).max(0.0);
        if crossing.remaining > 0.0 {
            return None;
        }
        let export = crossing.export;
        self.market.crossing = None;
        let offer = level.export(export)?;
        self.market.crossings = self.market.crossings.saturating_add(1);
        self.market.marks = self.market.marks.saturating_add(offer.marks);
        Some(offer.marks)
    }

    pub fn buy_monument(&mut self, level: Level) -> bool {
        if !level.has_trading() || self.market.blueprint || self.market.marks < PLAN_PRICE {
            return false;
        }
        self.market.marks -= PLAN_PRICE;
        self.market.blueprint = true;
        true
    }

    pub fn buy_imports(&mut self, level: Level) -> bool {
        if !level.has_trading()
            || self.market.marks < IMPORT_PRICE
            || self.room_for(Good::Imports) < IMPORT_UNITS
        {
            return false;
        }
        self.market.marks -= IMPORT_PRICE;
        self.cargo = Some(Cargo {
            kind: Good::Imports,
            units: IMPORT_UNITS + self.cargo.map_or(0, |load| load.units),
        });
        true
    }
}

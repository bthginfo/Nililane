use crate::{
    campaign::{CORE_HARBORS, Chapter, Region, harbor_stage},
    layout::distance,
    model::{Kind, Phase, Town, Weather},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contract {
    Oasis,
    SpiceRoad,
}

impl Contract {
    pub const ALL: [Self; 2] = [Self::Oasis, Self::SpiceRoad];

    pub fn name(self) -> &'static str {
        match self {
            Self::Oasis => "Zur Nachbaroase",
            Self::SpiceRoad => "Über die Gewürzstraße",
        }
    }

    pub fn available(self, index: usize) -> bool {
        Region::of(index) == Region::Oasis
            && harbor_stage(index)
                >= match self {
                    Self::Oasis => 4,
                    Self::SpiceRoad => CORE_HARBORS,
                }
    }

    // Datteln, Gewürze und Krüge aus dem Wasserüberschuss.
    pub fn supplies(self) -> (u16, u16, u16) {
        match self {
            Self::Oasis => (2, 0, 3),
            Self::SpiceRoad => (3, 3, 5),
        }
    }

    pub fn pay(self, chapter: &Chapter, level: u8) -> u32 {
        match self {
            Self::Oasis => 16 + u32::from(level) * 2,
            Self::SpiceRoad => 35 + u32::from(level) * 3 + u32::from(chapter.shift.night()) * 6,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaravanBlocked {
    Plan,
    Closed,
    Sent,
    Storm,
    Qanat,
    Water,
    Dates,
    Spices,
}

impl CaravanBlocked {
    pub fn label(self) -> &'static str {
        match self {
            Self::Plan => "Karawanen ziehen während der Öffnungszeit los.",
            Self::Closed => "Der nächste Aufbruch ist morgen.",
            Self::Sent => "Heute ist schon eine Karawane aufgebrochen.",
            Self::Storm => "Im Sturm wartet die Karawane in der Oase.",
            Self::Qanat => "Ein Qanat ab Stufe 2 muss höchstens zwei Bauplätze entfernt sein.",
            Self::Water => "Es fehlt Wasser in der Reserve. Decke erst den Bedarf der Betriebe.",
            Self::Dates => {
                "Halte Datteln in einem Palmenhain bis zu drei Bauplätze entfernt bereit."
            }
            Self::Spices => "Am Gewürzstand bis zu drei Bauplätze entfernt fehlen Gewürze.",
        }
    }
}

impl Town {
    pub fn caravan_supply(
        &self,
        chapter: &Chapter,
        lot: usize,
        contract: Contract,
    ) -> Result<(usize, Option<usize>), CaravanBlocked> {
        if !contract.available(self.chapter)
            || !chapter.layout.contains(lot)
            || !self.lots[lot].is_some_and(|b| b.kind == Kind::Caravanserai)
        {
            return Err(CaravanBlocked::Closed);
        }
        let Phase::Open(day) = &self.phase else {
            return Err(CaravanBlocked::Plan);
        };
        if day.seconds + 0.5 >= chapter.day_seconds() {
            return Err(CaravanBlocked::Closed);
        }
        if day.tally.caravans > 0 {
            return Err(CaravanBlocked::Sent);
        }
        if self.sky(chapter) == Weather::Storm {
            return Err(CaravanBlocked::Storm);
        }
        if contract == Contract::SpiceRoad && self.support(lot, Kind::Qanat, 2) < 2 {
            return Err(CaravanBlocked::Qanat);
        }
        let (dates, spices, water) = contract.supplies();
        if day.caravan_water < water {
            return Err(CaravanBlocked::Water);
        }
        let source = |kind, amount| {
            chapter
                .layout
                .lots()
                .filter(|&i| {
                    distance(i, lot) <= 3
                        && self.lots[i].is_some_and(|building| building.kind == kind)
                        && day.stock[i] >= amount
                })
                .max_by_key(|&i| (day.stock[i], std::cmp::Reverse(distance(i, lot))))
        };
        let garden = source(Kind::DateGarden, dates).ok_or(CaravanBlocked::Dates)?;
        let stand = if spices > 0 {
            Some(source(Kind::SpiceBazaar, spices).ok_or(CaravanBlocked::Spices)?)
        } else {
            None
        };
        Ok((garden, stand))
    }

    pub fn send_caravan(
        &mut self,
        chapter: &Chapter,
        lot: usize,
        contract: Contract,
    ) -> Result<u32, CaravanBlocked> {
        let (garden, stand) = self.caravan_supply(chapter, lot, contract)?;
        let pay = contract.pay(chapter, self.lots[lot].unwrap().level);
        let Phase::Open(day) = &mut self.phase else {
            return Err(CaravanBlocked::Plan);
        };
        let (dates, spices, water) = contract.supplies();
        day.stock[garden] -= dates;
        if let Some(stand) = stand {
            day.stock[stand] -= spices;
        }
        day.caravan_water -= water;
        day.tally.caravans += 1;
        day.tally.caravan_income += pay;
        if contract == Contract::SpiceRoad {
            day.tally.spice_routes += 1;
        }
        self.coins += pay;
        Ok(pay)
    }
}

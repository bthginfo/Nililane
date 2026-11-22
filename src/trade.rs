use serde::{Deserialize, Serialize};

use crate::{
    campaign::Chapter,
    model::{Kind, Phase, Town, Weather, distance},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SailingRoute {
    Coastal,
    Overseas,
}

impl SailingRoute {
    pub const ALL: [Self; 2] = [Self::Coastal, Self::Overseas];

    pub fn name(self) -> &'static str {
        match self {
            Self::Coastal => "Inselhandel",
            Self::Overseas => "Fernhandel",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Coastal => "boatyard",
            Self::Overseas => "sea_trade",
        }
    }

    pub fn provisions(self) -> u16 {
        match self {
            Self::Coastal => 3,
            Self::Overseas => 6,
        }
    }

    pub fn days(self) -> u32 {
        match self {
            Self::Coastal => 1,
            Self::Overseas => 2,
        }
    }

    pub fn required_level(self) -> u8 {
        match self {
            Self::Coastal => 1,
            Self::Overseas => 2,
        }
    }

    pub fn silver(self, level: u8) -> u32 {
        let bonus = u32::from(level.saturating_sub(self.required_level()));
        match self {
            Self::Coastal => 16 + bonus * 2,
            Self::Overseas => 42 + bonus * 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Voyage {
    pub route: SailingRoute,
    pub departed_day: u32,
    pub departed_at: f32,
    pub arrival: u32,
    pub silver: u32,
}

impl Voyage {
    pub fn valid(self, town: &Town, chapter: &Chapter) -> bool {
        chapter.region == crate::campaign::Region::Nordic
            && self.departed_day <= town.day
            && (0.0..chapter.day_seconds()).contains(&self.departed_at)
            && self.arrival >= town.day
            && (16..=58).contains(&self.silver)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Departure {
    Daylight,
    Boatyard,
    Upgrade,
    AtSea,
    Weather,
    Busy,
    Provisions,
}

impl Departure {
    pub fn hint(self) -> &'static str {
        match self {
            Self::Daylight => "Geht tagsüber bis kurz vor Sonnenuntergang",
            Self::Boatyard => "Hier braucht es ein Bootshaus",
            Self::Upgrade => "Ab Bootshaus Stufe 2",
            Self::AtSea => "Ein Schiff ist noch unterwegs",
            Self::Weather => "Bei Sturm bleibt das Schiff am Kai",
            Self::Busy => "Beende erst Nilis laufenden Auftrag",
            Self::Provisions => "Mehr Räucherfisch im Umkreis von drei Plätzen nötig",
        }
    }
}

impl Town {
    pub fn trade_source(
        &self,
        chapter: &Chapter,
        lot: usize,
        route: SailingRoute,
    ) -> Result<usize, Departure> {
        let yard = self
            .lots
            .get(lot)
            .copied()
            .flatten()
            .filter(|b| b.kind == Kind::Boatyard && chapter.layout.contains(lot))
            .ok_or(Departure::Boatyard)?;
        if yard.level < route.required_level() {
            return Err(Departure::Upgrade);
        }
        if self.voyage.is_some() {
            return Err(Departure::AtSea);
        }
        if self.weather(chapter) == Weather::Storm {
            return Err(Departure::Weather);
        }
        let Phase::Open(day) = &self.phase else {
            return Err(Departure::Daylight);
        };
        if day.seconds + 6.0 >= chapter.day_seconds() {
            return Err(Departure::Daylight);
        }
        if day.route.is_some() {
            return Err(Departure::Busy);
        }
        chapter
            .layout
            .lots()
            .filter(|&source| {
                distance(lot, source) <= 3
                    && self.lots[source].is_some_and(|b| b.kind == Kind::Smokehouse)
                    && day.stock[source] >= route.provisions()
            })
            .max_by_key(|&source| (day.stock[source], std::cmp::Reverse(distance(lot, source))))
            .ok_or(Departure::Provisions)
    }

    pub fn send_voyage(
        &mut self,
        chapter: &Chapter,
        lot: usize,
        route: SailingRoute,
    ) -> Result<Voyage, Departure> {
        let source = self.trade_source(chapter, lot, route)?;
        let level = self.lots[lot]
            .expect("a departure requires a boatyard")
            .level;
        let Phase::Open(day) = &mut self.phase else {
            return Err(Departure::Daylight);
        };
        day.stock[source] -= route.provisions();
        let voyage = Voyage {
            route,
            departed_day: self.day,
            departed_at: day.seconds,
            arrival: self.day + route.days(),
            silver: route.silver(level),
        };
        self.voyage = Some(voyage);
        Ok(voyage)
    }

    pub(super) fn collect_voyage(&mut self, chapter: &Chapter) {
        self.cargo_today = 0;
        let Some(mut voyage) = self.voyage.filter(|voyage| self.day >= voyage.arrival) else {
            return;
        };
        if self.weather(chapter) == Weather::Storm {
            voyage.arrival = self.day + 1;
            self.voyage = Some(voyage);
            return;
        }
        self.coins += voyage.silver;
        self.cargo_today = voyage.silver;
        self.totals.voyages += 1;
        self.totals.cargo_income += voyage.silver;
        self.voyage = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        campaign::{CHAPTERS, NORDIC_START},
        save::Campaign,
    };

    #[test]
    fn fish_is_reserved_at_departure_and_a_storm_delays_the_saved_return() {
        let index = NORDIC_START + 21;
        let chapter = &CHAPTERS[index];
        let mut town = Town::new(index, chapter);
        let yard = 9;
        let smoker = 7;
        assert!(town.upgrade(yard));
        assert!(town.open(chapter));
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let stock = day.stock[smoker];
        let trip = town
            .send_voyage(chapter, yard, SailingRoute::Overseas)
            .unwrap();
        assert_eq!(trip.arrival, 3);
        assert_eq!(trip.silver, 42);
        assert_eq!(
            town.send_voyage(chapter, yard, SailingRoute::Coastal)
                .unwrap_err(),
            Departure::AtSea
        );
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        assert_eq!(day.stock[smoker], stock - 6);
        let mut campaign = Campaign::default();
        campaign.enter_town(town, false);
        let bytes = serde_json::to_vec(&campaign).unwrap();
        let mut town = Campaign::decode(&bytes[..]).unwrap().town.unwrap();

        for next_day in 2..=4 {
            assert!(town.tick(chapter, chapter.day_seconds()));
            let before = town.coins;
            assert!(town.advance());
            assert_eq!(town.day, next_day);
            if next_day == 4 {
                assert!(town.voyage.is_none());
                assert_eq!(town.coins, before + trip.silver);
                assert_eq!(town.cargo_today, trip.silver);
                assert_eq!(town.totals.cargo_income, trip.silver);
                assert_eq!(town.totals.voyages, 1);
            } else {
                assert_eq!(town.coins, before);
                assert_eq!(town.totals.voyages, 0);
                if next_day == 3 {
                    assert_eq!(town.weather(chapter), Weather::Storm);
                    assert_eq!(town.voyage.unwrap().arrival, 4);
                }
            }
            assert!(town.open(chapter));
        }
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert!(town.advance());
        assert_eq!(town.totals.voyages, 1);
        assert_eq!(town.cargo_today, 0);
    }
}

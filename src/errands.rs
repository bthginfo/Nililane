use serde::{Deserialize, Serialize};

use crate::{
    campaign::Chapter,
    layout::distance,
    model::{Kind, Phase, Town, Weather},
};

pub const JOB_TYPES: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Job {
    Freight,
    Water,
    Welcome,
    Shade,
    Quay,
}

impl Job {
    pub const ALL: [Self; JOB_TYPES] = [
        Self::Freight,
        Self::Water,
        Self::Welcome,
        Self::Shade,
        Self::Quay,
    ];

    pub fn unlocked(self, chapter: usize) -> bool {
        chapter >= [1, 1, 2, 3, 6][self as usize]
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Freight => "Nachschub bringen",
            Self::Water => "Wasser bringen",
            Self::Welcome => "Gäste begleiten",
            Self::Shade => "Schatten schaffen",
            Self::Quay => "Kai räumen",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Freight => "storehouse",
            Self::Water => "water",
            Self::Welcome => "ferry",
            Self::Shade => "grove",
            Self::Quay => "wind",
        }
    }

    pub fn purpose(self) -> &'static str {
        match self {
            Self::Freight => "+4 Vorrat · +6 vom Speicher",
            Self::Water => "+4 Vorrat · +2 Krüge",
            Self::Welcome => "+2 Gäste mit normalem Budget",
            Self::Shade => "+5 Rast- oder Badeplätze",
            Self::Quay => "+2 Plätze Laufweite vom Kai",
        }
    }

    pub fn requirements(self) -> &'static str {
        match self {
            Self::Freight => "Du brauchst einen Laden.",
            Self::Water => "Du brauchst eine Quelle und einen Laden mit Wasserbedarf.",
            Self::Welcome => "Du brauchst einen Laden als Ziel.",
            Self::Shade => "Du brauchst einen Hain und eine Stoa oder Therme.",
            Self::Quay => "Geht bei Wind, Regen oder Sturm.",
        }
    }

    pub fn delivered(self) -> &'static str {
        match self {
            Self::Freight => "Der Nachschub ist da.",
            Self::Water => "Nili hat frisches Wasser gebracht.",
            Self::Welcome => "Die Fährgäste sind auf dem Weg zum Laden.",
            Self::Shade => "Die neuen Schattenplätze sind fertig.",
            Self::Quay => "Der Kai ist frei. Fährgäste kommen weiter.",
        }
    }

    pub fn limit(chapter: usize) -> u8 {
        if chapter < 6 { 2 } else { 3 }
    }

    pub fn offers(self, town: &Town, chapter: &Chapter, lot: usize) -> bool {
        self.route(town, chapter, lot).is_some()
    }

    pub fn targets(self, town: &Town, chapter: &Chapter) -> Vec<usize> {
        chapter
            .layout
            .lots()
            .filter(|&lot| self.offers(town, chapter, lot))
            .collect()
    }

    fn route(self, town: &Town, chapter: &Chapter, lot: usize) -> Option<(Stop, Stop)> {
        if !self.unlocked(town.chapter) || !chapter.layout.contains(lot) {
            return None;
        }
        let nearest = |predicate: fn(Kind) -> bool| {
            chapter
                .layout
                .lots()
                .filter(|&i| town.lots[i].is_some_and(|b| predicate(b.kind)))
                .min_by_key(|&i| distance(i, lot))
                .map(|i| Stop::Parcel(i as u8))
        };
        let target = Stop::Parcel(lot as u8);
        if self == Self::Quay {
            return (lot == chapter.layout.dock()
                && matches!(
                    town.weather(chapter),
                    Weather::Wind | Weather::Rain | Weather::Storm
                ))
            .then_some((Stop::Pier, Stop::Quay));
        }
        let kind = town.lots[lot]?.kind;
        match self {
            Self::Freight if kind.service().is_some() => Some((
                nearest(|k| k == Kind::Storehouse).unwrap_or(Stop::Quay),
                target,
            )),
            Self::Water if kind.water_user() => Some((nearest(Kind::water_source)?, target)),
            Self::Welcome if kind.service().is_some() => Some((Stop::Quay, target)),
            Self::Shade if matches!(kind, Kind::Stoa | Kind::Baths) => {
                Some((nearest(|k| k == Kind::Grove)?, target))
            }
            _ => None,
        }
    }

    pub fn plan(self, town: &Town, chapter: &Chapter, lot: usize) -> Result<Route, Blocked> {
        let Phase::Open(day) = &town.phase else {
            return Err(Blocked::Morning);
        };
        if day.route.is_some() {
            return Err(Blocked::Busy);
        }
        if day.tally.routes[self as usize] != 0 {
            return Err(Blocked::Done);
        }
        if day.tally.routes_today() >= u32::from(Self::limit(town.chapter)) {
            return Err(Blocked::Limit);
        }
        let (pickup, delivery) = self
            .route(town, chapter, lot)
            .ok_or(Blocked::Requirements)?;
        let from = day.nili_at;
        let started = day.seconds;
        let pickup_at = started + from.duration(pickup, chapter);
        let depart_at = pickup_at + 0.9;
        let arrival = depart_at + pickup.duration(delivery, chapter);
        let finish = arrival + 0.65;
        let margin = if self == Self::Welcome { 2.1 } else { 0.1 };
        if finish + margin > chapter.day_seconds() {
            return Err(Blocked::Late);
        }
        Ok(Route {
            job: self,
            from,
            pickup,
            delivery,
            started,
            pickup_at,
            depart_at,
            arrival,
            finish,
        })
    }
}

#[derive(Clone, Copy)]
pub enum Blocked {
    Morning,
    Busy,
    Done,
    Limit,
    Requirements,
    Late,
}

impl Blocked {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Morning => "sun",
            Self::Busy | Self::Late => "guest_wait",
            Self::Done => "check",
            Self::Limit | Self::Requirements => "lock",
        }
    }

    pub fn label(self, job: Job) -> &'static str {
        match self {
            Self::Morning => "Starte zuerst den Tag.",
            Self::Busy => "Nili ist noch unterwegs.",
            Self::Done => "Für heute erledigt.",
            Self::Limit => "Heute sind alle Touren vergeben.",
            Self::Requirements => job.requirements(),
            Self::Late => "Dafür ist es heute zu spät.",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stop {
    #[default]
    Quay,
    Pier,
    Parcel(u8),
}

impl Stop {
    fn lot(self, chapter: &Chapter) -> usize {
        match self {
            Self::Quay | Self::Pier => chapter.layout.dock(),
            Self::Parcel(lot) => usize::from(lot),
        }
    }

    pub fn label(self, town: &Town) -> &'static str {
        match self {
            Self::Quay => "Kai",
            Self::Pier => "Anleger",
            Self::Parcel(i) => town.lots[usize::from(i)].map_or("Bauplatz", |b| b.kind.name()),
        }
    }

    pub fn valid(self, town: &Town, chapter: &Chapter) -> bool {
        match self {
            Self::Quay | Self::Pier => true,
            Self::Parcel(lot) => {
                chapter.layout.contains(usize::from(lot)) && town.lots[usize::from(lot)].is_some()
            }
        }
    }

    fn duration(self, to: Self, chapter: &Chapter) -> f32 {
        if self == to {
            return 0.2;
        }
        let quay_walk = f32::from(matches!(self, Self::Quay | Self::Pier))
            + f32::from(matches!(to, Self::Quay | Self::Pier));
        let blocks = distance(self.lot(chapter), to.lot(chapter)) as f32;
        1.3 + blocks * 0.68 + quay_walk * 0.65
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Route {
    pub job: Job,
    pub from: Stop,
    pub pickup: Stop,
    pub delivery: Stop,
    pub started: f32,
    pub pickup_at: f32,
    pub depart_at: f32,
    pub arrival: f32,
    pub finish: f32,
}

impl Route {
    pub fn valid(self, town: &Town, chapter: &Chapter, age: f32) -> bool {
        [self.from, self.pickup, self.delivery]
            .iter()
            .all(|s| s.valid(town, chapter))
            && self.started >= 0.0
            && self.started <= age
            && age < self.finish
            && self.started < self.pickup_at
            && self.pickup_at < self.depart_at
            && self.depart_at < self.arrival
            && self.arrival < self.finish
            && self.finish <= chapter.day_seconds()
            && (matches!(self.delivery, Stop::Parcel(_)) || self.job == Job::Quay)
    }

    pub fn progress(self, seconds: f32) -> f32 {
        ((seconds - self.started) / (self.finish - self.started)).clamp(0.0, 1.0)
    }
}

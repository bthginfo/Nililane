use serde::{Deserialize, Serialize};

use crate::{
    campaign::{Chapter, REGION_SIZE, harbor_stage},
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
        let stage = harbor_stage(chapter);
        if chapter < REGION_SIZE {
            stage >= [1, 1, 2, 3, 6][self as usize]
        } else {
            stage >= [0, 0, 0, 3, 5][self as usize]
        }
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
            Self::Freight => "4 Portionen Nachschub, aus dem Speicher 6",
            Self::Water => "4 Portionen mehr und 2 Krüge Wasser",
            Self::Welcome => "2 zusätzliche Fährgäste",
            Self::Shade => "5 zusätzliche Rast- oder Badeplätze",
            Self::Quay => "Fährgäste gehen zwei Plätze weiter",
        }
    }

    pub fn requirements(self) -> &'static str {
        match self {
            Self::Freight => "Du brauchst einen Laden.",
            Self::Water => "Dafür brauchst du eine Quelle und einen Laden, der Wasser braucht.",
            Self::Welcome => "Du brauchst einen Laden als Ziel.",
            Self::Shade => "Du brauchst einen Hain und eine Stoa oder Therme.",
            Self::Quay => "Nur bei Wind, Regen oder Sturm.",
        }
    }

    pub fn delivered(self) -> &'static str {
        match self {
            Self::Freight => "Der Nachschub ist da.",
            Self::Water => "Nili hat frisches Wasser gebracht.",
            Self::Welcome => "Die Fährgäste sind am Ziel.",
            Self::Shade => "Die neuen Schattenplätze sind fertig.",
            Self::Quay => "Der Kai ist frei. Fährgäste kommen weiter.",
        }
    }

    pub fn collect_action(self) -> &'static str {
        match self {
            Self::Freight => "Kiste aufnehmen",
            Self::Water => "Krüge füllen",
            Self::Welcome => "Gäste begrüßen",
            Self::Shade => "Zweige sammeln",
            Self::Quay => "Treibgut greifen",
        }
    }

    pub fn deliver_action(self) -> &'static str {
        match self {
            Self::Freight => "Kiste abgeben",
            Self::Water => "Wasser abgeben",
            Self::Welcome => "Gäste verabschieden",
            Self::Shade => "Zweige verteilen",
            Self::Quay => "Treibgut ablegen",
        }
    }

    pub fn limit(chapter: usize) -> u8 {
        if chapter < 6 {
            3
        } else if chapter < 12 {
            4
        } else {
            5
        }
    }

    pub fn daily_max(self, chapter: usize) -> u32 {
        match self {
            Self::Welcome => {
                if chapter < 12 {
                    2
                } else {
                    3
                }
            }
            Self::Freight | Self::Water if chapter >= 6 => 2,
            _ => 1,
        }
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
        if day.tally.routes[self as usize] >= self.daily_max(town.chapter) {
            return Err(Blocked::Done);
        }
        if day.tally.routes_today() >= u32::from(Self::limit(town.chapter)) {
            return Err(Blocked::Limit);
        }
        let (pickup, delivery) = self
            .route(town, chapter, lot)
            .ok_or(Blocked::Requirements)?;
        Route::new(
            self,
            false,
            day.nili_at,
            pickup,
            delivery,
            day.seconds,
            chapter,
        )
        .ok_or(Blocked::Late)
    }
}

#[derive(Clone, Copy, Debug)]
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
            Self::Limit => "Nili hat seine Touren für heute erledigt.",
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteStage {
    Approach,
    Collect,
    Carry,
    Deliver,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ManualStage {
    Pickup,
    Collect(f32),
    Delivery,
    Unload(f32),
}

impl ManualStage {
    pub fn pickup(self) -> bool {
        matches!(self, Self::Pickup | Self::Collect(_))
    }

    pub fn action(self, job: Job) -> Option<&'static str> {
        match self {
            Self::Pickup => Some(job.collect_action()),
            Self::Delivery => Some(job.deliver_action()),
            _ => None,
        }
    }

    pub fn animation(self) -> (RouteStage, f32) {
        match self {
            Self::Pickup => (RouteStage::Approach, 0.0),
            Self::Collect(progress) => (RouteStage::Collect, progress),
            Self::Delivery => (RouteStage::Carry, 0.0),
            Self::Unload(progress) => (RouteStage::Deliver, progress),
        }
    }

    pub fn confirm(self) -> Self {
        match self {
            Self::Pickup => Self::Collect(0.0),
            Self::Delivery => Self::Unload(0.0),
            stage => stage,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NiliControl {
    pub position: [f32; 2],
    pub yaw: f32,
    pub stage: ManualStage,
}

impl NiliControl {
    pub fn target(self, route: Route) -> Stop {
        if self.stage.pickup() {
            route.pickup
        } else {
            route.delivery
        }
    }

    pub fn valid(self) -> bool {
        self.position.iter().all(|coord| coord.abs() <= 30.0)
            && self.yaw.is_finite()
            && (0.0..=1.0).contains(&self.stage.animation().1)
    }
}

impl RouteStage {
    pub fn caption(self, job: Job) -> &'static str {
        match (self, job) {
            (Self::Approach, _) => "Unterwegs zum Abholort",
            (Self::Collect, Job::Freight) => "Hebt die Kiste",
            (Self::Collect, Job::Water) => "Füllt die Krüge",
            (Self::Collect, Job::Welcome) => "Winkt den Gästen",
            (Self::Collect, Job::Shade) => "Sammelt Zweige",
            (Self::Collect, Job::Quay) => "Hebt Treibgut auf",
            (Self::Carry, Job::Welcome) => "Zeigt den Weg",
            (Self::Carry, _) => "Unterwegs",
            (Self::Deliver, Job::Freight) => "Setzt die Kiste ab",
            (Self::Deliver, Job::Water) => "Gießt Wasser ein",
            (Self::Deliver, Job::Welcome) => "Verabschiedet sich",
            (Self::Deliver, Job::Shade) => "Verteilt die Zweige",
            (Self::Deliver, Job::Quay) => "Räumt den Kai frei",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Route {
    pub job: Job,
    #[serde(default)]
    pub favor: bool,
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
    pub(crate) fn new(
        job: Job,
        favor: bool,
        from: Stop,
        pickup: Stop,
        delivery: Stop,
        started: f32,
        chapter: &Chapter,
    ) -> Option<Self> {
        let pickup_at = started + from.duration(pickup, chapter);
        let depart_at = pickup_at + 1.05;
        let arrival = depart_at + pickup.duration(delivery, chapter);
        let finish = arrival + 0.90;
        let margin = if job == Job::Welcome { 2.1 } else { 0.1 };
        (finish + margin <= chapter.day_seconds()).then_some(Self {
            job,
            favor,
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

    pub fn stage(self, seconds: f32) -> (RouteStage, f32) {
        use RouteStage::*;
        let (stage, start, end) = if seconds < self.pickup_at {
            (Approach, self.started, self.pickup_at)
        } else if seconds < self.depart_at {
            (Collect, self.pickup_at, self.depart_at)
        } else if seconds < self.arrival {
            (Carry, self.depart_at, self.arrival)
        } else {
            (Deliver, self.arrival, self.finish)
        };
        (stage, ((seconds - start) / (end - start)).clamp(0.0, 1.0))
    }

    pub fn reward(self) -> (&'static str, &'static str) {
        if self.favor {
            return ("+6 Münzen", "coin");
        }
        match self.job {
            Job::Freight if matches!(self.pickup, Stop::Parcel(_)) => {
                ("+6 Portionen", "guest_empty")
            }
            Job::Freight | Job::Water => ("+4 Portionen", "guest_empty"),
            Job::Welcome => ("+2 Gäste", "ferry"),
            Job::Shade => ("+5 Plätze", "stoa"),
            Job::Quay => ("Kai frei", "check"),
        }
    }

    pub fn valid(self, town: &Town, chapter: &Chapter, age: f32, controlled: bool) -> bool {
        [self.from, self.pickup, self.delivery]
            .iter()
            .all(|s| s.valid(town, chapter))
            && self.started >= 0.0
            && self.started <= age
            && (controlled || age < self.finish)
            && self.started < self.pickup_at
            && self.pickup_at < self.depart_at
            && self.depart_at < self.arrival
            && self.arrival < self.finish
            && self.finish <= chapter.day_seconds()
            && (matches!(self.delivery, Stop::Parcel(_)) || self.job == Job::Quay)
            && (!self.favor
                || matches!(
                    (&town.phase, self.delivery),
                    (Phase::Open(day), Stop::Parcel(lot))
                        if day.favor_at.is_none() && day.favor.job() == self.job
                            && town.lots[usize::from(lot)].is_some_and(|b| day.favor.allows(b.kind))
                ))
    }
}

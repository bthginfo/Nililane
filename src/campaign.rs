#[path = "districts.rs"]
mod districts;
#[path = "japan_districts.rs"]
mod japan;
#[path = "nordic_districts.rs"]
mod nordic;
#[path = "oasis_districts.rs"]
mod oasis;
#[path = "harbor_progression.rs"]
mod progression;

use crate::{
    errands::Job,
    layout::Layout,
    model::{DAY_SECONDS, Favor, Kind, Service, Weather},
};

pub const REGION_SIZE: usize = 24;
pub const CORE_HARBORS: usize = 12;

pub const fn harbor_stage(index: usize) -> usize {
    index % REGION_SIZE
}
pub const JAPAN_START: usize = REGION_SIZE * 2;
pub const NORDIC_START: usize = REGION_SIZE * 3;
pub const OASIS_START: usize = REGION_SIZE * 4;
pub const CHAPTER_COUNT: usize = REGION_SIZE * 5;

pub fn main_route_end(chapter: usize) -> bool {
    harbor_stage(chapter) + 1 == CORE_HARBORS
}

pub fn next_harbor(chapter: usize) -> Option<usize> {
    if main_route_end(chapter) {
        let start = Region::of(chapter).start() + REGION_SIZE;
        (start < CHAPTER_COUNT).then_some(start)
    } else {
        (harbor_stage(chapter) + 1 < REGION_SIZE).then_some(chapter + 1)
    }
}

pub fn previous_harbor(chapter: usize) -> Option<usize> {
    if chapter > 0 && harbor_stage(chapter) == 0 {
        Some(chapter - REGION_SIZE + CORE_HARBORS - 1)
    } else {
        chapter.checked_sub(1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shift {
    Day,
    Late,
    Long,
    Night,
}

impl Shift {
    pub fn night(self) -> bool {
        self == Self::Night
    }

    fn minutes(self) -> (u16, u16) {
        match self {
            Self::Day => (7 * 60, 19 * 60),
            Self::Late => (7 * 60, 20 * 60),
            Self::Long => (7 * 60, 21 * 60 + 30),
            Self::Night => (19 * 60, 30 * 60),
        }
    }

    pub fn clock(self, progress: f32) -> String {
        let (start, end) = self.minutes();
        let minute =
            (start + (progress.clamp(0.0, 1.0) * f32::from(end - start)) as u16) % (24 * 60);
        format!("{:02}:{:02}", minute / 60, minute % 60)
    }

    pub fn hours(self) -> String {
        format!("{}–{}", self.clock(0.0), self.clock(1.0))
    }

    fn extra_seconds(self) -> f32 {
        match self {
            Self::Day => 0.0,
            Self::Late => 12.0,
            Self::Long => 24.0,
            Self::Night => 14.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Greece,
    Rome,
    Japan,
    Nordic,
    Oasis,
}

impl Region {
    pub const ALL: [Self; 5] = [
        Self::Greece,
        Self::Rome,
        Self::Japan,
        Self::Nordic,
        Self::Oasis,
    ];

    pub fn of(chapter: usize) -> Self {
        Self::ALL[(chapter / REGION_SIZE).min(Self::ALL.len() - 1)]
    }

    pub fn start(self) -> usize {
        match self {
            Self::Greece => 0,
            Self::Rome => REGION_SIZE,
            Self::Japan => JAPAN_START,
            Self::Nordic => NORDIC_START,
            Self::Oasis => OASIS_START,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Greece => "Griechenland",
            Self::Rome => "Rom",
            Self::Japan => "Japan",
            Self::Nordic => "Norden",
            Self::Oasis => "Oasenreiche",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Greece => "ferry",
            Self::Rome => "forum",
            Self::Japan => "temple",
            Self::Nordic => "boatyard",
            Self::Oasis => "oasis_home",
        }
    }

    pub fn currency(self) -> &'static str {
        match self {
            Self::Greece => "Drachmen",
            Self::Rome => "Denare",
            Self::Japan => "Mon",
            Self::Nordic => "Silber",
            Self::Oasis => "Dirham",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Goal {
    Build(Kind, u32),
    Serve(Service, u32),
    Renown(u32),
    Help(u32),
    Earn(u32),
    FeastGuests(u32),
    Variety(u32),
    MarketDay(u32),
    Water(u32),
    Harvest(u32),
    Bell(u32),
    Catch(u32),
    Voyages(u32),
    CargoIncome(u32),
    Caravans(u32),
    CaravanIncome(u32),
    SpiceRoutes(u32),
    Route(Job, u32),
    RouteVariety(u32),
    FerryDay(u32),
    GoodDay(u32),
    Specials(u32),
    Residents(u32),
    Upgrade(Kind, u32),
    Pair(Kind, Kind, u32),
}

impl Goal {
    pub fn target(self) -> u32 {
        match self {
            Self::Build(_, n)
            | Self::Serve(_, n)
            | Self::Renown(n)
            | Self::Help(n)
            | Self::Earn(n)
            | Self::FeastGuests(n)
            | Self::Variety(n)
            | Self::MarketDay(n)
            | Self::Water(n)
            | Self::Harvest(n)
            | Self::Bell(n)
            | Self::Catch(n)
            | Self::Voyages(n)
            | Self::CargoIncome(n)
            | Self::Caravans(n)
            | Self::CaravanIncome(n)
            | Self::SpiceRoutes(n)
            | Self::Route(_, n)
            | Self::RouteVariety(n)
            | Self::FerryDay(n)
            | Self::GoodDay(n)
            | Self::Specials(n)
            | Self::Residents(n)
            | Self::Upgrade(_, n)
            | Self::Pair(_, _, n) => n,
        }
    }

    pub fn label(self) -> String {
        match self {
            Self::Build(kind, _) => format!("{} bauen", kind.name()),
            Self::Serve(s, _) => match s {
                Service::Bread => "Brot verkaufen",
                Service::Pottery => "Keramik verkaufen",
                Service::Shade => "Gästen Rast bieten",
                Service::Meze => "Meze servieren",
                Service::Bed => "Reisenden ein Bett anbieten",
                Service::Oil => "Olivenöl verkaufen",
                Service::Offering => "Weihegaben ausgeben",
                Service::Bath => "Badegäste empfangen",
                Service::Trade => "Gäste am Markt bedienen",
                Service::Rice => "Reisgerichte servieren",
                Service::Tea => "Schalen Tee servieren",
                Service::Lantern => "Lampions verkaufen",
                Service::Prayer => "Besuche am Tempel",
                Service::SmokedFish => "Räucherfisch verkaufen",
                Service::Textiles => "Webwaren verkaufen",
                Service::HearthMeal => "Gäste am Herd bewirten",
                Service::Honey => "Honig verkaufen",
                Service::DyedCloth => "Gefärbte Stoffe verkaufen",
                Service::Tablets => "Wachstafeln ausgeben",
                Service::Casks => "Fässer verkaufen",
                Service::Dates => "Datteln verkaufen",
                Service::Spices => "Gewürze verkaufen",
                Service::Glass => "Glaswaren verkaufen",
                Service::Incense => "Duftwaren verkaufen",
            }
            .into(),
            Self::Renown(_) => "Ansehen sammeln".into(),
            Self::Help(_) => "Bitten vom Kai erfüllen".into(),
            Self::Earn(_) => "Einnahmen sammeln".into(),
            Self::FeastGuests(_) => "Gäste an einem Festtag".into(),
            Self::Variety(_) => "Verschiedene Wünsche an einem Tag".into(),
            Self::MarketDay(_) => "Einnahmen an einem Tag".into(),
            Self::Water(_) => "Wasser bereitstellen".into(),
            Self::Harvest(_) => "Reis an einem Tag ernten".into(),
            Self::Bell(_) => "Tempelglocke läuten".into(),
            Self::Catch(_) => "Fische an einem Tag fangen".into(),
            Self::Voyages(_) => "Handelsschiffe zurückholen".into(),
            Self::CargoIncome(_) => "Silber aus dem Seehandel".into(),
            Self::Caravans(_) => "Karawanen losschicken".into(),
            Self::CaravanIncome(_) => "Dirham im Karawanenhandel verdienen".into(),
            Self::SpiceRoutes(_) => "Karawanen auf die Gewürzstraße schicken".into(),
            Self::Route(job, _) => job.name().into(),
            Self::RouteVariety(_) => "Verschiedene Touren".into(),
            Self::FerryDay(_) => "Fährgäste an einem Tag".into(),
            Self::GoodDay(_) => "Zufriedenheit (%)".into(),
            Self::Specials(_) => "Lieblingsangebote verkaufen".into(),
            Self::Residents(_) => "Nachbarn im Viertel".into(),
            Self::Upgrade(kind, _) => format!("{} · Ausbaustufe", kind.name()),
            Self::Pair(a, b, _) => format!("{} neben {}", a.name(), b.name()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Chapter {
    pub region: Region,
    pub shift: Shift,
    pub name: &'static str,
    pub district: &'static str,
    pub difficulty: &'static str,
    pub greeting: &'static str,
    pub hint: &'static str,
    pub layout: Layout,
    pub coins: u32,
    pub spring: u16,
    pub ferry: u16,
    pub walk: usize,
    pub unlock: usize,
    pub building_set: Option<&'static [Kind]>,
    pub par: u32,
    pub favor_goal: u32,
    pub starts: &'static [(usize, Kind)],
    pub locals: &'static [Service],
    pub guests: &'static [Service],
    pub weather: &'static [Weather],
    pub favors: &'static [Favor],
    pub goals: [Goal; 3],
}

impl Chapter {
    pub fn allowed(&self) -> &'static [Kind] {
        const GREEK_PLANS: [Kind; 12] = [
            House, Well, Bakery, Grove, Potter, Stoa, Taverna, HerbGarden, Inn, Storehouse, Press,
            Shrine,
        ];
        self.building_set.unwrap_or_else(|| {
            if self.unlock >= 7 {
                &GREEK_PLANS[..self.unlock + 1]
            } else {
                &Kind::ALL[..self.unlock]
            }
        })
    }

    pub fn day_seconds(&self) -> f32 {
        let walking_room = self.layout.width().saturating_sub(3) as f32 * 7.5;
        let market_room = (self.par.saturating_sub(8) as f32 * 2.25).min(23.0);
        let regional = match self.region {
            Region::Greece => 0.0,
            Region::Rome => 5.0,
            Region::Japan => 10.0,
            Region::Nordic => 12.0,
            Region::Oasis => 14.0,
        };
        DAY_SECONDS + walking_room + market_room + regional + self.shift.extra_seconds()
    }

    pub fn extra_ferry_guests(&self) -> u16 {
        self.layout.width().saturating_sub(6) as u16 * 3
    }

    pub fn harbor_plans(&self) -> bool {
        self.unlock >= 5 || self.building_set.is_some()
    }
}

use Favor::{Cargo, Jugs, Shade as ShadeFavor};
use Goal::*;
use Kind::*;
use Service::*;
use Weather::*;

const GREEK_START: [Chapter; 12] = [
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Brot für die Bucht",
        district: "Naxos",
        difficulty: "Sanft",
        greeting: "Kleio wartet auf frisches Brot. Nili hat schon einen Platz für den Ofen gefunden.",
        hint: "Wenn du den günstigen Preis wählst, können alle Gäste Brot kaufen.",
        layout: Layout::legacy(3),
        coins: 35,
        spring: 3,
        ferry: 2,
        walk: 4,
        unlock: 3,
        building_set: None,
        par: 4,
        favor_goal: 2,
        starts: &[(1, House)],
        locals: &[Bread],
        guests: &[Bread],
        weather: &[Clear],
        favors: &[Cargo, Jugs],
        goals: [Build(Bakery, 1), Serve(Bread, 15), Renown(18)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Der trockene Sommer",
        district: "Amorgos",
        difficulty: "Sanft",
        greeting: "Der Wind duftet nach Thymian. Kleio wünscht sich einen Brunnen an der Gasse.",
        hint: "Baue einen Brunnen. Schick Nili tagsüber mit Wasser zur Backstube.",
        layout: Layout::legacy(4),
        coins: 30,
        spring: 0,
        ferry: 3,
        walk: 4,
        unlock: 4,
        building_set: None,
        par: 6,
        favor_goal: 3,
        starts: &[(1, House), (7, Bakery)],
        locals: &[Bread],
        guests: &[Bread],
        weather: &[Heat, Clear, Heat, Wind],
        favors: &[Jugs, Cargo, ShadeFavor],
        goals: [
            Pair(Bakery, Well, 1),
            Route(Job::Water, 2),
            Serve(Bread, 24),
        ],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Ein Krug für Delos",
        district: "Delos",
        difficulty: "Leicht",
        greeting: "Doros bringt seine schönste Schale mit. Bald legt das erste Schiff an.",
        hint: "Eine Töpferei am Anleger ist für Fährgäste leicht zu erreichen.",
        layout: Layout::legacy(4),
        coins: 40,
        spring: 2,
        ferry: 5,
        walk: 3,
        unlock: 5,
        building_set: None,
        par: 6,
        favor_goal: 3,
        starts: &[(1, House), (7, Bakery)],
        locals: &[Bread, Bread, Pottery, Bread],
        guests: &[Pottery, Bread, Pottery],
        weather: &[Clear, Wind, Feast, Cloud],
        favors: &[Cargo, Jugs],
        goals: [
            Serve(Pottery, 16),
            Route(Job::Freight, 2),
            Upgrade(Potter, 2),
        ],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Die Säulenhalle",
        district: "Rhodos",
        difficulty: "Mittel",
        greeting: "Menon plant eine Stoa, in der die Nachbarn im Schatten sitzen können.",
        hint: "Ein Hain neben dem Wohnhof spendet den Nachbarn Schatten.",
        layout: Layout::legacy(4),
        coins: 40,
        spring: 2,
        ferry: 5,
        walk: 4,
        unlock: 6,
        building_set: None,
        par: 7,
        favor_goal: 3,
        starts: &[(1, House), (2, Well), (7, Bakery)],
        locals: &[Bread, Shade, Bread, Pottery],
        guests: &[Shade, Pottery, Bread, Shade],
        weather: &[Heat, Clear, Heat, Feast],
        favors: &[ShadeFavor, Cargo, Jugs],
        goals: [
            Serve(Shade, 22),
            Pair(House, Grove, 1),
            Route(Job::Shade, 3),
        ],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Feigen zum Sonnenfest",
        district: "Samos",
        difficulty: "Mittel",
        greeting: "Über der Gasse hängen Feigen und Kränze. Eleni deckt schon die Tische.",
        hint: "Die Taverne braucht zwei Laibe und zwei Portionen Oliven. Mit einem Ofen nebenan gibt es eine Portion Meze mehr.",
        layout: Layout::legacy(5),
        coins: 32,
        spring: 2,
        ferry: 7,
        walk: 4,
        unlock: 7,
        building_set: None,
        par: 8,
        favor_goal: 4,
        starts: &[(1, House), (7, Bakery), (2, Grove), (3, Well)],
        locals: &[Bread, Meze, Bread, Shade],
        guests: &[Meze, Meze, Shade, Bread, Pottery],
        weather: &[Clear, Feast, Heat, Feast],
        favors: &[Cargo, ShadeFavor, Jugs],
        goals: [Pair(Taverna, Bakery, 1), Serve(Meze, 30), Earn(132)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Ein Bett am Meer",
        district: "Kos",
        difficulty: "Mittel",
        greeting: "Der Wind hält die Fähren im Hafen. Mara sucht eine Unterkunft für die Reisenden.",
        hint: "Ein Gasthaus am Kai ist leicht zu erreichen. Mit genug Wasser kann das Gasthaus mehr Betten anbieten.",
        layout: Layout::legacy(5),
        coins: 46,
        spring: 1,
        ferry: 8,
        walk: 3,
        unlock: 8,
        building_set: None,
        par: 8,
        favor_goal: 4,
        starts: &[(1, House), (7, Bakery), (8, Well), (9, Stoa)],
        locals: &[Bread, Shade, Pottery, Bread],
        guests: &[Bed, Meze, Bed, Pottery, Shade],
        weather: &[Wind, Clear, Feast, Heat],
        favors: &[Jugs, Cargo, ShadeFavor],
        goals: [Serve(Bed, 24), Upgrade(Inn, 2), FerryDay(9)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Die zwei Werften",
        district: "Paros",
        difficulty: "Knifflig",
        greeting: "An beiden Enden der Bucht stellen Töpfer ihre schönsten Schalen aus.",
        hint: "Zwei Töpfereien an verschiedenen Enden der Gasse erreichen mehr Nachbarn.",
        layout: Layout::legacy(6),
        coins: 50,
        spring: 1,
        ferry: 8,
        walk: 3,
        unlock: 8,
        building_set: None,
        par: 8,
        favor_goal: 4,
        starts: &[(0, House), (4, House), (7, Bakery), (8, Well)],
        locals: &[Pottery, Bread, Pottery, Shade],
        guests: &[Pottery, Bed, Pottery, Meze, Shade],
        weather: &[Wind, Heat, Feast, Cloud],
        favors: &[Cargo, Jugs, ShadeFavor],
        goals: [Build(Potter, 2), Serve(Pottery, 38), Route(Job::Welcome, 4)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Das Fest der Laternen",
        district: "Kythnos",
        difficulty: "Knifflig",
        greeting: "Die Töpfer zünden ihre Tonlaternen an. Bald legt die Festfähre an.",
        hint: "Zum Fest kommen mehr Gäste, die auch mehr ausgeben können. Sorge vorher für volle Regale.",
        layout: Layout::legacy(6),
        coins: 52,
        spring: 1,
        ferry: 10,
        walk: 4,
        unlock: 8,
        building_set: None,
        par: 10,
        favor_goal: 5,
        starts: &[(2, House), (7, Bakery), (8, Well), (10, Stoa)],
        locals: &[Shade, Bread, Pottery, Bread],
        guests: &[Pottery, Shade, Bread, Pottery, Shade, Bed],
        weather: &[Heat, Wind, Feast, Clear],
        favors: &[Jugs, Cargo, ShadeFavor],
        goals: [Pair(Potter, Well, 1), Upgrade(Stoa, 3), GoodDay(76)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Unter dem Stern der Dione",
        district: "Thera",
        difficulty: "Knifflig",
        greeting: "Menon holt die Honigkrüge vom Hang. Auf der Tafel fehlt noch Platz für warmes Essen.",
        hint: "Beim Ausbau steigt der Wasserbedarf. Prüfe im Hafenplan, ob das Wasser reicht.",
        layout: Layout::legacy(6),
        coins: 59,
        spring: 0,
        ferry: 10,
        walk: 4,
        unlock: 8,
        building_set: None,
        par: 10,
        favor_goal: 5,
        starts: &[(1, House), (7, Bakery), (8, Well), (10, Stoa)],
        locals: &[Shade, Bread, Meze, Shade],
        guests: &[Shade, Bed, Meze, Meze, Pottery],
        weather: &[Heat, Clear, Heat, Feast],
        favors: &[ShadeFavor, Jugs, Cargo],
        goals: [Pair(Taverna, HerbGarden, 1), Serve(Meze, 36), MarketDay(36)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Die Hafengemeinschaft",
        district: "Milos",
        difficulty: "Anspruchsvoll",
        greeting: "Nili rollt die Karte aus. Am Hafen ist Platz für ein ganzes Viertel.",
        hint: "Nachbarn kaufen gern günstig. Am Kai können sich manche Gäste höhere Preise leisten.",
        layout: Layout::legacy(6),
        coins: 64,
        spring: 1,
        ferry: 11,
        walk: 3,
        unlock: 8,
        building_set: None,
        par: 10,
        favor_goal: 5,
        starts: &[(0, House), (2, House), (7, Bakery), (9, Grove)],
        locals: &[Bread, Meze, Shade, Pottery],
        guests: &[Bed, Pottery, Meze, Bread, Bed, Shade],
        weather: &[Wind, Cloud, Feast, Heat, Clear],
        favors: &[Jugs, Cargo, ShadeFavor],
        goals: [Earn(425), Pair(House, Grove, 2), Serve(Bed, 34)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Die letzte Nachtfähre",
        district: "Ikaria",
        difficulty: "Anspruchsvoll",
        greeting: "Mit der letzten Fähre kommen Gäste, die noch ein Abendessen und ein Bett suchen.",
        hint: "Halte alle fünf Angebote in der Nähe des Kais bereit.",
        layout: Layout::legacy(6),
        coins: 66,
        spring: 0,
        ferry: 12,
        walk: 3,
        unlock: 8,
        building_set: None,
        par: 11,
        favor_goal: 6,
        starts: &[(1, House), (4, House), (7, Bakery), (8, Well)],
        locals: &[Bread, Shade, Pottery, Bread, Meze],
        guests: &[Bed, Bed, Pottery, Meze, Shade, Bread, Bed],
        weather: &[Wind, Heat, Clear, Feast, Wind],
        favors: &[Cargo, Jugs, ShadeFavor],
        goals: [Variety(5), Serve(Bed, 37), RouteVariety(4)],
    },
    Chapter {
        region: Region::Greece,
        shift: crate::campaign::Shift::Day,
        name: "Nilis lange Tafel",
        district: "Kallisti",
        difficulty: "Anspruchsvoll",
        greeting: "Boote aus der ganzen Ägäis liegen vor Anker. Heute deckt Nili eine lange Tafel.",
        hint: "Mit zwei Öfen bleibt genug Brot für die Tavernen und den Verkauf.",
        layout: Layout::legacy(6),
        coins: 73,
        spring: 1,
        ferry: 13,
        walk: 4,
        unlock: 8,
        building_set: None,
        par: 11,
        favor_goal: 6,
        starts: &[(1, House), (5, House), (7, Bakery), (8, Well)],
        locals: &[Bread, Meze, Shade, Pottery, Bread],
        guests: &[Bed, Meze, Shade, Pottery, Bread, Meze, Bed],
        weather: &[Clear, Heat, Feast, Wind, Feast],
        favors: &[Cargo, Jugs, ShadeFavor],
        goals: [Serve(Meze, 48), Serve(Bed, 42), Renown(132)],
    },
];

pub static CHAPTERS: [Chapter; CHAPTER_COUNT] = {
    let mut chapters = [GREEK_START[0]; CHAPTER_COUNT];
    let mut i = 0;
    while i < GREEK_START.len() {
        chapters[i] = GREEK_START[i];
        i += 1;
    }
    while i < REGION_SIZE {
        chapters[i] = districts::GREEK_LATE[i - GREEK_START.len()];
        i += 1;
    }
    while i < JAPAN_START {
        chapters[i] = districts::ROMAN[i - REGION_SIZE];
        i += 1;
    }
    while i < NORDIC_START {
        chapters[i] = japan::CHAPTERS[i - JAPAN_START];
        i += 1;
    }
    while i < OASIS_START {
        chapters[i] = nordic::CHAPTERS[i - NORDIC_START];
        i += 1;
    }
    while i < CHAPTER_COUNT {
        chapters[i] = oasis::CHAPTERS[i - OASIS_START];
        i += 1;
    }
    let mut index = 0;
    while index < CHAPTER_COUNT {
        chapters[index].layout = chapters[index].layout.for_chapter(index);
        chapters[index].shift = match harbor_stage(index) {
            7 | 14 | 21 => Shift::Night,
            17.. => Shift::Long,
            9..=16 => Shift::Late,
            _ => Shift::Day,
        };
        chapters[index] = progression::revise(chapters[index], harbor_stage(index));
        index += 1;
    }
    chapters
};

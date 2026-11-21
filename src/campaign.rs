#[path = "districts.rs"]
mod districts;

use crate::model::{DAY_SECONDS, Favor, Kind, Service, Weather};

pub const REGION_SIZE: usize = 24;
pub const CHAPTER_COUNT: usize = REGION_SIZE * 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Greece,
    Rome,
}

impl Region {
    pub fn of(chapter: usize) -> Self {
        if chapter < REGION_SIZE {
            Self::Greece
        } else {
            Self::Rome
        }
    }

    pub fn start(self) -> usize {
        match self {
            Self::Greece => 0,
            Self::Rome => REGION_SIZE,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Greece => "Griechenland",
            Self::Rome => "Rom",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Greece => "ferry",
            Self::Rome => "forum",
        }
    }

    pub fn currency(self) -> &'static str {
        match self {
            Self::Greece => "Drachmen",
            Self::Rome => "Denare",
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
                Service::Bed => "Reisende beherbergen",
                Service::Oil => "Olivenöl verkaufen",
                Service::Offering => "Weihegaben ausgeben",
                Service::Bath => "Badegäste empfangen",
                Service::Trade => "Marktbesuche betreuen",
            }
            .into(),
            Self::Renown(_) => "Ansehen sammeln".into(),
            Self::Help(_) => "Hafenbitten erfüllen".into(),
            Self::Earn(_) => "Einnahmen sammeln".into(),
            Self::FeastGuests(_) => "Gäste an einem Festtag".into(),
            Self::Variety(_) => "Wunscharten an einem Tag".into(),
            Self::MarketDay(_) => "Einnahmen an einem Tag".into(),
            Self::Water(_) => "Krüge Wasser bereitstellen".into(),
            Self::Upgrade(kind, _) => format!("{} · Ausbaustufe", kind.name()),
            Self::Pair(a, b, _) => format!("{} neben {}", a.name(), b.name()),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Chapter {
    pub region: Region,
    pub name: &'static str,
    pub district: &'static str,
    pub difficulty: &'static str,
    pub greeting: &'static str,
    pub hint: &'static str,
    pub columns: usize,
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
        self.building_set.unwrap_or(&Kind::ALL[..self.unlock])
    }

    pub fn day_seconds(&self) -> f32 {
        let growth = self.par.saturating_sub(8) as f32;
        match self.region {
            Region::Greece => {
                DAY_SECONDS + (growth * 1.45).min(24.0) + if self.par >= 15 { 13.0 } else { 0.0 }
            }
            Region::Rome => 45.0 + (growth * 1.15).min(16.0),
        }
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
        name: "Brot für die Bucht",
        district: "Naxos",
        difficulty: "Sanft",
        greeting: "Kleio winkt vom ersten Wohnhof. Als die Fähre wieder anlegt, soll es in der Gasse nach Brot duften. Nili kennt schon den besten Platz für den Ofen.",
        hint: "Baue nahe dem Wohnhof. Mit dem günstigen Preis kann jede Familie Brot kaufen.",
        columns: 3,
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
        name: "Der trockene Sommer",
        district: "Amorgos",
        difficulty: "Sanft",
        greeting: "Der Meltemi trägt den Duft von wildem Thymian herüber. Auf Amorgos steht die Quelle tief. Kleio bittet Nili, die neuen Höfe um einen Brunnen anzulegen.",
        hint: "Backstuben mit einem Brunnen höchstens zwei Plätze entfernt werden zuerst versorgt.",
        columns: 4,
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
        goals: [Pair(Bakery, Well, 1), Build(Grove, 1), Serve(Bread, 28)],
    },
    Chapter {
        region: Region::Greece,
        name: "Ein Krug für Delos",
        district: "Delos",
        difficulty: "Leicht",
        greeting: "Doros bringt eine Musterfliese vom Südkai mit. Die Fährgäste haben Keramik bestellt und wollen beim Geruch von frischem Brot durch die Gasse schlendern.",
        hint: "Die Fähre legt rechts an. Stelle die Töpferei in die Nähe des Anlegers.",
        columns: 4,
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
        goals: [Serve(Pottery, 16), Serve(Bread, 18), Upgrade(Potter, 2)],
    },
    Chapter {
        region: Region::Greece,
        name: "Die Säulenhalle",
        district: "Rhodos",
        difficulty: "Mittel",
        greeting: "Im Pflaster des alten Platzes liegt ein Mosaik. Menon hat seine Zeichen gedeutet und einen Plan für eine Stoa entworfen. Unter dem Dach sollen Nachbarn und Reisende zusammenrücken.",
        hint: "Olivenhaine neben Wohnhöfen halten sie bei Hitze kühl. Eine Stoa schafft Rastplätze für alle.",
        columns: 4,
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
        goals: [Serve(Shade, 22), Pair(House, Grove, 1), Renown(47)],
    },
    Chapter {
        region: Region::Greece,
        name: "Feigen zum Sonnenfest",
        district: "Samos",
        difficulty: "Mittel",
        greeting: "Am Abend hängen Feigen und Kränze über der Gasse. Eleni möchte vor dem Sonnenfest ihre erste Taverne eröffnen. Bei Nili ist ein Platz am gedeckten Tisch frei.",
        hint: "Jede Taverne verarbeitet zwei Laibe und zwei Olivenportionen. Direkt neben dem Ofen reicht es für eine Portion mehr.",
        columns: 5,
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
        name: "Ein Bett am Meer",
        district: "Kos",
        difficulty: "Mittel",
        greeting: "Der Wind lässt die Heimfähren im Hafen liegen. Mara leiht Nili das Familienbuch der Insel. Darin steht, welche Gäste heute Abend noch eine Herberge suchen.",
        hint: "Gasthäuser am Kai sind für Reisende gut erreichbar. Zwei kleine Brunnen versorgen mehrere Betriebe zuverlässig.",
        columns: 5,
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
        goals: [Serve(Bed, 24), Upgrade(Inn, 2), Renown(68)],
    },
    Chapter {
        region: Region::Greece,
        name: "Die zwei Werften",
        district: "Paros",
        difficulty: "Knifflig",
        greeting: "Zwei Bootsbauer streiten darüber, auf welcher Seite der Bucht das schönere Geschirr gebrannt wird. Nili verteilt die Aufträge und lädt beide Werkstätten an denselben Tisch ein.",
        hint: "Nah gelegene und günstige Läden füllen die ersten Bestellungen. Zwei Töpfereien an verschiedenen Straßenenden erreichen mehr Nachbarn.",
        columns: 6,
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
        goals: [Build(Potter, 2), Serve(Pottery, 38), Help(4)],
    },
    Chapter {
        region: Region::Greece,
        name: "Das Fest der Laternen",
        district: "Kythnos",
        difficulty: "Knifflig",
        greeting: "Die Töpfer entzünden die ersten Tonlaternen. In drei Tagen kommt die Festfähre und die Stoa soll voller Stimmen sein. Nili legt die Liste der erwarteten Gäste auf den Brunnenrand.",
        hint: "An Festtagen sind die Fähren voller und die Budgets höher. Prüfe vor dem Öffnen, ob alle Werkstätten genug Wasser bekommen.",
        columns: 6,
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
        goals: [Pair(Potter, Well, 1), Upgrade(Stoa, 3), FeastGuests(13)],
    },
    Chapter {
        region: Region::Greece,
        name: "Unter dem Stern der Dione",
        district: "Thera",
        difficulty: "Knifflig",
        greeting: "In einer windstillen Nacht leuchtet Diones Stern über dem alten Heiligtum. Morgen füllen Pilger die Arkaden. Menon und Nili wollen ihnen einen Hof mit Schatten und warmem Abendbrot schenken.",
        hint: "Vergleiche den Wasservorrat mit der Nachfrage. Beim Ausbau steigen Ertrag und Wasserbedarf.",
        columns: 6,
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
        goals: [Serve(Shade, 36), Serve(Meze, 36), Renown(94)],
    },
    Chapter {
        region: Region::Greece,
        name: "Die Hafengemeinschaft",
        district: "Milos",
        difficulty: "Anspruchsvoll",
        greeting: "Der Hafenrat hat den Schreiber mit einer langen Liste geschickt. Zeit für eine kurze Pause im Schatten: Nili holt die großen Karten heraus und verteilt zwölf Bauplätze.",
        hint: "Günstige Preise passen zu Nachbarn. Feine Angebote am Kai eignen sich für Gäste mit größerem Budget.",
        columns: 6,
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
        name: "Die letzte Nachtfähre",
        district: "Ikaria",
        difficulty: "Anspruchsvoll",
        greeting: "Die Wetterfahne klappert am Kai. Wenn die letzte Nachtfähre einläuft, finden ihre Gäste erst Schatten, dann Abendbrot und schließlich ein freies Bett. Mara bittet Nili, die Aussicht am Kai im Auge zu behalten.",
        hint: "Ein vielseitiger Hafen erfüllt alle fünf Wunscharten am selben Tag. Halte für die Fährgäste genügend Vorräte in Reichweite des Kais bereit.",
        columns: 6,
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
        goals: [Variety(5), Serve(Bed, 37), Help(6)],
    },
    Chapter {
        region: Region::Greece,
        name: "Nilis lange Tafel",
        district: "Kallisti",
        difficulty: "Anspruchsvoll",
        greeting: "Boote von allen Inseln liegen vor Anker. Eleni hat die Feigen, Doros die Schalen und Kleio das Brot. An Nilis langer Tafel soll Platz für jede Reisegeschichte sein.",
        hint: "Zwei Öfen sichern Brot für die Theke und die Tavernen. Plane die Herberge am Kai und bewahre Raum für einen schattigen Hof.",
        columns: 6,
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
    while i < CHAPTER_COUNT {
        chapters[i] = districts::ROMAN[i - REGION_SIZE];
        i += 1;
    }
    chapters
};

//! Fictional market towns inspired by Viking Age routes from Lofoten to Hedeby.
use super::{Chapter, Goal::*, Layout, REGION_SIZE, Region};
use crate::{
    errands::Job,
    model::{
        Favor::{Cargo, Jugs},
        Kind::{
            Boatyard, FeastHall, Fishery, Longhouse, Smokehouse, Storehouse, Weaver, Well, Woodlot,
        },
        Service::{HearthMeal as Hearth, SmokedFish, Textiles},
        Weather::{Clear, Cloud, Feast, Rain, Storm, Wind},
    },
};

const FISHING: &[crate::model::Kind] = &[Longhouse, Well, Fishery, Smokehouse, Woodlot];
const TRADE: &[crate::model::Kind] = &[Longhouse, Well, Fishery, Smokehouse, Woodlot, Boatyard];
const WEAVING: &[crate::model::Kind] = &[
    Longhouse, Well, Fishery, Smokehouse, Woodlot, Boatyard, Weaver,
];
const COAST: &[crate::model::Kind] = &[
    Longhouse, Well, Fishery, Smokehouse, Woodlot, Boatyard, Weaver, FeastHall, Storehouse,
];

const LAGOON: Chapter = Chapter {
    region: Region::Nordic,
    name: "",
    district: "",
    difficulty: "Sanft",
    greeting: "",
    hint: "",
    layout: Layout::legacy(4),
    coins: 58,
    spring: 4,
    ferry: 7,
    walk: 5,
    unlock: 0,
    building_set: Some(FISHING),
    par: 8,
    favor_goal: 3,
    starts: &[(1, Longhouse), (2, Well), (6, Fishery)],
    locals: &[SmokedFish],
    guests: &[SmokedFish],
    weather: &[Cloud, Clear, Rain, Wind, Feast],
    favors: &[Cargo, Jugs, Cargo],
    goals: [
        Pair(Smokehouse, Fishery, 1),
        Serve(SmokedFish, 25),
        Renown(30),
    ],
};

const BOATS: Chapter = Chapter {
    building_set: Some(TRADE),
    layout: Layout::legacy(5),
    ferry: 9,
    starts: &[
        (1, Longhouse),
        (2, Well),
        (3, Fishery),
        (7, Smokehouse),
        (8, Woodlot),
    ],
    coins: 70,
    par: 12,
    favor_goal: 5,
    weather: &[Wind, Cloud, Clear, Rain, Feast, Clear],
    ..LAGOON
};

const CLOTH: Chapter = Chapter {
    building_set: Some(WEAVING),
    difficulty: "Leicht",
    starts: &[
        (1, Longhouse),
        (2, Well),
        (3, Fishery),
        (7, Smokehouse),
        (8, Woodlot),
        (9, Boatyard),
    ],
    locals: &[SmokedFish, Textiles, SmokedFish],
    guests: &[Textiles, SmokedFish, Textiles, SmokedFish],
    ferry: 12,
    par: 14,
    favor_goal: 6,
    ..BOATS
};

const HEARTH: Chapter = Chapter {
    building_set: Some(COAST),
    difficulty: "Mittel",
    starts: &[
        (1, Longhouse),
        (2, Well),
        (3, Fishery),
        (7, Smokehouse),
        (8, Woodlot),
        (9, Boatyard),
        (10, Weaver),
    ],
    locals: &[SmokedFish, Textiles, Hearth, SmokedFish, Hearth],
    guests: &[Hearth, SmokedFish, Textiles, Hearth, SmokedFish, Textiles],
    weather: &[Cloud, Clear, Wind, Feast, Rain, Storm, Clear],
    ferry: 16,
    coins: 93,
    par: 17,
    favor_goal: 7,
    ..CLOTH
};

const STRAIT: Chapter = Chapter {
    layout: Layout::legacy(6),
    difficulty: "Anspruchsvoll",
    starts: &[
        (1, Longhouse),
        (2, Well),
        (3, Fishery),
        (7, Smokehouse),
        (8, Woodlot),
        (9, Boatyard),
        (10, Weaver),
        (11, FeastHall),
    ],
    ferry: 20,
    coins: 115,
    par: 23,
    favor_goal: 10,
    ..HEARTH
};

const OCEAN: Chapter = Chapter {
    ferry: 24,
    coins: 131,
    par: 27,
    favor_goal: 12,
    weather: &[Rain, Wind, Storm, Cloud, Clear, Feast, Wind, Clear],
    ..STRAIT
};

pub(super) const CHAPTERS: [Chapter; REGION_SIZE] = [
    Chapter {
        name: "Rauch über dem ersten Fjord",
        district: "Borg",
        greeting: "Liv bringt den Fang vom Ufer. Im Langhaus warten die ersten Gäste auf warmen Räucherfisch.",
        hint: "Neben der Fischerhütte macht die Räucherei mehr Portionen aus dem Fang.",
        ..LAGOON
    },
    Chapter {
        name: "Trockenes Holz im Regen",
        district: "Vågan",
        greeting: "Eira sammelt Zweige und deckt den Holzstapel ab. Am Fjord ziehen Regentropfen über das Wasser.",
        hint: "Ein Holzplatz versorgt die Räucherei. Nebenan reicht das trockene Holz für zusätzliche Portionen.",
        starts: &[(1, Longhouse), (2, Well), (3, Fishery), (7, Smokehouse)],
        goals: [
            Pair(Smokehouse, Woodlot, 1),
            Catch(14),
            Serve(SmokedFish, 42),
        ],
        par: 10,
        ..LAGOON
    },
    Chapter {
        name: "Das erste Handelsschiff",
        district: "Skrova",
        greeting: "Leif hat eine Route zur Nachbarinsel gefunden. Für die Überfahrt braucht die Besatzung drei Portionen.",
        hint: "Geh mit Nili zum Bootshaus und wähle den Inselhandel. Der Proviant kommt aus einer Räucherei in der Nähe.",
        goals: [Build(Boatyard, 1), Voyages(3), CargoIncome(48)],
        ..BOATS
    },
    Chapter {
        name: "Wolle in den Fenstern",
        district: "Henningsvær",
        greeting: "Signe hängt die ersten gewebten Bänder vor die Tür. Kinder bleiben stehen und zeigen auf die Farben.",
        hint: "Im Tagesplan siehst du, für wen sich bunte Bänder oder warme Schals lohnen.",
        goals: [Build(Weaver, 1), Serve(Textiles, 32), GoodDay(77)],
        ..CLOTH
    },
    Chapter {
        name: "Die Halle am Feuer",
        district: "Reine",
        greeting: "Tora stellt Bänke am Herd auf. Fischer und Weber teilen sich heute die warme Halle.",
        hint: "Eine Räucherei neben der Festhalle bringt mehr Essen auf die Tafel.",
        goals: [
            Pair(FeastHall, Smokehouse, 1),
            Serve(Hearth, 35),
            Variety(3),
        ],
        ..HEARTH
    },
    Chapter {
        name: "Mit vollen Kisten",
        district: "Røst",
        greeting: "Auf den Inseln wird nach Fisch gefragt. Leif will die Fahrt über das offene Meer wagen.",
        hint: "Ab Stufe 2 kann das Bootshaus Fernhandel betreiben. Sechs Portionen bringen nach zwei Tagen mehr Silber.",
        goals: [
            Upgrade(Boatyard, 2),
            CargoIncome(108),
            Serve(SmokedFish, 47),
        ],
        ..HEARTH
    },
    Chapter {
        name: "Bänke für die Fähre",
        district: "Trondenes",
        greeting: "Ein kalter Wind kommt von der Landzunge. Tora hält am Herd Plätze für Fährgäste frei.",
        hint: "Mit einer Gästetour begleitet Nili Reisende vom Kai direkt in die Halle.",
        goals: [Route(Job::Welcome, 4), Serve(Hearth, 51), FerryDay(12)],
        ..HEARTH
    },
    Chapter {
        name: "Holz an der Anlegestelle",
        district: "Senja",
        greeting: "Böen werfen Äste über die Ufergasse. Eira bittet Nili, den Kai freizuräumen.",
        hint: "Beim Räumen öffnet sich der Weg für Fährgäste. Bei Sturm warten die Handelsschiffe mit der Abfahrt.",
        goals: [Route(Job::Quay, 4), Voyages(5), Serve(SmokedFish, 57)],
        ..STRAIT
    },
    Chapter {
        name: "Fang nach dem Sturm",
        district: "Kvaløya",
        greeting: "Die Netze hängen zum Trocknen aus. Liv wartet auf einen ruhigen Morgen für den nächsten Fang.",
        hint: "Wind und Sturm verkleinern den Fang. Mehr als eine Fischerhütte kann die Lücken ausgleichen.",
        goals: [Catch(30), Upgrade(Fishery, 3), Serve(SmokedFish, 70)],
        ..STRAIT
    },
    Chapter {
        name: "Ein Markt am Strand",
        district: "Sommarøy",
        greeting: "Die Fähre bringt Körbe, Wolle und Besucher. Signe hat ihren Stand bis an die Tür verlängert.",
        hint: "Passende Angebote erhöhen die Tageseinnahmen. Auch an günstigen Tagen zählt der Absatz.",
        goals: [Upgrade(Weaver, 3), Serve(Textiles, 62), MarketDay(84)],
        ..STRAIT
    },
    Chapter {
        name: "Vom Wald zum Herd",
        district: "Hitra",
        greeting: "Die Scheite liegen unter Fichtenzweigen. Eira plant genug Platz für die Räucherei und die warme Halle.",
        hint: "Ordne die Räucherei zwischen Holzplatz und Festhalle an. Kurze Wege liefern mehr Portionen.",
        goals: [
            Pair(Smokehouse, Woodlot, 1),
            Pair(FeastHall, Smokehouse, 1),
            Serve(Hearth, 76),
        ],
        ..STRAIT
    },
    Chapter {
        name: "Gegen die Strömung",
        district: "Frøya",
        greeting: "Die Leute beobachten die Wolken über der Einfahrt. Leif entscheidet erst dann über die nächste Fahrt.",
        hint: "Vergleiche Proviant und Wetter für morgen und übermorgen. Wartezeiten belegen das Schiff.",
        goals: [Voyages(9), CargoIncome(169), FerryDay(16)],
        ..STRAIT
    },
    Chapter {
        name: "Tücher für die Nachbarn",
        district: "Smøla",
        greeting: "Am Morgen hält Signe drei Muster ans Fenster. Die Nachbarn kommen schon vor der Fähre.",
        hint: "Mehr Nachbarn verändern die Nachfrage. Die Spezialitäten jedes Ladens folgen seiner Kundschaft.",
        goals: [Upgrade(Longhouse, 4), Serve(Textiles, 85), GoodDay(86)],
        ..STRAIT
    },
    Chapter {
        name: "Die zweite Räucherei",
        district: "Averøya",
        greeting: "Der erste Holzstapel ist aufgebraucht. Liv und Eira richten einen zweiten Hof für den Fang ein.",
        hint: "Vorräte in zwei Räuchereien halten den Verkauf am Laufen, wenn ein Schiff Proviant lädt.",
        goals: [Build(Smokehouse, 2), Catch(38), Serve(SmokedFish, 113)],
        ..OCEAN
    },
    Chapter {
        name: "Festtag auf der kleinen Insel",
        district: "Grip",
        greeting: "Die Fähre liegt voll in der Bucht. In der Halle rücken die Gäste zusammen.",
        hint: "Am Festtag kommen mehr Reisende. Ausbauten und Lieferungen schaffen freie Plätze für den Ansturm.",
        goals: [FeastGuests(26), Upgrade(FeastHall, 4), Serve(Hearth, 112)],
        ..OCEAN
    },
    Chapter {
        name: "Kisten voller Wolle",
        district: "Giske",
        greeting: "Signe stapelt Ballen am Speicher. Die hölzerne Tür ist vom vielen Öffnen ganz hell geworden.",
        hint: "Ein Speicher neben der Weberei hilft schon morgens. Tagsüber kann Nili dort Nachschub holen.",
        goals: [
            Pair(Storehouse, Weaver, 1),
            Route(Job::Freight, 7),
            Serve(Textiles, 104),
        ],
        ..OCEAN
    },
    Chapter {
        name: "Ankunft am Felsen",
        district: "Runde",
        greeting: "Vögel ziehen über die Felsen. Zwei Fähren warten darauf, dass der Anleger frei wird.",
        hint: "Wenn du den Kai räumst, kommen Fährgäste weiter. Mit Nili kannst du zusätzlich einzelne Gruppen begleiten.",
        goals: [Route(Job::Quay, 5), Route(Job::Welcome, 6), FerryDay(22)],
        ..OCEAN
    },
    Chapter {
        name: "Ein Herd für alle",
        district: "Selja",
        greeting: "In der Halle findet jedes Alter einen Platz. Liv bringt den letzten Fang gerade rechtzeitig.",
        hint: "Genug Plätze helfen der Stimmung. Im Gästebuch siehst du, wessen Wunsch noch offenblieb.",
        goals: [Serve(Hearth, 121), GoodDay(89), Variety(3)],
        ..OCEAN
    },
    Chapter {
        name: "Silber hinter der Landzunge",
        district: "Kaupanger",
        greeting: "Leif kennt die Buchten jetzt auswendig. Die nächste Ladung geht weit über den Fjord hinaus.",
        hint: "Fernhandel bringt mehr Silber je Fahrt. Kurze Fahrten machen Platz für die nächste Ladung.",
        goals: [CargoIncome(300), Upgrade(Boatyard, 4), Earn(704)],
        ..OCEAN
    },
    Chapter {
        name: "Markttag zwischen Bergen",
        district: "Balestrand",
        greeting: "Wasser glitzert auf den Dächern. Kaufleute warten an drei Türen auf den nächsten Platz.",
        hint: "Beobachte die Preise an einem Festtag. Das Silber vom Schiff steht getrennt von den Tageseinnahmen.",
        goals: [MarketDay(135), FeastGuests(30), Serve(Textiles, 125)],
        par: 29,
        ..OCEAN
    },
    Chapter {
        name: "Hilfe auf dem Holzsteg",
        district: "Moster",
        greeting: "Kisten rutschen über das nasse Holz. Eira zeigt Nili, wo eine Hand frei bleiben soll.",
        hint: "Bring zuerst Material, wenn etwas fehlt. Eine kurze Tour erleichtert den Verkauf für den Rest des Tages.",
        goals: [RouteVariety(4), Serve(SmokedFish, 132), Help(10)],
        par: 29,
        ..OCEAN
    },
    Chapter {
        name: "Die lange Route",
        district: "Karmøy",
        greeting: "Die Besatzung kennt jeden Ankerplatz. Im Ort werden schon die Körbe für die nächste Fahrt geflochten.",
        hint: "Segle in ruhigen Wetterfenstern. Plane genug Fisch für die Ausfahrt und für die Gasse.",
        goals: [Voyages(13), CargoIncome(345), Catch(43)],
        par: 31,
        favor_goal: 13,
        ..OCEAN
    },
    Chapter {
        name: "Die Hallen von Ribe",
        district: "Ribe",
        greeting: "Liv, Signe und Tora decken gemeinsam die Tische. Hinter der Tür wartet der ganze Markt.",
        hint: "Gute Preise, große Hallen und ein freier Kai machen einen Festtag möglich.",
        goals: [GoodDay(92), Serve(Hearth, 155), MarketDay(155)],
        par: 32,
        favor_goal: 14,
        ..OCEAN
    },
    Chapter {
        name: "Die letzte Fahrt im Abendlicht",
        district: "Hedeby",
        greeting: "Alle Reisen führen an diesem Abend zusammen. Leif wartet auf das Segel hinter der Einfahrt.",
        hint: "Halte den Markt und das Bootshaus gleichzeitig am Laufen. Die letzte Ladung zählt bei ihrer Rückkehr.",
        goals: [Voyages(15), CargoIncome(420), FeastGuests(34)],
        ferry: 28,
        par: 35,
        coins: 150,
        favor_goal: 16,
        difficulty: "Finale Nordische Häfen",
        ..OCEAN
    },
];

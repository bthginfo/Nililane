use super::{Chapter, Goal::*, Region};
use crate::errands::Job;
use crate::model::{
    Favor::{Cargo, Jugs, Shade as ShadeFavor},
    Kind::*,
    Service::*,
    Weather::*,
};

const GREEK_TRADE: Chapter = Chapter {
    region: Region::Greece,
    name: "",
    district: "",
    difficulty: "Anspruchsvoll",
    greeting: "",
    hint: "",
    layout: super::Layout::legacy(6),
    coins: 68,
    spring: 1,
    ferry: 14,
    walk: 5,
    unlock: 9,
    building_set: None,
    par: 16,
    favor_goal: 8,
    starts: &[(1, House), (2, Well), (3, Grove), (7, Bakery)],
    locals: &[Bread, Meze, Shade, Bread, Pottery],
    guests: &[Bread, Pottery, Bed, Shade, Meze, Pottery, Bed, Bread],
    weather: &[Clear, Heat, Wind, Cloud, Feast],
    favors: &[Cargo, Jugs, ShadeFavor],
    goals: [Earn(1), Renown(1), Help(1)],
};

const GREEK_OIL: Chapter = Chapter {
    unlock: 10,
    par: 18,
    favor_goal: 9,
    coins: 72,
    ferry: 16,
    guests: &[Oil, Pottery, Bed, Oil, Meze, Shade, Oil, Bread, Oil],
    locals: &[Bread, Meze, Oil, Shade, Bread, Pottery],
    weather: &[Clear, Cloud, Heat, Feast, Wind],
    ..GREEK_TRADE
};

const GREEK_TEMPLE: Chapter = Chapter {
    unlock: 11,
    coins: 86,
    par: 22,
    favor_goal: 11,
    ferry: 18,
    guests: &[
        Offering, Bed, Oil, Pottery, Offering, Meze, Shade, Bread, Offering, Bed,
    ],
    locals: &[Bread, Offering, Meze, Pottery, Shade, Oil, Bread],
    weather: &[Heat, Rain, Cloud, Feast, Wind],
    ..GREEK_OIL
};

pub(super) const GREEK_LATE: [Chapter; 12] = [
    Chapter {
        name: "Der Speicher der Bucht",
        district: "Andros",
        greeting: "Die Schiffe bringen Korn. Zwischen den Öfen ist Platz für einen Speicher.",
        hint: "Der Speicher füllt die Vorräte von Läden bis zu zwei Bauplätze entfernt auf.",
        par: 15,
        ferry: 17,
        starts: &[(1, House), (2, Well), (7, Bakery), (8, Taverna)],
        goals: [
            Pair(Storehouse, Bakery, 1),
            Serve(Bread, 66),
            Route(Job::Freight, 5),
        ],
        ..GREEK_TRADE
    },
    Chapter {
        name: "Die erste Ölpresse",
        district: "Aigina",
        greeting: "Am Kai steht der erste Korb Oliven. Eleni möchte daraus Öl pressen.",
        hint: "Neben einem Hain produziert die Ölpresse mehr.",
        par: 17,
        goals: [Pair(Press, Grove, 1), Serve(Oil, 100), Upgrade(Grove, 3)],
        ..GREEK_OIL
    },
    Chapter {
        name: "Duftender Markt",
        district: "Chios",
        greeting: "Am Markt riecht es nach Kräutern und warmem Brot. Bald legen die Händler an.",
        hint: "Tavernen bekommen zuerst Oliven. Baue den Hain aus, damit genug für die Presse übrig bleibt.",
        par: 18,
        coins: 66,
        starts: &[(1, House), (2, Well), (3, Grove), (7, Bakery), (8, Taverna)],
        goals: [Serve(Meze, 63), Serve(Oil, 100), MarketDay(74)],
        guests: &[Oil, Pottery, Meze, Oil, Meze, Shade, Oil, Bread, Bed],
        ..GREEK_OIL
    },
    Chapter {
        name: "Die Gaben im Marmorhof",
        district: "Epidauros",
        greeting: "Doros bringt bemalte Schalen zum Heiligtum. Nili bereitet den Vorhof für das Fest vor.",
        hint: "Für die Weihegaben braucht das Heiligtum zwei Schalen aus der nächsten Töpferei. Halte auch Schalen für den Verkauf zurück.",
        par: 18,
        starts: &[(1, House), (2, Well), (3, Grove), (4, Potter), (7, Bakery)],
        goals: [Pair(Shrine, Potter, 1), Serve(Offering, 80), FerryDay(15)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Rast im Pinienwind",
        district: "Skopelos",
        greeting: "Der Wind treibt die Boote hinter die Felsen. In der Stoa finden Gäste Schutz.",
        hint: "Bei Wind, Regen oder Sturm kann Nili den Kai räumen.",
        par: 18,
        spring: 0,
        weather: &[Wind, Cloud, Storm, Heat, Rain, Feast],
        locals: &[Shade, Bread, Shade, Meze, Pottery, Shade],
        guests: &[
            Shade, Meze, Bed, Shade, Oil, Offering, Bread, Shade, Pottery,
        ],
        goals: [Serve(Shade, 111), Route(Job::Quay, 5), Route(Job::Shade, 7)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Meer aus Tonlaternen",
        district: "Syros",
        greeting: "Über jedem Tor hängt ein Tonstern. Die Fährleute feiern heute mit.",
        hint: "Prüfe die Nachfrage im Hafenplan, bevor du mehr Fährgäste einlädst.",
        par: 20,
        ferry: 19,
        goals: [FeastGuests(18), Upgrade(Potter, 4), Serve(Offering, 93)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Zwei Schreiber am Kai",
        district: "Tinos",
        greeting: "Kleio legt frisches Brot und Olivenöl an einem gemeinsamen Stand aus.",
        hint: "Läden auf der Promenade sind für Fährgäste leichter zu erreichen.",
        par: 21,
        walk: 4,
        coins: 88,
        goals: [Serve(Bread, 75), Serve(Oil, 121), Renown(390)],
        guests: &[
            Oil, Pottery, Bread, Oil, Bed, Shade, Oil, Offering, Bread, Meze,
        ],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Die Abende von Serifos",
        district: "Serifos",
        greeting: "Mara zählt die freien Betten, während am Kai die Abendfähre anlegt.",
        hint: "Bei Hitze hilft ein zweiter Brunnen. Tagsüber kann Nili Wasser bringen.",
        par: 27,
        spring: 0,
        guests: &[Bed, Meze, Bed, Bread, Meze, Bed, Offering, Pottery, Shade],
        goals: [Serve(Bed, 145), Route(Job::Welcome, 7), Water(19)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Der Hof der sieben Wünsche",
        district: "Kea",
        greeting: "Die Kinder haben sieben Wünsche auf bunte Tonscherben gemalt.",
        hint: "Erfülle sieben verschiedene Wünsche am selben Tag. Günstige Preise helfen.",
        par: 22,
        ferry: 20,
        walk: 5,
        goals: [Variety(7), GoodDay(83), Serve(Offering, 91)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Brunnen aus hellem Stein",
        district: "Kythira",
        greeting: "Das Becken am alten Tor ist leer. Die Insel braucht Wasser für heiße Tage.",
        hint: "Auf Stufe 4 liefert ein Brunnen 20 Krüge. Läden in der Nähe bekommen zuerst Wasser.",
        par: 23,
        coins: 76,
        spring: 0,
        weather: &[Heat, Heat, Wind, Feast, Clear],
        goals: [Upgrade(Well, 4), Serve(Pottery, 95), Serve(Offering, 111)],
        guests: &[
            Offering, Pottery, Bed, Pottery, Offering, Meze, Shade, Oil, Offering, Pottery,
        ],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Der Kranz der Schiffer",
        district: "Zakynthos",
        greeting: "Die Schiffer hängen einen Olivenkranz an den Kai. Ihre Boote bringen neue Bestellungen.",
        hint: "Mit einem Speicher hast du mehr Vorrat, ohne mehr Wasser zu verbrauchen.",
        par: 24,
        ferry: 21,
        guests: &[
            Oil, Meze, Oil, Offering, Shade, Bread, Bed, Meze, Oil, Pottery,
        ],
        goals: [Serve(Oil, 185), Serve(Meze, 103), FeastGuests(20)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Das Bündnis der Inselhäfen",
        district: "Korinth",
        difficulty: "Finale Griechenland",
        greeting: "Ein Brief aus Rom ist da. Am Tiber wollen sie Nilis Baupläne sehen.",
        hint: "Stelle zuerst die Versorgung sicher. Nutze dann die Einnahmen für den Ausbau.",
        par: 26,
        coins: 110,
        ferry: 23,
        favor_goal: 15,
        goals: [Variety(7), Serve(Offering, 162), FerryDay(23)],
        ..GREEK_TEMPLE
    },
];

const PORT_PLANS: &[crate::model::Kind] = &[
    House, Well, Bakery, Grove, Potter, Stoa, Taverna, Inn, Storehouse,
];
const WATER_PLANS: &[crate::model::Kind] = &[
    House, Well, Bakery, Grove, Potter, Taverna, Inn, Storehouse, Aqueduct, Baths,
];
const CITY_PLANS: &[crate::model::Kind] = &[
    House, Well, Bakery, Grove, Potter, Taverna, Inn, Storehouse, Aqueduct, Baths, Forum,
];
const ALL_ROMAN: &[crate::model::Kind] = &[
    House, Well, Bakery, Grove, Potter, Taverna, Inn, Storehouse, Aqueduct, Baths, Forum, Villa,
];

const ROME_PORT: Chapter = Chapter {
    region: Region::Rome,
    name: "",
    district: "",
    difficulty: "Mittel",
    greeting: "",
    hint: "",
    layout: super::Layout::legacy(6),
    coins: 74,
    spring: 1,
    ferry: 13,
    walk: 5,
    unlock: 0,
    building_set: Some(PORT_PLANS),
    par: 12,
    favor_goal: 7,
    starts: &[(0, House), (1, Well), (2, Grove), (7, Bakery)],
    locals: &[Bread, Pottery, Meze, Bread, Shade, Bread],
    guests: &[Bread, Bed, Meze, Shade, Pottery, Bread, Bed, Meze],
    weather: &[Cloud, Rain, Wind, Feast],
    favors: &[Cargo, Jugs, ShadeFavor],
    goals: [Earn(1), Renown(1), Help(1)],
};

const ROME_WATER: Chapter = Chapter {
    difficulty: "Knifflig",
    spring: 0,
    coins: 84,
    ferry: 16,
    building_set: Some(WATER_PLANS),
    par: 17,
    favor_goal: 9,
    locals: &[Bread, Pottery, Bath, Meze, Bath, Bread],
    guests: &[Bed, Bath, Meze, Pottery, Bed, Bath, Bread, Bath, Meze],
    weather: &[Heat, Rain, Cloud, Feast, Wind],
    ..ROME_PORT
};

const ROME_CITY: Chapter = Chapter {
    difficulty: "Anspruchsvoll",
    coins: 92,
    ferry: 19,
    building_set: Some(CITY_PLANS),
    par: 20,
    favor_goal: 11,
    locals: &[Bread, Trade, Pottery, Meze, Bath, Bread, Trade],
    guests: &[
        Trade, Bed, Bath, Meze, Bread, Pottery, Trade, Bed, Bath, Trade,
    ],
    ..ROME_WATER
};

const ROME_FINAL: Chapter = Chapter {
    coins: 112,
    ferry: 23,
    par: 25,
    favor_goal: 14,
    walk: 5,
    building_set: Some(ALL_ROMAN),
    starts: &[(0, House), (1, Well), (2, Grove), (3, Villa), (7, Bakery)],
    weather: &[Rain, Heat, Cloud, Feast, Heat, Wind],
    ..ROME_CITY
};

pub(super) const ROMAN: [Chapter; 24] = [
    Chapter {
        name: "Ein Schiff aus Korinth",
        district: "Ostia",
        greeting: "Flavia wartet am Tiber auf Nili. Am Kai steht schon der erste Ofen.",
        hint: "Viele Fährgäste wollen Brot. Plane einen Ofen in der Nähe des Kais.",
        par: 12,
        goals: [
            Serve(Bread, 68),
            Pair(Storehouse, Bakery, 1),
            RouteVariety(3),
        ],
        guests: &[Bread, Bed, Meze, Bread, Pottery, Shade, Bread, Bed],
        ..ROME_PORT
    },
    Chapter {
        name: "Die Ziegel am neuen Kai",
        district: "Portus",
        greeting: "Am sechseckigen Hafen füllen sich die Speicher. Die Besatzungen gehen an Land.",
        hint: "Baue den Speicher zwischen die Läden. So bekommen mehrere mehr Vorrat.",
        par: 14,
        ferry: 16,
        goals: [Serve(Pottery, 48), Serve(Meze, 54), MarketDay(70)],
        guests: &[Pottery, Meze, Bed, Pottery, Shade, Meze, Bread, Pottery],
        ..ROME_PORT
    },
    Chapter {
        name: "Gäste an der Via Appia",
        district: "Aricia",
        greeting: "Auf der Pflasterstraße klappern Sandalen. Flavia sucht Betten für die Reisenden.",
        hint: "Ein Gasthaus am Anleger erreicht viele Gäste. Mit genug Wasser und einem Ausbau stehen mehr Betten bereit.",
        par: 13,
        guests: &[Bed, Bed, Bread, Bed, Meze, Pottery, Shade, Bed],
        goals: [Serve(Bed, 84), Upgrade(Inn, 3), Route(Job::Welcome, 6)],
        ..ROME_PORT
    },
    Chapter {
        name: "Das erste Aquädukt",
        district: "Tibur",
        greeting: "Gaius zeigt Nili die alten Pfeiler im Tal. Hier soll bald wieder Wasser fließen.",
        hint: "Neben einem Brunnen liefert das Aquädukt vier zusätzliche Krüge.",
        par: 15,
        starts: &[(0, House), (2, Well), (3, Grove), (7, Bakery), (9, Taverna)],
        goals: [Pair(Aqueduct, Well, 1), Water(33), Serve(Bread, 81)],
        guests: &[Bread, Bath, Pottery, Bread, Meze, Bath, Bread, Bed],
        ..ROME_WATER
    },
    Chapter {
        name: "Das erste Bad",
        district: "Praeneste",
        greeting: "Über den neuen Mosaiken steigt Dampf auf. Livia öffnet das Bad.",
        hint: "Ein nahes Aquädukt bringt mehr Badeplätze. Es darf höchstens zwei Bauplätze entfernt sein.",
        par: 15,
        starts: &[
            (0, House),
            (2, Well),
            (3, Grove),
            (4, Aqueduct),
            (7, Bakery),
        ],
        goals: [Pair(Baths, Aqueduct, 1), Serve(Bath, 72), Upgrade(Baths, 3)],
        ..ROME_WATER
    },
    Chapter {
        name: "Ein Dach aus Wasser",
        district: "Veii",
        greeting: "Der Regen füllt die Krüge. Am Kai sammelt sich Treibgut.",
        hint: "Räumt Nili den Kai, laufen die Fährgäste zwei Plätze weiter.",
        par: 17,
        weather: &[Rain, Heat, Storm, Feast, Cloud],
        goals: [Route(Job::Quay, 5), Serve(Bath, 86), Water(39)],
        guests: &[Pottery, Bath, Bed, Pottery, Bath, Bread, Meze, Bath],
        ..ROME_WATER
    },
    Chapter {
        name: "Brot unter Arkaden",
        district: "Tusculum",
        greeting: "Livia hat Platz für einen zweiten Ofen gefunden. Die Nachbarn freuen sich schon.",
        hint: "Günstiges Brot in Wohnnähe hilft Familien mit kleinem Budget.",
        par: 18,
        locals: &[Bread, Bread, Bath, Pottery, Bread, Meze],
        guests: &[Bread, Meze, Bath, Bread, Bed, Pottery, Bread, Meze],
        goals: [Build(Bakery, 2), Serve(Bread, 124), Serve(Meze, 61)],
        ..ROME_WATER
    },
    Chapter {
        name: "Plätze beim Stadtfest",
        district: "Lanuvium",
        greeting: "Vor den Bädern stehen neue Bänke. Die Händler rechnen am Festtag mit vielen Gästen.",
        hint: "Am Festtag helfen volle Regale, kurze Wege und bezahlbare Preise.",
        par: 18,
        ferry: 18,
        goals: [FeastGuests(17), Serve(Bath, 100), GoodDay(82)],
        ..ROME_WATER
    },
    Chapter {
        name: "Der erste Marktplatz",
        district: "Alba Longa",
        greeting: "Flavia spannt ein rotes Tuch über den Markt. Die ersten Händler sind da.",
        hint: "Ein Forum hilft Fährgästen, die Läden in seiner Nähe zu erreichen.",
        par: 18,
        goals: [Build(Forum, 1), Serve(Trade, 96), Earn(1_140)],
        ..ROME_CITY
    },
    Chapter {
        name: "Mosaik am Tiber",
        district: "Fidenae",
        greeting: "Neben dem neuen Bad füllt sich der Markt. Ein Schiff mit Schalen legt an.",
        hint: "Stell das Forum in die Nähe der Töpferei. Dann kommen Fährgäste leichter hin.",
        par: 20,
        walk: 4,
        goals: [Serve(Bath, 100), Serve(Pottery, 81), Upgrade(Forum, 3)],
        guests: &[
            Pottery, Bath, Bed, Bath, Trade, Meze, Pottery, Bread, Pottery, Trade,
        ],
        ..ROME_CITY
    },
    Chapter {
        name: "Der Pachtvertrag",
        district: "Gabii",
        greeting: "Drei Werkstätten teilen sich das Wasser. Ihre Türen sollen zum Fest offen stehen.",
        hint: "Bei normalen Preisen verdienst du mehr pro Verkauf. Günstige Preise können sich mehr Gäste leisten.",
        par: 19,
        ferry: 18,
        goals: [Variety(6), MarketDay(106), Serve(Trade, 112)],
        ..ROME_CITY
    },
    Chapter {
        name: "Das Bad der Quellen",
        district: "Aquae Albulae",
        greeting: "Livia hat warme Quellen gefunden. Gaius möchte ein zweites Becken anschließen.",
        hint: "Fährgäste haben es vom Kai aus nah zum Bad. Ein zweites Bad im Ort hilft den Nachbarn.",
        par: 20,
        spring: 5,
        coins: 100,
        guests: &[Bath, Bed, Bath, Bath, Trade, Bread, Pottery, Meze],
        goals: [Build(Baths, 2), Serve(Bath, 172), Water(42)],
        ..ROME_CITY
    },
    Chapter {
        name: "Eine Villa am Fluss",
        district: "Lavinium",
        greeting: "Aus dem Hof der Villa klingt Musik. Die ersten Nachbarn ziehen ein.",
        hint: "In Villen wohnen mehr Menschen. Viele von ihnen können sich höhere Preise leisten.",
        par: 20,
        starts: ROME_PORT.starts,
        locals: &[Trade, Bath, Pottery, Meze, Trade, Bread, Bed, Trade],
        goals: [Upgrade(Villa, 3), Serve(Trade, 132), FerryDay(19)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Gassen der Händler",
        district: "Formiae",
        greeting: "Die Händler stellen große Tonkrüge vor das Forum. Bald öffnet ein zweiter Markt.",
        hint: "Baue ein Forum am Kai und eines bei den Wohnhäusern.",
        par: 21,
        walk: 4,
        goals: [Build(Forum, 2), Serve(Trade, 172), MarketDay(116)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Schwalben am Stadttor",
        district: "Anxur",
        greeting: "Schwalben nisten am Stadttor. Gaius zählt Reisende, die noch ein Bett suchen.",
        hint: "Thermen und Gasthäuser brauchen Wasser. Nili kann an heißen Tagen aushelfen.",
        par: 21,
        guests: &[Bed, Bath, Bed, Trade, Meze, Bed, Pottery, Bread, Bed],
        goals: [Serve(Bed, 171), Serve(Bath, 107), Route(Job::Water, 9)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Weg durch den Garten",
        district: "Cumae",
        greeting: "Am Tempelberg wächst ein Olivenhain. Die Familien treffen sich zum Essen und Baden.",
        hint: "Plane je Taverne zwei Laibe und zwei Portionen Oliven ein.",
        par: 22,
        locals: &[Meze, Bread, Bath, Trade, Meze, Pottery, Meze],
        guests: &[Meze, Bath, Bed, Trade, Bread, Meze, Pottery, Meze, Trade],
        goals: [Serve(Meze, 180), Pair(Taverna, Bakery, 1), FeastGuests(22)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Die Lastkähne",
        district: "Neapolis",
        greeting: "Lastkähne und Fähren liegen nebeneinander. Am Markt ist heute viel los.",
        hint: "Wähle einen Fährtag, wenn Gäste alle sechs Angebote erreichen können.",
        par: 22,
        ferry: 24,
        goals: [Variety(6), GoodDay(88), Serve(Pottery, 109)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Fenster zur Hafenmauer",
        district: "Puteoli",
        greeting: "Aus dem Gasthaus sieht Flavia die Frachter. Einige Reisende bleiben über Nacht.",
        hint: "Mit zwei Gasthäusern finden mehr Reisende ein Bett. Prüfe vor dem Ausbau, ob das Wasser reicht.",
        par: 23,
        guests: &[Bed, Bath, Trade, Bed, Bread, Meze, Bed, Pottery, Trade],
        goals: [Upgrade(Inn, 4), Serve(Bed, 202), Serve(Trade, 146)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Zwei Hallen im Regen",
        district: "Capua",
        greeting: "Familien warten unter den Arkaden. Nili und Gaius suchen Platz für den Markt.",
        hint: "Wähle an Regentagen den Nachbarschaftstag, wenn viele Höfe und Villen bewohnt sind.",
        par: 23,
        weather: &[Rain, Storm, Heat, Cloud, Feast, Wind],
        goals: [Serve(Bath, 151), Serve(Trade, 164), Upgrade(Villa, 4)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Am langen Mosaik",
        district: "Pompeii",
        greeting: "Ein Fischmosaik führt zum Markt. Die Töpfer wollen am anderen Ende der Gasse eröffnen.",
        hint: "Wasser und ein naher Speicher füllen die Auslage der Töpferei.",
        par: 24,
        guests: &[
            Pottery, Trade, Bread, Pottery, Bed, Meze, Bath, Pottery, Bread,
        ],
        goals: [Serve(Pottery, 221), Serve(Bread, 122), FeastGuests(22)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Steine für die Stadt",
        district: "Anio",
        greeting: "Gaius trägt den letzten Bogenstein zum Aquädukt. Die Thermen öffnen zum Fest.",
        hint: "Jede Ausbaustufe am Aquädukt bringt fünf Krüge. Ein Brunnen daneben bringt vier.",
        par: 25,
        spring: 0,
        weather: &[Heat, Heat, Wind, Rain, Feast],
        goals: [Water(51), Upgrade(Aqueduct, 4), Serve(Bath, 169)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Bogen von Trastevere",
        district: "Trastevere",
        greeting: "Hinter dem roten Tor duftet es nach Brot. Die Gastwirte bereiten das Essen vor.",
        hint: "Günstige Preise helfen Familien mit wenig Geld. Am Kai haben manche Gäste mehr Geld dabei.",
        par: 26,
        ferry: 24,
        guests: &[Meze, Trade, Meze, Bed, Bath, Pottery, Bread, Meze, Trade],
        goals: [MarketDay(125), Serve(Meze, 228), Renown(640)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Garten am Palatin",
        district: "Palatin",
        greeting: "Über der Gasse liegen die Villen. Unten füllen sich Markt und Bad.",
        hint: "Verteile die Läden so, dass Gäste vom Kai und aus den Villen sie gut erreichen.",
        par: 27,
        ferry: 26,
        goals: [Variety(6), Serve(Bed, 197), Earn(2_500)],
        locals: &[Bread, Trade, Pottery, Bed, Meze, Bath, Bread, Trade],
        guests: &[
            Trade, Bed, Bath, Bed, Meze, Bread, Pottery, Trade, Bed, Bath,
        ],
        ..ROME_FINAL
    },
    Chapter {
        name: "Ein Platz für ganz Rom",
        district: "Rom",
        difficulty: "Finale Rom",
        greeting: "Am Forum wehen die Fahnen von Ostia und Korinth. Nili und Flavia decken die Tische.",
        hint: "Sorge zuerst für genug Waren. Schaffe dann mehr Wohnraum und eine bessere Wasserversorgung.",
        par: 30,
        ferry: 27,
        coins: 132,
        favor_goal: 18,
        goals: [FeastGuests(25), Serve(Trade, 232), FerryDay(25)],
        ..ROME_FINAL
    },
];

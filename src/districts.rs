use super::{Chapter, Goal::*, Region};
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
    columns: 6,
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
        greeting: "Der Kornschreiber bündelt die Lieferungen von drei Anlegestellen. Er zeigt Nili einen leeren Speicherplatz zwischen den Öfen und der Straße.",
        hint: "Der Speicher gibt Läden bis zu zwei Plätze entfernt Zusatzvorrat. Beim Ausbau wächst sein Zuschlag.",
        par: 15,
        ferry: 17,
        starts: &[(1, House), (2, Well), (7, Bakery), (8, Taverna)],
        goals: [Pair(Storehouse, Bakery, 1), Serve(Bread, 66), Earn(640)],
        ..GREEK_TRADE
    },
    Chapter {
        name: "Die erste Ölpresse",
        district: "Aigina",
        greeting: "Der erste Korb Oliven liegt auf dem Kai. Eleni möchte daraus goldgelbes Öl pressen und bittet Nili um einen Platz direkt neben dem Hain.",
        hint: "Die Ölpresse verbraucht pro Stufe bis zu eine zusätzliche Olivenportion. Ein Hain direkt daneben trägt zum Tagesertrag bei.",
        par: 17,
        goals: [Pair(Press, Grove, 1), Serve(Oil, 100), Upgrade(Grove, 3)],
        ..GREEK_OIL
    },
    Chapter {
        name: "Duftender Markt",
        district: "Chios",
        greeting: "Aus den Töpfen am Markt steigt der Duft von Kräutern und Brot. Wenn die Händler anlegen, soll es sowohl Meze als auch Öl für die Rückfahrt geben.",
        hint: "Tavernen werden zuerst mit Oliven versorgt. Mit einem ausgebauten Hain bleibt genug für die Presse.",
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
        greeting: "Im Heiligtum hängen die ersten bemalten Schalen. Doros hat den Ton vorbereitet. Nili soll Werkstatt und Vorhof so setzen, dass am Festtag jeder eine Gabe findet.",
        hint: "Das Heiligtum verwendet zwei Schalen aus dem nächsten Vorrat. Plane auch Keramik für den eigenen Verkaufsstand ein.",
        par: 18,
        starts: &[(1, House), (2, Well), (3, Grove), (4, Potter), (7, Bakery)],
        goals: [
            Pair(Shrine, Potter, 1),
            Serve(Offering, 80),
            Serve(Pottery, 43),
        ],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Rast im Pinienwind",
        district: "Skopelos",
        greeting: "Der Nordwind hält manche Boote im Schutz der Felsen. Die Wächter öffnen währenddessen die Stoa. Wer den Weg findet, soll dort Wasser, Schatten und eine Mahlzeit bekommen.",
        hint: "Die Wasserfahrt bringt vier Krüge, belegt aber zwei Plätze auf der Fähre. Fährtage haben den gegenläufigen Effekt.",
        par: 18,
        spring: 0,
        weather: &[Wind, Cloud, Storm, Heat, Rain, Feast],
        locals: &[Shade, Bread, Shade, Meze, Pottery, Shade],
        guests: &[
            Shade, Meze, Bed, Shade, Oil, Offering, Bread, Shade, Pottery,
        ],
        goals: [Serve(Shade, 111), Serve(Meze, 54), Help(11)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Meer aus Tonlaternen",
        district: "Syros",
        greeting: "Vor jedem Tor hängt ein anderer Tonstern. Die Fährleute schlagen ihren Ankunftsabend als Festtag vor. Nili soll die vielen Wünsche an einem Abend zusammenbringen.",
        hint: "Ein Fährtag lohnt sich erst, wenn Vorräte und Läden für die zusätzlichen Gäste reichen. Schau dir die Wünsche im Hafenbuch an.",
        par: 20,
        ferry: 19,
        goals: [FeastGuests(18), Upgrade(Potter, 4), Serve(Offering, 93)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Zwei Schreiber am Kai",
        district: "Tinos",
        greeting: "Zwei Schreiber notieren dieselben Lieferungen auf verschiedenen Tafeln. Kleio schlägt vor, Brot und Öl aus benachbarten Werkstätten zu zählen und Gästen eine gemeinsame Karte zu geben.",
        hint: "Ein kurzer Weg hilft günstigen Läden. Geschäfte auf der Promenade erreichen mehr Menschen vom Kai.",
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
        greeting: "Sobald die Sonne hinter den Felsen sinkt, reichen die großen Bänke kaum. Mara legt Gästelisten auf die Tische. Die Herbergen brauchen Hilfe aus der Gasse.",
        hint: "Bei Regen kommt Wasser hinzu. In heißen Phasen versorgst du Werkstätten mit einer Wasserfahrt oder einem zweiten Brunnen.",
        par: 27,
        spring: 0,
        guests: &[Bed, Meze, Bed, Bread, Meze, Bed, Offering, Pottery, Shade],
        goals: [Serve(Bed, 145), Serve(Meze, 77), Water(19)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Der Hof der sieben Wünsche",
        district: "Kea",
        greeting: "In der Marktnische liegt je ein Gegenstand aus jeder Werkstatt. Die Kinder der Insel haben alle sieben Wünsche auf Keramikscherben gezeichnet.",
        hint: "Alle sieben Wünsche müssen an einem einzigen Tag erfüllt werden. Halte Angebote günstig, wenn verschiedenste Gäste kommen.",
        par: 22,
        ferry: 20,
        walk: 5,
        goals: [Variety(7), Earn(1_340), Serve(Offering, 91)],
        ..GREEK_TEMPLE
    },
    Chapter {
        name: "Brunnen aus hellem Stein",
        district: "Kythira",
        greeting: "Das Becken am alten Tor ist leer. Der Rat setzt drei kalte Krüge auf die Treppe und bittet Nili, den Wasserfluss für die heißen Wochen zu sichern.",
        hint: "Ein Brunnen der Stufe 4 liefert zwanzig Krüge. Läden näher an einer Wasserstelle werden zuerst gefüllt.",
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
        greeting: "Die Schiffer knüpfen einen Kranz aus Olivenzweigen. Jedes Boot bringt andere Bestellungen und im alten Hafen muss alles rechtzeitig bereitstehen.",
        hint: "Fährtage helfen bei großen Aufträgen. Ein Speicher erhöht die Vorräte, ohne den Wasserverbrauch der Läden anzutreiben.",
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
        greeting: "Aus zwölf Buchten, Marktplätzen und Werften ist ein gemeinsamer Hafenrat geworden. Ein Schiff hat ein römisches Siegel mitgebracht. Die Schiffer wollen Nilis Baupläne auch am Tiber lesen.",
        hint: "Sorge zuerst für alle Warenketten. Lege Festtage und wichtige Lieferungen auf verschiedene Bauphasen, damit genug Geld für den Ausbau bleibt.",
        par: 26,
        coins: 110,
        ferry: 23,
        favor_goal: 15,
        goals: [Variety(7), Serve(Offering, 162), Earn(2_050)],
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
    columns: 6,
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
        greeting: "Nach der Überfahrt zieht Nili das Notizbuch am Tiber aus dem Gepäck. Flavia hat rote Dachziegel und einen neuen Ofen bereitgestellt. Am Kai riecht es nach Korn und Zypressen.",
        hint: "Die Preise funktionieren wie in Griechenland. Achte zuerst auf die vielen Brotwünsche der Fährgäste.",
        par: 12,
        goals: [Serve(Bread, 68), Pair(Storehouse, Bakery, 1), Renown(125)],
        guests: &[Bread, Bed, Meze, Bread, Pottery, Shade, Bread, Bed],
        ..ROME_PORT
    },
    Chapter {
        name: "Die Ziegel am neuen Kai",
        district: "Portus",
        greeting: "Die Speicher am sechseckigen Hafen werden voll. Man verkauft Schalen, Vorräte und warme Mahlzeiten an jede Besatzung, die hier festmacht.",
        hint: "Speicher liefern auch an Töpfereien und Tavernen. Stell ihn so, dass mehrere Läden in Reichweite liegen.",
        par: 14,
        ferry: 16,
        goals: [Serve(Pottery, 48), Serve(Meze, 54), MarketDay(70)],
        guests: &[Pottery, Meze, Bed, Pottery, Shade, Meze, Bread, Pottery],
        ..ROME_PORT
    },
    Chapter {
        name: "Gäste an der Via Appia",
        district: "Aricia",
        greeting: "Sandalen klappern auf der alten Pflasterstraße. Flavia bringt Reisenden eine Lampe, doch die Herberge braucht mehr Betten und eine Straße voller Anlaufstellen.",
        hint: "Baue das Gasthaus nahe am rechten Anleger. Mit mehr Wasser und einer höheren Stufe wächst sein Vorrat.",
        par: 13,
        guests: &[Bed, Bed, Bread, Bed, Meze, Pottery, Shade, Bed],
        goals: [Serve(Bed, 84), Upgrade(Inn, 3), Earn(750)],
        ..ROME_PORT
    },
    Chapter {
        name: "Das erste Aquädukt",
        district: "Tibur",
        greeting: "Über dem Tal stehen drei alte Pfeiler. Der Steinmetz Gaius zeigt Nili die Falllinie fürs Wasser. Neben dem Brunnen ist Platz für die erste Arkade.",
        hint: "Aquädukte versorgen die ganze Gasse. Ein Brunnen direkt daneben erhöht ihren Ertrag um vier Krüge.",
        par: 15,
        starts: &[(0, House), (2, Well), (3, Grove), (7, Bakery), (9, Taverna)],
        goals: [Pair(Aqueduct, Well, 1), Water(33), Serve(Bread, 81)],
        guests: &[Bread, Bath, Pottery, Bread, Meze, Bath, Bread, Bed],
        ..ROME_WATER
    },
    Chapter {
        name: "Das erste Bad",
        district: "Praeneste",
        greeting: "Livia nimmt den Deckel vom Becken. Warmer Dampf steigt aus den Mosaiken. Zur Eröffnung sollen auch die Töpfer und Bäcker mit ihren Körben kommen.",
        hint: "Thermen brauchen mehr Wasser als andere Läden. Ein Aquädukt höchstens zwei Plätze entfernt erhöht die Badeplätze.",
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
        greeting: "Regen tropft von den Mauerbögen. Die Krüge sind voll und die Fähre kommt mit weniger Gästen. Nili soll den Wechsel aus Regen und Hitze für die Gasse einplanen.",
        hint: "Bei Regen wachsen die Wasserreserven, aber der Anleger wird ruhiger. Wähle den Hafenplan passend zum Tageswetter.",
        par: 17,
        weather: &[Rain, Heat, Storm, Feast, Cloud],
        goals: [Serve(Pottery, 66), Serve(Bath, 86), Water(39)],
        guests: &[Pottery, Bath, Bed, Pottery, Bath, Bread, Meze, Bath],
        ..ROME_WATER
    },
    Chapter {
        name: "Brot unter Arkaden",
        district: "Tusculum",
        greeting: "Livia hat auf der Straße ein Tuch ausgerollt und verschiedene Pachtpläne gezeichnet. Ein zweiter Ofen könnte die Läden tagsüber deutlich besser versorgen.",
        hint: "Zwei Backstuben teilen die Nachfrage. Ein günstiger Ofen in Wohnnähe erreicht Familien mit kleinem Budget.",
        par: 18,
        locals: &[Bread, Bread, Bath, Pottery, Bread, Meze],
        guests: &[Bread, Meze, Bath, Bread, Bed, Pottery, Bread, Meze],
        goals: [Build(Bakery, 2), Serve(Bread, 124), Serve(Meze, 61)],
        ..ROME_WATER
    },
    Chapter {
        name: "Plätze beim Stadtfest",
        district: "Lanuvium",
        greeting: "Vor den Bädern werden Bänke gestellt. Die Händler erwarten einen vollen Festtag und haben den Stadtschreiber geschickt, um die Vorräte zu zählen.",
        hint: "Prüfe an einem Festtag alle Wünsche. Zusätzliche Plätze helfen nur, wenn die Gäste sie erreichen und bezahlen können.",
        par: 18,
        ferry: 18,
        goals: [FeastGuests(17), Serve(Bath, 100), MarketDay(92)],
        ..ROME_WATER
    },
    Chapter {
        name: "Der erste Marktplatz",
        district: "Alba Longa",
        greeting: "Flavia spannt ein rotes Tuch über die neuen Stände. Ein Forum verbindet Hafen und Handwerk. Die Händler zeichnen den kürzesten Weg auf den Boden.",
        hint: "Ein Forum erhöht die Reichweite der Fährgäste bei Läden bis zu zwei Plätze entfernt.",
        par: 18,
        goals: [Build(Forum, 1), Serve(Trade, 96), Earn(1_140)],
        ..ROME_CITY
    },
    Chapter {
        name: "Mosaik am Tiber",
        district: "Fidenae",
        greeting: "Kinder legen ein blaues Mosaik in den Boden der Thermen. Nebenan füllt sich der Markt. Auf dem Fluss fährt heute ein Schiff mit gebrannten Schalen ein.",
        hint: "Das Forum kann dazu beitragen, dass der vom Kai weiter entfernte Tonladen erreichbar wird.",
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
        greeting: "Drei Werkstätten teilen sich das Wasser aus dem neuen Speicherhof. Die Pächter wünschen einen Festtag, an dem Gäste alle Angebote nutzen können.",
        hint: "Mit dem üblichen Preis verdienst du mehr. Manche Nachbarn brauchen dennoch ein günstiges Angebot.",
        par: 19,
        ferry: 18,
        goals: [Variety(6), MarketDay(106), Serve(Trade, 112)],
        ..ROME_CITY
    },
    Chapter {
        name: "Das Bad der Quellen",
        district: "Aquae Albulae",
        greeting: "Die mineralischen Quellen sind warm. Livia stellt Schalen an den Rand der Bäder. Gaius soll ein zweites Becken anschließen, ohne die Backstuben auf dem Hügel zu vergessen.",
        hint: "Nah gelegene Wasserstellen versorgen zuerst. Zwei Thermen sind sinnvoll, wenn du die Fährgäste am Kai und die Nachbarn im Ort erreichst.",
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
        greeting: "Aus dem Atrium der Villa ist Musik zu hören. Flavia bringt die ersten Bewohner vorbei. Ein paar von ihnen haben mehr für Markt und Thermen eingeplant.",
        hint: "Eine Villa zieht mehr Nachbarn an als ein Hof. Viele können sich auch feine Angebote leisten.",
        par: 20,
        starts: ROME_PORT.starts,
        locals: &[Trade, Bath, Pottery, Meze, Trade, Bread, Bed, Trade],
        goals: [Upgrade(Villa, 3), Serve(Trade, 132), Renown(410)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Gassen der Händler",
        district: "Formiae",
        greeting: "Dolien und kleine Schalen stehen vor dem Forum. Die Händler wollen zwei gute Marktzugänge, damit der Hafen während der warmen Tage nicht auseinanderläuft.",
        hint: "Ein Forum an einem Rand kann auch Läden im Hinterland erreichen lassen. Das zweite gehört zu den Wohnhäusern.",
        par: 21,
        walk: 4,
        goals: [Build(Forum, 2), Serve(Trade, 172), MarketDay(116)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Schwalben am Stadttor",
        district: "Anxur",
        greeting: "Schwalben ziehen ins Stadttor ein. Gaius erzählt, dass in diesem Frühjahr mehr Gäste in der Stadt schlafen als sonst. Nili zählt Krüge und freie Betten.",
        hint: "Der Wasservorrat muss für Gasthäuser und Thermen reichen. Eine Wasserfahrt hilft auf sehr heißen Tagen.",
        par: 21,
        guests: &[Bed, Bath, Bed, Trade, Meze, Bed, Pottery, Bread, Bed],
        goals: [Serve(Bed, 171), Serve(Bath, 107), Help(13)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Weg durch den Garten",
        district: "Cumae",
        greeting: "Ein Olivenhain wächst an der Straße zum Tempelberg. Die Familien kommen zuerst zum Essen und später zum Bad. Am Abend ziehen die Händler durch den Garten zum Kai.",
        hint: "Olivenhöfe liefern Vorrat für mehrere Tavernen. Jede Taverne nimmt zwei Brote aus dem nächstgelegenen Ofen.",
        par: 22,
        locals: &[Meze, Bread, Bath, Trade, Meze, Pottery, Meze],
        guests: &[Meze, Bath, Bed, Trade, Bread, Meze, Pottery, Meze, Trade],
        goals: [Serve(Meze, 180), Pair(Taverna, Bakery, 1), FeastGuests(22)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Die Lastkähne",
        district: "Neapolis",
        greeting: "Mehr Lastkähne als kleine Segel liegen auf dem Wasser. Den Markt erreichen viele Körbe, doch ein volles Schiff bringt auch Gäste mit unterschiedlichen Wünschen.",
        hint: "Nutze den Fährtag, sobald alle sechs Wünsche erreichbar sind. Plane für den Regen weniger Fährgäste ein.",
        par: 22,
        ferry: 24,
        goals: [Variety(6), Earn(1_950), Serve(Pottery, 109)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Fenster zur Hafenmauer",
        district: "Puteoli",
        greeting: "Vom großen Gasthaus sieht man die Mauer, an der die Frachter anlegen. Flavia fragt, wie viele der Reisenden auch abends noch einen Platz bekommen.",
        hint: "Die Stufe 4 lohnt sich dort, wo die Nachfrage dauerhaft hoch bleibt. Das Hafenbuch zeigt den aktuellen Engpass.",
        par: 23,
        guests: &[Bed, Bath, Trade, Bed, Bread, Meze, Bed, Pottery, Trade],
        goals: [Upgrade(Inn, 4), Serve(Bed, 202), Serve(Trade, 146)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Zwei Hallen im Regen",
        district: "Capua",
        greeting: "Auf beiden Seiten der Straße warten Familien unter den Bögen. Für die Therme sind die Krüge voll, für den Markt fehlt ein Dach. Nili begleitet Gaius zu den neuen Bauplätzen.",
        hint: "Bei Regen kommen weniger Menschen von außen. Ein Nachbarschaftstag lädt die Bewohner deiner Höfe und Villen ein.",
        par: 23,
        weather: &[Rain, Storm, Heat, Cloud, Feast, Wind],
        goals: [Serve(Bath, 151), Serve(Trade, 164), Upgrade(Villa, 4)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Am langen Mosaik",
        district: "Pompeii",
        greeting: "Auf dem Pflaster zieht sich ein Fischmosaik bis zum Markt. Die Bäcker stellen ihren Brotstand an den Anfang und die Töpfer wollen die zweite Hälfte der Gasse erschließen.",
        hint: "Die Töpferei verkauft mehr mit kurzem Weg und Wasser. Speicher stützen die Vorräte, wenn Material knapp wird.",
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
        greeting: "Gaius trägt den letzten Bogenstein durch die Gasse. Die Thermen sollen am Festtag reichlich Wasser haben. Aus beiden Villen schauen schon Bewohner heraus.",
        hint: "Aquädukte wachsen beim Ausbau um fünf Krüge pro Stufe. Ein Brunnen daneben bringt zusätzlich vier.",
        par: 25,
        spring: 0,
        weather: &[Heat, Heat, Wind, Rain, Feast],
        goals: [Water(51), Upgrade(Aqueduct, 4), Serve(Bath, 169)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Bogen von Trastevere",
        district: "Trastevere",
        greeting: "Hinter dem roten Tor werden Schalen und Brot verteilt. Die Gastwirte bieten feine Mahlzeiten an und suchen jemanden, der auch die Familienpreise im Blick behält.",
        hint: "Ein höherer Preis lohnt sich bei wohlhabenden Kunden. Günstige Läden versorgen mehr Menschen und gewinnen schneller Ansehen.",
        par: 26,
        ferry: 24,
        guests: &[Meze, Trade, Meze, Bed, Bath, Pottery, Bread, Meze, Trade],
        goals: [MarketDay(125), Serve(Meze, 228), Renown(640)],
        ..ROME_FINAL
    },
    Chapter {
        name: "Der Garten am Palatin",
        district: "Palatin",
        greeting: "Die Villen liegen über der Straße. In der großen Halle hat jemand Nilis alte Baupläne aufgehängt. Jetzt soll darunter an einem vollen Markt- und Badetag Platz für alle sein.",
        hint: "In großen Vierteln bilden Villen die Nachfrage im Hinterland. Für die Gäste von rechts müssen dieselben Angebote auch vom Kai aus erreichbar sein.",
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
        greeting: "Am Forum hängen die Fahnen von Ostia und Korinth. Nili und Flavia decken die Ecktische. Vom Markt bis zur Therme öffnen die Türen für die ganze Stadt.",
        hint: "Versorge die Stadt in Etappen. Erweitere Wasser und Wohnraum, sobald Bäder, Forum, Herberge und Handwerk auf einem verlässlichen Weg liegen.",
        par: 30,
        ferry: 27,
        coins: 132,
        favor_goal: 18,
        goals: [FeastGuests(25), Serve(Trade, 232), Earn(2_850)],
        ..ROME_FINAL
    },
];

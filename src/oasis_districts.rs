use super::{Chapter, Goal::*, REGION_SIZE, Region, Shift};
use crate::{
    errands::Job,
    layout::Layout,
    model::{
        Favor::{Cargo, Jugs, Shade as ShadeFavor},
        Kind::{self, *},
        Service::{Bed, Dates, Glass, Incense, Shade, Spices, Textiles},
        Weather::{Clear, Cloud, Feast, Heat, Rain, Storm, Wind},
    },
};

const GROVE: &[Kind] = &[OasisHome, Cistern, DateGarden];
const SOUK: &[Kind] = &[OasisHome, Cistern, DateGarden, SpiceBazaar, Caravanserai];
const WATER: &[Kind] = &[
    OasisHome,
    Cistern,
    DateGarden,
    SpiceBazaar,
    Caravanserai,
    Qanat,
];
const GLASS: &[Kind] = &[
    OasisHome,
    Cistern,
    DateGarden,
    SpiceBazaar,
    Caravanserai,
    Qanat,
    Glassworks,
];
const PERFUME: &[Kind] = &[
    OasisHome,
    Cistern,
    DateGarden,
    SpiceBazaar,
    Caravanserai,
    Qanat,
    Glassworks,
    Perfumer,
];
const GARDENS: &[Kind] = &[
    OasisHome,
    Cistern,
    DateGarden,
    SpiceBazaar,
    Caravanserai,
    Qanat,
    Glassworks,
    Perfumer,
    MoonGarden,
];
const PALACE: &[Kind] = &[
    OasisHome,
    Cistern,
    DateGarden,
    SpiceBazaar,
    Caravanserai,
    Qanat,
    Glassworks,
    Perfumer,
    MoonGarden,
    CarpetHall,
];

const FIRST: Chapter = Chapter {
    region: Region::Oasis,
    shift: Shift::Day,
    name: "",
    district: "",
    difficulty: "Sanft",
    greeting: "",
    hint: "Baue den Dattelhain nah an der Zisterne. Der Tagesplan zeigt, wie viel Wasser und wie viele Gäste du erwarten kannst.",
    layout: Layout::legacy(4),
    coins: 58,
    spring: 5,
    ferry: 7,
    walk: 5,
    unlock: 0,
    building_set: Some(GROVE),
    par: 7,
    favor_goal: 3,
    starts: &[(1, OasisHome), (2, Cistern)],
    locals: &[Dates],
    guests: &[Dates],
    weather: &[Clear, Heat, Cloud, Feast, Wind],
    favors: &[Cargo, Jugs, Cargo],
    goals: [Build(DateGarden, 1), Serve(Dates, 30), Renown(32)],
};

const MARKET: Chapter = Chapter {
    difficulty: "Leicht",
    building_set: Some(SOUK),
    starts: &[(1, OasisHome), (2, Cistern), (7, DateGarden)],
    coins: 77,
    ferry: 11,
    par: 10,
    favor_goal: 4,
    locals: &[Dates, Spices, Dates, Bed],
    guests: &[Dates, Spices, Bed, Spices, Dates],
    weather: &[Clear, Heat, Wind, Cloud, Feast],
    favors: &[Cargo, Jugs, ShadeFavor],
    ..FIRST
};

const CARAVAN: Chapter = Chapter {
    difficulty: "Mittel",
    layout: Layout::legacy(5),
    building_set: Some(WATER),
    starts: &[
        (1, OasisHome),
        (2, Cistern),
        (7, DateGarden),
        (8, Caravanserai),
        (3, SpiceBazaar),
    ],
    coins: 88,
    ferry: 13,
    par: 13,
    favor_goal: 5,
    locals: &[Dates, Spices, Bed, Dates, Spices],
    guests: &[Dates, Bed, Spices, Spices, Dates, Bed, Dates],
    ..MARKET
};

const GLASS_TOWN: Chapter = Chapter {
    building_set: Some(GLASS),
    starts: &[
        (1, OasisHome),
        (2, Cistern),
        (7, DateGarden),
        (8, Caravanserai),
        (3, SpiceBazaar),
        (9, Qanat),
    ],
    coins: 107,
    ferry: 16,
    par: 15,
    favor_goal: 6,
    locals: &[Dates, Spices, Glass, Bed, Dates, Glass],
    guests: &[Dates, Glass, Spices, Bed, Glass, Dates, Spices, Bed],
    weather: &[Clear, Wind, Heat, Feast, Cloud, Rain],
    ..CARAVAN
};

const SPICE_TOWN: Chapter = Chapter {
    layout: Layout::legacy(6),
    difficulty: "Anspruchsvoll",
    building_set: Some(PERFUME),
    starts: &[
        (1, OasisHome),
        (2, Cistern),
        (7, DateGarden),
        (8, Caravanserai),
        (3, SpiceBazaar),
        (9, Qanat),
        (4, Glassworks),
    ],
    coins: 123,
    ferry: 19,
    par: 18,
    favor_goal: 8,
    guests: &[
        Spices, Incense, Glass, Bed, Dates, Spices, Incense, Glass, Dates,
    ],
    locals: &[Dates, Spices, Bed, Incense, Glass, Spices, Dates],
    weather: &[Clear, Heat, Wind, Cloud, Feast, Rain, Storm],
    ..GLASS_TOWN
};

const PALACE_TOWN: Chapter = Chapter {
    building_set: Some(PALACE),
    coins: 143,
    ferry: 23,
    par: 22,
    favor_goal: 9,
    locals: &[Dates, Spices, Glass, Incense, Textiles, Shade, Bed],
    guests: &[
        Glass, Incense, Dates, Shade, Textiles, Bed, Spices, Glass, Shade, Textiles,
    ],
    weather: &[Wind, Clear, Feast, Heat, Rain, Cloud, Storm],
    ..SPICE_TOWN
};

pub(super) const CHAPTERS: [Chapter; REGION_SIZE] = [
    Chapter {
        name: "Die ersten Datteln",
        district: "Ain Nura",
        greeting: "Nura wartet mit einer Schale Datteln vor den Lehmhöfen. Am Kanal legen die ersten Reisenden an.",
        ..FIRST
    },
    Chapter {
        name: "Die rote Treppe",
        district: "Sahl Roshan",
        greeting: "Yasmin zeigt dir die Treppe zur Zisterne. Zwischen den Höfen tragen die ersten Palmen Früchte.",
        hint: "Ein zweiter Hof bringt mehr Familien in die Gasse. Die Zisterne versorgt die Dattelgärten.",
        goals: [Pair(DateGarden, Cistern, 1), Residents(6), Serve(Dates, 39)],
        ..FIRST
    },
    Chapter {
        name: "Der Duft von Kardamom",
        district: "Amarat",
        greeting: "Du riechst den Basar schon auf dem Wasser. Samir möchte neue Gewürze auslegen.",
        hint: "Passe das Angebot am Gewürzbasar an die Gäste an. Lieblingsangebote bringen mehr Einnahmen.",
        goals: [Build(SpiceBazaar, 1), Serve(Spices, 29), Specials(6)],
        ..MARKET
    },
    Chapter {
        name: "Ein Hof für Reisende",
        district: "Qasr Safra",
        greeting: "Ilyas deckt den Hof zu. Nach der Überfahrt suchen viele Gäste einen schattigen Schlafplatz.",
        hint: "Die Karawanserei braucht Wasser. Im Tagesplan kannst du die Nachfrage nach Betten sehen.",
        goals: [Build(Caravanserai, 1), Serve(Bed, 32), GoodDay(81)],
        ..MARKET
    },
    Chapter {
        name: "Aufbruch zur Nachbaroase",
        district: "Bir Talin",
        greeting: "Die Karawanenführerin Dalia wartet vor dem Tor. Vor dem Aufbruch braucht sie Datteln und Wasser.",
        hint: "Geh mit Nili zur Karawanserei. Jede Karawane braucht zwei Datteln aus dem Umkreis und drei freie Wasserkrüge.",
        building_set: Some(SOUK),
        goals: [Caravans(4), Serve(Dates, 51), Route(Job::Water, 3)],
        ..CARAVAN
    },
    Chapter {
        name: "Wasser unter dem Sand",
        district: "Wadi Mehr",
        greeting: "Bahir kennt den Verlauf des alten Wasserstollens. Beim Hof ist Platz für einen neuen Schacht.",
        hint: "Ein Qanat führt Wasser aus dem Untergrund zur Oase. In seiner Nähe tragen Dattelpalmen mehr Früchte.",
        goals: [Pair(Qanat, DateGarden, 1), Water(28), MarketDay(76)],
        ..CARAVAN
    },
    Chapter {
        name: "Der lange Markttisch",
        district: "Darsin",
        greeting: "Zahra breitet die Festdecken aus. Die Nachbarn bringen Schalen vom anderen Ufer.",
        hint: "Beim Fest kommen mehr Menschen. Plane Vorräte und Angebote für mehrere Altersgruppen ein.",
        goals: [FeastGuests(18), Specials(19), Residents(9)],
        ..CARAVAN
    },
    Chapter {
        name: "Die blaue Nacht",
        district: "Ain Azraq",
        greeting: "Schiffe gleiten im Dunkeln zwischen den Lampen. Datteln und Gewürze bleiben bis zum Morgen im Verkauf.",
        hint: "In dieser Nacht zählen Gewürze und Datteln stärker als Betten. Die Karawane zieht auch nach Mitternacht los.",
        guests: &[Spices, Dates, Spices, Bed, Dates, Spices, Dates],
        locals: &[Dates, Spices, Dates, Bed, Spices],
        goals: [Serve(Spices, 58), Caravans(5), MarketDay(84)],
        ..CARAVAN
    },
    Chapter {
        name: "Gläser am Kanal",
        district: "Khusran",
        greeting: "Narim kühlt die ersten blauen Flaschen auf dem Sims ab. Vor dem Laden füllen sich die Becken.",
        hint: "Die Glasbläserei braucht Krüge zum Kühlen. Ein Werkstattausbau bringt neue Angebote.",
        goals: [Build(Glassworks, 1), Serve(Glass, 54), Route(Job::Water, 5)],
        ..GLASS_TOWN
    },
    Chapter {
        name: "Schattengassen",
        district: "Dar Nakhla",
        greeting: "Mina zählt die neuen Höfe. Die Nachbarn wollen ihre Einkäufe zu Fuß erledigen.",
        hint: "Verteile Wohnhäuser und Läden. Nahe Angebote verkürzen die Wege und verbessern die Stimmung.",
        goals: [Residents(13), GoodDay(86), Variety(4)],
        ..GLASS_TOWN
    },
    Chapter {
        name: "Über den Hügel",
        district: "Sahl Rumi",
        greeting: "Die Karawane probt den Aufbruch vor dem heißen Wind. Am Kanal gibt es mehr Platz für Lieferungen.",
        hint: "Halte Datteln für die Karawane zurück. Im Sturm bleibt sie in der Oase.",
        goals: [CaravanIncome(137), Upgrade(DateGarden, 4), Serve(Glass, 75)],
        ..GLASS_TOWN
    },
    Chapter {
        name: "Das Fest der hundert Gläser",
        district: "Madinat Nur",
        greeting: "Unter den Bögen stehen Reihen aus blauen Gläsern. Am Festtag warten Händler und Familien auf die Öffnung.",
        hint: "Halte am Festtag mehrere Läden und genug Plätze für die Reisenden bereit.",
        goals: [MarketDay(125), FeastGuests(23), Specials(30)],
        ..GLASS_TOWN
    },
    Chapter {
        name: "Die Gewürzstraße",
        district: "Qasr Alwan",
        greeting: "Soraya bringt duftende Harze zum Basar. Hinter den Toren beginnt die Gewürzstraße.",
        hint: "Ein Qanat ab Stufe 2 in Reichweite und fünf freie Krüge ermöglichen die Fernkarawane. Sie braucht Datteln und Gewürze.",
        goals: [SpiceRoutes(5), Upgrade(Qanat, 2), Serve(Incense, 47)],
        ..SPICE_TOWN
    },
    Chapter {
        name: "Höfe aus Sandstein",
        district: "Bir Sadaf",
        greeting: "Das neue Viertel wächst um die Wasserbecken. Oskan hängt duftende Bündel vor seine Werkstatt.",
        hint: "Verteile Duftwaren und Glas zwischen den Wohnhöfen. Kurze Wege helfen Stammgästen.",
        goals: [Build(Perfumer, 2), Residents(15), GoodDay(85)],
        ..SPICE_TOWN
    },
    Chapter {
        name: "Jasmin bei Mondlicht",
        district: "Raml Yasmin",
        greeting: "In den Gärten öffnen sich die ersten Blüten. Die Nachtgäste folgen den Lichtern bis zum Wasser.",
        hint: "Mondgärten haben nachts vier zusätzliche Plätze. Ein naher Dattelhain bringt weitere Plätze.",
        building_set: Some(GARDENS),
        guests: &[
            Shade, Incense, Glass, Spices, Shade, Bed, Incense, Shade, Glass,
        ],
        locals: &[Shade, Dates, Incense, Glass, Spices, Shade],
        goals: [
            Pair(MoonGarden, DateGarden, 1),
            Serve(Glass, 54),
            SpiceRoutes(6),
        ],
        ..SPICE_TOWN
    },
    Chapter {
        name: "Das Dach der Glasmacher",
        district: "Hajar Lulu",
        greeting: "Bahir hat eine größere Feuerstelle gebaut. Das Glas soll bis zum Festtag kühl werden.",
        hint: "Baue die Glasbläserei aus und halte Wasser übrig, wenn der Ofen heiß läuft.",
        building_set: Some(GARDENS),
        goals: [Upgrade(Glassworks, 4), Serve(Glass, 111), Water(46)],
        ..SPICE_TOWN
    },
    Chapter {
        name: "Gewebte Geschichten",
        district: "Ribat Sada",
        greeting: "Rima spannt rote und blaue Fäden. Beim Gewürzbasar ist Platz für ihre Teppichhalle.",
        hint: "Mit einem Gewürzbasar in der Nähe hat die Teppichhalle mehr Vorrat. Ab höheren Stufen kannst du Muster wechseln.",
        goals: [
            Pair(CarpetHall, SpiceBazaar, 1),
            Serve(Textiles, 70),
            Variety(5),
        ],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Der Wind über den Dächern",
        district: "Qasr Dun",
        greeting: "Trockener Wind zieht vom Rücken der Düne herab. Die Läden binden die Markisen neu fest.",
        hint: "Stürme können Karawanen aufhalten. Nutze ruhige Tage für die langen Routen.",
        goals: [RouteVariety(4), SpiceRoutes(7), CaravanIncome(360)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Die Hofbrunnen",
        district: "Ain Zamra",
        greeting: "An der Steintreppe wachsen drei Becken zusammen. Die Händler kommen auch aus dem neuen Viertel.",
        hint: "Ein Qanat und nahe Zisternen bringen Wasser für Werkstätten und Karawanen.",
        goals: [Pair(Qanat, Cistern, 2), Residents(18), Water(65)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Rotes Glas, blaue Fäden",
        district: "Dar Jawahir",
        greeting: "Rima tauscht mit Narim Muster. Zum Fest stehen Glas und Teppiche nebeneinander auf den Tischen.",
        hint: "Prüfe die erwarteten Gäste. Unterschiedliche Lieblingsangebote tragen zum gemeinsamen Tagesumsatz bei.",
        goals: [Serve(Textiles, 112), Serve(Glass, 110), Specials(60)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Das Tor zur Wüste",
        district: "Hisn Qamar",
        greeting: "Vor dem Tor ist Platz für Kisten und Lasttiere. Die Händler wollen mehr als eine kurze Tagesreise machen.",
        hint: "Plane Gewürze und Datteln für die Fernkarawane ein, bevor sie ausverkauft sind.",
        goals: [Upgrade(Caravanserai, 4), SpiceRoutes(9), CaravanIncome(515)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Goldene Fenster",
        district: "Al Manara",
        greeting: "Über den Wasserstufen hängen die Leuchten bis zum Morgengrauen. Reisende kommen für Stoff, Duft und einen Platz im Garten.",
        hint: "Im Mondgarten und in der Duftwerkstatt lohnt sich ein Angebot für die späten Gäste.",
        guests: &[
            Shade, Incense, Textiles, Glass, Bed, Shade, Incense, Textiles, Dates, Shade,
        ],
        locals: &[Shade, Dates, Textiles, Incense, Glass, Shade, Bed],
        goals: [Upgrade(MoonGarden, 4), Serve(Incense, 111), GoodDay(88)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Der Handelstag am Strom",
        district: "Mersa Lujain",
        greeting: "Am Strom füllen sich die Gassen von beiden Seiten. Samir hält das Lager für späte Gäste offen.",
        hint: "Versorge die Fernkarawane neben dem laufenden Markt. Ein gut besuchter Festtag bringt zusätzliche Einnahmen.",
        goals: [MarketDay(149), Serve(Spices, 125), FeastGuests(29)],
        ..PALACE_TOWN
    },
    Chapter {
        name: "Tausend Lichter",
        district: "Madinat Layl",
        greeting: "Die Händler lassen Lichter ins Wasser spiegeln. Nura begrüßt die letzte Karawane der Reise.",
        hint: "Halte genug Wasser, Wohnraum und verschiedene Angebote für die ganze Oase bereit.",
        goals: [SpiceRoutes(11), Residents(21), Variety(7)],
        ..PALACE_TOWN
    },
];

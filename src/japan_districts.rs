//! Nili's route through the harbors of the Inland Sea.
use super::{Chapter, Goal::*, Layout, REGION_SIZE, Region};
use crate::{
    errands::Job,
    model::{
        Favor::{Cargo, Jugs, Shade as ShadeFavor},
        Kind::{
            Inn, Machiya, PaperWorkshop, RiceKitchen, RicePaddy, Storehouse, TeaGarden, TeaHouse,
            Temple, Well,
        },
        Service::{Bed, Lantern, Prayer, Rice, Tea},
        Weather::{Clear, Cloud, Feast, Heat, Rain, Storm, Wind},
    },
};

const RICE_PLANS: &[crate::model::Kind] = &[Machiya, Well, RicePaddy, RiceKitchen];
const TEMPLE_PLANS: &[crate::model::Kind] = &[Machiya, Well, RicePaddy, RiceKitchen, Temple];
const TEA_PLANS: &[crate::model::Kind] = &[
    Machiya,
    Well,
    RicePaddy,
    RiceKitchen,
    Temple,
    TeaGarden,
    TeaHouse,
];
const HARBOR_PLANS: &[crate::model::Kind] = &[
    Machiya,
    Well,
    RicePaddy,
    RiceKitchen,
    Temple,
    TeaGarden,
    TeaHouse,
    PaperWorkshop,
    Inn,
    Storehouse,
];

const COAST: Chapter = Chapter {
    region: Region::Japan,
    shift: crate::campaign::Shift::Day,
    name: "",
    district: "",
    difficulty: "Sanft",
    greeting: "",
    hint: "",
    layout: Layout::legacy(4),
    coins: 53,
    spring: 4,
    ferry: 6,
    walk: 5,
    unlock: 0,
    building_set: Some(RICE_PLANS),
    par: 7,
    favor_goal: 3,
    starts: &[(1, Machiya), (2, Well), (6, RicePaddy)],
    locals: &[Rice],
    guests: &[Rice],
    weather: &[Clear, Cloud, Rain, Feast, Wind],
    favors: &[Cargo, Jugs, Cargo],
    goals: [Build(RiceKitchen, 1), Serve(Rice, 22), Renown(26)],
};

const PILGRIM: Chapter = Chapter {
    building_set: Some(TEMPLE_PLANS),
    ferry: 9,
    starts: &[(1, Machiya), (2, Well), (3, RicePaddy), (7, RiceKitchen)],
    locals: &[Rice, Rice, Prayer],
    guests: &[Prayer, Rice, Prayer, Rice, Prayer],
    favor_goal: 4,
    par: 9,
    weather: &[Cloud, Clear, Rain, Feast],
    ..COAST
};

const TEA_COAST: Chapter = Chapter {
    building_set: Some(TEA_PLANS),
    layout: Layout::legacy(5),
    ferry: 11,
    starts: &[
        (1, Machiya),
        (2, Well),
        (3, RicePaddy),
        (7, RiceKitchen),
        (8, Temple),
    ],
    locals: &[Tea, Rice, Prayer, Rice, Tea, Rice],
    guests: &[Tea, Prayer, Rice, Tea, Prayer, Rice, Tea],
    favors: &[Cargo, ShadeFavor, Jugs, Cargo],
    coins: 68,
    par: 11,
    favor_goal: 5,
    difficulty: "Leicht",
    ..PILGRIM
};

const LANTERN_COAST: Chapter = Chapter {
    building_set: Some(HARBOR_PLANS),
    ferry: 15,
    starts: &[
        (1, Machiya),
        (2, Well),
        (3, RicePaddy),
        (7, RiceKitchen),
        (8, Temple),
        (9, TeaGarden),
    ],
    locals: &[Rice, Tea, Prayer, Rice, Tea, Lantern, Rice, Prayer],
    guests: &[
        Prayer, Rice, Tea, Lantern, Bed, Lantern, Prayer, Rice, Bed, Tea,
    ],
    favors: &[Cargo, ShadeFavor, Jugs, Cargo],
    coins: 87,
    par: 15,
    favor_goal: 7,
    difficulty: "Mittel",
    weather: &[Clear, Cloud, Wind, Feast, Rain, Heat],
    ..TEA_COAST
};

const ISLAND_MARKETS: Chapter = Chapter {
    layout: Layout::legacy(6),
    difficulty: "Anspruchsvoll",
    coins: 105,
    ferry: 19,
    par: 19,
    favor_goal: 9,
    starts: &[
        (1, Machiya),
        (2, Well),
        (3, RicePaddy),
        (7, RiceKitchen),
        (8, Temple),
        (9, TeaGarden),
        (10, Inn),
    ],
    locals: &[Rice, Lantern, Tea, Prayer, Bed, Rice, Prayer, Tea],
    guests: &[
        Tea, Prayer, Lantern, Bed, Rice, Bed, Prayer, Lantern, Tea, Rice, Bed,
    ],
    weather: &[Cloud, Rain, Feast, Wind, Clear, Heat],
    ..LANTERN_COAST
};

const STRAIT: Chapter = Chapter {
    par: 24,
    coins: 119,
    ferry: 22,
    favor_goal: 11,
    weather: &[Rain, Wind, Clear, Feast, Storm, Cloud, Heat],
    ..ISLAND_MARKETS
};

pub(super) const CHAPTERS: [Chapter; REGION_SIZE] = [
    Chapter {
        name: "Reis am Anlegeplatz",
        district: "Tomonoura",
        greeting: "Aki wartet zwischen den Holzhäusern. Mit der ersten Fähre kommen hungrige Reisende.",
        hint: "Neben einem Reisfeld bereitet die Reisküche mehr Portionen zu.",
        goals: [Pair(RiceKitchen, RicePaddy, 1), Serve(Rice, 23), Renown(26)],
        ..COAST
    },
    Chapter {
        name: "Die Glocke über dem Wasser",
        district: "Miyajima",
        greeting: "Hana hat einen Platz für einen Tempel gefunden. Vom Anleger führt ein kurzer Weg dorthin.",
        hint: "Geh mit Nili zum Tempel und läute die Glocke. Zwei zusätzliche Pilger kommen zur Halle.",
        goals: [Build(Temple, 1), Bell(3), Serve(Prayer, 16)],
        ..PILGRIM
    },
    Chapter {
        name: "Tee an der Steintreppe",
        district: "Onomichi",
        greeting: "Den Hang hinauf duftet es nach frisch geröstetem Tee. Auch die Kinder kommen hinunter zum Markt.",
        hint: "Ein Teegarten in der Nähe vergrößert den Vorrat des Teehauses.",
        goals: [Pair(TeaHouse, TeaGarden, 1), Serve(Tea, 20), GoodDay(77)],
        ..TEA_COAST
    },
    Chapter {
        name: "Felder am Hang",
        district: "Ikuchijima",
        greeting: "Das Wasser steht schon zwischen den jungen Reishalmen. Aki zeigt Nili den Brunnen.",
        hint: "Ein Brunnen am Feld steigert die Ernte. Die Höfe brauchen Reis, Tee und einen erreichbaren Tempel.",
        goals: [Harvest(19), Serve(Rice, 34), Serve(Prayer, 25)],
        par: 12,
        ..TEA_COAST
    },
    Chapter {
        name: "Licht vor der Gebetshalle",
        district: "Ōmishima",
        greeting: "Yui faltet Papier für den nächsten Festtag. Die älteren Nachbarn helfen beim Aufhängen.",
        hint: "Baue die Lampionwerkstatt neben den Tempel. Beim Läuten kommt dann ein Gast mehr.",
        goals: [Pair(Temple, PaperWorkshop, 1), Serve(Lantern, 21), Bell(5)],
        starts: &[
            (1, Machiya),
            (2, Well),
            (3, RicePaddy),
            (4, TeaGarden),
            (7, RiceKitchen),
            (8, Temple),
        ],
        ..LANTERN_COAST
    },
    Chapter {
        name: "Betten für die Nachtfähre",
        district: "Hakatajima",
        greeting: "Die Reisenden legen spät an. Sora sucht einen trockenen Ort für ihre Taschen und Betten.",
        hint: "Ein Gasthaus am Kai verkürzt die Wege. Nili kann Reisende von dort zu ihrem Ziel begleiten.",
        goals: [Build(Inn, 1), Serve(Bed, 28), Route(Job::Welcome, 4)],
        ..LANTERN_COAST
    },
    Chapter {
        name: "Eine Gasse für alle",
        district: "Ōshima",
        greeting: "Am Anfang der Gasse wird Tee ausgeschenkt, am Ende leuchten schon die Windlichter.",
        hint: "Platziere einen Speicher so, dass er an mehreren Läden in Reichweite liegt.",
        goals: [Pair(Storehouse, RiceKitchen, 1), Variety(5), MarketDay(62)],
        par: 17,
        ..LANTERN_COAST
    },
    Chapter {
        name: "Der Kai in der Strömung",
        district: "Kurushima",
        greeting: "Wind drückt Treibgut an den Steg. Zwischen den Inseln werden die Fähren langsamer.",
        hint: "Nach Wind und Sturm hilft Nili am Kai. Räumt er ihn frei, gehen die Fährgäste weiter.",
        goals: [Route(Job::Quay, 4), FerryDay(11), Serve(Prayer, 35)],
        weather: &[Wind, Cloud, Storm, Rain, Feast, Clear],
        par: 17,
        ..LANTERN_COAST
    },
    Chapter {
        name: "Reis und Papier auf dem Markt",
        district: "Imabari",
        greeting: "Sora kommt mit geknüpften Körben. Die Küchen und die Werkstätten rüsten sich für den Markttag.",
        hint: "Ein Feld und ein Brunnen neben der Reisküche helfen, die wachsende Nachfrage zu decken.",
        goals: [Upgrade(RiceKitchen, 3), Serve(Lantern, 39), Earn(250)],
        ..ISLAND_MARKETS
    },
    Chapter {
        name: "Wenn die Gärten duften",
        district: "Gogoshima",
        greeting: "Der Duft aus dem Teegarten zieht über den Weg zum Tempel. Hana lädt die Nachbarn ein.",
        hint: "Teegärten geben benachbarten Tempeln zusätzliche Plätze und versorgen das Teehaus.",
        goals: [Pair(Temple, TeaGarden, 1), Serve(Tea, 38), GoodDay(82)],
        ..ISLAND_MARKETS
    },
    Chapter {
        name: "Händler unter den Dächern",
        district: "Matsuyama",
        greeting: "An jedem Fenster hängt ein anderer Lampion. Die Reisenden wollen etwas für den Heimweg mitnehmen.",
        hint: "Setze Preise nach dem Tagesplan. Passende Lampions bringen mehr Umsatz und Ansehen.",
        goals: [Upgrade(PaperWorkshop, 3), Serve(Lantern, 55), MarketDay(76)],
        ..ISLAND_MARKETS
    },
    Chapter {
        name: "Ruhe am Zedernhang",
        district: "Kashima",
        greeting: "Unter den Zedern ist es ruhig. Die Pilger warten auf den ersten Klang der Glocke.",
        hint: "Du kannst pro Tempel einmal am Tag läuten. Gäste aus den Wohnhäusern gehen auch selbst zum Gebet.",
        goals: [Bell(9), Serve(Prayer, 56), FerryDay(13)],
        ..ISLAND_MARKETS
    },
    Chapter {
        name: "Der lange Weg vom Anleger",
        district: "Nuwajima",
        greeting: "Farne wachsen bis ans Wasser. Die Fähre bringt heute Familien und Pilger mit müden Beinen.",
        hint: "Begleite Gäste vom Kai zu einem Tempel, Gasthaus oder einem Laden mit freien Plätzen.",
        goals: [Route(Job::Welcome, 6), Serve(Bed, 65), Serve(Prayer, 53)],
        ..ISLAND_MARKETS
    },
    Chapter {
        name: "Zwei Ernten in der Bucht",
        district: "Suō-Ōshima",
        greeting: "Die Bauern legen Kornmatten am Wasser aus. Aki möchte zwei Reisküchen versorgen.",
        hint: "Verteile Reisfelder und Brunnen auf beide Straßenseiten, damit die Küchen kurze Wege haben.",
        goals: [
            Pair(RiceKitchen, RicePaddy, 2),
            Harvest(34),
            Serve(Rice, 81),
        ],
        ..STRAIT
    },
    Chapter {
        name: "Der kleine Herbergsmarkt",
        district: "Kaminoseki",
        greeting: "Das Gasthaus am Anleger ist fast voll. Sora stellt einen Korb mit Essensschalen auf die Treppe.",
        hint: "Gasthaus, Reisküche und Teehaus in der Nähe des Kais decken mehrere Wünsche einer Ankunft.",
        goals: [Upgrade(Inn, 4), Serve(Bed, 88), MarketDay(94)],
        ..STRAIT
    },
    Chapter {
        name: "Nach der Regenfront",
        district: "Iwakuni",
        greeting: "Wasser läuft von der Uferstraße. Am Abend sollen die Lampions wieder leuchten.",
        hint: "Räume den Anleger bei rauer See. In der ruhigen Bucht lohnt sich ein Fährtag.",
        goals: [Route(Job::Quay, 5), Serve(Lantern, 74), FerryDay(18)],
        weather: &[Rain, Storm, Clear, Wind, Feast, Cloud],
        ..STRAIT
    },
    Chapter {
        name: "Tee vor der Überfahrt",
        district: "Etajima",
        greeting: "Am Steg bringen Nachbarn Tücher und Teeschalen zum Trocknen hinaus.",
        hint: "Teegärten versorgen Teehäuser mit mehr Portionen. Große Mengen brauchen ausgebauten Garten und Laden.",
        goals: [Upgrade(TeaGarden, 4), Serve(Tea, 92), GoodDay(85)],
        ..STRAIT
    },
    Chapter {
        name: "Fünf Türen am Meer",
        district: "Kurahashijima",
        greeting: "Eine Gruppe möchte zuerst in den Tempel, eine zweite sucht warme Reisbällchen.",
        hint: "Versorge alle fünf Wünsche an einem Tag. Ein Festtag bringt zusätzliche Fährgäste.",
        goals: [Variety(5), FeastGuests(20), Serve(Prayer, 89)],
        ..STRAIT
    },
    Chapter {
        name: "Die Werkstatt hinterm Speicher",
        district: "Shimo-Kamagari",
        greeting: "Yui trägt fertige Lampions aus der Werkstatt. Es fehlen noch Vorräte für den großen Abend.",
        hint: "Ein Speicher stärkt Läden bis zwei Bauplätze entfernt und ist eine Quelle für Nachschubtouren.",
        goals: [
            Pair(Storehouse, PaperWorkshop, 1),
            Route(Job::Freight, 6),
            Serve(Lantern, 98),
        ],
        ..STRAIT
    },
    Chapter {
        name: "Glocken zwischen den Gärten",
        district: "Ōsakishimojima",
        greeting: "Zwei Tempelhöfe liegen am Hang. Hana wartet am unteren Tor auf die erste Gruppe.",
        hint: "Ein Teegarten bringt zusätzliche Plätze. Läute an beiden Tempeln, solange noch Zeit für den Besuch bleibt.",
        goals: [Build(Temple, 2), Bell(14), Serve(Prayer, 107)],
        par: 26,
        ..STRAIT
    },
    Chapter {
        name: "Volle Körbe für die Heimfahrt",
        district: "Yugejima",
        greeting: "Auf den Reisfeldern liegen geflochtene Matten, am Kai warten drei Boote auf ihre Ladung.",
        hint: "Mehr Reis bringt mehr Umsatz, wenn auch Küche, Wasser und Nachfrage zusammenpassen.",
        goals: [Harvest(40), Serve(Rice, 133), MarketDay(108)],
        par: 26,
        ..STRAIT
    },
    Chapter {
        name: "Besuch aus den Nachbarbuchten",
        district: "Mukaishima",
        greeting: "Jüngere Gäste laufen zur Reisküche, die älteren suchen Tee und Schatten.",
        hint: "Plane mit den Altersgruppen im Tagesplan und verteile die Angebote entlang der Gasse.",
        goals: [GoodDay(89), Variety(5), Earn(735)],
        par: 27,
        ..STRAIT
    },
    Chapter {
        name: "Das Fest der Lampions",
        district: "Naoshima",
        greeting: "In der Gasse hängt eine Reihe Papierlichter. Nili läuft früh los, um Reisende abzuholen.",
        hint: "Gib jeder Abfahrt genug Plätze. Ein Nachschubauftrag und eine Gästetour lassen sich am selben Tag erledigen.",
        goals: [FeastGuests(25), Serve(Lantern, 132), RouteVariety(4)],
        coins: 130,
        par: 29,
        favor_goal: 13,
        ..STRAIT
    },
    Chapter {
        name: "Die Lichter der Inlandsee",
        district: "Awajishima",
        greeting: "Aki, Hana und Yui kommen für das letzte Hafenfest zusammen. Über dem Wasser klingt die Glocke.",
        hint: "Für das Finale brauchen Pilger einen ruhigen Tempelhof und Reisende eine gut versorgte Gasse.",
        goals: [Bell(18), Serve(Prayer, 149), MarketDay(125)],
        difficulty: "Finale Japan",
        ferry: 25,
        coins: 142,
        par: 31,
        favor_goal: 14,
        weather: &[Clear, Cloud, Wind, Feast, Rain, Feast, Storm],
        ..STRAIT
    },
];

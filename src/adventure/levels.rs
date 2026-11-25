use super::{Build, Good};
use crate::{
    campaign::{CORE_HARBORS, Region},
    model::Kind,
};

pub const LEVELS: usize = CORE_HARBORS * Region::ALL.len();

#[derive(Clone, Copy)]
pub struct Level {
    pub index: usize,
    pub region: Region,
    pub stage: usize,
    pub name: &'static str,
    pub theme: &'static str,
    pub landmark: &'static str,
    pub relic: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Requirement {
    pub kind: Build,
    pub count: u8,
    pub tier: u8,
}

#[derive(Clone, Copy)]
pub struct SupplyOrder {
    pub title: &'static str,
    pub goods: [(Good, u16); 2],
}

#[derive(Clone, Copy)]
pub struct WorkOrder {
    pub workshop: Build,
    pub tier: u8,
    pub base: Good,
    pub base_needed: u16,
    pub ingredient: Good,
    pub units: u16,
    pub companion: Good,
    pub title: &'static str,
}

const NAMES: [[(&str, &str); CORE_HARBORS]; Region::ALL.len()] = [
    [
        ("Kleions Ufer", "Erste Häuser an der geschützten Bucht"),
        ("Tal der Oliven", "Oliven und Mehl für eine Taverne"),
        ("Rote Klippen", "Ziegel für das Töpferviertel"),
        ("Der Kiefernhang", "Werkzeug für die Siedlung am Hang"),
        ("Drei Windbuchten", "Kräuter für Wohnhöfe und Imkerei"),
        ("Salzgärten", "Ein Gasthaus am alten Pfad"),
        ("Hain des Zeus", "Tongefäße und ein geheiligter Stein"),
        ("Theras Aschefelder", "Ein Viertel für die Färberinnen"),
        ("Sonnenbogen", "Eine ausgebaute Ölpresse im Olivental"),
        ("Tal der Zikaden", "Werkstätten für das Sommerfest"),
        ("Klippe der Myrten", "Das Lotsenhaus über der Meerenge"),
        ("Die Torinsel", "Das große Heiligtum und volle Speicher"),
    ],
    [
        ("Flavias Landung", "Der erste Hof an der Tiberbucht"),
        ("Silberstrand", "Ein öffentlicher Rastplatz am Meer"),
        ("Lehmfelder von Aricia", "Ein Brennofen für rote Ziegel"),
        ("Die alte Via", "Thermen für das Viertel"),
        (
            "Hügel der Mühlen",
            "Ein Speicher und Mehl für die Backstuben",
        ),
        ("Der Travertinbogen", "Gartenplätze im neuen Viertel"),
        ("Wasser über dem Tal", "Ein Aquädukt und Steine vom Berg"),
        ("Hoher Tiber", "Wachstafeln für den Platz am Forum"),
        ("Die blaue Quelle", "Ein Gartenhof mit Wasserzuleitung"),
        ("Stufen von Lavinium", "Wohnraum für eine größere Siedlung"),
        ("Die rote Terrasse", "Ein Markt unter Arkaden"),
        (
            "Kap der Thermen",
            "Ein großes Bad und ein Platz für Händler",
        ),
    ],
    [
        (
            "Kiefern der Inlandsee",
            "Ein Machiya-Haus und Holz für den Steg",
        ),
        (
            "Terrasse am Reisfeld",
            "Bestellte Felder und warme Reisgerichte",
        ),
        ("Kranichufer", "Ziegel für den Baugrund am Hang"),
        ("Mondbambus", "Tee für eine Rast am Waldrand"),
        ("Die Pfade von Nagi", "Pilger finden Wasser und Unterkunft"),
        ("Ahornpass", "Lampions für die Gasse"),
        ("Glockenhain", "Eine alte Glocke und ein Tempel"),
        ("Zedernkap", "Glasur für die Keramikwerkstatt"),
        ("Blüten der Strömung", "Ein Gemüseviertel und ein Teehaus"),
        ("Wasser im Nebel", "Werkstätten für Seide am kleinen Kai"),
        ("Kamin der Inseln", "Schalen und Festlampions"),
        (
            "Tor der Pflaumenblüte",
            "Ein großer Tempel und eine Seidengalerie",
        ),
    ],
    [
        ("Livs Landspitze", "Das erste Langhaus am Felsen"),
        ("Wiese der Fjorde", "Fang und Feuer für die Räucherei"),
        ("Schieferbucht", "Ziegel und ein Schuppen für Vorräte"),
        ("Der lange Sund", "Holz für Boote und Netze"),
        ("Wollklippen", "Schafweiden und die erste Weberei"),
        ("Rauch am Hochufer", "Platz für ein Fest in der Siedlung"),
        ("Der Stein von Eira", "Ein Wappen für die große Halle"),
        ("Hafengang der Stürme", "Fässer für den nächsten Handelszug"),
        ("Schafe unterm Nordstern", "Wolle und ein ausgebauter Hof"),
        ("Die grüne Bucht", "Ein gemeinsames Fest entlang der Straße"),
        ("Felsenfeuer", "Ein Signal für die fernen Boote"),
        ("Die lange Fjordnacht", "Ein Leuchtfeuer und volle Vorräte"),
    ],
    [
        ("Nuras Quellen", "Datteln am ersten Hof"),
        ("Pfad der Datteln", "Wasser für die Palmengärten"),
        ("Ziegelkessel", "Lehm aus der warmen Schlucht"),
        ("Läden am Strom", "Ein Basar für Reisende"),
        ("Jasminhang", "Wassergefäße für den nächsten Aufbruch"),
        ("Kupfer unter dem Sand", "Gläser und Duftöle"),
        (
            "Der Brunnen im Fels",
            "Ein alter Wasserschlüssel für den Qanat",
        ),
        ("Schattentor", "Eine Herberge für Karawanen"),
        (
            "Hof der stillen Gärten",
            "Pflanzen und Wasser am Mondgarten",
        ),
        ("Regen aus Gold", "Duftwaren und Teppiche für den Handel"),
        ("Treppe der Sterne", "Eine ausgebaute Glashalle"),
        (
            "Tal der tausend Leuchten",
            "Teppichhalle und Mondgarten über dem Strom",
        ),
    ],
];

impl Level {
    pub fn at(index: usize) -> Self {
        let region = Region::ALL[index / CORE_HARBORS];
        let stage = index % CORE_HARBORS;
        let (name, theme) = NAMES[region as usize][stage];
        let (landmark, relics) = match region {
            Region::Greece => (
                "Tempel des Zeus",
                [
                    "Heiliger Bergstein",
                    "Ölkrug aus Myrte",
                    "Amphore der Lotsen",
                    "Sonnenscheibe",
                ],
            ),
            Region::Rome => (
                "Quellheiligtum",
                [
                    "Travertinsiegel",
                    "Bronzene Wassernadel",
                    "Wegstein aus Marmor",
                    "Schlüssel der Thermen",
                ],
            ),
            Region::Japan => (
                "Schrein der Zedern",
                [
                    "Glockenstück",
                    "Plättchen aus Jade",
                    "Sternenlampion",
                    "Pflaumenblütensiegel",
                ],
            ),
            Region::Nordic => (
                "Halle der Eira",
                [
                    "Bernsteinwappen",
                    "Silberne Webnadel",
                    "Rabenfibel",
                    "Laterne aus Horn",
                ],
            ),
            Region::Oasis => (
                "Tor der Quelle",
                [
                    "Bronzener Wasserschlüssel",
                    "Mondbecher",
                    "Sternenglas",
                    "Siegel der Karawanen",
                ],
            ),
        };
        let relic = relics[match stage {
            8 => 1,
            10 => 2,
            11 => 3,
            _ => 0,
        }];
        Self {
            index,
            region,
            stage,
            name,
            theme,
            landmark,
            relic,
        }
    }

    pub fn home(self) -> Kind {
        match self.region {
            Region::Greece | Region::Rome => Kind::House,
            Region::Japan => Kind::Machiya,
            Region::Nordic => Kind::Longhouse,
            Region::Oasis => Kind::OasisHome,
        }
    }

    pub fn well(self) -> Kind {
        if self.region == Region::Oasis {
            Kind::Cistern
        } else {
            Kind::Well
        }
    }

    pub fn hall(self) -> &'static str {
        match self.region {
            Region::Greece => "Bouleuterion",
            Region::Rome => "Kurie",
            Region::Japan => "Dorfhaus",
            Region::Nordic => "Thinghalle",
            Region::Oasis => "Ratshof",
        }
    }

    pub fn diameter(self) -> f32 {
        73.0 + self.stage as f32 * 7.0 + self.region as usize as f32 * 4.5
    }

    pub fn inland_plots(self) -> usize {
        10 + self.stage * 2
    }

    pub fn plots(self) -> usize {
        self.inland_plots() + if self.region == Region::Nordic { 2 } else { 0 }
    }

    pub fn coastal(self, lot: usize) -> bool {
        self.region == Region::Nordic && (self.inland_plots()..self.plots()).contains(&lot)
    }

    pub fn requirements(self) -> [Requirement; 3] {
        use Build::{Brickyard, Forge, Harbor, Sawmill};
        use Kind::*;
        let special = match self.region {
            Region::Greece => [
                Bakery,
                Taverna,
                Potter,
                Storehouse,
                Apiary,
                Inn,
                Shrine,
                Dyer,
                Press,
                Stoa,
                FerryOffice,
                Shrine,
            ],
            Region::Rome => [
                Bakery,
                Stoa,
                Potter,
                Baths,
                Mill,
                Storehouse,
                Aqueduct,
                Scribe,
                GardenCourt,
                Villa,
                Forum,
                Baths,
            ],
            Region::Japan => [
                RicePaddy,
                RiceKitchen,
                PaperWorkshop,
                TeaHouse,
                TeaGarden,
                Inn,
                Temple,
                CeramicStudio,
                MarketGarden,
                SilkShop,
                PaperWorkshop,
                Temple,
            ],
            Region::Nordic => [
                Fishery,
                Smokehouse,
                Woodlot,
                Boatyard,
                Sheepfold,
                FeastHall,
                FeastHall,
                Cooperage,
                Weaver,
                Storehouse,
                SignalTower,
                SignalTower,
            ],
            Region::Oasis => [
                DateGarden,
                Caravanserai,
                Glassworks,
                SpiceBazaar,
                Caravanserai,
                Perfumer,
                Qanat,
                Caravanserai,
                MoonGarden,
                CarpetHall,
                Glassworks,
                CarpetHall,
            ],
        }[self.stage];
        let structure = match self.stage {
            0 | 1 => Sawmill,
            2 => Brickyard,
            3 => Forge,
            4..=5 => Harbor(self.well()),
            6..=7 => Brickyard,
            8..=9 => Harbor(self.well()),
            _ => Forge,
        };
        let tier = 1 + ((self.stage.saturating_sub(2)) / 2) as u8;
        [
            Requirement {
                kind: Harbor(self.home()),
                count: 1 + (self.stage / 3) as u8,
                tier: 1,
            },
            Requirement {
                kind: structure,
                count: 1,
                tier: (tier - 1).max(1),
            },
            Requirement {
                kind: Harbor(special),
                count: 1,
                tier,
            },
        ]
    }

    pub fn support(self) -> Option<Requirement> {
        let kind = match self.stage {
            4 => match self.region {
                Region::Greece => Kind::HerbGarden,
                Region::Rome => Kind::Bakery,
                Region::Japan => Kind::TeaHouse,
                Region::Nordic => Kind::Weaver,
                Region::Oasis => Kind::DateGarden,
            },
            _ => return None,
        };
        Some(Requirement {
            kind: Build::Harbor(kind),
            count: 1,
            tier: 1,
        })
    }

    pub fn goals(self) -> impl Iterator<Item = Requirement> {
        let [_, workshop, special] = self.requirements();
        let well = Build::Harbor(self.well());
        let utilities = (self.stage >= 1 && workshop.kind != well).then_some(Requirement {
            kind: well,
            count: 1,
            tier: if self.stage >= 8 { 2 } else { 1 },
        });
        let mill = (self.stage >= 5 && workshop.kind != Build::Sawmill).then_some(Requirement {
            kind: Build::Sawmill,
            count: 1,
            tier: if self.stage >= 10 { 3 } else { 2 },
        });
        let storehouse = Build::Harbor(Kind::Storehouse);
        let warehouse = (self.stage >= 3 && special.kind != storehouse).then_some(Requirement {
            kind: storehouse,
            count: 1,
            tier: if self.stage >= 9 { 2 } else { 1 },
        });
        self.requirements()
            .into_iter()
            .chain(self.support())
            .chain(utilities)
            .chain(mill)
            .chain(warehouse)
            .chain((self.stage == CORE_HARBORS - 1).then_some(Requirement {
                kind: Build::Monument,
                count: 1,
                tier: 2,
            }))
    }

    pub fn work_order(self) -> Option<WorkOrder> {
        use Good::*;
        let (stages, workshop, base, ingredients, companion, title) = match self.region {
            Region::Greece => (
                [4, 7, 9, 11],
                Build::Harbor(Kind::Potter),
                Craft,
                [Local, Oil, Cloth, Oil],
                Boards,
                "Amphoren für die Küstenwache",
            ),
            Region::Rome => (
                [5, 8, 10, 11],
                Build::Brickyard,
                Bricks,
                [Stone, Tools, Local, Tools],
                Bricks,
                "Reliefziegel für die Wachstation",
            ),
            Region::Japan => (
                [5, 8, 9, 11],
                Build::Harbor(Kind::PaperWorkshop),
                Paper,
                [Boards, Cloth, Local, Cloth],
                Meals,
                "Laternen für die Wegstation",
            ),
            Region::Nordic => (
                [4, 7, 9, 11],
                Build::Harbor(Kind::Weaver),
                Cloth,
                [Wood, Craft, Local, Tools],
                Fish,
                "Flechtbänder für das Küstenhaus",
            ),
            Region::Oasis => (
                [5, 7, 10, 11],
                Build::Harbor(Kind::Perfumer),
                Perfume,
                [Food, Water, Cloth, Craft],
                Water,
                "Duftöle für die Karawane",
            ),
        };
        let wave = stages.into_iter().position(|stage| stage == self.stage)?;
        Some(WorkOrder {
            workshop,
            tier: [2, 3, 3, 4][wave],
            base,
            base_needed: [4, 5, 6, 8][wave],
            ingredient: ingredients[wave],
            units: [4, 5, 6, 8][wave],
            companion,
            title,
        })
    }

    pub(super) fn regular_orders(self) -> u8 {
        match self.stage {
            0 => 0,
            1..=5 => 1,
            6..=9 => 2,
            _ => 3,
        }
    }

    pub fn order_count(self) -> u8 {
        self.regular_orders() + u8::from(self.work_order().is_some())
    }

    pub fn work_order_delivered(self, delivered: u8) -> bool {
        self.work_order().is_some() && delivered > self.regular_orders()
    }

    pub fn outpost(self) -> &'static str {
        match self.region {
            Region::Greece => "Küstenposten",
            Region::Rome => "Wachstation",
            Region::Japan => "Wegstation",
            Region::Nordic => "Küstenschutzhaus",
            Region::Oasis => "Rastplatz der Karawane",
        }
    }

    pub fn order(self, index: u8) -> Option<SupplyOrder> {
        use Good::*;
        if index >= self.regular_orders() {
            let order = (index == self.regular_orders())
                .then(|| self.work_order())
                .flatten()?;
            return Some(SupplyOrder {
                title: order.title,
                goods: [
                    (Masterwork, order.units),
                    (order.companion, 3 + self.stage as u16 / 2),
                ],
            });
        }
        let scale = self.stage as u16;
        let (title, goods) = match (index, self.stage, self.region) {
            (0, 1, _) => ("Vorräte für die erste Schicht", [(Boards, 8), (Meals, 6)]),
            (0, 2, _) => (
                "Den Anleger ausbessern",
                [(Boards, 8 + scale), (Bricks, 6 + scale)],
            ),
            (0, 3, _) => (
                "Werkzeug für die Bauleute",
                [(Boards, 7 + scale), (Tools, 4 + scale)],
            ),
            (0, _, Region::Greece) => (
                "Proviant für die Küstenwache",
                [(Meals, 6 + scale), (Oil, 5 + scale)],
            ),
            (0, 4 | 8 | 10, Region::Rome) => (
                "Mehl für die Wachstation",
                [(Flour, 8 + scale), (Boards, 5 + scale)],
            ),
            (0, _, Region::Rome) => (
                "Die Wachstation instand setzen",
                [(Bricks, 8 + scale), (Tools, 4 + scale)],
            ),
            (0, _, Region::Japan) => (
                "Die Wegstation einrichten",
                [(Boards, 8 + scale), (Paper, 5 + scale)],
            ),
            (0, _, Region::Nordic) => (
                "Wintervorräte anlegen",
                [(Cloth, 6 + scale), (Fish, 7 + scale)],
            ),
            (0, _, Region::Oasis) => (
                "Die Karawane versorgen",
                [(Water, 14 + scale), (Perfume, 5 + scale)],
            ),
            (1, _, _) => (
                "Den Außenposten wetterfest machen",
                [(Boards, 9 + scale), (Tools, 4 + scale)],
            ),
            (_, _, Region::Rome) => (
                "Werkzeug für die nächste Reise",
                [(Craft, 6 + scale), (Tools, 5 + scale)],
            ),
            _ => (
                "Waren für die nächste Reise",
                [(Craft, 6 + scale), (Cloth, 5 + scale)],
            ),
        };
        Some(SupplyOrder { title, goods })
    }

    pub fn plan(self) -> Option<Build> {
        (self.stage >= 6).then(|| self.requirements()[2].kind)
    }

    pub fn expedition(self) -> bool {
        matches!(self.stage, 6 | 8 | 10 | 11)
    }

    pub fn offering(self) -> Option<(Good, u16)> {
        match self.stage {
            7 => Some(match self.region {
                Region::Greece => (Good::Meals, 12),
                Region::Rome => (Good::Bricks, 12),
                Region::Japan => (Good::Paper, 12),
                Region::Nordic => (Good::Boards, 16),
                Region::Oasis => (Good::Water, 20),
            }),
            9 => Some(match self.region {
                Region::Greece => (Good::Craft, 14),
                Region::Rome => (Good::Meals, 15),
                Region::Japan => (Good::Meals, 16),
                Region::Nordic => (Good::Craft, 16),
                Region::Oasis => (Good::Craft, 16),
            }),
            11 => Some(match self.region {
                Region::Greece => (Good::Craft, 18),
                Region::Rome => (Good::Craft, 20),
                Region::Japan => (Good::Cloth, 20),
                Region::Nordic => (Good::Cloth, 21),
                Region::Oasis => (Good::Cloth, 22),
            }),
            _ => None,
        }
    }

    /// The processed good each region is known for.
    pub fn signature(self) -> Good {
        match self.region {
            Region::Greece => Good::Oil,
            Region::Rome => Good::Flour,
            Region::Japan => Good::Paper,
            Region::Nordic => Good::Fish,
            Region::Oasis => Good::Perfume,
        }
    }

    pub fn reserve(self) -> (Good, u16) {
        match self.stage {
            0 => (Good::Boards, 8),
            1 => (
                if matches!(self.region, Region::Rome | Region::Oasis) {
                    Good::Water
                } else {
                    Good::Meals
                },
                10,
            ),
            2 => (Good::Bricks, 12),
            3 => (Good::Tools, 10),
            4 => match self.region {
                Region::Greece => (Good::Food, 21),
                Region::Rome | Region::Japan => (Good::Meals, 16),
                Region::Nordic => (Good::Fiber, 12),
                Region::Oasis => (Good::Water, 21),
            },
            5 => (
                if self.region == Region::Oasis {
                    Good::Craft
                } else {
                    Good::Tools
                },
                14,
            ),
            6 => (self.signature(), 18),
            7 => (
                if matches!(self.region, Region::Greece | Region::Japan) {
                    Good::Cloth
                } else {
                    Good::Craft
                },
                16,
            ),
            8 => (Good::Local, 24),
            9 => (
                match self.region {
                    Region::Greece => Good::Meals,
                    Region::Rome => Good::Bricks,
                    _ => Good::Cloth,
                },
                20,
            ),
            10 => (self.signature(), 24),
            _ => (Good::Tools, 28),
        }
    }
}

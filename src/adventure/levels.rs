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

    pub fn plots(self) -> usize {
        10 + self.stage * 2
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
                DateGarden,
                SpiceBazaar,
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
        self.requirements().into_iter().chain(self.support())
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
                Region::Japan => (Good::Craft, 12),
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
            6 => (Good::Local, 20),
            7 => (
                if matches!(self.region, Region::Greece | Region::Japan) {
                    Good::Cloth
                } else {
                    Good::Craft
                },
                16,
            ),
            8 => (Good::Water, 27),
            9 => (
                match self.region {
                    Region::Greece => Good::Meals,
                    Region::Rome => Good::Bricks,
                    _ => Good::Cloth,
                },
                20,
            ),
            10 => (Good::Craft, 25),
            _ => (Good::Tools, 28),
        }
    }
}

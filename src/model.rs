use serde::{Deserialize, Serialize};

use crate::{
    campaign::{Chapter, Goal, Region, harbor_stage},
    campaign_orders::{SalesProgress, TradeOrder, care_goal, trade_order},
    errands::{
        Blocked, JOB_TYPES, Job, ManualStage, NiliControl, Route, RouteStage, Stop, WELCOME_GUESTS,
    },
    trade::Voyage,
};

pub use crate::layout::{LOTS, distance};
pub const DAY_SECONDS: f32 = 65.0;
pub const MAX_LEVEL: u8 = 6;
pub const SERVICES: usize = 24;
pub const BUILDINGS: usize = 52;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    Bread,
    Pottery,
    Shade,
    Meze,
    Bed,
    Oil,
    Offering,
    Bath,
    Trade,
    Rice,
    Tea,
    Lantern,
    Prayer,
    SmokedFish,
    Textiles,
    HearthMeal,
    Honey,
    DyedCloth,
    Tablets,
    Casks,
    Dates,
    Spices,
    Glass,
    Incense,
}

impl Service {
    pub const ALL: [Self; SERVICES] = [
        Self::Bread,
        Self::Pottery,
        Self::Shade,
        Self::Meze,
        Self::Bed,
        Self::Oil,
        Self::Offering,
        Self::Bath,
        Self::Trade,
        Self::Rice,
        Self::Tea,
        Self::Lantern,
        Self::Prayer,
        Self::SmokedFish,
        Self::Textiles,
        Self::HearthMeal,
        Self::Honey,
        Self::DyedCloth,
        Self::Tablets,
        Self::Casks,
        Self::Dates,
        Self::Spices,
        Self::Glass,
        Self::Incense,
    ];

    pub fn priced(self) -> bool {
        self != Self::Prayer
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Bread => "Brot",
            Self::Pottery => "Keramik",
            Self::Shade => "Rast",
            Self::Meze => "Meze",
            Self::Bed => "Herberge",
            Self::Oil => "Olivenöl",
            Self::Offering => "Weihegaben",
            Self::Bath => "Thermen",
            Self::Trade => "Markt",
            Self::Rice => "Reisgerichte",
            Self::Tea => "Tee",
            Self::Lantern => "Lampions",
            Self::Prayer => "Gebet",
            Self::SmokedFish => "Räucherfisch",
            Self::Textiles => "Webwaren",
            Self::HearthMeal => "Essen am Herd",
            Self::Honey => "Honig",
            Self::DyedCloth => "Gefärbte Stoffe",
            Self::Tablets => "Wachstafeln",
            Self::Casks => "Fässer",
            Self::Dates => "Datteln",
            Self::Spices => "Gewürze",
            Self::Glass => "Glaswaren",
            Self::Incense => "Duftwaren",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    House,
    Well,
    Bakery,
    Grove,
    Potter,
    Stoa,
    Taverna,
    Inn,
    Storehouse,
    Press,
    Shrine,
    Aqueduct,
    Baths,
    Forum,
    Villa,
    HerbGarden,
    RicePaddy,
    RiceKitchen,
    TeaGarden,
    TeaHouse,
    PaperWorkshop,
    Temple,
    Machiya,
    Longhouse,
    Fishery,
    Smokehouse,
    Woodlot,
    Boatyard,
    Weaver,
    FeastHall,
    Apiary,
    Dyer,
    FerryOffice,
    Mill,
    Scribe,
    GardenCourt,
    MarketGarden,
    CeramicStudio,
    SilkShop,
    Sheepfold,
    Cooperage,
    SignalTower,
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
}

impl Kind {
    pub const ALL: [Self; BUILDINGS] = [
        Self::House,
        Self::Well,
        Self::Bakery,
        Self::Grove,
        Self::Potter,
        Self::Stoa,
        Self::Taverna,
        Self::Inn,
        Self::Storehouse,
        Self::Press,
        Self::Shrine,
        Self::Aqueduct,
        Self::Baths,
        Self::Forum,
        Self::Villa,
        Self::HerbGarden,
        Self::RicePaddy,
        Self::RiceKitchen,
        Self::TeaGarden,
        Self::TeaHouse,
        Self::PaperWorkshop,
        Self::Temple,
        Self::Machiya,
        Self::Longhouse,
        Self::Fishery,
        Self::Smokehouse,
        Self::Woodlot,
        Self::Boatyard,
        Self::Weaver,
        Self::FeastHall,
        Self::Apiary,
        Self::Dyer,
        Self::FerryOffice,
        Self::Mill,
        Self::Scribe,
        Self::GardenCourt,
        Self::MarketGarden,
        Self::CeramicStudio,
        Self::SilkShop,
        Self::Sheepfold,
        Self::Cooperage,
        Self::SignalTower,
        Self::OasisHome,
        Self::Cistern,
        Self::DateGarden,
        Self::SpiceBazaar,
        Self::Caravanserai,
        Self::Qanat,
        Self::Glassworks,
        Self::Perfumer,
        Self::MoonGarden,
        Self::CarpetHall,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::House => "Wohnhof",
            Self::Well => "Brunnen",
            Self::Bakery => "Backstube",
            Self::Grove => "Olivenhain",
            Self::Potter => "Töpferei",
            Self::Stoa => "Stoa",
            Self::Taverna => "Taverne",
            Self::Inn => "Gasthaus",
            Self::Storehouse => "Speicher",
            Self::Press => "Ölpresse",
            Self::Shrine => "Heiligtum",
            Self::Aqueduct => "Aquädukt",
            Self::Baths => "Thermen",
            Self::Forum => "Forum",
            Self::Villa => "Villa",
            Self::HerbGarden => "Kräutergarten",
            Self::RicePaddy => "Reisfeld",
            Self::RiceKitchen => "Reisküche",
            Self::TeaGarden => "Teegarten",
            Self::TeaHouse => "Teehaus",
            Self::PaperWorkshop => "Lampionwerkstatt",
            Self::Temple => "Tempel",
            Self::Machiya => "Machiya-Wohnhaus",
            Self::Longhouse => "Langhaus",
            Self::Fishery => "Fischerhütte",
            Self::Smokehouse => "Räucherei",
            Self::Woodlot => "Holzplatz",
            Self::Boatyard => "Bootshaus",
            Self::Weaver => "Weberei",
            Self::FeastHall => "Festhalle",
            Self::Apiary => "Imkerei",
            Self::Dyer => "Färberei",
            Self::FerryOffice => "Lotsenhaus",
            Self::Mill => "Getreidemühle",
            Self::Scribe => "Schreibstube",
            Self::GardenCourt => "Gartenhof",
            Self::MarketGarden => "Gemüsegarten",
            Self::CeramicStudio => "Keramikwerkstatt",
            Self::SilkShop => "Seidenweberei",
            Self::Sheepfold => "Schafweide",
            Self::Cooperage => "Fassbinderei",
            Self::SignalTower => "Signalfeuer",
            Self::OasisHome => "Hofhaus",
            Self::Cistern => "Zisterne",
            Self::DateGarden => "Dattelhain",
            Self::SpiceBazaar => "Gewürzbasar",
            Self::Caravanserai => "Karawanserei",
            Self::Qanat => "Qanat",
            Self::Glassworks => "Glasbläserei",
            Self::Perfumer => "Duftwerkstatt",
            Self::MoonGarden => "Mondgarten",
            Self::CarpetHall => "Teppichhalle",
        }
    }

    pub fn cost(self) -> u32 {
        match self {
            Self::House => 21,
            Self::Well => 13,
            Self::Bakery => 19,
            Self::Grove => 16,
            Self::Potter => 24,
            Self::Stoa => 24,
            Self::Taverna => 31,
            Self::Inn => 34,
            Self::Storehouse => 32,
            Self::Press => 34,
            Self::Shrine => 39,
            Self::Aqueduct => 45,
            Self::Baths => 49,
            Self::Forum => 46,
            Self::Villa => 41,
            Self::HerbGarden => 23,
            Self::RicePaddy => 18,
            Self::RiceKitchen => 23,
            Self::TeaGarden => 22,
            Self::TeaHouse => 28,
            Self::PaperWorkshop => 31,
            Self::Temple => 33,
            Self::Machiya => 26,
            Self::Longhouse => 23,
            Self::Fishery => 20,
            Self::Smokehouse => 25,
            Self::Woodlot => 17,
            Self::Boatyard => 34,
            Self::Weaver => 28,
            Self::FeastHall => 35,
            Self::Apiary => 24,
            Self::Dyer => 29,
            Self::FerryOffice => 34,
            Self::Mill => 22,
            Self::Scribe => 30,
            Self::GardenCourt => 36,
            Self::MarketGarden => 21,
            Self::CeramicStudio => 29,
            Self::SilkShop => 36,
            Self::Sheepfold => 22,
            Self::Cooperage => 31,
            Self::SignalTower => 37,
            Self::OasisHome => 23,
            Self::Cistern => 17,
            Self::DateGarden => 22,
            Self::SpiceBazaar => 25,
            Self::Caravanserai => 35,
            Self::Qanat => 37,
            Self::Glassworks => 33,
            Self::Perfumer => 34,
            Self::MoonGarden => 30,
            Self::CarpetHall => 39,
        }
    }

    pub fn service(self) -> Option<Service> {
        match self {
            Self::Bakery => Some(Service::Bread),
            Self::Potter => Some(Service::Pottery),
            Self::Stoa => Some(Service::Shade),
            Self::Taverna => Some(Service::Meze),
            Self::Inn => Some(Service::Bed),
            Self::Press => Some(Service::Oil),
            Self::Shrine => Some(Service::Offering),
            Self::Baths => Some(Service::Bath),
            Self::Forum => Some(Service::Trade),
            Self::RiceKitchen => Some(Service::Rice),
            Self::TeaHouse => Some(Service::Tea),
            Self::PaperWorkshop => Some(Service::Lantern),
            Self::Temple => Some(Service::Prayer),
            Self::Smokehouse => Some(Service::SmokedFish),
            Self::Weaver => Some(Service::Textiles),
            Self::FeastHall => Some(Service::HearthMeal),
            Self::Apiary => Some(Service::Honey),
            Self::Dyer => Some(Service::DyedCloth),
            Self::Scribe => Some(Service::Tablets),
            Self::GardenCourt => Some(Service::Shade),
            Self::CeramicStudio => Some(Service::Pottery),
            Self::SilkShop => Some(Service::Textiles),
            Self::Cooperage => Some(Service::Casks),
            Self::DateGarden => Some(Service::Dates),
            Self::SpiceBazaar => Some(Service::Spices),
            Self::Caravanserai => Some(Service::Bed),
            Self::Glassworks => Some(Service::Glass),
            Self::Perfumer => Some(Service::Incense),
            Self::MoonGarden => Some(Service::Shade),
            Self::CarpetHall => Some(Service::Textiles),
            _ => None,
        }
    }

    pub fn water_user(self) -> bool {
        matches!(
            self,
            Self::Bakery
                | Self::Potter
                | Self::Inn
                | Self::Baths
                | Self::RiceKitchen
                | Self::FeastHall
                | Self::Dyer
                | Self::GardenCourt
                | Self::CeramicStudio
                | Self::DateGarden
                | Self::Caravanserai
                | Self::Glassworks
                | Self::Perfumer
                | Self::MoonGarden
        )
    }

    pub fn water_source(self) -> bool {
        matches!(
            self,
            Self::Well | Self::Aqueduct | Self::Cistern | Self::Qanat
        )
    }

    /// Places where guests sit down: shade halls and gardens, baths, tea and prayer.
    pub fn rest_stop(self) -> bool {
        matches!(
            self.service(),
            Some(Service::Shade | Service::Bath | Service::Tea | Service::Prayer)
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Price {
    Fair,
    #[default]
    Regular,
    Fine,
}

impl Price {
    pub const ALL: [Self; 3] = [Self::Fair, Self::Regular, Self::Fine];

    pub fn label(self) -> &'static str {
        match self {
            Self::Fair => "Günstig",
            Self::Regular => "Normal",
            Self::Fine => "Hoch",
        }
    }

    pub fn coins(self, service: Service) -> u32 {
        let base = match service {
            Service::Bread => 3,
            Service::Pottery => 5,
            Service::Shade => 3,
            Service::Meze => 5,
            Service::Bed => 6,
            Service::Oil => 6,
            Service::Offering => 6,
            Service::Bath => 7,
            Service::Trade | Service::Lantern => 5,
            Service::Rice | Service::Tea => 4,
            Service::SmokedFish | Service::Textiles => 6,
            Service::HearthMeal => 7,
            Service::Honey => 5,
            Service::DyedCloth => 6,
            Service::Tablets => 6,
            Service::Casks => 7,
            Service::Dates => 4,
            Service::Spices => 6,
            Service::Glass => 7,
            Service::Incense => 6,
            Service::Prayer => return 0,
        };
        match self {
            Self::Fair => base - 1,
            Self::Regular => base,
            Self::Fine => base + 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Age {
    Child,
    Adult,
    Senior,
}

impl Age {
    pub const ALL: [Self; 3] = [Self::Child, Self::Adult, Self::Senior];

    pub const fn stage(self) -> usize {
        match self {
            Self::Adult => 0,
            Self::Child => 2,
            Self::Senior => 4,
        }
    }

    pub fn available(self, chapter: usize) -> bool {
        harbor_stage(chapter) >= self.stage()
    }

    pub fn on_island(self, chapter: usize) -> Self {
        if self.available(chapter) {
            self
        } else {
            Self::Adult
        }
    }

    pub fn guest(index: usize) -> Self {
        match index % 7 {
            0 | 4 => Self::Child,
            3 | 6 => Self::Senior,
            _ => Self::Adult,
        }
    }

    pub fn group(self) -> &'static str {
        match self {
            Self::Child => "Kinder",
            Self::Adult => "Erwachsene",
            Self::Senior => "Ältere Gäste",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Child => "Kind",
            Self::Adult => "Erwachsener Gast",
            Self::Senior => "Älterer Gast",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Child => "age_child",
            Self::Adult => "age_adult",
            Self::Senior => "age_senior",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Assortment {
    #[default]
    Standard,
    Colorful,
    Hearty,
    Classic,
    Festival,
    Market,
    Signature,
    HouseSpecial,
    Celebration,
}

impl Assortment {
    pub const ALL: [Self; 9] = [
        Self::Standard,
        Self::Colorful,
        Self::Hearty,
        Self::Classic,
        Self::Festival,
        Self::Market,
        Self::Signature,
        Self::HouseSpecial,
        Self::Celebration,
    ];

    pub fn unlocked(self, level: u8) -> bool {
        self.tier() <= level
    }

    pub fn tier(self) -> u8 {
        match self {
            Self::Festival => 2,
            Self::Market => 3,
            Self::Signature => 4,
            Self::HouseSpecial => 5,
            Self::Celebration => 6,
            _ => 1,
        }
    }

    pub fn standard(&self) -> bool {
        *self == Self::Standard
    }

    pub fn supported(kind: Kind) -> bool {
        matches!(
            kind,
            Kind::Bakery
                | Kind::Taverna
                | Kind::Potter
                | Kind::RiceKitchen
                | Kind::TeaHouse
                | Kind::PaperWorkshop
                | Kind::Smokehouse
                | Kind::Weaver
                | Kind::FeastHall
                | Kind::Apiary
                | Kind::Dyer
                | Kind::Scribe
                | Kind::CeramicStudio
                | Kind::SilkShop
                | Kind::Cooperage
                | Kind::SpiceBazaar
                | Kind::Glassworks
                | Kind::Perfumer
                | Kind::CarpetHall
        )
    }

    pub fn flavor(self) -> Option<Age> {
        match self {
            Self::Standard => None,
            Self::Colorful | Self::Festival | Self::Celebration => Some(Age::Child),
            Self::Hearty | Self::Market | Self::HouseSpecial => Some(Age::Adult),
            Self::Classic | Self::Signature => Some(Age::Senior),
        }
    }

    pub fn extra_coins(self) -> u32 {
        match self {
            Self::Standard => 0,
            Self::Colorful => 3,
            Self::Hearty => 2,
            Self::Classic => 3,
            Self::Festival => 5,
            Self::Market => 4,
            Self::Signature => 6,
            Self::HouseSpecial => 6,
            Self::Celebration => 7,
        }
    }

    pub fn label(self, kind: Kind) -> &'static str {
        let offers = match kind {
            Kind::Bakery => [
                "Landbrot",
                "Honigfladen",
                "Olivenlaib",
                "Kräuterbrot",
                "Feigenkringel",
                "Sesamlaib",
                "Rosmarinbrot",
                "Mohnlaib",
                "Festkringel",
            ],
            Kind::Potter => [
                "Schlichte Schalen",
                "Bunte Becher",
                "Vorratskrüge",
                "Blaue Schalen",
                "Tierbecher",
                "Glasurkrüge",
                "Reliefschalen",
                "Marktkrüge",
                "Tierpokale",
            ],
            Kind::Taverna => [
                "Tagesgericht",
                "Honigobst",
                "Oliventeller",
                "Kräutereintopf",
                "Dattelkuchen",
                "Markt-Meze",
                "Linsen auf Salbei",
                "Meze des Hauses",
                "Honigfest",
            ],
            Kind::RiceKitchen => [
                "Onigiri",
                "Süßer Reis",
                "Reis mit Miso",
                "Sesamreis",
                "Bunte Reisbällchen",
                "Reis mit Pilzen",
                "Reis mit Nori",
                "Gemüsereis",
                "Fest-Onigiri",
            ],
            Kind::TeaHouse => [
                "Grüntee",
                "Gerösteter Reis",
                "Kräftiger Tee",
                "Milder Tee",
                "Tee mit Reiskuchen",
                "Gerösteter Tee",
                "Frühlingsblätter",
                "Tee des Hauses",
                "Tee mit Dango",
            ],
            Kind::PaperWorkshop => [
                "Papierlampion",
                "Tierlampion",
                "Reiselampion",
                "Mondlampion",
                "Blütenlampion",
                "Marktlampion",
                "Laterne aus Washi",
                "Faltlaterne",
                "Festlampion",
            ],
            Kind::Smokehouse => [
                "Räucherfisch",
                "Fischhäppchen",
                "Wacholderfisch",
                "Fisch am Eichenrauch",
                "Kleine Fischspieße",
                "Fischbrot",
                "Lang geräucherter Fisch",
                "Fjordplatte",
                "Bunte Spieße",
            ],
            Kind::Weaver => [
                "Leinenband",
                "Bunte Bänder",
                "Wollschal",
                "Gemustertes Leinen",
                "Farbenfroher Gürtel",
                "Wolltuch",
                "Handgewebter Mantel",
                "Festliche Schärpe",
                "Flechtband in drei Farben",
            ],
            Kind::FeastHall => [
                "Eintopf vom Herd",
                "Beerenbrei",
                "Fischsuppe",
                "Kräutereintopf",
                "Honiggrütze",
                "Räucherfischplatte",
                "Herdtopf mit Wurzeln",
                "Wintermahl",
                "Festbrei mit Beeren",
            ],
            Kind::Apiary => [
                "Blütenhonig",
                "Honigfladen",
                "Thymianhonig",
                "Pinienhonig",
                "Honigobst",
                "Honig mit Feigen",
                "Dunkler Waldhonig",
                "Waben vom Hang",
                "Festgläser",
            ],
            Kind::Dyer => [
                "Naturfarben",
                "Bunte Bänder",
                "Indigotuch",
                "Krappfärbung",
                "Farbfäden",
                "Purpurstreifen",
                "Wurzelfarben",
                "Hafenblau",
                "Feststoffe",
            ],
            Kind::Scribe => [
                "Wachstafel",
                "Bildtafel",
                "Handelsbuch",
                "Brief auf Papyrus",
                "Spieltafel",
                "Vertragsrolle",
                "Kopie eines Gedichts",
                "Stadtplan",
                "Festprogramm",
            ],
            Kind::CeramicStudio => [
                "Schlichte Teeschale",
                "Bemaltes Schälchen",
                "Reisschale",
                "Glasurbecher",
                "Kleine Tierfigur",
                "Töpferset",
                "Rauchglasur",
                "Teekanne",
                "Feine Festschale",
            ],
            Kind::SilkShop => [
                "Leinenband",
                "Blütenband",
                "Seidentuch",
                "Indigoschal",
                "Bunte Haarbänder",
                "Vorratsbeutel",
                "Gewebter Obi",
                "Reisehaori",
                "Feststoff",
            ],
            Kind::Cooperage => [
                "Kleines Fass",
                "Korb mit Deckel",
                "Reisefass",
                "Salzfass",
                "Bunter Eimer",
                "Fischkiste",
                "Lagerfass",
                "Fjordfass",
                "Festtonne",
            ],
            Kind::SpiceBazaar => [
                "Kräutersalz",
                "Zimtzucker",
                "Kardamom",
                "Kümmelmischung",
                "Bunte Gewürzdosen",
                "Gerösteter Koriander",
                "Milde Kurkuma",
                "Safranmischung",
                "Gewürztruhe",
            ],
            Kind::Glassworks => [
                "Trinkglas",
                "Türkisglas",
                "Reiseflasche",
                "Ölfläschchen",
                "Glasvogel",
                "Bauchige Karaffe",
                "Graviertes Glas",
                "Rubinbecher",
                "Gläsernes Sternenlicht",
            ],
            Kind::Perfumer => [
                "Rosenwasser",
                "Süße Orangenblüte",
                "Jasminöl",
                "Mildes Zedernöl",
                "Blütenbeutel",
                "Bergamotteöl",
                "Weihrauch",
                "Nachtjasmin",
                "Dufttruhe",
            ],
            Kind::CarpetHall => [
                "Schlichte Matte",
                "Tiermuster",
                "Reiseteppich",
                "Knotenkissen",
                "Sternenteppich",
                "Gestreifter Läufer",
                "Klassischer Webteppich",
                "Mondläufer",
                "Festteppich",
            ],
            _ => return "Grundangebot",
        };
        offers[self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Building {
    pub kind: Kind,
    pub level: u8,
    pub price: Price,
    #[serde(default, skip_serializing_if = "Assortment::standard")]
    pub assortment: Assortment,
}

impl Building {
    pub const fn new(kind: Kind) -> Self {
        Self {
            kind,
            level: 1,
            price: Price::Regular,
            assortment: Assortment::Standard,
        }
    }

    pub fn upgrade_cost(self) -> u32 {
        self.kind.cost() / 2 + 4 * u32::from(self.level)
    }

    pub fn refund(self) -> u32 {
        let investments: u32 = (1..self.level)
            .map(|level| Self { level, ..self }.upgrade_cost())
            .sum();
        (self.kind.cost() + investments) / 2
    }

    pub fn residents(self) -> u16 {
        match self.kind {
            Kind::House | Kind::Machiya | Kind::OasisHome => 2 + u16::from(self.level),
            Kind::Villa => 2 + u16::from(self.level) * 2,
            Kind::Longhouse => 3 + u16::from(self.level),
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Cloud,
    Wind,
    Heat,
    Feast,
    Rain,
    Storm,
}

impl Weather {
    pub fn name(self) -> &'static str {
        match self {
            Self::Clear => "Sonne",
            Self::Cloud => "Wolken",
            Self::Storm => "Sturm",
            Self::Wind => "Wind",
            Self::Heat => "Hitze",
            Self::Feast => "Festtag",
            Self::Rain => "Regen",
        }
    }

    pub fn ferry_change(self) -> i16 {
        match self {
            Self::Wind => -2,
            Self::Rain => -3,
            Self::Storm => -5,
            Self::Feast => 4,
            _ => 0,
        }
    }

    pub fn swell(self) -> f32 {
        match self {
            Self::Clear | Self::Heat => 0.18,
            Self::Cloud | Self::Feast => 0.32,
            Self::Rain => 0.58,
            Self::Wind => 0.78,
            Self::Storm => 1.0,
        }
    }

    pub fn rainfall(self) -> u16 {
        match self {
            Self::Rain => 4,
            Self::Storm => 5,
            _ => 0,
        }
    }

    pub fn brightness(self) -> f32 {
        match self {
            Self::Clear | Self::Feast => 1.0,
            Self::Heat => 1.12,
            Self::Wind => 0.89,
            Self::Cloud => 0.72,
            Self::Rain => 0.55,
            Self::Storm => 0.41,
        }
    }

    pub fn rain_strength(self) -> f32 {
        match self {
            Self::Rain => 0.62,
            Self::Storm => 1.0,
            _ => 0.0,
        }
    }
}

pub const WEATHER_PHASES: [f32; 3] = [0.0, 0.30, 0.71];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Favor {
    Jugs,
    Shade,
    Cargo,
    Fire,
    Crates,
    BrokenJars,
    ShadeSail,
    LostGuests,
}

impl Favor {
    pub fn name(self) -> &'static str {
        match self {
            Self::Jugs => "Wasser verteilen",
            Self::Shade => "Schatten spenden",
            Self::Cargo => "Ware vom Kai holen",
            Self::Fire => "Ofenbrand löschen",
            Self::Crates => "Verrutschte Kisten sichern",
            Self::BrokenJars => "Scherben zusammenkehren",
            Self::ShadeSail => "Sonnensegel befestigen",
            Self::LostGuests => "Verirrte Reisegruppe begleiten",
        }
    }

    pub fn note(self) -> &'static str {
        match self {
            Self::Jugs => "2 Portionen mehr für Läden in der Nähe",
            Self::Shade => "2 Plätze mehr an jedem Ort zum Ausruhen",
            Self::Cargo => "3 Portionen für einen Laden",
            Self::Fire => "Nach dem Löschen kommen 3 Portionen dazu.",
            Self::Crates => "3 gerettete Portionen für einen Laden",
            Self::BrokenJars => "Räumt die Gasse frei und rettet 2 Portionen.",
            Self::ShadeSail => "5 zusätzliche Plätze im Schatten",
            Self::LostGuests => "2 Gäste finden mit Nili ans Ziel.",
        }
    }

    pub(crate) fn job(self) -> Job {
        match self {
            Self::Jugs | Self::Fire => Job::Water,
            Self::Shade | Self::ShadeSail => Job::Shade,
            Self::Cargo | Self::Crates => Job::Freight,
            Self::BrokenJars => Job::Quay,
            Self::LostGuests => Job::Welcome,
        }
    }

    pub fn allows(self, kind: Kind) -> bool {
        match self {
            Self::Jugs => kind.water_source(),
            Self::Shade => matches!(
                kind,
                Kind::Grove
                    | Kind::Stoa
                    | Kind::Baths
                    | Kind::TeaGarden
                    | Kind::Temple
                    | Kind::DateGarden
                    | Kind::MoonGarden
            ),
            Self::Cargo | Self::Crates => kind.service().is_some_and(Service::priced),
            Self::Fire => matches!(
                kind,
                Kind::Bakery
                    | Kind::Potter
                    | Kind::RiceKitchen
                    | Kind::PaperWorkshop
                    | Kind::Smokehouse
                    | Kind::Glassworks
            ),
            Self::BrokenJars => matches!(
                kind,
                Kind::Potter
                    | Kind::Taverna
                    | Kind::Press
                    | Kind::TeaHouse
                    | Kind::Glassworks
                    | Kind::Perfumer
            ),
            Self::ShadeSail => matches!(
                kind,
                Kind::Stoa
                    | Kind::Taverna
                    | Kind::Baths
                    | Kind::TeaHouse
                    | Kind::Weaver
                    | Kind::SpiceBazaar
                    | Kind::CarpetHall
            ),
            Self::LostGuests => {
                matches!(
                    kind,
                    Kind::Inn
                        | Kind::Taverna
                        | Kind::Forum
                        | Kind::Stoa
                        | Kind::Temple
                        | Kind::TeaHouse
                        | Kind::Caravanserai
                        | Kind::MoonGarden
                        | Kind::FeastHall
                )
            }
        }
    }

    pub fn incident(self) -> bool {
        matches!(
            self,
            Self::Fire | Self::Crates | Self::BrokenJars | Self::ShadeSail | Self::LostGuests
        )
    }

    pub fn event_name(self) -> &'static str {
        match self {
            Self::Fire => "Ofenbrand",
            Self::Crates => "Lose Kisten",
            Self::BrokenJars => "Zerbrochene Krüge",
            Self::ShadeSail => "Loses Sonnensegel",
            Self::LostGuests => "Verirrte Reisegruppe",
            _ => self.name(),
        }
    }

    pub fn resolved_name(self) -> &'static str {
        match self {
            Self::Fire => "Feuer gelöscht",
            Self::Crates => "Kisten gesichert",
            Self::BrokenJars => "Die Gasse ist frei",
            Self::ShadeSail => "Das Sonnensegel hält",
            Self::LostGuests => "Reisegruppe angekommen",
            _ => "Bitte vom Kai erfüllt",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Jugs => "water",
            Self::Shade => "grove",
            Self::Cargo | Self::Crates => "storehouse",
            Self::Fire => "fire",
            Self::BrokenJars => "scraps",
            Self::ShadeSail => "wind",
            Self::LostGuests => "guest_steps",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    Home(u8),
    Ferry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Miss {
    NoShop,
    Walk,
    Price,
    Empty,
}

impl Miss {
    pub const ALL: [Self; 4] = [Self::NoShop, Self::Walk, Self::Price, Self::Empty];

    pub fn label(self) -> &'static str {
        match self {
            Self::NoShop => "Angebot fehlt",
            Self::Walk => "Weg zu weit",
            Self::Price => "Zu teuer",
            Self::Empty => "Ausverkauft",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::NoShop => "Baue einen Laden mit diesem Angebot.",
            Self::Walk => "Baue näher bei den Wohnhöfen oder am Kai.",
            Self::Price => "Senk morgen den Preis.",
            Self::Empty => "Schick Nili mit Nachschub oder baue morgen aus.",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Visit {
    pub origin: Origin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<Age>,
    #[serde(default)]
    pub specialty_bonus: u32,
    pub wants: Service,
    pub budget: Price,
    pub at: f32,
    pub resolved: bool,
    pub shop: Option<u8>,
    #[serde(default)]
    pub miss: Option<Miss>,
    #[serde(default)]
    pub guided_to: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declined_at: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Pleased,
    Missed(Miss),
}

impl Visit {
    pub fn age(&self, index: usize) -> Age {
        self.age.unwrap_or_else(|| Age::guest(index))
    }

    pub fn verdict(&self) -> Option<Verdict> {
        if !self.resolved {
            return None;
        }
        if self.shop.is_some() {
            return Some(Verdict::Pleased);
        }
        self.miss.map(Verdict::Missed)
    }
}

pub enum GuestTip {
    Shop(usize),
    Build(usize, Kind),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Tally {
    #[serde(deserialize_with = "read_service_counts")]
    pub served: [u32; SERVICES],
    #[serde(deserialize_with = "read_service_counts")]
    pub missed: [u32; SERVICES],
    #[serde(default, deserialize_with = "read_service_counts")]
    pub produced: [u32; SERVICES],
    #[serde(default)]
    pub dogs: u32,
    #[serde(default)]
    pub cleaned: u32,
    #[serde(default)]
    pub guests_by_age: [GuestCount; 3],
    #[serde(default)]
    pub best_care: u32,
    #[serde(default)]
    pub best_favorite_menu: [u32; 3],
    #[serde(default)]
    pub gaps: [u32; 4],
    pub income: u32,
    pub reputation: u32,
    pub favors: u32,
    #[serde(default)]
    pub feast_peak: u32,
    #[serde(default)]
    pub widest_day: u32,
    #[serde(default)]
    pub record_income: u32,
    #[serde(default)]
    pub peak_water: u32,
    #[serde(default)]
    pub routes: [u32; JOB_TYPES],
    #[serde(default)]
    pub ferry_served: u32,
    #[serde(default)]
    pub best_ferry_day: u32,
    #[serde(default)]
    pub best_mood: u32,
    #[serde(default)]
    pub street_bonus: u32,
    #[serde(default)]
    pub specialties: [u32; 3],
    #[serde(default)]
    pub specialty_income: u32,
    #[serde(default)]
    pub harvest_peak: u32,
    #[serde(default)]
    pub bell_rings: u32,
    #[serde(default)]
    pub catch_peak: u32,
    #[serde(default)]
    pub voyages: u32,
    #[serde(default)]
    pub cargo_income: u32,
    #[serde(default)]
    pub caravans: u32,
    #[serde(default)]
    pub caravan_income: u32,
    #[serde(default)]
    pub spice_routes: u32,
    #[serde(default)]
    pub best_streak: u32,
    #[serde(default)]
    pub streak_bonus: u32,
}

impl Tally {
    pub fn care(&self, age: Age, minimum: u32) -> u32 {
        let count = self.guests_by_age[age as usize];
        if count.served >= minimum {
            satisfaction_percent(count.served, count.responses).unwrap_or(0)
        } else {
            0
        }
    }

    pub fn favorite_variety(&self, age: Age) -> u32 {
        self.guests_by_age[age as usize].favorites.count_ones()
    }

    pub fn routes_today(&self) -> u32 {
        self.routes.iter().sum()
    }

    pub fn served_total(&self) -> u32 {
        self.served.iter().sum()
    }

    pub fn responses(&self) -> u32 {
        self.served_total() + self.missed.iter().sum::<u32>()
    }

    pub fn satisfaction(&self) -> Option<u32> {
        satisfaction_percent(self.served_total(), self.responses())
    }

    fn add(&mut self, other: &Self) {
        for service in Service::ALL {
            let i = service as usize;
            self.served[i] += other.served[i];
            self.missed[i] += other.missed[i];
            self.produced[i] += other.produced[i];
        }
        self.dogs += other.dogs;
        self.cleaned += other.cleaned;
        self.best_care = self.best_care.max(other.best_care);
        for (best, current) in self
            .best_favorite_menu
            .iter_mut()
            .zip(other.best_favorite_menu)
        {
            *best = (*best).max(current);
        }
        self.income += other.income;
        self.reputation += other.reputation;
        self.favors += other.favors;
        for (gap, added) in self.gaps.iter_mut().zip(other.gaps) {
            *gap += added;
        }
        self.feast_peak = self.feast_peak.max(other.feast_peak);
        self.widest_day = self.widest_day.max(other.widest_day);
        self.record_income = self.record_income.max(other.record_income);
        self.peak_water = self.peak_water.max(other.peak_water);
        for (total, count) in self.routes.iter_mut().zip(other.routes) {
            *total += count;
        }
        self.ferry_served += other.ferry_served;
        self.best_ferry_day = self.best_ferry_day.max(other.best_ferry_day);
        self.best_mood = self.best_mood.max(other.best_mood);
        self.street_bonus += other.street_bonus;
        self.specialty_income += other.specialty_income;
        self.caravans += other.caravans;
        self.caravan_income += other.caravan_income;
        self.spice_routes += other.spice_routes;
        self.best_streak = self.best_streak.max(other.best_streak);
        self.streak_bonus += other.streak_bonus;
        self.harvest_peak = self.harvest_peak.max(other.harvest_peak);
        self.bell_rings += other.bell_rings;
        self.catch_peak = self.catch_peak.max(other.catch_peak);
        for (total, added) in self.specialties.iter_mut().zip(other.specialties) {
            *total += added;
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct GuestCount {
    pub served: u32,
    pub responses: u32,
    favorites: u32,
}

pub fn satisfaction_percent(served: u32, total: u32) -> Option<u32> {
    let total = u64::from(total);
    (total > 0).then(|| ((u64::from(served) * 100 + total / 2) / total) as u32)
}

fn read_service_counts<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<[u32; SERVICES], D::Error> {
    let values = Vec::<u32>::deserialize(deserializer)?;
    let mut result = [0; SERVICES];
    for (target, value) in result.iter_mut().zip(values) {
        *target = value;
    }
    Ok(result)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Day {
    pub seconds: f32,
    #[serde(default)]
    pub water: u16,
    #[serde(default)]
    pub caravan_water: u16,
    #[serde(deserialize_with = "extend_plot_array")]
    pub stock: [u16; LOTS],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplied: Option<[u16; LOTS]>,
    pub visits: Vec<Visit>,
    pub tally: Tally,
    #[serde(default)]
    sales: SalesProgress,
    pub favor: Favor,
    pub favor_at: Option<u8>,
    #[serde(default)]
    pub nili_at: Stop,
    #[serde(default)]
    pub route: Option<Route>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nili: Option<NiliControl>,
    #[serde(default)]
    pub street_claims: u8,
    #[serde(default)]
    pub rice_harvest: u16,
    #[serde(default)]
    pub rung_bells: u32,
    #[serde(default)]
    pub fish_catch: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Gathering>,
    /// Guests served in a row without anyone leaving unhappy.
    #[serde(default)]
    pub streak: u16,
    /// The incident was solved quickly enough for the extra reward.
    #[serde(default)]
    pub favor_rushed: bool,
}

impl Day {
    pub fn stock_capacity(&self, lot: usize) -> u16 {
        self.supplied.as_ref().map_or_else(
            || {
                let served = self
                    .visits
                    .iter()
                    .filter(|visit| visit.shop == Some(lot as u8))
                    .count();
                self.stock[lot].saturating_add(served as u16)
            },
            |supplied| supplied[lot].max(self.stock[lot]),
        )
    }

    fn restock(&mut self, lot: usize, amount: u16) {
        self.stock[lot] += amount;
        if let Some(supplied) = &mut self.supplied {
            supplied[lot] += amount;
        }
    }

    pub fn manual_route(&self) -> Option<(Route, NiliControl)> {
        Some((self.route?, self.nili?))
    }

    pub fn route_pose(&self) -> Option<(Route, (RouteStage, f32))> {
        let route = self.route?;
        let pose = self
            .nili
            .map_or_else(|| route.stage(self.seconds), |nili| nili.stage.animation());
        Some((route, pose))
    }

    pub fn errand_title(&self, route: Route) -> &'static str {
        if route.favor {
            self.favor.name()
        } else {
            route.job.name()
        }
    }

    pub fn errand_icon(&self, route: Route) -> &'static str {
        if route.favor {
            self.favor.icon()
        } else {
            route.job.icon()
        }
    }

    pub fn errand_action(&self, route: Route, stage: ManualStage) -> Option<&'static str> {
        if route.favor {
            match (self.favor, stage) {
                (Favor::Fire, ManualStage::Delivery) => return Some("Feuer löschen"),
                (Favor::BrokenJars, ManualStage::Pickup) => return Some("Scherben aufkehren"),
                (Favor::BrokenJars, ManualStage::Delivery) => {
                    return Some("Scherben in den Korb legen");
                }
                (Favor::ShadeSail, ManualStage::Pickup) => return Some("Seil holen"),
                (Favor::ShadeSail, ManualStage::Delivery) => return Some("Segel festbinden"),
                _ => {}
            }
        }
        stage.action(route.job)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub tally: Tally,
    pub grant: u32,
    pub stars: u8,
    #[serde(default)]
    pub contract_bonus: u32,
    /// Day records broken today, one bit per [`DayRecord`].
    #[serde(default)]
    pub records: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayRecord {
    Income,
    Variety,
    Ferry,
    Mood,
}

impl DayRecord {
    pub const ALL: [Self; 4] = [Self::Income, Self::Variety, Self::Ferry, Self::Mood];

    pub fn label(self) -> &'static str {
        match self {
            Self::Income => "Einnahmen",
            Self::Variety => "Vielfalt",
            Self::Ferry => "Fährgäste",
            Self::Mood => "Zufriedenheit",
        }
    }

    pub fn bit(self) -> u8 {
        1 << self as u8
    }

    /// Which of today's figures beat every earlier day in this harbor.
    fn broken(today: &Tally, before: &Tally) -> u8 {
        [
            (Self::Income, today.record_income > before.record_income),
            (Self::Variety, today.widest_day > before.widest_day),
            (Self::Ferry, today.best_ferry_day > before.best_ferry_day),
            (Self::Mood, today.best_mood > before.best_mood),
        ]
        .into_iter()
        .filter(|&(_, beaten)| beaten)
        .fold(0, |records, (record, _)| records | record.bit())
    }

    pub fn list(records: u8) -> impl Iterator<Item = Self> {
        Self::ALL
            .into_iter()
            .filter(move |record| records & record.bit() != 0)
    }
}

/// A themed group that visits the harbor on some days. It only ever adds guests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayEvent {
    TradeShip,
    Wedding,
    SchoolTrip,
    Pilgrims,
}

/// Guests one day event brings at most.
pub const EVENT_GUESTS: usize = 3;
/// Every this many happy guests in a row earn one point of reputation.
pub const STREAK_STEP: u16 = 5;

impl DayEvent {
    pub const ALL: [Self; 4] = [
        Self::TradeShip,
        Self::Wedding,
        Self::SchoolTrip,
        Self::Pilgrims,
    ];

    pub fn title(self, region: Region) -> &'static str {
        use Region::*;
        match (self, region) {
            (Self::TradeShip, Greece) => "Handelsschiff aus Tyros",
            (Self::TradeShip, Rome) => "Getreideschiff aus Alexandria",
            (Self::TradeShip, Japan) => "Kaufmannsschiff aus Sakai",
            (Self::TradeShip, Nordic) => "Knorr aus Birka",
            (Self::TradeShip, Oasis) => "Händlerkarawane aus Palmyra",
            (Self::Wedding, Japan) => "Hochzeit am Schrein",
            (Self::Wedding, Nordic) => "Hochzeit im Langhaus",
            (Self::Wedding, _) => "Hochzeit in der Gasse",
            (Self::SchoolTrip, _) => "Schulausflug",
            (Self::Pilgrims, Greece) => "Pilger auf dem Weg nach Delphi",
            (Self::Pilgrims, Rome) => "Pilger auf dem Weg nach Ephesos",
            (Self::Pilgrims, Japan) => "Pilger auf dem Weg nach Ise",
            (Self::Pilgrims, Nordic) => "Pilger auf dem Weg nach Nidaros",
            (Self::Pilgrims, Oasis) => "Pilger auf dem Weg zur Quelle",
        }
    }

    pub fn note(self) -> &'static str {
        match self {
            Self::TradeShip => "Gäste mit vollem Beutel suchen etwas Seltenes.",
            Self::Wedding => "Die Hochzeitsgäste feiern und suchen etwas zu essen.",
            Self::SchoolTrip => "Eine Schulklasse kommt mit der Fähre.",
            Self::Pilgrims => "Ältere Reisende suchen Ruhe und Rast.",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::TradeShip => "ferry",
            Self::Wedding => "house",
            Self::SchoolTrip => Age::Child.icon(),
            Self::Pilgrims => Age::Senior.icon(),
        }
    }

    fn fits(self, island: usize) -> bool {
        match self {
            Self::SchoolTrip => Age::Child.available(island),
            Self::Pilgrims => Age::Senior.available(island),
            _ => true,
        }
    }

    fn age(self, guest: usize, island: usize) -> Age {
        match self {
            Self::TradeShip => Age::guest(guest + island),
            Self::Wedding => Age::Adult,
            Self::SchoolTrip => Age::Child,
            Self::Pilgrims => Age::Senior,
        }
        .on_island(island)
    }

    /// Services the group asks for first, before any other shop in town.
    fn favorites(self) -> &'static [Service] {
        use Service::*;
        match self {
            Self::TradeShip => &[],
            Self::Wedding => &[Meze, HearthMeal, Rice, Bread, Honey, Tea, Oil],
            Self::SchoolTrip => &[Honey, Bread, Lantern, Pottery, Glass, Textiles],
            Self::Pilgrims => &[Prayer, Offering, Bath, Shade, Tea, Incense],
        }
    }
}

/// The day event as it unfolds: when the group arrives and how many come.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gathering {
    pub event: DayEvent,
    pub at: f32,
    pub guests: u8,
}

#[derive(Clone, Copy)]
pub enum DailyContract {
    Specialty(Age, u32),
    Turnover(u32),
    Variety(u32),
    Street(u32),
}

impl DailyContract {
    pub fn progress(self, tally: &Tally) -> (u32, u32) {
        match self {
            Self::Specialty(age, goal) => (tally.specialties[age as usize], goal),
            Self::Turnover(goal) => (tally.income, goal),
            Self::Variety(goal) => (
                tally.served.iter().filter(|&&sales| sales > 0).count() as u32,
                goal,
            ),
            Self::Street(goal) => (tally.street_bonus, goal),
        }
    }

    pub fn label(self) -> (&'static str, &'static str) {
        match self {
            Self::Specialty(Age::Child, _) => ("Lieblingsangebote für Kinder", "age_child"),
            Self::Specialty(Age::Adult, _) => ("Lieblingsangebote für Erwachsene", "age_adult"),
            Self::Specialty(Age::Senior, _) => ("Lieblingsangebote für ältere Gäste", "age_senior"),
            Self::Turnover(_) => ("Münzen verdienen", "coin"),
            Self::Variety(_) => ("Verschiedene Wünsche erfüllen", "guest_book"),
            Self::Street(_) => ("Ansehen durch Hilfe in der Gasse", "nili"),
        }
    }

    /// Later harbors pay more for the daily extra, since its targets keep growing.
    pub fn reward(self, island: usize) -> u32 {
        5 + harbor_stage(island) as u32 / 2
    }
}

/// An incident solved this soon after it appears earns an extra reward.
pub const RUSH_SECONDS: f32 = 20.0;
pub const RUSH_COINS: u32 = 3;
pub const RUSH_RENOWN: u32 = 2;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Plan,
    Open(Box<Day>),
    Report(Box<Report>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarborPlan {
    #[default]
    Regular,
    Ferry,
    Water,
    Neighbors,
}

impl HarborPlan {
    pub const ALL: [Self; 4] = [Self::Regular, Self::Ferry, Self::Water, Self::Neighbors];

    pub fn name(self) -> &'static str {
        match self {
            Self::Regular => "Normal",
            Self::Ferry => "Fährtag",
            Self::Water => "Wasserfahrt",
            Self::Neighbors => "Nachbarschaft",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Town {
    pub chapter: usize,
    pub day: u32,
    pub coins: u32,
    #[serde(deserialize_with = "extend_plot_array")]
    pub lots: [Option<Building>; LOTS],
    pub totals: Tally,
    #[serde(default)]
    sales: SalesProgress,
    pub phase: Phase,
    #[serde(default)]
    pub harbor: HarborPlan,
    #[serde(default)]
    pub last_day: Option<Tally>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_visits: Vec<Visit>,
    #[serde(default)]
    pub lessons: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voyage: Option<Voyage>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cargo_today: u32,
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

#[derive(Default)]
pub struct Production {
    pub stock: [u16; LOTS],
    pub water: u16,
    pub olives: u16,
    pub used_water: u16,
    pub used_olives: u16,
    pub rice: u16,
    pub used_rice: u16,
    pub fish: u16,
    pub wood: u16,
    pub short_water: [bool; LOTS],
}

pub struct SupplyNeed {
    pub stock: u16,
    pub guests: u16,
    pub short_water: bool,
}

pub struct OfferForecast {
    pub matches: [u16; Assortment::ALL.len()],
    pub extra_coins: [u32; Assortment::ALL.len()],
}

/// What each price would bring one shop today, if nobody restocks it.
pub struct PriceForecast {
    pub sales: [u16; Price::ALL.len()],
    pub coins: [u32; Price::ALL.len()],
    /// Guests who wanted to buy here and left without it.
    pub misses: [u16; Price::ALL.len()],
}

impl PriceForecast {
    /// The price that earns the most. On a tie the one that leaves fewer guests
    /// unhappy, then the cheaper one.
    pub fn best(&self) -> Option<Price> {
        Price::ALL
            .into_iter()
            .rev()
            .max_by_key(|&price| {
                let i = price as usize;
                (self.coins[i], std::cmp::Reverse(self.misses[i]))
            })
            .filter(|&price| self.coins[price as usize] > 0)
    }
}

impl SupplyNeed {
    pub fn shortage(&self) -> u16 {
        self.guests.saturating_sub(self.stock)
    }

    pub fn suggested_job(&self, town: &Town, chapter: &Chapter, lot: usize) -> Option<Job> {
        if self.shortage() == 0 {
            return None;
        }
        [self.short_water.then_some(Job::Water), Some(Job::Freight)]
            .into_iter()
            .flatten()
            .find(|job| job.plan(town, chapter, lot).is_ok())
    }
}

pub fn is_open(chapter: &Chapter, lot: usize) -> bool {
    chapter.layout.contains(lot)
}

fn extend_plot_array<'de, D, T>(deserializer: D) -> Result<[T; LOTS], D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default + Copy,
{
    let values = Vec::<T>::deserialize(deserializer)?;
    if values.len() != crate::layout::ORIGINAL_COLUMNS * 2 && values.len() != LOTS {
        return Err(serde::de::Error::custom("unexpected number of plots"));
    }
    let mut plots = [T::default(); LOTS];
    plots[..values.len()].copy_from_slice(&values);
    Ok(plots)
}

impl Town {
    pub fn daily_contract(&self, chapter: &Chapter) -> Option<DailyContract> {
        if harbor_stage(self.chapter) < 2 || matches!(self.phase, Phase::Report(_)) {
            return None;
        }
        let scale = (chapter.layout.width().saturating_sub(4) / 2) as u32;
        // Yesterday's market sets the bar, so the extra grows with the harbor.
        let none = Tally::default();
        let yesterday = self.last_day.as_ref().unwrap_or(&none);
        let plan = self.visit_plan(chapter);
        let specialty = |age: Age, floor: u32| {
            let expected = plan
                .iter()
                .enumerate()
                .filter(|(index, visit)| visit.age(*index) == age)
                .count() as u32;
            let beat = (yesterday.specialties[age as usize] + 1).min(expected);
            DailyContract::Specialty(age, floor.max(beat))
        };
        Some(match (self.day as usize + self.chapter) % 6 {
            0 => specialty(Age::Child, 1 + scale),
            1 => DailyContract::Turnover((17 + scale * 13).max(yesterday.income * 11 / 10)),
            2 => specialty(Age::Adult, 2 + scale),
            3 => DailyContract::Street(1 + scale.min(1)),
            4 => specialty(Age::Senior.on_island(self.chapter), 1 + scale),
            _ => {
                let offered = Service::ALL
                    .into_iter()
                    .filter(|&service| {
                        plan.iter().any(|visit| visit.wants == service)
                            && self
                                .lots
                                .iter()
                                .flatten()
                                .any(|b| b.kind.service() == Some(service))
                    })
                    .count() as u32;
                let floor = 2 + scale.min(1);
                DailyContract::Variety(floor.max((yesterday.widest_day + 1).min(offered)))
            }
        })
    }

    /// Today's themed visitors, as planned in the morning or as they happen.
    pub fn day_event(&self, chapter: &Chapter) -> Option<Gathering> {
        match &self.phase {
            Phase::Open(day) => day.event,
            Phase::Report(_) => None,
            Phase::Plan => {
                self.planned_event()?;
                self.day_plan(chapter, &self.production(chapter)).event
            }
        }
    }

    fn planned_event(&self) -> Option<DayEvent> {
        let stage = harbor_stage(self.chapter);
        if stage < 2 || self.day < 2 || (self.chapter < crate::coach::ISLANDS && self.day <= 2) {
            return None;
        }
        let seed =
            self.day.wrapping_mul(0x2c1b_3c6d) ^ (self.chapter as u32).wrapping_mul(0x297a_2d39);
        let seed = (seed ^ (seed >> 15)).wrapping_mul(0x846c_a68b);
        let seed = seed ^ (seed >> 16);
        if !seed.is_multiple_of(3) {
            return None;
        }
        let options: Vec<_> = DayEvent::ALL
            .into_iter()
            .filter(|event| event.fits(self.chapter))
            .collect();
        options.get((seed / 3) as usize % options.len()).copied()
    }

    /// The group only asks for things the street can still spare after the regular guests,
    /// so an event day never costs a regular guest their visit.
    fn event_guests(&self, chapter: &Chapter, event: DayEvent, day: &Day) -> Vec<Visit> {
        let mut spare = day.clone();
        resolve_visits(&self.lots, &mut spare, chapter, chapter.day_seconds(), None);
        let mut wishes = [0u16; SERVICES];
        for visit in &day.visits {
            wishes[visit.wants as usize] += 1;
        }
        // Trade goods stay reserved for ships and caravans.
        let reserved = [Service::SmokedFish, Service::Dates, Service::Spices];
        let mut offered: Vec<_> = Service::ALL
            .into_iter()
            .filter(|service| !reserved.contains(service))
            .filter(|&service| {
                self.lots
                    .iter()
                    .flatten()
                    .any(|b| b.kind.service() == Some(service))
            })
            .collect();
        offered.sort_by_key(|&service| {
            let favorite = event.favorites().iter().position(|&s| s == service);
            (
                favorite.unwrap_or(usize::MAX),
                wishes[service as usize],
                std::cmp::Reverse(Price::Regular.coins(service)),
            )
        });
        let homes: Vec<_> = self
            .lots
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_some_and(|b| b.residents() > 0))
            .map(|(lot, _)| lot as u8)
            .collect();
        let closing = chapter.day_seconds();
        let slot = (self.day as usize * 7 + self.chapter * 3) % 5;
        let arrival = closing * (0.42 + slot as f32 * 0.05);
        let mut guests = Vec::new();
        for n in 0..EVENT_GUESTS {
            let origin = match (event, homes.is_empty()) {
                (DayEvent::Wedding, false) => Origin::Home(homes[n % homes.len()]),
                _ => Origin::Ferry,
            };
            let mut visit = Visit {
                origin,
                age: Some(event.age(n + self.day as usize, self.chapter)),
                specialty_bonus: 0,
                wants: Service::Bread,
                budget: Price::Fine,
                at: arrival + guests.len() as f32 * 1.2,
                resolved: false,
                shop: None,
                miss: None,
                guided_to: None,
                declined_at: None,
            };
            for &service in &offered {
                visit.wants = service;
                if let Ok(shop) = target(&self.lots, &spare, &visit, chapter) {
                    spare.stock[shop] -= 1;
                    guests.push(visit);
                    break;
                }
            }
        }
        guests
    }

    pub fn new(chapter: usize, rule: &Chapter) -> Self {
        let mut lots = [None; LOTS];
        for &(lot, kind) in rule.starts {
            lots[lot] = Some(Building::new(kind));
        }
        Self {
            chapter,
            day: 1,
            coins: rule.coins,
            lots,
            totals: Tally::default(),
            sales: SalesProgress::default(),
            phase: Phase::Plan,
            harbor: HarborPlan::Regular,
            last_day: None,
            last_visits: Vec::new(),
            lessons: 0,
            voyage: None,
            cargo_today: 0,
        }
    }

    pub fn weather(&self, chapter: &Chapter) -> Weather {
        self.weather_ahead(chapter, 0)
    }

    pub fn weather_ahead(&self, chapter: &Chapter, days: usize) -> Weather {
        let day = self.day.saturating_sub(1) as usize + days;
        let pattern = if chapter.weather.len() == 1 {
            &[Weather::Clear, Weather::Rain, Weather::Wind, Weather::Cloud][..]
        } else {
            chapter.weather
        };
        pattern[day % pattern.len()]
    }

    pub fn weather_schedule(&self, chapter: &Chapter) -> [Weather; 3] {
        use Weather::*;
        let alternate = (self.day as usize + self.chapter).is_multiple_of(2);
        match self.weather(chapter) {
            Clear if alternate => [Cloud, Clear, Wind],
            Clear => [Clear, Cloud, Clear],
            Cloud => [Clear, Cloud, if alternate { Rain } else { Wind }],
            Heat => [Clear, Heat, if alternate { Wind } else { Cloud }],
            Wind => [if alternate { Cloud } else { Clear }, Wind, Cloud],
            Rain => [Cloud, Rain, if alternate { Wind } else { Clear }],
            Storm => [Rain, Storm, Cloud],
            Feast => [Cloud, Feast, Clear],
        }
    }

    pub fn sky_phase(&self, chapter: &Chapter) -> usize {
        let progress = match &self.phase {
            Phase::Plan => 0.0,
            Phase::Open(day) => day.seconds / chapter.day_seconds(),
            Phase::Report(_) => 1.0,
        };
        WEATHER_PHASES
            .iter()
            .rposition(|&start| progress >= start)
            .unwrap_or(0)
    }

    pub fn sky(&self, chapter: &Chapter) -> Weather {
        self.weather_schedule(chapter)[self.sky_phase(chapter)]
    }

    pub fn sky_blend(&self, chapter: &Chapter) -> (Weather, Weather, f32) {
        let schedule = self.weather_schedule(chapter);
        let phase = self.sky_phase(chapter);
        match &self.phase {
            Phase::Open(day) if phase > 0 => {
                let since = day.seconds - WEATHER_PHASES[phase] * chapter.day_seconds();
                let blend = (since / 3.7).clamp(0.0, 1.0);
                (
                    schedule[phase - 1],
                    schedule[phase],
                    blend * blend * (3.0 - 2.0 * blend),
                )
            }
            _ => (schedule[phase], schedule[phase], 1.0),
        }
    }

    pub fn planning(&self) -> bool {
        matches!(self.phase, Phase::Plan)
    }

    pub fn feedback(&self) -> Option<&Tally> {
        match &self.phase {
            Phase::Open(day) => Some(&day.tally),
            Phase::Report(report) => Some(&report.tally),
            Phase::Plan => self.last_day.as_ref(),
        }
    }

    pub fn guest_history(&self) -> &[Visit] {
        if let Phase::Open(day) = &self.phase {
            &day.visits
        } else {
            &self.last_visits
        }
    }

    pub fn guest_tip(&self, chapter: &Chapter, guest: usize) -> Option<GuestTip> {
        let visit = self.guest_history().get(guest)?;
        match (visit.verdict()?, &self.phase) {
            (_, Phase::Report(_)) => None,
            (Verdict::Missed(Miss::Price | Miss::Empty), _) => {
                let lot = usize::from(visit.declined_at.or(visit.guided_to)?);
                self.lots[lot]
                    .is_some_and(|b| b.kind.service() == Some(visit.wants))
                    .then_some(GuestTip::Shop(lot))
            }
            (Verdict::Missed(Miss::NoShop | Miss::Walk), Phase::Plan) => {
                let kind = chapter
                    .allowed()
                    .iter()
                    .copied()
                    .find(|kind| kind.service() == Some(visit.wants))?;
                let origin = match visit.origin {
                    Origin::Home(lot) => usize::from(lot),
                    Origin::Ferry => chapter.layout.dock(),
                };
                chapter
                    .layout
                    .lots()
                    .filter(|&lot| self.lots[lot].is_none())
                    .filter(|&lot| {
                        distance(origin, lot)
                            <= walking_range(&self.lots, visit.origin, lot, chapter, false)
                    })
                    .min_by_key(|&lot| distance(origin, lot))
                    .map(|lot| GuestTip::Build(lot, kind))
            }
            _ => None,
        }
    }

    pub fn count(&self, kind: Kind) -> u32 {
        self.lots
            .iter()
            .flatten()
            .filter(|building| building.kind == kind)
            .count() as u32
    }

    pub fn next_favor(&self, chapter: &Chapter) -> Favor {
        if let Phase::Open(day) = &self.phase {
            return day.favor;
        }
        let seed =
            self.day.wrapping_mul(0x9e3779b9) ^ (self.chapter as u32).wrapping_mul(0x85ebca77);
        let seed = (seed ^ (seed >> 16)).wrapping_mul(0x45d9f3b);
        let stage = harbor_stage(self.chapter);
        if stage >= 3 && self.day >= 2 && seed % 5 < 2 {
            let rough = matches!(
                self.weather(chapter),
                Weather::Wind | Weather::Rain | Weather::Storm
            );
            let options = if rough {
                [
                    Favor::Crates,
                    Favor::ShadeSail,
                    Favor::BrokenJars,
                    Favor::LostGuests,
                ]
            } else {
                [
                    Favor::Fire,
                    Favor::LostGuests,
                    Favor::BrokenJars,
                    Favor::ShadeSail,
                ]
            };
            let start = ((seed >> 13) as usize) % options.len();
            for event in options.into_iter().cycle().skip(start).take(options.len()) {
                if stage < 5 && matches!(event, Favor::ShadeSail | Favor::LostGuests) {
                    continue;
                }
                if self.incident_lot(chapter, event).is_some() {
                    return event;
                }
            }
        }
        chapter.favors[(self.day.saturating_sub(1) as usize) % chapter.favors.len()]
    }

    fn incident_lot(&self, chapter: &Chapter, favor: Favor) -> Option<usize> {
        chapter
            .layout
            .lots()
            .filter(|&lot| self.lots[lot].is_some_and(|building| favor.allows(building.kind)))
            .min_by_key(|&lot| distance(lot, chapter.layout.dock()))
    }

    pub fn incident(&self, chapter: &Chapter) -> Option<(Favor, usize)> {
        let Phase::Open(day) = &self.phase else {
            return None;
        };
        if day.favor.incident()
            && day.favor_at.is_none()
            && (day.seconds >= self.favor_ready_at(chapter, day.favor)
                || day.route.is_some_and(|route| route.favor))
        {
            self.incident_lot(chapter, day.favor)
                .map(|lot| (day.favor, lot))
        } else {
            None
        }
    }

    fn favor_ready_at(&self, chapter: &Chapter, favor: Favor) -> f32 {
        if favor.incident() {
            let slot = (self.day as usize * 19 + self.chapter * 7) % 13;
            chapter.day_seconds() * (0.14 + slot as f32 * 0.012)
        } else {
            8.0
        }
    }

    pub fn visitors(&self, chapter: &Chapter) -> (u16, u16) {
        let residents = self
            .lots
            .iter()
            .flatten()
            .map(|b| self.home_visits(*b))
            .sum();
        let pilot = self.support(chapter.layout.dock(), Kind::FerryOffice, 2);
        let ferry = (chapter.ferry + chapter.extra_ferry_guests() + pilot.div_ceil(2))
            .saturating_add_signed(self.weather(chapter).ferry_change());
        let ferry = match self.harbor {
            HarborPlan::Regular => ferry,
            HarborPlan::Ferry => ferry + 3,
            HarborPlan::Water | HarborPlan::Neighbors => ferry.saturating_sub(2),
        };
        (residents, ferry)
    }

    fn home_visits(&self, building: Building) -> u16 {
        let residents = building.residents();
        residents + u16::from(residents > 0 && self.harbor == HarborPlan::Neighbors)
    }

    pub fn set_harbor(&mut self, chapter: &Chapter, rule: HarborPlan) -> bool {
        if !self.planning() || !chapter.harbor_plans() {
            return false;
        }
        self.harbor = rule;
        true
    }

    pub fn build(&mut self, chapter: &Chapter, lot: usize, kind: Kind) -> bool {
        if !self.planning()
            || !is_open(chapter, lot)
            || self.lots[lot].is_some()
            || self.coins < kind.cost()
            || !chapter.allowed().contains(&kind)
        {
            return false;
        }
        self.lots[lot] = Some(Building::new(kind));
        self.coins -= kind.cost();
        true
    }

    pub fn upgrade(&mut self, lot: usize) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        if building.level >= MAX_LEVEL || self.coins < building.upgrade_cost() {
            return false;
        }
        self.coins -= building.upgrade_cost();
        building.level += 1;
        true
    }

    pub fn set_price(&mut self, lot: usize, price: Price) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        if !building.kind.service().is_some_and(Service::priced) {
            return false;
        }
        building.price = price;
        true
    }

    pub fn remove(&mut self, lot: usize) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::take) else {
            return false;
        };
        self.coins += building.refund();
        true
    }

    pub fn neighboring(&self, lot: usize, kind: Kind, range: usize) -> bool {
        self.lots.iter().enumerate().any(|(other, building)| {
            building.is_some_and(|b| b.kind == kind)
                && other != lot
                && distance(lot, other) <= range
        })
    }

    pub fn support(&self, lot: usize, kind: Kind, range: usize) -> u16 {
        self.lots
            .iter()
            .enumerate()
            .filter(|(i, _)| distance(*i, lot) <= range)
            .filter_map(|(_, building)| building.filter(|b| b.kind == kind))
            .map(|b| u16::from(b.level))
            .max()
            .unwrap_or(0)
    }

    pub fn herb_support(&self, lot: usize) -> u16 {
        self.lots
            .iter()
            .enumerate()
            .filter_map(|(garden, b)| {
                let b = b.filter(|b| b.kind == Kind::HerbGarden)?;
                (distance(lot, garden) <= herb_range(b.level)).then_some(u16::from(b.level) + 1)
            })
            .max()
            .unwrap_or(0)
    }

    pub fn kitchens_in_reach(&self, lot: usize, level: u8) -> usize {
        self.lots
            .iter()
            .enumerate()
            .filter(|(kitchen, b)| {
                distance(lot, *kitchen) <= herb_range(level)
                    && b.is_some_and(|b| matches!(b.kind, Kind::Taverna | Kind::Bakery))
            })
            .count()
    }

    pub fn production(&self, chapter: &Chapter) -> Production {
        let weather = self.weather(chapter);
        let heat = weather == Weather::Heat;
        let mut plan = Production {
            water: chapter.spring + weather.rainfall(),
            ..Production::default()
        };
        for (lot, building) in self.lots.iter().enumerate() {
            let Some(building) = building else { continue };
            match building.kind {
                Kind::Well | Kind::Cistern => plan.water += 4 + u16::from(building.level) * 4,
                Kind::Qanat => {
                    plan.water += 8
                        + u16::from(building.level) * 5
                        + 3 * u16::from(self.neighboring(lot, Kind::Cistern, 2));
                }
                Kind::Grove => plan.olives += 2 + u16::from(building.level) * 3,
                Kind::RicePaddy => {
                    plan.rice += self.paddy_harvest(lot, building.level, weather);
                }
                Kind::Fishery => plan.fish += fish_catch(building.level, weather),
                Kind::Woodlot => plan.wood += 3 + 2 * u16::from(building.level),
                Kind::Aqueduct => {
                    plan.water += 10
                        + u16::from(building.level) * 5
                        + u16::from(self.neighboring(lot, Kind::Well, 1)) * 4;
                }
                _ => {}
            }
        }
        plan.water = match self.harbor {
            HarborPlan::Ferry => plan.water.saturating_sub(2),
            HarborPlan::Water => plan.water + 4,
            _ => plan.water,
        };
        // The closest workshops get their jars first. The stable plot order breaks ties.
        let mut workshops: Vec<_> = self
            .lots
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(lot, b)| b.filter(|b| b.kind.water_user()).map(|b| (lot, b)))
            .collect();
        let water_distance = |lot| {
            self.lots
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    b.filter(|b| b.kind.water_source())
                        .map(|_| distance(lot, i))
                })
                .min()
                .unwrap_or(LOTS)
        };
        // Nearby jobs in the same range retain street order from morning to morning.
        workshops.sort_by_key(|(lot, _)| water_distance(*lot).min(3));
        for (lot, b) in workshops {
            let level = u16::from(b.level);
            let cost = level + u16::from(heat) + 2 * u16::from(b.kind == Kind::Baths);
            let stocked = plan.water - plan.used_water >= cost;
            plan.short_water[lot] = !stocked;
            if stocked {
                plan.used_water += cost;
            }
            plan.stock[lot] = match (b.kind, stocked) {
                (Kind::DateGarden, true) => {
                    7 + level * 3 + 3 * u16::from(self.neighboring(lot, Kind::Qanat, 2))
                }
                (Kind::DateGarden, false) => 3 + level,
                (Kind::Caravanserai | Kind::Glassworks | Kind::Perfumer, true) => 4 + level * 3,
                (Kind::MoonGarden, true) => {
                    4 + level * 3
                        + 4 * u16::from(chapter.shift.night())
                        + self.support(lot, Kind::DateGarden, 2)
                }
                (Kind::Bakery, true) => 4 + level * 3,
                (Kind::Bakery, false) => 2 + level,
                (Kind::RiceKitchen, _) => {
                    let rice = (2 + level).min(plan.rice - plan.used_rice);
                    plan.used_rice += rice;
                    let base = if stocked { 3 + level * 2 } else { 1 + level };
                    base + rice * 2 + 2 * u16::from(self.neighboring(lot, Kind::RicePaddy, 1))
                }
                (Kind::Potter | Kind::Inn, true) => 3 + level * 2,
                (Kind::FeastHall, true) => 5 + level * 3,
                (Kind::Dyer | Kind::CeramicStudio, true) => 5 + level * 3,
                (Kind::GardenCourt, true) => {
                    6 + level * 3 + 4 * u16::from(self.neighboring(lot, Kind::Aqueduct, 2))
                }
                (Kind::Baths, true) => {
                    4 + level * 3 + 2 * u16::from(self.neighboring(lot, Kind::Aqueduct, 2))
                }
                _ => 1 + level,
            };
        }

        let (mut catch, mut wood) = (plan.fish, plan.wood);
        for (lot, b) in self.lots.iter().copied().enumerate() {
            let Some(b) = b else { continue };
            let level = u16::from(b.level);
            match b.kind {
                Kind::Stoa => plan.stock[lot] = 4 + level * 2,
                Kind::Forum => plan.stock[lot] = 5 + level * 3,
                Kind::TeaHouse => plan.stock[lot] = 4 + level * 2 + self.tea_support(lot),
                Kind::PaperWorkshop => plan.stock[lot] = 4 + level * 3,
                Kind::Weaver => plan.stock[lot] = 5 + level * 3,
                Kind::Apiary => {
                    plan.stock[lot] = 6 + level * 3 + self.support(lot, Kind::HerbGarden, 2)
                }
                Kind::Scribe => plan.stock[lot] = 5 + level * 3 + self.support(lot, Kind::Forum, 2),
                Kind::SilkShop => {
                    plan.stock[lot] = 5 + level * 3 + self.support(lot, Kind::MarketGarden, 2)
                }
                Kind::Cooperage => plan.stock[lot] = 5 + level * 3,
                Kind::SpiceBazaar => plan.stock[lot] = 6 + level * 3,
                Kind::CarpetHall => {
                    plan.stock[lot] = 4 + level * 3 + self.support(lot, Kind::SpiceBazaar, 2)
                }
                Kind::Smokehouse => {
                    let fish = catch.min(2 + level * 2);
                    catch -= fish;
                    if fish > 0 {
                        let fuel = wood.min(1 + level / 2);
                        wood -= fuel;
                        let near = self.neighboring(lot, Kind::Fishery, 1);
                        let dry = fuel > 0 && self.neighboring(lot, Kind::Woodlot, 1);
                        plan.stock[lot] =
                            fish * 2 + level + fuel * 2 + 2 * u16::from(near) + 2 * u16::from(dry);
                    }
                }
                Kind::Temple => {
                    plan.stock[lot] = 4 + level * 3 + 2 * u16::from(self.tea_support(lot) > 0);
                }
                Kind::Taverna => {
                    let bakery = self.nearest_stock(lot, Kind::Bakery, 2, &plan);
                    plan.stock[lot] = match bakery {
                        Some(source) if plan.olives - plan.used_olives >= 2 => {
                            plan.stock[source] -= 2;
                            plan.used_olives += 2;
                            3 + level * 3 + u16::from(distance(lot, source) <= 1)
                        }
                        _ => 1 + level,
                    };
                }
                Kind::Shrine => {
                    plan.stock[lot] = match self.nearest_stock(lot, Kind::Potter, 2, &plan) {
                        Some(source) => {
                            plan.stock[source] -= 2;
                            4 + level * 2
                        }
                        None => 1 + level,
                    };
                }
                _ => {}
            }
        }
        for (lot, b) in self.lots.iter().enumerate() {
            let Some(b) = b else { continue };
            // Any smokehouse in the district with two portions left supplies the hall;
            // a smokehouse right next door adds one more seat, like an oven for a tavern.
            if b.kind == Kind::FeastHall
                && let Some(source) = self.nearest_stock(lot, Kind::Smokehouse, 2, &plan)
            {
                plan.stock[source] -= 2;
                plan.stock[lot] += 5 + u16::from(distance(lot, source) <= 1);
            }
            if b.kind == Kind::Press {
                let level = u16::from(b.level);
                let olives = (2 + level).min(plan.olives - plan.used_olives);
                plan.used_olives += olives;
                plan.stock[lot] = 1 + olives * 2 + u16::from(self.neighboring(lot, Kind::Grove, 1));
            }
            if b.kind.service().is_some_and(Service::priced) {
                let supported = self
                    .lots
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| distance(lot, *i) <= 2)
                    .filter_map(|(_, b)| b.filter(|b| b.kind == Kind::Storehouse))
                    .map(|b| u16::from(b.level))
                    .max()
                    .unwrap_or(0);
                plan.stock[lot] += supported;
            }
            if matches!(b.kind, Kind::Bakery | Kind::Taverna) {
                plan.stock[lot] += self.herb_support(lot);
                plan.stock[lot] += self.support(lot, Kind::Apiary, 1);
            }
            let nearby = match b.kind {
                Kind::Bakery => Some(Kind::Mill),
                Kind::RiceKitchen | Kind::TeaHouse => Some(Kind::MarketGarden),
                Kind::Weaver => Some(Kind::Sheepfold),
                _ => None,
            };
            if let Some(source) = nearby {
                let level = self.support(lot, source, 2);
                plan.stock[lot] += level + u16::from(level > 0) * 2;
            }
        }
        plan
    }

    pub fn paddy_harvest(&self, lot: usize, level: u8, weather: Weather) -> u16 {
        5 + 4 * u16::from(level)
            + 2 * u16::from(self.neighboring(lot, Kind::Well, 1))
            + u16::from(weather == Weather::Rain)
    }

    pub fn tea_support(&self, lot: usize) -> u16 {
        self.lots
            .iter()
            .enumerate()
            .filter_map(|(garden, b)| {
                let b = b.filter(|b| b.kind == Kind::TeaGarden)?;
                (distance(lot, garden) <= herb_range(b.level)).then_some(2 * u16::from(b.level))
            })
            .max()
            .unwrap_or(0)
    }

    pub fn projected_upgrade(
        &self,
        chapter: &Chapter,
        lot: usize,
    ) -> Option<(Building, Production)> {
        let building = self.lots.get(lot).copied().flatten()?;
        if building.level >= MAX_LEVEL {
            return None;
        }
        let upgraded = Building {
            level: building.level + 1,
            ..building
        };
        let mut preview = self.clone();
        preview.lots[lot] = Some(upgraded);
        Some((upgraded, preview.production(chapter)))
    }

    fn nearest_stock(
        &self,
        lot: usize,
        kind: Kind,
        amount: u16,
        plan: &Production,
    ) -> Option<usize> {
        self.lots
            .iter()
            .enumerate()
            .filter(|(i, source)| {
                source.is_some_and(|b| b.kind == kind) && plan.stock[*i] >= amount
            })
            .min_by_key(|(i, _)| distance(lot, *i))
            .map(|(i, _)| i)
    }

    pub fn household(&self, lot: usize) -> [u16; 3] {
        let mut ages = [0; 3];
        if let Some(building) = self.lots.get(lot).copied().flatten() {
            for person in 0..usize::from(building.residents()) {
                ages[resident_age(building.kind, lot, person).on_island(self.chapter) as usize] +=
                    1;
            }
        }
        ages
    }

    pub fn age_forecast(&self, chapter: &Chapter) -> [u16; 3] {
        let mut ages = [0; 3];
        let mut count = |visits: &[Visit]| {
            for (index, visit) in visits.iter().enumerate() {
                ages[visit.age(index) as usize] += 1;
            }
        };
        match &self.phase {
            Phase::Open(day) => count(&day.visits),
            Phase::Report(_) => count(&self.last_visits),
            Phase::Plan => count(&self.planned_visits(chapter)),
        }
        ages
    }

    pub fn visit_plan(&self, chapter: &Chapter) -> Vec<Visit> {
        let (residents, ferry) = self.visitors(chapter);
        let count = usize::from(residents + ferry);
        let mut visitors = Vec::with_capacity(count);
        let weather = self.weather(chapter);
        let offset = (self.day as usize - 1) * 3;
        for (lot, b) in self.lots.iter().enumerate() {
            let Some(b) = b.filter(|b| b.residents() > 0) else {
                continue;
            };
            for local in 0..usize::from(self.home_visits(b)) {
                let index = visitors.len() + offset;
                let heat_shade = weather == Weather::Heat
                    && local == 1
                    && !self.neighboring(lot, Kind::Grove, 1)
                    && chapter.allowed().contains(&Kind::Stoa);
                visitors.push(Visit {
                    origin: Origin::Home(lot as u8),
                    age: Some(resident_age(b.kind, lot, local).on_island(self.chapter)),
                    specialty_bonus: 0,
                    wants: if heat_shade {
                        Service::Shade
                    } else {
                        chapter.locals[index % chapter.locals.len()]
                    },
                    budget: if b.kind == Kind::Villa && local % 3 != 0 {
                        Price::Fine
                    } else if local == 0 {
                        Price::Fair
                    } else {
                        Price::Regular
                    },
                    at: 0.0,
                    resolved: false,
                    shop: None,
                    miss: None,
                    guided_to: None,
                    declined_at: None,
                });
            }
        }
        for n in 0..usize::from(ferry) {
            let budget = if weather == Weather::Feast || n % 3 == 1 {
                Price::Fine
            } else if n % 5 == 4 {
                Price::Fair
            } else {
                Price::Regular
            };
            visitors.push(Visit {
                origin: Origin::Ferry,
                age: Some(
                    Age::guest(n + offset + self.chapter + self.harbor as usize * 2)
                        .on_island(self.chapter),
                ),
                specialty_bonus: 0,
                wants: chapter.guests[(n + offset) % chapter.guests.len()],
                budget,
                at: 0.0,
                resolved: false,
                shop: None,
                miss: None,
                guided_to: None,
                declined_at: None,
            });
        }
        let total = count.saturating_sub(1).max(1) as f32;
        for (i, visit) in visitors.iter_mut().enumerate() {
            visit.at = 6.0 + (i as f32 / total) * (chapter.day_seconds() - 8.4);
        }
        visitors
    }

    pub fn forecast(&self, chapter: &Chapter) -> [u16; SERVICES] {
        let mut demand = [0; SERVICES];
        for visit in self.planned_visits(chapter) {
            demand[visit.wants as usize] += 1;
        }
        demand
    }

    pub fn supply_need(
        &self,
        chapter: &Chapter,
        lot: usize,
        production: &Production,
    ) -> Option<SupplyNeed> {
        let service = self.lots.get(lot)?.as_ref()?.kind.service()?;
        let mut day = match &self.phase {
            Phase::Open(day) => day.as_ref().clone(),
            _ => self.day_plan(chapter, production),
        };
        let stock = day.stock[lot];
        day.stock[lot] = u16::MAX;
        let mut visits: Vec<_> = std::mem::take(&mut day.visits)
            .into_iter()
            .enumerate()
            .collect();
        visits.sort_by(|a, b| a.1.at.total_cmp(&b.1.at));
        let mut guests = 0;
        for (_, visit) in visits
            .iter()
            .filter(|(_, v)| !v.resolved && v.wants == service)
        {
            if let Ok(shop) = target(&self.lots, &day, visit, chapter) {
                if shop == lot {
                    guests += 1;
                } else {
                    day.stock[shop] -= 1;
                }
            }
        }
        Some(SupplyNeed {
            stock,
            guests,
            short_water: production.short_water[lot],
        })
    }

    pub fn offer_forecast(
        &self,
        chapter: &Chapter,
        lot: usize,
        production: &Production,
    ) -> Option<OfferForecast> {
        let building = self.lots.get(lot).copied().flatten()?;
        if !self.planning() || !Assortment::supported(building.kind) {
            return None;
        }
        let mut forecast = OfferForecast {
            matches: [0; Assortment::ALL.len()],
            extra_coins: [0; Assortment::ALL.len()],
        };
        let base = self.day_plan(chapter, production);
        let mut lots = self.lots;
        for choice in Assortment::ALL
            .into_iter()
            .filter(|choice| choice.unlocked(building.level))
        {
            let Some(age) = choice.flavor() else { continue };
            lots[lot] = Some(Building {
                assortment: choice,
                ..building
            });
            let mut market = base.clone();
            resolve_visits(&lots, &mut market, chapter, chapter.day_seconds(), None);
            let served = market
                .visits
                .iter()
                .filter(|visit| {
                    visit.shop == Some(lot as u8)
                        && visit.age == Some(age)
                        && visit.specialty_bonus > 0
                })
                .count() as u16;
            forecast.matches[choice as usize] = served;
            forecast.extra_coins[choice as usize] = u32::from(served) * choice.extra_coins();
        }
        Some(forecast)
    }

    /// Plays today's planned market once per price, so the price cards can show the money
    /// each choice brings. Walking range, budgets, stock and other shops all count.
    pub fn price_forecast(
        &self,
        chapter: &Chapter,
        lot: usize,
        production: &Production,
    ) -> Option<PriceForecast> {
        let building = self.lots.get(lot).copied().flatten()?;
        let service = building.kind.service().filter(|service| service.priced())?;
        if !self.planning() {
            return None;
        }
        let mut forecast = PriceForecast {
            sales: [0; Price::ALL.len()],
            coins: [0; Price::ALL.len()],
            misses: [0; Price::ALL.len()],
        };
        let mut town = self.clone();
        for price in Price::ALL {
            // The plan is made again, because a day event picks its wishes by price too.
            town.lots[lot] = Some(Building { price, ..building });
            let mut market = town.day_plan(chapter, production);
            resolve_visits(
                &town.lots,
                &mut market,
                chapter,
                chapter.day_seconds(),
                None,
            );
            let i = price as usize;
            for visit in market.visits.iter().filter(|visit| visit.wants == service) {
                if visit.shop == Some(lot as u8) {
                    forecast.sales[i] += 1;
                    forecast.coins[i] += price.coins(service) + visit.specialty_bonus;
                } else if visit.shop.is_none() && visit.declined_at == Some(lot as u8) {
                    forecast.misses[i] += 1;
                }
            }
        }
        Some(forecast)
    }

    pub fn set_assortment(&mut self, lot: usize, value: Assortment) -> bool {
        if !self.planning() {
            return false;
        }
        let Some(building) = self.lots.get_mut(lot).and_then(Option::as_mut) else {
            return false;
        };
        if !Assortment::supported(building.kind)
            || !value.unlocked(building.level)
            || value
                .flavor()
                .is_some_and(|age| !age.available(self.chapter))
        {
            return false;
        }
        building.assortment = value;
        true
    }

    pub fn open(&mut self, chapter: &Chapter) -> bool {
        if !self.planning() {
            return false;
        }
        let production = self.production(chapter);
        self.phase = Phase::Open(Box::new(self.day_plan(chapter, &production)));
        true
    }

    fn day_plan(&self, chapter: &Chapter, production: &Production) -> Day {
        let mut tally = Tally::default();
        for (lot, building) in self.lots.iter().enumerate() {
            if let Some(service) = building.and_then(|building| building.kind.service()) {
                tally.produced[service as usize] += u32::from(production.stock[lot]);
            }
        }
        let mut day = Day {
            seconds: 0.0,
            water: production.water,
            caravan_water: production.water.saturating_sub(production.used_water),
            stock: production.stock,
            supplied: Some(production.stock),
            visits: self.visit_plan(chapter),
            tally,
            sales: self.sales,
            favor: self.next_favor(chapter),
            favor_at: None,
            nili_at: Stop::Quay,
            route: None,
            nili: None,
            street_claims: 0,
            rice_harvest: production.rice,
            rung_bells: 0,
            fish_catch: production.fish,
            event: None,
            streak: 0,
            favor_rushed: false,
        };
        if let Some(event) = self.planned_event() {
            let guests = self.event_guests(chapter, event, &day);
            if let Some(first) = guests.first() {
                day.event = Some(Gathering {
                    event,
                    at: first.at,
                    guests: guests.len() as u8,
                });
            }
            day.visits.extend(guests);
        }
        day
    }

    /// Everyone expected today, including the guests of a day event.
    pub fn planned_visits(&self, chapter: &Chapter) -> Vec<Visit> {
        self.day_plan(chapter, &self.production(chapter)).visits
    }

    pub fn bell_ready(&self, chapter: &Chapter, lot: usize) -> bool {
        let Phase::Open(day) = &self.phase else {
            return false;
        };
        chapter.layout.contains(lot)
            && self.lots[lot].is_some_and(|building| building.kind == Kind::Temple)
            && day.route.is_none()
            && day.rung_bells & (1 << lot) == 0
            && day.seconds + 1.0 < chapter.day_seconds()
    }

    pub fn bell_guests(&self, lot: usize) -> u16 {
        2 + u16::from(self.neighboring(lot, Kind::PaperWorkshop, 1))
    }

    pub fn ring_bell(&mut self, chapter: &Chapter, lot: usize) -> bool {
        if !self.bell_ready(chapter, lot) {
            return false;
        }
        let guests = self.bell_guests(lot);
        let Phase::Open(day) = &mut self.phase else {
            return false;
        };
        day.rung_bells |= 1 << lot;
        day.tally.bell_rings += 1;
        day.restock(lot, guests);
        let at = day.seconds;
        invite_party(
            &self.lots,
            day,
            lot,
            at,
            chapter.day_seconds(),
            self.chapter,
            guests,
        );
        true
    }

    pub fn dispatch(&mut self, chapter: &Chapter, job: Job, lot: usize) -> Result<Route, Blocked> {
        let route = job.plan(self, chapter, lot)?;
        let Phase::Open(day) = &mut self.phase else {
            return Err(Blocked::Morning);
        };
        day.route = Some(route);
        Ok(route)
    }

    pub fn favor_plan(&self, chapter: &Chapter, lot: usize) -> Option<Route> {
        let Phase::Open(day) = &self.phase else {
            return None;
        };
        let kind = self.lots.get(lot)?.as_ref()?.kind;
        if day.seconds < self.favor_ready_at(chapter, day.favor)
            || day.route.is_some()
            || day.favor_at.is_some()
            || !chapter.layout.contains(lot)
            || !day.favor.allows(kind)
            || (day.favor.incident() && self.incident_lot(chapter, day.favor) != Some(lot))
        {
            return None;
        }
        let target = Stop::Parcel(lot as u8);
        let pickup = match day.favor {
            Favor::Cargo | Favor::LostGuests => Stop::Quay,
            Favor::Crates => Stop::Pier,
            Favor::BrokenJars => target,
            Favor::ShadeSail => chapter
                .layout
                .lots()
                .filter(|&i| self.lots[i].is_some_and(|b| b.kind == Kind::Storehouse))
                .min_by_key(|&i| distance(lot, i))
                .map_or(Stop::Quay, |i| Stop::Parcel(i as u8)),
            Favor::Fire => chapter
                .layout
                .lots()
                .filter(|&i| self.lots[i].is_some_and(|building| building.kind.water_source()))
                .min_by_key(|&i| distance(lot, i))
                .map_or(Stop::Quay, |i| Stop::Parcel(i as u8)),
            Favor::Jugs => target,
            Favor::Shade => chapter
                .layout
                .lots()
                .filter(|&i| {
                    self.lots[i].is_some_and(|b| {
                        matches!(b.kind, Kind::Grove | Kind::TeaGarden | Kind::DateGarden)
                    })
                })
                .min_by_key(|&i| distance(lot, i))
                .map_or(target, |i| Stop::Parcel(i as u8)),
        };
        Route::new(
            day.favor.job(),
            true,
            (day.nili_at, day.nili),
            pickup,
            target,
            day.seconds,
            chapter,
        )
    }

    pub fn aid(&mut self, chapter: &Chapter, lot: usize) -> bool {
        let Some(route) = self.favor_plan(chapter, lot) else {
            return false;
        };
        let Phase::Open(day) = &mut self.phase else {
            return false;
        };
        day.route = Some(route);
        true
    }

    pub fn tick(&mut self, chapter: &Chapter, elapsed: f32) -> bool {
        let closing = matches!(&self.phase,
            Phase::Open(day) if day.seconds + elapsed >= chapter.day_seconds());
        let contract = closing.then(|| self.daily_contract(chapter)).flatten();
        let order = trade_order(chapter, self.chapter);
        let rush = self.rush_deadline(chapter);
        let Phase::Open(day) = &mut self.phase else {
            return false;
        };
        let until = (day.seconds + elapsed).min(chapter.day_seconds());
        if let Some(route) = day
            .route
            .filter(|route| day.nili.is_none() && route.finish <= until)
        {
            resolve_visits(&self.lots, day, chapter, route.finish, order);
            finish_route(
                &self.lots,
                day,
                route,
                route.finish,
                chapter.day_seconds(),
                self.chapter,
                rush,
            );
        }
        resolve_visits(&self.lots, day, chapter, until, order);
        day.seconds = until;
        if day.seconds < chapter.day_seconds() {
            return false;
        }
        let mut tally = day.tally.clone();
        tally.reputation += tally.street_bonus;
        self.last_visits = std::mem::take(&mut day.visits);
        tally.widest_day = tally.served.iter().filter(|&&served| served > 0).count() as u32;
        tally.record_income = tally.income;
        tally.peak_water = u32::from(day.water);
        tally.harvest_peak = u32::from(day.rice_harvest);
        tally.catch_peak = u32::from(day.fish_catch);
        tally.best_ferry_day = tally.ferry_served;
        if tally.responses() >= 12 {
            tally.best_mood = tally.satisfaction().unwrap_or(0);
        }
        if let Some(Goal::GuestCare(age, minimum, _)) = care_goal(chapter, self.chapter) {
            tally.best_care = tally.care(age, minimum);
        }
        tally.best_favorite_menu = Age::ALL.map(|age| tally.favorite_variety(age));
        self.sales = day.sales;
        if self.weather(chapter) == Weather::Feast {
            tally.feast_peak = tally.served_total();
        }
        // The first day in a harbor sets the marks; records start on the second.
        let records = if self.last_day.is_some() {
            DayRecord::broken(&tally, &self.totals)
        } else {
            0
        };
        self.totals.add(&tally);
        self.last_day = Some(tally.clone());
        let contract_bonus = contract.map_or(0, |goal| {
            let (have, need) = goal.progress(&tally);
            if have >= need {
                goal.reward(self.chapter)
            } else {
                0
            }
        });
        let grant = 6 + u32::from(tally.served_total() == 0) * 3 + contract_bonus;
        self.coins += tally.income + grant;
        self.phase = Phase::Report(Box::new(Report {
            tally,
            grant,
            stars: 0,
            contract_bonus,
            records,
        }));
        if chapter
            .objectives(self.chapter)
            .all(|goal| self.goal(goal).0 >= goal.target())
            && let Phase::Report(report) = &mut self.phase
        {
            report.stars = 1
                + u8::from(self.day <= chapter.par)
                + u8::from(self.totals.favors >= chapter.favor_goal);
        }
        true
    }

    pub fn finish_controlled(&mut self, chapter: &Chapter) -> Option<Route> {
        let rush = self.rush_deadline(chapter);
        let Phase::Open(day) = &mut self.phase else {
            return None;
        };
        let (route, nili) = day.manual_route()?;
        if !matches!(nili.stage, ManualStage::Unload(progress) if progress >= 1.0) {
            return None;
        }
        finish_route(
            &self.lots,
            day,
            route,
            day.seconds,
            chapter.day_seconds(),
            self.chapter,
            rush,
        );
        Some(route)
    }

    /// Until when today's incident still pays the rush bonus.
    pub fn rush_deadline(&self, chapter: &Chapter) -> Option<f32> {
        let Phase::Open(day) = &self.phase else {
            return None;
        };
        (day.favor.incident() && day.favor_at.is_none())
            .then(|| self.favor_ready_at(chapter, day.favor) + RUSH_SECONDS)
    }

    pub fn advance(&mut self) -> bool {
        if !matches!(self.phase, Phase::Report(_)) {
            return false;
        }
        self.day += 1;
        self.phase = Phase::Plan;
        self.harbor = HarborPlan::Regular;
        self.collect_voyage(&crate::campaign::CHAPTERS[self.chapter]);
        true
    }

    pub fn goal(&self, goal: Goal) -> (u32, u32) {
        let value = match goal {
            Goal::Build(kind, _) => self.count(kind),
            Goal::Serve(service, _) => self.totals.served[service as usize],
            Goal::Renown(_) => self.totals.reputation,
            Goal::Help(_) => self.totals.favors,
            Goal::Earn(_) => self.totals.income,
            Goal::FeastGuests(_) => self.totals.feast_peak,
            Goal::Variety(_) => self.totals.widest_day,
            Goal::MarketDay(_) => self.totals.record_income,
            Goal::Water(_) => self.totals.peak_water,
            Goal::Route(job, _) => self.totals.routes[job as usize],
            Goal::RouteVariety(_) => self.totals.routes.iter().filter(|&&n| n > 0).count() as u32,
            Goal::FerryDay(_) => self.totals.best_ferry_day,
            Goal::GoodDay(_) => self.totals.best_mood,
            Goal::Specials(_) => self.totals.specialties.iter().sum(),
            Goal::Residents(_) => self
                .lots
                .iter()
                .flatten()
                .map(|b| u32::from(b.residents()))
                .sum(),
            Goal::Harvest(_) => self.totals.harvest_peak,
            Goal::Bell(_) => self.totals.bell_rings,
            Goal::Catch(_) => self.totals.catch_peak,
            Goal::Voyages(_) => self.totals.voyages,
            Goal::CargoIncome(_) => self.totals.cargo_income,
            Goal::Caravans(_) => self.totals.caravans,
            Goal::CaravanIncome(_) => self.totals.caravan_income,
            Goal::SpiceRoutes(_) => self.totals.spice_routes,
            Goal::Upgrade(kind, _) => self.building_level(kind),
            Goal::Sale(_, false) => self.sale_progress().basic,
            Goal::Sale(_, true) => self.sale_progress().special,
            Goal::TradeUpgrade(order) => {
                if self.sale_progress().basic >= order.basic {
                    self.building_level(order.shop)
                } else {
                    0
                }
            }
            Goal::Produce(service, _) => self.totals.produced[service as usize],
            Goal::Dogs(_) => self.totals.dogs,
            Goal::Clean(_) => self.totals.cleaned,
            Goal::GuestCare(..) => self.totals.best_care,
            Goal::FavoriteMenu(age, _) => self.totals.best_favorite_menu[age as usize],
            Goal::Rebuild(order) => self.lots[order.lot]
                .filter(|building| building.kind == order.new)
                .map_or(0, |building| u32::from(building.level)),
            Goal::Pair(first, second, _) => self
                .lots
                .iter()
                .enumerate()
                .filter(|(lot, b)| {
                    b.is_some_and(|b| b.kind == first) && self.neighboring(*lot, second, 1)
                })
                .count() as u32,
        };
        (value, goal.target())
    }

    /// Goal progress including the running day, as the player sees it on the street.
    pub fn live_goal(&self, goal: Goal) -> (u32, u32) {
        let (value, need) = self.goal(goal);
        let Phase::Open(day) = &self.phase else {
            return (value, need);
        };
        let today = &day.tally;
        let chapter = &crate::campaign::CHAPTERS[self.chapter];
        let shown = match goal {
            Goal::Serve(service, _) => value + today.served[service as usize],
            Goal::Renown(_) => value + today.reputation,
            Goal::Earn(_) => value + today.income,
            Goal::Help(_) => value + today.favors,
            Goal::FeastGuests(_) if self.weather(chapter) == Weather::Feast => {
                value.max(today.served_total())
            }
            Goal::Variety(_) => value.max(today.served.iter().filter(|&&n| n > 0).count() as u32),
            Goal::MarketDay(_) => value.max(today.income),
            Goal::Water(_) => value.max(u32::from(day.water)),
            Goal::Harvest(_) => value.max(u32::from(day.rice_harvest)),
            Goal::Specials(_) => value + today.specialties.iter().sum::<u32>(),
            Goal::Caravans(_) => value + today.caravans,
            Goal::CaravanIncome(_) => value + today.caravan_income,
            Goal::SpiceRoutes(_) => value + today.spice_routes,
            Goal::Bell(_) => value + today.bell_rings,
            Goal::Catch(_) => value.max(u32::from(day.fish_catch)),
            Goal::Route(job, _) => value + today.routes[job as usize],
            Goal::RouteVariety(_) => self
                .totals
                .routes
                .iter()
                .zip(today.routes)
                .filter(|&(record, today)| *record + today > 0)
                .count() as u32,
            Goal::FerryDay(_) => value.max(today.ferry_served),
            Goal::Produce(service, _) => value + today.produced[service as usize],
            Goal::Dogs(_) => value + today.dogs,
            Goal::Clean(_) => value + today.cleaned,
            Goal::GuestCare(age, minimum, _) => value.max(today.care(age, minimum)),
            Goal::FavoriteMenu(age, _) => value.max(today.favorite_variety(age)),
            _ => value,
        };
        (shown, need)
    }

    /// One bit per objective that is already complete, counting the running day.
    pub fn goals_met(&self, chapter: &Chapter) -> u64 {
        chapter
            .objectives(self.chapter)
            .enumerate()
            .filter(|&(_, goal)| self.live_goal(goal).0 >= goal.target())
            .fold(0, |met, (index, _)| met | 1 << index.min(63))
    }

    fn sale_progress(&self) -> SalesProgress {
        match &self.phase {
            Phase::Open(day) => day.sales,
            _ => self.sales,
        }
    }

    fn building_level(&self, kind: Kind) -> u32 {
        self.lots
            .iter()
            .flatten()
            .filter(|building| building.kind == kind)
            .map(|building| u32::from(building.level))
            .max()
            .unwrap_or(0)
    }
}

pub fn fish_catch(level: u8, weather: Weather) -> u16 {
    let penalty = match weather {
        Weather::Storm => 6,
        Weather::Wind => 2,
        _ => 0,
    };
    7 + 4 * u16::from(level) - penalty
}

fn herb_range(level: u8) -> usize {
    1 + usize::from(level >= 3)
}

fn resident_age(kind: Kind, lot: usize, person: usize) -> Age {
    use Age::*;
    let order = if kind == Kind::Villa {
        [Senior, Adult, Senior, Adult, Child, Adult]
    } else {
        [Adult, Child, Adult, Child, Senior, Adult]
    };
    order[(person + lot % 3) % order.len()]
}

fn resolve_visits(
    lots: &[Option<Building>; LOTS],
    day: &mut Day,
    chapter: &Chapter,
    until: f32,
    order: Option<TradeOrder>,
) {
    let mut due: Vec<_> = day
        .visits
        .iter()
        .enumerate()
        .filter(|(_, visit)| !visit.resolved && visit.at <= until)
        .map(|(i, visit)| (i, visit.at))
        .collect();
    if due.len() > 1 {
        due.sort_by(|a, b| a.1.total_cmp(&b.1));
    }
    for (index, _) in due {
        let visit = &day.visits[index];
        let service = visit.wants as usize;
        let result = target(lots, day, visit, chapter);
        let visit = &mut day.visits[index];
        let age = visit.age(index);
        let guests = &mut day.tally.guests_by_age[age as usize];
        guests.responses += 1;
        visit.resolved = true;
        match result {
            Ok(shop) => {
                let b = lots[shop].expect("a sale requires a building");
                day.stock[shop] -= 1;
                day.tally.served[service] += 1;
                guests.served += 1;
                if let Some(order) = order {
                    day.sales.sale(order, b, age);
                }
                let specialty = b.assortment.flavor() == Some(age);
                if specialty {
                    visit.specialty_bonus = b.assortment.extra_coins();
                    day.tally.specialty_income += visit.specialty_bonus;
                    day.tally.specialties[age as usize] += 1;
                    guests.favorites |= 1 << service;
                }
                day.tally.income += b.price.coins(visit.wants) + visit.specialty_bonus;
                day.tally.reputation += 1
                    + u32::from(b.price == Price::Fair && visit.budget == Price::Fine)
                    + u32::from(specialty);
                day.tally.ferry_served += u32::from(visit.origin == Origin::Ferry);
                visit.shop = Some(shop as u8);
                day.streak = day.streak.saturating_add(1);
                day.tally.best_streak = day.tally.best_streak.max(u32::from(day.streak));
                if day.streak.is_multiple_of(STREAK_STEP) {
                    day.tally.reputation += 1;
                    day.tally.streak_bonus += 1;
                }
            }
            Err(decline) => {
                day.tally.missed[service] += 1;
                day.tally.gaps[decline.reason as usize] += 1;
                visit.miss = Some(decline.reason);
                visit.declined_at = decline.lot;
                day.streak = 0;
            }
        }
    }
}

fn finish_route(
    lots: &[Option<Building>; LOTS],
    day: &mut Day,
    route: Route,
    at: f32,
    closing: f32,
    island: usize,
    rush: Option<f32>,
) {
    day.route = None;
    day.nili_at = route.delivery;
    if route.favor {
        if let Stop::Parcel(lot) = route.delivery {
            finish_favor(lots, day, usize::from(lot), at, closing, island, rush);
        }
        return;
    }
    day.tally.routes[route.job as usize] += 1;
    let lot = match route.delivery {
        Stop::Parcel(i) => usize::from(i),
        _ => {
            day.tally.income += 5;
            day.tally.reputation += 2;
            return;
        }
    };
    match route.job {
        Job::Freight => {
            day.restock(
                lot,
                if matches!(route.pickup, Stop::Parcel(_)) {
                    6
                } else {
                    4
                },
            );
        }
        Job::Water => {
            day.restock(lot, 4);
            day.water += 2;
        }
        Job::Welcome => invite_party(lots, day, lot, at, closing, island, WELCOME_GUESTS),
        Job::Shade => day.restock(lot, 5),
        Job::Quay => {}
    }
    day.tally.income += 3;
    day.tally.reputation += 1 + u32::from(route.job != Job::Freight);
}

fn invite_party(
    lots: &[Option<Building>; LOTS],
    day: &mut Day,
    lot: usize,
    at: f32,
    closing: f32,
    island: usize,
    guests: u16,
) {
    let Some(shop) = lots[lot] else {
        return;
    };
    if let Some(wants) = shop.kind.service() {
        for guest in 0..guests {
            day.visits.push(Visit {
                origin: Origin::Ferry,
                age: Some(Age::guest(day.visits.len()).on_island(island)),
                specialty_bonus: 0,
                wants,
                // Nili's guests already agreed on the shop, so they pay its price.
                budget: shop.price,
                at: (at + 0.6 + f32::from(guest) * 1.2).min(closing),
                resolved: false,
                shop: None,
                miss: None,
                guided_to: Some(lot as u8),
                declined_at: None,
            });
        }
    }
}

fn finish_favor(
    lots: &[Option<Building>; LOTS],
    day: &mut Day,
    lot: usize,
    at: f32,
    closing: f32,
    island: usize,
    rush: Option<f32>,
) {
    match day.favor {
        Favor::Jugs => {
            for (neighbor, building) in lots.iter().enumerate() {
                if building.is_some_and(|b| b.kind.water_user()) && distance(lot, neighbor) <= 3 {
                    day.restock(neighbor, 2);
                }
            }
        }
        Favor::Shade => {
            for (neighbor, building) in lots.iter().enumerate() {
                if building.is_some_and(|b| b.kind.rest_stop()) {
                    day.restock(neighbor, 2);
                }
            }
        }
        Favor::Cargo | Favor::Crates | Favor::Fire => day.restock(lot, 3),
        Favor::BrokenJars => day.restock(lot, 2),
        Favor::ShadeSail => day.restock(lot, 5),
        Favor::LostGuests => {
            // Like the temple bell: the shop gets room for the group Nili brings.
            day.restock(lot, WELCOME_GUESTS);
            invite_party(lots, day, lot, at, closing, island, WELCOME_GUESTS);
        }
    }
    day.tally.income += 6;
    day.tally.reputation += 2;
    day.tally.favors += 1;
    day.favor_at = Some(lot as u8);
    if day.favor.incident() && rush.is_some_and(|deadline| at <= deadline) {
        day.tally.income += RUSH_COINS;
        day.tally.reputation += RUSH_RENOWN;
        day.favor_rushed = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Decline {
    reason: Miss,
    lot: Option<u8>,
}

fn walking_range(
    lots: &[Option<Building>; LOTS],
    origin: Origin,
    lot: usize,
    chapter: &Chapter,
    quay_cleared: bool,
) -> usize {
    match origin {
        Origin::Home(_) => 4,
        Origin::Ferry => {
            chapter.walk
                + 2 * usize::from(quay_cleared)
                + usize::from(lots.iter().enumerate().any(|(i, b)| {
                    b.is_some_and(|b| b.kind == Kind::Forum) && distance(i, lot) <= 2
                }))
        }
    }
}

fn target(
    lots: &[Option<Building>; LOTS],
    day: &Day,
    visit: &Visit,
    chapter: &Chapter,
) -> Result<usize, Decline> {
    let origin = match visit.origin {
        Origin::Home(lot) => usize::from(lot),
        Origin::Ferry => chapter.layout.dock(),
    };
    let mut best = None;
    let mut declined = None;
    let quay_cleared = day.tally.routes[Job::Quay as usize] > 0;
    for (lot, building) in lots.iter().enumerate() {
        if visit.guided_to.is_some_and(|i| usize::from(i) != lot) {
            continue;
        }
        let Some(building) = building.filter(|b| b.kind.service() == Some(visit.wants)) else {
            continue;
        };
        let steps = distance(origin, lot);
        let range = walking_range(lots, visit.origin, lot, chapter, quay_cleared);
        let taste = u32::from(visit.age.is_some() && visit.age == building.assortment.flavor()) * 3;
        let merit = (steps as u32 * 2 + building.price.coins(visit.wants)).saturating_sub(taste);
        let candidate = (merit, lot);
        let failure = if visit.guided_to.is_none() && steps > range {
            Some(Miss::Walk)
        } else if visit.wants.priced() && building.price as u8 > visit.budget as u8 {
            Some(Miss::Price)
        } else if day.stock[lot] == 0 {
            Some(Miss::Empty)
        } else {
            None
        };
        if let Some(reason) = failure {
            let rank = (std::cmp::Reverse(reason as u8), candidate);
            if declined.is_none_or(|(prior, _)| rank < prior) {
                declined = Some((
                    rank,
                    Decline {
                        reason,
                        lot: Some(lot as u8),
                    },
                ));
            }
        } else if best.is_none_or(|score| candidate < score) {
            best = Some(candidate);
        }
    }
    best.map(|(_, lot)| lot).ok_or_else(|| {
        declined.map_or(
            Decline {
                reason: Miss::NoShop,
                lot: None,
            },
            |(_, decline)| decline,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{campaign::CHAPTERS, save::Campaign};
    use Kind::*;

    const PLANS: [&[(usize, Kind)]; 12] = [
        &[(2, Bakery), (8, Well)],
        &[(8, Well), (0, Grove)],
        &[(9, Potter), (8, Well)],
        &[(9, Stoa), (0, Grove), (8, Potter)],
        &[(8, Taverna), (9, Stoa), (10, Potter), (6, Bakery)],
        &[(10, Inn), (2, Grove), (4, Potter), (3, Taverna)],
        &[(6, Potter), (11, Potter), (10, Stoa), (1, Grove), (5, Inn)],
        &[(9, Potter), (11, Inn), (3, Grove)],
        &[
            (3, Grove),
            (9, Taverna),
            (0, House),
            (11, Potter),
            (5, Inn),
            (6, Bakery),
        ],
        &[
            (11, Inn),
            (1, Grove),
            (8, Taverna),
            (10, Potter),
            (6, Well),
            (5, Stoa),
        ],
        &[
            (11, Inn),
            (3, Grove),
            (9, Taverna),
            (10, Potter),
            (5, Stoa),
            (6, Well),
        ],
        &[
            (11, Inn),
            (9, Taverna),
            (3, Grove),
            (10, Stoa),
            (4, Potter),
            (6, Bakery),
        ],
    ];

    #[test]
    fn first_twelve_harbors_can_earn_three_stars_and_unlock_the_next() {
        let mut save = Campaign::default();
        for (i, chapter) in CHAPTERS.iter().take(PLANS.len()).enumerate() {
            assert!(save.unlocked(i), "{}", chapter.district);
            let mut town = Town::new(i, chapter);
            let mut result = None;
            let layout = reference_layout(i, chapter);
            for _ in 1..=chapter.par {
                prepare_market(&mut town, chapter, &layout);
                improve_objectives(&mut town, chapter);
                if i == 5
                    && town.day >= 3
                    && let Some(lot) = town
                        .lots
                        .iter()
                        .position(|b| b.is_some_and(|b| b.kind == Taverna && b.level < 2))
                {
                    town.upgrade(lot);
                }
                let policy = if i == 10 {
                    HarborPlan::Ferry
                } else {
                    HarborPlan::Regular
                };
                town = play_market(town, chapter, policy, Price::Fair);
                let Phase::Report(report) = &town.phase else {
                    unreachable!()
                };
                if report.stars == 3 {
                    let day = town.day;
                    save.record(i, day, report.stars);
                    result = Some(day);
                    break;
                }
                town.advance();
            }
            assert!(
                result.is_some(),
                "{} fehlte am Richttag: {:?}, {} Bitten, {} Gäste. Gebäude: {:?}",
                chapter.district,
                chapter
                    .objectives(i)
                    .map(|goal| (goal.label(), town.goal(goal)))
                    .collect::<Vec<_>>(),
                town.totals.favors,
                town.totals.served_total(),
                town.lots,
            );
            eprintln!(
                "{}: drei Sterne an Tag {}",
                chapter.district,
                result.unwrap()
            );
        }
    }

    #[test]
    fn saving_during_a_favor_restores_and_delivers_it_once() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fair));
        assert!(town.open(chapter));
        town.tick(chapter, 12.0);
        assert!(town.aid(chapter, 2));
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        assert_eq!(day.tally.favors, 0);
        assert!(day.route.is_some_and(|route| route.favor));
        let serialized = serde_json::to_vec(&town).unwrap();
        let mut restored: Town = serde_json::from_slice(&serialized).unwrap();
        assert!(!restored.aid(chapter, 2));
        assert_eq!(restored.lots[2].unwrap().price, Price::Fair);
        town.tick(chapter, DAY_SECONDS);
        restored.tick(chapter, DAY_SECONDS);
        assert_eq!(town.coins, restored.coins);
        assert_eq!(town.totals.served, restored.totals.served);
        assert_eq!(restored.totals.favors, 1);
    }

    #[test]
    fn complaints_remain_actionable_the_next_morning_after_a_reload() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fine));
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        let guest = town
            .guest_history()
            .iter()
            .position(|v| v.verdict() == Some(Verdict::Missed(Miss::Price)))
            .unwrap();
        assert_eq!(town.guest_history()[guest].declined_at, Some(2));
        assert!(town.guest_tip(chapter, guest).is_none());
        let mut town: Town = serde_json::from_slice(&serde_json::to_vec(&town).unwrap()).unwrap();
        assert!(town.advance());
        assert!(matches!(
            town.guest_tip(chapter, guest),
            Some(GuestTip::Shop(2))
        ));
        assert!(town.remove(2));
        assert!(town.guest_tip(chapter, guest).is_none());
        assert_eq!(
            town.guest_history()[guest].verdict(),
            Some(Verdict::Missed(Miss::Price))
        );
    }

    #[test]
    fn a_missing_shop_points_to_a_buildable_plot_within_walking_range() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert!(town.advance());
        let guest = town
            .guest_history()
            .iter()
            .position(|v| matches!(v.origin, Origin::Home(1)))
            .unwrap();
        let Some(GuestTip::Build(lot, Bakery)) = town.guest_tip(chapter, guest) else {
            panic!("expected a recommendation for a bakery");
        };
        assert!(is_open(chapter, lot));
        assert!(distance(1, lot) <= 4);
        assert!(town.build(chapter, lot, Bakery));
    }

    #[test]
    fn tavern_uses_two_breads_and_rewards_an_adjacent_oven() {
        let chapter = &CHAPTERS[4];
        let mut town = Town::new(4, chapter);
        let bread = town.production(chapter).stock[7];
        assert!(town.build(chapter, 8, Taverna));
        let plan = town.production(chapter);
        assert_eq!(plan.stock[7], bread - 2);
        assert_eq!(plan.stock[8], 7);
        assert_eq!(plan.used_olives, 2);
        assert!(!town.build(chapter, 11, Bakery));
    }

    #[test]
    fn real_visits_use_budget_distance_and_stock() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.build(chapter, 0, Potter));
        let visit = Visit {
            origin: Origin::Ferry,
            age: None,
            specialty_bonus: 0,
            wants: Service::Pottery,
            budget: Price::Fair,
            at: 0.0,
            resolved: false,
            shop: None,
            miss: None,
            guided_to: None,
            declined_at: None,
        };
        let review = |town: &Town| {
            let mut market = town.clone();
            assert!(market.open(chapter));
            let Phase::Open(day) = &market.phase else {
                unreachable!()
            };
            target(&market.lots, day, &visit, chapter)
        };
        assert_eq!(
            review(&town),
            Err(Decline {
                reason: Miss::Walk,
                lot: Some(0)
            })
        );
        assert!(town.remove(0));
        assert!(town.build(chapter, 9, Potter));
        assert_eq!(
            review(&town),
            Err(Decline {
                reason: Miss::Price,
                lot: Some(9)
            })
        );
        assert!(town.set_price(9, Price::Fair));
        assert_eq!(review(&town), Ok(9));
        assert!(town.open(chapter));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.stock[9] = 0;
        assert_eq!(
            target(&town.lots, day, &visit, chapter),
            Err(Decline {
                reason: Miss::Empty,
                lot: Some(9)
            })
        );
    }

    #[test]
    fn a_resumed_nili_route_delivers_once_to_the_chosen_plot() {
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert!(town.build(chapter, 8, Well));
        assert!(town.open(chapter));
        let route = town
            .dispatch(chapter, Job::Water, 7)
            .unwrap_or_else(|_| panic!("reachable water route"));
        assert!(matches!(
            town.dispatch(chapter, Job::Freight, 7),
            Err(Blocked::Busy)
        ));
        assert!(!town.tick(chapter, route.pickup_at + 0.2));
        let packed = serde_json::to_vec(&town).unwrap();
        let mut resumed: Town = serde_json::from_slice(&packed).unwrap();
        let elapsed = route.finish - route.pickup_at;
        for market in [&mut town, &mut resumed] {
            assert!(!market.tick(chapter, elapsed));
            let Phase::Open(day) = &market.phase else {
                unreachable!()
            };
            assert!(day.route.is_none());
            assert_eq!(day.tally.routes[Job::Water as usize], 1);
            assert_eq!(day.nili_at, Stop::Parcel(7));
            assert!(matches!(
                market.dispatch(chapter, Job::Water, 7),
                Err(Blocked::Done)
            ));
            assert!(market.tick(chapter, chapter.day_seconds()));
            assert_eq!(market.goal(Goal::Route(Job::Water, 2)), (1, 2));
        }
        assert_eq!(town.coins, resumed.coins);
        assert_eq!(town.totals.served, resumed.totals.served);
    }

    // Guided guests used to carry a normal budget and walked away from a high price.
    // Nili's guests have chosen the shop, so they pay whatever it asks.
    #[test]
    fn a_guided_party_uses_the_selected_shop_and_pays_its_price() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.build(chapter, 0, Potter));
        assert!(town.set_price(0, Price::Fine));
        assert!(town.open(chapter));
        let route = town
            .dispatch(chapter, Job::Welcome, 0)
            .unwrap_or_else(|_| panic!("reachable escort"));
        town.tick(chapter, route.finish + 2.0);
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let party: Vec<_> = day
            .visits
            .iter()
            .filter(|visit| visit.guided_to == Some(0))
            .collect();
        assert_eq!(party.len(), 2);
        assert!(party.iter().all(|visit| {
            visit.resolved && visit.origin == Origin::Ferry && visit.shop == Some(0)
        }));
    }

    #[test]
    fn nili_only_escorts_guests_to_a_shop_with_room_and_lost_guests_bring_their_own() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.build(chapter, 0, Potter));
        assert!(town.open(chapter));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.stock[0] = 1;
        assert!(matches!(
            town.dispatch(chapter, Job::Welcome, 0),
            Err(Blocked::Stock)
        ));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.stock[0] = 0;
        day.favor = Favor::LostGuests;
        let lots = town.lots;
        finish_favor(&lots, day, 0, 0.0, chapter.day_seconds(), 2, None);
        assert_eq!(day.stock[0], 2);
        resolve_visits(&lots, day, chapter, chapter.day_seconds(), None);
        let party: Vec<_> = day
            .visits
            .iter()
            .filter(|v| v.guided_to == Some(0))
            .collect();
        assert_eq!(party.len(), 2);
        assert!(party.iter().all(|visit| visit.shop == Some(0)));
    }

    fn prepare_market(town: &mut Town, chapter: &Chapter, layout: &[(usize, Kind)]) {
        use crate::campaign::Goal::*;
        let near = |lot| distance(lot, chapter.layout.dock());
        let planned = |kind, lot| layout.iter().any(|&(i, k)| i == lot && k == kind);
        for goal in chapter.objectives(town.chapter) {
            if let Rebuild(order) = goal
                && !town.lots[order.lot].is_some_and(|building| building.kind == order.new)
            {
                let refund = town.lots[order.lot].map_or(0, Building::refund);
                if town.coins + refund >= order.new.cost() {
                    town.remove(order.lot);
                    assert!(town.build(chapter, order.lot, order.new));
                }
            }
        }
        for goal in chapter.objectives(town.chapter) {
            let Pair(first, second, need) = goal else {
                continue;
            };
            while town.goal(goal).0 < need {
                let mut best = None;
                for a in chapter.layout.lots() {
                    if town.lots[a].is_some_and(|b| b.kind != first)
                        || (town.lots[a].is_some() && town.neighboring(a, second, 1))
                    {
                        continue;
                    }
                    for b in chapter.layout.lots().filter(|&b| distance(a, b) == 1) {
                        if town.lots[b].is_some_and(|building| building.kind != second) {
                            continue;
                        }
                        let cost = if town.lots[a].is_none() {
                            first.cost()
                        } else {
                            0
                        } + if town.lots[b].is_none() {
                            second.cost()
                        } else {
                            0
                        };
                        if cost == 0 || cost > town.coins {
                            continue;
                        }
                        let rank = (
                            cost,
                            !planned(first, a),
                            !planned(second, b),
                            near(a) + near(b),
                            a,
                            b,
                        );
                        if best.is_none_or(|prior| rank < prior) {
                            best = Some(rank);
                        }
                    }
                }
                let Some((_, _, _, _, a, b)) = best else {
                    break;
                };
                let mut built = false;
                if town.lots[a].is_none() {
                    built |= town.build(chapter, a, first);
                }
                if town.lots[b].is_none() {
                    built |= town.build(chapter, b, second);
                }
                if !built || town.lots[a].is_none() || town.lots[b].is_none() {
                    break;
                }
            }
        }
        for goal in chapter.objectives(town.chapter) {
            let (kind, count) = match goal {
                Build(kind, count) => (kind, count),
                Upgrade(kind, _) => (kind, 1),
                TradeUpgrade(order) => (order.shop, 1),
                Serve(service, _) | Produce(service, _) => {
                    if town
                        .lots
                        .iter()
                        .flatten()
                        .any(|b| b.kind.service() == Some(service))
                    {
                        continue;
                    }
                    let Some(kind) = crate::campaign_orders::shop_for(service, chapter) else {
                        continue;
                    };
                    (kind, 1)
                }
                Residents(_) => {
                    let kind = *chapter
                        .allowed()
                        .iter()
                        .find(|&&kind| Building::new(kind).residents() > 0)
                        .unwrap();
                    (kind, if town.day >= 4 { 3 } else { 2 })
                }
                _ => continue,
            };
            for _ in town.count(kind)..count {
                let lot = chapter
                    .layout
                    .lots()
                    .filter(|&lot| town.lots[lot].is_none())
                    .min_by_key(|&lot| {
                        (
                            !planned(kind, lot),
                            layout.iter().any(|&(i, _)| i == lot),
                            near(lot),
                        )
                    });
                if let Some(lot) = lot {
                    town.build(chapter, lot, kind);
                }
            }
        }
        if town.day >= 3
            && chapter
                .goals
                .iter()
                .any(|g| matches!(g, Residents(need) if town.goal(*g).0 < *need))
            && let Some(lot) = town
                .lots
                .iter()
                .position(|b| b.is_some_and(|b| b.residents() > 0 && b.level < MAX_LEVEL))
        {
            town.upgrade(lot);
        }
        if town.chapter >= crate::campaign::OASIS_START + crate::campaign::CORE_HARBORS {
            let cistern = town
                .lots
                .iter()
                .position(|lot| lot.is_some_and(|b| b.kind == Qanat && b.level < 2));
            if let Some(lot) = cistern {
                town.upgrade(lot);
            }
        }
        for &(lot, kind) in layout {
            if town.lots[lot].is_none() && is_open(chapter, lot) && !town.build(chapter, lot, kind)
            {
                break;
            }
        }
    }

    fn improve_objectives(town: &mut Town, chapter: &Chapter) {
        for goal in chapter.objectives(town.chapter) {
            if let Goal::Rebuild(order) = goal {
                if town.goal(goal).0 < goal.target()
                    && town.lots[order.lot].is_some_and(|building| building.kind == order.new)
                {
                    town.upgrade(order.lot);
                }
                continue;
            }
            let (kind, level) = match goal {
                Goal::Upgrade(kind, level) => (kind, level),
                Goal::TradeUpgrade(order)
                    if town.goal(Goal::Sale(order, false)).0 >= order.basic =>
                {
                    (order.shop, goal.target())
                }
                _ => continue,
            };
            if town.goal(goal).0 >= level {
                continue;
            }
            let lot = chapter
                .layout
                .lots()
                .filter(|&lot| town.lots[lot].is_some_and(|b| b.kind == kind))
                .max_by_key(|&lot| town.lots[lot].map(|b| b.level));
            if let Some(lot) = lot {
                town.upgrade(lot);
            }
        }
    }

    fn reference_layout(i: usize, chapter: &Chapter) -> Vec<(usize, Kind)> {
        use crate::campaign::{Goal::*, JAPAN_START, NORDIC_START, REGION_SIZE};

        if i >= crate::campaign::OASIS_START {
            return [
                (8, Caravanserai),
                (7, DateGarden),
                (3, SpiceBazaar),
                (4, Glassworks),
                (9, Qanat),
                (10, Perfumer),
                (11, MoonGarden),
                (5, CarpetHall),
                (0, OasisHome),
            ]
            .into_iter()
            .filter(|(_, kind)| chapter.allowed().contains(kind))
            .collect();
        }

        if i >= NORDIC_START {
            return match i - NORDIC_START {
                0 => vec![(7, Smokehouse), (8, Woodlot)],
                1 => vec![(8, Woodlot), (6, Fishery)],
                2 => vec![(9, Boatyard), (6, Fishery)],
                3 => vec![(10, Weaver), (6, Fishery)],
                4..=6 => vec![
                    (6, FeastHall),
                    (4, Smokehouse),
                    (0, Fishery),
                    (5, Storehouse),
                ],
                7 => vec![
                    (4, Smokehouse),
                    (0, Fishery),
                    (6, FeastHall),
                    (5, Storehouse),
                    (11, Cooperage),
                ],
                10 => vec![
                    (6, FeastHall),
                    (4, Smokehouse),
                    (0, Fishery),
                    (5, Storehouse),
                ],
                17 => vec![
                    (4, Smokehouse),
                    (0, Fishery),
                    (6, Cooperage),
                    (5, Storehouse),
                ],
                15 => vec![
                    (4, Storehouse),
                    (6, Smokehouse),
                    (0, Fishery),
                    (5, FeastHall),
                ],
                _ => vec![
                    (4, Smokehouse),
                    (0, Fishery),
                    (5, Storehouse),
                    (6, FeastHall),
                ],
            };
        }

        if i >= JAPAN_START {
            return match i - JAPAN_START {
                0 => vec![(7, RiceKitchen)],
                1 => vec![(8, Temple)],
                2 | 3 => vec![(4, TeaHouse), (10, TeaGarden), (0, RicePaddy)],
                4 => vec![(9, PaperWorkshop), (10, TeaHouse), (5, Inn)],
                5..=7 => vec![
                    (4, PaperWorkshop),
                    (10, TeaHouse),
                    (11, Inn),
                    (6, Storehouse),
                ],
                13 => vec![
                    (4, RiceKitchen),
                    (6, RicePaddy),
                    (5, PaperWorkshop),
                    (11, TeaHouse),
                    (0, Storehouse),
                ],
                20 => vec![
                    (6, RiceKitchen),
                    (0, RicePaddy),
                    (4, PaperWorkshop),
                    (11, TeaHouse),
                    (5, Storehouse),
                ],
                21 => vec![
                    (4, PaperWorkshop),
                    (11, TeaHouse),
                    (5, SilkShop),
                    (6, CeramicStudio),
                    (0, RicePaddy),
                    (12, RiceKitchen),
                ],
                19 | 23 => vec![
                    (4, PaperWorkshop),
                    (11, TeaHouse),
                    (6, Temple),
                    (5, Storehouse),
                ],
                _ => vec![
                    (4, PaperWorkshop),
                    (11, TeaHouse),
                    (5, Storehouse),
                    (0, RicePaddy),
                ],
            };
        }
        if let Some(plan) = PLANS.get(i) {
            return match i {
                5 => vec![
                    (2, Grove),
                    (6, Bakery),
                    (8, Taverna),
                    (10, Inn),
                    (4, Potter),
                    (9, Stoa),
                ],
                6 => vec![(3, Grove), (9, Taverna), (6, Potter), (10, Stoa), (2, Inn)],
                7 => vec![
                    (9, Apiary),
                    (3, Grove),
                    (8, Taverna),
                    (11, Inn),
                    (6, Storehouse),
                ],
                _ => plan.to_vec(),
            };
        }
        if i == REGION_SIZE - 1 {
            return vec![
                (4, Potter),
                (10, Dyer),
                (11, Apiary),
                (6, Storehouse),
                (9, Press),
                (5, Shrine),
                (8, Taverna),
                (13, Stoa),
                (14, HerbGarden),
            ];
        }
        if i < REGION_SIZE {
            return [
                (6, Storehouse),
                (3, Grove),
                (9, Press),
                (5, Shrine),
                (8, Taverna),
                (4, Potter),
                (10, Stoa),
                (11, Inn),
            ]
            .into_iter()
            .filter(|(_, k)| chapter.allowed().contains(k))
            .collect();
        }
        if chapter.district == "Puteoli" {
            return vec![
                (11, Inn),
                (5, Forum),
                (4, Aqueduct),
                (6, Inn),
                (10, Baths),
                (9, Potter),
                (8, Taverna),
            ];
        }
        let aqueduct = if chapter.district == "Tibur" { 1 } else { 4 };
        let mut plan = vec![
            (aqueduct, Aqueduct),
            (10, Baths),
            (5, Forum),
            (11, Inn),
            (9, Potter),
            (8, Taverna),
            (10, Stoa),
        ];
        if chapter.district == "Lavinium" {
            plan.push((3, Villa));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Baths, 2))) {
            plan.push((3, Baths));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Forum, 2))) {
            plan.push((6, Forum));
        } else if chapter.goals.iter().any(|g| matches!(g, Build(Bakery, 2))) {
            plan.push((6, Bakery));
        }
        if chapter.district == "Tibur" {
            plan.push((6, Potter));
        }
        plan.push((6, Storehouse));
        plan.into_iter()
            .filter(|(_, k)| chapter.allowed().contains(k))
            .collect()
    }

    fn walk_to(town: &mut Town, chapter: &Chapter, end: bevy::prelude::Vec3) {
        use crate::world::motion;
        use bevy::prelude::Vec2;
        let Phase::Open(day) = &town.phase else {
            return;
        };
        let current = Vec2::from_array(day.nili.unwrap().position);
        let end = motion::ground(end, chapter.layout);
        let points = [Vec2::new(current.x, 0.0), Vec2::new(end.x, 0.0), end];
        for point in points {
            loop {
                let Phase::Open(day) = &mut town.phase else {
                    return;
                };
                let nili = day.nili.as_mut().unwrap();
                let delta = point - Vec2::from_array(nili.position);
                if delta.length_squared() < 0.0025 {
                    break;
                }
                assert!(
                    motion::walk(nili, delta.clamp_length_max(4.55 * 0.20), chapter.layout) > 0.0
                );
                if town.tick(chapter, 0.20) {
                    return;
                }
            }
        }
        if let Phase::Open(day) = &town.phase {
            assert!(end.distance(Vec2::from_array(day.nili.unwrap().position)) < 0.32);
        }
    }

    fn walk_errand(town: &mut Town, chapter: &Chapter, route: Route) {
        use crate::world::motion;
        for (index, stop) in [route.pickup, route.delivery].into_iter().enumerate() {
            let Phase::Open(day) = &mut town.phase else {
                return;
            };
            let nili = day.nili.as_mut().unwrap();
            nili.stage = if index == 0 {
                ManualStage::Pickup
            } else {
                ManualStage::Delivery
            };
            if !motion::can_work(*nili, route, chapter.layout) {
                walk_to(town, chapter, motion::stop(stop, chapter.layout));
            }
            if !matches!(town.phase, Phase::Open(_)) {
                return;
            }
            if town.tick(chapter, 0.88) {
                return;
            }
        }
        if let Phase::Open(day) = &mut town.phase {
            day.nili.as_mut().unwrap().stage = ManualStage::Unload(1.0);
        }
        assert!(town.finish_controlled(chapter).is_some());
    }

    fn play_market(mut town: Town, chapter: &Chapter, policy: HarborPlan, tariff: Price) -> Town {
        town.set_harbor(chapter, policy);
        for i in 0..LOTS {
            town.set_price(i, tariff);
        }
        let goal_age = chapter
            .objectives(town.chapter)
            .find_map(|goal| match goal {
                Goal::FavoriteMenu(age, _) if town.goal(goal).0 < goal.target() => Some(age),
                _ => None,
            });
        if goal_age.is_some()
            || chapter
                .goals
                .iter()
                .any(|goal| matches!(goal, Goal::Specials(_)))
        {
            let production = town.production(chapter);
            for lot in 0..LOTS {
                let Some(forecast) = town.offer_forecast(chapter, lot, &production) else {
                    continue;
                };
                let building = town.lots[lot].unwrap();
                let choice = Assortment::ALL
                    .into_iter()
                    .filter(|choice| {
                        choice.unlocked(building.level)
                            && choice
                                .flavor()
                                .is_none_or(|age| age.available(town.chapter))
                    })
                    .max_by_key(|&choice| {
                        (
                            goal_age.is_some_and(|age| choice.flavor() == Some(age)),
                            forecast.matches[choice as usize],
                            forecast.extra_coins[choice as usize],
                        )
                    })
                    .unwrap();
                town.set_assortment(lot, choice);
            }
        }
        if let Some(order) = trade_order(chapter, town.chapter)
            && town.goal(Goal::Sale(order, true)).0 < order.special_sales
        {
            let basic = town.goal(Goal::Sale(order, false)).0 < order.basic;
            for lot in chapter.layout.lots() {
                if let Some(b) = town.lots[lot]
                    && b.kind == order.shop
                {
                    if basic {
                        assert!(town.set_assortment(lot, Assortment::Standard));
                    } else if order.special.unlocked(b.level) {
                        assert!(town.set_assortment(lot, order.special));
                    }
                    town.set_price(lot, Price::Fair);
                }
            }
        }
        assert!(town.open(chapter));
        let ready = town.favor_ready_at(chapter, town.next_favor(chapter));
        if let Phase::Open(day) = &mut town.phase {
            day.nili = Some(NiliControl {
                position: [0.0, 0.10],
                yaw: 0.0,
                stage: ManualStage::Pickup,
            });
        }
        if chapter.region == crate::campaign::Region::Japan {
            use crate::world::motion;
            let temples: Vec<_> = chapter
                .layout
                .lots()
                .filter(|&lot| town.lots[lot].is_some_and(|b| b.kind == Temple))
                .collect();
            for lot in temples {
                walk_to(&mut town, chapter, motion::entrance(lot, chapter.layout));
                assert!(motion::can_inspect(&town, lot, chapter.layout));
                assert!(town.ring_bell(chapter, lot));
            }
        }
        if chapter.region == crate::campaign::Region::Nordic {
            use crate::{trade::SailingRoute, world::motion};
            let remaining = |cash: bool| {
                chapter
                    .goals
                    .iter()
                    .find_map(|&goal| {
                        if !matches!(goal, Goal::CargoIncome(_) | Goal::Voyages(_))
                            || cash != matches!(goal, Goal::CargoIncome(_))
                        {
                            return None;
                        }
                        let (have, need) = town.goal(goal);
                        Some(need.saturating_sub(have))
                    })
                    .unwrap_or(0)
            };
            let ships = remaining(false);
            let silver = remaining(true);
            if ships + silver > 0
                && let Some(lot) = chapter.layout.lots().find(|&lot| {
                    town.trade_source(chapter, lot, SailingRoute::Coastal)
                        .is_ok()
                })
            {
                let level = town.lots[lot].unwrap().level;
                let coastal = SailingRoute::Coastal;
                let overseas = SailingRoute::Overseas;
                let route = if silver > coastal.silver(level) * ships.max(1)
                    && town.trade_source(chapter, lot, overseas).is_ok()
                {
                    overseas
                } else {
                    coastal
                };
                if town.trade_source(chapter, lot, route).is_ok() {
                    walk_to(&mut town, chapter, motion::entrance(lot, chapter.layout));
                    assert!(motion::can_inspect(&town, lot, chapter.layout));
                    assert!(town.send_voyage(chapter, lot, route).is_ok());
                }
            }
        }
        if chapter.region == crate::campaign::Region::Oasis
            && chapter.goals.iter().any(|goal| {
                matches!(
                    goal,
                    Goal::Caravans(_) | Goal::CaravanIncome(_) | Goal::SpiceRoutes(_)
                )
            })
            && let Some(lot) = chapter
                .layout
                .lots()
                .find(|&lot| town.lots[lot].is_some_and(|b| b.kind == Caravanserai))
        {
            use crate::{caravan::Contract, world::motion};
            let contract = [Contract::SpiceRoad, Contract::Oasis]
                .into_iter()
                .find(|&contract| town.caravan_supply(chapter, lot, contract).is_ok());
            if let Some(contract) = contract {
                walk_to(&mut town, chapter, motion::entrance(lot, chapter.layout));
                assert!(motion::can_inspect(&town, lot, chapter.layout));
                assert!(town.send_caravan(chapter, lot, contract).is_ok());
            }
        }
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        town.tick(chapter, (ready + 0.12 - day.seconds).max(0.0));
        let demand = town.forecast(chapter);
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let best = town
            .lots
            .iter()
            .enumerate()
            .filter_map(|(i, building)| {
                let b = building.filter(|b| day.favor.allows(b.kind))?;
                town.favor_plan(chapter, i)?;
                let deficit = b
                    .kind
                    .service()
                    .map_or(0, |s| demand[s as usize].saturating_sub(day.stock[i]));
                Some((deficit, i))
            })
            .max();
        if let Some((_, lot)) = best
            && let Some(route) = town.favor_plan(chapter, lot)
        {
            assert!(town.aid(chapter, lot));
            walk_errand(&mut town, chapter, route);
        }
        help_on_street(&mut town, chapter);
        let distinct_required = chapter
            .goals
            .iter()
            .any(|g| matches!(g, Goal::RouteVariety(_)));
        let mut jobs = Job::ALL;
        jobs.sort_by_key(|&job| {
            let mandatory = chapter.goals.iter().any(|&g| {
                matches!(g, Goal::Route(candidate, count) if candidate == job && town.totals.routes[job as usize] < count)
            });
            let distinct = distinct_required && town.totals.routes[job as usize] == 0;
            (!mandatory, !distinct, job != Job::Quay)
        });
        for job in jobs {
            let Phase::Open(day) = &town.phase else { break };
            let targets = job.targets(&town, chapter);
            let selected = targets
                .iter()
                .filter_map(|&lot| {
                    let planned = job.plan(&town, chapter, lot).ok()?;
                    let service = town.lots[lot].and_then(|b| b.kind.service());
                    let stock = day.stock[lot];
                    let demand = service.map_or(0, |service| {
                        day.visits
                            .iter()
                            .filter(|v| !v.resolved && v.wants == service)
                            .count() as u16
                    });
                    let relevant = service.is_some_and(|s| {
                        chapter
                            .goals
                            .iter()
                            .any(|&g| matches!(g, Goal::Serve(kind, _) if kind == s))
                    });
                    let benefit = if job == Job::Welcome {
                        i32::from(stock.min(2)) * 20 + i32::from(relevant) * 12 - i32::from(demand)
                    } else {
                        i32::from(demand.saturating_sub(stock)) * 7 + i32::from(relevant)
                    };
                    Some((benefit, -(planned.finish as i32), lot))
                })
                .max();
            if let Some((_, _, lot)) = selected
                && let Ok(route) = town.dispatch(chapter, job, lot)
            {
                walk_errand(&mut town, chapter, route);
            }
        }
        town.tick(chapter, chapter.day_seconds());
        town
    }

    fn help_on_street(town: &mut Town, chapter: &Chapter) {
        use crate::street::{Activity, help, spots};
        let goal = chapter.objectives(town.chapter).find(|&goal| {
            matches!(goal, Goal::Dogs(_) | Goal::Clean(_)) && town.goal(goal).0 < goal.target()
        });
        let Some(goal) = goal else { return };
        let Phase::Open(day) = &town.phase else {
            return;
        };
        let fraction = if matches!(goal, Goal::Dogs(_)) {
            0.50
        } else {
            0.20
        };
        town.tick(
            chapter,
            (chapter.day_seconds() * fraction - day.seconds).max(0.0),
        );
        if let Some((i, spot)) = spots(town, chapter)
            .into_iter()
            .enumerate()
            .filter_map(|(i, spot)| Some((i, spot?)))
            .find(|(_, spot)| {
                !spot.claimed
                    && match goal {
                        Goal::Dogs(_) => spot.activity == Activity::Dog,
                        _ => !spot.activity.animal(),
                    }
            })
        {
            let [x, z] = spot.position;
            walk_to(town, chapter, bevy::prelude::Vec3::new(x, 0.0, z));
            help(town, chapter, i);
        }
    }

    #[test]
    fn all_120_harbors_can_earn_three_stars_and_unlock_the_next() {
        use crate::campaign::{JAPAN_START, NORDIC_START, REGION_SIZE};

        let mut save = Campaign::default();
        assert!(!save.unlocked(REGION_SIZE));
        assert!(!save.unlocked(JAPAN_START));
        assert!(!save.unlocked(NORDIC_START));
        let mut failures = Vec::new();
        for (i, chapter) in CHAPTERS.iter().enumerate() {
            if failures.is_empty() {
                assert!(save.unlocked(i));
            }
            assert!(
                chapter.starts.iter().all(|&(lot, kind)| {
                    is_open(chapter, lot) && chapter.allowed().contains(&kind)
                }),
                "invalid starter plots in {}",
                chapter.district
            );
            let mut town = Town::new(i, chapter);
            let layout = reference_layout(i, chapter);
            let mut result = None;
            for _ in 0..chapter.par {
                prepare_market(&mut town, chapter, &layout);
                improve_objectives(&mut town, chapter);
                for desired in [
                    DateGarden,
                    SpiceBazaar,
                    Cistern,
                    OasisHome,
                    Caravanserai,
                    Glassworks,
                    Perfumer,
                    MoonGarden,
                    CarpetHall,
                    Apiary,
                    Dyer,
                    FerryOffice,
                    GardenCourt,
                    Scribe,
                    CeramicStudio,
                    SilkShop,
                    Cooperage,
                    Fishery,
                    Smokehouse,
                    Weaver,
                    FeastHall,
                    Well,
                    Boatyard,
                    Longhouse,
                    RicePaddy,
                    RiceKitchen,
                    TeaGarden,
                    TeaHouse,
                    PaperWorkshop,
                    Temple,
                    Machiya,
                    Aqueduct,
                    Grove,
                    House,
                    Villa,
                    Bakery,
                    Potter,
                    Press,
                    Shrine,
                    Baths,
                    Inn,
                    Taverna,
                    Forum,
                    Stoa,
                ] {
                    for lot in 0..LOTS {
                        if town.day >= 3
                            && town.coins > 27
                            && town.lots[lot].is_some_and(|b| b.kind == desired)
                        {
                            town.upgrade(lot);
                        }
                    }
                }
                let policies = if chapter.harbor_plans() {
                    &HarborPlan::ALL[..]
                } else {
                    &[HarborPlan::Regular]
                };
                let mut best: Option<(f64, Town)> = None;
                for &policy in policies {
                    for tariff in Price::ALL {
                        let next = play_market(town.clone(), chapter, policy, tariff);
                        let progress: f64 = chapter
                            .objectives(i)
                            .map(|g| {
                                let (old, need) = town.goal(g);
                                let (new, _) = next.goal(g);
                                f64::from(new.min(need).saturating_sub(old.min(need)))
                                    / f64::from(need)
                            })
                            .sum();
                        let Phase::Report(report) = &next.phase else {
                            unreachable!()
                        };
                        let score = progress * 2400.0
                            + 1_000.0 * f64::from(report.stars)
                            + 18.0 * f64::from(report.tally.favors)
                            + f64::from(report.tally.served_total())
                            + f64::from(report.tally.income) * 0.18;
                        if best.as_ref().is_none_or(|(prior, _)| score > *prior) {
                            best = Some((score, next));
                        }
                    }
                }
                town = best.expect("a policy is always available").1;
                let Phase::Report(report) = &town.phase else {
                    unreachable!()
                };
                if report.stars == 3 {
                    save.record(i, town.day, report.stars);
                    save.town = Some(town.clone());
                    let encoded = serde_json::to_vec(&save).unwrap();
                    save = Campaign::decode(encoded.as_slice()).unwrap_or_else(|| {
                        panic!("Cannot resume {} after completion", chapter.district)
                    });
                    assert_eq!(save.records[i].stars, 3);
                    assert_eq!(save.town.as_ref().unwrap().day, town.day);
                    result = Some((town.day, report.stars));
                    break;
                }
                town.advance();
            }
            let Some((days, stars)) = result else {
                failures.push(format!(
                    "{}: {:?}, Hafenbitten {}/{}",
                    chapter.district,
                    chapter
                        .objectives(i)
                        .map(|goal| (goal.label(), town.goal(goal)))
                        .collect::<Vec<_>>(),
                    town.totals.favors,
                    chapter.favor_goal
                ));
                continue;
            };
            eprintln!(
                "{:02} {:<15} {days} / {} Tage, {stars} Sterne",
                i + 1,
                chapter.district,
                chapter.par
            );
        }
        assert!(
            failures.is_empty(),
            "Unfinished routes: {}",
            failures.join("\n")
        );
    }

    #[test]
    fn nili_can_restock_and_finish_a_short_errand_late_in_the_shift() {
        let mut chapter = CHAPTERS[9];
        chapter.shift = crate::campaign::Shift::Long;
        chapter.favors = &[Favor::Jugs];
        chapter.starts = &[(2, Well), (7, Bakery)];
        let mut town = Town::new(9, &chapter);
        let lot = chapter
            .layout
            .lots()
            .find(|&lot| town.lots[lot].is_some_and(|b| b.kind == Well))
            .unwrap();
        assert!(town.open(&chapter));
        let close = chapter.day_seconds();
        assert_eq!(chapter.shift.clock(1.0), "21:30");
        town.tick(&chapter, close - 3.3);
        if let Phase::Open(day) = &mut town.phase {
            day.favor = Favor::Jugs;
            let position = crate::world::motion::entrance(lot, chapter.layout);
            day.nili = Some(NiliControl {
                position: [position.x, position.z],
                yaw: 0.0,
                stage: ManualStage::Pickup,
            });
        }
        let route = town.favor_plan(&chapter, lot).unwrap();
        assert!(route.finish < close);
        assert!(town.aid(&chapter, lot));
        walk_errand(&mut town, &chapter, route);
        let Phase::Open(day) = &town.phase else {
            panic!("Shift ended before Nili finished")
        };
        assert_eq!(day.tally.favors, 1);
        assert!(day.seconds < close);
    }

    #[test]
    fn a_restock_extends_the_inventory_total_and_old_saves_keep_their_count() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.open(chapter));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        let start = day.stock_capacity(2);
        day.stock[2] -= 1;
        assert_eq!(day.stock_capacity(2), start);
        day.restock(2, 3);
        assert_eq!(day.stock_capacity(2), start + 3);
        assert_eq!(day.stock[2], start + 2);
        day.supplied = None;
        assert_eq!(day.stock_capacity(2), start + 2);
    }

    #[test]
    fn a_temple_bell_admits_three_pilgrims_once_when_a_lampion_workshop_is_next_door() {
        use crate::campaign::JAPAN_START;

        let index = JAPAN_START + 4;
        let mut chapter = CHAPTERS[index];
        chapter.ferry = 0;
        chapter.locals = &[Service::Prayer];
        chapter.guests = &[Service::Prayer];
        let mut town = Town::new(index, &chapter);
        assert!(!town.ring_bell(&chapter, 8));
        assert!(town.build(&chapter, 9, PaperWorkshop));
        assert!(!town.set_price(8, Price::Fine));
        assert!(town.open(&chapter));
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let before = (day.visits.len(), day.stock[8]);
        assert!(town.ring_bell(&chapter, 8));
        let serialized = serde_json::to_vec(&town).unwrap();
        let mut resumed: Town = serde_json::from_slice(&serialized).unwrap();
        assert!(!resumed.ring_bell(&chapter, 8));
        let Phase::Open(day) = &resumed.phase else {
            unreachable!()
        };
        assert_eq!(day.visits.len(), before.0 + 3);
        assert_eq!(day.stock[8], before.1 + 3);
        assert!(
            day.visits[before.0..]
                .iter()
                .all(|visit| { visit.wants == Service::Prayer && visit.guided_to == Some(8) })
        );
        assert!(resumed.tick(&chapter, chapter.day_seconds()));
        assert_eq!(resumed.goal(Goal::Bell(1)), (1, 1));
        assert_eq!(
            resumed.totals.served[Service::Prayer as usize],
            before.0 as u32 + 3
        );
        assert_eq!(resumed.totals.income, 0);
    }

    #[test]
    fn weather_varies_inside_each_day_and_big_harbors_have_more_time() {
        let mut observed = Vec::new();
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            let mut town = Town::new(index, chapter);
            for day_number in 1..=8 {
                town.day = day_number;
                let schedule = town.weather_schedule(chapter);
                assert!(schedule.windows(2).any(|pair| pair[0] != pair[1]));
                assert_eq!(town.sky(chapter), schedule[0]);
                observed.extend(schedule);
                assert!(town.open(chapter));
                for (step, phase) in [(0.40, 1), (0.40, 2)] {
                    assert!(!town.tick(chapter, chapter.day_seconds() * step));
                    assert_eq!(town.sky(chapter), schedule[phase]);
                }
                assert!(town.tick(chapter, chapter.day_seconds()));
                assert!(town.advance());
            }
        }
        assert!(observed.contains(&Weather::Rain));
        assert!(observed.contains(&Weather::Storm));
        assert!(CHAPTERS[47].day_seconds() >= 115.0);
        assert!(CHAPTERS[47].extra_ferry_guests() > CHAPTERS[0].extra_ferry_guests());
    }

    #[test]
    fn each_region_introduces_age_groups_in_the_same_order() {
        for start in [0, crate::campaign::REGION_SIZE] {
            for (offset, expected) in [
                (0, [false, true, false]),
                (2, [true, true, false]),
                (4, [true; 3]),
            ] {
                let index = start + offset;
                let chapter = &CHAPTERS[index];
                let town = Town::new(index, chapter);
                let ages = town.age_forecast(chapter);
                assert_eq!(
                    ages.map(|count| count > 0),
                    expected,
                    "{}",
                    chapter.district
                );
            }
        }
    }

    #[test]
    fn matched_offers_predict_the_cash_paid_and_unlock_through_six() {
        let chapter = &CHAPTERS[4];
        let mut town = Town::new(4, chapter);
        town.lots.fill(None);
        town.coins = 400;
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.build(chapter, 8, Well));
        assert!(town.build(chapter, 1, House));
        assert!(town.set_price(2, Price::Fair));
        assert!(!town.set_assortment(2, Assortment::Festival));
        for step in 2..=MAX_LEVEL {
            assert!(town.upgrade(2));
            let new = Assortment::ALL
                .into_iter()
                .find(|choice| choice.tier() == step)
                .unwrap();
            assert!(town.set_assortment(2, new));
        }
        assert!(!town.upgrade(2));
        let ages = town.age_forecast(chapter);
        assert_eq!(
            usize::from(ages.iter().sum::<u16>()),
            town.visit_plan(chapter).len()
        );
        assert_eq!(
            town.household(1).iter().sum::<u16>(),
            town.lots[1].unwrap().residents()
        );
        let forecast = town
            .offer_forecast(chapter, 2, &town.production(chapter))
            .unwrap();
        let expected = forecast.extra_coins[Assortment::Celebration as usize];
        assert!(expected > 0);
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert_eq!(town.last_day.as_ref().unwrap().specialty_income, expected);
    }

    #[test]
    fn price_cards_predict_the_sales_coins_and_misses_of_each_price() {
        let mut cheaper_wins = 0;
        let mut dearer_wins = 0;
        for (index, chapter) in CHAPTERS.iter().enumerate().step_by(5) {
            let mut town = Town::new(index, chapter);
            prepare_market(&mut town, chapter, &reference_layout(index, chapter));
            let production = town.production(chapter);
            for lot in chapter.layout.lots() {
                let Some(forecast) = town.price_forecast(chapter, lot, &production) else {
                    continue;
                };
                let service = town.lots[lot].unwrap().kind.service().unwrap();
                for price in Price::ALL {
                    let mut played = town.clone();
                    assert!(played.set_price(lot, price));
                    assert!(played.open(chapter));
                    assert!(played.tick(chapter, chapter.day_seconds()));
                    let here = || {
                        played
                            .last_visits
                            .iter()
                            .filter(move |visit| visit.wants == service)
                    };
                    let sold = || here().filter(|visit| visit.shop == Some(lot as u8));
                    let actual = (
                        sold().count() as u16,
                        sold()
                            .map(|visit| price.coins(service) + visit.specialty_bonus)
                            .sum::<u32>(),
                        here()
                            .filter(|visit| {
                                visit.shop.is_none() && visit.declined_at == Some(lot as u8)
                            })
                            .count() as u16,
                    );
                    let i = price as usize;
                    assert_eq!(
                        (forecast.sales[i], forecast.coins[i], forecast.misses[i]),
                        actual,
                        "{} plot {lot}: {}",
                        chapter.district,
                        price.label()
                    );
                }
                match forecast.best() {
                    Some(Price::Fair) => cheaper_wins += 1,
                    Some(Price::Fine) => dearer_wins += 1,
                    _ => {}
                }
            }
            // Prices are set in the morning, so there is nothing to forecast once the day runs.
            assert!(town.open(chapter));
            assert!(
                chapter
                    .layout
                    .lots()
                    .all(|lot| town.price_forecast(chapter, lot, &production).is_none())
            );
        }
        // Which price earns most depends on the street, so the cards must not guess.
        assert!(
            cheaper_wins > 0 && dearer_wins > 0,
            "{cheaper_wins} / {dearer_wins}"
        );
    }

    #[test]
    fn shade_work_reaches_every_resting_place_including_gardens() {
        for kind in [Stoa, Baths, TeaHouse, Temple, GardenCourt, MoonGarden] {
            assert!(kind.rest_stop(), "{}", kind.name());
        }
        assert!(!Bakery.rest_stop() && !Grove.rest_stop());
        let index = crate::campaign::REGION_SIZE + 5;
        let chapter = &CHAPTERS[index];
        let mut town = Town::new(index, chapter);
        town.lots.fill(None);
        for (lot, kind) in [(0, Grove), (1, GardenCourt), (2, MoonGarden), (3, Bakery)] {
            town.lots[lot] = Some(Building::new(kind));
        }
        assert!(town.open(chapter));
        assert_eq!(Job::Shade.targets(&town, chapter), [1, 2]);
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        let before = day.stock;
        day.favor = Favor::Shade;
        let lots = town.lots;
        finish_favor(&lots, day, 0, 10.0, chapter.day_seconds(), index, None);
        assert_eq!(day.stock[1], before[1] + 2);
        assert_eq!(day.stock[2], before[2] + 2);
        assert_eq!(day.stock[3], before[3]);
    }

    #[test]
    fn feast_halls_take_two_smoked_fish_each_and_never_overdraw_the_smokehouse() {
        let index = crate::campaign::NORDIC_START + 10;
        let chapter = &CHAPTERS[index];
        let mut town = Town::new(index, chapter);
        town.lots.fill(None);
        town.lots[5] = Some(Building::new(Fishery));
        let halls = [1, 2, 3, 6, 7];
        for lot in halls {
            town.lots[lot] = Some(Building::new(FeastHall));
        }
        let without = town.production(chapter);
        town.lots[0] = Some(Building::new(Smokehouse));
        let with = town.production(chapter);
        // Nine portions feed four halls; the fifth finds only one portion left.
        assert_eq!(with.stock[0], 1);
        let extra: Vec<_> = halls
            .iter()
            .map(|&lot| with.stock[lot] - without.stock[lot])
            .collect();
        assert_eq!(extra, [6, 5, 5, 6, 0]);
    }

    #[test]
    fn herb_beds_supply_only_kitchens_and_do_not_stack() {
        let chapter = &CHAPTERS[11];
        let mut town = Town::new(11, chapter);
        town.lots.fill(None);
        for (lot, kind) in [(1, Bakery), (7, Taverna), (2, Potter)] {
            town.lots[lot] = Some(Building::new(kind));
        }
        let before = town.production(chapter);
        town.lots[0] = Some(Building::new(HerbGarden));
        town.lots[8] = Some(Building::new(HerbGarden));
        let after = town.production(chapter);
        assert_eq!(after.water, before.water);
        assert_eq!(after.stock[1], before.stock[1] + 2);
        assert_eq!(after.stock[7], before.stock[7] + 2);
        assert_eq!(after.stock[2], before.stock[2]);
    }

    #[test]
    fn live_goals_count_the_running_day_before_it_closes() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fair));
        let serve = Goal::Serve(Service::Bread, 15);
        assert_eq!(town.goals_met(chapter), 0b001);
        assert!(town.open(chapter));
        assert!(!town.tick(chapter, chapter.day_seconds() * 0.6));
        assert_eq!(town.goal(serve).0, 0);
        let (sold, _) = town.live_goal(serve);
        assert!(sold > 0);
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert_eq!(
            town.goal(serve).0,
            town.totals.served[Service::Bread as usize]
        );
        assert!(town.goal(serve).0 >= sold);
    }

    #[test]
    fn longer_shifts_allow_more_tours() {
        assert_eq!(Job::limit(0), 3);
        let longest = (0..CHAPTERS.len())
            .max_by(|&a, &b| {
                CHAPTERS[a]
                    .day_seconds()
                    .total_cmp(&CHAPTERS[b].day_seconds())
            })
            .unwrap();
        assert!(Job::limit(longest) >= 7, "{}", Job::limit(longest));
        for index in 1..CHAPTERS.len() {
            let base = [3, 4, 5][usize::from(index >= 6) + usize::from(index >= 12)];
            assert!(Job::limit(index) >= base);
        }
    }

    #[test]
    fn day_events_only_add_guests_who_find_what_they_want() {
        let mut seen = Vec::new();
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            let mut town = Town::new(index, chapter);
            for day in 1..=8 {
                town.day = day;
                let regular = town.visit_plan(chapter);
                let planned = town.planned_visits(chapter);
                let event = town.day_event(chapter);
                assert_eq!(event, town.day_event(chapter));
                let Some(gathering) = event else {
                    assert_eq!(planned.len(), regular.len());
                    continue;
                };
                assert!(day >= 2 && harbor_stage(index) >= 2, "{}", chapter.district);
                assert!(gathering.guests > 0 && usize::from(gathering.guests) <= EVENT_GUESTS);
                assert_eq!(planned.len(), regular.len() + usize::from(gathering.guests));
                seen.push(gathering.event);

                // The same market without the group serves every regular guest just as well.
                let mut quiet = town.clone();
                assert!(quiet.open(chapter));
                let Phase::Open(day) = &mut quiet.phase else {
                    unreachable!()
                };
                assert_eq!(day.event, Some(gathering));
                day.visits.truncate(regular.len());
                let mut busy = town.clone();
                assert!(busy.open(chapter));
                let packed = serde_json::to_vec(&busy).unwrap();
                let mut busy: Town = serde_json::from_slice(&packed).unwrap();
                assert!(quiet.tick(chapter, chapter.day_seconds()));
                assert!(busy.tick(chapter, chapter.day_seconds()));
                let served = |visits: &[Visit]| visits.iter().filter(|v| v.shop.is_some()).count();
                assert!(served(&busy.last_visits[..regular.len()]) >= served(&quiet.last_visits));
                assert!(
                    busy.last_visits[regular.len()..]
                        .iter()
                        .all(|visit| visit.shop.is_some()),
                    "{} Tag {}",
                    chapter.district,
                    town.day
                );
            }
        }
        for event in DayEvent::ALL {
            assert!(seen.contains(&event), "{event:?}");
        }
        let share = seen.len() as f32 / (CHAPTERS.len() * 7) as f32;
        assert!((0.15..0.45).contains(&share), "{share}");
    }

    #[test]
    fn five_happy_guests_in_a_row_earn_renown_and_a_miss_resets_the_streak() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.set_price(2, Price::Fair));
        assert!(town.open(chapter));
        let lots = town.lots;
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        let guest = |at: f32| Visit {
            origin: Origin::Home(1),
            age: Some(Age::Adult),
            specialty_bonus: 0,
            wants: Service::Bread,
            budget: Price::Regular,
            at,
            resolved: false,
            shop: None,
            miss: None,
            guided_to: None,
            declined_at: None,
        };
        day.visits = (0..9).map(|n| guest(1.0 + n as f32)).collect();
        day.stock[2] = 7;
        resolve_visits(&lots, day, chapter, 20.0, None);
        assert_eq!(day.tally.served_total(), 7);
        assert_eq!(day.tally.best_streak, 7);
        assert_eq!(day.tally.streak_bonus, 1);
        assert_eq!(day.tally.reputation, 7 + 1);
        assert_eq!(day.streak, 0);
        day.stock[2] = 5;
        day.visits.extend((0..5).map(|n| guest(21.0 + n as f32)));
        resolve_visits(&lots, day, chapter, 30.0, None);
        assert_eq!(day.streak, 5);
        assert_eq!(day.tally.streak_bonus, 2);
    }

    #[test]
    fn day_records_start_on_the_second_day_and_name_each_new_best() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.open(chapter));
        assert!(town.tick(chapter, chapter.day_seconds()));
        let Phase::Report(report) = &town.phase else {
            unreachable!()
        };
        assert_eq!(report.records, 0);
        let before = town.totals.clone();
        let mut today = before.clone();
        today.record_income += 1;
        today.best_ferry_day += 2;
        let records = DayRecord::broken(&today, &before);
        assert_eq!(
            DayRecord::list(records).collect::<Vec<_>>(),
            [DayRecord::Income, DayRecord::Ferry]
        );
    }

    #[test]
    fn the_daily_extra_grows_with_yesterday_and_pays_more_later() {
        let index = 9;
        let chapter = &CHAPTERS[index];
        let mut town = Town::new(index, chapter);
        while (town.day as usize + index) % 6 != 1 {
            town.day += 1;
        }
        let floor = match town.daily_contract(chapter) {
            Some(DailyContract::Turnover(goal)) => goal,
            _ => panic!("expected a turnover day"),
        };
        town.last_day = Some(Tally {
            income: 400,
            ..Tally::default()
        });
        assert!(matches!(
            town.daily_contract(chapter),
            Some(DailyContract::Turnover(440))
        ));
        town.last_day = Some(Tally::default());
        assert!(matches!(
            town.daily_contract(chapter),
            Some(DailyContract::Turnover(goal)) if goal == floor
        ));
        let reward = DailyContract::Turnover(1).reward(index);
        assert_eq!(reward, 5 + 9 / 2);
        assert!(DailyContract::Turnover(1).reward(23) > reward);
    }

    #[test]
    fn a_quick_rescue_pays_the_rush_bonus_once() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert!(town.build(chapter, 2, Bakery));
        assert!(town.open(chapter));
        let lots = town.lots;
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.favor = Favor::Fire;
        let mut late = day.clone();
        finish_favor(&lots, day, 2, 9.0, chapter.day_seconds(), 0, Some(10.0));
        assert!(day.favor_rushed);
        assert_eq!(day.tally.income, 6 + RUSH_COINS);
        assert_eq!(day.tally.reputation, 2 + RUSH_RENOWN);
        finish_favor(
            &lots,
            &mut late,
            2,
            11.0,
            chapter.day_seconds(),
            0,
            Some(10.0),
        );
        assert!(!late.favor_rushed);
        assert_eq!(late.tally.income, 6);
        assert!(town.rush_deadline(chapter).is_none());
    }

    #[test]
    fn guests_can_be_guided_twice_and_unfinished_work_expires_at_dusk() {
        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        assert!(town.open(chapter));
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.nili = Some(NiliControl {
            position: [0.0, 0.1],
            yaw: 0.0,
            stage: ManualStage::Pickup,
        });
        for _ in 0..2 {
            let route = town.dispatch(chapter, Job::Welcome, 7).unwrap();
            walk_errand(&mut town, chapter, route);
        }
        assert!(matches!(
            Job::Welcome.plan(&town, chapter, 7),
            Err(Blocked::Done)
        ));
        assert_eq!(
            town.guest_history()
                .iter()
                .filter(|visit| visit.guided_to.is_some())
                .count(),
            4
        );
        assert!(town.dispatch(chapter, Job::Freight, 7).is_ok());
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert_eq!(
            town.last_day.as_ref().unwrap().routes[Job::Freight as usize],
            0
        );
    }
}

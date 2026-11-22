use serde::{Deserialize, Serialize};

use crate::campaign::{CORE_HARBORS, JAPAN_START, NORDIC_START, OASIS_START, Region};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Category {
    #[default]
    Garment,
    Head,
    Accessory,
}

impl Category {
    pub const ALL: [Self; 3] = [Self::Garment, Self::Head, Self::Accessory];

    pub fn label(self) -> &'static str {
        match self {
            Self::Garment => "Gewänder",
            Self::Head => "Hüte",
            Self::Accessory => "Schmuck & Taschen",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Garment => Cosmetic::Sea.icon(),
            Self::Head => Cosmetic::Straw.icon(),
            Self::Accessory => Cosmetic::Pearls.icon(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cosmetic {
    Sea,
    Saffron,
    Toga,
    Straw,
    Laurel,
    Petasos,
    Olive,
    Linen,
    Coral,
    Night,
    Mosaic,
    Sunbonnet,
    Indigo,
    Diadem,
    Wreath,
    Headband,
    Satchel,
    Shell,
    Pearls,
    Brooch,
    Scarf,
    Belt,
    Basket,
    Amulet,
    IndigoKosode,
    AutumnKosode,
    Haori,
    Kasa,
    PilgrimHat,
    Kanzashi,
    Obi,
    Inro,
    FjordTunic,
    DyedCloak,
    WinterCoat,
    WoolHood,
    SailorsCap,
    BraidBand,
    AmberBrooch,
    WovenBag,
    LapisRobe,
    DuneWrap,
    GlassPendant,
    AegeanChiton,
    OlivePeplos,
    MyrtleCrown,
    FoldedPetasos,
    ShellPin,
    MulberryToga,
    MosaicTunic,
    RedPileus,
    GoldLaurel,
    SignetCord,
    SakuraKosode,
    MinoCape,
    RiverKasa,
    CranePin,
    RicePouch,
    SeaCloak,
    TabletDress,
    FurCap,
    KnitBand,
    KnotPendant,
    OasisCaftan,
    MoonSilk,
    StarTurban,
    CaravanVeil,
    RoseFlask,
}

impl Cosmetic {
    pub const ALL: [Self; 68] = [
        Self::Sea,
        Self::Saffron,
        Self::Toga,
        Self::Straw,
        Self::Laurel,
        Self::Petasos,
        Self::Olive,
        Self::Linen,
        Self::Coral,
        Self::Night,
        Self::Mosaic,
        Self::Sunbonnet,
        Self::Indigo,
        Self::Diadem,
        Self::Wreath,
        Self::Headband,
        Self::Satchel,
        Self::Shell,
        Self::Pearls,
        Self::Brooch,
        Self::Scarf,
        Self::Belt,
        Self::Basket,
        Self::Amulet,
        Self::IndigoKosode,
        Self::AutumnKosode,
        Self::Haori,
        Self::Kasa,
        Self::PilgrimHat,
        Self::Kanzashi,
        Self::Obi,
        Self::Inro,
        Self::FjordTunic,
        Self::DyedCloak,
        Self::WinterCoat,
        Self::WoolHood,
        Self::SailorsCap,
        Self::BraidBand,
        Self::AmberBrooch,
        Self::WovenBag,
        Self::LapisRobe,
        Self::DuneWrap,
        Self::GlassPendant,
        Self::AegeanChiton,
        Self::OlivePeplos,
        Self::MyrtleCrown,
        Self::FoldedPetasos,
        Self::ShellPin,
        Self::MulberryToga,
        Self::MosaicTunic,
        Self::RedPileus,
        Self::GoldLaurel,
        Self::SignetCord,
        Self::SakuraKosode,
        Self::MinoCape,
        Self::RiverKasa,
        Self::CranePin,
        Self::RicePouch,
        Self::SeaCloak,
        Self::TabletDress,
        Self::FurCap,
        Self::KnitBand,
        Self::KnotPendant,
        Self::OasisCaftan,
        Self::MoonSilk,
        Self::StarTurban,
        Self::CaravanVeil,
        Self::RoseFlask,
    ];

    pub fn name(self) -> &'static str {
        if let Some(item) = self.collection() {
            return item.name;
        }
        [
            "Meerblau",
            "Safrangewand",
            "Purpurtoga",
            "Fischerhut",
            "Lorbeerkranz",
            "Reisehut",
            "Oliventunika",
            "Leinentoga",
            "Korallenkleid",
            "Nachttunika",
            "Mosaiktoga",
            "Sonnenhaube",
            "Indigokappe",
            "Diadem",
            "Blütenkranz",
            "Stirnband",
            "Ledertasche",
            "Muschelkette",
            "Perlenkette",
            "Jadefibel",
            "Halstuch",
            "Goldgürtel",
            "Weidenkorb",
            "Bernstein",
            "Indigo-Kosode",
            "Ahorn-Kosode",
            "Fest-Haori",
            "Bambus-Kasa",
            "Pilgerhut",
            "Kanzashi",
            "Seidenobi",
            "Lack-Inrō",
            "Fjordtunika",
            "Krappmantel",
            "Festmantel",
            "Wollkapuze",
            "Seglermütze",
            "Flechtband",
            "Bernsteinfibel",
            "Webtasche",
            "Lapisgewand",
            "Dünentuch",
            "Glasanhänger",
        ][self as usize]
    }

    pub fn icon(self) -> &'static str {
        if let Some(item) = self.collection() {
            return item.icon;
        }
        [
            "wear_sea",
            "wear_saffron",
            "wear_toga",
            "wear_straw",
            "wear_laurel",
            "wear_petasos",
            "wear_olive",
            "wear_linen",
            "wear_coral",
            "wear_night",
            "wear_mosaic",
            "wear_sunbonnet",
            "wear_indigo",
            "wear_diadem",
            "wear_wreath",
            "wear_headband",
            "wear_satchel",
            "wear_shell",
            "wear_pearls",
            "wear_brooch",
            "wear_scarf",
            "wear_belt",
            "wear_basket",
            "wear_amulet",
            "wear_indigo_kosode",
            "wear_ahorn",
            "wear_haori",
            "wear_kasa",
            "wear_pilgrim",
            "wear_kanzashi",
            "wear_obi",
            "wear_inro",
            "wear_fjord",
            "wear_krapp",
            "wear_winter",
            "wear_wool",
            "wear_sailor",
            "wear_braid",
            "wear_amber_brooch",
            "wear_woven_bag",
            "wear_lapis",
            "wear_dune_wrap",
            "wear_glass_pendant",
        ][self as usize]
    }

    pub fn cost(self) -> u16 {
        if let Some(item) = self.collection() {
            return item.cost;
        }
        [
            3, 6, 16, 4, 12, 9, 4, 5, 5, 6, 7, 4, 5, 8, 6, 3, 4, 3, 6, 4, 4, 5, 5, 6, 4, 6, 8, 4,
            6, 7, 5, 7, 4, 6, 8, 4, 6, 6, 5, 7, 5, 6, 8,
        ][self as usize]
    }

    pub fn category(self) -> Category {
        if let Some(item) = self.collection() {
            return item.category;
        }
        match self {
            Self::Sea
            | Self::Saffron
            | Self::Toga
            | Self::Olive
            | Self::Linen
            | Self::Coral
            | Self::Night
            | Self::Mosaic
            | Self::IndigoKosode
            | Self::AutumnKosode
            | Self::Haori
            | Self::FjordTunic
            | Self::DyedCloak
            | Self::WinterCoat
            | Self::LapisRobe => Category::Garment,
            Self::Straw
            | Self::Laurel
            | Self::Petasos
            | Self::Sunbonnet
            | Self::Indigo
            | Self::Diadem
            | Self::Wreath
            | Self::Headband
            | Self::Kasa
            | Self::PilgrimHat
            | Self::Kanzashi
            | Self::WoolHood
            | Self::SailorsCap
            | Self::BraidBand
            | Self::DuneWrap => Category::Head,
            _ => Category::Accessory,
        }
    }

    pub fn requires_chapter(self) -> Option<usize> {
        if let Some(item) = self.collection() {
            return Some(item.region.start() + item.stage);
        }
        let island = match self {
            Self::IndigoKosode => 0,
            Self::AutumnKosode => 9,
            Self::Haori => 23,
            Self::Kasa => 1,
            Self::PilgrimHat => 7,
            Self::Kanzashi => 15,
            Self::Obi => 4,
            Self::Inro => 19,
            Self::FjordTunic => return Some(NORDIC_START),
            Self::DyedCloak => return Some(NORDIC_START + 5),
            Self::WinterCoat => return Some(NORDIC_START + 23),
            Self::WoolHood => return Some(NORDIC_START + 1),
            Self::SailorsCap => return Some(NORDIC_START + 7),
            Self::BraidBand => return Some(NORDIC_START + 14),
            Self::AmberBrooch => return Some(NORDIC_START + 4),
            Self::WovenBag => return Some(NORDIC_START + 19),
            Self::LapisRobe => return Some(OASIS_START),
            Self::DuneWrap => return Some(OASIS_START + 4),
            Self::GlassPendant => return Some(OASIS_START + 15),
            _ => return None,
        };
        Some(JAPAN_START + island)
    }

    fn collection(self) -> Option<&'static Collection> {
        COLLECTION.get((self as usize).checked_sub(Self::LapisRobe as usize + 3)?)
    }

    pub fn requires_adventure(self) -> Option<usize> {
        let item = self.collection()?;
        Some(item.region as usize * CORE_HARBORS + item.stage / 2)
    }

    pub fn region(self) -> Option<Region> {
        self.requires_chapter().map(Region::of)
    }
}

struct Collection {
    name: &'static str,
    icon: &'static str,
    region: Region,
    category: Category,
    stage: usize,
    cost: u16,
}

const COLLECTION: [Collection; 25] = [
    Collection {
        name: "Meeres-Chiton",
        icon: "wear_aegean_chiton",
        region: Region::Greece,
        category: Category::Garment,
        stage: 2,
        cost: 5,
    },
    Collection {
        name: "Olivenpeplos",
        icon: "wear_olive_peplos",
        region: Region::Greece,
        category: Category::Garment,
        stage: 8,
        cost: 8,
    },
    Collection {
        name: "Myrtenkranz",
        icon: "wear_myrtle_crown",
        region: Region::Greece,
        category: Category::Head,
        stage: 4,
        cost: 5,
    },
    Collection {
        name: "Faltpetasos",
        icon: "wear_folded_petasos",
        region: Region::Greece,
        category: Category::Head,
        stage: 12,
        cost: 8,
    },
    Collection {
        name: "Perlmuttfibel",
        icon: "wear_shell_pin",
        region: Region::Greece,
        category: Category::Accessory,
        stage: 16,
        cost: 9,
    },
    Collection {
        name: "Maulbeertoga",
        icon: "wear_mulberry_toga",
        region: Region::Rome,
        category: Category::Garment,
        stage: 2,
        cost: 5,
    },
    Collection {
        name: "Aventin-Tunika",
        icon: "wear_aventin_tunic",
        region: Region::Rome,
        category: Category::Garment,
        stage: 8,
        cost: 8,
    },
    Collection {
        name: "Roter Pileus",
        icon: "wear_red_pileus",
        region: Region::Rome,
        category: Category::Head,
        stage: 4,
        cost: 5,
    },
    Collection {
        name: "Goldlorbeer",
        icon: "wear_gold_laurel",
        region: Region::Rome,
        category: Category::Head,
        stage: 12,
        cost: 8,
    },
    Collection {
        name: "Siegel am Band",
        icon: "wear_signet_cord",
        region: Region::Rome,
        category: Category::Accessory,
        stage: 16,
        cost: 9,
    },
    Collection {
        name: "Sakura-Kosode",
        icon: "wear_sakura_kosode",
        region: Region::Japan,
        category: Category::Garment,
        stage: 2,
        cost: 5,
    },
    Collection {
        name: "Mino-Regenumhang",
        icon: "wear_mino_cape",
        region: Region::Japan,
        category: Category::Garment,
        stage: 8,
        cost: 8,
    },
    Collection {
        name: "Fluss-Kasa",
        icon: "wear_river_kasa",
        region: Region::Japan,
        category: Category::Head,
        stage: 4,
        cost: 5,
    },
    Collection {
        name: "Kranichspange",
        icon: "wear_crane_pin",
        region: Region::Japan,
        category: Category::Head,
        stage: 12,
        cost: 8,
    },
    Collection {
        name: "Genähter Reisbeutel",
        icon: "wear_rice_pouch",
        region: Region::Japan,
        category: Category::Accessory,
        stage: 16,
        cost: 9,
    },
    Collection {
        name: "Küstenmantel",
        icon: "wear_sea_cloak",
        region: Region::Nordic,
        category: Category::Garment,
        stage: 2,
        cost: 5,
    },
    Collection {
        name: "Webkleid",
        icon: "wear_tablet_dress",
        region: Region::Nordic,
        category: Category::Garment,
        stage: 8,
        cost: 8,
    },
    Collection {
        name: "Winterkappe",
        icon: "wear_fur_cap",
        region: Region::Nordic,
        category: Category::Head,
        stage: 4,
        cost: 5,
    },
    Collection {
        name: "Geflochtener Wollreif",
        icon: "wear_knit_band",
        region: Region::Nordic,
        category: Category::Head,
        stage: 12,
        cost: 8,
    },
    Collection {
        name: "Knotenanhänger",
        icon: "wear_knot_pendant",
        region: Region::Nordic,
        category: Category::Accessory,
        stage: 16,
        cost: 9,
    },
    Collection {
        name: "Palmengarten-Kaftan",
        icon: "wear_oasis_caftan",
        region: Region::Oasis,
        category: Category::Garment,
        stage: 2,
        cost: 5,
    },
    Collection {
        name: "Mondseide",
        icon: "wear_moon_silk",
        region: Region::Oasis,
        category: Category::Garment,
        stage: 8,
        cost: 8,
    },
    Collection {
        name: "Sternenturban",
        icon: "wear_star_turban",
        region: Region::Oasis,
        category: Category::Head,
        stage: 4,
        cost: 5,
    },
    Collection {
        name: "Karawanentuch",
        icon: "wear_caravan_veil",
        region: Region::Oasis,
        category: Category::Head,
        stage: 12,
        cost: 8,
    },
    Collection {
        name: "Rosenflakon",
        icon: "wear_rose_flask",
        region: Region::Oasis,
        category: Category::Accessory,
        stage: 16,
        cost: 9,
    },
];

#[derive(Default, Deserialize, Serialize)]
pub struct Wardrobe {
    owned: u64,
    #[serde(default, skip_serializing_if = "empty_bits")]
    expedition_owned: u64,
    garment: Option<Cosmetic>,
    head: Option<Cosmetic>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    accessory: Option<Cosmetic>,
}

fn empty_bits(bits: &u64) -> bool {
    *bits == 0
}

impl Wardrobe {
    pub fn is_empty(&self) -> bool {
        self.owned == 0 && self.expedition_owned == 0
    }

    pub fn owns(&self, item: Cosmetic) -> bool {
        let index = item as usize;
        if index < Cosmetic::AegeanChiton as usize {
            self.owned & (1 << index) != 0
        } else {
            self.expedition_owned & (1 << (index - Cosmetic::AegeanChiton as usize)) != 0
        }
    }

    pub fn wearing(&self, item: Cosmetic) -> bool {
        let slot = match item.category() {
            Category::Garment => self.garment,
            Category::Head => self.head,
            Category::Accessory => self.accessory,
        };
        slot == Some(item)
    }

    pub fn spent(&self) -> u16 {
        Cosmetic::ALL
            .into_iter()
            .filter(|&item| self.owns(item))
            .map(Cosmetic::cost)
            .sum()
    }

    pub fn equip(&mut self, item: Cosmetic, available: u16) -> bool {
        if !self.owns(item) && item.cost() > available {
            return false;
        }
        let index = item as usize;
        if index < Cosmetic::AegeanChiton as usize {
            self.owned |= 1 << index;
        } else {
            self.expedition_owned |= 1 << (index - Cosmetic::AegeanChiton as usize);
        }
        let slot = match item.category() {
            Category::Garment => &mut self.garment,
            Category::Head => &mut self.head,
            Category::Accessory => &mut self.accessory,
        };
        *slot = (*slot != Some(item)).then_some(item);
        true
    }
}

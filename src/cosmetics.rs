use serde::{Deserialize, Serialize};

use crate::campaign::{JAPAN_START, NORDIC_START, Region};

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
}

impl Cosmetic {
    pub const ALL: [Self; 40] = [
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
    ];

    pub fn name(self) -> &'static str {
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
        ][self as usize]
    }

    pub fn icon(self) -> &'static str {
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
        ][self as usize]
    }

    pub fn cost(self) -> u16 {
        [
            3, 6, 16, 4, 12, 9, 4, 5, 5, 6, 7, 4, 5, 8, 6, 3, 4, 3, 6, 4, 4, 5, 5, 6, 4, 6, 8, 4,
            6, 7, 5, 7, 4, 6, 8, 4, 6, 6, 5, 7,
        ][self as usize]
    }

    pub fn category(self) -> Category {
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
            | Self::WinterCoat => Category::Garment,
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
            | Self::BraidBand => Category::Head,
            _ => Category::Accessory,
        }
    }

    pub fn requires_chapter(self) -> Option<usize> {
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
            _ => return None,
        };
        Some(JAPAN_START + island)
    }

    pub fn region(self) -> Option<Region> {
        self.requires_chapter().map(Region::of)
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Wardrobe {
    owned: u64,
    garment: Option<Cosmetic>,
    head: Option<Cosmetic>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    accessory: Option<Cosmetic>,
}

impl Wardrobe {
    pub fn is_empty(&self) -> bool {
        self.owned == 0
    }

    pub fn owns(&self, item: Cosmetic) -> bool {
        self.owned & (1u64 << item as u32) != 0
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
        self.owned |= 1u64 << item as u32;
        let slot = match item.category() {
            Category::Garment => &mut self.garment,
            Category::Head => &mut self.head,
            Category::Accessory => &mut self.accessory,
        };
        *slot = (*slot != Some(item)).then_some(item);
        true
    }
}

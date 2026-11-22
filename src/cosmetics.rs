use serde::{Deserialize, Serialize};

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
}

impl Cosmetic {
    pub const ALL: [Self; 24] = [
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
        ][self as usize]
    }

    pub fn cost(self) -> u16 {
        [
            3, 6, 16, 4, 12, 9, 4, 5, 5, 6, 7, 4, 5, 8, 6, 3, 4, 3, 6, 4, 4, 5, 5, 6,
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
            | Self::Mosaic => Category::Garment,
            Self::Straw
            | Self::Laurel
            | Self::Petasos
            | Self::Sunbonnet
            | Self::Indigo
            | Self::Diadem
            | Self::Wreath
            | Self::Headband => Category::Head,
            _ => Category::Accessory,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Wardrobe {
    owned: u32,
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
        self.owned & (1 << item as u32) != 0
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
        self.owned |= 1 << item as u32;
        let slot = match item.category() {
            Category::Garment => &mut self.garment,
            Category::Head => &mut self.head,
            Category::Accessory => &mut self.accessory,
        };
        *slot = (*slot != Some(item)).then_some(item);
        true
    }
}

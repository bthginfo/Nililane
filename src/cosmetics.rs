use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cosmetic {
    Sea,
    Saffron,
    Toga,
    Straw,
    Laurel,
    Petasos,
}

impl Cosmetic {
    pub const ALL: [Self; 6] = [
        Self::Sea,
        Self::Saffron,
        Self::Toga,
        Self::Straw,
        Self::Laurel,
        Self::Petasos,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Sea => "Meerblau",
            Self::Saffron => "Safrangewand",
            Self::Toga => "Purpurtoga",
            Self::Straw => "Fischerhut",
            Self::Laurel => "Lorbeerkranz",
            Self::Petasos => "Roter Reisehut",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Sea => "wear_sea",
            Self::Saffron => "wear_saffron",
            Self::Toga => "wear_toga",
            Self::Straw => "wear_straw",
            Self::Laurel => "wear_laurel",
            Self::Petasos => "wear_petasos",
        }
    }

    pub fn cost(self) -> u16 {
        [3, 6, 16, 4, 12, 9][self as usize]
    }

    pub fn headwear(self) -> bool {
        matches!(self, Self::Straw | Self::Laurel | Self::Petasos)
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Wardrobe {
    owned: u8,
    garment: Option<Cosmetic>,
    head: Option<Cosmetic>,
}

impl Wardrobe {
    pub fn is_empty(&self) -> bool {
        self.owned == 0
    }

    pub fn owns(&self, item: Cosmetic) -> bool {
        self.owned & (1 << item as u8) != 0
    }

    pub fn wearing(&self, item: Cosmetic) -> bool {
        (if item.headwear() {
            self.head
        } else {
            self.garment
        }) == Some(item)
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
        self.owned |= 1 << item as u8;
        let slot = if item.headwear() {
            &mut self.head
        } else {
            &mut self.garment
        };
        *slot = (*slot != Some(item)).then_some(item);
        true
    }
}

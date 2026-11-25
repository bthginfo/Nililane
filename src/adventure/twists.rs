use crate::campaign::Region;

use super::{Good, Level, terrain::unit};

/// A small, friendly peculiarity of one island. The first island of every region has
/// none; neighbouring islands never share one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Twist {
    RichWoods,
    OreVein,
    FairWind,
    FestiveMood,
    BusyHands,
    FertileSoil,
}

/// Extra trees with [`Twist::RichWoods`], in percent.
pub const RICH_WOODS: u16 = 30;
/// Extra units per field node with [`Twist::FertileSoil`], in percent.
pub const FERTILE_SOIL: u16 = 40;
/// Extra units in the single rich node of [`Twist::OreVein`].
pub const ORE_VEIN: u16 = 48;
/// Trade ships sail this much faster with [`Twist::FairWind`].
pub const FAIR_WIND: f32 = 0.6;
/// Workshops finish this much sooner with [`Twist::BusyHands`].
pub const BUSY_HANDS: f32 = 1.15;

impl Twist {
    pub const ALL: [Self; 6] = [
        Self::RichWoods,
        Self::OreVein,
        Self::FairWind,
        Self::FestiveMood,
        Self::BusyHands,
        Self::FertileSoil,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::RichWoods => "Reiche Wälder",
            Self::OreVein => "Erzader",
            Self::FairWind => "Günstiger Wind",
            Self::FestiveMood => "Festlaune",
            Self::BusyHands => "Fleißige Hände",
            Self::FertileSoil => "Fruchtbarer Boden",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::RichWoods => "adventure_wood",
            Self::OreVein => "adventure_ore",
            Self::FairWind => "wind",
            Self::FestiveMood => "star",
            Self::BusyHands => "adventure_tools",
            Self::FertileSoil => "adventure_food",
        }
    }

    /// One short line that tells the player what is different here.
    pub fn line(self, region: Region) -> String {
        match self {
            Self::RichWoods => format!("Hier wachsen {RICH_WOODS} % mehr Bäume."),
            Self::OreVein => format!(
                "Eine reiche Ader liefert viel {} an einer Stelle.",
                Good::Ore.name(region)
            ),
            Self::FairWind => "Handelsschiffe sind hier schneller zurück.".to_owned(),
            Self::FestiveMood => "Feste bringen dir hier mehr Lohn.".to_owned(),
            Self::BusyHands => "Werkstätten arbeiten hier 15 % schneller.".to_owned(),
            Self::FertileSoil => format!(
                "Sammelstellen geben hier {FERTILE_SOIL} % mehr {}.",
                Good::Food.name(region)
            ),
        }
    }

    fn fits(self, level: Level) -> bool {
        match self {
            Self::FairWind => level.has_trading(),
            Self::FestiveMood => level.encounter_count() >= 2,
            Self::OreVein => level.stage >= 3,
            Self::RichWoods | Self::BusyHands | Self::FertileSoil => true,
        }
    }
}

impl Level {
    /// The island's twist, chosen once from its index.
    pub fn twist(self) -> Option<Twist> {
        if self.stage == 0 {
            return None;
        }
        let previous = Level::at(self.index - 1).twist();
        let mut options = Twist::ALL
            .into_iter()
            .filter(move |&twist| twist.fits(self) && Some(twist) != previous);
        let count = options.clone().count();
        let pick = (unit(self.index as u32 * 7_919 + 1_301) * count as f32) as usize;
        options.nth(pick.min(count - 1))
    }

    pub fn has_twist(self, twist: Twist) -> bool {
        self.twist() == Some(twist)
    }
}

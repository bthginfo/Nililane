use crate::{
    campaign::{Chapter, harbor_stage},
    model::{Phase, Town, Weather},
};

pub const SPOTS: usize = 3;
pub const LITTER_STAGE: usize = 1;
pub const PET_STAGE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    Leaves,
    Scraps,
    Cat,
    Dog,
}

impl Activity {
    pub fn animal(self) -> bool {
        matches!(self, Self::Cat | Self::Dog)
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Leaves => "leaves",
            Self::Scraps => "scraps",
            Self::Cat => "cat",
            Self::Dog => "dog",
        }
    }

    pub fn action(self) -> &'static str {
        match self {
            Self::Leaves => "Laub aufheben",
            Self::Scraps => "Unrat aufheben",
            Self::Cat => "Katze streicheln",
            Self::Dog => "Hund streicheln",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Leaves => "Laub",
            Self::Scraps => "Unrat",
            Self::Cat => "Katze",
            Self::Dog => "Hund",
        }
    }

    pub fn reward(self) -> u32 {
        if self.animal() { 2 } else { 1 }
    }
}

#[derive(Clone, Copy)]
pub struct Spot {
    pub activity: Activity,
    pub position: [f32; 2],
    pub yaw: f32,
    pub walking: bool,
    pub claimed: bool,
    pub remaining: f32,
}

impl Spot {
    pub fn distance(self, [x, z]: [f32; 2]) -> f32 {
        (self.position[0] - x).powi(2) + (self.position[1] - z).powi(2)
    }

    pub fn in_range(self, from: [f32; 2]) -> bool {
        !self.claimed && self.distance(from) <= 0.84_f32.powi(2)
    }
}

pub fn spots(town: &Town, chapter: &Chapter) -> [Option<Spot>; SPOTS] {
    let Phase::Open(day) = &town.phase else {
        return [None; SPOTS];
    };
    let stage = harbor_stage(town.chapter);
    if stage < LITTER_STAGE {
        return [None; SPOTS];
    }
    let length = chapter.day_seconds();
    let weather = town.weather_schedule(chapter)[1];
    let breezy = matches!(weather, Weather::Wind | Weather::Rain | Weather::Storm);
    let seed = (town.chapter as u32)
        .wrapping_mul(13)
        .wrapping_add(town.day);
    let span = (chapter.layout.width() as f32 - 1.0) * crate::layout::LOT_SPACING * 0.5;
    std::array::from_fn(|slot| {
        let (activity, start, end) = match slot {
            0 => (
                if breezy {
                    Activity::Leaves
                } else {
                    Activity::Scraps
                },
                0.12,
                1.0,
            ),
            1 if stage >= PET_STAGE && !seed.is_multiple_of(5) => (
                if seed % 4 < 2 {
                    Activity::Cat
                } else {
                    Activity::Dog
                },
                0.30 + (seed % 3) as f32 * 0.052,
                0.66 + (seed % 3) as f32 * 0.052,
            ),
            2 if !seed.is_multiple_of(4) => (
                if breezy {
                    Activity::Scraps
                } else {
                    Activity::Leaves
                },
                0.59,
                1.0,
            ),
            _ => return None,
        };
        let fraction = day.seconds / length;
        let claimed = day.street_claims & (1 << slot) != 0;
        if !(start..end).contains(&fraction) || claimed && !activity.animal() {
            return None;
        }
        let lane = [-0.82, 0.16, 0.91][slot];
        let sign = if seed.wrapping_add(slot as u32).is_multiple_of(2) {
            -1.0
        } else {
            1.0
        };
        let origin = match slot {
            0 => span * 0.22 * sign,
            1 => span * 0.42 * sign,
            _ => span * 0.59 * sign,
        };
        let age = (fraction - start) / (end - start);
        let arrival = ((age / 0.24).clamp(0.0, 1.0) - 1.0) * 1.15;
        let departure = ((age - 0.78) / 0.22).clamp(0.0, 1.0) * 1.75;
        let x = if activity.animal() {
            origin + sign * (arrival + departure)
        } else {
            origin
        };
        Some(Spot {
            activity,
            position: [x.clamp(-span - 0.4, span + 0.4), lane],
            yaw: if sign > 0.0 {
                std::f32::consts::FRAC_PI_2
            } else {
                -std::f32::consts::FRAC_PI_2
            },
            walking: activity.animal() && (age < 0.24 || age > 0.78),
            claimed,
            remaining: (end - fraction) * length,
        })
    })
}

pub fn help(town: &mut Town, chapter: &Chapter, index: usize) -> Option<Activity> {
    let spot = spots(town, chapter).get(index).copied().flatten()?;
    let Phase::Open(day) = &mut town.phase else {
        return None;
    };
    // The same proximity gate applies to the controller and to touch markers.
    if day.route.is_some() || !spot.in_range(day.nili?.position) {
        return None;
    }
    day.street_claims |= 1 << index;
    day.tally.street_bonus += spot.activity.reward();
    Some(spot.activity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        campaign::CHAPTERS,
        errands::{ManualStage, NiliControl},
    };

    #[test]
    fn petting_requires_proximity_and_pays_once_even_if_the_pet_stays() {
        let chapter = &CHAPTERS[3];
        let mut town = Town::new(3, chapter);
        town.day = 2;
        assert!(town.open(chapter));
        assert!(!town.tick(chapter, chapter.day_seconds() * 0.48));
        let pet = spots(&town, chapter)[1].unwrap();
        assert!(pet.activity.animal());
        assert!(help(&mut town, chapter, 1).is_none());
        let Phase::Open(day) = &mut town.phase else {
            unreachable!()
        };
        day.nili = Some(NiliControl {
            position: pet.position,
            yaw: 0.0,
            stage: ManualStage::Pickup,
        });
        assert_eq!(help(&mut town, chapter, 1), Some(pet.activity));
        assert!(spots(&town, chapter)[1].unwrap().claimed);
        assert!(help(&mut town, chapter, 1).is_none());
        assert!(!town.tick(chapter, chapter.day_seconds() * 0.35));
        assert!(spots(&town, chapter)[1].is_none());
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert_eq!(town.last_day.unwrap().street_bonus, 2);
    }
}

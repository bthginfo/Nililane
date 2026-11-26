use serde::{Deserialize, Serialize};

use crate::{
    campaign::{Chapter, Goal, Region, harbor_stage, richer, tutorial},
    model::{Age, Assortment, Building, Kind},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct TradeOrder {
    pub shop: Kind,
    pub basic: u32,
    pub special: Assortment,
    pub special_sales: u32,
    pub favorite_only: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct RebuildOrder {
    pub lot: usize,
    pub old: Kind,
    pub new: Kind,
    pub tier: u8,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub(crate) struct SalesProgress {
    pub basic: u32,
    pub special: u32,
}

impl SalesProgress {
    pub fn sale(&mut self, order: TradeOrder, building: Building, age: Age) {
        if building.kind != order.shop {
            return;
        }
        if self.basic < order.basic {
            if building.assortment == Assortment::Standard {
                self.basic += 1;
            }
        } else if building.assortment == order.special
            && building.level >= order.special.tier()
            && (!order.favorite_only || order.special.flavor() == Some(age))
            && self.special < order.special_sales
        {
            self.special += 1;
        }
    }
}

fn candidate(chapter: &Chapter, stage: usize) -> Option<Kind> {
    use Kind::*;
    let shops: &[Kind] = match chapter.region {
        Region::Greece => &[Bakery, Potter, Taverna, Apiary, Dyer],
        Region::Rome => &[Taverna, Bakery, Potter, Scribe, CeramicStudio, SilkShop],
        Region::Japan => &[RiceKitchen, TeaHouse, PaperWorkshop],
        Region::Nordic => &[Smokehouse, FeastHall, Weaver, Cooperage],
        Region::Oasis => &[SpiceBazaar, Perfumer, CarpetHall, Glassworks],
    };
    let new_workshop = match (chapter.region, stage) {
        (Region::Greece, 5) => Some(Taverna),
        (Region::Greece, 8) => Some(Dyer),
        (Region::Japan, 5) => Some(CeramicStudio),
        (Region::Nordic, 6) => Some(Cooperage),
        (Region::Oasis, 3) => Some(Glassworks),
        (Region::Oasis, 7) => Some(Perfumer),
        _ => None,
    };
    new_workshop
        .into_iter()
        .chain((0..shops.len()).map(|offset| shops[(stage / 2 + offset) % shops.len()]))
        .find(|&shop| {
            Assortment::supported(shop)
                && (chapter.allowed().contains(&shop)
                    || chapter.starts.iter().any(|&(_, kind)| kind == shop))
                && shop.service().is_some_and(|service| {
                    chapter.locals.contains(&service) || chapter.guests.contains(&service)
                })
        })
}

/// Harbors at these stages hand out a trade order when one of their shops fits.
pub(crate) const fn trade_stage(stage: usize) -> bool {
    matches!(stage, 3 | 5 | 7 | 9 | 11 | 14 | 16 | 18 | 20 | 22 | 23)
}

pub(crate) fn trade_order(chapter: &Chapter, index: usize) -> Option<TradeOrder> {
    let stage = harbor_stage(index);
    if !trade_stage(stage) {
        return None;
    }
    let shop = candidate(chapter, stage)?;
    let special = match stage {
        0..=5 => Assortment::Festival,
        6..=14 => Assortment::Market,
        16 | 18 | 20 | 23 => Assortment::Signature,
        _ => Assortment::HouseSpecial,
    };
    Some(TradeOrder {
        shop,
        basic: 5 + stage as u32 / 3,
        special,
        special_sales: if stage < 14 {
            4 + stage as u32 / 4
        } else {
            3 + stage as u32 / 7
        },
        favorite_only: stage >= 14,
    })
}

fn rebuild(chapter: &Chapter, stage: usize) -> Option<Goal> {
    use Kind::*;
    let (old, new, tier) = match (chapter.region, stage) {
        (Region::Greece, 19) => (Bakery, Apiary, 2),
        (Region::Rome, 15) => (Well, Aqueduct, 2),
        (Region::Japan, 19) => (Inn, Temple, 2),
        (Region::Nordic, 19) => (Woodlot, Cooperage, 2),
        (Region::Oasis, 19) => (Caravanserai, CarpetHall, 3),
        _ => return None,
    };
    let &(lot, _) = chapter.starts.iter().find(|&&(_, kind)| kind == old)?;
    Some(Goal::Rebuild(RebuildOrder {
        lot,
        old,
        new,
        tier,
    }))
}

pub(crate) fn care_goal(chapter: &Chapter, index: usize) -> Option<Goal> {
    let stage = harbor_stage(index);
    let (age, minimum, percent) = match stage {
        6 => (Age::Adult, 7, 72),
        11 => (Age::Child, 7, 77),
        12 => (Age::Senior, 8, 77),
        15 | 19 => (Age::Adult, 11, 81),
        _ => return None,
    };
    let min = if chapter.region == Region::Nordic && stage == 6 {
        6
    } else {
        minimum
    };
    Some(Goal::GuestCare(age, min, percent))
}

pub(crate) fn extra_goals(chapter: &Chapter, index: usize) -> [Option<Goal>; 4] {
    let stage = harbor_stage(index);
    if stage < 2 {
        return [None; 4];
    }
    let shop = candidate(chapter, stage);
    let service = shop
        .and_then(Kind::service)
        .or_else(|| chapter.locals.first().copied());
    let production = service.map(|service| Goal::Produce(service, 9 + stage as u32 / 2));
    let care = care_goal(chapter, index);
    // The first Greek ports keep their simple extras; later harbors ask for a plan instead
    // of a production count, and keep the list short.
    let simple = tutorial(index);
    let fresh = || richer(chapter, index, 2);
    let planned = chapter
        .goals
        .iter()
        .filter(|goal| !goal.plain_counter() && !goal.street_task())
        .count();
    let side = match stage {
        4 | 8 | 13 | 17 | 21 | 23 => Some(Goal::Dogs(1)),
        6 | 11 | 12 | 15 | 19 => care,
        10 => shop
            .filter(|&shop| {
                !chapter
                    .goals
                    .iter()
                    .any(|goal| matches!(goal, Goal::Upgrade(kind, _) if *kind == shop))
            })
            .map(|shop| Goal::Upgrade(shop, 2)),
        9 | 14 | 22 => production,
        18 => Some(Goal::FavoriteMenu(Age::Adult, 2)),
        _ => Some(Goal::Clean(if stage >= 11 { 2 } else { 1 })),
    };
    if let Some(order) = trade_order(chapter, index) {
        let bonus = if matches!(stage, 7 | 11 | 14 | 18 | 22 | 23) {
            side
        } else {
            None
        };
        // The three trade steps count as one goal with a plan.
        let bonus = match bonus {
            Some(Goal::Produce(..)) if !simple => Some(fresh()),
            bonus => bonus,
        }
        .filter(|goal| simple || goal.street_task() || planned + 2 <= MOST_PLANNED);
        return [
            Some(Goal::Sale(order, false)),
            Some(Goal::TradeUpgrade(order)),
            Some(Goal::Sale(order, true)),
            bonus,
        ];
    }
    let secondary = if matches!(side, Some(Goal::Produce(..))) {
        Some(Goal::Clean(1))
    } else {
        side.or(Some(Goal::Clean(1)))
    };
    let spatial = match (chapter.region, stage) {
        (Region::Rome, 6) => Some(Goal::Pair(Kind::GardenCourt, Kind::Aqueduct, 1)),
        (Region::Nordic, 10) => Some(Goal::Pair(Kind::Cooperage, Kind::Boatyard, 1)),
        _ => rebuild(chapter, stage),
    };
    if simple {
        return [production, secondary, spatial, None];
    }
    let first = spatial.unwrap_or_else(fresh);
    // A street errand keeps the list short when the harbor already asks for enough.
    let secondary = secondary.map(|goal| {
        if goal.street_task() || planned + 2 <= MOST_PLANNED {
            goal
        } else {
            Goal::Clean(if stage >= 11 { 2 } else { 1 })
        }
    });
    [Some(first), secondary, None, None]
}

/// Later harbors ask for at most this many goals beyond one plain counter and street errands.
pub(crate) const MOST_PLANNED: usize = 4;

pub(crate) fn quest_dog(index: usize) -> bool {
    matches!(harbor_stage(index), 4 | 8 | 13 | 17 | 21 | 23)
}

#[cfg(test)]
pub(crate) fn shop_for(service: crate::model::Service, chapter: &Chapter) -> Option<Kind> {
    chapter
        .allowed()
        .iter()
        .copied()
        .find(|kind| kind.service() == Some(service))
        .or_else(|| {
            chapter
                .starts
                .iter()
                .find_map(|&(_, kind)| (kind.service() == Some(service)).then_some(kind))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::{CHAPTERS, REGION_SIZE};

    #[test]
    fn trade_counts_only_the_requested_products_in_order() {
        let mut progress = SalesProgress::default();
        let order = TradeOrder {
            shop: Kind::Bakery,
            basic: 2,
            special: Assortment::Festival,
            special_sales: 1,
            favorite_only: true,
        };
        let basic = Building::new(order.shop);
        let special = Building {
            assortment: order.special,
            level: order.special.tier(),
            ..basic
        };
        let age = Age::Child;
        progress.sale(order, special, age);
        progress.sale(order, Building::new(Kind::Potter), age);
        assert_eq!((progress.basic, progress.special), (0, 0));
        progress.sale(order, basic, age);
        progress.sale(order, special, age);
        assert_eq!((progress.basic, progress.special), (1, 0));
        progress.sale(order, basic, age);
        progress.sale(
            order,
            Building {
                level: 1,
                ..special
            },
            age,
        );
        progress.sale(
            order,
            Building {
                assortment: Assortment::Colorful,
                ..special
            },
            age,
        );
        assert_eq!((progress.basic, progress.special), (2, 0));
        progress.sale(order, special, Age::Adult);
        assert_eq!(progress.special, 0);
        progress.sale(order, special, age);
        progress.sale(order, special, age);
        assert_eq!((progress.basic, progress.special), (2, 1));
    }

    #[test]
    fn each_region_has_varied_objectives_and_reachable_trade_offers() {
        for (region, chapters) in CHAPTERS.chunks(REGION_SIZE).enumerate() {
            let mut lengths = std::collections::BTreeSet::new();
            let mut shops = Vec::new();
            let mut rebuilds = 0;
            for (stage, chapter) in chapters.iter().enumerate() {
                let index = region * REGION_SIZE + stage;
                let goals: Vec<_> = chapter.objectives(index).collect();
                lengths.insert(goals.len());
                assert!(
                    (if stage < 2 { 3..=3 } else { 5..=7 }).contains(&goals.len()),
                    "{}",
                    chapter.district
                );
                if let Some(order) = trade_order(chapter, index) {
                    assert!(order.special.flavor().unwrap().available(index));
                    if !shops.contains(&order.shop) {
                        shops.push(order.shop);
                    }
                }
                for goal in goals {
                    if let Goal::Rebuild(order) = goal {
                        assert!(
                            chapter.layout.contains(order.lot)
                                && chapter.allowed().contains(&order.new)
                        );
                        assert!(chapter.starts.contains(&(order.lot, order.old)));
                        rebuilds += 1;
                    }
                }
            }
            assert_eq!(lengths.into_iter().collect::<Vec<_>>(), [3, 5, 6, 7]);
            assert!(shops.len() >= 2, "{:?}: {shops:?}", chapters[0].region);
            assert_eq!(rebuilds, 1);
        }
    }

    #[test]
    fn later_harbors_mix_at_most_one_plain_counter_with_planned_goals() {
        let trade = |goal: &Goal| matches!(goal, Goal::Sale(..) | Goal::TradeUpgrade(..));
        let introduced = |goal: &Goal| {
            matches!(
                goal,
                Goal::Streak(_)
                    | Goal::TourDay(_)
                    | Goal::Rush(_)
                    | Goal::FanDay(..)
                    | Goal::SteadyMood(_)
                    | Goal::Call(..)
            )
        };
        let mut kinds = std::collections::HashMap::new();
        for (index, chapter) in CHAPTERS.iter().enumerate() {
            let goals: Vec<_> = chapter.objectives(index).collect();
            if tutorial(index) {
                assert!(
                    goals.iter().all(|goal| !introduced(goal)),
                    "{} stays simple",
                    chapter.district
                );
                continue;
            }
            let counters = goals
                .iter()
                .filter(|goal| !trade(goal) && goal.plain_counter())
                .count();
            // The three trade steps form one order with a plan.
            let planned = goals
                .iter()
                .filter(|goal| !trade(goal) && !goal.plain_counter() && !goal.street_task())
                .count()
                + usize::from(goals.iter().any(trade));
            assert!(counters <= 1, "{}: {counters} counters", chapter.district);
            assert!(
                (2..=MOST_PLANNED).contains(&planned),
                "{}: {planned} goals with a plan",
                chapter.district
            );
            let labels: Vec<_> = goals.iter().map(|goal| goal.label()).collect();
            for (i, label) in labels.iter().enumerate() {
                assert!(
                    !labels[..i].contains(label),
                    "{} asks twice: {label}",
                    chapter.district
                );
            }
            for goal in goals.iter().filter(|goal| introduced(goal)) {
                *kinds.entry(std::mem::discriminant(goal)).or_insert(0u32) += 1;
            }
        }
        // Every new kind turns up often enough to become familiar.
        assert_eq!(kinds.len(), 6);
        assert!(kinds.values().all(|&count| count >= 5), "{kinds:?}");
        assert!(kinds.values().sum::<u32>() >= 100);
    }
}

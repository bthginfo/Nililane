use crate::{
    campaign::Chapter,
    errands::Job,
    model::{Favor, Kind, Phase, Price, Town, distance},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Build(usize, Kind),
    Price(usize),
    StartDay,
    Arrivals,
    Aid(usize, Favor),
    Water(usize),
    Evening,
}

impl Step {
    pub fn lot(self) -> Option<usize> {
        match self {
            Self::Build(lot, _) | Self::Price(lot) | Self::Aid(lot, _) | Self::Water(lot) => {
                Some(lot)
            }
            _ => None,
        }
    }
}

pub fn recommended_price(town: &Town, lot: usize) -> Option<Price> {
    let building = town.lots.get(lot)?.as_ref()?;
    (town.chapter == 0
        && town.day == 1
        && building.kind == Kind::Bakery
        && building.price == Price::Regular)
        .then_some(Price::Fair)
}

pub fn next(town: &Town, chapter: &Chapter, selected: usize, saw_prices: bool) -> Option<Step> {
    let naxos = town.chapter == 0 && town.day <= 2;
    let amorgos = town.chapter == 1 && town.day == 1;

    match &town.phase {
        Phase::Plan if naxos || amorgos => {
            let bakery = town
                .lots
                .iter()
                .position(|b| b.is_some_and(|b| b.kind == Kind::Bakery));
            if naxos && bakery.is_none() {
                let home = town
                    .lots
                    .iter()
                    .position(|b| b.is_some_and(|b| b.kind == Kind::House));
                return build_near(town, chapter, Kind::Bakery, home.unwrap_or(selected), 2);
            }
            if (amorgos || town.day == 2)
                && town.count(Kind::Well) == 0
                && let Some(step) =
                    build_near(town, chapter, Kind::Well, bakery.unwrap_or(selected), 8)
            {
                return Some(step);
            }
            if !saw_prices
                && let Some(lot) = bakery
                && recommended_price(town, lot).is_some()
            {
                return Some(Step::Price(lot));
            }
            Some(Step::StartDay)
        }
        Phase::Open(day) => {
            if day.route.is_some() {
                return None;
            }
            if amorgos
                && day.tally.routes[Job::Water as usize] == 0
                && let Some(lot) = chapter.layout.lots().find(|&lot| {
                    town.lots[lot].is_some_and(|b| b.kind == Kind::Bakery)
                        && Job::Water.plan(town, chapter, lot).is_ok()
                })
            {
                return Some(Step::Water(lot));
            }
            if town.can_aid(selected) {
                return Some(Step::Aid(selected, day.favor));
            }
            if let Some(lot) = chapter
                .layout
                .lots()
                .filter(|&lot| town.can_aid(lot))
                .min_by_key(|&lot| {
                    let stock = if day.favor == Favor::Cargo {
                        day.stock[lot]
                    } else {
                        0
                    };
                    (stock, distance(selected, lot))
                })
            {
                return Some(Step::Aid(lot, day.favor));
            }
            if naxos {
                return Some(if day.seconds < 8.0 {
                    Step::Arrivals
                } else {
                    Step::Evening
                });
            }
            None
        }
        _ => None,
    }
}

fn build_near(
    town: &Town,
    chapter: &Chapter,
    kind: Kind,
    center: usize,
    preferred: usize,
) -> Option<Step> {
    if town.coins < kind.cost() {
        return None;
    }
    chapter
        .layout
        .lots()
        .filter(|&lot| town.lots[lot].is_none())
        .min_by_key(|&lot| (distance(center, lot), lot != preferred))
        .map(|lot| Step::Build(lot, kind))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::CHAPTERS;

    #[test]
    fn the_first_day_points_to_real_actions_without_locking_other_choices() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        assert_eq!(
            next(&town, chapter, 1, false),
            Some(Step::Build(2, Kind::Bakery))
        );
        assert!(town.build(chapter, 2, Kind::Bakery));
        assert_eq!(next(&town, chapter, 1, false), Some(Step::Price(2)));
        assert_eq!(next(&town, chapter, 1, true), Some(Step::StartDay));
        assert!(town.open(chapter));
        assert_eq!(next(&town, chapter, 1, true), Some(Step::Arrivals));
        town.tick(chapter, 8.1);
        assert_eq!(
            next(&town, chapter, 1, true),
            Some(Step::Aid(2, Favor::Cargo))
        );
        assert!(town.aid(2));
        assert_eq!(next(&town, chapter, 1, true), Some(Step::Evening));
    }

    #[test]
    fn amorgos_marks_a_free_plot_next_to_the_oven_and_then_the_water_delivery() {
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert_eq!(
            next(&town, chapter, 1, false),
            Some(Step::Build(8, Kind::Well))
        );
        assert!(town.build(chapter, 8, Kind::Well));
        assert!(town.open(chapter));
        assert_eq!(next(&town, chapter, 1, false), Some(Step::Water(7)));
    }
}

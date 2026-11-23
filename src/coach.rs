use crate::{
    campaign::Chapter,
    errands::{Job, Route, Stop},
    model::{Favor, Kind, Phase, Price, Town, distance},
};

pub const ISLANDS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Build(usize, Kind),
    Price(usize),
    Upgrade(usize),
    StartDay,
    Arrivals,
    Aid(usize, Favor),
    Tour(usize, Job),
    Evening,
}

impl Step {
    pub fn lot(self) -> Option<usize> {
        match self {
            Self::Build(lot, _)
            | Self::Price(lot)
            | Self::Upgrade(lot)
            | Self::Aid(lot, _)
            | Self::Tour(lot, _) => Some(lot),
            _ => None,
        }
    }

    pub fn tour(self) -> Option<Job> {
        match self {
            Self::Tour(_, job) => Some(job),
            _ => None,
        }
    }
}

// Each town saves which moments the player has already seen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moment {
    Build,
    BuildAgain,
    Price,
    Start,
    Ferry,
    Favor,
    Guests,
    Tour,
    TourAgain,
    Delivery,
    Upgrade,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prompt {
    Action(Step),
    Feedback(Option<usize>),
    Delivered(Job, Stop),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub moment: Moment,
    pub prompt: Prompt,
}

impl Moment {
    fn seen(self, town: &Town) -> bool {
        town.lessons & self.bit() != 0
    }

    pub fn bit(self) -> u32 {
        1 << self as u32
    }
}

pub fn recommended_price(town: &Town, lot: usize) -> Option<Price> {
    let b = town.lots.get(lot)?.as_ref()?;
    let relevant = matches!(
        (town.chapter, b.kind),
        (0, Kind::Bakery) | (2, Kind::Potter)
    );
    (relevant && town.day == 1 && b.price == Price::Regular).then_some(Price::Fair)
}

pub fn next(town: &Town, chapter: &Chapter, selected: usize, saw_prices: bool) -> Option<Step> {
    match &town.phase {
        Phase::Plan if town.chapter < ISLANDS && town.day <= 2 => {
            let bakery = find(town, Kind::Bakery);
            let potter = find(town, Kind::Potter);
            let build = match town.chapter {
                0 if bakery.is_none() => {
                    (Kind::Bakery, find(town, Kind::House).unwrap_or(selected), 2)
                }
                _ if matches!((town.chapter, town.day), (0, 2) | (1, _))
                    && town.count(Kind::Well) == 0 =>
                {
                    (Kind::Well, bakery.unwrap_or(selected), 8)
                }
                2 if potter.is_none() => {
                    (Kind::Potter, chapter.layout.dock(), chapter.layout.dock())
                }
                _ => {
                    if town.chapter == 2
                        && town.day >= 2
                        && let Some(lot) = potter
                        && let Some(step) = delos_upgrade(town, chapter, lot)
                    {
                        return Some(step);
                    }
                    if !saw_prices
                        && let Some(lot) = if town.chapter == 2 { potter } else { bakery }
                        && recommended_price(town, lot).is_some()
                    {
                        return Some(Step::Price(lot));
                    }
                    return Some(Step::StartDay);
                }
            };
            build_near(town, chapter, build.0, build.1, build.2).or(Some(Step::StartDay))
        }
        Phase::Open(day) if day.route.is_none() => {
            if let Some(step) = suggested_tour(town, chapter) {
                return Some(step);
            }
            if let Some(lot) = harbor_favor(town, chapter, selected) {
                return Some(Step::Aid(lot, day.favor));
            }
            (town.chapter == 0 && town.day <= 2).then_some(if day.seconds < 8.0 {
                Step::Arrivals
            } else {
                Step::Evening
            })
        }
        _ => None,
    }
}

pub(crate) fn harbor_favor(town: &Town, chapter: &Chapter, selected: usize) -> Option<usize> {
    let Phase::Open(day) = &town.phase else {
        return None;
    };
    if town.favor_plan(chapter, selected).is_some() {
        return Some(selected);
    }
    chapter
        .layout
        .lots()
        .filter(|&lot| town.favor_plan(chapter, lot).is_some())
        .min_by_key(|&lot| {
            let stock = if day.favor == Favor::Cargo {
                day.stock[lot]
            } else {
                0
            };
            (stock, distance(selected, lot))
        })
}

fn delos_upgrade(town: &Town, chapter: &Chapter, lot: usize) -> Option<Step> {
    let building = town.lots[lot]?;
    if building.level != 1 || town.coins < building.upgrade_cost() {
        return None;
    }
    let before = town.production(chapter);
    let (_, after) = town.projected_upgrade(chapter, lot)?;
    let loses_stock = after
        .stock
        .iter()
        .zip(before.stock)
        .any(|(&after, before)| after < before);
    if loses_stock || after.stock[lot] == before.stock[lot] {
        build_near(town, chapter, Kind::Well, lot, lot)
    } else {
        Some(Step::Upgrade(lot))
    }
}

fn suggested_tour(town: &Town, chapter: &Chapter) -> Option<Step> {
    let Phase::Open(day) = &town.phase else {
        return None;
    };
    let (job, building, earliest) = match town.chapter {
        1 => (Job::Water, Kind::Bakery, 2.5),
        2 => (Job::Freight, Kind::Potter, 11.5),
        _ => return None,
    };
    if town.day > 3 || day.seconds < earliest || town.totals.routes[job as usize] >= 2 {
        return None;
    }
    chapter
        .layout
        .lots()
        .find(|&lot| {
            town.lots[lot].is_some_and(|b| b.kind == building)
                && job.plan(town, chapter, lot).is_ok()
        })
        .map(|lot| Step::Tour(lot, job))
}

pub fn lesson(town: &Town, step: Option<Step>, delivered: Option<Route>) -> Option<Lesson> {
    use Moment::*;
    if town.chapter >= ISLANDS || town.day > 2 {
        return None;
    }
    let offer = |moment: Moment, prompt| (!moment.seen(town)).then_some(Lesson { moment, prompt });
    let action = |moment, step| offer(moment, Prompt::Action(step));
    let Phase::Open(day) = &town.phase else {
        return match step? {
            step @ Step::Build(..) => action(if town.day == 1 { Build } else { BuildAgain }, step),
            step @ Step::Price(..) => action(Price, step),
            step @ Step::Upgrade(..) => action(Upgrade, step),
            Step::StartDay if town.chapter == 0 && town.day == 1 => action(Start, Step::StartDay),
            _ => None,
        };
    };
    if day.route.is_some() {
        return None;
    }
    if let Some(route) = delivered.filter(|route| !route.favor)
        && matches!(
            (town.chapter, route.job),
            (1, Job::Water) | (2, Job::Freight)
        )
        && let Some(note) = offer(Delivery, Prompt::Delivered(route.job, route.delivery))
    {
        return Some(note);
    }
    if town.chapter == 0 {
        if town.day == 1
            && (8.0..=20.0).contains(&day.seconds)
            && let Some(note) = action(Ferry, Step::Arrivals)
        {
            return Some(note);
        }
        if day.seconds >= 9.0
            && let Some(step @ Step::Aid(..)) = step
            && let Some(note) = action(Favor, step)
        {
            return Some(note);
        }
        if day.seconds >= 13.0
            && let Some(visit) = day.visits.iter().rev().find(|v| v.resolved)
        {
            return offer(Guests, Prompt::Feedback(visit.shop.map(usize::from)));
        }
    }
    if let Some(step @ Step::Tour(..)) = step {
        return action(if town.day == 1 { Tour } else { TourAgain }, step);
    }
    None
}

fn find(town: &Town, kind: Kind) -> Option<usize> {
    town.lots
        .iter()
        .position(|b| b.is_some_and(|b| b.kind == kind))
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
    fn naxos_pauses_once_per_moment_and_uses_real_actions() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        let next = |town: &Town| super::next(town, chapter, 1, false);
        let build = Some(Step::Build(2, Kind::Bakery));
        assert_eq!(next(&town), build);
        assert_eq!(lesson(&town, build, None).unwrap().moment, Moment::Build);
        town.lessons |= Moment::Build.bit();
        assert_eq!(lesson(&town, build, None), None);
        assert_eq!(next(&town), build);
        assert!(town.build(chapter, 2, Kind::Bakery));
        assert_eq!(next(&town), Some(Step::Price(2)));
        assert!(town.open(chapter));
        town.tick(chapter, 8.1);
        assert_eq!(
            lesson(&town, next(&town), None).unwrap().moment,
            Moment::Ferry
        );
        town.lessons |= Moment::Ferry.bit();
        town.tick(chapter, 0.95);
        assert_eq!(
            lesson(&town, next(&town), None).unwrap().moment,
            Moment::Favor
        );
        assert!(town.aid(chapter, 2));
        assert_eq!(lesson(&town, next(&town), None), None);
        town.tick(chapter, 8.0);
        assert_eq!(
            lesson(&town, next(&town), None).unwrap().moment,
            Moment::Guests
        );
    }

    #[test]
    fn amorgos_marks_water_and_delos_marks_a_town_with_real_demand() {
        let chapter = &CHAPTERS[1];
        let mut town = Town::new(1, chapter);
        assert_eq!(
            next(&town, chapter, 1, false),
            Some(Step::Build(8, Kind::Well))
        );
        assert!(town.build(chapter, 8, Kind::Well));
        assert!(town.open(chapter));
        town.tick(chapter, 2.6);
        assert_eq!(
            next(&town, chapter, 1, false),
            Some(Step::Tour(7, Job::Water))
        );
        let route = town.dispatch(chapter, Job::Water, 7).unwrap();
        town.tick(chapter, route.finish - route.started + 0.01);
        assert_eq!(
            lesson(&town, next(&town, chapter, 7, false), Some(route))
                .unwrap()
                .moment,
            Moment::Delivery
        );

        let chapter = &CHAPTERS[2];
        let mut town = Town::new(2, chapter);
        let Some(Step::Build(lot, Kind::Potter)) = next(&town, chapter, 1, false) else {
            panic!("Töpferei")
        };
        assert!(distance(chapter.layout.dock(), lot) <= 1);
        assert!(town.build(chapter, lot, Kind::Potter));
        assert!(town.open(chapter));
        town.tick(chapter, 12.0);
        assert_eq!(
            next(&town, chapter, 1, false),
            Some(Step::Tour(lot, Job::Freight))
        );
        assert!(town.tick(chapter, chapter.day_seconds()));
        assert!(town.advance());
        town.coins = 60;
        let (_, dry_upgrade) = town.projected_upgrade(chapter, lot).unwrap();
        assert!(dry_upgrade.stock[lot] < town.production(chapter).stock[lot]);
        let Some(Step::Build(well, Kind::Well)) = next(&town, chapter, lot, true) else {
            panic!("Delos needs water before upgrading")
        };
        assert!(town.build(chapter, well, Kind::Well));
        assert_eq!(next(&town, chapter, lot, true), Some(Step::Upgrade(lot)));
    }
}

use std::collections::{HashSet, VecDeque};

use super::*;

struct Expedition {
    terrain: Terrain,
    island: Island,
    visits: usize,
    rounds: usize,
}

impl Expedition {
    fn start(index: usize) -> Self {
        let level = Level::at(index);
        Self {
            terrain: Terrain::new(level),
            island: Island::new(level),
            visits: 0,
            rounds: 0,
        }
    }

    fn walk_to(&mut self, position: Vec2, distance: f32) {
        let here = Vec2::from_array(self.island.position);
        let can_walk = |p| self.terrain.walkable(p, &self.island.lots);
        let direct = here.distance(position);
        if direct < distance {
            return;
        }
        let stride = 0.4;
        let travel = (direct - distance + stride).min(direct);
        let steps = (travel / stride).ceil() as u32;
        let direction = (position - here).normalize();
        if (1..=steps).all(|step| can_walk(here + direction * (step as f32 * stride).min(travel))) {
            self.island.position = (here + direction * travel).to_array();
            self.visits += 1;
            return;
        }
        let start = here.round().as_ivec2();
        let mut open = VecDeque::from([start]);
        let mut seen = HashSet::from([start]);
        while let Some(at) = open.pop_front() {
            let point = at.as_vec2();
            if point.distance(position) < distance && can_walk(point) {
                self.island.position = point.to_array();
                self.visits += 1;
                return;
            }
            for d in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
                let next = at + d;
                if can_walk(next.as_vec2())
                    && can_walk((at.as_vec2() + next.as_vec2()) * 0.5)
                    && seen.insert(next)
                {
                    open.push_back(next);
                }
            }
        }
        panic!(
            "{}: keine Route von {here:?} nach {position:?}",
            self.terrain.level.name
        );
    }

    fn supply(&mut self, good: Good, target: u16) {
        if self.island.amount(good) >= target {
            return;
        }
        if Good::RAW.contains(&good) {
            let (index, deposit) = self
                .terrain
                .deposits
                .iter()
                .copied()
                .enumerate()
                .find(|(_, deposit)| deposit.kind == good)
                .unwrap();
            self.walk_to(deposit.at, reach(Target::Deposit(index)));
            while self.island.amount(good) < target {
                self.island.gather(good);
            }
            return;
        }
        let level = self.terrain.level;
        let output = |build: Build| {
            build
                .recipe(1)
                .is_some_and(|recipe| recipe.output.0 == good)
        };
        let workshop = self
            .island
            .lots
            .iter()
            .position(|lot| lot.is_some_and(|property| output(property.kind)));
        let lot = workshop.unwrap_or_else(|| {
            let plan = Build::all(level)
                .into_iter()
                .filter(|&plan| output(plan) && !self.island.locked(level, plan, 1))
                .min_by_key(|plan| plan.cost(1).iter().map(|(_, amount)| amount).sum::<u16>())
                .unwrap_or_else(|| {
                    panic!("{}: keine verfügbare Produktion für {good:?}", level.name)
                });
            self.build(plan, None)
        });
        let task = Task::Produce(lot);
        let property = self.island.lots[lot].unwrap();
        let recipe = property.kind.recipe(property.tier).unwrap();
        assert!(
            recipe.need.iter().all(|&(input, _)| input != good),
            "{}: Produktionskreislauf für {good:?}",
            level.name
        );
        let rounds = (target - self.island.amount(good))
            .div_ceil(recipe.output.1)
            .max(1);
        let inputs: Vec<_> = recipe
            .need
            .into_iter()
            .map(|(input, amount)| (input, amount * rounds))
            .collect();
        for _ in 0..4 {
            if self.island.ready(&inputs) {
                break;
            }
            for &(input, amount) in inputs.iter().rev() {
                self.supply(input, amount);
            }
        }
        assert!(
            self.island.ready(&inputs),
            "{}: fehlende Vorlage für {good:?}",
            level.name
        );
        let assigned = self.island.assign(task, &self.terrain);
        for _ in 0..rounds {
            if assigned {
                self.island.tick_work(&self.terrain);
            } else {
                self.walk_to(self.terrain.plots[lot], reach(Target::Plot(lot)));
                assert!(
                    self.island.produce(lot),
                    "{}: Betrieb {property:?} steht still",
                    level.name
                );
            }
            self.rounds += 1;
        }
        if assigned {
            assert!(self.island.unassign(task));
        }
        assert!(
            self.island.amount(good) >= target,
            "{}: zu wenig {good:?}",
            level.name
        );
    }

    fn build(&mut self, kind: Build, previous: Option<usize>) -> usize {
        let level = self.terrain.level;
        let lot = previous
            .or_else(|| self.island.lots.iter().position(Option::is_none))
            .unwrap_or_else(|| panic!("{}: kein Bauplatz für {kind:?}", level.name));
        let tier = self.island.lots[lot].map_or(1, |p| p.tier + 1);
        assert!(
            !self.island.locked(level, kind, tier),
            "{}: {kind:?} {tier} gesperrt",
            level.name
        );
        let costs = kind.cost(tier);
        for _ in 0..4 {
            if self.island.ready(&costs) {
                break;
            }
            for &(good, count) in costs.iter().rev() {
                self.supply(good, count);
            }
        }
        // Supporting workshops may have occupied that vacant lot in the meantime.
        let lot = if previous.is_none() && self.island.lots[lot].is_some() {
            self.island.lots.iter().position(Option::is_none).unwrap()
        } else {
            lot
        };
        let at = self.terrain.plots[lot];
        self.walk_to(at, reach(Target::Plot(lot)));
        assert!(
            self.island.build(level, lot, kind),
            "{}: Bau abgelehnt für {kind:?} Stufe {tier}",
            level.name
        );
        lot
    }

    fn quest(&mut self) {
        let level = self.terrain.level;
        let goal = level.requirements()[2];
        assert!(self.island.locked(level, level.plan().unwrap(), goal.tier));
        assert_eq!(advance_quest(&mut self.island, level, Target::Relic), None);
        self.walk_to(self.terrain.shrine, reach(Target::Shrine));
        assert!(advance_quest(&mut self.island, level, Target::Shrine).is_some());
        if level.expedition() {
            assert_eq!(self.island.quest, Quest::Searching);
            self.walk_to(self.terrain.relic, reach(Target::Relic));
            assert!(advance_quest(&mut self.island, level, Target::Relic).is_some());
            assert_eq!(self.island.quest, Quest::Carrying);
            self.walk_to(self.terrain.shrine, reach(Target::Shrine));
            assert!(advance_quest(&mut self.island, level, Target::Shrine).is_some());
        }
        if let Some((good, count)) = level.offering() {
            assert_eq!(self.island.quest, Quest::Offering);
            assert!(advance_quest(&mut self.island, level, Target::Shrine).is_none());
            self.supply(good, count);
            self.walk_to(self.terrain.shrine, reach(Target::Shrine));
            let before = self.island.amount(good);
            assert!(advance_quest(&mut self.island, level, Target::Shrine).is_some());
            assert_eq!(self.island.amount(good), before - count);
        }
        assert_eq!(self.island.quest, Quest::Finished);
        assert!(!self.island.locked(level, level.plan().unwrap(), goal.tier));
    }

    fn requirements(&mut self, count: usize) {
        let level = self.terrain.level;
        for req in level.requirements()[..count].iter().copied() {
            while self.island.progress(req) < req.count {
                let slot = self
                    .island
                    .lots
                    .iter()
                    .position(|p| p.is_some_and(|p| p.kind == req.kind && p.tier < req.tier));
                self.build(req.kind, slot);
            }
        }
    }
}

#[test]
fn all_sixty_expeditions_have_walkable_goals_and_finish_with_real_production() {
    let mut total_visits = 0;
    let mut total_cycles = 0;
    for index in 0..LEVELS {
        let mut run = Expedition::start(index);
        let level = run.terrain.level;
        assert!(!run.island.victory(level));
        assert!(run.island.lots.iter().all(Option::is_none));
        run.requirements(2);
        if level.plan().is_some() {
            run.quest();
        }
        run.requirements(3);
        if let Some(goal) = level.support() {
            run.build(goal.kind, None);
        }
        let (good, count) = level.reserve();
        run.supply(good, count);
        assert!(
            run.island.victory(level),
            "{} ({index}): {:?}",
            level.name,
            run.island
        );
        run.island.completed = true;
        let encoded = serde_json::to_string(&run.island).unwrap();
        let restored: Island = serde_json::from_str(&encoded).unwrap();
        assert!(restored.completed && restored.victory(level));
        assert_eq!(restored.stores, run.island.stores);
        assert_eq!(restored.position, run.island.position);
        total_visits += run.visits;
        total_cycles += run.rounds;
        println!(
            "{} {:02}/12 · {} Wege · {} Arbeitsgänge",
            level.region.name(),
            level.stage + 1,
            run.visits,
            run.rounds
        );
    }
    assert!(total_visits > 600 && total_cycles > 400);
}

#[test]
fn every_resource_and_landmark_is_on_land_and_outside_the_plots() {
    for index in 0..LEVELS {
        let level = Level::at(index);
        let map = Terrain::new(level);
        assert!(map.walkable(Vec2::new(0.0, 3.2), &vec![None; level.plots()]));
        for good in Good::RAW {
            assert!(map.deposits.iter().any(|d| d.kind == good));
        }
        for p in &map.plots {
            assert!(map.accessible(*p));
            assert!(p.length() > 3.75);
        }
        for deposit in &map.deposits {
            assert!(
                map.accessible(deposit.at),
                "{}: {:?} im Wasser",
                level.name,
                deposit.kind
            );
            assert!(
                map.plots.iter().all(|plot| plot.distance(deposit.at) > 2.2),
                "{}: {:?} überlappt mit einem Bauplatz",
                level.name,
                deposit
            );
        }
        if level.plan().is_some() {
            assert!(map.accessible(map.shrine));
            assert!(map.plots.iter().all(|plot| plot.distance(map.shrine) > 3.0));
        }
        if level.expedition() {
            assert!(map.accessible(map.relic));
            assert!(
                map.height(map.relic) > 3.4,
                "{}: das Fundstück liegt im Flachland",
                level.name
            );
        }
    }
}

#[test]
fn empty_inventory_stops_work_without_spending_and_starts_again_at_night() {
    let mut run = Expedition::start(0);
    let level = run.terrain.level;
    let home = run.build(Build::Harbor(level.home()), None);
    let saw = run.build(Build::Sawmill, None);
    let work = Task::Produce(saw);
    assert!(run.island.assign(work, &run.terrain));
    assert!(!run.island.assign(Task::Produce(home), &run.terrain));
    let before = run.island.stores;
    run.island.tick_work(&run.terrain);
    assert_eq!(run.island.stores, before);
    run.island.phase = 0.05;
    run.supply(Good::Wood, 2);
    run.supply(Good::Food, 1);
    run.island.tick_work(&run.terrain);
    assert_eq!(run.island.amount(Good::Boards), 4);
    assert!(run.island.unassign(work));
}

#[test]
fn original_and_new_outfits_use_independent_bits_and_shared_rewards() {
    use crate::cosmetics::Cosmetic;
    let saved = r#"{"owned":1,"garment":"Sea","head":null}"#;
    let mut wardrobe: crate::cosmetics::Wardrobe = serde_json::from_str(saved).unwrap();
    assert!(wardrobe.wearing(Cosmetic::Sea));
    assert!(wardrobe.equip(Cosmetic::RoseFlask, 9));
    assert!(wardrobe.owns(Cosmetic::Sea));
    assert!(wardrobe.owns(Cosmetic::RoseFlask));
    assert!(wardrobe.wearing(Cosmetic::Sea));
    let result: crate::cosmetics::Wardrobe =
        serde_json::from_str(&serde_json::to_string(&wardrobe).unwrap()).unwrap();
    assert!(result.wearing(Cosmetic::RoseFlask));

    let mut save = Campaign::default();
    let item = Cosmetic::AegeanChiton;
    assert!(!save.cosmetic_available(item));
    let milestone = item.requires_adventure().unwrap();
    for index in 0..=milestone {
        save.reward_adventure(index);
    }
    save.reward_adventure(milestone);
    assert_eq!(save.available_stars(), (milestone as u16 + 1) * 2);
    assert!(save.cosmetic_available(item));
    assert!(!save.equip(item));
    save.reward_adventure(milestone + 1);
    assert!(save.equip(item));
}

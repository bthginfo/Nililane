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
        if here.distance(position) < distance {
            return;
        }
        let route = self
            .terrain
            .route(here, position, distance - 0.05, &self.island.lots)
            .unwrap_or_else(|| {
                panic!(
                    "{}: keine Route von {here:?} nach {position:?}",
                    self.terrain.level.name
                )
            });
        self.island.position = route.first().copied().unwrap_or(here).to_array();
        self.visits += 1;
    }

    fn supply(&mut self, good: Good, target: u16) {
        if self.island.amount(good) >= target {
            return;
        }
        if Good::RAW.contains(&good) {
            while self.island.amount(good) < target {
                while self.island.free_storage() < NILI_CAPACITY {
                    let surplus = Good::ALL
                        .into_iter()
                        .filter(|&kind| {
                            kind.uses_storage() && kind != good && self.island.amount(kind) > 0
                        })
                        .max_by_key(|&kind| self.island.amount(kind))
                        .unwrap_or_else(|| {
                            panic!("{}: Kein Platz für {good:?}", self.terrain.level.name)
                        });
                    assert!(self.island.discard_stock(surplus) > 0);
                }
                let index = self
                    .island
                    .available_near(&self.terrain, good, terrain::HALL_DOOR)
                    .unwrap_or_else(|| {
                        panic!(
                            "{}: Inselvorrat an {good:?} erschöpft",
                            self.terrain.level.name
                        )
                    });
                let deposit = self.terrain.deposits[index];
                self.walk_to(deposit.at, reach(Target::Deposit(index)));
                let before = self.island.amount(good);
                assert!(self.island.carry_resource(&self.terrain, index).is_some());
                assert_eq!(self.island.amount(good), before);
                self.walk_to(terrain::HALL_DOOR, 1.15);
                assert!(self.island.unload().is_some());
                assert!(
                    self.island.cargo.is_none(),
                    "{}: Lager kann die Fracht nicht aufnehmen",
                    self.terrain.level.name
                );
            }
            return;
        }
        let level = self.terrain.level;
        let output = |build: Build| {
            build
                .recipe(1, level.region)
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
        let recipe = property.kind.recipe(property.tier, level.region).unwrap();
        assert!(
            recipe.need.iter().all(|&(input, _)| input != good),
            "{}: Produktionskreislauf für {good:?}",
            level.name
        );
        while self.island.amount(good) < target {
            for _ in 0..4 {
                if self.island.ready(&recipe.need) {
                    break;
                }
                for &(input, amount) in recipe.need.iter().rev() {
                    self.supply(input, amount);
                }
            }
            let assigned = self.island.assign(task, &self.terrain);
            if !assigned {
                self.walk_to(self.terrain.plots[lot], reach(Target::Plot(lot)));
            }
            assert!(
                self.island.start_production(&self.terrain, lot),
                "{}: Betrieb {property:?} steht still",
                level.name
            );
            let batch = self.island.batch(lot).unwrap();
            self.island
                .tick_production(&self.terrain, batch.duration + 0.01);
            assert!(
                self.island.batch(lot).is_none(),
                "{}: Lager voll für {good:?}",
                level.name
            );
            if assigned {
                assert!(self.island.unassign(task));
            }
            self.rounds += 1;
        }
        assert!(
            self.island.amount(good) >= target,
            "{}: zu wenig {good:?}",
            level.name
        );
    }

    fn build(&mut self, kind: Build, previous: Option<usize>) -> usize {
        let level = self.terrain.level;
        let vacant = |town: &Island| {
            town.lots
                .iter()
                .enumerate()
                .find(|(lot, p)| p.is_none() && kind.fits(level, *lot))
                .map(|(lot, _)| lot)
        };
        let lot = previous
            .or_else(|| vacant(&self.island))
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
            vacant(&self.island).unwrap()
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
        let offset = Vec2::from_array(self.island.position) - at;
        if Terrain::inside_building(offset) {
            self.island.position = (at + Terrain::just_outside(offset)).to_array();
        }
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
        if level.stage >= 1 {
            run.build(Build::Harbor(crate::model::Kind::Storehouse), None);
        }
        if level.stage >= 7 {
            run.build(Build::Harbor(crate::model::Kind::Storehouse), None);
        }
        if level.plan().is_some() {
            run.quest();
        }
        run.requirements(3);
        for goal in level.goals() {
            while run.island.progress(goal) < goal.count {
                let slot = run
                    .island
                    .lots
                    .iter()
                    .position(|p| p.is_some_and(|p| p.kind == goal.kind && p.tier < goal.tier));
                run.build(goal.kind, slot);
            }
        }
        for index in 0..level.order_count() {
            let order = level.order(index).unwrap();
            for _ in 0..4 {
                if run.island.ready(&order.goods) {
                    break;
                }
                for (good, count) in order.goods {
                    run.supply(good, count);
                }
            }
            run.walk_to(terrain::HALL_DOOR, 1.1);
            assert!(
                run.island.pack_order(level),
                "{}: {}",
                level.name,
                order.title
            );
            run.walk_to(run.terrain.outpost, reach(Target::Outpost));
            assert!(run.island.deliver_order(level));
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
        if level.order_count() > 0 {
            assert!(map.accessible(map.outpost));
            assert!(
                map.plots
                    .iter()
                    .all(|plot| plot.distance(map.outpost) > 3.1)
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
    run.island.tick_production(&run.terrain, 1.0);
    assert_eq!(run.island.stores, before);
    run.island.phase = 0.05;
    run.supply(Good::Wood, 2);
    run.supply(Good::Food, 1);
    run.island.tick_production(&run.terrain, 0.1);
    assert_eq!(run.island.amount(Good::Boards), 0);
    let duration = run.island.batch(saw).unwrap().duration;
    run.island.tick_production(&run.terrain, duration + 0.01);
    assert_eq!(run.island.amount(Good::Boards), 4);
    assert!(run.island.unassign(work));
}

#[test]
fn nili_finishes_the_work_animation_then_unloads_once() {
    let mut adventure = Adventure::default();
    adventure.open(0, true);
    adventure.screen = Screen::Walk;
    let mut campaign = Campaign::default();
    let mut game = Game::default();
    game.view = View::Adventure;
    let target = Target::Deposit(0);
    let at = adventure.target_position(target).unwrap() + Vec2::X;
    adventure.active_mut().unwrap().position = at.to_array();
    act(
        Command::Use(target),
        &mut adventure,
        &mut game,
        &mut campaign,
    );
    assert!(adventure.harvesting.is_some());
    process_work(&mut adventure, &mut game, &mut campaign, 0.17);
    assert!(adventure.active().unwrap().cargo.is_none());
    for _ in 0..24 {
        process_work(&mut adventure, &mut game, &mut campaign, 0.17);
    }
    let island = adventure.active().unwrap();
    assert_eq!(island.amount(Good::Wood), 0);
    assert_eq!(island.cargo.unwrap().units, NILI_CAPACITY);
    assert!(adventure.target_position(target).is_none());
    act(
        Command::Use(Target::Hall),
        &mut adventure,
        &mut game,
        &mut campaign,
    );
    assert!(adventure.active().unwrap().cargo.is_some());
    adventure.active_mut().unwrap().position = terrain::HALL_DOOR.to_array();
    act(
        Command::Use(Target::Hall),
        &mut adventure,
        &mut game,
        &mut campaign,
    );
    act(
        Command::Use(Target::Hall),
        &mut adventure,
        &mut game,
        &mut campaign,
    );
    assert!(adventure.active().unwrap().cargo.is_none());
    assert_eq!(
        adventure.active().unwrap().amount(Good::Wood),
        NILI_CAPACITY
    );
}

#[test]
fn a_citizen_walks_extracts_and_returns_with_saved_cargo() {
    let mut run = Expedition::start(3);
    let level = run.terrain.level;
    run.build(Build::Harbor(level.home()), None);
    let site = run
        .island
        .available_near(&run.terrain, Good::Stone, terrain::HALL_DOOR)
        .unwrap();
    let task = Task::Gather(site);
    assert!(run.island.assign(task, &run.terrain));
    let before = run.island.amount(Good::Stone);
    for _ in 0..1200 {
        run.island.tick_hauling(&run.terrain, 0.10);
        if run.island.hauls[0].cargo.is_some() {
            break;
        }
    }
    let haul = &run.island.hauls[0];
    let load = haul.cargo.unwrap();
    assert_eq!(run.island.amount(Good::Stone), before);
    assert!(Vec2::from_array(haul.position).distance(terrain::HALL_DOOR) > 2.0);
    assert!(run.island.unassign(task));
    let snapshot = serde_json::to_string(&run.island).unwrap();
    let mut restored: Island = serde_json::from_str(&snapshot).unwrap();
    restored.prepare_logistics(&run.terrain);
    for _ in 0..1200 {
        restored.tick_hauling(&run.terrain, 0.10);
        if restored.workers.is_empty() {
            break;
        }
    }
    assert!(restored.workers.is_empty());
    assert_eq!(restored.amount(Good::Stone), before + load.units);
    assert_eq!(
        restored.remaining(&run.terrain, site),
        run.island.remaining(&run.terrain, site)
    );
}

#[test]
fn a_full_hall_keeps_the_remainder_on_nili_and_a_warehouse_expands_capacity() {
    let level = Level::at(0);
    let terrain = Terrain::new(level);
    let mut island = Island::new(level);
    assert_eq!(island.add(Good::Stone, 45), 45);
    assert_eq!(island.carry_resource(&terrain, 0).unwrap().units, 10);
    assert_eq!(island.unload().unwrap().units, 3);
    assert!(island.unload().is_none());
    assert_eq!(island.cargo.unwrap().units, 7);
    assert_eq!(island.discard_stock(Good::Stone), 5);
    assert_eq!(island.unload().unwrap().units, 5);
    assert_eq!(island.cargo.unwrap().units, 2);
    assert!(island.build(level, 0, Build::Harbor(crate::model::Kind::Storehouse)));
    assert_eq!(island.storage_capacity(), 84);
    assert_eq!(island.unload().unwrap().units, 2);
    assert!(island.cargo.is_none());
}

#[test]
fn nordic_fish_requires_a_coastal_hut_a_worker_and_a_completed_trip() {
    use crate::model::Kind;
    let mut run =
        Expedition::start(Region::Nordic.start() / crate::campaign::REGION_SIZE * CORE_HARBORS);
    let level = run.terrain.level;
    assert!(!run.terrain.fishing.is_empty());
    run.build(Build::Harbor(level.home()), None);
    let lot = run.build(Build::Harbor(Kind::Fishery), None);
    assert!(level.coastal(lot));
    assert!(
        run.terrain
            .fishing
            .iter()
            .all(|place| run.terrain.shore(place.at) < 0.0)
    );
    run.supply(Good::Food, 1);
    assert!(!run.island.start_production(&run.terrain, lot));
    assert!(run.island.assign(Task::Produce(lot), &run.terrain));
    let fish_at_sea = run.island.fish_left.iter().sum::<u16>();
    assert!(run.island.start_production(&run.terrain, lot));
    let trip = run.island.batch(lot).unwrap();
    assert!(trip.fishing.is_some());
    assert!(trip.duration > Build::Sawmill.recipe(1, level.region).unwrap().seconds);
    run.island
        .tick_production(&run.terrain, trip.duration * 0.45);
    assert_eq!(run.island.amount(Good::Fish), 0);
    let saved = serde_json::to_vec(&run.island).unwrap();
    let mut loaded: Island = serde_json::from_slice(&saved).unwrap();
    loaded.tick_production(&run.terrain, trip.duration);
    assert_eq!(loaded.amount(Good::Fish), trip.output.units);
    assert_eq!(
        loaded.fish_left.iter().sum::<u16>(),
        fish_at_sea - trip.output.units
    );
}

#[test]
fn an_older_island_keeps_stored_material_when_the_inventory_grows() {
    let level = Level::at(0);
    let mut value = serde_json::to_value(Island::new(level)).unwrap();
    let supplies = value["stores"].as_array_mut().unwrap();
    supplies.truncate(14);
    supplies[Good::Stone as usize] = serde_json::json!(32);
    for key in [
        "cargo",
        "remaining",
        "hauls",
        "shipment",
        "orders_delivered",
        "fish_left",
        "batches",
    ] {
        value.as_object_mut().unwrap().remove(key);
    }
    let restored: Island = serde_json::from_value(value).unwrap();
    assert_eq!(restored.amount(Good::Stone), 32);
    assert_eq!(restored.amount(Good::Fish), 0);
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

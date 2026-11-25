use super::*;

#[test]
fn pre_masterwork_saves_keep_their_stock_and_in_progress_sites() {
    let level = Level::at(6);
    let mut island = Island::new(level);
    island.add(Good::Wood, 11);
    assert!(island.plan_build(level, 0, Build::Sawmill));
    let mut snapshot = serde_json::to_value(&island).unwrap();
    snapshot["stores"].as_array_mut().unwrap().truncate(19);
    for field in ["needed", "materials"] {
        snapshot["construction"][0][field]
            .as_array_mut()
            .unwrap()
            .truncate(19);
    }
    for field in [
        "masterwork_lots",
        "base_crafted",
        "overflow",
        "dropped",
        "encounters",
        "market",
    ] {
        snapshot.as_object_mut().unwrap().remove(field);
    }
    let restored: Island = serde_json::from_value(snapshot).unwrap();
    assert_eq!(restored.amount(Good::Wood), 11);
    assert_eq!(restored.amount(Good::Masterwork), 0);
    assert_eq!(
        restored.site(0).unwrap().needed,
        island.site(0).unwrap().needed
    );
    assert_eq!(restored.base_crafted, 0);
    assert_eq!(restored.overflow_total(), 0);
    assert!(restored.dropped.is_empty());
    assert!(!restored.market.blueprint);
}

#[test]
fn island_encounters_appear_through_play_and_use_walkable_locations() {
    let kinds = [
        Event::Arrival,
        Event::Fire,
        Event::Ruin,
        Event::Fever,
        Event::Festival,
        Event::Relic,
    ];
    for region in Region::ALL {
        let mut seen = [false; 6];
        for stage in 1..CORE_HARBORS {
            let level = Level::at(region as usize * CORE_HARBORS + stage);
            let map = Terrain::new(level);
            let mut island = Island::new(level);
            assert!(island.update_encounters(level, 12.0).offered.is_empty());
            // Later visitors only come once the island has reached its goals.
            for goal in level.goals() {
                for _ in island.progress(goal)..goal.count {
                    let free = (0..island.lots.len())
                        .find(|&lot| island.lots[lot].is_none() && goal.kind.fits(level, lot))
                        .unwrap();
                    island.lots[free] = Some(Property {
                        kind: goal.kind,
                        tier: goal.tier,
                    });
                }
            }
            let events = island.update_encounters(level, 200.0);
            assert_eq!(events.offered.len(), level.encounter_count());
            let mut on_island = [false; 6];
            for id in events.offered {
                let spec = level.encounter(id).unwrap();
                let kind = kinds.iter().position(|&kind| kind == spec.event).unwrap();
                assert!(!on_island[kind]);
                on_island[kind] = true;
                seen[kind] = true;
                let at = map.events[id];
                assert!(
                    map.walkable(at, &island.lots)
                        && map.plots.iter().all(|p| p.distance(at) > 3.3),
                    "{} · {}",
                    level.name,
                    spec.title
                );
                assert!(
                    map.route(terrain::HALL_DOOR, at, 1.1, &island.lots)
                        .is_some(),
                    "{} · {}",
                    level.name,
                    spec.title
                );
                let source = match spec.event {
                    Event::Fire => map.shore_intake,
                    Event::Relic => map.finds[id],
                    _ => terrain::HALL_DOOR,
                };
                assert!(
                    map.route(at, source, 1.3, &island.lots).is_some(),
                    "{} · {}",
                    level.name,
                    spec.title
                );
                island.encounters.changed(id, EncounterStatus::Seeking);
                if let Some((good, count)) = spec.cargo
                    && island.amount(good) < count
                {
                    assert!(!island.encounter_pickup(level, id));
                    island.add(good, count - island.amount(good));
                }
                assert!(island.encounter_pickup(level, id));
                assert!(island.has_cargo());
                assert!(!island.pack_order(level));
                island.encounters.changed(id, EncounterStatus::Working);
                let before = island.amount(spec.reward.0);
                assert_eq!(island.update_encounters(level, 10.1).finished, [id]);
                assert_eq!(island.amount(spec.reward.0), before + spec.reward.1);
                assert!(island.update_encounters(level, 12.0).finished.is_empty());
            }
        }
        assert!(seen.into_iter().all(|appeared| appeared));
    }
}

#[test]
fn a_fire_can_be_put_out_using_a_well_or_the_shore() {
    let mut adventure = Adventure::default();
    adventure.open(2, true);
    adventure.screen = Screen::Walk;
    let mut game = Game::default();
    let mut save = Campaign::default();
    let map = adventure.terrain.clone().unwrap();
    assert_eq!(map.level.encounter(0).unwrap().event, Event::Fire);
    let site = Target::Encounter(0);
    let water = Target::EncounterSource(0);
    let island = adventure.active_mut().unwrap();
    island.encounters.changed(0, EncounterStatus::Offered);
    island.position = map.events[0].to_array();
    act(Command::Use(site), &mut adventure, &mut game, &mut save);
    assert_eq!(adventure.encounter_target(0), Some(water));
    assert_eq!(adventure.target_position(water), Some(map.shore_intake));
    let island = adventure.active_mut().unwrap();
    island.lots[0] = Some(Property {
        kind: Build::Harbor(map.level.well()),
        tier: 1,
    });
    assert_eq!(adventure.target_position(water), Some(map.plots[0]));
    adventure.active_mut().unwrap().lots[0] = None;
    adventure.active_mut().unwrap().position = map.shore_intake.to_array();
    act(Command::Use(water), &mut adventure, &mut game, &mut save);
    assert_eq!(adventure.waypoint, Some(site));
    adventure.active_mut().unwrap().position = map.events[0].to_array();
    act(Command::Use(site), &mut adventure, &mut game, &mut save);
    assert_eq!(
        adventure.active().unwrap().encounters.visits[0].status,
        EncounterStatus::Working
    );
    assert_eq!(
        adventure
            .active_mut()
            .unwrap()
            .update_encounters(map.level, 3.0)
            .finished,
        [0]
    );
    assert!(adventure.target_position(site).is_none());
}

#[test]
fn showing_a_waypoint_leaves_nili_in_place_and_finds_a_walkable_detour() {
    let mut adventure = Adventure::default();
    adventure.open(0, true);
    let mut game = Game::default();
    let mut save = Campaign::default();
    let start = Vec2::new(-5.0, -0.3);
    adventure.active_mut().unwrap().position = start.to_array();
    let target = Target::Deposit(1);

    act(
        Command::SetRoute(target),
        &mut adventure,
        &mut game,
        &mut save,
    );
    assert_eq!(adventure.screen, Screen::Walk);
    assert_eq!(adventure.active().unwrap().position, start.to_array());
    let path = adventure.waypoint_path().unwrap();
    assert_eq!(path[0], start);
    assert!(path.len() > 2, "the direct path crosses the town hall");
    let destination = adventure.target_position(target).unwrap();
    assert!(path.last().unwrap().distance(destination) < reach(target));
    let terrain = adventure.terrain.as_ref().unwrap();
    let lots = &adventure.active().unwrap().lots;
    for edge in path.windows(2) {
        assert!(
            terrain
                .walk(edge[0], edge[1] - edge[0], lots)
                .distance(edge[1])
                < 0.001
        );
    }

    act(Command::ClearRoute, &mut adventure, &mut game, &mut save);
    assert!(adventure.waypoint_path().is_none());
}

#[test]
fn a_completed_island_can_be_played_on_and_its_summary_reopened() {
    let mut adventure = Adventure {
        book: Book::default(),
        ..Adventure::default()
    };
    adventure.open(0, false);
    adventure.active_mut().unwrap().completed = true;
    let mut game = Game::default();
    let mut save = Campaign::default();
    save.reward_adventure(0);
    let stars = save.available_stars();

    act(Command::Summary, &mut adventure, &mut game, &mut save);
    assert_eq!(adventure.screen, Screen::Finish);
    assert_eq!(first(&adventure), Some(Action::Adventure(Command::Next)));
    act(Command::Close, &mut adventure, &mut game, &mut save);
    assert_eq!(adventure.screen, Screen::Walk);
    assert_eq!(adventure.active().unwrap().index, 0);
    assert!(adventure.unlocked(1));
    adventure.active_mut().unwrap().position = [5.0, 2.1];
    act(Command::Summary, &mut adventure, &mut game, &mut save);
    assert_eq!(adventure.active().unwrap().position, [5.0, 2.1]);
    assert_eq!(adventure.next_island().unwrap().index, 1);
    assert_eq!(save.available_stars(), stars);

    adventure.open(LEVELS - 1, false);
    adventure.active_mut().unwrap().completed = true;
    act(Command::Summary, &mut adventure, &mut game, &mut save);
    assert_eq!(first(&adventure), Some(Action::Adventure(Command::Close)));
}

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
                while self.island.free_storage() < self.island.capacity() {
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
        if good == Good::Masterwork {
            self.supply_masterwork(target);
            return;
        }
        if good == Good::Imports {
            while self.island.amount(good) < target {
                self.ensure_trade_marks(IMPORT_PRICE);
                self.walk_to(self.terrain.trade_pier, reach(Target::TradeHarbor));
                assert!(self.island.buy_imports(level));
                self.walk_to(terrain::HALL_DOOR, 1.15);
                assert_eq!(self.island.unload().unwrap().units, IMPORT_UNITS);
            }
            return;
        }
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
        let had_masterwork = self.island.set_masterwork(level, lot, false);
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
        if had_masterwork {
            assert!(self.island.set_masterwork(level, lot, true));
        }
    }

    fn supply_masterwork(&mut self, target: u16) {
        let level = self.terrain.level;
        let order = level.work_order().unwrap();
        let lot = self
            .island
            .lots
            .iter()
            .position(|property| property.is_some_and(|property| property.kind == order.workshop))
            .unwrap_or_else(|| self.build(order.workshop, None));
        if self.island.base_crafted < order.base_needed {
            assert!(!self.island.set_masterwork(level, lot, true));
            while self.island.base_crafted < order.base_needed {
                self.run_workshop(lot);
            }
        }
        while self.island.lots[lot].unwrap().tier < order.tier {
            assert!(!self.island.set_masterwork(level, lot, true));
            self.build(order.workshop, Some(lot));
        }
        if !self.island.makes_masterwork(lot) {
            assert!(self.island.set_masterwork(level, lot, true));
        }
        while self.island.amount(Good::Masterwork) < target {
            self.run_workshop(lot);
        }
        assert!(self.island.set_masterwork(level, lot, false));
    }

    fn ensure_trade_marks(&mut self, target: u16) {
        let level = self.terrain.level;
        let offer = level.export(Export::Regular).unwrap();
        while self.island.market.marks < target {
            self.supply(offer.good, offer.units);
            self.walk_to(terrain::HALL_DOOR, 1.15);
            assert!(self.island.pack_export(level, Export::Regular));
            self.walk_to(self.terrain.trade_pier, reach(Target::TradeHarbor));
            assert!(self.island.launch_export(level));
            assert!(
                self.island
                    .tick_export(level, offer.seconds * 0.5)
                    .is_none()
            );
            assert_eq!(
                self.island.tick_export(level, offer.seconds * 0.5 + 0.01),
                Some(offer.marks)
            );
        }
    }

    fn run_workshop(&mut self, lot: usize) {
        let level = self.terrain.level;
        let recipe = self.island.recipe_for(level, lot).unwrap();
        for (good, quantity) in recipe.need {
            self.supply(good, quantity);
        }
        self.walk_to(self.terrain.plots[lot], reach(Target::Plot(lot)));
        let before = self.island.amount(recipe.output.0);
        assert!(
            self.island.start_production(&self.terrain, lot),
            "{}: {lot} steht still",
            level.name
        );
        let batch = self.island.batch(lot).unwrap();
        self.island
            .tick_production(&self.terrain, batch.duration * 0.5);
        assert_eq!(self.island.amount(recipe.output.0), before);
        self.island
            .tick_production(&self.terrain, batch.duration * 0.5 + 0.01);
        assert!(self.island.amount(recipe.output.0) > before);
        self.rounds += 1;
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
            self.island.plan_build(level, lot, kind),
            "{}: Bau abgelehnt für {kind:?} Stufe {tier}",
            level.name
        );
        assert!(self.island.with_crew(lot, -1));
        while !self.island.site(lot).unwrap().ready() {
            self.walk_to(terrain::HALL_DOOR, 1.15);
            let load = self.island.take_building_load(lot).unwrap();
            self.walk_to(at, reach(Target::Plot(lot)));
            assert_eq!(self.island.drop_building_load(lot), Some(load));
        }
        let seconds = self.island.site(lot).unwrap().seconds();
        assert_eq!(
            self.island
                .tick_construction(&self.terrain, seconds + 0.03, Some(lot))
                .1,
            [lot]
        );
        assert!(!Terrain::inside_building(
            Vec2::from_array(self.island.position) - at
        ));
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
            if goal.kind == Build::Monument {
                assert!(run.island.locked(level, goal.kind, 1));
                run.ensure_trade_marks(PLAN_PRICE);
                run.walk_to(run.terrain.trade_pier, reach(Target::TradeHarbor));
                assert!(run.island.buy_monument(level));
            }
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
fn the_trade_harbor_respects_positions_and_a_crossing_pays_once_after_reload() {
    for region in Region::ALL {
        let index = region as usize * CORE_HARBORS + 8;
        let mut a = Adventure::default();
        a.open(index, true);
        a.screen = Screen::Walk;
        let map = a.terrain.clone().unwrap();
        let offer = map.level.export(Export::Regular).unwrap();
        assert!(
            map.route(
                terrain::HALL_DOOR,
                map.trade_pier,
                1.4,
                &a.active().unwrap().lots
            )
            .is_some()
        );
        a.active_mut().unwrap().add(offer.good, offer.units);
        let mut game = Game::default();
        let mut campaign = Campaign::default();
        act(
            Command::PackExport(Export::Regular),
            &mut a,
            &mut game,
            &mut campaign,
        );
        assert_eq!(a.waypoint, Some(Target::TradeHarbor));
        assert_eq!(a.active().unwrap().amount(offer.good), 0);
        act(Command::SellExport, &mut a, &mut game, &mut campaign);
        assert!(a.active().unwrap().market.crossing.is_none());
        a.active_mut().unwrap().position = map.trade_pier.to_array();
        act(Command::SellExport, &mut a, &mut game, &mut campaign);
        let island = a.active_mut().unwrap();
        assert_eq!(island.tick_export(map.level, 7.0), None);
        let state = serde_json::to_vec(island).unwrap();
        let mut restored: Island = serde_json::from_slice(&state).unwrap();
        assert_eq!(
            restored.tick_export(map.level, offer.seconds),
            Some(offer.marks)
        );
        assert_eq!(restored.tick_export(map.level, offer.seconds), None);
        assert_eq!(restored.market.marks, offer.marks);
        *a.active_mut().unwrap() = restored;
        a.active_mut().unwrap().position = terrain::HALL_DOOR.to_array();
        act(Command::BuyImports, &mut a, &mut game, &mut campaign);
        assert_eq!(a.active().unwrap().market.marks, offer.marks);
        assert!(a.active().unwrap().cargo.is_none());

        a.active_mut().unwrap().position = map.trade_pier.to_array();
        act(Command::BuyImports, &mut a, &mut game, &mut campaign);
        assert_eq!(a.waypoint, Some(Target::Hall));
        let island = a.active().unwrap();
        assert_eq!(island.amount(Good::Imports), 0);
        assert_eq!(island.market.marks, offer.marks - IMPORT_PRICE);
        assert_eq!(island.cargo.unwrap().units, IMPORT_UNITS);
        let parcel_save = serde_json::to_vec(island).unwrap();
        *a.active_mut().unwrap() = serde_json::from_slice(&parcel_save).unwrap();
        act(Command::Use(Target::Hall), &mut a, &mut game, &mut campaign);
        assert_eq!(a.active().unwrap().amount(Good::Imports), 0);
        a.active_mut().unwrap().position = terrain::HALL_DOOR.to_array();
        act(Command::Use(Target::Hall), &mut a, &mut game, &mut campaign);
        let island = a.active().unwrap();
        assert_eq!(island.amount(Good::Imports), IMPORT_UNITS);
        assert!(island.cargo.is_none());
        assert!(!island.market.blueprint);
    }
}

#[test]
fn open_menus_refresh_when_an_encounter_appears_or_a_ship_returns() {
    let mut adventure = Adventure::default();
    adventure.open(8, true);
    adventure.screen = Screen::Mission;
    let mut game = Game::default();
    let mut campaign = Campaign::default();
    let level = adventure.terrain.as_ref().unwrap().level;
    let island = adventure.active_mut().unwrap();
    island.lots[0] = Some(Property {
        kind: Build::Harbor(level.home()),
        tier: 1,
    });
    island.encounters.elapsed = 39.94;
    let stock = island.stores;
    adventure.group = 4;
    game.dirty = false;
    process_work(&mut adventure, &mut game, &mut campaign, 0.12);
    assert!(game.dirty);
    let island = adventure.active_mut().unwrap();
    assert_eq!(island.stores, stock);
    assert_eq!(island.encounters.visits[0].status, EncounterStatus::Offered);
    let offer = level.export(Export::Regular).unwrap();
    island.add(offer.good, offer.units);
    assert!(island.pack_export(level, Export::Regular));
    assert!(island.launch_export(level));
    island.market.crossing.as_mut().unwrap().remaining = 0.05;

    adventure.group = 5;
    game.dirty = false;
    process_work(&mut adventure, &mut game, &mut campaign, 0.12);
    assert!(game.dirty);
    assert_eq!(adventure.screen, Screen::Mission);
    let island = adventure.active().unwrap();
    assert_eq!(island.stores, stock);
    assert_eq!(island.market.marks, offer.marks);
    assert!(island.market.crossing.is_none());
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
fn the_last_stop_pauses_a_workshop_without_consuming_inputs_again() {
    let mut adventure = Adventure::default();
    adventure.open(0, true);
    adventure.screen = Screen::Lot(2);
    let map = adventure.terrain.clone().unwrap();
    let task = Task::Produce(2);
    let island = adventure.active_mut().unwrap();
    for lot in &mut island.lots[..2] {
        *lot = Some(Property {
            kind: Build::Harbor(map.level.home()),
            tier: 1,
        });
    }
    island.lots[2] = Some(Property {
        kind: Build::Sawmill,
        tier: 1,
    });
    island.position = (map.plots[2] + Vec2::Y * 2.4).to_array();
    island.add(Good::Wood, 2);
    island.add(Good::Food, 1);
    assert!(island.assign(task, &map));
    assert!(island.assign(task, &map));
    assert!(island.start_production(&map, 2));
    let mut game = Game::default();
    let mut save = Campaign::default();
    act(
        Command::Assign(task, false),
        &mut adventure,
        &mut game,
        &mut save,
    );
    let island = adventure.active_mut().unwrap();
    assert!(island.batch(2).unwrap().working());
    island.tick_production(&map, 1.0);
    let left = island.batch(2).unwrap().left;
    act(
        Command::Assign(task, false),
        &mut adventure,
        &mut game,
        &mut save,
    );
    let island = adventure.active().unwrap();
    assert_eq!(island.assignment(task), 0);
    let mut saved = serde_json::to_value(island).unwrap();
    let mut loaded: Island = serde_json::from_value(saved.clone()).unwrap();
    assert!(!loaded.batch(2).unwrap().working());
    assert!(!loaded.tick_production(&map, left + 10.0));
    assert_eq!(loaded.batch(2).unwrap().left, left);
    assert_eq!(loaded.amount(Good::Boards), 0);
    assert_eq!(loaded.amount(Good::Wood), 0);
    assert_eq!(loaded.amount(Good::Food), 0);

    // Old saves with a batch in progress had no pause state.
    saved["batches"][2]
        .as_object_mut()
        .unwrap()
        .remove("paused");
    let old: Island = serde_json::from_value(saved).unwrap();
    assert!(old.batch(2).unwrap().working());
    *adventure.active_mut().unwrap() = loaded;
    act(Command::Work(2), &mut adventure, &mut game, &mut save);
    let island = adventure.active_mut().unwrap();
    assert!(island.batch(2).unwrap().working());
    assert!(island.tick_production(&map, left + 0.1));
    assert_eq!(island.amount(Good::Boards), 4);
    assert!(island.batch(2).is_none());
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
    assert_eq!(island.cargo.unwrap().units, logistics::NILI_CAPACITY);
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
        logistics::NILI_CAPACITY
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
    let snapshot = serde_json::to_string(&run.island).unwrap();
    let mut restored: Island = serde_json::from_str(&snapshot).unwrap();
    restored.prepare_logistics(&run.terrain);
    for _ in 0..1200 {
        restored.tick_hauling(&run.terrain, 0.10);
        if restored.amount(Good::Stone) > before {
            break;
        }
    }
    assert_eq!(restored.assignment(task), 1);
    assert!(restored.hauls[0].cargo.is_none());
    assert_eq!(restored.amount(Good::Stone), before + load.units);
    assert_eq!(
        restored.remaining(&run.terrain, site),
        run.island.remaining(&run.terrain, site)
    );
}

#[test]
fn an_overflowing_haul_stops_and_preserves_its_remainder_at_the_hall() {
    for space in [0, 2] {
        let mut run = Expedition::start(0);
        let kind = Good::Wood;
        let region = run.terrain.level;
        run.build(Build::Harbor(region.home()), None);
        let task = Task::Gather(0);
        assert_eq!(run.terrain.deposits[0].kind, kind);
        let staff = run.island.available_staff();
        assert!(run.island.assign(task, &run.terrain));
        for _ in 0..1600 {
            run.island.tick_hauling(&run.terrain, 0.10);
            if run.island.hauls[0].cargo.is_some() {
                break;
            }
        }
        let load = run.island.hauls[0].cargo.unwrap();
        let count = run.island.amount(kind);
        let fill = run.island.free_storage() - space;
        assert_eq!(run.island.add(Good::Stone, fill), fill);
        for _ in 0..1600 {
            run.island.tick_hauling(&run.terrain, 0.10);
            if run.island.workers.is_empty() {
                break;
            }
        }
        assert_eq!(run.island.available_staff(), staff);
        assert!(run.island.hauls.is_empty());
        assert_eq!(run.island.amount(kind), count + space);
        let outside = load.units - space;
        assert_eq!(run.island.overflow_total(), u32::from(outside));
        assert_eq!(run.island.store_overflow(kind), 0);
        let mut restored: Island =
            serde_json::from_slice(&serde_json::to_vec(&run.island).unwrap()).unwrap();
        let space = restored.discard_stock(Good::Stone);
        assert_eq!(restored.store_overflow(kind), space.min(outside));
        assert_eq!(
            restored.overflow_total(),
            u32::from(outside.saturating_sub(space))
        );
        assert!(restored.workers.is_empty());
    }
}

#[test]
fn stopping_a_gatherer_is_immediate_during_work_and_while_carrying() {
    let mut run = Expedition::start(0);
    let region = run.terrain.level;
    run.build(Build::Harbor(region.home()), None);
    let task = Task::Gather(0);
    let before = run.island.remaining(&run.terrain, 0);
    assert!(run.island.assign(task, &run.terrain));
    for _ in 0..1600 {
        run.island.tick_hauling(&run.terrain, 0.10);
        if run.island.hauls[0].work > 0.0 {
            break;
        }
    }
    assert!(run.island.hauls[0].work > 0.0 && run.island.hauls[0].cargo.is_none());
    assert!(run.island.unassign(task));
    assert_eq!(run.island.assignment(task), 0);
    assert_eq!(run.island.remaining(&run.terrain, 0), before);
    assert!(run.island.dropped.is_empty());

    assert!(run.island.assign(task, &run.terrain));
    for _ in 0..1600 {
        run.island.tick_hauling(&run.terrain, 0.10);
        if run.island.hauls[0].cargo.is_some() {
            break;
        }
    }
    let load = run.island.hauls[0].cargo.unwrap();
    let position = run.island.hauls[0].position;
    let warehouse = run.island.amount(load.kind);
    assert!(run.island.unassign(task));
    assert!(run.island.hauls.is_empty());
    assert!(run.island.workers.is_empty());
    let mut restored: Island =
        serde_json::from_slice(&serde_json::to_vec(&run.island).unwrap()).unwrap();
    assert_eq!(
        restored.dropped_load(0),
        Some(DroppedLoad {
            cargo: load,
            position
        })
    );
    assert_eq!(restored.amount(load.kind), warehouse);
    assert_eq!(restored.pick_up_load(0), Some(load));
    assert_eq!(restored.cargo, Some(load));
    assert!(restored.pick_up_load(0).is_none());
}

#[test]
fn the_left_stick_press_toggles_storage_and_allows_sorting() {
    use bevy::ecs::system::SystemState;
    let mut world = World::new();
    let mut state = SystemState::<(ui::FocusButtons, ui::WalkPad)>::new(&mut world);
    let (buttons, pad) = state.get(&world);
    let mut a = Adventure::default();
    a.open(0, true);
    a.screen = Screen::Walk;
    let mut game = Game::default();
    let mut save = Campaign::default();
    let mut controls = Controls::default();
    controls.pressed = controls::L3;
    let window = Window {
        focused: true,
        ..default()
    };
    let hud = Hud::default();
    let mouse = ButtonInput::default();
    let touches = Touches::default();
    a.active_mut().unwrap().add(Good::Stone, 10);
    step(
        &mut a, &mut game, &mut save, &controls, &buttons, None, &hud, &window, None, &mouse,
        &touches, &pad, 0.0,
    );
    assert_eq!(a.screen, Screen::Storage);
    act(
        Command::ClearStock(Good::Stone),
        &mut a,
        &mut game,
        &mut save,
    );
    assert_eq!(a.active().unwrap().amount(Good::Stone), 5);
    step(
        &mut a, &mut game, &mut save, &controls, &buttons, None, &hud, &window, None, &mouse,
        &touches, &pad, 0.0,
    );
    assert_eq!(a.screen, Screen::Walk);
}

#[test]
fn a_full_hall_keeps_the_remainder_on_nili_and_a_warehouse_expands_capacity() {
    let level = Level::at(0);
    let terrain = Terrain::new(level);
    let mut island = Island::new(level);
    assert_eq!(island.add(Good::Stone, HALL_STORAGE - 3), HALL_STORAGE - 3);
    assert_eq!(island.carry_resource(&terrain, 0).unwrap().units, 10);
    assert_eq!(island.unload().unwrap().units, 3);
    assert!(island.unload().is_none());
    assert_eq!(island.cargo.unwrap().units, 7);
    assert_eq!(island.discard_stock(Good::Stone), 5);
    assert_eq!(island.unload().unwrap().units, 5);
    assert_eq!(island.cargo.unwrap().units, 2);
    assert!(island.unload().is_none());

    let mut run = Expedition::start(0);
    run.build(Build::Harbor(crate::model::Kind::Storehouse), None);
    assert_eq!(run.island.storage_capacity(), HALL_STORAGE + 36);
}

#[test]
fn building_from_the_hand_uses_no_storage_and_requires_time() {
    let mut run = Expedition::start(0);
    let level = run.terrain.level;
    let home = Build::Harbor(level.home());
    assert!(run.island.plan_build(level, 2, home));
    assert!(run.island.lots[2].is_none());
    assert!(
        run.island
            .tick_construction(&run.terrain, 20.0, Some(2))
            .1
            .is_empty()
    );
    assert_eq!(run.island.add(Good::Stone, HALL_STORAGE), HALL_STORAGE);
    let wood = run.island.carry_resource(&run.terrain, 0).unwrap();
    assert!(run.island.unload().is_none());
    let laid = run.island.drop_building_load(2).unwrap();
    assert_eq!(laid.kind, Good::Wood);
    assert!(laid.units > 0);
    assert_eq!(run.island.raw_stock(), HALL_STORAGE);
    assert_eq!(
        run.island.cargo.map_or(0, |remainder| remainder.units),
        wood.units - laid.units
    );
    for (good, _) in home.cost(1) {
        let need = run.island.site(2).unwrap().missing(good);
        if need > 0 {
            run.island.cargo = Some(logistics::Cargo {
                kind: good,
                units: need,
            });
            assert_eq!(run.island.drop_building_load(2).unwrap().units, need);
        }
    }
    let duration = run.island.site(2).unwrap().seconds();
    assert!(
        run.island
            .tick_construction(&run.terrain, duration * 0.6, Some(2))
            .1
            .is_empty()
    );
    assert_eq!(
        run.island
            .tick_construction(&run.terrain, duration * 0.5, Some(2))
            .1,
        [2]
    );
    assert!(run.island.plan_build(level, 2, home));
    assert!(run.island.site(2).unwrap().seconds() > duration);
}

#[test]
fn builders_walk_materials_out_and_a_second_builder_speeds_up_work() {
    let mut run = Expedition::start(0);
    let level = run.terrain.level;
    let home = Build::Harbor(level.home());
    run.build(home, None);
    let lot = 4;
    for (good, count) in home.cost(1) {
        assert_eq!(run.island.add(good, count), count);
    }
    assert!(run.island.plan_build(level, lot, home));
    let mut solo = run.island.clone();
    let mut team = run.island;
    assert!(team.with_crew(lot, 1));
    let walks = |island: &mut Island| {
        island.tick_construction(&run.terrain, 0.12, None);
        island.tick_construction(&run.terrain, 0.12, None);
        assert!(
            island
                .builders
                .iter()
                .any(|builder| builder.walk.cargo.is_some())
        );
        assert!(
            island
                .site(lot)
                .unwrap()
                .materials
                .iter()
                .all(|&units| units == 0)
        );
        (0..650)
            .find(|_| {
                island.tick_construction(&run.terrain, 0.12, None);
                island.lots[lot].is_some()
            })
            .expect("construction must complete")
    };
    assert!(walks(&mut team) < walks(&mut solo));
}

#[test]
fn couriers_finish_a_delivery_once_after_save_and_reload() {
    for region in 0..5 {
        let mut run = Expedition::start(region * CORE_HARBORS + 1);
        let level = run.terrain.level;
        let order = level.order(0).unwrap();
        run.island.lots[0] = Some(Property {
            kind: Build::Harbor(level.home()),
            tier: 1,
        });
        for (good, count) in order.goods {
            assert_eq!(run.island.add(good, count), count);
        }
        assert!(run.island.send_courier(level));
        assert!(!run.island.pack_order(level));
        assert_eq!(run.island.orders_delivered, 0);
        for _ in 0..12 {
            run.island.tick_courier(&run.terrain, 0.12);
        }
        let first_leg = run.island.courier.as_ref().unwrap().walk.position;
        assert_ne!(first_leg, terrain::HALL_DOOR.to_array());
        let snapshot = serde_json::to_vec(&run.island).unwrap();
        let mut restored: Island = serde_json::from_slice(&snapshot).unwrap();
        for _ in 0..1800 {
            restored.tick_courier(&run.terrain, 0.12);
            if restored.courier.is_none() {
                break;
            }
        }
        assert_eq!(restored.orders_delivered, 1, "{}", level.name);
        assert!(restored.courier.is_none());
        assert_eq!(restored.assigned_staff(), 0);
    }
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
    assert!(run.island.unassign(Task::Produce(lot)));
    let left = run.island.batch(lot).unwrap().left;
    assert!(!run.island.tick_production(&run.terrain, trip.duration));
    assert_eq!(run.island.batch(lot).unwrap().left, left);
    assert!(!run.island.start_production(&run.terrain, lot));
    let saved = serde_json::to_vec(&run.island).unwrap();
    let mut loaded: Island = serde_json::from_slice(&saved).unwrap();
    assert!(loaded.assign(Task::Produce(lot), &run.terrain));
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
fn a_finished_building_pushes_walkers_out_and_the_courier_still_delivers() {
    let mut run = Expedition::start(CORE_HARBORS + 1);
    let level = run.terrain.level;
    run.build(Build::Harbor(level.home()), None);
    run.build(Build::Harbor(level.home()), None);
    let lot = run.island.lots.iter().position(Option::is_none).unwrap();
    let plot = run.terrain.plots[lot];
    let home = Build::Harbor(level.home());
    assert!(run.island.plan_build(level, lot, home));
    assert!(run.island.with_crew(lot, -1));
    for (good, count) in home.cost(1) {
        run.island.cargo = Some(logistics::Cargo {
            kind: good,
            units: count,
        });
        assert!(run.island.drop_building_load(lot).is_some());
    }
    let order = level.order(0).unwrap();
    for (good, count) in order.goods {
        assert_eq!(run.island.add(good, count), count);
    }
    assert!(run.island.send_courier(level));
    let gatherer = Task::Gather(0);
    assert!(run.island.assign(gatherer, &run.terrain));
    // Everybody stands on the unfinished site, which is still open ground.
    let inside = (plot + Vec2::new(0.3, -0.2)).to_array();
    run.island.courier.as_mut().unwrap().walk.position = plot.to_array();
    run.island.hauls[0].position = inside;
    run.island.position = inside;
    run.island.drop_load(logistics::DroppedLoad {
        cargo: logistics::Cargo {
            kind: Good::Stone,
            units: 3,
        },
        position: plot.to_array(),
    });
    let seconds = run.island.site(lot).unwrap().seconds();
    let (_, completed) = run
        .island
        .tick_construction(&run.terrain, seconds + 0.03, Some(lot));
    assert_eq!(completed, [lot]);
    let lots = run.island.lots.clone();
    let terrain = run.terrain.clone();
    let outside = |at: [f32; 2]| {
        let at = Vec2::from_array(at);
        !Terrain::inside_building(at - plot) && terrain.walkable(at, &lots)
    };
    assert!(outside(run.island.courier.as_ref().unwrap().walk.position));
    assert!(outside(run.island.hauls[0].position));
    assert!(outside(run.island.position));
    assert!(outside(run.island.dropped_load(0).unwrap().position));
    for _ in 0..2400 {
        run.island.tick_courier(&run.terrain, 0.12);
        if run.island.courier.is_none() {
            break;
        }
    }
    assert_eq!(run.island.orders_delivered, 1);
    assert!(run.island.courier.is_none());

    // Saves from before the fix are repaired when the island is loaded.
    run.island.hauls[0].position = plot.to_array();
    let mut restored: Island =
        serde_json::from_slice(&serde_json::to_vec(&run.island).unwrap()).unwrap();
    restored.prepare_logistics(&run.terrain);
    assert!(outside(restored.hauls[0].position));
}

#[test]
fn a_damaged_island_keeps_the_others_and_the_original_file_as_a_backup() {
    let mut first = Island::new(Level::at(0));
    first.completed = true;
    let mut drifted = Island::new(Level::at(1));
    drifted.position = [900.0, 900.0];
    drifted.add(Good::Wood, 7);
    let mut islands = vec![serde_json::Value::Null; LEVELS];
    islands[0] = serde_json::to_value(&first).unwrap();
    islands[1] = serde_json::to_value(&drifted).unwrap();
    islands[2] = serde_json::json!({ "index": 2, "lots": "kaputt" });
    let text = serde_json::json!({ "version": 1, "last": 1, "islands": islands }).to_string();
    let dir = std::env::temp_dir().join(format!("nililane-abenteuer-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join(BOOK_FILE);
    fs::write(&path, &text).unwrap();

    let book = Book::load(&path);
    assert!(book.islands[0].as_ref().unwrap().completed);
    let island = book.islands[1].as_ref().unwrap();
    assert_eq!(island.position, terrain::HALL_DOOR.to_array());
    assert_eq!(island.amount(Good::Wood), 7);
    assert!(book.islands[2].is_none());
    assert_eq!(book.last, Some(1));
    assert_eq!(
        fs::read_to_string(path.with_extension("bak")).unwrap(),
        text
    );

    fs::write(&path, "{ kaputt").unwrap();
    let book = Book::load(&path);
    assert!(book.islands.iter().all(Option::is_none));
    assert_eq!(
        fs::read_to_string(path.with_extension("bak")).unwrap(),
        "{ kaputt"
    );
    fs::remove_dir_all(&dir).unwrap();

    // The campaign still remembers islands whose save went missing.
    let adventure = Adventure {
        book: Book::default(),
        ..Adventure::default()
    };
    let mut campaign = Campaign::default();
    campaign.reward_adventure(4);
    assert!(!adventure.unlocked(5));
    assert!(!adventure.restore_rewards(&mut campaign));
    assert!(adventure.unlocked(5));
    assert!(!adventure.unlocked(6));
}

#[test]
fn later_visitors_wait_for_progress_come_back_with_new_events_and_ignored_fires_go_out() {
    let level = (0..LEVELS)
        .map(Level::at)
        .find(|level| {
            level.encounter_count() == encounters::SLOTS
                && level
                    .encounter(0)
                    .is_some_and(|spec| spec.event == Event::Fire)
        })
        .unwrap();
    let mut island = Island::new(level);
    island.lots[0] = Some(Property {
        kind: Build::Harbor(level.home()),
        tier: 1,
    });
    assert_eq!(island.update_encounters(level, 400.0).offered, [0]);
    let stock = island.stores;
    assert!(island.update_encounters(level, 149.0).burned_out.is_empty());
    assert_eq!(island.update_encounters(level, 1.5).burned_out, [0]);
    assert_eq!(
        island.encounters.visits[0].status,
        EncounterStatus::Finished
    );
    assert_eq!(island.stores, stock);

    assert!(island.update_encounters(level, 200.0).offered.is_empty());
    assert_eq!(island.update_encounters(level, 41.0).offered, [0]);
    let visit = island.encounters.visits[0];
    assert_eq!(visit.status, EncounterStatus::Offered);
    assert!(visit.round > 0);
    let next = island.encounter(level, 0).unwrap();
    assert_ne!(next.event, Event::Fire);
    for slot in 1..encounters::SLOTS {
        assert_ne!(level.encounter(slot).unwrap().event, next.event);
        assert_eq!(
            island.encounters.visits[slot].status,
            EncounterStatus::Hidden
        );
    }
    let saved: Island = serde_json::from_slice(&serde_json::to_vec(&island).unwrap()).unwrap();
    assert_eq!(saved.encounter(level, 0).unwrap().event, next.event);
    let old: encounters::Visit =
        serde_json::from_str(r#"{"status":"Finished","since":3.0}"#).unwrap();
    assert_eq!(old.round, 0);

    for goal in level.goals() {
        for _ in island.progress(goal)..goal.count {
            let free = (0..island.lots.len())
                .find(|&lot| island.lots[lot].is_none() && goal.kind.fits(level, lot))
                .unwrap();
            island.lots[free] = Some(Property {
                kind: goal.kind,
                tier: goal.tier,
            });
        }
    }
    assert_eq!(island.update_encounters(level, 0.1).offered, [1, 2]);
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

/// Writes developed settlements for visual checks:
/// `NILILANE_SHOWCASE_DIR=/tmp/x cargo test --release showcase -- --ignored`
#[test]
#[ignore]
fn showcase_settlements_for_visual_review() {
    let Some(dir) = std::env::var_os("NILILANE_SHOWCASE_DIR") else {
        return;
    };
    let mut book = Book::default();
    let picks: Vec<usize> = std::env::var("NILILANE_SHOWCASE_ISLANDS")
        .ok()
        .map(|list| list.split(',').filter_map(|i| i.parse().ok()).collect())
        .unwrap_or_else(|| vec![5]);
    for &index in &picks {
        let mut run = Expedition::start(index);
        let level = run.terrain.level;
        run.requirements(2);
        if level.stage >= 1 {
            run.build(Build::Harbor(crate::model::Kind::Storehouse), None);
        }
        run.requirements(3);
        for goal in level.goals() {
            if goal.kind == Build::Monument {
                continue;
            }
            while run.island.progress(goal) < goal.count {
                let slot = run
                    .island
                    .lots
                    .iter()
                    .position(|p| p.is_some_and(|p| p.kind == goal.kind && p.tier < goal.tier));
                run.build(goal.kind, slot);
            }
        }
        run.island.position = (terrain::HALL_DOOR + Vec2::new(1.5, 2.5)).to_array();
        for earlier in 0..index {
            if book.islands[earlier].is_none() {
                let mut done = Island::new(Level::at(earlier));
                done.completed = true;
                book.islands[earlier] = Some(done);
            }
        }
        book.islands[index] = Some(run.island);
        book.last = Some(index);
    }
    std::fs::create_dir_all(&dir).unwrap();
    let path = std::path::Path::new(&dir).join("nililane-abenteuer-v1.json");
    std::fs::write(path, serde_json::to_string(&book).unwrap()).unwrap();
}

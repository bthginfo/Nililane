mod levels;
mod simulation;
pub(crate) mod terrain;
#[cfg(test)]
mod tests;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io};

use crate::{
    campaign::{CORE_HARBORS, Region},
    controls::{self, Controls},
    game::{Action, Game, Panel, View},
    save::Campaign,
    ui::{self, Hud},
};

pub(crate) use levels::{LEVELS, Level};
pub(crate) use simulation::{Build, Good, Island, Property, Quest, Task};
pub(crate) use terrain::Terrain;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Deposit(usize),
    Plot(usize),
    Hall,
    Shrine,
    Relic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Enter,
    Home,
    Choose(usize),
    Area(Region),
    Start,
    Atlas,
    Tutorial,
    Close,
    Pause,
    Help,
    Map,
    Labor,
    Inspect(Target),
    SetRoute(Target),
    ClearRoute,
    Use(Target),
    Build(Build),
    Demolish,
    Remove,
    Group(u8),
    Page(u8),
    Assign(Task, bool),
    Work(usize),
    Next,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Atlas,
    Brief,
    Walk,
    Lot(usize),
    Demolish(usize),
    Resource(usize),
    Labor,
    Mission,
    Map,
    Pause,
    Help,
    Finish,
}

#[derive(Serialize, Deserialize)]
struct Book {
    version: u8,
    last: Option<usize>,
    islands: Vec<Option<Island>>,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            version: 1,
            last: None,
            islands: vec![None; LEVELS],
        }
    }
}

#[derive(Resource)]
pub struct Adventure {
    book: Book,
    practice: Option<Island>,
    pub screen: Screen,
    pub selected: usize,
    pub terrain: Option<Terrain>,
    pub target: Option<Target>,
    pub waypoint: Option<Target>,
    pub group: u8,
    pub page: u8,
    pub map_zoom: f32,
    pub scene_change: Option<(usize, f32)>,
    pub heading: f32,
    pub gait: f32,
    pub work_swing: f32,
    pub walkthrough: bool,
    work_clock: f32,
    hand_clock: f32,
    save_clock: f32,
    pointer: Option<ui::WalkPointer>,
    pub moved: bool,
}

impl Default for Adventure {
    fn default() -> Self {
        let path = crate::save::path().with_file_name("nililane-abenteuer-v1.json");
        let mut book = fs::File::open(path)
            .ok()
            .and_then(|file| serde_json::from_reader::<_, Book>(file).ok())
            .filter(|book| book.version == 1)
            .unwrap_or_default();
        book.islands.resize(LEVELS, None);
        for (index, stored) in book.islands.iter_mut().enumerate() {
            let Some(island) = stored else { continue };
            let level = Level::at(index);
            let terrain = Terrain::new(level);
            if island.index != index
                || island.lots.len() != level.plots()
                || !terrain.accessible(Vec2::from_array(island.position))
                || !island.phase.is_finite()
                || island.lots.iter().flatten().any(|property| {
                    !(1..=crate::model::MAX_LEVEL).contains(&property.tier)
                        || !Build::all(level).contains(&property.kind)
                })
            {
                *stored = None;
            } else {
                let max = usize::from(island.staff());
                island.workers.retain(|job| match job {
                    Task::Gather(i) => terrain.deposits.get(*i).is_some(),
                    Task::Produce(i) => island
                        .lots
                        .get(*i)
                        .copied()
                        .flatten()
                        .is_some_and(|p| p.kind.recipe(p.tier).is_some()),
                });
                island.workers.truncate(max);
            }
        }
        book.last = book
            .last
            .filter(|&i| i < LEVELS && book.islands[i].is_some());
        let selected = book.last.unwrap_or(0);
        Self {
            book,
            practice: None,
            screen: Screen::Atlas,
            selected,
            terrain: None,
            target: None,
            waypoint: None,
            group: 0,
            page: 0,
            map_zoom: 0.0,
            scene_change: None,
            heading: 0.0,
            gait: 0.0,
            work_swing: 0.0,
            walkthrough: true,
            work_clock: 0.0,
            hand_clock: 0.0,
            save_clock: 0.0,
            pointer: None,
            moved: false,
        }
    }
}

impl Adventure {
    pub fn active(&self) -> Option<&Island> {
        self.practice.as_ref().or_else(|| {
            self.book
                .islands
                .get(self.book.last?)
                .and_then(Option::as_ref)
        })
    }

    fn active_mut(&mut self) -> Option<&mut Island> {
        self.practice.as_mut().or_else(|| {
            self.book
                .islands
                .get_mut(self.book.last?)
                .and_then(Option::as_mut)
        })
    }

    pub fn continuing(&self, index: usize) -> bool {
        self.book.islands.get(index).is_some_and(Option::is_some)
    }

    pub fn completed(&self, index: usize) -> bool {
        self.book
            .islands
            .get(index)
            .and_then(Option::as_ref)
            .is_some_and(|island| island.completed)
    }

    pub fn unlocked(&self, index: usize) -> bool {
        index < LEVELS && (index == 0 || self.completed(index - 1))
    }

    pub fn practice(&self) -> bool {
        self.practice.is_some()
    }

    pub fn open(&mut self, index: usize, practice: bool) {
        let level = Level::at(index);
        self.practice = practice.then(|| Island::new(level));
        if !practice {
            self.book.last = Some(index);
            self.book.islands[index].get_or_insert_with(|| Island::new(level));
        }
        self.selected = index;
        self.terrain = Some(Terrain::new(level));
        self.target = None;
        self.waypoint = None;
        self.map_zoom = 0.0;
        self.heading = 0.0;
        self.work_clock = 0.0;
        self.hand_clock = 0.0;
        self.work_swing = 0.0;
        self.scene_change = None;
        self.walkthrough = index == 0 || practice;
    }

    fn save(&mut self, game: &mut Game) {
        self.save_clock = 0.0;
        if self.practice() {
            return;
        }
        let result = (|| -> io::Result<()> {
            let path = crate::save::path().with_file_name("nililane-abenteuer-v1.json");
            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }
            let temporary = path.with_extension("tmp");
            let file = fs::File::create(&temporary)?;
            serde_json::to_writer(&file, &self.book)?;
            file.sync_all()?;
            fs::rename(temporary, path)
        })();
        if let Err(error) = result {
            warn!("Nililane Abenteuer: {error}");
            game.adventure_notice("Die Insel konnte nicht gespeichert werden.", false);
        }
    }

    pub fn restore_rewards(&self, campaign: &mut Campaign) -> bool {
        let mut changed = false;
        for (index, island) in self.book.islands.iter().enumerate() {
            if island.as_ref().is_some_and(|island| island.completed) {
                changed |= campaign.reward_adventure(index);
            }
        }
        changed
    }

    pub fn target_position(&self, target: Target) -> Option<Vec2> {
        let terrain = self.terrain.as_ref()?;
        match target {
            Target::Deposit(id) => terrain.deposits.get(id).map(|d| d.at),
            Target::Plot(id) => terrain.plots.get(id).copied(),
            Target::Hall => Some(Vec2::ZERO),
            Target::Shrine => Some(terrain.shrine),
            Target::Relic => Some(terrain.relic),
        }
    }

    pub fn close_enough(&self, target: Target) -> bool {
        let Some(at) = self.target_position(target) else {
            return false;
        };
        self.active()
            .is_some_and(|island| Vec2::from_array(island.position).distance(at) < reach(target))
    }

    fn candidate(&self) -> Option<Target> {
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        if let Some(preferred) = self
            .waypoint
            .or_else(|| self.tutorial().map(|(_, _, target)| target))
            && self.close_enough(preferred)
        {
            return Some(preferred);
        }
        let here = Vec2::from_array(island.position);
        let sites = (0..terrain.deposits.len())
            .map(Target::Deposit)
            .chain((0..terrain.plots.len()).map(Target::Plot))
            .chain([Target::Hall])
            .chain(
                (terrain.level.plan().is_some() && island.quest != Quest::Finished)
                    .then_some(Target::Shrine),
            )
            .chain((island.quest == Quest::Searching).then_some(Target::Relic));
        sites
            .filter_map(|target| {
                let gap = self.target_position(target)?.distance(here);
                (gap <= reach(target)).then_some((gap, target))
            })
            .min_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, target)| target)
    }

    fn pick(&self, camera: &Camera, transform: &GlobalTransform, point: Vec2) -> Option<Target> {
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        let sites = (0..terrain.deposits.len())
            .map(Target::Deposit)
            .chain((0..terrain.plots.len()).map(Target::Plot))
            .chain([Target::Hall])
            .chain(
                (terrain.level.plan().is_some() && island.quest != Quest::Finished)
                    .then_some(Target::Shrine),
            )
            .chain((island.quest == Quest::Searching).then_some(Target::Relic));
        sites
            .filter_map(|target| {
                let p = self.target_position(target)?;
                let world = crate::world::adventure_position(terrain, p);
                let height = match target {
                    Target::Plot(lot) => {
                        island.lots[lot].map_or(0.13, |p| 1.8 + f32::from(p.tier) * 0.21)
                    }
                    Target::Hall => 3.05,
                    Target::Shrine => 2.9,
                    Target::Relic => 1.15,
                    Target::Deposit(id) => {
                        if terrain.deposits[id].kind == Good::Wood {
                            1.8
                        } else {
                            0.62
                        }
                    }
                };
                let base = camera.world_to_viewport(transform, world).ok()?;
                let top = camera
                    .world_to_viewport(transform, world + Vec3::Y * height)
                    .ok()?;
                let vertical = top - base;
                let fraction = ((point - base).dot(vertical) / vertical.length_squared().max(1.0))
                    .clamp(0.0, 1.0);
                let distance = (base + vertical * fraction).distance(point);
                let margin = (camera.logical_viewport_size()?.x * 0.036).clamp(19.0, 48.0);
                (distance < margin).then_some((distance, target))
            })
            .min_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, target)| target)
    }

    fn guided_plan(&self) -> Option<Build> {
        let (terrain, island) = self.terrain.as_ref().zip(self.active())?;
        if !self.walkthrough || island.completed {
            return None;
        }
        let level = terrain.level;
        [
            Build::Harbor(level.home()),
            Build::Sawmill,
            level.requirements()[2].kind,
        ]
        .into_iter()
        .find(|&kind| {
            island
                .lots
                .iter()
                .flatten()
                .all(|property| property.kind != kind)
        })
    }

    pub fn tutorial(&self) -> Option<(&'static str, &'static str, Target)> {
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        if terrain.level.stage != 0 || !self.walkthrough || island.completed {
            return None;
        }
        let home = Build::Harbor(terrain.level.home());
        let has_home = island.lots.iter().flatten().any(|p| p.kind == home);
        let woodwork = island
            .lots
            .iter()
            .position(|p| p.is_some_and(|p| p.kind == Build::Sawmill));
        if has_home && let Some(lot) = woodwork {
            if island.amount(Good::Food) == 0 && island.amount(Good::Boards) < 8 {
                return Some((
                    "adventure_food",
                    "Sammle etwas Essen für die Holzwerkstatt.",
                    Target::Deposit(2),
                ));
            }
            if island.assignment(Task::Produce(lot)) == 0
                && island.workers.len() >= usize::from(island.staff())
            {
                return Some((
                    "adventure_labor",
                    "Öffne „Bürger“ und mach einen Bürger für das Sägewerk frei.",
                    Target::Plot(lot),
                ));
            }
            if island.assignment(Task::Produce(lot)) == 0 {
                return Some((
                    "age_adult",
                    "Öffne das Sägewerk und weise einen Bürger zu.",
                    Target::Plot(lot),
                ));
            }
            if island.amount(Good::Wood) < 3 && island.amount(Good::Boards) < 8 {
                return Some((
                    "adventure_wood",
                    "Holz hält die Werkstatt in Betrieb.",
                    Target::Deposit(0),
                ));
            }
            let goal = terrain.level.requirements()[2].kind;
            if !island.lots.iter().flatten().any(|p| p.kind == goal) {
                let plot = island
                    .lots
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.is_none())
                    .map(|(i, _)| i)?;
                if island.amount(Good::Wood) < 5 && island.amount(Good::Boards) >= 8 {
                    return Some((
                        "adventure_wood",
                        "Sammle Holz für das letzte Gebäude.",
                        Target::Deposit(0),
                    ));
                }
                if island.amount(Good::Stone) < 5 {
                    return Some((
                        "adventure_stone",
                        "Sammle Stein für das letzte Gebäude.",
                        Target::Deposit(1),
                    ));
                }
                return Some((
                    "hammer",
                    "Das letzte Gebäude steht in deinen Aufträgen.",
                    Target::Plot(plot),
                ));
            }
            return Some((
                "adventure_boards",
                "Halte Holz und Essen für acht Bretter bereit.",
                Target::Plot(lot),
            ));
        }
        let plan = if has_home { Build::Sawmill } else { home };
        let costs = plan.cost(1);
        if costs
            .iter()
            .any(|&(kind, n)| kind == Good::Wood && island.amount(kind) < n)
        {
            return Some((
                "adventure_wood",
                "Geh zum Holz und sammle es ein.",
                Target::Deposit(0),
            ));
        }
        if costs
            .iter()
            .any(|&(kind, n)| kind == Good::Stone && island.amount(kind) < n)
        {
            return Some((
                "adventure_stone",
                "Geh zum Felsen und sammle Stein.",
                Target::Deposit(1),
            ));
        }
        let lot = island
            .lots
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_none())
            .min_by(|(a, _), (b, _)| {
                terrain.plots[*a]
                    .length_squared()
                    .total_cmp(&terrain.plots[*b].length_squared())
            })
            .map(|(i, _)| i)?;
        let hint = if has_home {
            "Baue ein Sägewerk auf dem markierten Platz."
        } else {
            "Baue das erste Wohnhaus auf dem markierten Platz."
        };
        Some(("hammer", hint, Target::Plot(lot)))
    }
}

fn reach(target: Target) -> f32 {
    match target {
        Target::Plot(_) | Target::Hall | Target::Shrine => 2.9,
        _ => 2.4,
    }
}

fn close(game: &mut Game, adventure: &mut Adventure, screen: Screen) {
    adventure.screen = screen;
    adventure.moved = false;
    adventure.pointer = None;
    game.walk_axis = Vec2::ZERO;
    adventure.group = 0;
    adventure.page = 0;
    game.focus = None;
    game.dirty = true;
}

fn act(command: Command, adventure: &mut Adventure, game: &mut Game, save: &mut Campaign) {
    match command {
        Command::Enter => {}
        Command::Atlas => {
            adventure.save(game);
            adventure.practice = None;
            adventure.terrain = None;
            adventure.selected = adventure.book.last.unwrap_or(adventure.selected);
            close(game, adventure, Screen::Atlas);
        }
        Command::Home => {
            adventure.save(game);
            adventure.practice = None;
            adventure.terrain = None;
            close(game, adventure, Screen::Atlas);
            game.transition(View::Title, Panel::None);
        }
        Command::Choose(index) => {
            adventure.selected = index.min(LEVELS - 1);
            game.focus = Some(Action::Adventure(
                if adventure.unlocked(adventure.selected) {
                    Command::Start
                } else {
                    command
                },
            ));
            game.dirty = true;
        }
        Command::Area(region) => {
            adventure.selected = (0..CORE_HARBORS)
                .map(|n| region as usize * CORE_HARBORS + n)
                .find(|&n| !adventure.completed(n))
                .unwrap_or((region as usize + 1) * CORE_HARBORS - 1);
            game.focus = Some(Action::Adventure(Command::Choose(adventure.selected)));
            game.dirty = true;
        }
        Command::Start | Command::Tutorial | Command::Next => {
            let practice = command == Command::Tutorial;
            let index = match command {
                Command::Next => adventure.selected + 1,
                Command::Tutorial => 0,
                _ => adventure.selected,
            };
            if adventure.unlocked(index) || practice {
                let brief = !adventure.continuing(index) && index % CORE_HARBORS != 0;
                adventure.open(index, practice);
                close(
                    game,
                    adventure,
                    if brief { Screen::Brief } else { Screen::Walk },
                );
                adventure.save(game);
            }
        }
        Command::Close => {
            let next = match adventure.screen {
                Screen::Walk => Screen::Pause,
                Screen::Help => Screen::Pause,
                Screen::Demolish(lot) => Screen::Lot(lot),
                Screen::Finish => {
                    act(Command::Atlas, adventure, game, save);
                    return;
                }
                Screen::Atlas => {
                    act(Command::Home, adventure, game, save);
                    return;
                }
                _ => Screen::Walk,
            };
            close(game, adventure, next);
        }
        Command::Pause => close(game, adventure, Screen::Pause),
        Command::Help => close(game, adventure, Screen::Help),
        Command::Map => close(game, adventure, Screen::Map),
        Command::Labor => close(game, adventure, Screen::Labor),
        Command::SetRoute(target) => {
            if adventure.target_position(target).is_some() {
                adventure.waypoint = Some(target);
                close(game, adventure, Screen::Walk);
            }
        }
        Command::ClearRoute => {
            adventure.waypoint = None;
            game.dirty = true;
        }
        Command::Inspect(target) => {
            if target != Target::Hall && !adventure.close_enough(target) {
                return;
            }
            if adventure.waypoint == Some(target)
                && matches!(target, Target::Plot(_) | Target::Deposit(_))
            {
                adventure.waypoint = None;
            }
            match target {
                Target::Plot(lot) => {
                    let guide = adventure
                        .active()
                        .filter(|island| island.lots[lot].is_none())
                        .and_then(|_| adventure.guided_plan());
                    close(game, adventure, Screen::Lot(lot));
                    if let Some(guide) = guide {
                        adventure.group = guide.page();
                        if let Some(terrain) = &adventure.terrain {
                            adventure.page = Build::all(terrain.level)
                                .into_iter()
                                .filter(|plan| plan.page() == guide.page())
                                .position(|plan| plan == guide)
                                .map_or(0, |i| (i / 6) as u8);
                        }
                    }
                }
                Target::Deposit(id) => close(game, adventure, Screen::Resource(id)),
                Target::Hall | Target::Shrine | Target::Relic => {
                    adventure.target = Some(target);
                    close(game, adventure, Screen::Mission);
                }
            }
        }
        Command::Use(target) => {
            if !adventure.close_enough(target) {
                return;
            }
            let Some(terrain) = adventure.terrain.as_ref() else {
                return;
            };
            let deposit = match target {
                Target::Deposit(i) => terrain.deposits.get(i).map(|d| d.kind),
                _ => None,
            };
            let level = terrain.level;
            if deposit.is_some() && adventure.hand_clock > 0.0 {
                return;
            }
            let at = adventure.target_position(target);
            let Some(island) = adventure.active_mut() else {
                return;
            };
            match target {
                Target::Deposit(_) if let Some(kind) = deposit => {
                    let focus = at.map_or(Vec2::ZERO, |at| at - Vec2::from_array(island.position));
                    island.gather(kind);
                    if focus.length_squared() > 0.01 {
                        adventure.heading = focus.x.atan2(focus.y);
                    }
                    adventure.hand_clock = 0.28;
                    adventure.work_swing = 0.46;
                    game.harvest(kind, level.region);
                    if adventure.waypoint == Some(target) {
                        adventure.waypoint = None;
                    }
                }
                Target::Hall | Target::Plot(_) => {
                    act(Command::Inspect(target), adventure, game, save);
                    return;
                }
                Target::Shrine | Target::Relic => {
                    if let Some(message) = advance_quest(island, level, target) {
                        adventure.waypoint = match island.quest {
                            Quest::Searching => Some(Target::Relic),
                            Quest::Carrying | Quest::Offering => Some(Target::Shrine),
                            _ => None,
                        };
                        close(game, adventure, Screen::Walk);
                        game.adventure_notice(message, true);
                    } else {
                        act(Command::Inspect(target), adventure, game, save);
                        return;
                    }
                }
                _ => return,
            }
            game.dirty = true;
            adventure.save(game);
        }
        Command::Build(kind) => {
            let Screen::Lot(lot) = adventure.screen else {
                return;
            };
            let Some(terrain) = adventure.terrain.as_ref() else {
                return;
            };
            let level = terrain.level;
            let plot = terrain.plots[lot];
            if adventure.close_enough(Target::Plot(lot))
                && adventure
                    .active_mut()
                    .is_some_and(|island| island.build(level, lot, kind))
            {
                if let Some(island) = adventure.active_mut() {
                    let offset = Vec2::from_array(island.position) - plot;
                    if Terrain::inside_building(offset) {
                        island.position = (plot + Terrain::just_outside(offset)).to_array();
                    }
                }
                adventure.scene_change = Some((lot, game.clock));
                game.adventure_built(kind.name(level.region));
                close(game, adventure, Screen::Walk);
                adventure.save(game);
            }
        }
        Command::Group(group) => {
            adventure.group = group.min(2);
            adventure.page = 0;
            game.dirty = true;
        }
        Command::Page(page) => {
            adventure.page = page;
            game.dirty = true;
        }
        Command::Demolish => {
            if let Screen::Lot(lot) = adventure.screen {
                close(game, adventure, Screen::Demolish(lot));
            }
        }
        Command::Remove => {
            let Screen::Demolish(lot) = adventure.screen else {
                return;
            };
            if let Some(island) = adventure.active_mut()
                && let Some(property) = island.lots[lot].take()
            {
                for (good, n) in property.kind.cost(1) {
                    island.add(good, n / 2);
                }
                island.workers.retain(|job| *job != Task::Produce(lot));
                island.workers.truncate(usize::from(island.staff()));
                adventure.save(game);
                close(game, adventure, Screen::Walk);
            }
        }
        Command::Assign(task, assign) => {
            if let Some(layout) = adventure.terrain.clone()
                && let Some(island) = adventure.active_mut()
            {
                let done = if assign {
                    island.assign(task, &layout)
                } else {
                    island.unassign(task)
                };
                if done {
                    game.adventure_notice("Bürgerauftrag geändert", true);
                    adventure.save(game);
                }
            }
            game.dirty = true;
        }
        Command::Work(lot) => {
            if adventure.close_enough(Target::Plot(lot))
                && adventure.hand_clock <= 0.0
                && adventure
                    .active_mut()
                    .is_some_and(|island| island.produce(lot))
            {
                adventure.hand_clock = 1.45;
                adventure.work_swing = 0.46;
                game.adventure_notice("Waren eingelagert", true);
                game.dirty = true;
                adventure.save(game);
            }
        }
    }
    check_victory(adventure, game, save);
}

fn advance_quest(island: &mut Island, level: Level, target: Target) -> Option<&'static str> {
    level.plan()?;
    match (target, island.quest) {
        (Target::Shrine, Quest::Unheard) if level.expedition() => {
            island.quest = Quest::Searching;
            Some("Der Weg zum Fundstück ist markiert.")
        }
        (Target::Shrine, Quest::Unheard) => {
            island.quest = Quest::Offering;
            Some("Die Lieferung steht in deinen Aufträgen.")
        }
        (Target::Relic, Quest::Searching) => {
            island.quest = Quest::Carrying;
            Some(level.relic)
        }
        (Target::Shrine, Quest::Carrying) if level.offering().is_some() => {
            island.quest = Quest::Offering;
            Some("Fundstück übergeben. Jetzt fehlen noch die Waren.")
        }
        (Target::Shrine, Quest::Carrying) => {
            island.quest = Quest::Finished;
            Some("Bauplan freigeschaltet")
        }
        (Target::Shrine, Quest::Offering) if island.turn_in(level) => {
            Some("Lieferung abgegeben. Bauplan freigeschaltet.")
        }
        _ => None,
    }
}

fn check_victory(adventure: &mut Adventure, game: &mut Game, save: &mut Campaign) {
    if let Some(terrain) = adventure.terrain.as_ref() {
        let level = terrain.level;
        if let Some(island) = adventure.active_mut()
            && !island.completed
            && island.victory(level)
        {
            island.completed = true;
            adventure.save(game);
            if !adventure.practice() {
                save.reward_adventure(level.index);
                game.save(save);
            }
            close(game, adventure, Screen::Finish);
            game.adventure_complete();
        }
    }
}

pub(crate) fn first(a: &Adventure) -> Option<Action> {
    let initial = match a.screen {
        Screen::Atlas if !a.unlocked(a.selected) => Command::Choose(a.selected),
        Screen::Lot(lot) => {
            let (terrain, island) = a.terrain.as_ref().zip(a.active())?;
            if let Some(property) = island.lots[lot] {
                let job = Task::Produce(lot);
                if a.walkthrough
                    && property.kind == Build::Sawmill
                    && island.assignment(job) == 0
                    && island.workers.len() < usize::from(island.staff())
                {
                    Command::Assign(job, true)
                } else {
                    Command::Close
                }
            } else {
                let ready = |plan: Build| {
                    !island.locked(terrain.level, plan, 1) && island.ready(&plan.cost(1))
                };
                let plans: Vec<_> = Build::all(terrain.level)
                    .into_iter()
                    .filter(|plan| plan.page() == a.group)
                    .skip(usize::from(a.page) * 6)
                    .take(6)
                    .collect();
                a.guided_plan()
                    .filter(|plan| plans.contains(plan) && ready(*plan))
                    .or_else(|| plans.into_iter().find(|&plan| ready(plan)))
                    .map_or(Command::Close, Command::Build)
            }
        }
        Screen::Atlas => Command::Start,
        Screen::Walk => return None,
        Screen::Brief => Command::Close,
        Screen::Demolish(_) | Screen::Resource(_) | Screen::Labor | Screen::Map => Command::Close,
        Screen::Mission => match a.target {
            Some(target @ (Target::Shrine | Target::Relic))
                if a.close_enough(target)
                    && a.terrain
                        .as_ref()
                        .zip(a.active())
                        .is_some_and(|(terrain, island)| {
                            island.quest != Quest::Offering
                                || terrain
                                    .level
                                    .offering()
                                    .is_some_and(|offering| island.ready(&[offering]))
                        }) =>
            {
                Command::Use(target)
            }
            _ => Command::Close,
        },
        Screen::Pause | Screen::Help => Command::Close,
        Screen::Finish if a.practice() || a.selected + 1 == LEVELS => Command::Close,
        Screen::Finish => Command::Next,
    };
    Some(Action::Adventure(initial))
}

#[allow(clippy::too_many_arguments)]
pub fn step(
    adventure: &mut Adventure,
    game: &mut Game,
    save: &mut Campaign,
    controls: &Controls,
    buttons: &ui::FocusButtons,
    pressed: Option<Action>,
    hud: &Hud,
    window: &Window,
    view: Option<(&Camera, &GlobalTransform)>,
    mouse: &ButtonInput<MouseButton>,
    touches: &Touches,
    pad: &ui::WalkPad,
    dt: f32,
) {
    let dt = dt.min(0.38);
    let was = adventure.screen;
    if window.focused {
        process_work(adventure, game, save, dt);
    }
    if was != adventure.screen || !window.focused {
        return;
    }
    if let Some(action) = pressed {
        match action {
            Action::Adventure(command) => act(command, adventure, game, save),
            _ => crate::game::apply(action, game, save),
        }
        return;
    }
    if game.panel != Panel::None || adventure.screen != Screen::Walk {
        if controls.got(controls::B) {
            if game.panel != Panel::None {
                game.show(Panel::None);
            } else {
                act(Command::Close, adventure, game, save);
            }
        } else {
            if let Some(dir) = controls.direction {
                game.focus = ui::move_focus(game.focus.or(hud.first), dir, buttons).or(hud.first);
            }
            if controls.got(controls::A)
                && let Some(action) = game.focus.or(hud.first)
            {
                match action {
                    Action::Adventure(command) => act(command, adventure, game, save),
                    _ => crate::game::apply(action, game, save),
                }
            }
        }
        return;
    }
    if controls.got(controls::START) || controls.got(controls::B) {
        act(Command::Pause, adventure, game, save);
        return;
    }
    if controls.got(controls::X) {
        act(Command::Map, adventure, game, save);
        return;
    }
    if controls.got(controls::Y) {
        act(Command::Labor, adventure, game, save);
        return;
    }
    if controls.got(controls::RB) || controls.got(controls::VIEW) {
        adventure.target = Some(Target::Hall);
        close(game, adventure, Screen::Mission);
        return;
    }
    adventure.map_zoom = (adventure.map_zoom + controls.zoom * dt * 0.95).clamp(-1.2, 0.64);
    if controls.got(controls::LB) {
        adventure.map_zoom = 0.0;
    }
    let touch = ui::walk_input(pad, window, touches, mouse, &mut adventure.pointer);
    let axis = (controls.walk + touch).clamp_length_max(1.0);
    game.walk_axis = axis;
    let delta = Vec2::new(axis.x + axis.y * 0.115, axis.y - axis.x * 0.115) * 5.4 * dt;
    adventure.moved = false;
    if axis.length_squared() > 0.02
        && let Some(terrain) = adventure.terrain.as_ref()
        && let Some(island) = adventure.active()
    {
        let start = Vec2::from_array(island.position);

        let next = terrain.walk(start, delta, &island.lots);
        let distance = start.distance(next);
        if distance > 0.001 {
            adventure.active_mut().unwrap().position = next.to_array();
            adventure.moved = true;
            adventure.heading = delta.x.atan2(delta.y);
            adventure.gait += distance * 5.9;
        }
    }
    let target = adventure.candidate();
    if adventure.target != target {
        adventure.target = target;
        game.dirty = true;
    }
    if controls.got(controls::A)
        && let Some(target) = target
    {
        act(Command::Use(target), adventure, game, save);
    } else if controls.got(controls::R3)
        && let Some(target) = target
    {
        act(Command::Inspect(target), adventure, game, save);
    } else if adventure.pointer.is_none()
        && !buttons
            .iter()
            .any(|(_, _, _, state)| *state != Interaction::None)
        && let Some((camera, transform)) = view
    {
        let tap = touches
            .iter_just_pressed()
            .next()
            .map(|touch| touch.position())
            .or_else(|| {
                mouse
                    .just_pressed(MouseButton::Left)
                    .then(|| window.cursor_position())
                    .flatten()
            });
        if let Some(tap) = tap
            && let Some(target) = adventure.pick(camera, transform, tap)
        {
            let command = if adventure.close_enough(target) {
                Command::Use(target)
            } else {
                Command::SetRoute(target)
            };
            act(command, adventure, game, save);
        }
    }
}

fn process_work(adventure: &mut Adventure, game: &mut Game, save: &mut Campaign, dt: f32) {
    if !matches!(
        adventure.screen,
        Screen::Walk
            | Screen::Lot(_)
            | Screen::Resource(_)
            | Screen::Labor
            | Screen::Mission
            | Screen::Map
    ) || game.panel != Panel::None
    {
        return;
    }
    let dt = dt.min(0.18);
    adventure.hand_clock = (adventure.hand_clock - dt).max(0.0);
    adventure.work_swing = (adventure.work_swing - dt).max(0.0);
    adventure.work_clock += dt;
    adventure.save_clock += dt;
    let Some(terrain) = adventure.terrain.as_ref() else {
        return;
    };
    let island = match adventure.practice.as_mut() {
        Some(island) => Some(island),
        None => adventure
            .book
            .last
            .and_then(|index| adventure.book.islands[index].as_mut()),
    };
    let Some(island) = island else {
        return;
    };
    let night = |phase: f32| !(0.18..=0.82).contains(&phase);
    let was_night = night(island.phase);
    island.phase = (island.phase + dt / 208.0).rem_euclid(1.0);
    game.dirty |= was_night != night(island.phase);
    if adventure.work_clock >= 5.7 {
        adventure.work_clock -= 5.7;
        let before = island.stores;
        island.tick_work(terrain);
        if before != island.stores {
            game.dirty = true;
            check_victory(adventure, game, save);
        }
    }
    if adventure.save_clock >= 8.0 {
        adventure.save(game);
    }
}

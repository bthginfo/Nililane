mod advice;
mod construction;
mod encounters;
mod levels;
mod logistics;
mod port_trade;
mod simulation;
pub(crate) mod terrain;
#[cfg(test)]
mod tests;
mod twists;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io};

use crate::{
    campaign::{CORE_HARBORS, Region},
    controls::{self, Controls},
    game::{Action, Cue, Game, Panel, View},
    save::Campaign,
    ui::{self, Hud},
};

pub(crate) use advice::Advice;
pub(crate) use construction::{BuildingSite, MAX_BUILD_CREW};
pub(crate) use encounters::{Event, Status as EncounterStatus};
pub(crate) use levels::{LEVELS, Level};
pub(crate) use logistics::{DroppedLoad, RESOURCE_REACH};
pub(crate) use port_trade::{Export, IMPORT_PRICE, IMPORT_UNITS, PLAN_PRICE, monument_name};
pub(crate) use simulation::{
    Build, Good, HALL_STORAGE, Island, LANDMARK_REACH, Property, Quest, Task, landmark_bonus,
};
pub(crate) use terrain::Terrain;
pub(crate) use twists::Twist;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Deposit(usize),
    Dropped(usize),
    Encounter(usize),
    EncounterSource(usize),
    TradeHarbor,
    Plot(usize),
    Hall,
    Shrine,
    Relic,
    Outpost,
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
    Summary,
    Help,
    Map,
    Storage,
    Labor,
    Inspect(Target),
    SetRoute(Target),
    ClearRoute,
    CameraView,
    Use(Target),
    Deliveries,
    PackOrder,
    SendCourier,
    ReturnOrder,
    ClearStock(Good),
    StoreOverflow(Good),
    Build(Build),
    TakeMaterial(usize),
    BuildCrew(usize, i8),
    CancelBuild(usize),
    Demolish,
    Remove,
    Group(u8),
    Page(u8),
    Assign(Task, bool),
    Work(usize),
    Masterwork(usize, bool),
    Encounters,
    ReturnEncounter,
    Trading,
    PackExport(Export),
    UnpackExport,
    SellExport,
    BuyPlan,
    BuyImports,
    /// Buys one entry of the harbor market.
    Buy(usize),
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
    Storage,
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
    pub scene_changes: Vec<(usize, f32)>,
    pub heading: f32,
    pub gait: f32,
    pub work_swing: f32,
    pub harvesting: Option<Harvest>,
    pub building: Option<usize>,
    pub walkthrough: bool,
    hand_clock: f32,
    save_clock: f32,
    pointer: Option<ui::WalkPointer>,
    supply_for: Option<usize>,
    pub moved: bool,
    /// Seconds until the summary opens after the last island goal was met.
    pub finale: Option<f32>,
    milestones: Option<Milestones>,
    /// Islands the campaign save remembers as finished, one bit each.
    cleared: std::sync::atomic::AtomicU64,
}

const _: () = assert!(LEVELS <= 64);

#[derive(Clone, Copy)]
pub struct Harvest {
    pub site: usize,
    pub left: f32,
    pub duration: f32,
}

const BOOK_FILE: &str = "nililane-abenteuer-v1.json";
/// The beat between the completion fanfare and the summary.
pub(crate) const FINALE_SECONDS: f32 = 1.7;

fn book_path() -> std::path::PathBuf {
    crate::save::path().with_file_name(BOOK_FILE)
}

impl Book {
    /// Reads every island on its own, so one damaged entry cannot reset the whole campaign.
    /// The flag reports whether anything had to be thrown away.
    fn parse(bytes: &[u8]) -> (Self, bool) {
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(bytes) else {
            return (Self::default(), true);
        };
        if value.get("version").and_then(serde_json::Value::as_u64) != Some(1) {
            return (Self::default(), true);
        }
        let mut book = Self {
            last: value
                .get("last")
                .and_then(serde_json::Value::as_u64)
                .and_then(|last| usize::try_from(last).ok()),
            ..Self::default()
        };
        let Some(entries) = value.get("islands").and_then(serde_json::Value::as_array) else {
            return (book, true);
        };
        let mut damaged = false;
        for (index, entry) in entries.iter().enumerate() {
            if entry.is_null() {
                continue;
            }
            match Island::deserialize(entry) {
                Ok(island) if index < LEVELS => book.islands[index] = Some(island),
                _ => damaged = true,
            }
        }
        damaged |= book.repair();
        (book, damaged)
    }

    /// Fixes what can be fixed and drops islands that no longer fit their map.
    fn repair(&mut self) -> bool {
        let mut damaged = false;
        self.islands.resize(LEVELS, None);
        for (index, stored) in self.islands.iter_mut().enumerate() {
            let Some(island) = stored else { continue };
            let level = Level::at(index);
            let terrain = Terrain::new(level);
            if island.index != index
                || !(level.inland_plots()..=level.plots()).contains(&island.lots.len())
                || island.lots.iter().flatten().any(|property| {
                    !(1..=crate::model::MAX_LEVEL).contains(&property.tier)
                        || !Build::all(level).contains(&property.kind)
                })
            {
                *stored = None;
                damaged = true;
                continue;
            }
            if !terrain.accessible(Vec2::from_array(island.position)) {
                island.position = terrain::HALL_DOOR.to_array();
            }
            if !island.phase.is_finite() {
                island.phase = 0.36;
            }
            island.lots.resize(level.plots(), None);
            let max = usize::from(island.staff());
            island.validate_workers(&terrain, max);
            island.prepare_logistics(&terrain);
            island.prepare_production(&terrain);
        }
        self.last = self
            .last
            .filter(|&i| i < LEVELS && self.islands[i].is_some());
        damaged
    }

    fn load(path: &std::path::Path) -> Self {
        let Ok(bytes) = fs::read(path) else {
            return Self::default();
        };
        let (book, damaged) = Self::parse(&bytes);
        if damaged {
            // Keep the unreadable original, the next save would overwrite it.
            let backup = path.with_extension("bak");
            match fs::write(&backup, &bytes) {
                Ok(()) => warn!("Nililane Abenteuer: beschädigte Inseln, Kopie in {backup:?}"),
                Err(error) => warn!("Nililane Abenteuer: {error}"),
            }
        }
        book
    }
}

impl Default for Adventure {
    fn default() -> Self {
        let book = Book::load(&book_path());
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
            scene_changes: Vec::new(),
            heading: 0.0,
            gait: 0.0,
            work_swing: 0.0,
            harvesting: None,
            building: None,
            walkthrough: true,
            hand_clock: 0.0,
            save_clock: 0.0,
            pointer: None,
            supply_for: None,
            moved: false,
            finale: None,
            milestones: None,
            cleared: Default::default(),
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
        index < LEVELS && (index == 0 || self.completed(index - 1) || self.rewarded(index - 1))
    }

    fn rewarded(&self, index: usize) -> bool {
        let cleared = self.cleared.load(std::sync::atomic::Ordering::Relaxed);
        index < LEVELS && cleared & (1 << index) != 0
    }

    pub fn practice(&self) -> bool {
        self.practice.is_some()
    }

    pub fn open(&mut self, index: usize, practice: bool) {
        let level = Level::at(index);
        let terrain = Terrain::new(level);
        self.practice = practice.then(|| Island::new(level));
        if !practice {
            self.book.last = Some(index);
            self.book.islands[index].get_or_insert_with(|| Island::new(level));
        }
        if let Some(island) = self.active_mut() {
            island.prepare_logistics(&terrain);
            island.prepare_production(&terrain);
        }
        self.selected = index;
        self.terrain = Some(terrain);
        self.target = None;
        self.waypoint = None;
        self.map_zoom = 0.0;
        self.heading = 0.0;
        self.hand_clock = 0.0;
        self.work_swing = 0.0;
        self.harvesting = None;
        self.building = None;
        self.supply_for = None;
        self.finale = None;
        self.milestones = None;
        self.scene_changes.clear();
        self.walkthrough = level.stage == 0 || practice;
    }

    fn save(&mut self, game: &mut Game) {
        self.save_clock = 0.0;
        if self.practice() {
            return;
        }
        let result = (|| -> io::Result<()> {
            let path = book_path();
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

    /// Syncs finished islands both ways: stars for islands finished in the book, and
    /// unlocked successors for islands the campaign remembers even if their save got lost.
    pub fn restore_rewards(&self, campaign: &mut Campaign) -> bool {
        let mut changed = false;
        for (index, island) in self.book.islands.iter().enumerate() {
            if island.as_ref().is_some_and(|island| island.completed) {
                changed |= campaign.reward_adventure(index);
            }
        }
        let rewarded = serde_json::to_value(&*campaign)
            .ok()
            .and_then(|save| save.get("adventures").cloned())
            .and_then(|list| serde_json::from_value::<Vec<usize>>(list).ok())
            .unwrap_or_default();
        let mask = rewarded
            .into_iter()
            .filter(|&index| index < LEVELS)
            .fold(0u64, |mask, index| mask | 1 << index);
        self.cleared
            .fetch_or(mask, std::sync::atomic::Ordering::Relaxed);
        changed
    }

    pub fn target_position(&self, target: Target) -> Option<Vec2> {
        let terrain = self.terrain.as_ref()?;
        match target {
            Target::Deposit(id) => terrain
                .deposits
                .get(id)
                .filter(|_| {
                    self.active()
                        .is_some_and(|island| island.remaining(terrain, id) > 0)
                })
                .map(|d| d.at),
            Target::Dropped(id) => self
                .active()?
                .dropped_load(id)
                .map(|pile| Vec2::from_array(pile.position)),
            Target::Encounter(slot) => {
                let status = self.active()?.encounters.visits.get(slot)?.status;
                (!matches!(status, EncounterStatus::Hidden | EncounterStatus::Finished))
                    .then(|| terrain.events[slot])
            }
            Target::EncounterSource(slot) => {
                let island = self.active()?;
                let spec = island.encounter(terrain.level, slot)?;
                if island.encounters.visits[slot].status != EncounterStatus::Seeking {
                    return None;
                }
                Some(match spec.event {
                    Event::Fire => self
                        .encounter_well(slot)
                        .map_or(terrain.shore_intake, |lot| terrain.plots[lot]),
                    Event::Relic => terrain.finds[slot],
                    _ => terrain::HALL_DOOR + Vec2::new(-0.53, 0.26),
                })
            }
            Target::TradeHarbor => terrain.level.has_trading().then_some(terrain.trade_pier),
            Target::Plot(id) => terrain.plots.get(id).copied(),
            Target::Hall => Some(Vec2::ZERO),
            Target::Shrine => Some(terrain.shrine),
            Target::Relic => Some(terrain.relic),
            Target::Outpost => (terrain.level.order_count() > 0).then_some(terrain.outpost),
        }
    }

    pub fn encounter_spec(&self, slot: usize) -> Option<encounters::Encounter> {
        self.active()?.encounter(self.terrain.as_ref()?.level, slot)
    }

    pub fn encounter_well(&self, slot: usize) -> Option<usize> {
        let (terrain, island) = self.terrain.as_ref().zip(self.active())?;
        let at = *terrain.events.get(slot)?;
        island.lots.iter().enumerate().filter_map(|(lot, property)| {
            matches!(property, Some(Property {kind: Build::Harbor(kind), ..}) if kind.water_source())
                .then_some(lot)
        }).min_by(|&a, &b| terrain.plots[a].distance_squared(at).total_cmp(&terrain.plots[b].distance_squared(at)))
    }

    pub fn encounter_target(&self, slot: usize) -> Option<Target> {
        let status = self.active()?.encounters.visits.get(slot)?.status;
        Some(match status {
            EncounterStatus::Seeking => Target::EncounterSource(slot),
            EncounterStatus::Offered | EncounterStatus::Carrying | EncounterStatus::Working => {
                Target::Encounter(slot)
            }
            _ => return None,
        })
    }

    pub fn close_enough(&self, target: Target) -> bool {
        let Some(at) = self.target_position(target) else {
            return false;
        };
        self.active()
            .is_some_and(|island| Vec2::from_array(island.position).distance(at) < reach(target))
    }

    pub fn waypoint_path(&self) -> Option<Vec<Vec2>> {
        let target = self.waypoint?;
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        let here = Vec2::from_array(island.position);
        let destination = self.target_position(target)?;
        let mut path = terrain.route(here, destination, reach(target) - 0.12, &island.lots)?;
        path.push(here);
        path.reverse();
        Some(path)
    }

    pub fn next_island(&self) -> Option<Level> {
        let next = self.selected + 1;
        (!self.practice() && next < LEVELS && self.completed(self.selected))
            .then(|| Level::at(next))
    }

    pub fn delivery_action(&self) -> Option<Command> {
        let island = self.active()?;
        let level = self.terrain.as_ref()?.level;
        level.order(island.orders_delivered)?;
        if island.market.packed.is_some() || island.encounters.carrying().is_some() {
            return None;
        }
        if island.shipment.is_some() {
            return Some(if self.close_enough(Target::Outpost) {
                Command::Use(Target::Outpost)
            } else {
                Command::SetRoute(Target::Outpost)
            });
        }
        if island.courier.as_ref().is_some_and(|c| c.order.is_some()) {
            return None;
        }
        if !self.close_enough(Target::Hall) {
            return Some(Command::SetRoute(Target::Hall));
        }
        if island.cargo.is_some() {
            return Some(Command::Use(Target::Hall));
        }
        island.order_ready(level).then_some(Command::PackOrder)
    }

    pub fn supply_lot(&self) -> Option<usize> {
        let island = self.active()?;
        if island.has_cargo() {
            return None;
        }
        self.supply_for
            .filter(|&lot| island.site_supply(lot).is_some())
            .or_else(|| (0..island.lots.len()).find(|&lot| island.site_supply(lot).is_some()))
    }

    fn candidate(&self) -> Option<Target> {
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        if let Some(preferred) = self
            .waypoint
            .or_else(|| self.walkthrough_step().map(|(_, _, target)| target))
            && self.close_enough(preferred)
        {
            return Some(preferred);
        }
        let here = Vec2::from_array(island.position);
        targets(terrain, island)
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
        targets(terrain, island)
            .filter_map(|target| {
                let p = self.target_position(target)?;
                let world = crate::world::adventure_position(terrain, p);
                let height = match target {
                    Target::Plot(lot) => {
                        island.lots[lot].map_or(0.13, |p| 1.8 + f32::from(p.tier) * 0.21)
                    }
                    Target::Hall => 3.05,
                    Target::Shrine | Target::Outpost => 2.9,
                    Target::Relic => 1.15,
                    Target::Dropped(_) => 0.55,
                    Target::Encounter(_) | Target::EncounterSource(_) => 1.03,
                    Target::TradeHarbor => 1.50,
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

    /// What the HUD suggests next: the walkthrough on the first island, then the advice.
    /// The flag is true for walkthrough steps.
    pub fn guide(&self) -> Option<(Advice, bool)> {
        if let Some((icon, text, target)) = self.walkthrough_step() {
            return Some((
                Advice {
                    icon,
                    text: text.to_owned(),
                    target: Some(target),
                },
                true,
            ));
        }
        self.advice().map(|advice| (advice, false))
    }

    /// The guided target with its hint, used for markers and the route button.
    pub fn tutorial(&self) -> Option<(&'static str, String, Target)> {
        let (advice, _) = self.guide()?;
        Some((advice.icon, advice.text, advice.target?))
    }

    fn walkthrough_step(&self) -> Option<(&'static str, &'static str, Target)> {
        let terrain = self.terrain.as_ref()?;
        let island = self.active()?;
        if terrain.level.stage != 0 || !self.walkthrough || island.completed {
            return None;
        }
        if let Some((lot, site)) = island
            .construction
            .iter()
            .enumerate()
            .find_map(|(lot, site)| site.as_ref().map(|site| (lot, site)))
        {
            if island.cargo.is_some_and(|load| site.missing(load.kind) > 0) {
                return Some((
                    "hammer",
                    "Leg die Fracht auf der Baustelle ab.",
                    Target::Plot(lot),
                ));
            }
            if self.building == Some(lot) || site.ready() {
                return Some((
                    "hammer",
                    "Alles liegt bereit. Wähle „Mitbauen“.",
                    Target::Plot(lot),
                ));
            }
            if island.cargo.is_none()
                && island.site_supply(lot).is_some()
                && island.site_workers(lot) == 0
            {
                return Some((
                    "storehouse",
                    "Hol das Baumaterial am Rathaus ab.",
                    Target::Hall,
                ));
            }
            if island.cargo.is_none()
                && let Some((good, id)) = Good::RAW
                    .into_iter()
                    .filter(|&good| site.missing(good) > 0)
                    .find_map(|good| {
                        island
                            .available_near(terrain, good, Vec2::from_array(island.position))
                            .map(|id| (good, id))
                    })
            {
                let hint = if good == Good::Wood {
                    "Fälle einen Baum für die Baustelle."
                } else {
                    "Baue das fehlende Material ab."
                };
                return Some((good.icon(terrain.level.region), hint, Target::Deposit(id)));
            }
        }
        if island.cargo.is_some() {
            return Some((
                "storehouse",
                "Bring die Fracht zum Rathaus und lade sie dort ab.",
                Target::Hall,
            ));
        }
        let resource = |kind: Good, tip| {
            island
                .available_near(terrain, kind, Vec2::from_array(island.position))
                .map(|site| (kind.icon(terrain.level.region), tip, Target::Deposit(site)))
        };
        let home = Build::Harbor(terrain.level.home());
        let has_home = island.lots.iter().flatten().any(|p| p.kind == home);
        let woodwork = island
            .lots
            .iter()
            .position(|p| p.is_some_and(|p| p.kind == Build::Sawmill));
        if has_home && let Some(lot) = woodwork {
            if island.amount(Good::Food) == 0 && island.amount(Good::Boards) < 8 {
                return resource(Good::Food, "Sammle Feldfrüchte für die Holzwerkstatt.");
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
                return resource(Good::Wood, "Holz hält die Werkstatt in Betrieb.");
            }
            let goal = terrain.level.requirements()[2].kind;
            if !island.lots.iter().flatten().any(|p| p.kind == goal) {
                let plot = island
                    .lots
                    .iter()
                    .enumerate()
                    .find(|(lot, p)| p.is_none() && goal.fits(terrain.level, *lot))
                    .map(|(i, _)| i)?;
                if island.amount(Good::Wood) < 5 && island.amount(Good::Boards) >= 8 {
                    return resource(Good::Wood, "Sammle Holz für das nächste Gebäude.");
                }
                if island.amount(Good::Stone) < 5 {
                    return resource(Good::Stone, "Baue Stein für das nächste Gebäude ab.");
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
        let lot = island
            .lots
            .iter()
            .enumerate()
            .filter(|(lot, p)| p.is_none() && island.site(*lot).is_none())
            .min_by(|(a, _), (b, _)| {
                terrain.plots[*a]
                    .length_squared()
                    .total_cmp(&terrain.plots[*b].length_squared())
            })
            .map(|(i, _)| i)?;
        let hint = if has_home {
            "Plane ein Sägewerk auf dem markierten Platz."
        } else {
            "Plane das erste Wohnhaus auf dem markierten Bauplatz."
        };
        Some(("hammer", hint, Target::Plot(lot)))
    }
}

fn targets(terrain: &Terrain, island: &Island) -> impl Iterator<Item = Target> {
    (0..terrain.deposits.len())
        .map(Target::Deposit)
        .chain(
            island
                .dropped
                .iter()
                .enumerate()
                .filter_map(|(i, pile)| pile.as_ref().map(|_| Target::Dropped(i))),
        )
        .chain(
            (0..terrain.level.encounter_count())
                .flat_map(|slot| [Target::Encounter(slot), Target::EncounterSource(slot)]),
        )
        .chain(terrain.level.has_trading().then_some(Target::TradeHarbor))
        .chain((0..terrain.plots.len()).map(Target::Plot))
        .chain([Target::Hall])
        .chain(
            (terrain.level.plan().is_some() && island.quest != Quest::Finished)
                .then_some(Target::Shrine),
        )
        .chain((island.quest == Quest::Searching).then_some(Target::Relic))
        .chain((terrain.level.order_count() > 0).then_some(Target::Outpost))
}

fn reach(target: Target) -> f32 {
    match target {
        Target::Plot(_) | Target::Shrine | Target::Outpost => 2.9,
        Target::Hall => 3.75,
        Target::Deposit(_) => RESOURCE_REACH,
        Target::Dropped(_) => 1.55,
        Target::Encounter(_) => 2.32,
        Target::EncounterSource(_) => 2.55,
        Target::TradeHarbor => 2.65,
        _ => 2.4,
    }
}

fn close(game: &mut Game, adventure: &mut Adventure, screen: Screen) {
    adventure.screen = screen;
    adventure.moved = false;
    adventure.pointer = None;
    game.walk_axis = Vec2::ZERO;
    game.quick_menu = false;
    game.scroll_y = 0.0;
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
                Command::Next => {
                    let Some(next) = adventure.next_island() else {
                        return;
                    };
                    next.index
                }
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
                Screen::Atlas => {
                    act(Command::Home, adventure, game, save);
                    return;
                }
                _ => Screen::Walk,
            };
            close(game, adventure, next);
        }
        Command::Pause => close(game, adventure, Screen::Pause),
        Command::Summary => {
            if adventure.active().is_some_and(|island| island.completed) {
                adventure.finale = None;
                close(game, adventure, Screen::Finish);
            }
        }
        Command::Help => close(game, adventure, Screen::Help),
        Command::Map => close(game, adventure, Screen::Map),
        Command::Storage => close(game, adventure, Screen::Storage),
        Command::Encounters => {
            close(game, adventure, Screen::Mission);
            adventure.group = 4;
            adventure.page = u8::MAX;
        }
        Command::Trading => {
            if adventure
                .terrain
                .as_ref()
                .is_some_and(|map| map.level.has_trading())
            {
                close(game, adventure, Screen::Mission);
                adventure.group = 5;
            }
        }
        Command::PackExport(_)
        | Command::UnpackExport
        | Command::SellExport
        | Command::BuyPlan
        | Command::BuyImports
        | Command::Buy(_) => trade_action(command, adventure, game),
        Command::ReturnEncounter => {
            if adventure.close_enough(Target::Hall)
                && let Some(island) = adventure.active_mut()
            {
                let level = Level::at(island.index);
                if island.return_encounter_cargo(level) {
                    adventure.waypoint = None;
                    game.adventure_notice("Fracht wieder abgelegt.", true);
                    adventure.save(game);
                    game.dirty = true;
                } else {
                    game.adventure_notice("Schaffe zuerst Platz im Lager.", false);
                }
            }
        }
        Command::ClearStock(good) => {
            if adventure.screen != Screen::Storage {
                return;
            }
            let level = Level::at(adventure.selected);
            if let Some(island) = adventure.active_mut() {
                let count = island.discard_stock(good);
                if count > 0 {
                    game.adventure_notice(
                        format!("{count} {} aussortiert", good.name(level.region)),
                        true,
                    );
                    game.dirty = true;
                    adventure.save(game);
                }
            }
        }
        Command::StoreOverflow(good) => {
            if adventure.screen == Screen::Storage
                && adventure.close_enough(Target::Hall)
                && let Some(island) = adventure.active_mut()
            {
                let count = island.store_overflow(good);
                if count > 0 {
                    let region = Level::at(island.index).region;
                    game.adventure_notice(
                        format!("{count} {} eingelagert", good.name(region)),
                        true,
                    );
                    game.dirty = true;
                    adventure.save(game);
                }
            }
        }
        Command::Labor => close(game, adventure, Screen::Labor),
        Command::SetRoute(target) => {
            if adventure.target_position(target).is_some() {
                adventure.waypoint = Some(target);
                close(game, adventure, Screen::Walk);
            } else {
                game.adventure_notice("Dieses Ziel ist nicht mehr verfügbar.", false);
            }
        }
        Command::CameraView => {
            adventure.map_zoom = if adventure.map_zoom > 0.1 { 0.0 } else { 0.38 };
            game.quick_menu = false;
            game.dirty = true;
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
                Target::Dropped(_) => act(Command::Use(target), adventure, game, save),
                Target::Encounter(slot) | Target::EncounterSource(slot) => {
                    act(Command::Encounters, adventure, game, save);
                    adventure.page = slot as u8;
                }
                Target::TradeHarbor => act(Command::Trading, adventure, game, save),
                Target::Hall | Target::Shrine | Target::Relic | Target::Outpost => {
                    adventure.target = Some(target);
                    close(game, adventure, Screen::Mission);
                    adventure.group = match target {
                        Target::Outpost => 1,
                        Target::Shrine | Target::Relic => 2,
                        _ => 0,
                    };
                }
            }
        }
        Command::Use(target) => {
            if !adventure.close_enough(target) {
                return;
            }
            if matches!(target, Target::Encounter(_) | Target::EncounterSource(_)) {
                help_encounter(target, adventure, game, save);
                return;
            }
            if target == Target::TradeHarbor {
                let ready = adventure.active().is_some_and(|island| {
                    island.market.packed.is_some() && island.market.crossing.is_none()
                });
                act(
                    if ready {
                        Command::SellExport
                    } else {
                        Command::Trading
                    },
                    adventure,
                    game,
                    save,
                );
                return;
            }
            let Some(terrain) = adventure.terrain.as_ref() else {
                return;
            };
            let level = terrain.level;
            if let Target::Plot(lot) = target
                && let Some(site) = adventure.active().and_then(|island| island.site(lot))
            {
                let ready = site.ready();
                let position = terrain.plots[lot];
                let island = adventure.active_mut().unwrap();
                if let Some(load) = island.drop_building_load(lot) {
                    adventure.waypoint = None;
                    close(game, adventure, Screen::Walk);
                    game.adventure_notice(
                        format!(
                            "{} {} auf der Baustelle",
                            load.units,
                            load.kind.name(level.region)
                        ),
                        true,
                    );
                    adventure.save(game);
                } else if ready {
                    let at = position - Vec2::from_array(island.position);
                    adventure.heading = at.x.atan2(at.y);
                    adventure.building = (adventure.building != Some(lot)).then_some(lot);
                    close(game, adventure, Screen::Walk);
                } else {
                    act(Command::Inspect(target), adventure, game, save);
                }
                return;
            }
            if let Target::Deposit(site) = target {
                let kind = terrain.deposits[site].kind;
                if adventure.harvesting.is_some() || adventure.hand_clock > 0.0 {
                    return;
                }
                let Some(island) = adventure.active() else {
                    return;
                };
                if island.room_for(kind) == 0 {
                    game.adventure_notice("Lade zuerst deine Fracht ab.", false);
                    return;
                }
                let focus = terrain.deposits[site].at - Vec2::from_array(island.position);
                if focus.length_squared() > 0.01 {
                    adventure.heading = focus.x.atan2(focus.y);
                }
                let duration = kind.work_seconds();
                adventure.harvesting = Some(Harvest {
                    site,
                    left: duration,
                    duration,
                });
                adventure.building = None;
                if adventure.screen == Screen::Walk {
                    adventure.moved = false;
                    game.dirty = true;
                } else {
                    close(game, adventure, Screen::Walk);
                }
                game.harvest(kind);
                return;
            }
            if target == Target::Hall {
                if adventure.delivery_action() == Some(Command::PackOrder) {
                    act(Command::PackOrder, adventure, game, save);
                    return;
                }
                if let Some(lot) = adventure.supply_lot() {
                    act(Command::TakeMaterial(lot), adventure, game, save);
                    return;
                }
            }
            let Some(island) = adventure.active_mut() else {
                return;
            };
            match target {
                Target::Dropped(id) => {
                    let Some(load) = island.pick_up_load(id) else {
                        game.adventure_notice("Lade zuerst deine Fracht ab.", false);
                        return;
                    };
                    adventure.target = None;
                    adventure.waypoint = Some(Target::Hall);
                    close(game, adventure, Screen::Walk);
                    game.adventure_notice(
                        format!(
                            "{} {} aufgenommen",
                            load.units,
                            load.kind.name(level.region)
                        ),
                        true,
                    );
                }
                Target::Hall if island.cargo.is_some() => {
                    let Some(load) = island.unload() else {
                        game.adventure_notice(
                            "Rohstofflager voll. Schaffe Platz oder baue einen Speicher.",
                            false,
                        );
                        game.dirty = true;
                        return;
                    };
                    adventure.waypoint = island.cargo.map(|_| Target::Hall);
                    close(game, adventure, Screen::Walk);
                    game.adventure_notice(
                        format!(
                            "+{} {} im Rathaus",
                            load.units,
                            load.kind.name(level.region)
                        ),
                        true,
                    );
                }
                Target::Outpost if island.deliver_order(level) => {
                    adventure.waypoint = None;
                    close(game, adventure, Screen::Walk);
                    game.adventure_notice("Lieferung angekommen", true);
                }
                Target::Plot(lot) if island.can_help(lot) => {
                    lend_a_hand(adventure, game, lot, true);
                    return;
                }
                Target::Hall | Target::Plot(_) | Target::Outpost => {
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
        Command::Deliveries => {
            adventure.target = Some(Target::Hall);
            close(game, adventure, Screen::Mission);
            adventure.group = 1;
        }
        Command::SendCourier => {
            let level = Level::at(adventure.selected);
            if adventure
                .active_mut()
                .is_some_and(|island| island.send_courier(level))
            {
                close(game, adventure, Screen::Walk);
                game.adventure_notice("Ein Bürger bringt die Fracht zum Außenposten.", true);
                adventure.save(game);
            } else {
                game.adventure_notice(
                    "Für die Lieferung brauchst du Waren und einen freien Bürger.",
                    false,
                );
            }
        }
        Command::PackOrder | Command::ReturnOrder => {
            if !adventure.close_enough(Target::Hall) {
                return;
            }
            let level = Level::at(adventure.selected);
            let Some(island) = adventure.active_mut() else {
                return;
            };
            let packing = command == Command::PackOrder;
            let changed = if packing {
                island.pack_order(level)
            } else {
                island.return_order(level)
            };
            if changed {
                adventure.waypoint = packing.then_some(Target::Outpost);
                close(game, adventure, Screen::Walk);
                let message = if packing {
                    "Fracht gepackt. Folge der Markierung."
                } else {
                    "Fracht wieder eingelagert."
                };
                game.adventure_notice(message, true);
                adventure.save(game);
            } else if packing {
                game.adventure_notice(
                    "Sieh unter Lieferungen nach, welche Waren noch fehlen.",
                    false,
                );
            } else {
                game.adventure_notice("Im Lager ist noch kein Platz für die Fracht.", false);
            }
        }
        Command::Build(kind) => {
            let Screen::Lot(lot) = adventure.screen else {
                return;
            };
            let Some(terrain) = adventure.terrain.as_ref() else {
                return;
            };
            let level = terrain.level;
            if adventure.close_enough(Target::Plot(lot))
                && adventure
                    .active_mut()
                    .is_some_and(|island| island.plan_build(level, lot, kind))
            {
                close(game, adventure, Screen::Lot(lot));
                game.adventure_notice("Baustelle eingerichtet", true);
                adventure.save(game);
            }
        }
        Command::TakeMaterial(lot) => {
            if !adventure.close_enough(Target::Hall) {
                adventure.supply_for = Some(lot);
                act(Command::SetRoute(Target::Hall), adventure, game, save);
                return;
            }
            if let Some(load) = adventure
                .active_mut()
                .and_then(|island| island.take_building_load(lot))
            {
                let region = Level::at(adventure.selected).region;
                adventure.waypoint = Some(Target::Plot(lot));
                close(game, adventure, Screen::Walk);
                game.adventure_notice(
                    format!(
                        "{} {} für die Baustelle",
                        load.units,
                        load.kind.name(region)
                    ),
                    true,
                );
                adventure.save(game);
            } else {
                game.adventure_notice("Leere deine Hände oder fülle das Lager auf.", false);
            }
        }
        Command::BuildCrew(lot, change) => {
            if adventure
                .active_mut()
                .is_some_and(|island| island.with_crew(lot, change))
            {
                game.dirty = true;
                adventure.save(game);
            }
        }
        Command::CancelBuild(lot) => {
            if adventure
                .active_mut()
                .is_some_and(|island| island.cancel_build(lot))
            {
                if adventure.building == Some(lot) {
                    adventure.building = None;
                }
                close(game, adventure, Screen::Walk);
                game.adventure_notice(
                    "Baustelle aufgehoben. Material kommt zurück ins Lager.",
                    true,
                );
                adventure.save(game);
            } else {
                game.adventure_notice("Schaffe Platz im Lager für das Baumaterial.", false);
            }
        }
        Command::Group(group) => {
            adventure.group = group.min(if adventure.screen == Screen::Mission {
                5
            } else {
                2
            });
            adventure.page = if group == 4 { u8::MAX } else { 0 };
            game.scroll_y = 0.0;
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
            if adventure
                .active()
                .is_some_and(|island| island.batch(lot).is_some())
            {
                game.adventure_notice("Der Betrieb hat noch einen laufenden Arbeitsgang.", false);
                return;
            }
            if let Some(island) = adventure.active_mut()
                && let Some(property) = island.lots[lot].take()
            {
                if matches!(property.kind, Build::Harbor(kind) if crate::model::Building::new(kind).residents() > 0)
                    && island.assigned_staff() > usize::from(island.staff())
                {
                    island.lots[lot] = Some(property);
                    game.adventure_notice(
                        "Stelle zuerst Bürger frei, bevor du das Wohnhaus abreißt.",
                        false,
                    );
                    return;
                }
                let refunds = property.kind.cost(1);
                let raw_refund: u16 = refunds
                    .iter()
                    .filter(|(good, _)| good.uses_storage())
                    .map(|(_, units)| units / 2)
                    .sum();
                if raw_refund > island.free_storage()
                    || island.raw_stock() > island.storage_capacity()
                {
                    island.lots[lot] = Some(property);
                    game.adventure_notice("Schaffe zuerst Platz im Rohstofflager.", false);
                    return;
                }
                for (good, n) in refunds {
                    island.add(good, n / 2);
                }
                island.set_masterwork(Level::at(island.index), lot, false);
                while island.unassign(Task::Produce(lot)) {}
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
                    game.adventure_notice(
                        if assign {
                            "Auftrag vergeben"
                        } else {
                            "Auftrag beendet"
                        },
                        true,
                    );
                    adventure.save(game);
                }
            }
            game.dirty = true;
        }
        Command::Masterwork(lot, enabled) => {
            if adventure.close_enough(Target::Plot(lot))
                && let Some(island) = adventure.active_mut()
            {
                let level = Level::at(island.index);
                if island.set_masterwork(level, lot, enabled) {
                    adventure.save(game);
                    game.dirty = true;
                }
            }
        }
        Command::Work(lot)
            if adventure
                .active()
                .is_some_and(|island| island.can_help(lot)) =>
        {
            if adventure.close_enough(Target::Plot(lot)) {
                lend_a_hand(adventure, game, lot, false);
            }
        }
        Command::Work(lot) => {
            if adventure.close_enough(Target::Plot(lot))
                && adventure.hand_clock <= 0.0
                && let Some(map) = adventure.terrain.clone()
                && adventure
                    .active_mut()
                    .is_some_and(|island| island.start_production(&map, lot))
            {
                adventure.hand_clock = 1.45;
                adventure.work_swing = 0.46;
                game.adventure_notice("Produktion gestartet", true);
                game.dirty = true;
                adventure.save(game);
            }
        }
    }
    check_victory(adventure, game, save);
}

/// Starts Nili's help at a running workshop batch, or stops it when `toggle` is set.
fn lend_a_hand(adventure: &mut Adventure, game: &mut Game, lot: usize, toggle: bool) {
    let Some((terrain, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    let at = terrain.plots[lot] - Vec2::from_array(island.position);
    adventure.heading = at.x.atan2(at.y);
    let helping = adventure.building == Some(lot);
    adventure.building = (!helping || !toggle).then_some(lot);
    close(game, adventure, Screen::Walk);
    if !helping && adventure.building.is_some() {
        game.adventure_notice("Nili hilft mit. Der Arbeitsgang läuft schneller.", true);
    }
}

fn trade_action(command: Command, adventure: &mut Adventure, game: &mut Game) {
    use self::Command::*;
    let target = match command {
        PackExport(_) | UnpackExport => Target::Hall,
        _ => Target::TradeHarbor,
    };
    if !adventure.close_enough(target) {
        return;
    }
    let Some(island) = adventure.active_mut() else {
        return;
    };
    let level = Level::at(island.index);
    let (success, notice) = match command {
        PackExport(option) => (island.pack_export(level, option), "Handelsfracht gepackt"),
        UnpackExport => (
            island.unpack_export(level),
            "Handelsfracht wieder eingelagert",
        ),
        SellExport => (island.launch_export(level), "Das Handelsschiff legt ab."),
        BuyPlan => (island.buy_monument(level), "Sonderbauplan gekauft"),
        BuyImports => (
            island.buy_imports(level),
            "Baustoffe geladen. Trag sie zum Rathaus oder direkt zur Baustelle.",
        ),
        Buy(item) => {
            let Some(offer) = island.buy_goods(level, item) else {
                game.adventure_notice("Dafür fehlen Handelsmarken oder freie Hände.", false);
                return;
            };
            let site = island.building_load;
            adventure.waypoint = Some(site.map_or(Target::Hall, Target::Plot));
            let destination = if site.is_some() {
                "zur Baustelle"
            } else {
                "zum Rathaus"
            };
            game.adventure_notice(
                format!(
                    "{} {} gekauft · {destination}",
                    offer.units,
                    offer.good.name(level.region)
                ),
                true,
            );
            game.dirty = true;
            adventure.save(game);
            return;
        }
        _ => return,
    };
    if success {
        if matches!(command, PackExport(_)) {
            adventure.waypoint = Some(Target::TradeHarbor);
            close(game, adventure, Screen::Walk);
        } else if command == BuyImports {
            adventure.waypoint = Some(Target::Hall);
        } else if matches!(command, UnpackExport | SellExport) {
            adventure.waypoint = None;
            close(game, adventure, Screen::Walk);
        }
        game.adventure_notice(notice, true);
        game.dirty = true;
        adventure.save(game);
    }
}

fn help_encounter(target: Target, adventure: &mut Adventure, game: &mut Game, save: &mut Campaign) {
    let (slot, source) = match target {
        Target::Encounter(slot) => (slot, false),
        Target::EncounterSource(slot) => (slot, true),
        _ => return,
    };
    let Some(island) = adventure.active_mut() else {
        return;
    };
    let level = Level::at(island.index);
    let Some(spec) = island.encounter(level, slot) else {
        return;
    };
    let status = island.encounters.visits[slot].status;
    let (next, route, notice) = match (status, source) {
        (EncounterStatus::Offered, false) => (
            EncounterStatus::Seeking,
            Some(Target::EncounterSource(slot)),
            spec.request,
        ),
        (EncounterStatus::Seeking, true) => {
            if island.has_cargo() {
                game.adventure_notice("Lade zuerst deine Fracht ab.", false);
                return;
            }
            if !island.encounter_pickup(level, slot) {
                game.adventure_notice("Im Rathaus fehlt noch ein Teil der Fracht.", false);
                return;
            }
            (
                EncounterStatus::Carrying,
                Some(Target::Encounter(slot)),
                "Die Fracht ist unterwegs.",
            )
        }
        (EncounterStatus::Carrying, false) => {
            adventure.work_swing = 0.6;
            (
                EncounterStatus::Working,
                None,
                spec.action(EncounterStatus::Working),
            )
        }
        _ => {
            act(Command::Encounters, adventure, game, save);
            adventure.page = slot as u8;
            return;
        }
    };
    let island = adventure.active_mut().unwrap();
    if next != EncounterStatus::Carrying {
        island.encounters.changed(slot, next)
    }
    adventure.waypoint = route;
    close(game, adventure, Screen::Walk);
    if next == EncounterStatus::Seeking {
        act(Command::Encounters, adventure, game, save);
        adventure.page = slot as u8;
    } else {
        game.adventure_notice(notice, true);
    }
    adventure.save(game);
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
            // Let the moment land before the summary opens.
            adventure.finale = Some(FINALE_SECONDS);
            game.adventure_complete();
            for (step, pitch) in [0.94, 1.12, 1.26].into_iter().enumerate() {
                game.play_later(Cue::Star, 0.32 + step as f32 * 0.2, Some(pitch));
            }
            game.play_later(Cue::Goal, 1.05, None);
            game.dirty = true;
        }
    }
    celebrate(adventure, game);
}

/// What the player has reached so far, compared between updates to cheer for progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Milestones {
    reached: u32,
    rows: u32,
    staff: u16,
    capacity: u16,
    reserve_left: u16,
}

impl Milestones {
    fn of(island: &Island, level: Level) -> Self {
        let (stock, count) = level.reserve();
        let rows = milestone_rows(island, level).count();
        Self {
            reached: milestone_rows(island, level)
                .enumerate()
                .filter(|(_, (_, done))| *done)
                .fold(0, |mask, (row, _)| mask | 1 << row),
            rows: (1 << rows) - 1,
            staff: island.staff(),
            capacity: island.capacity(),
            reserve_left: count.saturating_sub(island.amount(stock)),
        }
    }
}

/// Every goal row except the reserve, which rises and falls with production.
fn milestone_rows(island: &Island, level: Level) -> impl Iterator<Item = (String, bool)> + '_ {
    let region = level.region;
    level
        .goals()
        .map(move |goal| {
            let name = goal.kind.name(region);
            let title = if goal.tier > 1 {
                format!("{name} Stufe {}", goal.tier)
            } else {
                name.to_owned()
            };
            (title, island.progress(goal) >= goal.count)
        })
        .chain(level.plan().map(|_| {
            (
                "Bauplan gefunden".to_owned(),
                island.quest == Quest::Finished,
            )
        }))
        .chain((level.order_count() > 0).then(|| {
            (
                "Alle Lieferungen".to_owned(),
                island.orders_delivered >= level.order_count(),
            )
        }))
}

fn celebrate(adventure: &mut Adventure, game: &mut Game) {
    let Some((terrain, island)) = adventure.terrain.as_ref().zip(adventure.active()) else {
        return;
    };
    let level = terrain.level;
    let now = Milestones::of(island, level);
    let was = adventure.milestones;
    if was == Some(now) {
        return;
    }
    let cheers = was
        .filter(|_| !island.completed)
        .map(|was| cheers(island, level, was, now))
        .unwrap_or_default();
    adventure.milestones = Some(now);
    for cheer in cheers {
        game.adventure_toast(cheer, true);
    }
}

fn cheers(island: &Island, level: Level, was: Milestones, now: Milestones) -> Vec<String> {
    let mut cheers = Vec::new();
    let fresh = now.reached & !was.reached;
    if fresh != 0 {
        let (met, total) = island.milestones(level);
        let reached = milestone_rows(island, level)
            .enumerate()
            .filter(|(row, _)| fresh & (1 << row) != 0)
            .map(|(_, (title, _))| title)
            .last()
            .unwrap_or_default();
        cheers.push(format!("Inselziel {met}/{total} · {reached}"));
    }
    if now.staff > was.staff {
        let new = now.staff - was.staff;
        let verb = if new == 1 { "zieht" } else { "ziehen" };
        cheers.push(format!("+{new} Bürger {verb} ein"));
    }
    if now.capacity > was.capacity {
        cheers.push(format!("Nili trägt jetzt bis zu {} Waren", now.capacity));
    }
    let (stock, count) = level.reserve();
    // Count down the stock goal once it is the last thing missing.
    if now.reserve_left < was.reserve_left && now.reached == now.rows {
        let name = stock.name(level.region);
        let crossed = |mark: u16| was.reserve_left > mark && now.reserve_left <= mark;
        if now.reserve_left == 0 {
            cheers.push(format!("Vorrat komplett · {count} {name}"));
        } else if crossed(count / 2) || crossed(count / 4) || crossed(5) {
            cheers.push(format!("Nur noch {} {name}", now.reserve_left));
        }
    }
    cheers
}

pub(crate) fn first(a: &Adventure) -> Option<Action> {
    let initial = match a.screen {
        Screen::Atlas if !a.unlocked(a.selected) => Command::Choose(a.selected),
        Screen::Lot(lot) => {
            let (terrain, island) = a.terrain.as_ref().zip(a.active())?;
            if let Some(site) = island.site(lot) {
                if (site.ready() || island.cargo.is_some_and(|load| site.missing(load.kind) > 0))
                    && a.close_enough(Target::Plot(lot))
                {
                    return Some(Action::Adventure(Command::Use(Target::Plot(lot))));
                }
                return Some(Action::Adventure(Command::Close));
            }
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
                    plan.fits(terrain.level, lot) && !island.locked(terrain.level, plan, 1)
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
        Screen::Demolish(_)
        | Screen::Resource(_)
        | Screen::Labor
        | Screen::Map
        | Screen::Storage => Command::Close,
        Screen::Mission if a.group == 1 => a.delivery_action().unwrap_or(Command::Close),
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
        Screen::Finish => a.next_island().map_or(Command::Close, |_| Command::Next),
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
    if controls.got(controls::L3)
        && game.panel == Panel::None
        && !matches!(
            adventure.screen,
            Screen::Atlas | Screen::Brief | Screen::Finish
        )
    {
        let command = if adventure.screen == Screen::Storage {
            Command::Close
        } else {
            Command::Storage
        };
        act(command, adventure, game, save);
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
    if controls.got(controls::B) && game.quick_menu {
        crate::game::apply(Action::QuickMenu, game, save);
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
    if controls.got(controls::VIEW) {
        let waypoint_action = if adventure.waypoint.is_some() {
            Some(Command::ClearRoute)
        } else {
            adventure
                .tutorial()
                .filter(|&(_, _, target)| !adventure.close_enough(target))
                .map(|(_, _, target)| Command::SetRoute(target))
        };
        if let Some(action) = waypoint_action {
            act(action, adventure, game, save);
            return;
        }
    }
    if controls.got(controls::RB) || controls.got(controls::VIEW) {
        if adventure.active().is_some_and(|island| island.completed) {
            act(Command::Summary, adventure, game, save);
            return;
        }
        adventure.target = Some(Target::Hall);
        close(game, adventure, Screen::Mission);
        return;
    }
    adventure.map_zoom = (adventure.map_zoom + controls.zoom * dt * 0.95).clamp(-1.2, 0.64);
    if controls.got(controls::LB) {
        adventure.map_zoom = 0.0;
    }
    if adventure.harvesting.is_some() {
        game.walk_axis = Vec2::ZERO;
        adventure.moved = false;
        return;
    }
    let touch = ui::walk_input(pad, window, touches, mouse, &mut adventure.pointer);
    game.phone_walk(touch);
    let axis = (controls.walk + touch).clamp_length_max(1.0);
    game.walk_axis = axis;
    if axis.length_squared() > 0.02 && adventure.building.take().is_some() {
        game.dirty = true;
    }
    let carrying = adventure.active().is_some_and(Island::has_cargo);
    let speed = if carrying { 4.75 } else { 5.4 };
    let delta = Vec2::new(axis.x + axis.y * 0.115, axis.y - axis.x * 0.115) * speed * dt;
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
            | Screen::Storage
    ) || game.panel != Panel::None
    {
        return;
    }
    let building = adventure.building.filter(|&lot| {
        adventure.screen == Screen::Walk && adventure.close_enough(Target::Plot(lot))
    });
    let dt = dt.min(0.18);
    if let Some(left) = adventure.finale {
        adventure.finale = Some(left - dt).filter(|&left| left > 0.0);
        if adventure.finale.is_none() && adventure.active().is_some_and(|island| island.completed) {
            close(game, adventure, Screen::Finish);
            return;
        }
    }
    adventure.hand_clock = (adventure.hand_clock - dt).max(0.0);
    adventure.work_swing = (adventure.work_swing - dt).max(0.0);
    adventure.save_clock += dt;
    let finished = if let Some(harvest) = &mut adventure.harvesting {
        harvest.left = (harvest.left - dt).max(0.0);
        harvest.left == 0.0
    } else {
        false
    };
    let harvested = finished.then(|| adventure.harvesting.take().unwrap());
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
    let before = island.stores;
    if let Some(harvest) = harvested {
        game.dirty = true;
        let kind = terrain.deposits[harvest.site].kind;
        let here = Vec2::from_array(island.position);
        // A citizen may have emptied the node meanwhile; the next one in reach will do.
        let site = Some(harvest.site)
            .filter(|&site| island.remaining(terrain, site) > 0)
            .or_else(|| {
                island
                    .available_near(terrain, kind, here)
                    .filter(|&id| terrain.deposits[id].at.distance(here) <= RESOURCE_REACH + 0.9)
            });
        if let Some(load) = site.and_then(|site| island.carry_resource(terrain, site)) {
            let carried = island.cargo.map_or(load.units, |cargo| cargo.units);
            let room = island.room_for(kind);
            let name = kind.name(terrain.level.region);
            let building = (0..island.lots.len()).find(|&lot| {
                island
                    .site(lot)
                    .is_some_and(|site| site.missing(load.kind) > 0)
            });
            let site_covered = building
                .and_then(|lot| island.site(lot))
                .is_some_and(|site| site.missing(kind) <= carried);
            let more = (room > 0 && !site_covered)
                .then(|| island.available_near(terrain, kind, here))
                .flatten()
                .filter(|&id| terrain.deposits[id].at.distance(here) <= 8.0);
            if let Some(next) = more {
                adventure.waypoint = Some(Target::Deposit(next));
                game.adventure_notice(format!("{carried} {name} · Platz für {room}"), true);
            } else {
                adventure.waypoint = Some(building.map_or(Target::Hall, Target::Plot));
                let destination = if building.is_some() {
                    "zur Baustelle"
                } else {
                    "zum Rathaus"
                };
                game.adventure_notice(format!("{carried} {name} geladen · {destination}"), true);
            }
        } else {
            adventure.waypoint = None;
            game.adventure_notice("Diese Rohstoffstelle ist erschöpft.", false);
        }
    }
    let outside_before = island.overflow_total();
    game.dirty |= island.tick_hauling(terrain, dt);
    let overflowed = island.overflow_total() > outside_before;
    let helping = building.is_some_and(|lot| island.help_production(lot, dt));
    game.dirty |= island.tick_production(terrain, dt);
    let delivered_before = island.orders_delivered;
    let courier_changed = island.tick_courier(terrain, dt);
    let (construction_changed, completed) = island.tick_construction(terrain, dt, building);
    let encounter_changes = island.update_encounters(terrain.level, dt);
    let market_credit = island.tick_export(terrain.level, dt);
    let delivered = island.orders_delivered != delivered_before;
    game.dirty |= courier_changed || construction_changed;
    if delivered {
        game.adventure_notice("Der Bürger hat die Lieferung abgegeben.", true);
    }
    if let Some(Target::Deposit(site)) = adventure.waypoint
        && island.remaining(terrain, site) == 0
    {
        adventure.waypoint = None;
        game.dirty = true;
        game.adventure_notice("Die markierte Rohstoffstelle ist erschöpft.", false);
    }
    if let Some(lot) = adventure.building
        && island.site(lot).is_none()
        && !island.can_help(lot)
        && !(helping
            && island.assignment(Task::Produce(lot)) > 0
            && island
                .recipe_for(terrain.level, lot)
                .is_some_and(|recipe| island.ready(&recipe.need)))
    {
        adventure.building = None;
        game.dirty = true;
    }
    if let Some(&last) = completed.last() {
        let now = game.clock;
        adventure
            .scene_changes
            .retain(|&(_, start)| now - start < 1.3);
        for &lot in &completed {
            if adventure.building == Some(lot) {
                adventure.building = None;
            }
            adventure.scene_changes.push((lot, now));
        }
        game.adventure_built(island.lots[last].unwrap().kind.name(terrain.level.region));
    }
    if overflowed {
        game.adventure_toast(
            "Lager voll · Restfracht liegt vor dem Rathaus. Sammelauftrag beendet.",
            false,
        );
    }
    for &slot in &encounter_changes.offered {
        game.adventure_toast(
            format!(
                "Neue Begegnung · {}",
                island.encounter(terrain.level, slot).unwrap().title
            ),
            true,
        );
    }
    for &slot in &encounter_changes.burned_out {
        let spec = island.encounter(terrain.level, slot).unwrap();
        game.adventure_toast(
            format!(
                "{} · Das Feuer ist von selbst erloschen. Kein Lohn.",
                spec.title
            ),
            false,
        );
        if matches!(
            adventure.waypoint,
            Some(Target::Encounter(id) | Target::EncounterSource(id)) if id == slot
        ) {
            adventure.waypoint = None;
        }
    }
    for &slot in &encounter_changes.finished {
        let spec = island.encounter(terrain.level, slot).unwrap();
        game.adventure_toast(
            format!(
                "{} erledigt · +{} {}",
                spec.title,
                spec.reward.1,
                spec.reward.0.name(terrain.level.region)
            ),
            true,
        );
    }
    if let Some(credit) = market_credit {
        game.adventure_toast(
            format!("Handelsschiff zurück · +{credit} Handelsmarken"),
            true,
        );
    }
    let market_or_encounters_changed = market_credit.is_some()
        || !encounter_changes.offered.is_empty()
        || !encounter_changes.finished.is_empty()
        || !encounter_changes.burned_out.is_empty();
    game.dirty |= market_or_encounters_changed;
    if before != island.stores || delivered || !completed.is_empty() {
        game.dirty = true;
        check_victory(adventure, game, save);
    }
    if market_or_encounters_changed
        || overflowed
        || harvested.is_some()
        || courier_changed
        || construction_changed
        || adventure.save_clock >= 8.0
    {
        adventure.save(game);
    }
}

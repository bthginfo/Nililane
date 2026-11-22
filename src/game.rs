use bevy::{
    audio::{AudioSink, AudioSinkPlayback, Volume},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    campaign::{
        CHAPTERS, CORE_HARBORS, JAPAN_START, NORDIC_START, OASIS_START, REGION_SIZE, Region,
        main_route_end, next_harbor,
    },
    caravan::Contract,
    coach::{self, Lesson, Prompt, Step},
    controls::{self, Controls, Direction},
    cosmetics::{Category, Cosmetic},
    errands::{Job, ManualStage, NiliControl, Route, RouteStage},
    layout::{lot_col, lot_id, lot_row},
    model::{
        self, Assortment, GuestTip, HarborPlan, Kind, LOTS, Miss, Phase, Price, Town, Weather,
    },
    save::Campaign,
    street::{self, Activity},
    trade::SailingRoute,
    ui::{self, Hud},
    world::{StreetCamera, motion},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum View {
    #[default]
    Title,
    Chart,
    Street,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Panel {
    #[default]
    None,
    Brief,
    Build,
    Details,
    Goals,
    Forecast,
    Nili,
    Pause,
    Options,
    Wardrobe,
    Guide,
    Demolish,
    Report,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Begin,
    Resume,
    Chart,
    Title,
    Tutorial,
    Choose(usize),
    Region(Region),
    ChartPage(bool),
    Sail,
    NextChapter,
    BonusHarbor,
    Back,
    Pause,
    Options,
    Wardrobe,
    WardrobeTab(Category),
    WardrobeRegion(Option<Region>),
    Wear(Cosmetic),
    Guide,
    GuideTab(usize),
    Music,
    Sounds,
    LargeUi,
    Plot,
    Coach(Step),
    Lesson(bool),
    SelectLot(usize),
    LookAtLot(usize),
    StreetHelp(usize),
    Goals,
    Forecast,
    Feedback,
    TomorrowNotes,
    GuestReview(usize),
    GuestAdvice(usize),
    ReviewReason(Miss),
    LedgerPage(usize),
    HarborPlan(HarborPlan),
    Build(Kind),
    BuildPage(usize),
    Upgrade,
    Assortment(Assortment),
    Price(Price),
    Demolish,
    ConfirmDemolish,
    Open,
    Aid,
    AidAt(usize),
    RingBell(usize),
    SendVoyage(usize, SailingRoute),
    SendCaravan(usize, Contract),
    Nili,
    ChooseJob(Job),
    Dispatch,
    DoErrand,
    CancelErrand,
    Speed,
    NextDay,
}

#[derive(Clone, Copy)]
enum Cue {
    Confirm,
    Back,
    Blocked,
    Earned,
    Day,
    Build,
    Hammer,
    Ferry,
    Alert,
    Street(Activity),
    Aid,
    Sale,
    TempleBell,
    CastOff,
}

pub const BUILD_TIME: f32 = 1.58;
pub const DELIVERY_TIME: f32 = 3.2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Nearby {
    Building(usize),
    Activity(usize, Activity),
}

#[derive(Clone, Copy)]
pub struct Delivery {
    pub route: Route,
    pub since: f32,
}

#[derive(Clone, Copy)]
pub struct Construction {
    pub since: f32,
    pub from: u8,
}

#[derive(Resource)]
pub struct Game {
    pub view: View,
    pub panel: Panel,
    pub selected: usize,
    pub chart: usize,
    pub guide: usize,
    pub wardrobe_tab: Category,
    pub wardrobe_region: Option<Region>,
    pub build_page: usize,
    pub ledger_page: usize,
    pub review_guest: Option<usize>,
    pub review_reason: Option<Miss>,
    pub return_to_guests: bool,
    pub job: Option<Job>,
    pub nearby: Option<Nearby>,
    pub destination: Option<usize>,
    pub street_action: Option<(usize, Activity, f32)>,
    pub bell_action: Option<(usize, f32)>,
    pub loading_cargo: Option<(usize, f32)>,
    pub saw_prices: bool,
    pub lesson: Option<Lesson>,
    delivered: Option<Delivery>,
    pub focus: Option<Action>,
    pub dirty: bool,
    pub world_version: u32,
    pub note: String,
    pub note_left: f32,
    pub speed: u8,
    pub zoom: f32,
    pub pan: Vec2,
    pub clock: f32,
    pub walk_axis: Vec2,
    pub walking: bool,
    pub walk_distance: f32,
    walk_pointer: Option<ui::WalkPointer>,
    pub work: [Option<Construction>; LOTS],
    pub preview: Town,
    preview_rome: Town,
    preview_japan: Town,
    preview_nordic: Town,
    preview_oasis: Town,
    checkpoint: f32,
    cue: Option<Cue>,
}

impl Default for Game {
    fn default() -> Self {
        use Kind::*;
        let mut preview = Town::new(7, &CHAPTERS[7]);
        preview.lots[..12].copy_from_slice(&[
            Some(model::Building::new(House)),
            Some(model::Building::new(Well)),
            Some(model::Building::new(Potter)),
            Some(model::Building::new(Stoa)),
            Some(model::Building::new(Grove)),
            Some(model::Building::new(Inn)),
            Some(model::Building::new(Grove)),
            None,
            Some(model::Building::new(Bakery)),
            Some(model::Building::new(Taverna)),
            None,
            Some(model::Building::new(House)),
        ]);
        preview.lots[1].as_mut().unwrap().level = 2;
        preview.lots[3].as_mut().unwrap().level = 3;
        preview.lots[5].as_mut().unwrap().level = 2;
        let mut preview_rome = Town::new(JAPAN_START - 1, &CHAPTERS[JAPAN_START - 1]);
        preview_rome.lots[..12].copy_from_slice(&[
            Some(model::Building::new(House)),
            Some(model::Building::new(Well)),
            Some(model::Building::new(Grove)),
            Some(model::Building::new(Villa)),
            Some(model::Building::new(Aqueduct)),
            Some(model::Building::new(Forum)),
            None,
            Some(model::Building::new(Bakery)),
            Some(model::Building::new(Taverna)),
            Some(model::Building::new(Potter)),
            Some(model::Building::new(Baths)),
            Some(model::Building::new(Inn)),
        ]);
        for (lot, level) in [(3, 3), (4, 3), (5, 3), (10, 4)] {
            preview_rome.lots[lot].as_mut().unwrap().level = level;
        }
        preview_rome.lots[12] = Some(model::Building::new(Grove));
        preview_rome.lots[14] = Some(model::Building::new(Storehouse));
        preview_rome.lots[16] = Some(model::Building::new(Villa));
        let island = JAPAN_START + 5;
        let mut preview_japan = Town::new(island, &CHAPTERS[island]);
        for (lot, kind, level) in [
            (0, Storehouse, 1),
            (4, Machiya, 2),
            (6, PaperWorkshop, 2),
            (8, Temple, 3),
            (10, TeaHouse, 2),
        ] {
            preview_japan.lots[lot] = Some(model::Building {
                level,
                ..model::Building::new(kind)
            });
        }
        let island = NORDIC_START + 4;
        let mut preview_nordic = Town::new(island, &CHAPTERS[island]);
        for (lot, kind, level) in [
            (0, Storehouse, 1),
            (4, Longhouse, 3),
            (5, Fishery, 2),
            (6, FeastHall, 3),
            (7, Smokehouse, 2),
            (9, Boatyard, 3),
        ] {
            preview_nordic.lots[lot] = Some(model::Building {
                level,
                ..model::Building::new(kind)
            });
        }
        let island = OASIS_START + 14;
        let mut preview_oasis = Town::new(island, &CHAPTERS[island]);
        for (lot, kind, level) in [
            (0, OasisHome, 4),
            (5, MoonGarden, 3),
            (6, Perfumer, 2),
            (8, Caravanserai, 5),
            (10, OasisHome, 3),
            (11, Glassworks, 4),
        ] {
            preview_oasis.lots[lot] = Some(model::Building {
                level,
                ..model::Building::new(kind)
            });
        }
        Self {
            view: View::Title,
            panel: Panel::None,
            selected: 0,
            chart: 0,
            guide: 0,
            wardrobe_tab: Category::default(),
            wardrobe_region: None,
            build_page: 0,
            ledger_page: 0,
            review_guest: None,
            review_reason: None,
            return_to_guests: false,
            job: None,
            nearby: None,
            destination: None,
            street_action: None,
            bell_action: None,
            loading_cargo: None,
            saw_prices: false,
            lesson: None,
            delivered: None,
            focus: None,
            dirty: true,
            world_version: 0,
            note: String::new(),
            note_left: 0.0,
            speed: 1,
            zoom: 0.0,
            pan: Vec2::ZERO,
            clock: 0.0,
            walk_axis: Vec2::ZERO,
            walking: false,
            walk_distance: 0.0,
            walk_pointer: None,
            work: [None; LOTS],
            preview,
            preview_rome,
            preview_japan,
            preview_nordic,
            preview_oasis,
            checkpoint: 0.0,
            cue: None,
        }
    }
}

impl Game {
    pub fn lot_action(&self, town: &Town, lot: usize) -> Action {
        let chapter = &CHAPTERS[town.chapter];
        if town.favor_plan(chapter, lot).is_some()
            && (town
                .incident(chapter)
                .is_some_and(|(_, target)| target == lot)
                || matches!(
                    coach::next(town, chapter, self.selected, self.saw_prices),
                    Some(Step::Aid(target, _)) if target == lot
                ))
        {
            Action::AidAt(lot)
        } else {
            Action::LookAtLot(lot)
        }
    }

    pub fn nearby_action(&self, town: &Town, lot: usize) -> Action {
        let selected = self.lot_action(town, lot);
        if selected == Action::LookAtLot(lot) && town.bell_ready(&CHAPTERS[town.chapter], lot) {
            Action::RingBell(lot)
        } else {
            selected
        }
    }

    pub fn visible_town<'a>(&'a self, save: &'a Campaign) -> &'a Town {
        let region = match self.view {
            View::Street => return save.active_town().unwrap_or(&self.preview),
            View::Chart => Region::of(self.chart),
            View::Title => save
                .active_town()
                .map_or(Region::Greece, |town| Region::of(town.chapter)),
        };
        match region {
            Region::Greece => &self.preview,
            Region::Rome => &self.preview_rome,
            Region::Japan => &self.preview_japan,
            Region::Nordic => &self.preview_nordic,
            Region::Oasis => &self.preview_oasis,
        }
    }

    fn transition(&mut self, view: View, panel: Panel) {
        if view != self.view {
            self.world_version += 1;
            self.delivered = None;
            self.job = None;
            self.nearby = None;
            self.destination = None;
            self.street_action = None;
            self.bell_action = None;
            self.loading_cargo = None;
            self.work.fill(None);
        }
        self.view = view;
        self.show(panel);
    }

    fn show(&mut self, panel: Panel) {
        if !matches!(panel, Panel::Build | Panel::Details | Panel::Demolish) {
            self.return_to_guests = false;
        }
        self.panel = panel;
        self.lesson = None;
        self.focus = None;
        self.dirty = true;
    }

    pub fn delivery(&self) -> Option<Delivery> {
        self.delivered
            .filter(|item| self.clock - item.since < DELIVERY_TIME)
    }

    fn offer_lesson(&mut self, save: &Campaign, step: Option<Step>) -> bool {
        if self.work.iter().any(Option::is_some) {
            return false;
        }
        let Some(town) = save.active_town() else {
            return false;
        };
        let delivered = self.delivery().map(|item| item.route);
        let Some(lesson) = coach::lesson(town, step, delivered) else {
            return false;
        };
        self.lesson = Some(lesson);
        self.focus = None;
        self.dirty = true;
        self.pan = Vec2::ZERO;
        self.zoom = 0.0;
        if matches!(town.phase, Phase::Open(_)) {
            self.save(save);
        }
        true
    }

    fn guest_book(&mut self, guest: Option<usize>) {
        self.ledger_page = 1;
        self.review_guest = guest;
        self.show(Panel::Forecast);
        self.focus = Some(Action::LedgerPage(1));
    }

    fn build_effect(&mut self, from: u8) {
        let from = self.work[self.selected].map_or(from, |work| work.from);
        self.work[self.selected] = Some(Construction {
            since: self.clock,
            from,
        });
        self.world_version += 1;
        self.show(Panel::None);
    }

    fn message(&mut self, message: impl Into<String>, cue: Cue) {
        self.note = message.into();
        self.note_left = 3.2;
        self.cue = Some(cue);
    }

    fn save(&mut self, save: &Campaign) {
        if let Err(error) = save.save() {
            warn!("Nililane: {error}");
            self.message(
                "Der Spielstand konnte nicht gespeichert werden.",
                Cue::Blocked,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn frame(
    mut game: ResMut<Game>,
    mut save: ResMut<Campaign>,
    controls: Res<Controls>,
    time: Res<Time>,
    real: Res<Time<Real>>,
    hud: Res<Hud>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<StreetCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    wheel: EventReader<bevy::input::mouse::MouseWheel>,
    touches: Res<Touches>,
    changed: Query<(&Interaction, &ui::ButtonAction), Changed<Interaction>>,
    buttons: ui::FocusButtons,
    walk_pad: ui::WalkPad,
) {
    let dt = time.delta_secs();
    let real_dt = real.delta_secs();
    game.clock += dt;
    let now = game.clock;
    let mut finished = false;
    for lot in &mut game.work {
        if lot.is_some_and(|work| now - work.since >= BUILD_TIME) {
            *lot = None;
            finished = true;
        }
    }
    if finished {
        game.world_version += 1;
    }
    game.note_left = (game.note_left - real_dt).max(0.0);

    let live = game.view == View::Street && game.panel == Panel::None;
    let pressed = changed.iter().find_map(|(state, action)| {
        (*state == Interaction::Pressed && !action.disabled).then_some(action.action)
    });
    game.walking = false;
    let running = game.view == View::Street
        && matches!(
            game.panel,
            Panel::None | Panel::Details | Panel::Nili | Panel::Forecast | Panel::Goals
        )
        && game.lesson.is_none()
        && window.focused;
    if running {
        let before = save.active_town().and_then(day_feedback);
        let previous_step = save.active_town().and_then(|town| {
            coach::next(
                town,
                &CHAPTERS[town.chapter],
                game.selected,
                game.saw_prices,
            )
        });
        let sky_before = save
            .active_town()
            .map(|town| town.sky_phase(&CHAPTERS[town.chapter]));
        let street_before = save.active_town().map(street_slots);
        let incident_before = save
            .active_town()
            .and_then(|town| town.incident(&CHAPTERS[town.chapter]));
        let completed_day = save.active_town_mut().is_some_and(|town| {
            town.tick(
                &CHAPTERS[town.chapter],
                real_dt.min(0.25) * f32::from(game.speed),
            )
        });
        if completed_day {
            finish_day(&mut game, &mut save);
            return;
        }
        let step = save.active_town().and_then(|town| {
            coach::next(
                town,
                &CHAPTERS[town.chapter],
                game.selected,
                game.saw_prices,
            )
        });
        if previous_step != step {
            game.dirty = true;
        }
        let after = save.active_town().and_then(day_feedback);
        if let (
            Some((time_before, sales_before, replies_before)),
            Some((time_after, sales_after, replies_after)),
        ) = (before, after)
        {
            if time_before < 8.0 && time_after >= 8.0 {
                game.dirty = true;
                game.cue = Some(Cue::Ferry);
            } else if sales_after > sales_before {
                game.cue = Some(Cue::Sale);
            }
            if replies_after != replies_before
                && matches!(game.panel, Panel::Details | Panel::Forecast | Panel::Goals)
            {
                game.dirty = true;
            }
        }
        if let Some(town) = save.active_town()
            && (sky_before != Some(town.sky_phase(&CHAPTERS[town.chapter]))
                || street_before != Some(street_slots(town)))
        {
            game.dirty = true;
        }
        if let Some((event, _)) = save
            .active_town()
            .and_then(|town| town.incident(&CHAPTERS[town.chapter]))
            && incident_before.is_none()
        {
            game.message(event.event_name(), Cue::Alert);
            game.dirty = true;
        }
        if after.is_some() {
            game.checkpoint += real_dt;
            if game.checkpoint >= 5.0 {
                game.checkpoint = 0.0;
                game.save(&save);
            }
        }
        if live
            && game.job.is_none()
            && save.active_town().and_then(active_route).is_none()
            && game.offer_lesson(&save, step)
        {
            return;
        }
    }

    game.walk_axis = Vec2::ZERO;
    let can_walk = live
        && game.lesson.is_none()
        && window.focused
        && save
            .active_town()
            .is_some_and(|town| matches!(town.phase, Phase::Open(_)));
    if can_walk {
        let touch = ui::walk_input(&walk_pad, &window, &touches, &mouse, &mut game.walk_pointer);
        let axis = (touch + controls.walk).clamp_length_max(1.0);
        if save.active_town().and_then(active_route).is_some() {
            if controls.got(controls::B) || pressed == Some(Action::CancelErrand) {
                apply(Action::CancelErrand, &mut game, &mut save);
            } else if controls.got(controls::START) || pressed == Some(Action::Pause) {
                apply(Action::Pause, &mut game, &mut save);
            } else if controls.got(controls::Y) || pressed == Some(Action::Speed) {
                apply(Action::Speed, &mut game, &mut save);
            } else {
                drive_errand(
                    &mut game,
                    &mut save,
                    axis,
                    dt,
                    controls.got(controls::A) || pressed == Some(Action::DoErrand),
                );
            }
            return;
        }
        if let Some(town) = save.active_town_mut() {
            walk_day(&mut game, town, axis, dt);
        }
    } else {
        game.walk_pointer = None;
    }

    if let Some(lesson) = game.lesson {
        let accept = controls.got(controls::A)
            || (controls.got(controls::Y) && lesson.prompt == Prompt::Action(Step::StartDay))
            || (controls.got(controls::R3)
                && matches!(lesson.prompt, Prompt::Action(Step::Tour(..))));
        if let Some(Action::Lesson(accepted)) = pressed {
            apply(Action::Lesson(accepted), &mut game, &mut save);
        } else if controls.got(controls::B) {
            apply(Action::Lesson(false), &mut game, &mut save);
        } else if accept {
            apply(Action::Lesson(true), &mut game, &mut save);
        }
        return;
    }
    if let Some(action) = pressed {
        apply(action, &mut game, &mut save);
        return;
    }

    if controls.got(controls::B) {
        apply(Action::Back, &mut game, &mut save);
        return;
    }
    if controls.got(controls::START) && game.view == View::Street {
        if matches!(game.panel, Panel::None | Panel::Pause) {
            apply(Action::Pause, &mut game, &mut save);
        }
        return;
    }
    if game.view == View::Chart && game.panel == Panel::None {
        let shortcuts = [
            (controls::Y, Action::Tutorial),
            (controls::RB, Action::ChartPage(true)),
            (controls::LB, Action::ChartPage(false)),
        ];
        if let Some((_, action)) = shortcuts.into_iter().find(|(key, _)| controls.got(*key)) {
            apply(action, &mut game, &mut save);
            return;
        }
    }

    if game.view == View::Street
        && controls.got(controls::VIEW)
        && matches!(game.panel, Panel::None | Panel::Forecast | Panel::Report)
    {
        let action = if game.panel == Panel::Forecast && game.ledger_page == 1 {
            Action::Back
        } else {
            Action::Feedback
        };
        apply(action, &mut game, &mut save);
        return;
    }

    if (live || game.panel == Panel::Nili)
        && controls.got(controls::R3)
        && save.active_town().is_some_and(|town| town.chapter > 0)
    {
        apply(Action::Nili, &mut game, &mut save);
        return;
    }
    if live {
        let Some(town) = save.active_town() else {
            return;
        };
        let chapter = &CHAPTERS[town.chapter];
        if controls.got(controls::X) {
            apply(Action::Goals, &mut game, &mut save);
            return;
        }
        if controls.got(controls::Y) && game.job.is_none() {
            let action = if town.planning() {
                Action::Open
            } else {
                Action::Speed
            };
            apply(action, &mut game, &mut save);
            return;
        }
        if controls.got(controls::A) {
            let action = if game.job.is_some() {
                Action::Dispatch
            } else if !town.planning() {
                match game.nearby {
                    Some(Nearby::Building(lot)) => game.nearby_action(town, lot),
                    Some(Nearby::Activity(slot, _)) => Action::StreetHelp(slot),
                    None => match game.lot_action(town, game.selected) {
                        action @ Action::AidAt(_) => action,
                        _ => return,
                    },
                }
            } else {
                Action::Plot
            };
            apply(action, &mut game, &mut save);
            return;
        }
        if controls.got(controls::RB) {
            apply(Action::Forecast, &mut game, &mut save);
            return;
        }
        if controls.got(controls::LB) {
            game.zoom = 0.0;
            game.pan = Vec2::ZERO;
        }
        if let Some(dir) = controls.direction
            && (town.planning() || game.job.is_some())
        {
            let row = lot_row(game.selected);
            let col = lot_col(game.selected);
            let (next_row, next_col) = match dir {
                Direction::Left => (row, col.saturating_sub(1)),
                Direction::Right => (row, col + 1),
                Direction::Up => (0, col),
                Direction::Down => (1, col),
            };
            select(
                &mut game,
                lot_id(next_row, next_col.min(chapter.layout.row(next_row) - 1)),
            );
        }
        let mut wheel = wheel;
        let scroll: f32 = wheel
            .read()
            .map(|event| {
                use bevy::input::mouse::MouseScrollUnit::*;
                match event.unit {
                    Line => event.y,
                    Pixel => event.y * 0.03,
                }
            })
            .sum();
        game.zoom = (game.zoom + controls.zoom * dt * 0.85 + scroll * 0.1).clamp(-0.45, 0.8);
        if controls.pan.length_squared() > 0.14 {
            game.pan += Vec2::new(controls.pan.x, -controls.pan.y) * dt * 6.0;
            game.pan = game.pan.clamp(Vec2::new(-5.0, -2.0), Vec2::new(5.0, 3.0));
        }
        // Marked requests take priority when tapping an occupied parcel.
        if game.walk_pointer.is_none()
            && !buttons
                .iter()
                .any(|(_, _, _, state)| *state != Interaction::None)
        {
            let touch = touches
                .iter_just_pressed()
                .next()
                .map(|touch| touch.position());
            let pointer = touch.or_else(|| {
                if mouse.just_pressed(MouseButton::Left) {
                    window.cursor_position()
                } else {
                    None
                }
            });
            if let (Some(pointer), Ok((camera, transform))) = (pointer, camera.single())
                && let Some(lot) = crate::world::pick(camera, transform, pointer, chapter.layout)
            {
                if game.job.is_none() && town.lots[lot].is_some() {
                    let action = game.lot_action(town, lot);
                    apply(action, &mut game, &mut save);
                } else if lot == game.selected && (town.planning() || game.job.is_some()) {
                    let action = if game.job.is_some() {
                        Action::Dispatch
                    } else {
                        Action::Plot
                    };
                    apply(action, &mut game, &mut save);
                } else {
                    select(&mut game, lot);
                }
            }
        }
    } else {
        if buttons.is_empty() {
            return;
        }
        if let Some(dir) = controls.direction {
            game.focus = ui::move_focus(game.focus.or(hud.first), dir, &buttons).or(hud.first);
        }
        if controls.got(controls::A)
            && let Some(action) = game.focus.or(hud.first)
        {
            apply(action, &mut game, &mut save);
        }
    }
}

fn active_route(town: &Town) -> Option<Route> {
    if let Phase::Open(day) = &town.phase {
        day.route
    } else {
        None
    }
}

fn take_control(town: &mut Town, game: &mut Game, resume: bool) {
    let Phase::Open(day) = &mut town.phase else {
        return;
    };
    if resume && day.nili.is_some() {
        return;
    }
    let layout = CHAPTERS[town.chapter].layout;
    let Some(route) = day.route else {
        day.nili.get_or_insert_with(|| NiliControl {
            position: if day.seconds == 0.0 {
                [0.0, 0.10]
            } else {
                motion::ground(motion::stop(day.nili_at, layout), layout).to_array()
            },
            yaw: 0.22,
            stage: ManualStage::Pickup,
        });
        return;
    };
    let current = motion::job_walk(route, day.seconds, layout);
    let stage = match route.stage(day.seconds).0 {
        RouteStage::Approach | RouteStage::Collect => ManualStage::Pickup,
        RouteStage::Carry | RouteStage::Deliver => ManualStage::Delivery,
    };
    let mut nili = day.nili.unwrap_or(NiliControl {
        position: motion::ground(current.at, layout).to_array(),
        yaw: current.yaw,
        stage,
    });
    nili.stage = stage;
    day.nili = Some(nili);
    game.zoom = 0.0;
    game.pan = Vec2::ZERO;
    game.walk_distance = 0.0;
    game.walk_axis = Vec2::ZERO;
    game.walk_pointer = None;
    game.checkpoint = 0.0;
    game.dirty = true;
}

fn street_slots(town: &Town) -> [Option<Activity>; street::SPOTS] {
    street::spots(town, &CHAPTERS[town.chapter]).map(|spot| spot.map(|spot| spot.activity))
}

fn steer(
    game: &mut Game,
    nili: &mut NiliControl,
    axis: Vec2,
    dt: f32,
    layout: crate::layout::Layout,
) {
    game.walk_axis = axis;
    let amount = motion::walk(nili, axis * (4.55 * dt.min(0.25)), layout);
    game.walk_distance += amount;
    game.walking = amount > 0.0001;
}

fn walk_day(game: &mut Game, town: &mut Town, axis: Vec2, dt: f32) {
    let chapter = &CHAPTERS[town.chapter];
    take_control(town, game, true);
    let Phase::Open(day) = &mut town.phase else {
        return;
    };
    let Some(nili) = day.nili.as_mut() else {
        return;
    };
    if !game
        .street_action
        .is_some_and(|(_, _, since)| game.clock - since < 0.50)
    {
        steer(game, nili, axis, dt, chapter.layout);
    }
    let position = nili.position;
    let lot = motion::nearby_lot(town, position, chapter.layout);
    if game.walking
        && game
            .job
            .is_some_and(|job| lot.is_some_and(|lot| job.plan(town, chapter, lot).is_ok()))
        && let Some(lot) = lot
    {
        select(game, lot);
    }
    let nearby = street::spots(town, chapter)
        .iter()
        .enumerate()
        .find_map(|(slot, spot)| {
            spot.filter(|spot| spot.in_range(position))
                .map(|spot| Nearby::Activity(slot, spot.activity))
        })
        .or_else(|| lot.map(Nearby::Building));
    if game.nearby != nearby {
        game.nearby = nearby;
        game.dirty = true;
    }
}

fn drive_errand(game: &mut Game, save: &mut Campaign, axis: Vec2, dt: f32, work: bool) {
    let Some(town) = save.active_town_mut() else {
        return;
    };
    let chapter = &CHAPTERS[town.chapter];
    let Phase::Open(day) = &mut town.phase else {
        return;
    };
    let Some((route, old)) = day.manual_route() else {
        return;
    };
    let nili = day.nili.as_mut().expect("manual route has a position");
    let layout = chapter.layout;
    let dt = dt.min(0.25);
    game.walk_axis = Vec2::ZERO;
    let mut checkpoint = false;
    match nili.stage {
        ManualStage::Pickup | ManualStage::Delivery => {
            steer(game, nili, axis, dt, layout);
            if work {
                if motion::can_work(*nili, route, layout) {
                    let point = motion::stop(nili.target(route), layout).xz();
                    let turn = point - Vec2::from_array(nili.position);
                    if turn.length_squared() > 0.004 {
                        nili.yaw = turn.x.atan2(turn.y);
                    }
                    nili.stage = nili.stage.confirm();
                    game.walking = false;
                    game.cue = Some(Cue::Confirm);
                    checkpoint = true;
                } else {
                    game.message("Geh näher an die Markierung.", Cue::Blocked);
                }
            }
        }
        ManualStage::Collect(progress) => {
            let next = (progress + dt / 0.86).min(1.0);
            nili.stage = if next < 1.0 {
                ManualStage::Collect(next)
            } else {
                checkpoint = true;
                ManualStage::Delivery
            };
        }
        ManualStage::Unload(progress) => {
            nili.stage = ManualStage::Unload((progress + dt / 0.79).min(1.0));
        }
    }
    if checkpoint || motion::can_work(old, route, layout) != motion::can_work(*nili, route, layout)
    {
        game.dirty = true;
    }
    if let Some(route) = town.finish_controlled(chapter) {
        delivery_notice(game, save, route);
        return;
    }
    if checkpoint {
        game.checkpoint = 0.0;
        game.save(save);
    }
}

fn finish_day(game: &mut Game, save: &mut Campaign) {
    game.job = None;
    game.nearby = None;
    game.destination = None;
    game.street_action = None;
    if let Some(town) = save.active_town()
        && let Phase::Report(report) = &town.phase
    {
        let (chapter, day, stars, income) = (
            town.chapter,
            town.day,
            report.stars,
            town.totals.income + town.totals.caravan_income,
        );
        if stars > 0 {
            save.record(chapter, day, stars);
            save.record_income(chapter, income);
        }
        game.cue = Some(if stars > 0 { Cue::Earned } else { Cue::Day });
    }
    game.show(Panel::Report);
    game.save(save);
}

fn delivery_notice(game: &mut Game, save: &Campaign, route: Route) {
    game.delivered = Some(Delivery {
        route,
        since: game.clock,
    });
    if route.favor {
        let town = save.active_town().unwrap();
        let title = if let Phase::Open(day) = &town.phase {
            day.favor.resolved_name()
        } else {
            "Bitte vom Kai erfüllt"
        };
        let currency = Region::of(town.chapter).currency();
        game.message(format!("{title} · +6 {currency} · +2 Ansehen"), Cue::Aid);
    } else {
        game.message(route.job.delivered(), Cue::Aid);
    }
    game.dirty = true;
    game.save(save);
}

fn day_feedback(town: &Town) -> Option<(f32, u32, u32)> {
    match &town.phase {
        Phase::Open(day) => Some((day.seconds, day.tally.served_total(), day.tally.responses())),
        _ => None,
    }
}

fn select(game: &mut Game, lot: usize) {
    if game.selected != lot {
        game.selected = lot;
        game.dirty = true;
    }
}

fn start(chapter: usize, tutorial: bool, game: &mut Game, save: &mut Campaign) {
    let available = if tutorial {
        chapter < coach::ISLANDS
    } else {
        save.unlocked(chapter)
    };
    if !available {
        return;
    }
    let rule = &CHAPTERS[chapter];
    let moored = (!tutorial).then(|| save.take_moored(chapter)).flatten();
    let resuming = moored.is_some();
    let mut town = moored.unwrap_or_else(|| Town::new(chapter, rule));
    let bonus = if tutorial || resuming {
        0
    } else {
        save.travel_bonus(chapter)
    };
    town.coins += bonus;
    game.saw_prices = false;
    game.delivered = None;
    game.bell_action = None;
    game.loading_cargo = None;
    game.selected = coach::next(&town, rule, 1, false)
        .and_then(Step::lot)
        .unwrap_or(1);
    save.enter_town(town, tutorial);
    game.chart = chapter;
    game.job = None;
    game.work.fill(None);
    game.zoom = 0.0;
    game.pan = Vec2::ZERO;
    game.speed = 1;
    game.checkpoint = 0.0;
    game.note_left = 0.0;
    game.transition(View::Street, Panel::Brief);
    if resuming {
        apply(Action::Resume, game, save);
    }
    if bonus > 0 {
        game.message(
            format!("+{bonus} Startmünzen aus dem letzten Hafen"),
            Cue::Earned,
        );
    }
    game.world_version += 1;
    game.save(save);
}

fn open_chart(game: &mut Game, save: &mut Campaign) {
    if save.leave_tutorial() {
        game.chart = save.town.as_ref().map_or(0, |town| town.chapter);
        game.saw_prices = false;
        game.speed = 1;
        game.zoom = 0.0;
        game.pan = Vec2::ZERO;
        game.checkpoint = 0.0;
        game.note_left = 0.0;
        game.save(save);
    } else if let Some(town) = &save.town {
        game.chart = town.chapter;
    }
    game.transition(View::Chart, Panel::None);
    game.focus = Some(Action::Choose(game.chart));
}

fn follow_step(step: Step, game: &mut Game, save: &mut Campaign) {
    if let Some(lot) = step.lot() {
        select(game, lot);
    }
    match step {
        Step::Build(..) | Step::Price(_) => apply(Action::Plot, game, save),
        Step::Upgrade(_) => {
            apply(Action::Plot, game, save);
            game.focus = Some(Action::Upgrade);
        }
        Step::StartDay => apply(Action::Open, game, save),
        Step::Aid(..) => apply(Action::Aid, game, save),
        Step::Tour(_, job) => {
            apply(Action::Nili, game, save);
            game.focus = Some(Action::ChooseJob(job));
        }
        Step::Arrivals | Step::Evening => {}
    }
}

fn apply(action: Action, game: &mut Game, save: &mut Campaign) {
    game.cue = Some(Cue::Confirm);

    match action {
        Action::Begin => start(0, false, game, save),
        Action::Resume => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            game.chart = town.chapter;
            game.selected = if town.planning() {
                coach::next(town, &CHAPTERS[town.chapter], 1, game.saw_prices)
                    .and_then(Step::lot)
                    .unwrap_or(1)
            } else {
                1
            };
            game.job = None;
            game.transition(
                View::Street,
                if matches!(town.phase, Phase::Report(_)) {
                    Panel::Report
                } else {
                    Panel::None
                },
            );
            take_control(town, game, true);
        }
        Action::Chart => open_chart(game, save),
        Action::Title => game.transition(View::Title, Panel::None),
        Action::Tutorial => {
            if game.view == View::Chart && game.panel == Panel::None {
                start(0, true, game, save);
            }
        }
        Action::Choose(chapter) => {
            if game.chart == chapter && save.unlocked(chapter) {
                apply(Action::Sail, game, save);
                return;
            }
            game.chart = chapter;
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::Region(region) => {
            if Region::of(game.chart) != region {
                game.chart = (region.start()..region.start() + REGION_SIZE)
                    .find(|&n| save.records[n].stars == 0)
                    .unwrap_or(region.start() + REGION_SIZE - 1);
            }
            game.focus = Some(Action::Choose(game.chart));
            game.dirty = true;
        }
        Action::ChartPage(next) => {
            let offset = Region::of(game.chart).start();
            let local = game.chart - offset;
            game.chart = offset + if next { CORE_HARBORS } else { 0 };
            if (local >= CORE_HARBORS) != next {
                game.focus = Some(Action::Choose(game.chart));
                game.dirty = true;
            } else {
                game.chart = offset + local;
            }
        }
        Action::Sail => {
            if !save.unlocked(game.chart) {
                return;
            }
            if save.town.as_ref().is_some_and(|town| {
                town.chapter == game.chart
                    && !matches!(&town.phase, Phase::Report(report) if report.stars > 0)
            }) {
                apply(Action::Resume, game, save);
            } else {
                start(game.chart, false, game, save);
            }
        }
        Action::NextChapter => {
            let Some(town) = save.active_town() else {
                return;
            };
            let tutorial = save.in_tutorial();
            if tutorial && town.chapter + 1 == coach::ISLANDS {
                open_chart(game, save);
                game.message("Tutorial abgeschlossen", Cue::Earned);
            } else if let Some(next) = next_harbor(town.chapter) {
                start(next, tutorial, game, save);
            } else {
                open_chart(game, save);
            }
        }
        Action::BonusHarbor => {
            if !save.in_tutorial()
                && let Some(town) = save.active_town()
                && main_route_end(town.chapter)
            {
                start(town.chapter + 1, false, game, save);
            }
        }
        Action::Back => {
            game.cue = Some(Cue::Back);
            if game.panel == Panel::None && game.job.take().is_some() {
                game.show(Panel::Nili);
                return;
            }
            match game.panel {
                Panel::Options | Panel::Guide | Panel::Wardrobe => {
                    let invoking = match game.panel {
                        Panel::Options => Action::Options,
                        Panel::Wardrobe => Action::Wardrobe,
                        _ => Action::Guide,
                    };
                    game.show(if game.view == View::Street {
                        Panel::Pause
                    } else {
                        Panel::None
                    });
                    game.focus = Some(invoking);
                }
                Panel::Details | Panel::Build if game.return_to_guests => {
                    game.guest_book(game.review_guest);
                    game.focus = game.review_guest.map(Action::GuestAdvice);
                }
                Panel::Forecast
                    if save
                        .active_town()
                        .is_some_and(|town| matches!(town.phase, Phase::Report(_))) =>
                {
                    game.show(Panel::Report);
                }
                Panel::Demolish => game.show(Panel::Details),
                Panel::Report => open_chart(game, save),
                Panel::None => match game.view {
                    View::Street => {
                        game.show(Panel::Pause);
                        game.save(save);
                    }
                    View::Chart => game.transition(View::Title, Panel::None),
                    View::Title => {}
                },
                _ => game.show(Panel::None),
            }
        }
        Action::Pause => {
            if game.panel == Panel::Pause {
                game.show(Panel::None);
            } else {
                game.show(Panel::Pause);
                game.save(save);
            }
        }
        Action::Options => game.show(Panel::Options),
        Action::Wardrobe => game.show(Panel::Wardrobe),
        Action::WardrobeTab(category) => {
            game.wardrobe_tab = category;
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::WardrobeRegion(region) => {
            game.wardrobe_region = region;
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::Wear(item) => {
            if game.panel == Panel::Wardrobe && save.equip(item) {
                game.focus = Some(action);
                game.dirty = true;
                game.save(save);
            }
        }
        Action::Guide => {
            game.guide = 0;
            game.show(Panel::Guide);
        }
        Action::GuideTab(page) => {
            game.guide = page;
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::Music | Action::Sounds | Action::LargeUi => {
            match action {
                Action::Music => save.music = !save.music,
                Action::Sounds => save.sounds = !save.sounds,
                _ => save.large_ui = !save.large_ui,
            }
            game.save(save);
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::Plot => {
            if let Some(town) = save.active_town() {
                let layout = CHAPTERS[town.chapter].layout;
                if !motion::can_inspect(town, game.selected, layout) {
                    if let Some(building) = town.lots[game.selected] {
                        game.destination = Some(game.selected);
                        game.show(Panel::None);
                        game.message(
                            format!("Geh mit Nili zu: {}", building.kind.name()),
                            Cue::Back,
                        );
                    }
                    return;
                }
                game.destination = None;
                game.build_page = 0;
                game.return_to_guests = false;
                if town.planning() && coach::recommended_price(town, game.selected).is_some() {
                    game.saw_prices = true;
                }
                game.show(if town.lots[game.selected].is_none() && town.planning() {
                    Panel::Build
                } else {
                    Panel::Details
                });
            }
        }
        Action::Coach(step) => {
            let Some(town) = save.active_town() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            if coach::next(town, chapter, game.selected, game.saw_prices) == Some(step) {
                follow_step(step, game, save);
            }
        }
        Action::Lesson(accepted) => {
            let (Some(lesson), Some(town)) = (game.lesson.take(), save.active_town_mut()) else {
                return;
            };
            town.lessons |= lesson.moment.bit();
            game.focus = None;
            game.dirty = true;
            game.save(save);
            if accepted {
                match lesson.prompt {
                    Prompt::Action(step) => follow_step(step, game, save),
                    Prompt::Feedback(_) => apply(Action::Feedback, game, save),
                    Prompt::Delivered(..) => {}
                }
            }
        }
        Action::SelectLot(lot) | Action::LookAtLot(lot) => {
            let Some(town) = save.active_town() else {
                return;
            };
            if !CHAPTERS[town.chapter].layout.contains(lot) {
                return;
            }
            select(game, lot);
            if matches!(action, Action::LookAtLot(_)) {
                apply(Action::Plot, game, save);
            }
        }
        Action::StreetHelp(slot) => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            let point = street::spots(town, chapter).get(slot).copied().flatten();
            if let Some(activity) = street::help(town, chapter, slot) {
                if let Phase::Open(day) = &mut town.phase
                    && let (Some(nili), Some(point)) = (&mut day.nili, point)
                {
                    let turn = Vec2::from_array(point.position) - Vec2::from_array(nili.position);
                    if turn.length_squared() > 0.001 {
                        nili.yaw = turn.x.atan2(turn.y);
                    }
                }
                game.street_action = Some((slot, activity, game.clock));
                game.nearby = None;
                game.dirty = true;
                game.message(
                    format!("+{} Ansehen bei Tagesende", activity.reward()),
                    Cue::Street(activity),
                );
                game.save(save);
            }
        }
        Action::CancelErrand => {
            if let Some(town) = save.active_town_mut()
                && let Phase::Open(day) = &mut town.phase
                && day.route.take().is_some()
            {
                game.dirty = true;
                game.message("Auftrag abgebrochen", Cue::Back);
                game.save(save);
            }
        }
        Action::AidAt(lot) => {
            if save
                .active_town()
                .is_some_and(|town| town.favor_plan(&CHAPTERS[town.chapter], lot).is_some())
            {
                select(game, lot);
                apply(Action::Aid, game, save);
            }
        }
        Action::RingBell(lot) => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            if motion::can_inspect(town, lot, chapter.layout) && town.ring_bell(chapter, lot) {
                let guests = town.bell_guests(lot);
                game.bell_action = Some((lot, game.clock));
                game.show(Panel::None);
                game.message(
                    format!("Die Glocke ruft {guests} Pilger zum Tempel"),
                    Cue::TempleBell,
                );
                game.save(save);
            }
        }
        Action::SendVoyage(lot, route) => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            if !motion::can_inspect(town, lot, chapter.layout) {
                return;
            }
            match town.send_voyage(chapter, lot, route) {
                Ok(voyage) => {
                    game.loading_cargo = Some((lot, game.clock));
                    game.show(Panel::None);
                    game.message(
                        format!("Schiff abgelegt · zurück ab Tag {}", voyage.arrival),
                        Cue::CastOff,
                    );
                    game.save(save);
                }
                Err(reason) => game.message(reason.hint(), Cue::Blocked),
            }
        }
        Action::SendCaravan(lot, contract) => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            if !motion::can_inspect(town, lot, chapter.layout) {
                return;
            }
            match town.send_caravan(chapter, lot, contract) {
                Ok(pay) => {
                    game.loading_cargo = Some((lot, game.clock));
                    game.show(Panel::None);
                    game.message(
                        format!("Karawane aufgebrochen · +{pay} Dirham"),
                        Cue::CastOff,
                    );
                    game.save(save);
                }
                Err(reason) => game.message(reason.label(), Cue::Blocked),
            }
        }
        Action::Nili => {
            game.job = None;
            game.show(if game.panel == Panel::Nili {
                Panel::None
            } else {
                Panel::Nili
            });
        }
        Action::ChooseJob(job) => {
            let Some(town) = save.active_town() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            let target = std::iter::once(game.selected)
                .chain(chapter.layout.lots())
                .find(|&lot| job.plan(town, chapter, lot).is_ok());
            if let Some(lot) = target {
                game.job = Some(job);
                select(game, lot);
                game.show(Panel::None);
            }
        }
        Action::Dispatch => {
            let (Some(town), Some(job)) = (save.active_town_mut(), game.job) else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            match town.dispatch(chapter, job, game.selected) {
                Ok(_) => {
                    game.job = None;
                    game.show(Panel::None);
                    take_control(town, game, false);
                    game.save(save);
                }
                Err(reason) => game.message(reason.label(job), Cue::Blocked),
            }
        }
        Action::Goals => game.show(Panel::Goals),
        Action::Forecast => {
            game.ledger_page = 0;
            game.review_guest = None;
            game.review_reason = None;
            game.show(Panel::Forecast);
            game.focus = Some(Action::LedgerPage(0));
        }
        Action::TomorrowNotes => {
            game.ledger_page = 3;
            game.show(Panel::Forecast);
            game.focus = Some(Action::LedgerPage(3));
        }
        Action::Feedback => {
            game.review_reason = None;
            game.guest_book(None);
        }
        Action::GuestReview(guest) => {
            if save.active_town().is_some_and(|town| {
                town.guest_history()
                    .get(guest)
                    .is_some_and(|v| v.verdict().is_some())
            }) {
                game.review_reason = None;
                game.guest_book(Some(guest));
            }
        }
        Action::GuestAdvice(guest) => {
            let Some(town) = save.active_town() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            let Some(tip) = town.guest_tip(chapter, guest) else {
                return;
            };
            game.review_guest = Some(guest);
            match tip {
                GuestTip::Shop(lot) => {
                    select(game, lot);
                    apply(Action::Plot, game, save);
                }
                GuestTip::Build(lot, kind) => {
                    if !town.planning() {
                        game.message("Neue Gebäude kannst du morgen früh bauen.", Cue::Back);
                        return;
                    }
                    select(game, lot);
                    game.build_page = usize::from(
                        chapter
                            .allowed()
                            .iter()
                            .skip(8)
                            .any(|&choice| choice == kind),
                    );
                    game.show(Panel::Build);
                    if town.coins >= kind.cost() {
                        game.focus = Some(Action::Build(kind));
                    }
                }
            }
            game.return_to_guests = matches!(game.panel, Panel::Details | Panel::Build);
        }
        Action::ReviewReason(reason) => {
            game.review_guest = None;
            game.review_reason = if game.review_reason == Some(reason) {
                None
            } else {
                Some(reason)
            };
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::LedgerPage(page) => {
            game.ledger_page = page;
            if page != 1 {
                game.review_guest = None;
            }
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::BuildPage(page) => {
            game.build_page = page;
            game.focus = Some(action);
            game.dirty = true;
        }
        Action::Demolish => game.show(Panel::Demolish),
        Action::Speed => {
            game.speed = if game.speed == 1 { 2 } else { 1 };
            game.dirty = true;
        }
        edit => {
            let Some(town) = save.active_town_mut() else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            let ok = match edit {
                Action::Build(kind) => town.build(chapter, game.selected, kind),
                Action::Upgrade => town.upgrade(game.selected),
                Action::Price(price) => town.set_price(game.selected, price),
                Action::Assortment(value) => town.set_assortment(game.selected, value),
                Action::HarborPlan(plan) => town.set_harbor(chapter, plan),
                Action::ConfirmDemolish => town.remove(game.selected),
                Action::Open => town.open(chapter),
                Action::Aid => town.aid(chapter, game.selected),
                Action::NextDay => town.advance(),
                _ => false,
            };
            if ok {
                match edit {
                    Action::Build(kind) => {
                        game.build_effect(0);
                        game.message(format!("Neubau · {}", kind.name()), Cue::Build);
                    }
                    Action::Upgrade => {
                        let building =
                            town.lots[game.selected].expect("an upgrade requires a building");
                        game.build_effect(building.level - 1);
                        game.message(
                            format!("{} · Stufe {}", building.kind.name(), building.level),
                            Cue::Hammer,
                        );
                    }
                    Action::ConfirmDemolish => {
                        game.work[game.selected] = None;
                        game.world_version += 1;
                        game.show(Panel::None);
                        game.message("Der Bauplatz ist wieder frei.", Cue::Back);
                    }
                    Action::Aid => {
                        game.show(Panel::None);
                        take_control(town, game, false);
                    }
                    Action::Open | Action::NextDay => {
                        game.speed = 1;
                        game.delivered = None;
                        game.destination = None;
                        game.street_action = None;
                        game.bell_action = None;
                        game.loading_cargo = None;
                        if edit == Action::Open {
                            take_control(town, game, false);
                        } else if town.cargo_today > 0 {
                            game.message(
                                format!("Der Handel ist zurück · +{} Silber", town.cargo_today),
                                Cue::Earned,
                            );
                        }
                        if edit == Action::NextDay
                            && let Some(lot) =
                                coach::next(town, chapter, game.selected, game.saw_prices)
                                    .and_then(Step::lot)
                        {
                            select(game, lot);
                        }
                        game.show(Panel::None);
                    }
                    Action::HarborPlan(_) | Action::Price(_) | Action::Assortment(_) => {
                        game.focus = Some(edit);
                    }
                    _ => {}
                }
                game.dirty = true;
                game.save(save);
            } else {
                game.message("Das geht im Moment nicht.", Cue::Blocked);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Track {
    Menu(Region),
    Harbor { chapter: usize, evening: bool },
}

impl Track {
    fn asset(self) -> String {
        match self {
            Self::Menu(Region::Greece) => "embedded://audio/aegean.ogg".to_owned(),
            Self::Menu(Region::Rome) => "embedded://audio/roman_quay.ogg".to_owned(),
            Self::Menu(Region::Japan) => Self::Harbor {
                chapter: JAPAN_START,
                evening: true,
            }
            .asset(),
            Self::Menu(Region::Nordic) => Self::Harbor {
                chapter: NORDIC_START,
                evening: true,
            }
            .asset(),
            Self::Menu(Region::Oasis) => Self::Harbor {
                chapter: OASIS_START,
                evening: true,
            }
            .asset(),
            Self::Harbor { chapter, evening } => {
                let suffix = if evening { "-abend" } else { "" };
                let source = if cfg!(target_os = "android") {
                    ""
                } else {
                    "embedded://"
                };
                format!("{source}audio/harbors/{chapter:02}{suffix}.ogg")
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct Music {
    track: Track,
    gain: f32,
}

#[derive(Component)]
pub(crate) struct Coast;

#[derive(Component)]
pub(crate) struct RainAudio;

type MusicVoices<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static mut Music, Option<&'static mut AudioSink>),
    (Without<Coast>, Without<RainAudio>),
>;

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn audio(
    mut commands: Commands,
    mut game: ResMut<Game>,
    save: Res<Campaign>,
    assets: Res<AssetServer>,
    mut music: MusicVoices,
    time: Res<Time<Real>>,
    mut coast: Query<Option<&mut AudioSink>, (With<Coast>, Without<RainAudio>)>,
    mut rain: Query<Option<&mut AudioSink>, (With<RainAudio>, Without<Coast>)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let town = game.visible_town(&save);
    let region = Region::of(town.chapter);
    let in_harbor = game.view == View::Street;
    let track = if in_harbor {
        Track::Harbor {
            chapter: town.chapter,
            evening: CHAPTERS[town.chapter].shift.night()
                || town.sky_phase(&CHAPTERS[town.chapter]) == 2,
        }
    } else {
        Track::Menu(region)
    };
    let (before, after, blend) = if in_harbor {
        town.sky_blend(&CHAPTERS[town.chapter])
    } else {
        (Weather::Clear, Weather::Clear, 1.0)
    };
    let swell = before.swell().lerp(after.swell(), blend);
    let rainfall = before.rain_strength().lerp(after.rain_strength(), blend);
    let focused = window.focused;
    let audible = focused && save.music;
    let has_track = music.iter().any(|(_, item, _)| item.track == track);
    let track_ready = music
        .iter()
        .any(|(_, item, sink)| item.track == track && sink.is_some());
    if !has_track {
        let mut settings = PlaybackSettings::LOOP.with_volume(Volume::Linear(0.0));
        settings.paused = true;
        commands.spawn((
            Music { track, gain: 0.0 },
            AudioPlayer::new(assets.load(track.asset())),
            settings,
        ));
    }
    let incident = in_harbor
        .then(|| town.incident(&CHAPTERS[town.chapter]))
        .flatten();
    let song_volume = match incident {
        Some((model::Favor::LostGuests, _)) => 0.37,
        Some(_) => 0.31,
        None => 0.43,
    };
    // Wait for the incoming asset, then fade the outgoing instruments before
    // the new arrangement starts.
    let fading_out = music
        .iter()
        .any(|(_, item, sink)| item.track != track && item.gain > 0.0 && sink.is_some());
    let elapsed = time.delta_secs().min(0.1);
    for (entity, mut item, sink) in &mut music {
        let current = item.track == track;
        let Some(mut sink) = sink else {
            if !current {
                commands.entity(entity).despawn();
            }
            continue;
        };
        let ready_to_play = audible && if current { !fading_out } else { !track_ready };
        let goal = if ready_to_play { 1.0 } else { 0.0 };
        let delta = (goal - item.gain).clamp(-elapsed / 0.55, elapsed / 0.72);
        item.gain += delta;
        if !current && track_ready && (!audible || item.gain <= 0.0) {
            commands.entity(entity).despawn();
            continue;
        }
        let fade = item.gain * item.gain * (3.0 - 2.0 * item.gain);
        let mix = sink.volume().to_linear().lerp(song_volume, elapsed * 8.0);
        // Use a separate envelope for the handoff, so a new track starts silently.
        sink.set_volume(Volume::Linear(if item.gain < 1.0 {
            song_volume * fade
        } else {
            mix
        }));
        if audible && item.gain > 0.0 {
            sink.play();
        } else {
            sink.pause();
        }
    }
    match coast.single_mut() {
        Err(_) => {
            commands.spawn((
                Coast,
                AudioPlayer::new(assets.load("embedded://audio/shore_loop.wav")),
                PlaybackSettings::LOOP.with_volume(Volume::Linear(0.42)),
            ));
        }
        Ok(Some(mut sink)) => {
            if save.sounds && window.focused {
                sink.set_volume(Volume::Linear(0.35 + swell * 0.28));
                sink.play();
            } else {
                sink.pause();
            }
        }
        Ok(None) => {}
    }
    match rain.single_mut() {
        Err(_) => {
            let mut settings = PlaybackSettings::LOOP.with_volume(Volume::Linear(0.32));
            settings.paused = true;
            commands.spawn((
                RainAudio,
                AudioPlayer::new(assets.load("embedded://audio/harbor_rain.ogg")),
                settings,
            ));
        }
        Ok(Some(mut sink)) => {
            if save.sounds && focused && rainfall > 0.01 {
                sink.set_volume(Volume::Linear(0.51 * rainfall));
                sink.play();
            } else {
                sink.pause();
            }
        }
        Ok(None) => {}
    }
    if let Some(cue) = game.cue.take().filter(|_| save.sounds && focused) {
        let file = match cue {
            Cue::Confirm => "embedded://audio/ui_choose.wav",
            Cue::Back => "embedded://audio/ui_back.wav",
            Cue::Blocked => "embedded://audio/ui_blocked.wav",
            Cue::Earned => "embedded://audio/chapter_chime.wav",
            Cue::Day => "embedded://audio/quest_accepted.wav",
            Cue::Build => "embedded://audio/build_place.wav",
            Cue::Hammer => "embedded://audio/tier_assembly.wav",
            Cue::Ferry => "embedded://audio/ferry_bell.wav",
            Cue::Alert => "embedded://audio/street_bell.wav",
            Cue::Street(Activity::Cat) => "embedded://audio/street_purr.wav",
            Cue::Street(Activity::Dog) => "embedded://audio/street_woof.wav",
            Cue::Street(_) => "embedded://audio/street_sweep.wav",
            Cue::Aid => "embedded://audio/nili_aid.wav",
            Cue::Sale => "embedded://audio/market_sale.wav",
            Cue::TempleBell => "embedded://audio/temple_bell.wav",
            Cue::CastOff => "embedded://audio/cast_off.wav",
        };
        commands.spawn((
            AudioPlayer::new(assets.load(file)),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.35)),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_marked_request_starts_from_nilis_current_position() {
        let chapter = &CHAPTERS[0];
        let mut town = Town::new(0, chapter);
        let mut game = Game {
            selected: 1,
            ..Game::default()
        };
        assert!(town.build(chapter, 2, Kind::Bakery));
        assert!(town.open(chapter));
        take_control(&mut town, &mut game, false);
        assert_eq!(game.lot_action(&town, 2), Action::LookAtLot(2));
        assert!(!town.tick(chapter, 9.0));
        assert_eq!(game.lot_action(&town, 2), Action::AidAt(2));
        assert_eq!(game.lot_action(&town, 1), Action::LookAtLot(1));
        assert!(town.aid(chapter, 2));
        take_control(&mut town, &mut game, false);
        let Phase::Open(day) = &town.phase else {
            unreachable!()
        };
        let (route, nili) = day.manual_route().unwrap();
        assert_eq!(route.delivery, crate::errands::Stop::Parcel(2));
        assert_eq!(nili.position, [0.0, 0.10]);
        assert!(matches!(nili.stage, ManualStage::Pickup));
    }
}

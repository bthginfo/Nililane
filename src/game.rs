use bevy::{
    audio::{AudioSink, AudioSinkPlayback, Volume},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    campaign::{CHAPTERS, REGION_SIZE, Region},
    coach::{self, Step},
    controls::{self, Controls, Direction},
    errands::{Job, Route},
    layout::{lot_col, lot_id, lot_row},
    model::{self, GuestTip, HarborPlan, Kind, LOTS, Miss, Phase, Price, Town, Weather},
    save::Campaign,
    ui::{self, Hud},
    world::StreetCamera,
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
    Guide,
    Demolish,
    Replace,
    Report,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Begin,
    Resume,
    Chart,
    Title,
    Choose(usize),
    Region(Region),
    ChartPage(bool),
    Sail,
    ConfirmSail,
    NextChapter,
    Back,
    Pause,
    Options,
    Guide,
    GuideTab(usize),
    Music,
    Sounds,
    LargeUi,
    Plot,
    Coach(Step),
    SelectLot(usize),
    LookAtLot(usize),
    Goals,
    Forecast,
    Feedback,
    GuestReview(usize),
    GuestAdvice(usize),
    ReviewReason(Miss),
    LedgerPage(usize),
    HarborPlan(HarborPlan),
    Build(Kind),
    BuildPage(usize),
    Upgrade,
    Price(Price),
    Demolish,
    ConfirmDemolish,
    Open,
    Aid,
    Nili,
    ChooseJob(Job),
    Dispatch,
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
    Aid,
    Sale,
}

pub const BUILD_TIME: f32 = 1.58;

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
    pub build_page: usize,
    pub ledger_page: usize,
    pub review_guest: Option<usize>,
    pub review_reason: Option<Miss>,
    pub return_to_guests: bool,
    pub job: Option<Job>,
    pub saw_prices: bool,
    pub focus: Option<Action>,
    pub dirty: bool,
    pub world_version: u32,
    pub note: String,
    pub note_left: f32,
    pub speed: u8,
    pub zoom: f32,
    pub pan: Vec2,
    pub clock: f32,
    pub work: [Option<Construction>; LOTS],
    pub preview: Town,
    preview_rome: Town,
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
        let mut preview_rome = Town::new(CHAPTERS.len() - 1, &CHAPTERS[CHAPTERS.len() - 1]);
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
        Self {
            view: View::Title,
            panel: Panel::None,
            selected: 0,
            chart: 0,
            guide: 0,
            build_page: 0,
            ledger_page: 0,
            review_guest: None,
            review_reason: None,
            return_to_guests: false,
            job: None,
            saw_prices: false,
            focus: None,
            dirty: true,
            world_version: 0,
            note: String::new(),
            note_left: 0.0,
            speed: 1,
            zoom: 0.0,
            pan: Vec2::ZERO,
            clock: 0.0,
            work: [None; LOTS],
            preview,
            preview_rome,
            checkpoint: 0.0,
            cue: None,
        }
    }
}

impl Game {
    pub fn visible_town<'a>(&'a self, save: &'a Campaign) -> &'a Town {
        let region = match self.view {
            View::Street => return save.town.as_ref().unwrap_or(&self.preview),
            View::Chart => Region::of(self.chart),
            View::Title => save
                .town
                .as_ref()
                .map_or(Region::Greece, |town| Region::of(town.chapter)),
        };
        match region {
            Region::Greece => &self.preview,
            Region::Rome => &self.preview_rome,
        }
    }

    fn transition(&mut self, view: View, panel: Panel) {
        if view != self.view {
            self.world_version += 1;
            self.job = None;
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
        self.focus = None;
        self.dirty = true;
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
    if live && game.job.is_none() && window.focused {
        let before = save.town.as_ref().and_then(day_feedback);
        let previous_step = save.town.as_ref().and_then(|town| {
            coach::next(
                town,
                &CHAPTERS[town.chapter],
                game.selected,
                game.saw_prices,
            )
        });
        let dispatched = save.town.as_ref().and_then(active_route);
        let completed_day = save.town.as_mut().is_some_and(|town| {
            town.tick(
                &CHAPTERS[town.chapter],
                real_dt.min(1.0) * f32::from(game.speed),
            )
        });
        if completed_day {
            let town = save.town.as_ref().expect("a day belongs to a town");
            if let Phase::Report(report) = &town.phase {
                let (chapter, day, stars) = (town.chapter, town.day, report.stars);
                if stars > 0 {
                    save.record(chapter, day, stars);
                }
                game.cue = Some(if stars > 0 { Cue::Earned } else { Cue::Day });
            }
            game.show(Panel::Report);
            game.save(&save);
            return;
        }
        let step = save.town.as_ref().and_then(|town| {
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
        let after = save.town.as_ref().and_then(day_feedback);
        if let (Some((time_before, sales_before)), Some((time_after, sales_after))) =
            (before, after)
        {
            if time_before < 8.0 && time_after >= 8.0 {
                game.dirty = true;
                game.cue = Some(Cue::Ferry);
            } else if sales_after > sales_before {
                game.cue = Some(Cue::Sale);
            }
        }
        if let Some(route) = dispatched
            && save.town.as_ref().and_then(active_route).is_none()
        {
            game.message(route.job.delivered(), Cue::Aid);
            game.dirty = true;
            game.save(&save);
        }
        if after.is_some() {
            game.checkpoint += real_dt;
            if game.checkpoint >= 5.0 {
                game.checkpoint = 0.0;
                game.save(&save);
            }
        }
    }

    let pressed = changed.iter().find_map(|(state, action)| {
        (*state == Interaction::Pressed && !action.disabled).then_some(action.action)
    });
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
    if game.view == View::Chart
        && game.panel == Panel::None
        && (controls.got(controls::LB) || controls.got(controls::RB))
    {
        apply(
            Action::ChartPage(controls.got(controls::RB)),
            &mut game,
            &mut save,
        );
        return;
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
        && save.town.as_ref().is_some_and(|town| town.chapter > 0)
    {
        apply(Action::Nili, &mut game, &mut save);
        return;
    }
    if live {
        let Some(town) = &save.town else { return };
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
            } else if town.can_aid(game.selected) {
                Action::Aid
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
        if let Some(dir) = controls.direction {
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
        // Clicking a visible parcel focuses it. A second tap or the action button opens it.
        if !buttons
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
                if lot == game.selected {
                    let action = if game.job.is_some() {
                        Action::Dispatch
                    } else if town.can_aid(lot) {
                        Action::Aid
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

fn day_feedback(town: &Town) -> Option<(f32, u32)> {
    match &town.phase {
        Phase::Open(day) => Some((day.seconds, day.tally.served_total())),
        _ => None,
    }
}

fn select(game: &mut Game, lot: usize) {
    if game.selected != lot {
        game.selected = lot;
        game.dirty = true;
    }
}

fn start(chapter: usize, game: &mut Game, save: &mut Campaign) {
    if !save.unlocked(chapter) {
        return;
    }
    let rule = &CHAPTERS[chapter];
    let town = Town::new(chapter, rule);
    game.saw_prices = false;
    game.selected = coach::next(&town, rule, 1, false)
        .and_then(Step::lot)
        .unwrap_or(1);
    save.town = Some(town);
    game.chart = chapter;
    game.job = None;
    game.work.fill(None);
    game.zoom = 0.0;
    game.pan = Vec2::ZERO;
    game.speed = 1;
    game.transition(View::Street, Panel::Brief);
    game.world_version += 1;
    game.save(save);
}

fn apply(action: Action, game: &mut Game, save: &mut Campaign) {
    game.cue = Some(Cue::Confirm);

    match action {
        Action::Begin => start(0, game, save),
        Action::Resume => {
            let Some(town) = &save.town else { return };
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
        }
        Action::Chart => {
            if let Some(town) = &save.town {
                game.chart = town.chapter;
            }
            game.transition(View::Chart, Panel::None);
            game.focus = Some(Action::Choose(game.chart));
        }
        Action::Title => game.transition(View::Title, Panel::None),
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
            game.chart = offset + if next { 12 } else { 0 };
            if (local >= 12) != next {
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
            } else if save.town.as_ref().is_some_and(
                |town| !matches!(&town.phase, Phase::Report(report) if report.stars > 0),
            ) {
                game.show(Panel::Replace);
            } else {
                start(game.chart, game, save);
            }
        }
        Action::ConfirmSail => start(game.chart, game, save),
        Action::NextChapter => {
            let Some(town) = &save.town else { return };
            if town.chapter + 1 == CHAPTERS.len() {
                game.transition(View::Chart, Panel::None);
            } else {
                start(town.chapter + 1, game, save);
            }
        }
        Action::Back => {
            game.cue = Some(Cue::Back);
            if game.panel == Panel::None && game.job.take().is_some() {
                game.show(Panel::Nili);
                return;
            }
            match game.panel {
                Panel::Options | Panel::Guide => {
                    let invoking = if game.panel == Panel::Options {
                        Action::Options
                    } else {
                        Action::Guide
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
                        .town
                        .as_ref()
                        .is_some_and(|town| matches!(town.phase, Phase::Report(_))) =>
                {
                    game.show(Panel::Report);
                }
                Panel::Demolish => game.show(Panel::Details),
                Panel::Report => game.transition(View::Chart, Panel::None),
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
            if let Some(town) = &save.town {
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
            let Some(town) = &save.town else { return };
            let chapter = &CHAPTERS[town.chapter];
            if coach::next(town, chapter, game.selected, game.saw_prices) == Some(step) {
                if let Some(lot) = step.lot() {
                    select(game, lot);
                }
                match step {
                    Step::Build(_, _) | Step::Price(_) => apply(Action::Plot, game, save),
                    Step::StartDay => apply(Action::Open, game, save),
                    Step::Aid(_, _) => apply(Action::Aid, game, save),
                    Step::Water(_) => apply(Action::ChooseJob(Job::Water), game, save),
                    Step::Arrivals | Step::Evening => {}
                }
            }
        }
        Action::SelectLot(lot) | Action::LookAtLot(lot) => {
            let Some(town) = &save.town else { return };
            if !CHAPTERS[town.chapter].layout.contains(lot) {
                return;
            }
            select(game, lot);
            if matches!(action, Action::LookAtLot(_)) {
                apply(Action::Plot, game, save);
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
            let Some(town) = &save.town else { return };
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
            let (Some(town), Some(job)) = (&mut save.town, game.job) else {
                return;
            };
            let chapter = &CHAPTERS[town.chapter];
            match town.dispatch(chapter, job, game.selected) {
                Ok(route) => {
                    game.job = None;
                    game.show(Panel::None);
                    game.message(
                        format!(
                            "Nili ist unterwegs · {} → {}",
                            route.pickup.label(town),
                            route.delivery.label(town)
                        ),
                        Cue::Confirm,
                    );
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
        Action::Feedback => {
            game.review_reason = None;
            game.guest_book(None);
        }
        Action::GuestReview(guest) => {
            if save.town.as_ref().is_some_and(|town| {
                town.guest_history()
                    .get(guest)
                    .is_some_and(|v| v.verdict().is_some())
            }) {
                game.review_reason = None;
                game.guest_book(Some(guest));
            }
        }
        Action::GuestAdvice(guest) => {
            let Some(town) = &save.town else { return };
            let chapter = &CHAPTERS[town.chapter];
            let Some(tip) = town.guest_tip(chapter, guest) else {
                return;
            };
            game.review_guest = Some(guest);
            match tip {
                GuestTip::Shop(lot) => {
                    select(game, lot);
                    game.show(Panel::Details);
                }
                GuestTip::Build(lot, kind) => {
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
            game.return_to_guests = true;
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
            let Some(town) = &mut save.town else { return };
            let chapter = &CHAPTERS[town.chapter];
            let ok = match edit {
                Action::Build(kind) => town.build(chapter, game.selected, kind),
                Action::Upgrade => town.upgrade(game.selected),
                Action::Price(price) => town.set_price(game.selected, price),
                Action::HarborPlan(plan) => town.set_harbor(chapter, plan),
                Action::ConfirmDemolish => town.remove(game.selected),
                Action::Open => town.open(chapter),
                Action::Aid => town.aid(game.selected),
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
                    Action::Aid => game.message(
                        format!(
                            "Nili hat geholfen. +6 {} · +2 Ansehen",
                            Region::of(town.chapter).currency()
                        ),
                        Cue::Aid,
                    ),
                    Action::Open | Action::NextDay => {
                        game.speed = 1;
                        if edit == Action::NextDay
                            && let Some(lot) =
                                coach::next(town, chapter, game.selected, game.saw_prices)
                                    .and_then(Step::lot)
                        {
                            select(game, lot);
                        }
                        game.show(Panel::None);
                    }
                    Action::HarborPlan(_) | Action::Price(_) => game.focus = Some(edit),
                    _ => {}
                }
                game.dirty = true;
                game.save(save);
            } else {
                game.message("Das geht gerade noch nicht.", Cue::Blocked);
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct Music(Region);

#[derive(Component)]
pub(crate) struct Coast;

#[derive(Component)]
pub(crate) struct RainAudio;

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn audio(
    mut commands: Commands,
    mut game: ResMut<Game>,
    save: Res<Campaign>,
    assets: Res<AssetServer>,
    music: Query<(Entity, &Music, Option<&AudioSink>), (Without<Coast>, Without<RainAudio>)>,
    mut coast: Query<Option<&mut AudioSink>, (With<Coast>, Without<RainAudio>)>,
    mut rain: Query<Option<&mut AudioSink>, (With<RainAudio>, Without<Coast>)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let town = game.visible_town(&save);
    let region = Region::of(town.chapter);
    let weather = if game.view == View::Street {
        town.weather(&CHAPTERS[town.chapter])
    } else {
        Weather::Clear
    };
    let spawn_track = |commands: &mut Commands| {
        let path = match region {
            Region::Greece => "embedded://audio/aegean.ogg",
            Region::Rome => "embedded://audio/roman_quay.ogg",
        };
        let mut settings = PlaybackSettings::LOOP.with_volume(Volume::Linear(0.30));
        settings.paused = !window.focused || !save.music;
        commands.spawn((Music(region), AudioPlayer::new(assets.load(path)), settings));
    };
    match music.single() {
        Err(_) => spawn_track(&mut commands),
        Ok((entity, playing, _)) if playing.0 != region => {
            commands.entity(entity).despawn();
            spawn_track(&mut commands);
        }
        Ok((_, _, Some(sink))) => {
            if save.music && window.focused {
                sink.play();
            } else {
                sink.pause();
            }
        }
        _ => {}
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
                sink.set_volume(Volume::Linear(0.35 + weather.swell() * 0.28));
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
            if save.sounds && window.focused && matches!(weather, Weather::Rain | Weather::Storm) {
                sink.set_volume(Volume::Linear(if weather == Weather::Storm {
                    0.51
                } else {
                    0.33
                }));
                sink.play();
            } else {
                sink.pause();
            }
        }
        Ok(None) => {}
    }
    if let Some(cue) = game
        .cue
        .take()
        .filter(|_| save.sounds && music.single().is_ok_and(|(_, _, sink)| sink.is_some()))
    {
        let file = match cue {
            Cue::Confirm => "embedded://audio/ui_choose.wav",
            Cue::Back => "embedded://audio/ui_back.wav",
            Cue::Blocked => "embedded://audio/ui_blocked.wav",
            Cue::Earned => "embedded://audio/chapter_chime.wav",
            Cue::Day => "embedded://audio/quest_accepted.wav",
            Cue::Build => "embedded://audio/build_place.wav",
            Cue::Hammer => "embedded://audio/tier_assembly.wav",
            Cue::Ferry => "embedded://audio/ferry_bell.wav",
            Cue::Aid => "embedded://audio/nili_aid.wav",
            Cue::Sale => "embedded://audio/market_sale.wav",
        };
        commands.spawn((
            AudioPlayer::new(assets.load(file)),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.35)),
        ));
    }
}

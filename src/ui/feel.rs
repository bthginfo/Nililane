// Motion and feedback for the interface. The HUD is rebuilt whenever the game
// marks it dirty, so every animation here is keyed to what is shown (a dialog
// title, a toast number, a report day) and keeps its clock outside the
// rebuilt entities; a rebuild never replays an animation.
use std::hash::{DefaultHasher, Hash, Hasher};

use super::*;
use crate::{controls, game::Cue};

/// Frame steps are capped so a single loading hitch does not skip the motion.
const MAX_STEP: f32 = 0.1;
/// How long a dialog takes to settle.
const OPEN_TIME: f32 = 0.18;
/// How long the scene curtain takes to lift.
const CURTAIN_TIME: f32 = 0.35;
const TOAST_SLIDE: f32 = 16.0;

/// Per-button motion state: the spring for the scale, the eased colours and
/// the horizontal shake of a refused tap.
#[derive(Component)]
pub(crate) struct Ease {
    pub ready: bool,
    pub scale: f32,
    pub speed: f32,
    pub fill: Color,
    pub border: Color,
    shake: f32,
    rest: Option<Val>,
}

impl Default for Ease {
    fn default() -> Self {
        Self {
            ready: false,
            scale: 1.0,
            speed: 0.0,
            fill: CARD,
            border: LINE,
            shake: 0.0,
            rest: None,
        }
    }
}

impl Ease {
    /// A slightly underdamped spring: presses and pop-ins overshoot a little
    /// and settle within about a third of a second, at any frame rate.
    pub fn spring(&mut self, target: f32, dt: f32) {
        const OMEGA: f32 = 24.0;
        const DAMPING: f32 = 2.0 * 0.58 * OMEGA;
        if (target - self.scale).abs() < 0.0005 && self.speed.abs() < 0.005 {
            self.scale = target;
            self.speed = 0.0;
            return;
        }
        let steps = (dt * 240.0).ceil().max(1.0);
        let h = dt / steps;
        for _ in 0..steps as usize {
            let pull = OMEGA * OMEGA * (target - self.scale) - DAMPING * self.speed;
            self.speed += pull * h;
            self.scale += self.speed * h;
        }
    }

    /// Starts a pop-in: the node springs up from 60 % of its size.
    pub fn pop(&mut self) {
        self.ready = true;
        self.scale = 0.6;
        self.speed = 0.0;
    }

    /// Starts a short sideways shake from the node's current offset.
    fn refuse(&mut self, left: Val) {
        if matches!(left, Val::Auto | Val::Px(_)) {
            if self.rest.is_none() {
                self.rest = Some(left);
            }
            self.shake = 0.32;
        }
    }

    pub fn shake(&mut self, node: &mut Node, dt: f32) {
        let Some(rest) = self.rest else {
            return;
        };
        self.shake = (self.shake - dt).max(0.0);
        if self.shake == 0.0 {
            self.rest = None;
            node.left = rest;
            return;
        }
        let base = if let Val::Px(x) = rest { x } else { 0.0 };
        let fade = self.shake / 0.32;
        let offset = (self.shake * 2.0 * std::f32::consts::PI * 11.0).sin() * 6.0 * fade;
        node.left = Val::Px(base + offset);
    }
}

/// The dimmed layer behind a dialog; the key identifies the dialog.
#[derive(Component)]
pub(crate) struct Veil(pub u64);

/// The shade of the HUD root, faded in when a dialog appears.
#[derive(Component)]
pub(crate) struct Backdrop(pub Color);

/// The toast row with its resting distance from the top.
#[derive(Component)]
pub(crate) struct ToastSlot(pub f32);

#[derive(Component)]
pub(crate) struct ToastText;

/// A full-screen layer above the HUD that hides scene changes.
#[derive(Component)]
pub(crate) struct Curtain;

/// One star of the evening report.
#[derive(Component)]
pub(crate) struct RevealStar {
    key: u64,
    index: u8,
    earned: bool,
}

/// A report figure that counts up to its value.
#[derive(Component)]
pub(crate) struct Counter {
    key: u64,
    value: u32,
    prefix: &'static str,
    suffix: &'static str,
}

/// A stable key for a dialog title.
pub(crate) fn key(name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish() | 1
}

/// The reveal key of an evening report.
pub(crate) fn report_key(chapter: usize, day: u32) -> u64 {
    ((chapter as u64) << 32 | u64::from(day)) + 1
}

fn ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

fn smooth(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Grows from nothing past full size and settles: 0 → 1.2 → 1.
fn pop(t: f32) -> f32 {
    if t <= 0.0 {
        0.001
    } else if t < 0.16 {
        1.2 * ease_out(t / 0.16)
    } else {
        1.2 - 0.2 * smooth((t - 0.16) / 0.16)
    }
}

pub(super) fn spawn_curtain(commands: &mut Commands) {
    commands.spawn((
        Curtain,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(INK),
        GlobalZIndex(900),
        FocusPolicy::Pass,
    ));
}

pub(super) fn star(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    key: u64,
    index: u8,
    earned: bool,
    size: f32,
) {
    p.spawn((
        RevealStar { key, index, earned },
        ImageNode::new(hud.pictures[if earned { "star" } else { "lock" }].clone()),
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            flex_shrink: 0.0,
            ..default()
        },
        Transform::from_scale(Vec3::splat(0.001)),
        FocusPolicy::Pass,
    ));
}

/// A report figure such as "+42" that counts up from zero.
pub(super) fn counter(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    key: u64,
    value: u32,
    prefix: &'static str,
    suffix: &'static str,
) {
    p.spawn((
        Counter {
            key,
            value,
            prefix,
            suffix,
        },
        Text::new(format!("{prefix}{value}{suffix}")),
        TextFont {
            font: hud.fonts[Face::Bold as usize].clone(),
            font_size: 25.0,
            ..default()
        },
        TextColor(GOLD),
        FocusPolicy::Pass,
    ));
}

/// Runs before the game reacts to input: remembers which button was pressed,
/// so the rebuilt button can still pulse, and refuses taps on disabled ones.
#[allow(clippy::type_complexity)]
pub(crate) fn press(
    mut game: ResMut<Game>,
    controls: Res<Controls>,
    hud: Res<Hud>,
    mut buttons: Query<(&Interaction, &ButtonAction, &mut Ease, &Node), Changed<Interaction>>,
    mut device: Local<Option<Device>>,
) {
    if device.is_some_and(|before| before != controls.device) {
        game.dirty = true;
    }
    *device = Some(controls.device);
    // Pointer presses are remembered by the game loop; here only refusals.
    for (state, button, mut ease, node) in &mut buttons {
        if *state != Interaction::Pressed {
            continue;
        }
        if button.disabled
            && !matches!(
                button.style,
                Style::Speech | Style::Spotlight | Style::Backdrop
            )
        {
            ease.refuse(node.left);
            game.play(Cue::Blocked);
        }
    }
    if controls.got(controls::A)
        && let Some(action) = game.focus.or(hud.first)
    {
        game.press(action);
    }
}

#[derive(Default)]
pub(crate) struct Overlays {
    view: Option<View>,
    panel: Panel,
    curtain: f32,
    veil: Option<u64>,
    opened: f32,
    fade_backdrop: bool,
}

/// The scene curtain, the dialog opening and the backdrop fade.
#[allow(clippy::type_complexity)]
pub(crate) fn animate_overlays(
    game: Res<Game>,
    time: Res<Time<Real>>,
    mut state: Local<Overlays>,
    mut curtain: Query<(&mut BackgroundColor, &mut Visibility), (With<Curtain>, Without<Backdrop>)>,
    mut veils: Query<(&Veil, &mut Transform)>,
    mut backdrop: Query<(&Backdrop, &mut BackgroundColor), Without<Curtain>>,
) {
    let dt = time.delta_secs().min(MAX_STEP);
    if state.view != Some(game.view) {
        state.view = Some(game.view);
        state.curtain = 1.0;
    } else if game.view == View::Street
        && game.panel == Panel::Report
        && !matches!(state.panel, Panel::Report | Panel::Forecast)
    {
        // A short dip to the evening report.
        state.curtain = state.curtain.max(0.55);
    }
    state.panel = game.panel;
    if let Ok((mut fill, mut visibility)) = curtain.single_mut() {
        let shown = smooth(state.curtain);
        let next = INK.with_alpha(shown);
        if fill.0 != next {
            fill.0 = next;
        }
        let visible = if shown > 0.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != visible {
            *visibility = visible;
        }
    }
    state.curtain = (state.curtain - dt / CURTAIN_TIME).max(0.0);

    let current = veils.iter().map(|(veil, _)| veil.0).max();
    if current != state.veil {
        if current.is_some() {
            state.opened = 0.0;
            state.fade_backdrop = state.veil.is_none();
        }
        state.veil = current;
    }
    let progress = ease_out(state.opened / OPEN_TIME);
    state.opened += dt;
    let size = 0.94 + 0.06 * progress;
    for (_, mut transform) in &mut veils {
        let shown = Vec3::new(size, size, 1.0);
        if transform.scale != shown {
            transform.scale = shown;
        }
    }
    let shade = if state.fade_backdrop { progress } else { 1.0 };
    for (target, mut fill) in &mut backdrop {
        let next = target.0.with_alpha(target.0.alpha() * shade);
        if fill.0 != next {
            fill.0 = next;
        }
    }
}

/// Slides the toast in, colours it by outcome and fades it out at the end.
#[allow(clippy::type_complexity)]
pub(crate) fn animate_toast(
    game: Res<Game>,
    time: Res<Time<Real>>,
    mut state: Local<(u32, f32)>,
    mut slots: Query<(&ToastSlot, &mut Node)>,
    mut boxes: Query<(&mut BackgroundColor, &mut Visibility), With<ToastBox>>,
    mut texts: Query<&mut TextColor, With<ToastText>>,
) {
    if state.0 != game.note_serial {
        *state = (game.note_serial, 0.0);
    } else {
        state.1 += time.delta_secs().min(MAX_STEP);
    }
    let age = state.1;
    let showing = game.note_left > 0.0 && game.lesson.is_none();
    let alpha = (age / 0.12).min(1.0) * (game.note_left / 0.3).min(1.0);
    let (fill, ink) = if game.note_failed {
        (RUST, PAPER)
    } else {
        (GOLD, INK)
    };
    for (slot, mut node) in &mut slots {
        let top = Val::Px(slot.0 - TOAST_SLIDE * (1.0 - ease_out(age / 0.18)));
        if node.top != top {
            node.top = top;
        }
    }
    for (mut background, mut visibility) in &mut boxes {
        let state = if showing {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visibility != state {
            *visibility = state;
        }
        let next = fill.with_alpha(alpha);
        if background.0 != next {
            background.0 = next;
        }
    }
    for mut color in &mut texts {
        let next = ink.with_alpha(alpha);
        if color.0 != next {
            color.0 = next;
        }
    }
}

#[derive(Default)]
pub(crate) struct Reveal {
    report: Option<u64>,
    age: f32,
    goals_for: Option<(usize, bool)>,
    done: Vec<bool>,
    popped: Vec<(String, f32)>,
    clock: f32,
}

/// The evening report reveal and the goal check badges.
#[allow(clippy::type_complexity)]
pub(crate) fn animate_report(
    mut game: ResMut<Game>,
    save: Res<Campaign>,
    time: Res<Time<Real>>,
    mut state: Local<Reveal>,
    mut stars: Query<(&RevealStar, &mut Transform), Without<GoalBadge>>,
    mut counters: Query<(&Counter, &mut Text)>,
    mut badges: Query<(&GoalBadge, &mut Transform), Without<RevealStar>>,
) {
    let dt = time.delta_secs().min(MAX_STEP);
    state.clock += dt;
    let report = stars
        .iter()
        .map(|(star, _)| star.key)
        .chain(counters.iter().map(|(counter, _)| counter.key))
        .next();
    if report != state.report {
        state.report = report;
        state.age = 0.0;
        if report.is_some() {
            let earned = stars.iter().filter(|(star, _)| star.earned).count();
            for i in 0..earned {
                // A rising whole-tone step per star.
                let pitch = 2f32.powf(i as f32 * 2.0 / 12.0);
                game.play_later(Cue::Star, 0.42 + i as f32 * 0.3, Some(pitch));
            }
        }
    } else {
        state.age += dt;
    }
    let age = state.age;
    for (star, mut transform) in &mut stars {
        let size = pop(age - 0.35 - f32::from(star.index) * 0.3);
        let shown = Vec3::new(size, size, 1.0);
        if transform.scale != shown {
            transform.scale = shown;
        }
    }
    let count = ease_out((age - 0.1) / 0.9);
    for (counter, mut text) in &mut counters {
        let value = (counter.value as f32 * count).round() as u32;
        let next = format!("{}{value}{}", counter.prefix, counter.suffix);
        if text.0 != next {
            text.0 = next;
        }
    }

    // Goal completion: watch the harbor goals while a harbor is open, so the
    // sound plays once even if the panel is rebuilt in the same frame.
    let watched = (game.view == View::Street)
        .then(|| save.active_town())
        .flatten();
    if let Some(town) = watched {
        let chapter = &CHAPTERS[town.chapter];
        let goals: Vec<_> = chapter
            .goals
            .into_iter()
            .chain(
                crate::campaign_orders::extra_goals(chapter, town.chapter)
                    .into_iter()
                    .flatten(),
            )
            .collect();
        let done: Vec<bool> = goals
            .iter()
            .map(|&goal| {
                let (have, need) = shown_goal(town, goal);
                have >= need
            })
            .collect();
        let owner = (town.chapter, save.in_tutorial());
        if state.goals_for != Some(owner) || state.done.len() != done.len() {
            state.goals_for = Some(owner);
        } else {
            let mut fresh = false;
            for (i, goal) in goals.iter().enumerate() {
                if done[i] && !state.done[i] {
                    fresh = true;
                    let clock = state.clock;
                    state.popped.push((goal.label(), clock));
                }
            }
            if fresh {
                game.play(Cue::Goal);
            }
        }
        state.done = done;
    } else {
        state.goals_for = None;
    }
    let clock = state.clock;
    state.popped.retain(|(_, at)| clock - *at < 0.6);
    if !state.popped.is_empty() {
        for (badge, mut transform) in &mut badges {
            let label = badge.0.label();
            let size = state
                .popped
                .iter()
                .find(|(name, _)| *name == label)
                .map_or(1.0, |(_, at)| {
                    1.0 + 0.4 * (1.0 - ((clock - at) / 0.45).clamp(0.0, 1.0)).powi(3)
                });
            let shown = Vec3::new(size, size, 1.0);
            if transform.scale != shown {
                transform.scale = shown;
            }
        }
    }
}

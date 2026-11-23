mod adventure;
mod bubbles;
mod guests;
mod lessons;
mod panels;
mod phone;
mod prompts;
mod screens;
mod steering;
mod touch;
mod wardrobe;

pub(crate) use adventure::project as project_adventure;
pub use steering::{WalkPad, WalkPointer, animate_walk_pad, walk_input};
pub(crate) use touch::PanelTouch;

use std::collections::HashMap;

use bevy::{prelude::*, ui::FocusPolicy, window::PrimaryWindow};

use crate::{
    campaign::CHAPTERS,
    controls::{Direction, insets},
    game::{Action, Game, Panel, View},
    model::{Kind, MAX_LEVEL, Phase, Service, Weather},
    save::Campaign,
};

const INK: Color = Color::srgb(0.064, 0.171, 0.181);
const DEEP: Color = Color::srgb(0.082, 0.205, 0.220);
const JADE: Color = Color::srgb(0.128, 0.29, 0.299);
const LINE: Color = Color::srgb(0.24, 0.39, 0.38);
const SAND: Color = Color::srgb(0.992, 0.932, 0.804);
const MUTED: Color = Color::srgb(0.729, 0.807, 0.756);
const GOLD: Color = Color::srgb(0.973, 0.803, 0.479);
const RUST: Color = Color::srgb(0.375, 0.211, 0.187);
const SHADOW: Color = Color::srgba(0.024, 0.092, 0.103, 0.63);
const DARK_TEXT: Color = Color::srgb(0.087, 0.23, 0.233);
const CARD: Color = Color::srgb(0.106, 0.248, 0.262);
const PAPER: Color = Color::srgb(0.95, 0.85, 0.68);

#[derive(Resource, Default)]
pub struct Hud {
    pub first: Option<Action>,
    root: Option<Entity>,
    fonts: [Handle<Font>; 3],
    pictures: HashMap<String, Handle<Image>>,
    adventure_map: Option<(usize, Handle<Image>)>,
    phone: bool,
    close_modal: Option<Action>,
    scroll_y: f32,
}

#[derive(Component, Clone, Copy)]
pub struct ButtonAction {
    pub action: Action,
    pub disabled: bool,
    style: Style,
}

#[derive(Clone, Copy)]
enum Style {
    Primary,
    Quiet,
    Selected,
    Danger,
    Speech,
    Spotlight,
}

#[derive(Clone, Copy)]
enum Face {
    Body,
    Bold,
    Serif,
}

#[derive(Component)]
pub(crate) enum Dynamic {
    Clock,
    Sales,
    Toast,
    Goal(crate::campaign::Goal),
    Stock(usize),
}

#[derive(Component)]
pub(crate) struct DynamicBar;

#[derive(Component)]
pub(crate) struct GoalBar(crate::campaign::Goal);

#[derive(Component)]
pub(crate) struct GoalBadge(crate::campaign::Goal, &'static str);

#[derive(Component)]
pub(crate) struct ToastBox;

pub fn setup(mut hud: ResMut<Hud>, assets: Res<AssetServer>) {
    hud.fonts = [
        assets.load("embedded://fonts/DMSans-Medium.ttf"),
        assets.load("embedded://fonts/DMSans-Bold.ttf"),
        assets.load("embedded://fonts/DMSerifText-Regular.ttf"),
    ];
    let icons = "house well bakery grove potter stoa taverna inn bread olive meze bed coin star sun water wind ferry fire chart book guide hammer lock check nili title_fade storehouse press shrine aqueduct baths forum villa mood cloud rain storm guest_book guest_down guest_empty guest_happy guest_mixed guest_steps guest_wait pad_dpad pad_view sunset pause budget age_child age_adult age_senior cat dog leaves scraps herb_garden machiya temple temple_bell rice_paddy rice_kitchen rice_bowl tea_garden tea_house tea_cup paper_workshop lampion longhouse fishery smokehouse woodlot boatyard sea_trade weaver feast_hall smoked_fish woven_cloth hearth_bowl moon apiary dyer ferry_office mill scribe garden_court market_garden ceramic_studio silk_shop sheepfold cooperage signal_tower oasis_home cistern date_garden spice_bazaar caravanserai qanat glassworks perfumer moon_garden carpet_hall caravan adventure adventure_wood adventure_stone adventure_clay adventure_ore adventure_food adventure_boards adventure_bricks adventure_tools adventure_blueprint adventure_labor adventure_hill adventure_craft adventure_cloth adventure_meals adventure_sawmill adventure_brickyard adventure_forge phone_menu phone_close phone_camera";
    for key in icons
        .split_whitespace()
        .map(str::to_owned)
        .chain((0..CHAPTERS.len()).map(|n| format!("island_{n}")))
        .chain((0..crate::campaign::Region::ALL.len()).map(|n| format!("adventure_cover_{n}")))
        .chain(crate::cosmetics::Cosmetic::ALL.map(|item| item.icon().to_owned()))
    {
        hud.pictures
            .insert(key.clone(), assets.load(format!("embedded://ui/{key}.png")));
    }
}

pub fn draw(
    mut commands: Commands,
    mut hud: ResMut<Hud>,
    mut game: ResMut<Game>,
    save: Res<Campaign>,
    expedition: Res<crate::adventure::Adventure>,
    mut images: ResMut<Assets<Image>>,
) {
    if !game.dirty {
        return;
    }
    game.dirty = false;
    hud.phone = game.phone;
    hud.scroll_y = game.scroll_y;
    hud.close_modal = Some(
        if game.view == View::Adventure && game.panel == Panel::None {
            Action::Adventure(crate::adventure::Command::Close)
        } else {
            Action::Back
        },
    );
    if game.view == View::Adventure
        && expedition.screen == crate::adventure::Screen::Map
        && let Some(terrain) = expedition.terrain.as_ref()
        && hud
            .adventure_map
            .as_ref()
            .is_none_or(|(index, _)| *index != terrain.level.index)
    {
        hud.adventure_map = Some((
            terrain.level.index,
            images.add(adventure::map_image(terrain)),
        ));
    }

    if let Some(entity) = hud.root.take() {
        commands.entity(entity).despawn();
    }
    hud.first = if game.view == View::Adventure && game.panel == Panel::None {
        crate::adventure::first(&expedition)
    } else {
        first(&game, &save)
    };
    let root = commands
        .spawn((
            Root,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            FocusPolicy::Pass,
        ))
        .with_children(|outer| {
            let contents = |base: &mut ChildSpawnerCommands| {
                match (game.view, game.panel) {
                    (_, Panel::Guide) => panels::guide(base, &hud, &game),
                    (_, Panel::Options) => panels::options(base, &hud, &save),
                    (_, Panel::Wardrobe) => wardrobe::draw(base, &hud, &save, &game),
                    (View::Title, _) => screens::title(base, &hud, &save),
                    (View::Adventure, _) => adventure::draw(base, &hud, &game, &expedition),
                    (View::Chart, _) => screens::chart(base, &hud, &game, &save),
                    (View::Street, panel) => {
                        let Some(town) = save.active_town() else {
                            return;
                        };
                        if panel == Panel::None {
                            if matches!(town.phase, Phase::Open(_)) {
                                bubbles::spawn(base, &hud);
                            }
                            prompts::spawn(base, &hud);
                            screens::street(base, &hud, &game, town, save.in_tutorial());
                            if let Some(lesson) = game.lesson {
                                lessons::spawn(base, &hud, lesson, town);
                            }
                        } else {
                            panels::street_panel(base, &hud, &game, town, save.in_tutorial());
                            if matches!(town.phase, Phase::Open(_))
                                && !matches!(
                                    panel,
                                    Panel::Details | Panel::Forecast | Panel::Goals | Panel::Nili
                                )
                            {
                                prompts::pause_chip(base, &hud);
                            }
                        }
                    }
                }
                base.spawn((
                    ToastBox,
                    Visibility::Hidden,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(38.0),
                        top: Val::Px(if game.phone { 12.0 } else { 28.0 }),
                        max_width: Val::Percent(34.0),
                        padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(GOLD),
                    BorderRadius::all(Val::Px(12.0)),
                    ZIndex(20),
                    FocusPolicy::Pass,
                ))
                .with_children(|toast| {
                    toast.spawn((
                        Dynamic::Toast,
                        Text::new(""),
                        TextFont {
                            font: hud.fonts[1].clone(),
                            font_size: 19.0,
                            ..default()
                        },
                        TextColor(INK),
                    ));
                });
            };
            if hud.phone {
                outer
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        FocusPolicy::Pass,
                    ))
                    .with_children(contents);
            } else {
                contents(outer);
            }
        })
        .id();
    hud.root = Some(root);
}

fn job_preview(
    town: &crate::model::Town,
    chapter: &crate::campaign::Chapter,
    job: crate::errands::Job,
) -> Result<(), crate::errands::Blocked> {
    use crate::errands::Blocked;
    let mut latest = Blocked::Requirements;
    for lot in job.targets(town, chapter) {
        match job.plan(town, chapter, lot) {
            Ok(_) => return Ok(()),
            Err(reason) => latest = reason,
        }
    }
    Err(latest)
}

fn first(game: &Game, save: &Campaign) -> Option<Action> {
    if game.lesson.is_some() {
        return Some(Action::Lesson(true));
    }
    let action = match (game.view, game.panel) {
        (_, Panel::Options) => Action::LargeUi,
        (_, Panel::Wardrobe) => Action::WardrobeTab(game.wardrobe_tab),
        (_, Panel::Guide) => Action::GuideTab(game.guide),
        (View::Title, _) => {
            if save.active_town().is_some() {
                Action::Resume
            } else {
                Action::Begin
            }
        }
        (View::Adventure, _) => return None,
        (View::Chart, _) => Action::Choose(game.chart),
        (View::Street, Panel::None) => return None,
        (View::Street, Panel::Pause) => Action::Back,
        (View::Street, Panel::Nili) => {
            let town = save.active_town()?;
            let chapter = &CHAPTERS[town.chapter];
            if game.phone
                && let Some(job) = game.job
            {
                return Some(
                    std::iter::once(game.selected)
                        .chain(job.targets(town, chapter))
                        .find(|&lot| job.plan(town, chapter, lot).is_ok())
                        .map_or(Action::Back, Action::DispatchTo),
                );
            }
            let guide = crate::coach::next(town, chapter, game.selected, game.saw_prices);
            if let Some(job) = guide.and_then(crate::coach::Step::tour) {
                return Some(Action::ChooseJob(job));
            }
            crate::errands::Job::ALL
                .into_iter()
                .find(|&job| job_preview(town, chapter, job).is_ok())
                .map_or(Action::Back, Action::ChooseJob)
        }
        (View::Street, Panel::Build) => {
            let town = save.active_town()?;
            let chapter = &CHAPTERS[town.chapter];
            let available = build_set(chapter, game.build_page);
            let starter = match crate::coach::next(town, chapter, game.selected, game.saw_prices) {
                Some(crate::coach::Step::Build(_, kind)) if available.contains(&kind) => Some(kind),
                _ => available
                    .iter()
                    .copied()
                    .find(|kind| kind.cost() <= town.coins),
            };
            starter.map_or(Action::Back, Action::Build)
        }
        (View::Street, Panel::Details) => {
            let town = save.active_town()?;
            let chapter = &CHAPTERS[town.chapter];
            if matches!(town.phase, Phase::Open(_)) {
                let lot = game.selected;
                if town.incident(chapter).is_some_and(|(_, at)| at == lot)
                    && town.favor_plan(chapter, lot).is_some()
                {
                    return Some(Action::Aid);
                }
                if town.bell_ready(chapter, lot) {
                    return Some(Action::RingBell(lot));
                }
                if town.lots[lot].is_some_and(|b| b.kind == Kind::Boatyard)
                    && let Some(route) = crate::trade::SailingRoute::ALL
                        .into_iter()
                        .find(|&route| town.trade_source(chapter, lot, route).is_ok())
                {
                    return Some(Action::SendVoyage(lot, route));
                }
                if let Some(job) = town
                    .supply_need(chapter, lot, &town.production(chapter))
                    .and_then(|need| need.suggested_job(town, chapter, lot))
                {
                    return Some(Action::ChooseJob(job));
                }
            }
            match town.lots[game.selected] {
                Some(b) if town.planning() && b.kind.service().is_some_and(Service::priced) => {
                    let price =
                        crate::coach::recommended_price(town, game.selected).unwrap_or(b.price);
                    Action::Price(price)
                }
                Some(b)
                    if town.planning() && b.level < MAX_LEVEL && b.upgrade_cost() <= town.coins =>
                {
                    Action::Upgrade
                }
                _ => Action::Back,
            }
        }
        (View::Street, Panel::Report) => {
            if matches!(save.active_town()?.phase, Phase::Report(ref r) if r.stars > 0) {
                Action::NextChapter
            } else {
                Action::NextDay
            }
        }
        (View::Street, _) => Action::Back,
    };
    Some(action)
}

#[derive(Component)]
pub(crate) struct Root;

pub use bubbles::update as update_reactions;
pub use guests::refresh as refresh_guest_summary;
pub use lessons::update as update_lessons;
pub use prompts::update as update_prompts;

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn refresh(
    game: Res<Game>,
    save: Res<Campaign>,
    hud: Res<Hud>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut scale: ResMut<UiScale>,
    mut root: Query<&mut Node, With<Root>>,
    mut buttons: Query<(
        &ButtonAction,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Outline,
    )>,
    mut texts: Query<(&Dynamic, &mut Text)>,
    mut toast: Query<&mut Visibility, With<ToastBox>>,
    mut bar: Query<&mut Node, (With<DynamicBar>, Without<Root>, Without<GoalBar>)>,
    mut goals: Query<
        (&GoalBar, &mut Node, &mut BackgroundColor),
        (Without<DynamicBar>, Without<Root>, Without<ButtonAction>),
    >,
    mut badges: Query<(&GoalBadge, &mut ImageNode)>,
) {
    let factor = if save.large_ui { 1.105 } else { 1.0 };
    let reference_height = if hud.phone { 650.0 } else { 720.0 };
    let actual = (window.width() / 1280.0)
        .min(window.height() / reference_height)
        .clamp(if hud.phone { 0.25 } else { 0.49 }, 2.5)
        * factor;
    if (scale.0 - actual).abs() > 0.001 {
        scale.0 = actual;
    }
    let ratio = window.scale_factor() * actual;
    let [left, top, right, bottom] =
        insets().map(|pixel| Val::Px(f32::from(pixel) / ratio.max(0.2)));
    if let Ok(mut node) = root.single_mut() {
        let padding = UiRect {
            left,
            top,
            right,
            bottom,
        };
        if node.padding != padding {
            node.padding = padding;
        }
    }
    for (button, input, mut fill, mut border, mut outline) in &mut buttons {
        let focused = game.focus.or(hud.first) == Some(button.action) && !button.disabled;
        let hovered = *input != Interaction::None && !button.disabled;
        outline.color = if focused { GOLD } else { Color::NONE };
        (fill.0, border.0) = if matches!(button.style, Style::Spotlight) {
            (Color::NONE, if hovered { SAND } else { GOLD })
        } else if matches!(button.style, Style::Speech) {
            (if hovered { PAPER } else { SAND }, PAPER)
        } else if button.disabled {
            (DEEP, CARD)
        } else if matches!(button.style, Style::Danger) {
            (
                RUST,
                if hovered {
                    PAPER
                } else {
                    Color::srgb(0.61, 0.35, 0.29)
                },
            )
        } else if focused {
            (
                if matches!(button.style, Style::Primary) {
                    GOLD
                } else {
                    JADE
                },
                GOLD,
            )
        } else if hovered {
            (
                if matches!(button.style, Style::Primary) {
                    PAPER
                } else {
                    JADE
                },
                PAPER,
            )
        } else {
            match button.style {
                Style::Primary => (GOLD, PAPER),
                Style::Quiet => (CARD, LINE),
                Style::Selected => (JADE, LINE),
                Style::Danger | Style::Speech | Style::Spotlight => unreachable!(),
            }
        };
    }
    let town = save.active_town();
    let length = town.map_or(crate::model::DAY_SECONDS, |town| {
        CHAPTERS[town.chapter].day_seconds()
    });
    let day = town.and_then(|town| {
        if let Phase::Open(day) = &town.phase {
            Some(day)
        } else {
            None
        }
    });
    if let Ok(mut visible) = toast.single_mut() {
        let state = if game.note_left > 0.0 && game.lesson.is_none() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *visible != state {
            *visible = state;
        }
    }
    for (value, mut text) in &mut texts {
        match value {
            Dynamic::Toast => {
                if text.0 != game.note {
                    text.0.clone_from(&game.note);
                }
            }
            Dynamic::Goal(goal) => {
                if let Some(town) = town {
                    let (have, need) = shown_goal(town, *goal);
                    let value = format!("{} / {need}", have.min(need));
                    if text.0 != value {
                        text.0 = value;
                    }
                }
            }
            Dynamic::Stock(lot) => {
                if let (Some(day), Some(service)) = (
                    day,
                    town.and_then(|town| town.lots[*lot])
                        .and_then(|b| b.kind.service()),
                ) {
                    let value = stock_text(service, day.stock[*lot]);
                    if text.0 != value {
                        text.0 = value;
                    }
                }
            }
            Dynamic::Clock => {
                if let (Some(town), Some(day)) = (town, day) {
                    let text_value = CHAPTERS[town.chapter].shift.clock(day.seconds / length);
                    if text.0 != text_value {
                        text.0 = text_value;
                    }
                }
            }
            Dynamic::Sales => {
                if let Some(day) = day {
                    let text_value = format!(
                        "{} versorgt  ·  +{} Münzen",
                        day.tally.served_total(),
                        day.tally.income
                    );
                    if text.0 != text_value {
                        text.0 = text_value;
                    }
                }
            }
        }
    }
    if let Some(town) = town {
        for (goal, mut fill, mut color) in &mut goals {
            let (have, need) = shown_goal(town, goal.0);
            let width = Val::Percent(have.min(need) as f32 / need as f32 * 100.0);
            if fill.width != width {
                fill.width = width;
            }
            let next = if have >= need { MUTED } else { GOLD };
            if color.0 != next {
                color.0 = next;
            }
        }
        for (badge, mut icon) in &mut badges {
            let (have, need) = shown_goal(town, badge.0);
            let next = &hud.pictures[if have >= need { "check" } else { badge.1 }];
            if icon.image != *next {
                icon.image = next.clone();
            }
        }
    }
    if let (Some(day), Ok(mut fill)) = (day, bar.single_mut()) {
        fill.width = Val::Percent(day.seconds / length * 100.0);
    }
}

pub type FocusButtons<'w, 's> = Query<
    'w,
    's,
    (
        &'static ComputedNode,
        &'static GlobalTransform,
        &'static ButtonAction,
        &'static Interaction,
    ),
>;

pub fn move_focus(
    current: Option<Action>,
    dir: Direction,
    buttons: &FocusButtons,
) -> Option<Action> {
    let enabled = buttons
        .iter()
        .filter(|(node, _, button, _)| !button.disabled && node.size().length_squared() > 1.0);
    let Some((_, origin, _, _)) = enabled
        .clone()
        .find(|(_, _, b, _)| Some(b.action) == current)
    else {
        return enabled.map(|(_, _, b, _)| b.action).next();
    };
    let axis = dir.vector();
    let origin = origin.translation().truncate();
    enabled
        .filter_map(|(_, at, b, _)| {
            let delta = at.translation().truncate() - origin;
            let along = delta.dot(axis);
            let across = delta.perp_dot(axis).abs();
            (along > 5.0).then_some((along + across * 2.3, b.action))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, action)| action)
        .or(current)
}

fn column(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(gap),
        ..default()
    }
}

fn row(gap: f32) -> Node {
    Node {
        align_items: AlignItems::Center,
        column_gap: Val::Px(gap),
        ..default()
    }
}

fn label(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    content: impl Into<String>,
    size: f32,
    face: Face,
    color: Color,
) {
    p.spawn((
        Text::new(content),
        TextFont {
            font: hud.fonts[face as usize].clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
        FocusPolicy::Pass,
    ));
}

fn picture(p: &mut ChildSpawnerCommands, hud: &Hud, key: &str, width: Val, height: Val) {
    let Some(image) = hud.pictures.get(key) else {
        return;
    };
    p.spawn((
        ImageNode::new(image.clone()),
        Node {
            width,
            height,
            flex_shrink: 0.0,
            ..default()
        },
        FocusPolicy::Pass,
    ));
}

fn icon(p: &mut ChildSpawnerCommands, hud: &Hud, key: &str, size: f32) {
    picture(p, hud, key, Val::Px(size), Val::Px(size));
}

fn line(p: &mut ChildSpawnerCommands, color: Color) {
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(color),
    ));
}

fn frame_node(width: Val, gap: f32, pad: f32) -> Node {
    Node {
        width,
        padding: UiRect::all(Val::Px(pad)),
        border: UiRect::all(Val::Px(1.0)),
        ..column(gap)
    }
}

fn card<'p>(p: &'p mut ChildSpawnerCommands, node: Node, color: Color) -> EntityCommands<'p> {
    p.spawn((
        node,
        BackgroundColor(color),
        BorderColor(LINE),
        BorderRadius::all(Val::Px(17.0)),
        FocusPolicy::Pass,
    ))
}

fn key(p: &mut ChildSpawnerCommands, hud: &Hud, name: &str, bright: bool) {
    if hud.phone {
        return;
    }
    if name == "View" {
        icon(p, hud, "pad_view", 31.0);
        return;
    }
    let accent = match name {
        "A" => Some(Color::srgb(0.55, 0.81, 0.58)),
        "B" => Some(Color::srgb(0.97, 0.63, 0.57)),
        "X" => Some(Color::srgb(0.60, 0.78, 0.98)),
        "Y" => Some(GOLD),
        _ => None,
    };
    p.spawn((
        Node {
            min_width: Val::Px(28.0),
            height: Val::Px(28.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::horizontal(Val::Px(if accent.is_some() { 3.0 } else { 6.0 })),
            border: UiRect::all(Val::Px(if accent.is_some() { 1.5 } else { 0.0 })),
            ..row(0.0)
        },
        BackgroundColor(if bright { DARK_TEXT } else { JADE }),
        BorderColor(accent.unwrap_or(JADE)),
        BorderRadius::all(Val::Px(if accent.is_some() { 15.0 } else { 7.0 })),
        FocusPolicy::Pass,
    ))
    .with_children(|pill| label(pill, hud, name, 15.0, Face::Bold, accent.unwrap_or(SAND)));
}

#[allow(clippy::too_many_arguments)]
fn button(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    action: Action,
    title: &str,
    image: &str,
    hotkey: &str,
    style: Style,
    disabled: bool,
) {
    let primary = matches!(style, Style::Primary) && !disabled;
    button_shell(
        p,
        action,
        style,
        disabled,
        Node {
            min_height: Val::Px(if hud.phone {
                if matches!(
                    action,
                    Action::QuickMenu
                        | Action::Open
                        | Action::Plot
                        | Action::DispatchTo(_)
                        | Action::DoErrand
                        | Action::Adventure(crate::adventure::Command::Use(_))
                ) {
                    72.0
                } else {
                    60.0
                }
            } else {
                51.0
            }),
            min_width: if hud.phone { Val::Px(64.0) } else { Val::Auto },
            flex_shrink: if hud.phone { 0.0 } else { 1.0 },
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(Val::Px(13.0), Val::Px(7.0)),
            ..row(9.0)
        },
    )
    .with_children(|item| {
        if !image.is_empty() {
            icon(item, hud, image, 29.0);
        }
        if !hotkey.is_empty() {
            key(item, hud, hotkey, primary);
        }
        label(
            item,
            hud,
            title,
            if hud.phone { 20.0 } else { 18.0 },
            Face::Bold,
            if primary {
                INK
            } else if disabled {
                MUTED
            } else {
                SAND
            },
        );
    });
}

fn button_shell<'p>(
    p: &'p mut ChildSpawnerCommands,
    action: Action,
    style: Style,
    disabled: bool,
    mut node: Node,
) -> EntityCommands<'p> {
    node.border = UiRect::all(Val::Px(2.0));
    p.spawn((
        Button,
        ButtonAction {
            action,
            disabled,
            style,
        },
        node,
        BackgroundColor(CARD),
        BorderColor(LINE),
        BorderRadius::all(Val::Px(12.0)),
        Outline::new(Val::Px(3.0), Val::Px(1.0), Color::NONE),
    ))
}

fn dark_scrim<'p>(p: &'p mut ChildSpawnerCommands) -> EntityCommands<'p> {
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(SHADOW),
        FocusPolicy::Block,
    ))
}

fn kind_icon(kind: Kind) -> &'static str {
    match kind {
        Kind::House => "house",
        Kind::Well => "well",
        Kind::Bakery => "bakery",
        Kind::Grove => "grove",
        Kind::Potter => "potter",
        Kind::Stoa => "stoa",
        Kind::Taverna => "taverna",
        Kind::Inn => "inn",
        Kind::Storehouse => "storehouse",
        Kind::HerbGarden => "herb_garden",
        Kind::Press => "press",
        Kind::Shrine => "shrine",
        Kind::Aqueduct => "aqueduct",
        Kind::Baths => "baths",
        Kind::Forum => "forum",
        Kind::Villa => "villa",
        Kind::Machiya => "machiya",
        Kind::RicePaddy => "rice_paddy",
        Kind::RiceKitchen => "rice_kitchen",
        Kind::TeaGarden => "tea_garden",
        Kind::TeaHouse => "tea_house",
        Kind::PaperWorkshop => "paper_workshop",
        Kind::Temple => "temple",
        Kind::Longhouse => "longhouse",
        Kind::Fishery => "fishery",
        Kind::Smokehouse => "smokehouse",
        Kind::Woodlot => "woodlot",
        Kind::Boatyard => "boatyard",
        Kind::Weaver => "weaver",
        Kind::FeastHall => "feast_hall",
        Kind::Apiary => "apiary",
        Kind::Dyer => "dyer",
        Kind::FerryOffice => "ferry_office",
        Kind::OasisHome => "oasis_home",
        Kind::Cistern => "cistern",
        Kind::DateGarden => "date_garden",
        Kind::SpiceBazaar => "spice_bazaar",
        Kind::Caravanserai => "caravanserai",
        Kind::Qanat => "qanat",
        Kind::Glassworks => "glassworks",
        Kind::Perfumer => "perfumer",
        Kind::MoonGarden => "moon_garden",
        Kind::CarpetHall => "carpet_hall",
        Kind::Mill => "mill",
        Kind::Scribe => "scribe",
        Kind::GardenCourt => "garden_court",
        Kind::MarketGarden => "market_garden",
        Kind::CeramicStudio => "ceramic_studio",
        Kind::SilkShop => "silk_shop",
        Kind::Sheepfold => "sheepfold",
        Kind::Cooperage => "cooperage",
        Kind::SignalTower => "signal_tower",
    }
}

fn has_places(service: Service) -> bool {
    matches!(
        service,
        Service::Shade
            | Service::Bed
            | Service::Bath
            | Service::Trade
            | Service::Prayer
            | Service::HearthMeal
    )
}

fn unavailable(service: Service) -> &'static str {
    if has_places(service) {
        "Belegt"
    } else {
        "Ausverkauft"
    }
}

fn stock_text(service: Service, stock: u16) -> String {
    if stock == 0 {
        unavailable(service).into()
    } else {
        format!("{stock} {}", stock_unit(service, stock))
    }
}

fn stock_unit(service: Service, amount: u16) -> &'static str {
    match service {
        Service::Bread => {
            if amount == 1 {
                "Laib"
            } else {
                "Laibe"
            }
        }
        Service::Pottery => {
            if amount == 1 {
                "Schale"
            } else {
                "Schalen"
            }
        }
        Service::Shade => {
            if amount == 1 {
                "Rastplatz"
            } else {
                "Rastplätze"
            }
        }
        Service::Meze => {
            if amount == 1 {
                "Portion Meze"
            } else {
                "Portionen Meze"
            }
        }
        Service::Bed => {
            if amount == 1 {
                "Bett"
            } else {
                "Betten"
            }
        }
        Service::Oil => {
            if amount == 1 {
                "Krug Öl"
            } else {
                "Krüge Öl"
            }
        }
        Service::Offering => {
            if amount == 1 {
                "Gabe"
            } else {
                "Gaben"
            }
        }
        Service::Bath => {
            if amount == 1 {
                "Badeplatz"
            } else {
                "Badeplätze"
            }
        }
        Service::Trade => {
            if amount == 1 {
                "Marktstand"
            } else {
                "Marktstände"
            }
        }
        Service::Rice => {
            if amount == 1 {
                "Reisschale"
            } else {
                "Reisschalen"
            }
        }
        Service::Tea => {
            if amount == 1 {
                "Schale Tee"
            } else {
                "Schalen Tee"
            }
        }
        Service::Lantern => {
            if amount == 1 {
                "Lampion"
            } else {
                "Lampions"
            }
        }
        Service::Honey => {
            if amount == 1 {
                "Glas Honig"
            } else {
                "Gläser Honig"
            }
        }
        Service::DyedCloth => {
            if amount == 1 {
                "Stoff"
            } else {
                "Stoffe"
            }
        }
        Service::Tablets => {
            if amount == 1 {
                "Tafel"
            } else {
                "Tafeln"
            }
        }
        Service::Dates => {
            if amount == 1 {
                "Dattel"
            } else {
                "Datteln"
            }
        }
        Service::Spices => {
            if amount == 1 {
                "Gewürzpaket"
            } else {
                "Gewürzpakete"
            }
        }
        Service::Glass => {
            if amount == 1 {
                "Glas"
            } else {
                "Gläser"
            }
        }
        Service::Incense => {
            if amount == 1 {
                "Flakon"
            } else {
                "Flakons"
            }
        }
        Service::Casks => {
            if amount == 1 {
                "Fass"
            } else {
                "Fässer"
            }
        }
        Service::Prayer | Service::HearthMeal => {
            if amount == 1 {
                "Platz"
            } else {
                "Plätze"
            }
        }
        Service::SmokedFish => {
            if amount == 1 {
                "Portion Fisch"
            } else {
                "Portionen Fisch"
            }
        }
        Service::Textiles => {
            if amount == 1 {
                "Webware"
            } else {
                "Webwaren"
            }
        }
    }
}

fn service_icon(service: Service) -> &'static str {
    match service {
        Service::Bread => "bread",
        Service::Pottery => "potter",
        Service::Shade => "stoa",
        Service::Meze => "meze",
        Service::Bed => "bed",
        Service::Oil => "press",
        Service::Offering => "shrine",
        Service::Bath => "baths",
        Service::Trade => "forum",
        Service::Rice => "rice_bowl",
        Service::Tea => "tea_cup",
        Service::Lantern => "lampion",
        Service::Prayer => "temple",
        Service::SmokedFish => "smoked_fish",
        Service::Textiles => "woven_cloth",
        Service::HearthMeal => "hearth_bowl",
        Service::Honey => "apiary",
        Service::DyedCloth => "dyer",
        Service::Tablets => "scribe",
        Service::Dates => "date_garden",
        Service::Spices => "spice_bazaar",
        Service::Glass => "glassworks",
        Service::Incense => "perfumer",
        Service::Casks => "cooperage",
    }
}

fn weather_icon(weather: Weather) -> &'static str {
    match weather {
        Weather::Clear | Weather::Heat => "sun",
        Weather::Wind => "wind",
        Weather::Feast => "star",
        Weather::Rain => "rain",
        Weather::Cloud => "cloud",
        Weather::Storm => "storm",
    }
}

fn chapter_picture(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    chapter: usize,
    width: Val,
    height: Val,
) {
    picture(p, hud, &format!("island_{chapter}"), width, height);
}

fn shown_goal(town: &crate::model::Town, goal: crate::campaign::Goal) -> (u32, u32) {
    use crate::campaign::Goal;
    let (value, need) = town.goal(goal);
    let Phase::Open(day) = &town.phase else {
        return (value, need);
    };
    let shown = match goal {
        Goal::Serve(service, _) => value + day.tally.served[service as usize],
        Goal::Renown(_) => value + day.tally.reputation,
        Goal::Earn(_) => value + day.tally.income,
        Goal::Help(_) => value + day.tally.favors,
        Goal::FeastGuests(_) if town.weather(&CHAPTERS[town.chapter]) == Weather::Feast => {
            value.max(day.tally.served_total())
        }
        Goal::Variety(_) => value.max(day.tally.served.iter().filter(|&&n| n > 0).count() as u32),
        Goal::MarketDay(_) => value.max(day.tally.income),
        Goal::Water(_) => value.max(u32::from(day.water)),
        Goal::Harvest(_) => value.max(u32::from(day.rice_harvest)),
        Goal::Specials(_) => value + day.tally.specialties.iter().sum::<u32>(),
        Goal::Caravans(_) => value + day.tally.caravans,
        Goal::CaravanIncome(_) => value + day.tally.caravan_income,
        Goal::SpiceRoutes(_) => value + day.tally.spice_routes,
        Goal::Bell(_) => value + day.tally.bell_rings,
        Goal::Catch(_) => value.max(u32::from(day.fish_catch)),
        Goal::Route(job, _) => value + day.tally.routes[job as usize],
        Goal::RouteVariety(_) => town
            .totals
            .routes
            .iter()
            .zip(day.tally.routes)
            .filter(|&(record, today)| *record + today > 0)
            .count() as u32,
        Goal::FerryDay(_) => value.max(day.tally.ferry_served),
        _ => value,
    };
    (shown, need)
}

fn build_set(chapter: &crate::campaign::Chapter, page: usize) -> &[Kind] {
    let kinds = chapter.allowed();
    if kinds.len() <= 8 {
        return kinds;
    }
    if page == 0 { &kinds[..8] } else { &kinds[8..] }
}

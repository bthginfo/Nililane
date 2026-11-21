mod panels;
mod screens;

use std::collections::HashMap;

use bevy::{prelude::*, ui::FocusPolicy, window::PrimaryWindow};

use crate::{
    campaign::CHAPTERS,
    controls::{Direction, insets},
    game::{Action, Game, Panel, View, can_aid},
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
    Mood,
    Route,
}

#[derive(Component)]
pub(crate) struct DynamicBar;

#[derive(Component)]
pub(crate) struct RouteBar;

#[derive(Component)]
pub(crate) struct GoalBar(crate::campaign::Goal);

#[derive(Component)]
pub(crate) struct ToastBox;

pub fn setup(mut hud: ResMut<Hud>, assets: Res<AssetServer>) {
    hud.fonts = [
        assets.load("embedded://fonts/DMSans-Medium.ttf"),
        assets.load("embedded://fonts/DMSans-Bold.ttf"),
        assets.load("embedded://fonts/DMSerifText-Regular.ttf"),
    ];
    let icons = "house well bakery grove potter stoa taverna inn bread olive meze bed coin star sun water wind ferry chart book guide hammer lock check nili title_fade storehouse press shrine aqueduct baths forum villa mood cloud rain storm";
    for key in icons
        .split_whitespace()
        .map(str::to_owned)
        .chain((0..CHAPTERS.len()).map(|n| format!("island_{n}")))
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
) {
    if !game.dirty {
        return;
    }
    game.dirty = false;
    if let Some(entity) = hud.root.take() {
        commands.entity(entity).despawn();
    }
    hud.first = first(&game, &save);
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
        .with_children(|base| {
            match (game.view, game.panel) {
                (_, Panel::Guide) => panels::guide(base, &hud, &game),
                (_, Panel::Options) => panels::options(base, &hud, &save),
                (View::Title, _) => screens::title(base, &hud, &save),
                (View::Chart, Panel::Replace) => panels::replace(base, &hud, &game),
                (View::Chart, _) => screens::chart(base, &hud, &game, &save),
                (View::Street, panel) => {
                    let Some(town) = &save.town else { return };
                    if panel == Panel::None {
                        screens::street(base, &hud, &game, town);
                    } else {
                        panels::street_panel(base, &hud, &game, town);
                    }
                }
            }
            base.spawn((
                ToastBox,
                Visibility::Hidden,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(38.0),
                    top: Val::Px(28.0),
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
    let action = match (game.view, game.panel) {
        (_, Panel::Options) => Action::LargeUi,
        (_, Panel::Guide) => Action::GuideTab(game.guide),
        (View::Title, _) => {
            if save.town.is_some() {
                Action::Resume
            } else {
                Action::Begin
            }
        }
        (View::Chart, Panel::Replace) => Action::Back,
        (View::Chart, _) => Action::Choose(game.chart),
        (View::Street, Panel::None) => return None,
        (View::Street, Panel::Pause) => Action::Back,
        (View::Street, Panel::Nili) => {
            let town = save.town.as_ref()?;
            let chapter = &CHAPTERS[town.chapter];
            crate::errands::Job::ALL
                .into_iter()
                .find(|&job| job_preview(town, chapter, job).is_ok())
                .map_or(Action::Back, Action::ChooseJob)
        }
        (View::Street, Panel::Build) => {
            let town = save.town.as_ref()?;
            let chapter = &CHAPTERS[town.chapter];
            let starter = if town.chapter == 0
                && town.count(Kind::Bakery) == 0
                && town.coins >= Kind::Bakery.cost()
            {
                Some(Kind::Bakery)
            } else {
                build_set(chapter, game.build_page)
                    .iter()
                    .copied()
                    .find(|kind| kind.cost() <= town.coins)
            };
            starter.map_or(Action::Back, Action::Build)
        }
        (View::Street, Panel::Details) => {
            let town = save.town.as_ref()?;
            match town.lots[game.selected] {
                Some(b) if town.planning() && b.kind.service().is_some() => Action::Price(b.price),
                Some(b)
                    if town.planning() && b.level < MAX_LEVEL && b.upgrade_cost() <= town.coins =>
                {
                    Action::Upgrade
                }
                _ => Action::Back,
            }
        }
        (View::Street, Panel::Report) => {
            if matches!(save.town.as_ref()?.phase, Phase::Report(ref r) if r.stars > 0) {
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
    mut bar: Query<
        &mut Node,
        (
            With<DynamicBar>,
            Without<Root>,
            Without<GoalBar>,
            Without<RouteBar>,
        ),
    >,
    mut goals: Query<
        (&GoalBar, &mut Node),
        (Without<DynamicBar>, Without<Root>, Without<RouteBar>),
    >,
    mut route_bar: Query<&mut Node, (With<RouteBar>, Without<Root>)>,
) {
    let factor = if save.large_ui { 1.105 } else { 1.0 };
    let actual = (window.width() / 1280.0)
        .min(window.height() / 720.0)
        .clamp(0.49, 2.5)
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
        (fill.0, border.0) = if button.disabled {
            (DEEP, CARD)
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
                Style::Danger => (RUST, Color::srgb(0.61, 0.35, 0.29)),
            }
        };
    }
    let town = save.town.as_ref();
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
        let state = if game.note_left > 0.0 {
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
                if let Some(day) = day {
                    let value = format!("{} Portionen übrig", day.stock[*lot]);
                    if text.0 != value {
                        text.0 = value;
                    }
                }
            }
            Dynamic::Route => {
                if let Some(day) = day
                    && let Some(route) = day.route
                {
                    let stage = if day.seconds < route.pickup_at {
                        format!(
                            "Zum Abholen · {}",
                            route.pickup.label(town.expect("a day has a town"))
                        )
                    } else if day.seconds < route.depart_at {
                        "Nili lädt auf …".into()
                    } else if day.seconds >= route.arrival {
                        "Am Ziel".into()
                    } else {
                        format!(
                            "Unterwegs · {} s",
                            (route.finish - day.seconds).ceil() as u32
                        )
                    };
                    if text.0 != stage {
                        text.0 = stage;
                    }
                }
            }
            Dynamic::Mood => {
                let rating = town
                    .and_then(|town| town.feedback())
                    .and_then(|day| day.satisfaction());
                let value = rating.map_or_else(|| "—".to_owned(), |rating| format!("{rating} %"));
                if text.0 != value {
                    text.0 = value;
                }
            }
            Dynamic::Clock => {
                if let Some(day) = day {
                    let hour = 7.0 + day.seconds / length * 12.0;
                    let text_value =
                        format!("{:02}:{:02}", hour as u8, ((hour % 1.0) * 60.0) as u8);
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
        for (goal, mut fill) in &mut goals {
            let (have, need) = shown_goal(town, goal.0);
            let width = Val::Percent(have.min(need) as f32 / need as f32 * 100.0);
            if fill.width != width {
                fill.width = width;
            }
        }
    }
    if let (Some(day), Ok(mut fill)) = (day, bar.single_mut()) {
        fill.width = Val::Percent(day.seconds / length * 100.0);
    }
    if let (Some(day), Ok(mut fill)) = (day, route_bar.single_mut())
        && let Some(route) = day.route
    {
        fill.width = Val::Percent(route.progress(day.seconds) * 100.0);
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
    p.spawn((
        Node {
            min_width: Val::Px(28.0),
            height: Val::Px(27.0),
            justify_content: JustifyContent::Center,
            padding: UiRect::horizontal(Val::Px(6.0)),
            ..row(0.0)
        },
        BackgroundColor(if bright { DARK_TEXT } else { JADE }),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Pass,
    ))
    .with_children(|pill| label(pill, hud, name, 14.0, Face::Bold, SAND));
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
            min_height: Val::Px(51.0),
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
            18.0,
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
        Kind::Press => "press",
        Kind::Shrine => "shrine",
        Kind::Aqueduct => "aqueduct",
        Kind::Baths => "baths",
        Kind::Forum => "forum",
        Kind::Villa => "villa",
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

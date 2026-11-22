use crate::{
    coach::{self, Step},
    layout::lot_row,
    model::{Favor, SERVICES, Town, distance},
    world::{StreetCamera, lot_position},
};

use super::*;

const MARKERS: usize = 3;
const WIDTH: f32 = 123.0;
const HEIGHT: f32 = 44.0;

fn step_words(step: Step) -> (&'static str, &'static str) {
    match step {
        Step::Build(_, Kind::Bakery) => ("Backstube bauen", "bakery"),
        Step::Build(_, _) => ("Brunnen bauen", "well"),
        Step::Price(_) => ("Brotpreis wählen", "coin"),
        Step::StartDay => ("Tag starten", "sun"),
        Step::Arrivals => ("Die ersten Gäste kommen", "ferry"),
        Step::Aid(_, Favor::Jugs) => ("Wasser verteilen", "water"),
        Step::Aid(_, Favor::Shade) => ("Schattenplätze schaffen", "stoa"),
        Step::Aid(_, Favor::Cargo) => ("Vorrat auffüllen", "guest_empty"),
        Step::Water(_) => ("Wasser zur Backstube", "water"),
        Step::Evening => ("Einnahmen am Abend", "sunset"),
    }
}

pub(super) fn strip(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, step: Step) {
    let (word, art) = step_words(step);
    let shortcut = match step {
        Step::StartDay => "Y",
        Step::Water(_) => "R3",
        Step::Build(lot, _) | Step::Price(lot) | Step::Aid(lot, _) if lot == game.selected => "A",
        _ => "",
    };
    p.spawn(Node {
        width: Val::Px(507.0),
        max_width: Val::Percent(100.0),
        ..row(8.0)
    })
    .with_children(|strip| {
        let node = Node {
            min_height: Val::Px(60.0),
            flex_grow: 1.0,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
            ..row(7.0)
        };
        let mut tile = if matches!(step, Step::Arrivals | Step::Evening) {
            card(strip, node, DEEP)
        } else {
            button_shell(strip, Action::Coach(step), Style::Primary, false, node)
        };
        let active = !matches!(step, Step::Arrivals | Step::Evening);
        tile.with_children(|r| {
            icon(r, hud, art, 43.0);
            if !shortcut.is_empty() {
                key(r, hud, shortcut, active);
            } else if step.lot().is_some() {
                icon(r, hud, "pad_dpad", 27.0);
            }
            label(
                r,
                hud,
                word,
                19.0,
                Face::Bold,
                if active { INK } else { SAND },
            );
        });
        if town.chapter == 0
            && town.day == 1
            && matches!(step, Step::Build(..) | Step::Price(_) | Step::StartDay)
        {
            sequence(strip, hud, step);
        } else if matches!(step, Step::Aid(..)) {
            card(
                strip,
                Node {
                    padding: UiRect::horizontal(Val::Px(8.0)),
                    min_height: Val::Px(60.0),
                    ..row(3.0)
                },
                DEEP,
            )
            .with_children(|rewards| {
                icon(rewards, hud, "coin", 25.0);
                label(rewards, hud, "+6", 16.0, Face::Bold, GOLD);
                icon(rewards, hud, "star", 24.0);
                label(rewards, hud, "+2", 16.0, Face::Bold, GOLD);
            });
        }
    });
}

fn sequence(p: &mut ChildSpawnerCommands, hud: &Hud, step: Step) {
    let current = match step {
        Step::Build(..) => 0,
        Step::Price(_) => 1,
        _ => 2,
    };
    card(
        p,
        Node {
            min_height: Val::Px(60.0),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
            ..row(4.0)
        },
        DEEP,
    )
    .with_children(|track| {
        for (i, art) in ["bakery", "coin", "sun"].into_iter().enumerate() {
            if i > 0 {
                label(track, hud, "›", 20.0, Face::Bold, MUTED);
            }
            track
                .spawn((
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        ..row(0.0)
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(if current == i { GOLD } else { CARD }),
                ))
                .with_children(|cell| {
                    icon(cell, hud, if i < current { "check" } else { art }, 25.0);
                });
        }
    });
}

#[derive(Component)]
pub struct Marker(usize);

#[derive(Component)]
pub struct MarkerIcon(usize);

#[derive(Component)]
pub struct MarkerText(usize);

pub(super) fn spawn(p: &mut ChildSpawnerCommands, hud: &Hud) {
    for slot in 0..MARKERS {
        button_shell(
            p,
            Action::SelectLot(0),
            Style::Speech,
            true,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(WIDTH),
                height: Val::Px(HEIGHT),
                padding: UiRect::axes(Val::Px(5.0), Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                ..row(3.0)
            },
        )
        .insert((Marker(slot), Visibility::Hidden, ZIndex(-2)))
        .with_children(|pin| {
            pin.spawn((
                MarkerIcon(slot),
                ImageNode::new(hud.pictures["hammer"].clone()),
                Node {
                    width: Val::Px(33.0),
                    height: Val::Px(33.0),
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
            pin.spawn((
                MarkerText(slot),
                Text::new(""),
                TextColor(INK),
                TextFont {
                    font: hud.fonts[1].clone(),
                    font_size: 15.5,
                    ..default()
                },
            ));
            pin.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(47.0),
                    bottom: Val::Px(-8.0),
                    width: Val::Px(10.0),
                    height: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(SAND),
                BorderRadius::bottom(Val::Px(5.0)),
                FocusPolicy::Pass,
            ));
        });
    }
}

#[derive(Clone, Copy)]
struct AtLot {
    lot: usize,
    title: &'static str,
    icon: &'static str,
    action: Action,
}

fn map_signs(town: &Town, game: &Game) -> Vec<AtLot> {
    let chapter = &CHAPTERS[town.chapter];
    if let Some(job) = game.job {
        let mut targets: Vec<_> = chapter
            .layout
            .lots()
            .filter(|&lot| job.plan(town, chapter, lot).is_ok())
            .collect();
        targets.sort_by_key(|&lot| distance(lot, game.selected));
        return targets
            .into_iter()
            .map(|lot| AtLot {
                lot,
                title: if lot == game.selected {
                    "Ziel"
                } else {
                    "Wählen"
                },
                icon: if lot == game.selected {
                    "check"
                } else {
                    job.icon()
                },
                action: Action::SelectLot(lot),
            })
            .collect();
    }
    let step = coach::next(town, chapter, game.selected, game.saw_prices);
    let mut signs = Vec::new();
    if let Some(step) = step
        && let Some(lot) = step.lot()
    {
        let title = match step {
            Step::Build(_, Kind::Bakery) => "Hier bauen",
            Step::Build(..) => "Brunnen",
            Step::Price(_) => "Preis",
            Step::Water(_) => "Wasser",
            Step::Aid(..) => "Nili hilft",
            _ => unreachable!("only steps with a plot get a marker"),
        };
        signs.push(AtLot {
            lot,
            title,
            icon: step_words(step).1,
            action: Action::Coach(step),
        });
    }

    if let Phase::Open(day) = &town.phase {
        let mut needs = [false; SERVICES];
        for visit in day.visits.iter().filter(|v| !v.resolved) {
            needs[visit.wants as usize] = true;
        }
        for lot in chapter.layout.lots() {
            let Some(service) = town.lots[lot].and_then(|b| b.kind.service()) else {
                continue;
            };
            if day.stock[lot] == 0
                && needs[service as usize]
                && !signs.iter().any(|sign| sign.lot == lot)
            {
                signs.push(AtLot {
                    lot,
                    title: "Leer",
                    icon: "guest_empty",
                    action: Action::LookAtLot(lot),
                });
            }
        }
    }
    signs
}

#[derive(Clone, Copy)]
struct Projected {
    sign: AtLot,
    at: Vec2,
}

type Marks<'w, 's> = Query<
    'w,
    's,
    (
        &'static Marker,
        &'static mut Node,
        &'static mut Visibility,
        &'static mut ButtonAction,
    ),
>;

#[allow(clippy::too_many_arguments)]
pub fn update(
    game: Res<Game>,
    save: Res<Campaign>,
    hud: Res<Hud>,
    scale: Res<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<StreetCamera>>,
    mut markers: Marks,
    mut images: Query<(&MarkerIcon, &mut ImageNode)>,
    mut labels: Query<(&MarkerText, &mut Text)>,
) {
    if markers.is_empty() {
        return;
    }
    let size = Vec2::new(window.width(), window.height()) / scale.0;
    let safe = insets().map(|n| f32::from(n) / (scale.0 * window.scale_factor()));
    let mut active: [Option<Projected>; MARKERS] = [None; MARKERS];
    if let (Some(town), Ok((camera, transform))) = (&save.town, camera.single())
        && game.view == View::Street
        && game.panel == Panel::None
    {
        let chapter = &CHAPTERS[town.chapter];
        let mut filled = 0;
        for sign in map_signs(town, &game) {
            let mut place = lot_position(sign.lot, chapter.layout.width());
            if town.lots[sign.lot].is_some() {
                place.y = 2.63;
                place.z += if lot_row(sign.lot) == 0 { 0.55 } else { -0.55 };
            } else {
                place.y = 0.40;
            }
            let Ok(point) = camera.world_to_viewport(transform, place) else {
                continue;
            };
            let at = point / scale.0 - Vec2::new(WIDTH * 0.5, HEIGHT + 3.0);
            if at.x < safe[0] + 8.0
                || at.x + WIDTH > size.x - safe[2] - 350.0
                || at.y < safe[1] + 109.0
                || at.y + HEIGHT > size.y - safe[3] - 145.0
                || active.iter().flatten().any(|p| {
                    let delta = (p.at - at).abs();
                    delta.x < WIDTH + 6.0 && delta.y < HEIGHT + 8.0
                })
            {
                continue;
            }
            active[filled] = Some(Projected { sign, at });
            filled += 1;
            if filled == MARKERS {
                break;
            }
        }
    }
    for (marker, mut node, mut visibility, mut button) in &mut markers {
        if let Some(p) = active[marker.0] {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Inherited;
            }
            let left = Val::Px(p.at.x - safe[0]);
            let top = Val::Px(p.at.y - safe[1]);
            if (node.left, node.top) != (left, top) {
                node.left = left;
                node.top = top;
            }
            if button.action != p.sign.action {
                button.action = p.sign.action;
            }
            if button.disabled {
                button.disabled = false;
            }
        } else if *visibility != Visibility::Hidden {
            *visibility = Visibility::Hidden;
            button.disabled = true;
        }
    }
    for (tag, mut image) in &mut images {
        if let Some(p) = active[tag.0]
            && image.image != hud.pictures[p.sign.icon]
        {
            image.image = hud.pictures[p.sign.icon].clone();
        }
    }
    for (tag, mut label) in &mut labels {
        if let Some(p) = active[tag.0]
            && label.0 != p.sign.title
        {
            label.0 = p.sign.title.into();
        }
    }
}

pub(super) fn pause_chip(p: &mut ChildSpawnerCommands, hud: &Hud) {
    card(
        p,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(24.0),
            left: Val::Px(23.0),
            padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
            ..row(5.0)
        },
        INK,
    )
    .with_children(|badge| {
        icon(badge, hud, "pause", 28.0);
        label(badge, hud, "Tag pausiert", 14.0, Face::Bold, SAND);
    });
}

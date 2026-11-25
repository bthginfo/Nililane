use std::borrow::Cow;

use crate::{
    coach::{self, Step},
    errands::{RouteStage, Stop},
    layout::lot_row,
    model::{Favor, SERVICES, Town, distance},
    world::{Art, Hippo, StreetCamera, lot_position, route_position},
};

use super::{
    floating::{self, CaptionMeasure, SafeViewport},
    *,
};

const MARKERS: usize = 4;
const MARKER_ICON: f32 = 33.0;
const MARKER_FONT: f32 = 15.5;
const MARKER_GAP: f32 = 3.0;
const MARKER_PAD: Vec2 = Vec2::new(5.0, 4.0);
const MARKER_TAIL: f32 = 8.0;

pub(super) fn marker_size(text: Vec2) -> Vec2 {
    Vec2::new(MARKER_ICON + MARKER_GAP + text.x, MARKER_ICON.max(text.y))
        + 2.0 * MARKER_PAD
        + Vec2::splat(4.0)
}

pub(super) fn stock_caption(service: Service, stock: u16, capacity: u16) -> String {
    format!("{stock}/{capacity} {}", stock_unit(service, capacity))
}

pub(super) fn step_words(step: Step) -> (&'static str, &'static str) {
    match step {
        Step::Build(_, Kind::Bakery) => ("Backstube bauen", "bakery"),
        Step::Build(_, Kind::Potter) => ("Töpferei bauen", "potter"),
        Step::Build(_, _) => ("Brunnen bauen", "well"),
        Step::Price(_) => ("Preis wählen", "coin"),
        Step::Upgrade(_) => ("Ausbau ansehen", "hammer"),
        Step::StartDay => ("Tag starten", "sun"),
        Step::Arrivals => ("Die ersten Gäste kommen", "ferry"),
        Step::Aid(_, Favor::Jugs) => ("Wasser verteilen", "water"),
        Step::Aid(_, Favor::Shade) => ("Schattenplätze schaffen", "stoa"),
        Step::Aid(_, Favor::Cargo) => ("Vorrat auffüllen", "guest_empty"),
        Step::Aid(_, Favor::Fire) => ("Ofenbrand löschen", "fire"),
        Step::Aid(_, Favor::Crates) => ("Kisten sichern", "storehouse"),
        Step::Aid(_, event) => (event.name(), event.icon()),
        Step::Tour(_, job) => (job.name(), job.icon()),
        Step::Evening => ("Einnahmen am Abend", "sunset"),
    }
}

pub(super) fn step_action(step: Step) -> Action {
    match step {
        Step::Aid(lot, _) => Action::AidAt(lot),
        _ => Action::Coach(step),
    }
}

pub(super) fn strip(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, step: Step) {
    let (word, art) = step_words(step);
    let shortcut = match step {
        Step::StartDay => "Y",
        Step::Tour(..) => "R3",
        Step::Build(lot, _) | Step::Price(lot) if lot == game.selected => "A",
        Step::Aid(lot, _) if lot == game.selected && game.nearby.is_none() => "A",
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
            button_shell(strip, step_action(step), Style::Primary, false, node)
        };
        let active = !matches!(step, Step::Arrivals | Step::Evening);
        tile.with_children(|r| {
            icon(r, hud, art, 43.0);
            if !shortcut.is_empty() {
                key(r, hud, shortcut, active);
            } else if step.lot().is_some() {
                dpad(r, hud, 27.0);
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
pub struct Marker {
    slot: usize,
    pub(super) bounds: Option<Rect>,
}

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
                padding: UiRect::axes(Val::Px(MARKER_PAD.x), Val::Px(MARKER_PAD.y)),
                justify_content: JustifyContent::Center,
                ..row(MARKER_GAP)
            },
        )
        .insert((
            Marker { slot, bounds: None },
            Visibility::Hidden,
            ZIndex(-2),
        ))
        .with_children(|pin| {
            pin.spawn((
                MarkerIcon(slot),
                ImageNode::new(hud.pictures["hammer"].clone()),
                Node {
                    width: Val::Px(MARKER_ICON),
                    height: Val::Px(MARKER_ICON),
                    flex_shrink: 0.0,
                    ..default()
                },
                FocusPolicy::Pass,
            ));
            pin.spawn((
                MarkerText(slot),
                Text::new(""),
                TextColor(INK),
                TextLayout::new_with_no_wrap(),
                TextFont {
                    font: hud.fonts[1].clone(),
                    font_size: MARKER_FONT,
                    ..default()
                },
                Node {
                    flex_shrink: 0.0,
                    ..default()
                },
                FocusPolicy::Pass,
            ));
            pin.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(47.0),
                    bottom: Val::Px(-MARKER_TAIL),
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
enum Point {
    Lot(usize),
    Stock(usize),
    Stop(Stop),
    Street([f32; 2]),
    Nili,
}

#[derive(Clone, Copy)]
struct MapSign {
    point: Point,
    title: &'static str,
    icon: &'static str,
    action: Option<Action>,
}

fn map_signs(town: &Town, game: &Game) -> Vec<MapSign> {
    let chapter = &CHAPTERS[town.chapter];
    let mut signs = Vec::new();
    let stock = if let Phase::Open(day) = &town.phase
        && let Some(nili) = day.nili
        && let Some(lot) = crate::world::motion::nearby_lot(town, nili.position, chapter.layout)
        && let Some(service) = town.lots[lot].and_then(|b| b.kind.service())
    {
        Some(MapSign {
            point: Point::Stock(lot),
            title: "",
            icon: service_icon(service),
            action: day.route.is_none().then(|| game.lot_action(town, lot)),
        })
    } else {
        None
    };
    if let Phase::Open(day) = &town.phase
        && let Some((route, nili)) = day.manual_route()
    {
        let can_work = crate::world::motion::can_work(nili, route, chapter.layout);
        signs.push(MapSign {
            point: Point::Stop(nili.target(route)),
            title: if can_work {
                day.errand_action(route, nili.stage).unwrap_or("Hier")
            } else if nili.stage.pickup() {
                "1 · Abholort"
            } else {
                "2 · Ziel"
            },
            icon: if nili.stage.pickup() {
                day.errand_icon(route)
            } else {
                "check"
            },
            action: can_work.then_some(Action::DoErrand),
        });
        signs.extend(stock);
        return signs;
    }
    if let Some(job) = game.job {
        if let Ok(route) = job.plan(town, chapter, game.selected) {
            signs.push(MapSign {
                point: Point::Stop(route.delivery),
                title: "2 · Hier liefern",
                icon: "check",
                action: Some(Action::Dispatch),
            });
            if route.pickup != route.delivery {
                signs.push(MapSign {
                    point: Point::Stop(route.pickup),
                    title: "1 · Abholen",
                    icon: job.icon(),
                    action: None,
                });
            }
        }
        let mut targets: Vec<_> = chapter
            .layout
            .lots()
            .filter(|&lot| lot != game.selected && job.plan(town, chapter, lot).is_ok())
            .collect();
        targets.sort_by_key(|&lot| distance(lot, game.selected));
        signs.extend(targets.into_iter().map(|lot| MapSign {
            point: Point::Lot(lot),
            title: "Ziel wählen",
            icon: job.icon(),
            action: Some(Action::SelectLot(lot)),
        }));
        return signs;
    }
    if let Phase::Open(day) = &town.phase
        && let Some(route) = day.route
    {
        let (stage, _) = route.stage(day.seconds);
        signs.push(MapSign {
            point: Point::Nili,
            title: stage.caption(route.job),
            icon: route.job.icon(),
            action: None,
        });
        if route.pickup != route.delivery
            && matches!(stage, RouteStage::Approach | RouteStage::Collect)
        {
            signs.push(MapSign {
                point: Point::Stop(route.pickup),
                title: "Abholort",
                icon: "storehouse",
                action: None,
            });
        }
        signs.push(MapSign {
            point: Point::Stop(route.delivery),
            title: "Lieferziel",
            icon: "check",
            action: None,
        });
        return signs;
    }
    signs.extend(stock);
    if let Some(done) = game.delivery() {
        let (title, icon) = done.route.reward();
        signs.push(MapSign {
            point: Point::Stop(done.route.delivery),
            title,
            icon,
            action: None,
        });
    }

    if let Some((favor, lot)) = town.incident(chapter) {
        signs.push(MapSign {
            point: match favor {
                Favor::Crates => Point::Stop(Stop::Pier),
                Favor::LostGuests => Point::Stop(Stop::Quay),
                _ => Point::Lot(lot),
            },
            title: favor.event_name(),
            icon: favor.icon(),
            action: town.favor_plan(chapter, lot).map(|_| Action::AidAt(lot)),
        });
    }
    if let Some(lot) = chapter
        .layout
        .lots()
        .filter(|&lot| town.bell_ready(chapter, lot))
        .min_by_key(|&lot| {
            (
                !crate::world::motion::can_inspect(town, lot, chapter.layout),
                distance(lot, game.selected),
            )
        })
    {
        let nearby = crate::world::motion::can_inspect(town, lot, chapter.layout);
        let calling = town.calling(lot).unwrap_or(crate::model::Calling::Bell);
        signs.push(MapSign {
            point: Point::Lot(lot),
            title: if nearby {
                calling.verb()
            } else {
                calling.sign()
            },
            icon: calling.icon(),
            action: Some(if nearby {
                Action::RingBell(lot)
            } else {
                Action::LookAtLot(lot)
            }),
        });
    }
    if let Some(lot) = chapter.layout.lots().find(|&lot| {
        town.trade_source(chapter, lot, crate::trade::SailingRoute::Coastal)
            .is_ok()
            && (chapter.goals.iter().any(|goal| {
                matches!(
                    goal,
                    crate::campaign::Goal::Voyages(_) | crate::campaign::Goal::CargoIncome(_)
                )
            }) || crate::world::motion::can_inspect(town, lot, chapter.layout))
    }) {
        signs.push(MapSign {
            point: Point::Lot(lot),
            title: "Seehandel",
            icon: "sea_trade",
            action: Some(Action::LookAtLot(lot)),
        });
    }
    let step = coach::next(town, chapter, game.selected, game.saw_prices);
    if let Some(step) = step
        && let Some(lot) = step.lot()
    {
        let title = match step {
            Step::Build(_, Kind::Bakery) => "Hier bauen",
            Step::Build(_, Kind::Potter) => "Töpferei",
            Step::Build(..) => "Brunnen",
            Step::Price(_) => "Preis",
            Step::Upgrade(_) => "Ausbau",
            Step::Tour(..) => "Ziel",
            Step::Aid(..) => "Nili hilft",
            _ => unreachable!("only steps with a plot get a marker"),
        };
        signs.push(MapSign {
            point: Point::Lot(lot),
            title,
            icon: step_words(step).1,
            action: Some(step_action(step)),
        });
    }

    if let Phase::Open(day) = &town.phase {
        if let Some(lot) = game.destination {
            signs.push(MapSign {
                point: Point::Stop(Stop::Parcel(lot as u8)),
                title: "Zum Eingang",
                icon: "guest_steps",
                action: Some(game.lot_action(town, lot)),
            });
        }
        for (slot, spot) in crate::street::spots(town, chapter).into_iter().enumerate() {
            let Some(spot) = spot.filter(|spot| !spot.claimed) else {
                continue;
            };
            signs.push(MapSign {
                point: Point::Street(spot.position),
                title: spot.activity.name(),
                icon: spot.activity.icon(),
                action: day
                    .nili
                    .filter(|nili| spot.in_range(nili.position))
                    .map(|_| Action::StreetHelp(slot)),
            });
        }
        let mut needs = [false; SERVICES];
        for visit in day.visits.iter().filter(|v| !v.resolved) {
            needs[visit.wants as usize] = true;
        }
        for lot in chapter.layout.lots() {
            let Some(service) = town.lots[lot].and_then(|b| b.kind.service()) else {
                continue;
            };
            if day.stock[lot] == 0 && needs[service as usize] {
                signs.push(MapSign {
                    point: Point::Lot(lot),
                    title: unavailable(service),
                    icon: "guest_empty",
                    action: Some(game.lot_action(town, lot)),
                });
            }
        }
    }
    signs
}

struct Projected {
    sign: MapSign,
    text: Cow<'static, str>,
    bounds: Rect,
}

type Marks<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Marker,
        &'static mut Node,
        &'static mut Visibility,
        &'static mut ButtonAction,
        &'static mut FocusPolicy,
        &'static mut feel::Ease,
    ),
>;

#[allow(clippy::too_many_arguments)]
pub fn update(
    game: Res<Game>,
    save: Res<Campaign>,
    hud: Res<Hud>,
    art: Res<Art>,
    scale: Res<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<StreetCamera>>,
    nili: Query<&Transform, With<Hippo>>,
    mut markers: Marks,
    mut images: Query<(&MarkerIcon, &mut ImageNode)>,
    mut labels: Query<(&MarkerText, &mut Text)>,
    mut measure: CaptionMeasure,
    mut shown: Local<Vec<u64>>,
) {
    if markers.is_empty() {
        shown.clear();
        return;
    }
    let viewport = SafeViewport::new(&window, scale.0);
    let mut active: [Option<Projected>; MARKERS] = [const { None }; MARKERS];
    if let (Some(town), Ok((camera, transform))) = (save.active_town(), camera.single())
        && game.view == View::Street
        && game.panel == Panel::None
    {
        let chapter = &CHAPTERS[town.chapter];
        let mut filled = 0;
        let manual = matches!(&town.phase, Phase::Open(day) if day.manual_route().is_some());
        let font = TextFont {
            font: hud.fonts[1].clone(),
            font_size: MARKER_FONT,
            ..default()
        };
        for sign in map_signs(town, &game) {
            let plot = |lot| {
                let mut place = lot_position(lot, chapter.layout.width());
                if town.lots[lot].is_some() {
                    place.y = 2.63;
                    place.z += if lot_row(lot) == 0 { 0.55 } else { -0.55 };
                } else {
                    place.y = 0.40;
                }
                place
            };
            let place = match sign.point {
                Point::Lot(lot) => plot(lot),
                Point::Stock(lot) => {
                    let mut at = lot_position(lot, chapter.layout.width());
                    let Some(building) = town.lots[lot] else {
                        continue;
                    };
                    at.y += art.roof_height(chapter.region, building.kind, building.level) + 0.30;
                    at.z += 0.40;
                    at
                }
                Point::Stop(Stop::Parcel(i)) => plot(usize::from(i)),
                Point::Stop(stop) => route_position(stop, chapter.layout) + Vec3::Y * 1.1,
                Point::Street([x, z]) => Vec3::new(x, 0.78, z),
                Point::Nili => {
                    let Ok(hippo) = nili.single() else { continue };
                    hippo.translation + Vec3::Y * 1.70
                }
            };
            let Ok(point) = camera.world_to_viewport(transform, place) else {
                continue;
            };
            let anchor = viewport.point(point);
            if !anchor.cmpge(Vec2::ZERO).all() || !anchor.cmple(viewport.size).all() {
                continue;
            }
            let text = if let Point::Stock(lot) = sign.point {
                let Phase::Open(day) = &town.phase else {
                    continue;
                };
                let Some(service) = town.lots[lot].and_then(|b| b.kind.service()) else {
                    continue;
                };
                Cow::Owned(stock_caption(
                    service,
                    day.stock[lot],
                    day.stock_capacity(lot),
                ))
            } else {
                Cow::Borrowed(sign.title)
            };
            let Some(text_size) = measure.size(&text, &font, viewport.pixels_per_unit) else {
                continue;
            };
            let size = marker_size(text_size);
            let in_rail = !manual && anchor.y - size.y < 337.0 && (!hud.phone || game.quick_menu);
            let limits = Rect {
                min: Vec2::new(8.0, if manual { 48.0 } else { 109.0 }),
                max: viewport.size
                    - Vec2::new(
                        12.0 + if in_rail { 337.0 } else { 0.0 },
                        MARKER_TAIL + if manual { 184.0 } else { 137.0 },
                    ),
            };
            let Some(mut bounds) = floating::above(limits, anchor, size, MARKER_TAIL + 3.0) else {
                continue;
            };
            let overlaps = |candidate| {
                active
                    .iter()
                    .flatten()
                    .any(|p| floating::overlap(p.bounds, candidate, Vec2::new(6.0, 10.0)))
            };
            if matches!(sign.point, Point::Stock(_)) && overlaps(bounds) {
                let Some(higher) =
                    floating::fit(limits, bounds.min - Vec2::Y * (size.y + 11.0), size)
                else {
                    continue;
                };
                bounds = higher;
            }
            if overlaps(bounds) {
                continue;
            }
            active[filled] = Some(Projected { sign, text, bounds });
            filled += 1;
            if filled == MARKERS {
                break;
            }
        }
    }
    let before = std::mem::take(&mut *shown);
    for (mut marker, mut node, mut visibility, mut button, mut focus, mut ease) in &mut markers {
        marker.bounds = active[marker.slot].as_ref().map(|p| p.bounds);
        if let Some(p) = &active[marker.slot] {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Inherited;
            }
            // New markers spring open; markers that survive a rebuild do not.
            let id = feel::key(p.sign.title) ^ feel::key(p.sign.icon).rotate_left(1);
            if !before.contains(&id) {
                ease.pop();
            }
            shown.push(id);
            floating::place_box(&mut node, p.bounds);
            if let Some(action) = p.sign.action
                && button.action != action
            {
                button.action = action;
            }
            let disabled = p.sign.action.is_none();
            if button.disabled != disabled {
                button.disabled = disabled;
            }
            let policy = if disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            };
            if *focus != policy {
                *focus = policy;
            }
        } else if *visibility != Visibility::Hidden {
            *visibility = Visibility::Hidden;
            button.disabled = true;
        }
    }
    for (tag, mut image) in &mut images {
        if let Some(p) = &active[tag.0]
            && image.image != hud.pictures[p.sign.icon]
        {
            image.image = hud.pictures[p.sign.icon].clone();
        }
    }
    for (tag, mut label) in &mut labels {
        if let Some(p) = &active[tag.0]
            && label.0 != p.text.as_ref()
        {
            p.text.as_ref().clone_into(&mut label.0);
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

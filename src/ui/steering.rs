use super::*;
use crate::{
    errands::{NiliControl, Route},
    model::{Day, Town},
    world::motion,
};

const PAD_SIZE: f32 = 146.0;
const THUMB_SIZE: f32 = 61.0;
const THUMB_TRAVEL: f32 = 36.0;

#[derive(Component)]
pub struct SteeringPad {
    floating: bool,
}

#[derive(Component)]
pub struct SteeringThumb;

#[derive(Clone, Copy)]
pub enum WalkPointer {
    Touch(u64, Vec2),
    Mouse(Vec2),
}

pub type WalkPad<'w, 's> = Query<
    'w,
    's,
    (
        &'static SteeringPad,
        &'static ComputedNode,
        &'static GlobalTransform,
    ),
>;

pub fn walk_input(
    pad: &WalkPad,
    window: &Window,
    touches: &Touches,
    mouse: &ButtonInput<MouseButton>,
    pointer: &mut Option<WalkPointer>,
) -> Vec2 {
    let Ok((pad, node, transform)) = pad.single() else {
        return Vec2::ZERO;
    };
    let center = transform.translation().truncate();
    let radius = node.size().x * 0.5;
    if radius < 1.0 {
        return Vec2::ZERO;
    }
    let physical = |at: Vec2| at * window.scale_factor();
    let zone = if pad.floating { radius * 1.12 } else { radius };
    let within = |at: Vec2| (at - center).length_squared() <= zone * zone;
    let origin = |point| if pad.floating { point } else { center };
    let read = |pointer: WalkPointer| match pointer {
        WalkPointer::Touch(id, start) => touches
            .get_pressed(id)
            .map(|touch| physical(touch.position()) - origin(start)),
        WalkPointer::Mouse(start) => mouse
            .pressed(MouseButton::Left)
            .then(|| {
                window
                    .physical_cursor_position()
                    .map(|point| point - origin(start))
            })
            .flatten(),
    };
    if pointer.is_some_and(|held| read(held).is_none()) {
        *pointer = None;
    }
    if pointer.is_none() {
        *pointer = touches
            .iter_just_pressed()
            .map(|touch| {
                let start = if pad.floating {
                    touch.start_position()
                } else {
                    touch.position()
                };
                (touch.id(), physical(start))
            })
            .find(|&(_, point)| within(point))
            .map(|(id, point)| WalkPointer::Touch(id, origin(point)));
        if pointer.is_none()
            && mouse.just_pressed(MouseButton::Left)
            && let Some(point) = window
                .physical_cursor_position()
                .filter(|&point| within(point))
        {
            *pointer = Some(WalkPointer::Mouse(origin(point)));
        }
    }
    let Some(movement) = pointer.and_then(read) else {
        return Vec2::ZERO;
    };
    let delta = movement / (radius * 0.58);
    if delta.length_squared() < 0.04 {
        return Vec2::ZERO;
    }
    delta.clamp_length_max(1.0)
}

pub fn animate_walk_pad(game: Res<Game>, mut thumb: Query<&mut Node, With<SteeringThumb>>) {
    if let Ok(mut thumb) = thumb.single_mut() {
        let anchor = (PAD_SIZE - THUMB_SIZE) * 0.5;
        let offset = Vec2::splat(anchor) + game.walk_axis * THUMB_TRAVEL;
        let at = (Val::Px(offset.x), Val::Px(offset.y));
        if (thumb.left, thumb.top) != at {
            (thumb.left, thumb.top) = at;
        }
    }
}

pub(super) fn pad(p: &mut ChildSpawnerCommands, hud: &Hud) {
    p.spawn((
        SteeringPad {
            floating: hud.phone,
        },
        Node {
            width: Val::Px(PAD_SIZE),
            height: Val::Px(PAD_SIZE),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            0.048,
            0.164,
            0.176,
            if hud.phone { 0.43 } else { 0.86 },
        )),
        BorderColor(MUTED),
        BorderRadius::MAX,
        FocusPolicy::Block,
    ))
    .with_children(|pad| {
        pad.spawn((
            SteeringThumb,
            Node {
                width: Val::Px(THUMB_SIZE),
                height: Val::Px(THUMB_SIZE),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(JADE),
            BorderRadius::MAX,
            FocusPolicy::Pass,
        ))
        .with_children(|thumb| icon(thumb, hud, "pad_dpad", 41.0));
    });
}

pub(super) fn free(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    p.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(23.0),
        bottom: Val::Px(23.0),
        right: Val::Px(353.0),
        align_items: AlignItems::End,
        ..row(12.0)
    })
    .with_children(|foot| {
        pad(foot, hud);
        card(foot, frame_node(Val::Auto, 5.0, 10.0), INK)
            .with_children(|near| nearby(near, hud, game, town));
    });
}

pub(super) fn nearby(near: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    use crate::game::Nearby;
    match game.nearby {
        Some(Nearby::Building(lot)) => {
            let b = town.lots[lot].unwrap();
            let action = game.nearby_action(town, lot);
            let (label, art) = match action {
                Action::RingBell(_) => ("Glocke läuten", "temple_bell"),
                Action::AidAt(_) => ("Hilfe starten", "nili"),
                _ if b.kind == Kind::Boatyard => ("Seehandel", "sea_trade"),
                _ => (b.kind.name(), kind_icon(b.kind)),
            };
            if matches!(action, Action::RingBell(_)) {
                near.spawn(row(6.0)).with_children(|choices| {
                    button(choices, hud, action, label, art, "A", Style::Primary, false);
                    button(
                        choices,
                        hud,
                        Action::LookAtLot(lot),
                        "Tempel ansehen",
                        "temple",
                        "",
                        Style::Quiet,
                        false,
                    );
                });
            } else {
                button(near, hud, action, label, art, "A", Style::Primary, false);
            }
        }
        Some(Nearby::Activity(slot, activity)) => {
            button(
                near,
                hud,
                Action::StreetHelp(slot),
                activity.action(),
                activity.icon(),
                "A",
                Style::Primary,
                false,
            );
        }
        None => {
            panels::metric(near, hud, "pad_dpad", "Linker Stick oder ziehen");
            if let Some(building) = game.destination.and_then(|lot| town.lots[lot]) {
                panels::metric(near, hud, kind_icon(building.kind), building.kind.name());
            } else {
                label(near, hud, "Geh zu einem Gebäude.", 15.0, Face::Body, MUTED);
            }
        }
    }
}

pub(super) fn draw(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    game: &Game,
    day: &Day,
    route: Route,
    nili: NiliControl,
) {
    let pickup = nili.stage.pickup();
    let target = nili.target(route);
    let action = day.errand_action(route, nili.stage);
    let layout = CHAPTERS[town.chapter].layout;
    let ready = motion::can_work(nili, route, layout);
    p.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(23.0),
        bottom: Val::Px(23.0),
        align_items: AlignItems::End,
        ..row(14.0)
    })
    .with_children(|foot| {
        pad(foot, hud);
        card(foot, frame_node(Val::Px(440.0), 6.0, 12.0), INK).with_children(|steps| {
            steps.spawn(row(6.0)).with_children(|line| {
                icon(line, hud, day.errand_icon(route), 25.0);
                label(line, hud, day.errand_title(route), 17.0, Face::Bold, GOLD);
            });
            label(
                steps,
                hud,
                format!(
                    "{} · {}",
                    if pickup { "1 Abholen" } else { "2 Abgeben" },
                    target.label(town)
                ),
                25.0,
                Face::Serif,
                SAND,
            );
            if let Some(action) = action {
                button(
                    steps,
                    hud,
                    Action::DoErrand,
                    action,
                    day.errand_icon(route),
                    "A",
                    Style::Primary,
                    !ready,
                );
            } else {
                let (stage, _) = nili.stage.animation();
                panels::metric(steps, hud, "guest_wait", stage.caption(route.job));
            }
            steps.spawn(row(7.0)).with_children(|legend| {
                icon(legend, hud, "pad_dpad", 19.0);
                label(
                    legend,
                    hud,
                    "Linker Stick oder ziehen",
                    15.0,
                    Face::Bold,
                    MUTED,
                );
            });
        });
    });
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(22.0),
        bottom: Val::Px(23.0),
        width: Val::Px(228.0),
        ..column(7.0)
    })
    .with_children(|actions| {
        button(
            actions,
            hud,
            Action::Speed,
            &format!("Tempo {}×", game.speed),
            "",
            "Y",
            Style::Quiet,
            false,
        );
        button(
            actions,
            hud,
            Action::CancelErrand,
            "Abbrechen",
            "",
            "B",
            Style::Quiet,
            false,
        );
        button(
            actions,
            hud,
            Action::Pause,
            "Menü",
            "",
            "Start",
            Style::Quiet,
            false,
        );
    });
}

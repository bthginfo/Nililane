use super::*;
use crate::{
    adventure::{Adventure, Command, Island, Level, Target},
    campaign::Chapter,
    coach::{self, Step},
    errands::{Blocked, Job},
    model::Town,
};

pub(super) fn pocket(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    harbor: bool,
    content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(14.0),
            top: Val::Px(12.0),
            align_items: AlignItems::FlexEnd,
            ..column(7.0)
        },
        ZIndex(2),
        FocusPolicy::Pass,
    ))
    .with_children(|corner| {
        corner.spawn(row(7.0)).with_children(|buttons| {
            if harbor {
                choice(buttons, hud, Action::Nili, "Touren", "nili");
            }
            button(
                buttons,
                hud,
                Action::QuickMenu,
                if game.quick_menu {
                    "Schließen"
                } else {
                    "Menü"
                },
                if game.quick_menu {
                    "phone_close"
                } else {
                    "phone_menu"
                },
                "",
                if game.quick_menu {
                    Style::Selected
                } else {
                    Style::Quiet
                },
                false,
            );
        });
        if game.quick_menu {
            card(corner, frame_node(Val::Px(370.0), 7.0, 10.0), INK).with_children(content);
        }
    });
}

pub(super) fn pad(p: &mut ChildSpawnerCommands, hud: &Hud, hint: bool) {
    p.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(16.0),
        bottom: Val::Px(14.0),
        align_items: AlignItems::Center,
        ..column(4.0)
    })
    .with_children(|anchor| {
        if hint {
            chip(anchor).with_children(|r| label(r, hud, "Ziehen", 19.0, Face::Bold, SAND));
        }
        steering::pad(anchor, hud);
    });
}

pub(super) fn harbor(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    let chapter = &CHAPTERS[town.chapter];
    let weather = town.sky(chapter);
    let open = if let Phase::Open(day) = &town.phase {
        Some(day)
    } else {
        None
    };
    let route = open.and_then(|day| day.manual_route().map(|(route, nili)| (day, route, nili)));
    let sky = if chapter.shift.night() && weather == Weather::Clear {
        "moon"
    } else {
        weather_icon(weather)
    };
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(14.0),
            top: Val::Px(12.0),
            ..column(7.0)
        },
        FocusPolicy::Pass,
    ))
    .with_children(|stack| {
        chip(stack).with_children(|r| {
            icon(r, hud, sky, 29.0);
            if open.is_some() {
                r.spawn((
                    Dynamic::Clock,
                    Text::new(chapter.shift.clock(0.0)),
                    TextColor(GOLD),
                    TextFont {
                        font: hud.fonts[1].clone(),
                        font_size: 23.0,
                        ..default()
                    },
                ));
            } else {
                label(r, hud, format!("Tag {}", town.day), 22.0, Face::Bold, SAND);
            }
            icon(r, hud, "coin", 26.0);
            label(r, hud, town.coins.to_string(), 22.0, Face::Bold, SAND);
        });
        if route.is_none()
            && game.job.is_none()
            && game.lesson.is_none()
            && let Some(step) = coach::next(town, chapter, game.selected, game.saw_prices)
            && !matches!(step, Step::StartDay | Step::Arrivals | Step::Evening)
        {
            let (title, icon) = prompts::step_words(step);
            button(
                stack,
                hud,
                prompts::step_action(step),
                title,
                icon,
                "",
                Style::Quiet,
                false,
            );
        }
    });
    if let Some((day, route, nili)) = route {
        errand(p, hud, town, day, route, nili, !game.phone_walked);
        return;
    }
    pocket(p, hud, game, true, |menu| {
        guests::summary(menu, hud, town);
        pair(
            menu,
            hud,
            [
                (Action::Goals, "Aufträge", "book"),
                (Action::Forecast, "Tagesplan", "ferry"),
            ],
        );
        choice(menu, hud, Action::Feedback, "Gästebuch", "guest_book");
        if open.is_some() {
            let speed = format!("Tempo {}×", game.speed);
            pair(
                menu,
                hud,
                [
                    (Action::Speed, &speed, "sun"),
                    (Action::Pause, "Pause", "pause"),
                ],
            );
        } else {
            choice(menu, hud, Action::Pause, "Pause", "pause");
        }
        camera(menu, hud, Action::CameraView, game.zoom > 0.1);
    });
    if open.is_some() {
        pad(p, hud, !game.phone_walked);
    }
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(14.0),
        bottom: Val::Px(14.0),
        align_items: AlignItems::FlexEnd,
        ..column(6.0)
    })
    .with_children(|near| {
        if let Some(job) = game.job {
            let ready = job.plan(town, chapter, game.selected);
            chip(near).with_children(|r| {
                icon(r, hud, job.icon(), 26.0);
                label(
                    r,
                    hud,
                    ready
                        .as_ref()
                        .map_or_else(|reason| (*reason).label(job), |_| job.name()),
                    18.0,
                    Face::Bold,
                    SAND,
                );
            });
            near.spawn(row(7.0)).with_children(|r| {
                choice(r, hud, Action::Back, "Zurück", "");
                button(
                    r,
                    hud,
                    Action::Dispatch,
                    "Nili steuern",
                    "nili",
                    "",
                    Style::Primary,
                    ready.is_err(),
                );
            });
        } else if town.planning() {
            near.spawn(row(7.0)).with_children(|r| {
                if let Some(property) = town.lots[game.selected] {
                    choice(
                        r,
                        hud,
                        Action::Plot,
                        property.kind.name(),
                        kind_icon(property.kind),
                    );
                } else {
                    choice(r, hud, Action::Plot, "Bauen", "hammer");
                }
                button(
                    r,
                    hud,
                    Action::Open,
                    if chapter.shift.night() {
                        "Nacht starten"
                    } else {
                        "Tag starten"
                    },
                    if chapter.shift.night() { "moon" } else { "sun" },
                    "",
                    Style::Primary,
                    false,
                );
            });
        } else if game.nearby.is_some() {
            near.spawn(row(7.0))
                .with_children(|r| steering::nearby(r, hud, game, town));
        } else if let Some(building) = game.destination.and_then(|lot| town.lots[lot]) {
            chip(near).with_children(|r| {
                icon(r, hud, kind_icon(building.kind), 27.0);
                label(r, hud, building.kind.name(), 18.0, Face::Bold, SAND);
            });
        }
    });
}

pub(super) fn tours(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    town: &Town,
    chapter: &Chapter,
) {
    if let Some(job) = game.job {
        panels::modal(
            p,
            hud,
            760.0,
            "TOUR STARTEN",
            job.name(),
            job.icon(),
            |page| {
                let mut lots = job.targets(town, chapter);
                lots.sort_by_key(|&lot| lot != game.selected);
                let mut possible = false;
                for lot in lots {
                    let Ok(route) = job.plan(town, chapter, lot) else {
                        continue;
                    };
                    possible = true;
                    card(page, frame_node(Val::Percent(100.0), 4.0, 9.0), CARD).with_children(
                        |card| {
                            card.spawn(Node {
                                justify_content: JustifyContent::SpaceBetween,
                                ..row(12.0)
                            })
                            .with_children(|line| {
                                line.spawn(row(9.0)).with_children(|stops| {
                                    icon(stops, hud, job.icon(), 29.0);
                                    label(
                                        stops,
                                        hud,
                                        route.pickup.label(town),
                                        19.0,
                                        Face::Bold,
                                        MUTED,
                                    );
                                    label(stops, hud, "→", 24.0, Face::Bold, GOLD);
                                    label(
                                        stops,
                                        hud,
                                        route.delivery.label(town),
                                        21.0,
                                        Face::Bold,
                                        SAND,
                                    );
                                });
                                button(
                                    line,
                                    hud,
                                    Action::DispatchTo(lot),
                                    "Starten",
                                    "",
                                    "",
                                    Style::Primary,
                                    false,
                                );
                            });
                        },
                    );
                }
                if !possible && let Err(reason) = job_preview(town, chapter, job) {
                    panels::metric(page, hud, reason.symbol(), reason.label(job));
                }
                choice(page, hud, Action::Back, "Alle Touren", "");
            },
        );
        return;
    }

    let day = if let Phase::Open(day) = &town.phase {
        Some(day)
    } else {
        None
    };
    let heading = format!(
        "{} / {} TOUREN HEUTE",
        day.map_or(0, |day| day.tally.routes_today()),
        Job::limit(town.chapter)
    );
    panels::modal(p, hud, 760.0, &heading, "Nilis Touren", "nili", |page| {
        if let Some(day) = day
            && let Some(lot) = coach::harbor_favor(town, chapter, game.selected)
        {
            let title = if day.favor.incident() {
                day.favor.event_name()
            } else {
                "Bitte vom Kai"
            };
            tour_choice(
                page,
                hud,
                Action::AidAt(lot),
                day.favor.icon(),
                title,
                "Hilfe starten",
                false,
            );
        }
        if town.chapter == 0 {
            let ready = coach::harbor_favor(town, chapter, game.selected).is_some();
            if !ready {
                let message = match day {
                    Some(day) if day.favor_at.is_some() => "Die Bitte vom Kai ist erledigt.",
                    Some(_) => "Hier erscheint die nächste Bitte vom Kai.",
                    None => "Bitten vom Kai erscheinen nach Ankunft der Fähre.",
                };
                panels::metric(page, hud, "ferry", message);
            }
            label(
                page,
                hud,
                "Eigene Touren gibt es ab Amorgos.",
                18.0,
                Face::Body,
                MUTED,
            );
        }
        for job in Job::ALL
            .into_iter()
            .filter(|job| job.unlocked(town.chapter))
        {
            let preview = day
                .ok_or(Blocked::Morning)
                .and_then(|_| job_preview(town, chapter, job));
            tour_choice(
                page,
                hud,
                Action::ChooseJob(job),
                job.icon(),
                job.name(),
                preview.map_or_else(|reason| reason.label(job), |_| job.purpose()),
                preview.is_err(),
            );
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn tour_choice(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    action: Action,
    art: &str,
    title: &str,
    details: &str,
    disabled: bool,
) {
    button_shell(
        p,
        action,
        Style::Quiet,
        disabled,
        Node {
            min_height: Val::Px(74.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            ..row(12.0)
        },
    )
    .with_children(|tile| {
        icon(tile, hud, art, 38.0);
        tile.spawn((
            Node {
                flex_grow: 1.0,
                min_width: Val::Px(0.0),
                ..column(2.0)
            },
            FocusPolicy::Pass,
        ))
        .with_children(|copy| {
            label(copy, hud, title, 21.0, Face::Bold, SAND);
            label(copy, hud, details, 17.0, Face::Body, MUTED);
        });
        if !disabled {
            label(tile, hud, "›", 30.0, Face::Bold, GOLD);
        }
    });
}

fn errand(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    day: &crate::model::Day,
    route: crate::errands::Route,
    nili: crate::errands::NiliControl,
    hint: bool,
) {
    pad(p, hud, hint);
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(14.0),
        top: Val::Px(12.0),
        ..row(6.0)
    })
    .with_children(|r| {
        choice(r, hud, Action::Pause, "Pause", "pause");
        choice(r, hud, Action::CancelErrand, "Tour beenden", "phone_close");
    });
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(14.0),
        bottom: Val::Px(14.0),
        align_items: AlignItems::FlexEnd,
        ..column(6.0)
    })
    .with_children(|r| {
        chip(r).with_children(|summary| {
            icon(summary, hud, day.errand_icon(route), 27.0);
            let action = if nili.stage.pickup() {
                "Abholen"
            } else {
                "Abgeben"
            };
            label(
                summary,
                hud,
                format!("{action} · {}", nili.target(route).label(town)),
                19.0,
                Face::Bold,
                SAND,
            );
        });
        if let Some(caption) = day.errand_action(route, nili.stage) {
            button(
                r,
                hud,
                Action::DoErrand,
                caption,
                day.errand_icon(route),
                "",
                Style::Primary,
                !crate::world::motion::can_work(nili, route, CHAPTERS[town.chapter].layout),
            );
        }
    });
}

pub(super) fn expedition(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    let mut top = corner_node();
    top.position_type = PositionType::Absolute;
    top.left = Val::Px(14.0);
    top.top = Val::Px(12.0);
    card(p, top, INK).with_children(|r| {
        icon(
            r,
            hud,
            if (0.18..=0.82).contains(&island.phase) {
                "sun"
            } else {
                "moon"
            },
            28.0,
        );
        label(r, hud, level.name, 24.0, Face::Serif, SAND);
    });
    pocket(p, hud, game, false, |menu| {
        let (command, title, art) = if island.completed {
            (Command::Summary, "Siedlungsbilanz", "check")
        } else {
            (Command::Inspect(Target::Hall), "Aufträge", "book")
        };
        choice(menu, hud, Action::Adventure(command), title, art);
        choice(
            menu,
            hud,
            Action::Adventure(Command::Map),
            "Inselkarte und Lager",
            "chart",
        );
        choice(
            menu,
            hud,
            Action::Adventure(Command::Labor),
            "Bürgeraufträge",
            "adventure_labor",
        );
        camera(
            menu,
            hud,
            Action::Adventure(Command::CameraView),
            a.map_zoom > 0.1,
        );
        choice(
            menu,
            hud,
            Action::Adventure(Command::Pause),
            "Pause",
            "pause",
        );
    });
}

fn pair(menu: &mut ChildSpawnerCommands, hud: &Hud, options: [(Action, &str, &str); 2]) {
    menu.spawn(row(6.0)).with_children(|line| {
        for (action, title, icon) in options {
            line.spawn(Node {
                flex_grow: 1.0,
                ..column(0.0)
            })
            .with_children(|slot| choice(slot, hud, action, title, icon));
        }
    });
}

pub(super) fn help(p: &mut ChildSpawnerCommands, hud: &Hud, back: Action) {
    for (art, title, hint) in [
        (
            "pad_dpad",
            "Nili bewegen",
            "Zieh den Stick unten links. Lass ihn los, um stehen zu bleiben.",
        ),
        (
            "hammer",
            "Etwas tun",
            "Tippe auf das Ziel oder auf die Aktion unten rechts.",
        ),
        (
            "phone_menu",
            "Mehr sehen",
            "Öffne das Menü für Aufträge, die Übersicht und die Kamera.",
        ),
        (
            "chart",
            "Listen durchsehen",
            "Wische in langen Fenstern nach oben und unten.",
        ),
    ] {
        p.spawn(row(10.0)).with_children(|r| {
            icon(r, hud, art, 40.0);
            r.spawn(column(1.0)).with_children(|c| {
                label(c, hud, title, 20.0, Face::Bold, SAND);
                label(c, hud, hint, 17.0, Face::Body, MUTED);
            });
        });
    }
    button(p, hud, back, "Zurück", "", "", Style::Primary, false);
}

fn camera(menu: &mut ChildSpawnerCommands, hud: &Hud, action: Action, zoomed: bool) {
    choice(
        menu,
        hud,
        action,
        if zoomed {
            "Kamera · Übersicht"
        } else {
            "Kamera · Näher ran"
        },
        "phone_camera",
    );
}

fn choice(p: &mut ChildSpawnerCommands, hud: &Hud, action: Action, title: &str, icon: &str) {
    button(p, hud, action, title, icon, "", Style::Quiet, false);
}

fn corner_node() -> Node {
    Node {
        padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
        ..row(8.0)
    }
}

fn chip<'p>(p: &'p mut ChildSpawnerCommands) -> EntityCommands<'p> {
    card(p, corner_node(), Color::srgba(0.059, 0.16, 0.17, 0.92))
}

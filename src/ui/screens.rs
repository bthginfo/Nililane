use super::*;
use crate::{
    campaign::{CORE_HARBORS, Goal, REGION_SIZE, Region, previous_harbor},
    coach,
    layout::{lot_col, lot_row},
    model::Town,
};

pub fn title(p: &mut ChildSpawnerCommands, hud: &Hud, save: &Campaign) {
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            width: Val::Percent(45.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.045, 0.142, 0.157, 0.94)),
        FocusPolicy::Pass,
    ));
    if let Some(fade) = hud.pictures.get("title_fade") {
        p.spawn((
            ImageNode::new(fade.clone()),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(45.0),
                top: Val::Px(0.0),
                width: Val::Percent(19.0),
                height: Val::Percent(100.0),
                ..default()
            },
            FocusPolicy::Pass,
        ));
    }
    p.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(48.0),
        top: Val::Px(0.0),
        bottom: Val::Px(0.0),
        width: Val::Percent(38.0),
        max_width: Val::Px(495.0),
        justify_content: JustifyContent::Center,
        ..column(if hud.phone { 11.0 } else { 19.0 })
    })
    .with_children(|stack| {
        stack.spawn(row(8.0)).with_children(|brand| {
            icon(brand, hud, "ferry", 35.0);
            label(brand, hud, "NILIS HAFENREISEN", 14.0, Face::Bold, GOLD);
        });
        label(
            stack,
            hud,
            "Nililane",
            if hud.phone { 76.0 } else { 94.0 },
            Face::Serif,
            SAND,
        );
        label(
            stack,
            hud,
            "Ein Nilpferd, eine Hafengasse
und viele kleine Geschichten.",
            26.0,
            Face::Serif,
            SAND,
        );
        stack.spawn(Node {
            height: Val::Px(10.0),
            ..default()
        });
        // The entries arrive one after another on the first title of a launch.
        feel::arriving(stack, 0, |stack| {
            if let Some(town) = save.active_town() {
                button(
                    stack,
                    hud,
                    Action::Resume,
                    if save.in_tutorial() {
                        "Tutorial fortsetzen"
                    } else {
                        "Reise fortsetzen"
                    },
                    if save.in_tutorial() { "guide" } else { "ferry" },
                    "A",
                    Style::Primary,
                    false,
                );
                stack
                    .spawn(Node {
                        margin: UiRect::top(Val::Px(if hud.phone { 11.0 } else { 19.0 })),
                        ..default()
                    })
                    .with_children(|place| {
                        label(
                            place,
                            hud,
                            format!("{}  ·  Tag {}", CHAPTERS[town.chapter].district, town.day),
                            18.0,
                            Face::Body,
                            MUTED,
                        );
                    });
            } else {
                button(
                    stack,
                    hud,
                    Action::Begin,
                    "Erste Reise",
                    "ferry",
                    "A",
                    Style::Primary,
                    false,
                );
            }
        });
        feel::arriving(stack, 1, |stack| {
            button(
                stack,
                hud,
                Action::Chart,
                if save.in_tutorial() {
                    "Übung beenden"
                } else {
                    "Kampagnenkarte"
                },
                "chart",
                "",
                Style::Quiet,
                false,
            );
        });
        feel::arriving(stack, 2, |stack| {
            button(
                stack,
                hud,
                Action::Adventure(crate::adventure::Command::Enter),
                "Abenteuerkampagne",
                "adventure",
                "",
                Style::Quiet,
                false,
            );
        });
        stack
            .spawn(Node {
                width: Val::Percent(100.0),
                ..row(9.0)
            })
            .with_children(|row| {
                feel::arriving(row, 3, |row| {
                    button(
                        row,
                        hud,
                        Action::Guide,
                        "Spielhilfe",
                        "guide",
                        "",
                        Style::Quiet,
                        false,
                    );
                });
                feel::arriving(row, 4, |row| {
                    button(
                        row,
                        hud,
                        Action::Options,
                        "Einstellungen",
                        "",
                        "",
                        Style::Quiet,
                        false,
                    );
                });
            });
    });
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(29.0),
            bottom: Val::Px(24.0),
            padding: UiRect::axes(Val::Px(15.0), Val::Px(7.0)),
            ..row(8.0)
        },
        BackgroundColor(Color::srgba(0.046, 0.171, 0.186, 0.87)),
        BorderRadius::all(Val::Px(11.0)),
        FocusPolicy::Pass,
    ))
    .with_children(|row| {
        icon(row, hud, "nili", 33.0);
        label(row, hud, "Nili hilft am Kai", 18.0, Face::Serif, SAND);
    });
}

pub fn chart(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, save: &Campaign) {
    let ch = &CHAPTERS[game.chart];
    let region = Region::of(game.chart);
    let offset = region.start();
    let second = game.chart - offset >= CORE_HARBORS;
    let start = offset + if second { CORE_HARBORS } else { 0 };
    let mut node = frame_node(Val::Percent(95.0), 10.0, 19.0);
    node.max_width = Val::Px(1190.0);
    let mut cover = overlay(p);
    cover.insert(feel::Veil(feel::key("Die Hafenreisen")));
    cover.with_children(|cover| {
        card(cover, node, INK).with_children(|body| {
            body.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(14.0)
            })
            .with_children(|title| {
                title.spawn(row(9.0)).with_children(|left| {
                    icon(left, hud, "chart", 47.0);
                    label(left, hud, "Die Hafenreisen", 41.0, Face::Serif, SAND);
                });
                let page_size = if second {
                    REGION_SIZE - CORE_HARBORS
                } else {
                    CORE_HARBORS
                };
                let completed = save.records[start..start + page_size]
                    .iter()
                    .filter(|r| r.stars > 0)
                    .count();
                title.spawn(row(15.0)).with_children(|info| {
                    label(
                        info,
                        hud,
                        format!("{completed} von {page_size} Häfen"),
                        18.0,
                        Face::Bold,
                        GOLD,
                    );
                    button(
                        info,
                        hud,
                        Action::Wardrobe,
                        "Garderobe",
                        "nili",
                        "",
                        Style::Quiet,
                        false,
                    );
                });
            });
            body.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(9.0)
            })
            .with_children(|tabs| {
                tabs.spawn(row(7.0)).with_children(|routes| {
                    for r in Region::ALL {
                        let locked = !save.unlocked(r.start());
                        button(
                            routes,
                            hud,
                            Action::Region(r),
                            r.name(),
                            if locked { "lock" } else { r.icon() },
                            "",
                            if r == region {
                                Style::TabOn
                            } else {
                                Style::Tab
                            },
                            false,
                        );
                    }
                });
                tabs.spawn(row(7.0)).with_children(|pages| {
                    button(
                        pages,
                        hud,
                        Action::ChartPage(false),
                        "Hauptreise",
                        "",
                        "LB",
                        if !second { Style::TabOn } else { Style::Tab },
                        false,
                    );
                    button(
                        pages,
                        hud,
                        Action::ChartPage(true),
                        "Bonusroute",
                        "star",
                        "RB",
                        if second { Style::TabOn } else { Style::Tab },
                        false,
                    );
                });
            });
            for start in [start, start + 6] {
                body.spawn(row(8.0)).with_children(|strip| {
                    for (n, chapter) in CHAPTERS.iter().enumerate().skip(start).take(6) {
                        let open = save.unlocked(n);
                        let stars = save.records[n].stars;
                        let style = if game.chart == n {
                            Style::Selected
                        } else {
                            Style::Quiet
                        };
                        button_shell(
                            strip,
                            Action::Choose(n),
                            style,
                            false,
                            Node {
                                flex_grow: 1.0,
                                flex_basis: Val::Px(0.0),
                                min_width: Val::Px(0.0),
                                padding: UiRect::all(Val::Px(6.0)),
                                ..column(4.0)
                            },
                        )
                        .with_children(|tile| {
                            chapter_picture(tile, hud, n, Val::Percent(100.0), Val::Px(65.0));
                            tile.spawn(Node {
                                justify_content: JustifyContent::SpaceBetween,
                                ..row(2.0)
                            })
                            .with_children(|caption| {
                                label(caption, hud, chapter.district, 17.0, Face::Bold, SAND);
                                if !open {
                                    icon(caption, hud, "lock", 21.0);
                                } else if stars > 0 {
                                    caption.spawn(row(2.0)).with_children(|marks| {
                                        icon(marks, hud, "star", 20.0);
                                        label(
                                            marks,
                                            hud,
                                            stars.to_string(),
                                            15.0,
                                            Face::Bold,
                                            GOLD,
                                        );
                                    });
                                }
                            });
                        });
                    }
                });
            }
            body.spawn(Node {
                min_height: Val::Px(92.0),
                align_items: AlignItems::Center,
                ..row(18.0)
            })
            .with_children(|info| {
                info.spawn(Node {
                    min_width: Val::Px(250.0),
                    ..column(2.0)
                })
                .with_children(|caption| {
                    caption.spawn(row(5.0)).with_children(|row| {
                        if ch.shift.night() {
                            icon(row, hud, "moon", 21.0);
                        }
                        label(
                            row,
                            hud,
                            format!("{}  ·  {}", ch.district, ch.difficulty),
                            15.0,
                            Face::Bold,
                            GOLD,
                        );
                    });
                    label(caption, hud, ch.name, 27.0, Face::Serif, SAND);
                    label(
                        caption,
                        hud,
                        format!(
                            "{} · {} Bauplätze",
                            ch.layout.shore.name(),
                            ch.layout.count()
                        ),
                        15.0,
                        Face::Body,
                        MUTED,
                    );
                    for bank in 0..2 {
                        caption.spawn(row(3.0)).with_children(|lots| {
                            for col in 0..ch.layout.row(bank) {
                                let dock = crate::layout::lot_id(bank, col) == ch.layout.dock();
                                lots.spawn((
                                    Node {
                                        width: Val::Px(14.0),
                                        height: Val::Px(8.0),
                                        ..default()
                                    },
                                    BorderRadius::all(Val::Px(2.0)),
                                    BackgroundColor(if dock {
                                        GOLD
                                    } else if bank == 0 {
                                        JADE
                                    } else {
                                        LINE
                                    }),
                                ));
                            }
                        });
                    }
                });
                let message = if save.unlocked(game.chart) {
                    format!(
                        "{} · {} von {}",
                        ch.greeting,
                        game.chart - start + 1,
                        CORE_HARBORS
                    )
                } else {
                    format!(
                        "Schließe {} ab, um diesen Hafen freizuschalten.",
                        CHAPTERS[previous_harbor(game.chart).unwrap_or(0)].district
                    )
                };
                label(info, hud, message, 17.0, Face::Body, MUTED);
            });
            line(body, LINE);
            body.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(10.0)
            })
            .with_children(|actions| {
                button(
                    actions,
                    hud,
                    Action::Back,
                    "Zurück",
                    "",
                    "B",
                    Style::Quiet,
                    false,
                );
                button(
                    actions,
                    hud,
                    Action::Tutorial,
                    "Tutorial neu starten",
                    "guide",
                    "Y",
                    Style::Quiet,
                    false,
                );
                let continuing = save.in_progress(game.chart);
                button(
                    actions,
                    hud,
                    Action::Sail,
                    if continuing { "Fortsetzen" } else { "Ablegen" },
                    "ferry",
                    "",
                    Style::Primary,
                    !save.unlocked(game.chart),
                );
            });
        });
    });
}

pub fn street(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, tutorial: bool) {
    if hud.phone {
        return phone::harbor(p, hud, game, town);
    }
    let ch = &CHAPTERS[town.chapter];
    let weather = town.sky(ch);
    let manual = match &town.phase {
        Phase::Open(day) => day.manual_route().map(|(route, nili)| (day, route, nili)),
        _ => None,
    };
    p.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(22.0),
        left: Val::Px(23.0),
        max_width: Val::Percent(66.0),
        ..column(7.0)
    })
    .with_children(|stack| {
        card(
            stack,
            Node {
                padding: UiRect::axes(Val::Px(15.0), Val::Px(8.0)),
                ..row(14.0)
            },
            Color::srgba(0.052, 0.173, 0.182, 0.96),
        )
        .with_children(|summary| {
            icon(summary, hud, if tutorial { "guide" } else { "ferry" }, 38.0);
            summary.spawn(column(0.0)).with_children(|details| {
                label(details, hud, ch.district, 30.0, Face::Serif, SAND);
                label(
                    details,
                    hud,
                    format!(
                        "Tag {}  ·  {}",
                        town.day,
                        if town.planning() {
                            if ch.shift.night() {
                                "Nacht vorbereiten"
                            } else {
                                "Planen"
                            }
                        } else {
                            if ch.shift.night() {
                                "Nachtbetrieb"
                            } else {
                                "Der Tag läuft"
                            }
                        }
                    ),
                    15.0,
                    Face::Bold,
                    MUTED,
                );
                if tutorial {
                    label(
                        details,
                        hud,
                        format!("Tutorial {}/{}", town.chapter + 1, coach::ISLANDS),
                        15.0,
                        Face::Bold,
                        GOLD,
                    );
                }
            });
            summary
                .spawn(Node {
                    width: Val::Px(1.0),
                    height: Val::Px(35.0),
                    ..default()
                })
                .insert(BackgroundColor(LINE));
            icon(
                summary,
                hud,
                if ch.shift.night() {
                    "moon"
                } else {
                    weather_icon(weather)
                },
                30.0,
            );
            summary.spawn(column(0.0)).with_children(|sky| {
                label(
                    sky,
                    hud,
                    if ch.shift.night() && weather == Weather::Clear {
                        "Klare Nacht"
                    } else {
                        weather.name()
                    },
                    17.0,
                    Face::Bold,
                    SAND,
                );
                label(sky, hud, ch.shift.hours(), 14.0, Face::Body, MUTED);
            });
        });
        if matches!(town.phase, Phase::Open(_)) {
            card(stack, frame_node(Val::Px(370.0), 6.0, 10.0), INK).with_children(|time| {
                time.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row(6.0)
                })
                .with_children(|r| {
                    r.spawn((
                        Dynamic::Clock,
                        Text::new(ch.shift.clock(0.0)),
                        TextColor(GOLD),
                        TextFont {
                            font: hud.fonts[1].clone(),
                            font_size: 17.0,
                            ..default()
                        },
                    ));
                    r.spawn((
                        Dynamic::Sales,
                        Text::new(""),
                        TextColor(SAND),
                        TextFont {
                            font: hud.fonts[0].clone(),
                            font_size: 15.0,
                            ..default()
                        },
                    ));
                });
                time.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(LINE),
                    BorderRadius::all(Val::Px(3.0)),
                ))
                .with_children(|track| {
                    track.spawn((
                        DynamicBar,
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(GOLD),
                        BorderRadius::all(Val::Px(3.0)),
                    ));
                });
            });
        }
        if let Some(voyage) = town.voyage {
            card(stack, frame_node(Val::Auto, 5.0, 8.0), INK).with_children(|ship| {
                panels::metric(
                    ship,
                    hud,
                    "sea_trade",
                    format!("Schiff auf See · ab Tag {}", voyage.arrival),
                );
            });
        } else if town.planning() && town.cargo_today > 0 {
            card(stack, frame_node(Val::Auto, 5.0, 8.0), INK).with_children(|ship| {
                panels::metric(
                    ship,
                    hud,
                    "sea_trade",
                    format!("Schiff zurück · +{} Silber", town.cargo_today),
                );
            });
        }
    });

    if let Some((day, route, nili)) = manual {
        steering::draw(p, hud, town, game, day, route, nili);
        return;
    }

    let rail = Node {
        position_type: PositionType::Absolute,
        right: Val::Px(22.0),
        top: Val::Px(22.0),
        width: Val::Px(317.0),
        padding: UiRect::all(Val::Px(16.0)),
        border: UiRect::all(Val::Px(1.0)),
        ..column(11.0)
    };
    card(p, rail, Color::srgba(0.052, 0.173, 0.182, 0.97)).with_children(|rail| {
        rail.spawn(Node {
            justify_content: JustifyContent::SpaceBetween,
            ..row(8.0)
        })
        .with_children(|stats| {
            stat(
                stats,
                hud,
                "coin",
                &town.coins.to_string(),
                Region::of(town.chapter).currency(),
            );
            stat(
                stats,
                hud,
                "star",
                &town.totals.reputation.to_string(),
                "Ansehen",
            );
            info_button(stats, hud, Info::Harbor, hud.info == Some(Info::Harbor));
        });
        if hud.info == Some(Info::Harbor) {
            info_card(rail, hud, Info::Harbor, Region::of(town.chapter).currency());
        }
        guests::summary(rail, hud, town);
        line(rail, LINE);
        label(rail, hud, ch.name, 27.0, Face::Serif, SAND);
        let trade = crate::campaign_orders::trade_order(ch, town.chapter).and_then(|order| {
            [
                Goal::Sale(order, false),
                Goal::TradeUpgrade(order),
                Goal::Sale(order, true),
            ]
            .into_iter()
            .find(|&step| super::shown_goal(town, step).0 < step.target())
        });
        let visible = if trade.is_some() { 2 } else { 3 };
        for g in trade.into_iter().chain(ch.goals).take(visible) {
            goal(rail, hud, town, g);
        }
        line(rail, LINE);
        button(
            rail,
            hud,
            Action::Goals,
            "Aufträge",
            "book",
            "X",
            Style::Quiet,
            false,
        );
        button(
            rail,
            hud,
            Action::Forecast,
            "Tagesplan",
            "ferry",
            "RB",
            Style::Quiet,
            false,
        );
    });

    let walking = matches!(town.phase, Phase::Open(_));
    if walking && game.job.is_none() {
        steering::free(p, hud, game, town);
        if let Some(step) = coach::next(town, ch, game.selected, game.saw_prices) {
            p.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(183.0),
                bottom: Val::Px(96.0),
                ..column(3.0)
            })
            .with_children(|coach| prompts::strip(coach, hud, game, town, step));
        }
    }
    if !walking || game.job.is_some() {
        if walking {
            p.spawn(Node {
                position_type: PositionType::Absolute,
                left: Val::Px(23.0),
                bottom: Val::Px(23.0),
                ..default()
            })
            .with_children(|pad| steering::pad(pad, hud));
        }
        let (place, tile, desc) = parcel_labels(town, game.selected);
        let plan = game.job.map(|job| (job, job.plan(town, ch, game.selected)));
        let step = coach::next(town, ch, game.selected, game.saw_prices);
        let (primary, hot, ico, action) = if plan.is_some() {
            ("Nili steuern", "A", "nili", Action::Dispatch)
        } else if town.favor_plan(ch, game.selected).is_some() {
            ("Nili steuern", "A", "nili", Action::Aid)
        } else if town.planning() && town.lots[game.selected].is_none() {
            ("Bauen", "A", "hammer", Action::Plot)
        } else if town.planning()
            && town.lots[game.selected].is_some_and(|b| b.kind.service().is_some())
        {
            ("Preis und Ausbau", "A", "coin", Action::Plot)
        } else {
            ("Details", "A", tile, Action::Plot)
        };
        let footer = Node {
            position_type: PositionType::Absolute,
            left: Val::Px(if walking { 183.0 } else { 22.0 }),
            bottom: Val::Px(23.0),
            right: Val::Px(360.0),
            ..column(9.0)
        };
        p.spawn(footer).with_children(|foot| {
            if let Some((job, status)) = plan {
                card(foot, frame_node(Val::Px(520.0), 4.0, 10.0), DEEP).with_children(|help| {
                    help.spawn(row(7.0)).with_children(|r| {
                        icon(r, hud, job.icon(), 29.0);
                        label(r, hud, job.name(), 20.0, Face::Bold, GOLD);
                    });
                    let ready = status.is_ok();
                    if let Ok(route) = status {
                        route_stops(help, hud, town, route);
                    } else if let Err(reason) = status {
                        label(help, hud, reason.label(job), 16.0, Face::Body, SAND);
                    }
                    help.spawn(row(5.0)).with_children(|r| {
                        dpad(r, hud, 26.0);
                        label(
                            r,
                            hud,
                            if ready {
                                "Ziel wechseln"
                            } else {
                                "Ziel wählen"
                            },
                            15.0,
                            Face::Bold,
                            MUTED,
                        );
                    });
                });
            } else if let Some(step) = step {
                prompts::strip(foot, hud, game, town, step);
            }
            card(
                foot,
                Node {
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::SpaceBetween,
                    row_gap: Val::Px(6.0),
                    ..row(10.0)
                },
                INK,
            )
            .with_children(|bar| {
                bar.spawn(row(6.0)).with_children(|details| {
                    icon(details, hud, tile, 40.0);
                    details.spawn(column(1.0)).with_children(|caption| {
                        label(caption, hud, place, 20.0, Face::Bold, SAND);
                        if matches!(town.phase, Phase::Open(_))
                            && town.lots[game.selected].is_some_and(|b| b.kind.service().is_some())
                        {
                            caption.spawn((
                                Dynamic::Stock(game.selected),
                                Text::new(desc),
                                TextColor(MUTED),
                                TextFont {
                                    font: hud.fonts[0].clone(),
                                    font_size: 15.0,
                                    ..default()
                                },
                            ));
                        } else {
                            label(caption, hud, desc, 15.0, Face::Body, MUTED);
                        }
                    });
                });
                bar.spawn(row(8.0)).with_children(|actions| {
                    button(
                        actions,
                        hud,
                        action,
                        primary,
                        ico,
                        hot,
                        if plan.is_some() {
                            Style::Primary
                        } else {
                            Style::Quiet
                        },
                        plan.is_some_and(|(_, status)| status.is_err()),
                    );
                    if plan.is_some() {
                        button(
                            actions,
                            hud,
                            Action::Back,
                            "Touren",
                            "",
                            "B",
                            Style::Quiet,
                            false,
                        );
                    } else if town.planning() {
                        button(
                            actions,
                            hud,
                            Action::Open,
                            if ch.shift.night() {
                                "Nacht starten"
                            } else {
                                "Tag starten"
                            },
                            "sun",
                            "Y",
                            if step.is_none() {
                                Style::Primary
                            } else {
                                Style::Quiet
                            },
                            false,
                        );
                    } else {
                        let label = format!("Tempo {}×", game.speed);
                        button(
                            actions,
                            hud,
                            Action::Speed,
                            &label,
                            "",
                            "Y",
                            Style::Quiet,
                            false,
                        );
                    }
                });
            });
        });
    }
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(22.0),
        bottom: Val::Px(23.0),
        width: Val::Px(317.0),
        align_items: AlignItems::Stretch,
        ..column(7.0)
    })
    .with_children(|actions| {
        if walking && game.job.is_none() {
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
        }
        if town.chapter > 0 {
            button(
                actions,
                hud,
                Action::Nili,
                "Nilis Touren",
                "nili",
                "R3",
                Style::Quiet,
                false,
            );
        }
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

fn route_stops(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, route: crate::errands::Route) {
    use crate::errands::Stop;
    let symbol = |stop| match stop {
        Stop::Parcel(i) => town.lots[usize::from(i)].map_or("hammer", |b| kind_icon(b.kind)),
        _ => "ferry",
    };
    p.spawn(row(7.0)).with_children(|path| {
        icon(path, hud, symbol(route.pickup), 26.0);
        label(path, hud, route.pickup.label(town), 16.0, Face::Body, SAND);
        if route.pickup != route.delivery {
            label(path, hud, "›", 22.0, Face::Bold, GOLD);
            icon(path, hud, symbol(route.delivery), 26.0);
            label(
                path,
                hud,
                route.delivery.label(town),
                16.0,
                Face::Body,
                SAND,
            );
        }
    });
}

pub fn stat(p: &mut ChildSpawnerCommands, hud: &Hud, pic: &str, amount: &str, unit: &str) {
    p.spawn(row(6.0)).with_children(|cell| {
        icon(cell, hud, pic, 31.0);
        cell.spawn(column(0.0)).with_children(|numbers| {
            label(numbers, hud, amount, 25.0, Face::Bold, GOLD);
            label(numbers, hud, unit, 14.0, Face::Body, MUTED);
        });
    });
}

/// A report figure that counts up from zero the first time the report shows.
pub fn counted_stat(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    pic: &str,
    reveal: u64,
    value: Option<(u32, &'static str, &'static str)>,
    unit: &str,
) {
    p.spawn(row(6.0)).with_children(|cell| {
        icon(cell, hud, pic, 31.0);
        cell.spawn(column(0.0)).with_children(|numbers| {
            if let Some((value, prefix, suffix)) = value {
                feel::counter(numbers, hud, reveal, value, prefix, suffix);
            } else {
                label(numbers, hud, "—", 25.0, Face::Bold, GOLD);
            }
            label(numbers, hud, unit, 14.0, Face::Body, MUTED);
        });
    });
}

pub fn goal(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, goal: Goal) {
    let (have, need) = super::shown_goal(town, goal);
    let done = have >= need;
    p.spawn(column(4.0)).with_children(|quest| {
        quest
            .spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(8.0)
            })
            .with_children(|row| {
                let image = match goal {
                    Goal::Build(k, _) | Goal::Upgrade(k, _) | Goal::Pair(k, _, _) => kind_icon(k),
                    Goal::Serve(s, _) | Goal::Produce(s, _) => service_icon(s),
                    Goal::Sale(order, _) | Goal::TradeUpgrade(order) => kind_icon(order.shop),
                    Goal::Dogs(_) => "dog",
                    Goal::Clean(_) => "scraps",
                    Goal::GuestCare(age, ..) | Goal::FavoriteMenu(age, _) => age.icon(),
                    Goal::Rebuild(order) => kind_icon(order.new),
                    Goal::Renown(_) => "star",
                    Goal::Help(_) => "nili",
                    Goal::Earn(_) => "coin",
                    Goal::FeastGuests(_) => "ferry",
                    Goal::Variety(_) => "book",
                    Goal::MarketDay(_) => "coin",
                    Goal::Water(_) => "water",
                    Goal::Harvest(_) => "rice_paddy",
                    Goal::Bell(_) => "temple_bell",
                    Goal::Catch(_) => "fishery",
                    Goal::Voyages(_) => "sea_trade",
                    Goal::CargoIncome(_) | Goal::CaravanIncome(_) => "coin",
                    Goal::Caravans(_) | Goal::SpiceRoutes(_) => "caravan",
                    Goal::Route(job, _) => job.icon(),
                    Goal::RouteVariety(_) => "nili",
                    Goal::FerryDay(_) => "ferry",
                    Goal::GoodDay(_) => "mood",
                    Goal::Specials(_) => "star",
                    Goal::Residents(_) => "house",
                };
                row.spawn(row_node()).with_children(|line| {
                    line.spawn((
                        GoalBadge(goal, image),
                        ImageNode::new(hud.pictures[if done { "check" } else { image }].clone()),
                        Node {
                            width: Val::Px(22.0),
                            height: Val::Px(22.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                        FocusPolicy::Pass,
                    ));
                    label(line, hud, goal.label(), 16.0, Face::Body, SAND);
                });
                row.spawn((
                    Dynamic::Goal(goal),
                    Text::new(format!("{} / {need}", have.min(need))),
                    TextLayout::new_with_no_wrap(),
                    Node {
                        flex_shrink: 0.0,
                        ..default()
                    },
                    TextFont {
                        font: hud.fonts[1].clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(if done { MUTED } else { GOLD }),
                ));
            });
        quest
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(5.0),
                    ..default()
                },
                BorderRadius::all(Val::Px(3.0)),
                BackgroundColor(LINE),
            ))
            .with_children(|track| {
                let shown = have.min(need) as f32 / need.max(1) as f32 * 100.0;
                track.spawn((
                    GoalBar(goal, shown),
                    Node {
                        width: Val::Percent(shown),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(if done { MUTED } else { GOLD }),
                    BorderRadius::all(Val::Px(3.0)),
                ));
            });
    });
}

fn row_node() -> Node {
    row(5.0)
}

fn parcel_labels(town: &Town, lot: usize) -> (&'static str, &'static str, String) {
    if let Some(b) = town.lots[lot] {
        let text = match (&town.phase, b.kind.service()) {
            (Phase::Open(day), Some(service)) => stock_text(service, day.stock[lot]),
            _ => format!(
                "Stufe {}  ·  {}",
                b.level,
                if lot_row(lot) == 0 {
                    "Gasse"
                } else {
                    "Promenade"
                },
            ),
        };
        (b.kind.name(), kind_icon(b.kind), text)
    } else {
        (
            "Freier Bauplatz",
            "hammer",
            format!(
                "{}  ·  Platz {}",
                if lot_row(lot) == 0 {
                    "Gasse"
                } else {
                    "Promenade"
                },
                lot_col(lot) + 1
            ),
        )
    }
}

use super::*;
use crate::{
    campaign::{Chapter, Goal, REGION_SIZE, Region},
    model::{COLS, Town},
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
        ..column(19.0)
    })
    .with_children(|stack| {
        stack.spawn(row(8.0)).with_children(|brand| {
            icon(brand, hud, "ferry", 35.0);
            label(
                brand,
                hud,
                "NILIS REISE DURCH DIE ÄGÄIS",
                14.0,
                Face::Bold,
                GOLD,
            );
        });
        label(stack, hud, "Nililane", 94.0, Face::Serif, SAND);
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
        if let Some(town) = &save.town {
            button(
                stack,
                hud,
                Action::Resume,
                "Reise fortsetzen",
                "ferry",
                "A",
                Style::Primary,
                false,
            );
            label(
                stack,
                hud,
                format!("{}  ·  Tag {}", CHAPTERS[town.chapter].district, town.day),
                18.0,
                Face::Body,
                MUTED,
            );
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
        button(
            stack,
            hud,
            Action::Chart,
            "Kampagnenkarte",
            "chart",
            "",
            Style::Quiet,
            false,
        );
        stack
            .spawn(Node {
                width: Val::Percent(100.0),
                ..row(9.0)
            })
            .with_children(|row| {
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
        label(row, hud, "Nili · Hafenhelfer", 18.0, Face::Serif, SAND);
    });
}

pub fn chart(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, save: &Campaign) {
    let ch = &CHAPTERS[game.chart];
    let region = Region::of(game.chart);
    let offset = region.start();
    let second = game.chart - offset >= 12;
    let start = offset + if second { 12 } else { 0 };
    let mut node = frame_node(Val::Percent(95.0), 10.0, 19.0);
    node.max_width = Val::Px(1190.0);
    dark_scrim(p).with_children(|cover| {
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
                let completed = save.records[offset..offset + REGION_SIZE]
                    .iter()
                    .filter(|r| r.stars > 0)
                    .count();
                label(
                    title,
                    hud,
                    format!("{completed} von {REGION_SIZE} Häfen"),
                    18.0,
                    Face::Bold,
                    GOLD,
                );
            });
            body.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(9.0)
            })
            .with_children(|tabs| {
                tabs.spawn(row(7.0)).with_children(|routes| {
                    for r in [Region::Greece, Region::Rome] {
                        let locked = r == Region::Rome && !save.unlocked(r.start());
                        button(
                            routes,
                            hud,
                            Action::Region(r),
                            r.name(),
                            if locked { "lock" } else { r.icon() },
                            "",
                            if r == region {
                                Style::Primary
                            } else {
                                Style::Quiet
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
                        "Häfen 1–12",
                        "",
                        "LB",
                        if !second {
                            Style::Selected
                        } else {
                            Style::Quiet
                        },
                        !second,
                    );
                    button(
                        pages,
                        hud,
                        Action::ChartPage(true),
                        "Häfen 13–24",
                        "",
                        "RB",
                        if second {
                            Style::Selected
                        } else {
                            Style::Quiet
                        },
                        second,
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
                    label(
                        caption,
                        hud,
                        format!("{}  ·  {}", ch.district, ch.difficulty),
                        15.0,
                        Face::Bold,
                        GOLD,
                    );
                    label(caption, hud, ch.name, 27.0, Face::Serif, SAND);
                });
                let message = if save.unlocked(game.chart) {
                    ch.greeting.to_owned()
                } else {
                    format!(
                        "Schließe {} ab, um diesen Hafen freizuschalten.",
                        CHAPTERS[game.chart - 1].district
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
                let continuing = save.town.as_ref().is_some_and(|t| {
                    t.chapter == game.chart
                        && !matches!(t.phase, Phase::Report(ref r) if r.stars > 0)
                });
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

pub fn street(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    let ch = &CHAPTERS[town.chapter];
    let weather = town.weather(ch);
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
            icon(summary, hud, "ferry", 38.0);
            summary.spawn(column(0.0)).with_children(|details| {
                label(details, hud, ch.district, 30.0, Face::Serif, SAND);
                label(
                    details,
                    hud,
                    format!(
                        "Tag {}  ·  {}",
                        town.day,
                        if town.planning() {
                            "Morgenplanung"
                        } else {
                            "Hafen geöffnet"
                        }
                    ),
                    15.0,
                    Face::Bold,
                    MUTED,
                );
            });
            summary
                .spawn(Node {
                    width: Val::Px(1.0),
                    height: Val::Px(35.0),
                    ..default()
                })
                .insert(BackgroundColor(LINE));
            icon(summary, hud, weather_icon(weather), 30.0);
            label(summary, hud, weather.name(), 17.0, Face::Bold, SAND);
        });
        if matches!(town.phase, Phase::Open(_)) {
            card(stack, frame_node(Val::Px(290.0), 6.0, 10.0), INK).with_children(|time| {
                time.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row(6.0)
                })
                .with_children(|r| {
                    r.spawn((
                        Dynamic::Clock,
                        Text::new("07:00"),
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
    });

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
        });
        card(
            rail,
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..row(4.0)
            },
            CARD,
        )
        .with_children(|meter| {
            meter.spawn(row(4.0)).with_children(|caption| {
                icon(caption, hud, "mood", 30.0);
                label(caption, hud, "Zufriedenheit", 16.0, Face::Bold, SAND);
            });
            meter.spawn((
                Dynamic::Mood,
                Text::new("—"),
                TextColor(GOLD),
                TextFont {
                    font: hud.fonts[1].clone(),
                    font_size: 22.0,
                    ..default()
                },
            ));
        });
        line(rail, LINE);
        label(rail, hud, ch.name, 27.0, Face::Serif, SAND);
        for g in ch.goals {
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
            "Hafenbuch",
            "ferry",
            "RB",
            Style::Quiet,
            false,
        );
    });

    let (place, tile, desc) = parcel_labels(town, game.selected);
    let (primary, hot, ico, action) = if can_aid(town, game.selected) {
        ("Nili hilft", "A", "nili", Action::Aid)
    } else if town.planning() && town.lots[game.selected].is_none() {
        ("Bauen", "A", "hammer", Action::Plot)
    } else {
        ("Ansehen", "A", tile, Action::Plot)
    };
    let footer = Node {
        position_type: PositionType::Absolute,
        left: Val::Px(22.0),
        bottom: Val::Px(23.0),
        right: Val::Px(360.0),
        ..column(9.0)
    };
    p.spawn(footer).with_children(|foot| {
        if let Some(coach) = tutorial(town, game.selected, ch) {
            card(
                foot,
                Node {
                    width: Val::Px(505.0),
                    max_width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(11.0), Val::Px(7.0)),
                    ..row(12.0)
                },
                DEEP,
            )
            .with_children(|hint| {
                icon(hint, hud, "nili", 53.0);
                hint.spawn(column(1.0)).with_children(|copy| {
                    label(copy, hud, coach.0, 16.0, Face::Bold, GOLD);
                    label(copy, hud, coach.1, 16.0, Face::Body, SAND);
                });
            });
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
                button(actions, hud, action, primary, ico, hot, Style::Quiet, false);
                if town.planning() {
                    button(
                        actions,
                        hud,
                        Action::Open,
                        "Tag öffnen",
                        "sun",
                        "Y",
                        Style::Primary,
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
                        Style::Primary,
                        false,
                    );
                }
            });
        });
    });
    p.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(22.0),
        bottom: Val::Px(23.0),
        width: Val::Px(317.0),
        align_items: AlignItems::Stretch,
        ..column(7.0)
    })
    .with_children(|actions| {
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

pub fn stat(p: &mut ChildSpawnerCommands, hud: &Hud, pic: &str, amount: &str, unit: &str) {
    p.spawn(row(6.0)).with_children(|cell| {
        icon(cell, hud, pic, 31.0);
        cell.spawn(column(0.0)).with_children(|numbers| {
            label(numbers, hud, amount, 25.0, Face::Bold, GOLD);
            label(numbers, hud, unit, 14.0, Face::Body, MUTED);
        });
    });
}

pub fn goal(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, goal: Goal) {
    let (have, need) = town.goal(goal);
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
                    Goal::Serve(s, _) => service_icon(s),
                    Goal::Renown(_) => "star",
                    Goal::Help(_) => "nili",
                    Goal::Earn(_) => "coin",
                    Goal::FeastGuests(_) => "ferry",
                    Goal::Variety(_) => "book",
                    Goal::MarketDay(_) => "coin",
                    Goal::Water(_) => "water",
                };
                row.spawn(row_node()).with_children(|line| {
                    icon(line, hud, if done { "check" } else { image }, 22.0);
                    label(line, hud, goal.label(), 16.0, Face::Body, SAND);
                });
                row.spawn((
                    Dynamic::Goal(goal),
                    Text::new(format!("{} / {need}", have.min(need))),
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
                track.spawn((
                    GoalBar(goal),
                    Node {
                        width: Val::Percent(have.min(need) as f32 / need as f32 * 100.0),
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
        let stock = match &town.phase {
            Phase::Open(_) if b.kind.service().is_some() => "Laden geöffnet".to_owned(),
            _ => format!(
                "Stufe {}  ·  {}",
                b.level,
                if lot < COLS { "Gasse" } else { "Promenade" }
            ),
        };
        (b.kind.name(), kind_icon(b.kind), stock)
    } else {
        (
            "Freier Bauplatz",
            "hammer",
            format!(
                "{}  ·  Platz {}",
                if lot < COLS { "Gasse" } else { "Promenade" },
                lot % COLS + 1
            ),
        )
    }
}

fn tutorial(town: &Town, lot: usize, ch: &Chapter) -> Option<(&'static str, &'static str)> {
    if town.chapter != 0 || town.day > 2 {
        if let Phase::Open(day) = &town.phase
            && day.favor_at.is_none()
            && day.seconds >= 8.0
        {
            return Some((day.favor.name(), day.favor.note()));
        }
        return None;
    }
    Some(match &town.phase {
        Phase::Plan if town.count(Kind::Bakery) == 0 => (
            "1 · Bauplatz wählen",
            "Der goldene Rahmen zeigt deinen Bauplatz. Drücke A oder tippe auf Bauen. Wähle die Backstube.",
        ),
        Phase::Plan if town.day == 2 && town.count(Kind::Well) == 0 => (
            "4 · Morgen verbessern",
            "Ein Brunnen kostet 13 Drachmen und hilft der Backstube. Schau mit RB ins Hafenbuch, bevor du öffnest.",
        ),
        Phase::Plan => (
            "2 · Die Gasse öffnen",
            "Im Grundstücksmenü kannst du günstiges Brot anbieten. Drücke Y, wenn der Ofen bereit ist.",
        ),
        Phase::Open(day) if day.favor_at.is_some() => (
            "Geliefert!",
            "Achte auf Läden und Gäste. Nach Sonnenuntergang siehst du, was sich morgen lohnt.",
        ),
        Phase::Open(day) if day.seconds < 8.0 => (
            "Die ersten Gäste",
            "Einige kommen von den Wohnhöfen, andere wie Kleio mit der Fähre vom rechten Kai.",
        ),
        Phase::Open(_) if can_aid(town, lot) => (
            "3 · Nili hilft",
            "Drücke A oder tippe auf Nili hilft. Nili bringt den Vorrat und bekommt eine kleine Belohnung.",
        ),
        Phase::Open(_) => (
            ch.favors[(town.day as usize - 1) % ch.favors.len()].name(),
            ch.favors[(town.day as usize - 1) % ch.favors.len()].note(),
        ),
        _ => (
            "Das Hafenbuch",
            "X zeigt die Aufträge. RB öffnet die Übersicht für Gäste und Vorräte.",
        ),
    })
}

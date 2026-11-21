use super::*;
use crate::{
    campaign::{Chapter, REGION_SIZE, Region},
    model::{HarborPlan, MAX_LEVEL, Miss, Price, Report, SERVICES, Town},
};

pub fn street_panel(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    let chapter = &CHAPTERS[town.chapter];
    match game.panel {
        Panel::Brief => brief(p, hud, town, chapter),
        Panel::Build => build(p, hud, game, town, chapter),
        Panel::Details => details(p, hud, game, town, chapter),
        Panel::Goals => goals(p, hud, town, chapter),
        Panel::Forecast => forecast(p, hud, game, town, chapter),
        Panel::Pause => pause(p, hud),
        Panel::Demolish => demolish(p, hud, game, town),
        Panel::Report => {
            if let Phase::Report(report) = &town.phase {
                evening(p, hud, town, chapter, report);
            }
        }
        _ => {}
    }
}

fn modal(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    width: f32,
    heading: &str,
    title: &str,
    image: &str,
    content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    dark_scrim(p).with_children(|overlay| {
        let mut node = frame_node(Val::Percent(93.0), 14.0, 24.0);
        node.max_width = Val::Px(width);
        card(overlay, node, INK).with_children(|page| {
            page.spawn(row(12.0)).with_children(|head| {
                icon(head, hud, image, 54.0);
                head.spawn(column(1.0)).with_children(|words| {
                    if !heading.is_empty() {
                        label(words, hud, heading, 15.0, Face::Bold, GOLD);
                    }
                    label(words, hud, title, 40.0, Face::Serif, SAND);
                });
            });
            line(page, LINE);
            content(page);
        });
    });
}

fn brief(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let heading = format!(
        "{}  ·  HAFEN {}  ·  {}",
        Region::of(town.chapter).name().to_uppercase(),
        town.chapter - Region::of(town.chapter).start() + 1,
        chapter.district.to_uppercase()
    );
    modal(p, hud, 850.0, &heading, chapter.name, "ferry", |page| {
        page.spawn(Node {
            align_items: AlignItems::Stretch,
            ..row(20.0)
        })
        .with_children(|row| {
            chapter_picture(row, hud, town.chapter, Val::Px(230.0), Val::Px(112.0));
            label(row, hud, chapter.greeting, 20.0, Face::Body, SAND);
        });
        hint(
            page,
            hud,
            if town.chapter == 0 {
                "Nili zeigt dir die ersten Schritte. Mit dem Steuerkreuz wechselst du den Bauplatz. A öffnet seine Aktionen, Y startet den Tag."
            } else {
                chapter.hint
            },
            "nili",
        );
        button(
            page,
            hud,
            Action::Back,
            if town.chapter == 0 {
                "Tutorial beginnen"
            } else {
                "An Land gehen"
            },
            "ferry",
            "A",
            Style::Primary,
            false,
        );
    });
}

fn build(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, chapter: &Chapter) {
    let balance = format!(
        "FREIER BAUPLATZ  ·  {} {}",
        town.coins,
        Region::of(town.chapter).currency().to_uppercase()
    );
    modal(
        p,
        hud,
        1110.0,
        &balance,
        "Was entsteht hier?",
        "hammer",
        |page| {
            if chapter.allowed().len() > 8 {
                page.spawn(row(7.0)).with_children(|tabs| {
                    for (i, name) in ["Höfe & Läden", "Sonderbauten"].into_iter().enumerate() {
                        button(
                            tabs,
                            hud,
                            Action::BuildPage(i),
                            name,
                            "",
                            "",
                            if game.build_page == i {
                                Style::Primary
                            } else {
                                Style::Quiet
                            },
                            false,
                        );
                    }
                });
            }
            for group in build_set(chapter, game.build_page).chunks(4) {
                page.spawn(Node {
                    align_items: AlignItems::Stretch,
                    ..row(9.0)
                })
                .with_children(|choices| {
                    for &kind in group {
                        let afford = town.coins >= kind.cost();
                        let suggested = town.chapter == 0
                            && town.count(Kind::Bakery) == 0
                            && kind == Kind::Bakery;
                        button_shell(
                            choices,
                            Action::Build(kind),
                            if suggested {
                                Style::Selected
                            } else {
                                Style::Quiet
                            },
                            !afford,
                            Node {
                                flex_grow: 1.0,
                                flex_basis: Val::Px(0.0),
                                min_width: Val::Px(0.0),
                                min_height: Val::Px(145.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..column(5.0)
                            },
                        )
                        .with_children(|item| {
                            item.spawn(Node {
                                justify_content: JustifyContent::SpaceBetween,
                                ..row(4.0)
                            })
                            .with_children(|head| {
                                icon(head, hud, kind_icon(kind), 49.0);
                                head.spawn(row(4.0)).with_children(|cost| {
                                    icon(cost, hud, "coin", 23.0);
                                    label(
                                        cost,
                                        hud,
                                        kind.cost().to_string(),
                                        20.0,
                                        Face::Bold,
                                        if afford { GOLD } else { MUTED },
                                    );
                                });
                            });
                            label(item, hud, kind.name(), 21.0, Face::Bold, SAND);
                            label(item, hud, kind.note(), 15.0, Face::Body, MUTED);
                        });
                    }
                });
            }
            button(
                page,
                hud,
                Action::Back,
                "Zurück zum Bauplatz",
                "",
                "B",
                Style::Quiet,
                false,
            );
        },
    );
}

fn details(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, chapter: &Chapter) {
    let lot = game.selected;
    let Some(b) = town.lots[lot] else {
        modal(
            p,
            hud,
            620.0,
            "HAFEN GEÖFFNET",
            "Freier Bauplatz",
            "hammer",
            |page| {
                label(
                    page,
                    hud,
                    "Am nächsten Morgen können hier Arbeiter anrücken.",
                    20.0,
                    Face::Body,
                    SAND,
                );
                button(
                    page,
                    hud,
                    Action::Back,
                    "Zurück",
                    "",
                    "B",
                    Style::Primary,
                    false,
                );
            },
        );
        return;
    };
    let currency = Region::of(town.chapter).currency();
    let heading = format!(
        "STUFE {}  ·  {} {}",
        b.level,
        town.coins,
        currency.to_uppercase()
    );
    modal(
        p,
        hud,
        760.0,
        &heading,
        b.kind.name(),
        kind_icon(b.kind),
        |page| {
            label(page, hud, b.kind.note(), 20.0, Face::Body, SAND);
            if let Some(service) = b.kind.service() {
                let stock = match &town.phase {
                    Phase::Open(day) => day.stock[lot],
                    _ => town.production(chapter).stock[lot],
                };
                hint(
                    page,
                    hud,
                    &format!(
                        "{stock} Portionen {}  ·  {} {currency} je Verkauf",
                        service.name(),
                        b.price.coins(service)
                    ),
                    service_icon(service),
                );
                if town.planning() {
                    label(page, hud, "Preis pro Besuch", 17.0, Face::Bold, GOLD);
                    page.spawn(row(9.0)).with_children(|prices| {
                        for price in Price::ALL {
                            button(
                                prices,
                                hud,
                                Action::Price(price),
                                &format!("{}  ·  {}", price.label(), price.coins(service)),
                                if price == b.price { "check" } else { "" },
                                "",
                                if price == b.price {
                                    Style::Primary
                                } else {
                                    Style::Quiet
                                },
                                false,
                            );
                        }
                    });
                }
            }
            if town.planning() {
                if let Some(estimate) = upgrade_estimate(town, chapter, lot) {
                    label(page, hud, estimate, 17.0, Face::Body, MUTED);
                }
                line(page, LINE);
                page.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row(10.0)
                })
                .with_children(|actions| {
                    if b.level < MAX_LEVEL {
                        button(
                            actions,
                            hud,
                            Action::Upgrade,
                            &format!("Ausbauen  ·  {}", b.upgrade_cost()),
                            "hammer",
                            "",
                            Style::Quiet,
                            town.coins < b.upgrade_cost(),
                        );
                    }
                    button(
                        actions,
                        hud,
                        Action::Demolish,
                        "Rückbau",
                        "",
                        "",
                        Style::Danger,
                        false,
                    );
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
                });
            } else {
                button(
                    page,
                    hud,
                    Action::Back,
                    "Zurück",
                    "",
                    "B",
                    Style::Primary,
                    false,
                );
            }
        },
    );
}

fn upgrade_estimate(town: &Town, chapter: &Chapter, lot: usize) -> Option<String> {
    let building = town.lots[lot]?;
    if building.level >= MAX_LEVEL {
        return None;
    }
    let mut preview = town.clone();
    let next = crate::model::Building {
        level: building.level + 1,
        ..building
    };
    preview.lots[lot] = Some(next);
    let before = town.production(chapter);
    let after = preview.production(chapter);
    let (from, to, unit) = if building.residents() > 0 {
        (building.residents(), next.residents(), "Nachbarn")
    } else if building.kind.water_source() {
        (before.water, after.water, "Wasserkrüge")
    } else if building.kind == Kind::Grove {
        (before.olives, after.olives, "Olivenportionen")
    } else if building.kind == Kind::Storehouse {
        (
            u16::from(building.level),
            u16::from(next.level),
            "Zusatzportionen pro Laden in Reichweite",
        )
    } else {
        (before.stock[lot], after.stock[lot], "Portionen")
    };
    Some(format!(
        "Stufe {} bei diesem Tagesplan  ·  {from} → {to} {unit}",
        next.level
    ))
}

fn goals(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    modal(
        p,
        hud,
        770.0,
        chapter.district,
        "Aufträge",
        "book",
        |page| {
            label(page, hud, chapter.greeting, 19.0, Face::Body, SAND);
            for task in chapter.goals {
                super::screens::goal(page, hud, town, task);
            }
            hint(page, hud, chapter.hint, "nili");
            label(
                page,
                hud,
                format!(
                    "Zusatzsterne für Abschluss bis Tag {} und {} erfüllte Hafenbitten.",
                    chapter.par, chapter.favor_goal
                ),
                17.0,
                Face::Body,
                MUTED,
            );
            button(
                page,
                hud,
                Action::Back,
                "Zur Gasse",
                "",
                "B",
                Style::Primary,
                false,
            );
        },
    );
}

fn forecast(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, chapter: &Chapter) {
    let books = ["Vorräte", "Zufriedenheit", "Tagesplan"];
    modal(
        p,
        hud,
        900.0,
        "GÄSTE & WERKSTÄTTEN",
        "Das Hafenbuch",
        "ferry",
        |page| {
            page.spawn(row(8.0)).with_children(|tabs| {
                for (i, name) in books.into_iter().enumerate() {
                    button(
                        tabs,
                        hud,
                        Action::LedgerPage(i),
                        name,
                        if i == 1 { "mood" } else { "" },
                        "",
                        if game.ledger_page == i {
                            Style::Primary
                        } else {
                            Style::Quiet
                        },
                        false,
                    );
                }
            });
            match game.ledger_page {
                0 => supplies(page, hud, town, chapter),
                1 => mood(page, hud, town),
                _ => day_plan(page, hud, town, chapter),
            }
            button(
                page,
                hud,
                Action::Back,
                "Zur Gasse",
                "",
                "B",
                Style::Quiet,
                false,
            );
        },
    );
}

fn supplies(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let (home, ferry) = town.visitors(chapter);
    let predicted = town.forecast(chapter);
    let production = town.production(chapter);
    let mut supply = [0; SERVICES];
    for (lot, building) in town.lots.iter().enumerate() {
        let Some(service) = building.and_then(|b| b.kind.service()) else {
            continue;
        };
        supply[service as usize] += match &town.phase {
            Phase::Open(day) => day.stock[lot],
            _ => production.stock[lot],
        };
    }
    page.spawn(row(22.0)).with_children(|figures| {
        super::screens::stat(figures, hud, "house", &home.to_string(), "Nachbarn");
        super::screens::stat(figures, hud, "ferry", &ferry.to_string(), "Fährgäste");
        super::screens::stat(
            figures,
            hud,
            "water",
            &format!("{}/{}", production.used_water, production.water),
            "Wasser genutzt",
        );
        if chapter.allowed().contains(&Kind::Grove) {
            super::screens::stat(
                figures,
                hud,
                "olive",
                &format!("{}/{}", production.used_olives, production.olives),
                "Oliven genutzt",
            );
        }
    });
    let services: Vec<_> = Service::ALL
        .into_iter()
        .filter(|s| predicted[*s as usize] != 0 || supply[*s as usize] != 0)
        .collect();
    let columns = if services.len() <= 5 {
        services.len().max(1)
    } else {
        services.len().div_ceil(2)
    };
    for group in services.chunks(columns) {
        page.spawn(Node {
            align_items: AlignItems::Stretch,
            ..row(7.0)
        })
        .with_children(|cards| {
            for &service in group {
                let i = service as usize;
                card(
                    cards,
                    Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        min_width: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..column(1.0)
                    },
                    CARD,
                )
                .with_children(|tile| {
                    icon(tile, hud, service_icon(service), 30.0);
                    label(tile, hud, service.name(), 17.0, Face::Bold, SAND);
                    label(
                        tile,
                        hud,
                        format!("{} gefragt", predicted[i]),
                        15.0,
                        Face::Body,
                        MUTED,
                    );
                    label(
                        tile,
                        hud,
                        format!(
                            "{} {}",
                            supply[i],
                            if town.planning() { "bereit" } else { "übrig" }
                        ),
                        16.0,
                        Face::Bold,
                        if supply[i] < predicted[i] { GOLD } else { SAND },
                    );
                });
            }
            for _ in group.len()..columns {
                cards.spawn(Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    ..default()
                });
            }
        });
    }
}

fn mood(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town) {
    let Some(day) = town.feedback().filter(|day| day.responses() > 0) else {
        hint(
            page,
            hud,
            "Die ersten Gäste sind noch unterwegs. Hier siehst du, was ihnen gefällt und was ihnen fehlt.",
            "mood",
        );
        return;
    };
    page.spawn(row(24.0)).with_children(|figures| {
        super::screens::stat(
            figures,
            hud,
            "mood",
            &format!("{} %", day.satisfaction().unwrap_or(0)),
            if town.planning() {
                "Zufrieden gestern"
            } else {
                "Zufrieden heute"
            },
        );
        super::screens::stat(
            figures,
            hud,
            "check",
            &format!("{}/{}", day.served_total(), day.responses()),
            "Wünsche erfüllt",
        );
    });
    page.spawn(row(8.0)).with_children(|reasons| {
        for (reason, image) in Miss::ALL
            .into_iter()
            .zip(["hammer", "ferry", "coin", "storehouse"])
        {
            card(
                reasons,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(9.0)),
                    ..column(2.0)
                },
                CARD,
            )
            .with_children(|item| {
                icon(item, hud, image, 31.0);
                label(
                    item,
                    hud,
                    day.gaps[reason as usize].to_string(),
                    22.0,
                    Face::Bold,
                    GOLD,
                );
                label(item, hud, reason.label(), 16.0, Face::Body, SAND);
            });
        }
    });
    if let Some(why) = Miss::ALL
        .into_iter()
        .filter(|m| day.gaps[*m as usize] > 0)
        .max_by_key(|m| day.gaps[*m as usize])
    {
        hint(page, hud, why.hint(), "nili");
    } else {
        hint(
            page,
            hud,
            "Alle bisherigen Gäste haben gefunden, was sie gesucht haben.",
            "check",
        );
    }
}

fn day_plan(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let weather = town.weather(chapter);
    page.spawn(row(10.0)).with_children(|days| {
        for (index, name) in ["Heute", "Morgen", "Übermorgen"].into_iter().enumerate() {
            let forecast = town.weather_ahead(chapter, index);
            card(
                days,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..row(7.0)
                },
                CARD,
            )
            .with_children(|day| {
                icon(day, hud, weather_icon(forecast), 34.0);
                day.spawn(column(0.0)).with_children(|caption| {
                    label(caption, hud, name, 14.0, Face::Body, MUTED);
                    label(caption, hud, forecast.name(), 19.0, Face::Bold, SAND);
                });
            });
        }
    });
    hint(page, hud, weather.note(), weather_icon(weather));
    let favor = town.next_favor(chapter);
    let done = matches!(&town.phase, Phase::Open(day) if day.favor_at.is_some());
    hint(
        page,
        hud,
        &if done {
            "Nili hat die Hafenbitte erfüllt.".to_owned()
        } else {
            format!("{} · {}", favor.name(), favor.note())
        },
        "nili",
    );
    if chapter.harbor_plans() {
        label(
            page,
            hud,
            "Hafenbeschluss für heute",
            19.0,
            Face::Bold,
            GOLD,
        );
        page.spawn(row(8.0)).with_children(|choices| {
            for choice in HarborPlan::ALL {
                button(
                    choices,
                    hud,
                    Action::HarborPlan(choice),
                    choice.name(),
                    "",
                    "",
                    if town.harbor == choice {
                        Style::Primary
                    } else {
                        Style::Quiet
                    },
                    !town.planning(),
                );
            }
        });
        label(page, hud, town.harbor.note(), 18.0, Face::Body, MUTED);
    }
}

fn evening(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    report: &Report,
) {
    let clear = report.stars > 0;
    let ending = clear && town.chapter + 1 == CHAPTERS.len();
    let rome_unlocked = clear && town.chapter + 1 == REGION_SIZE;
    let title = if ending {
        "Ein Platz für ganz Rom"
    } else if rome_unlocked {
        "Das Bündnis steht"
    } else if clear {
        "Der Hafen blüht auf"
    } else {
        "Abend am Kai"
    };
    modal(
        p,
        hud,
        872.0,
        &format!("{}  ·  TAG {}", chapter.district.to_uppercase(), town.day),
        title,
        if clear { "star" } else { "sun" },
        |page| {
            page.spawn(row(24.0)).with_children(|figures| {
                super::screens::stat(
                    figures,
                    hud,
                    "coin",
                    &format!("+{}", report.tally.income + report.grant),
                    &format!("{} heute", Region::of(town.chapter).currency()),
                );
                super::screens::stat(
                    figures,
                    hud,
                    "house",
                    &report.tally.served_total().to_string(),
                    "Gäste versorgt",
                );
                super::screens::stat(
                    figures,
                    hud,
                    "star",
                    &format!("+{}", report.tally.reputation),
                    "Ansehen heute",
                );
                if report.tally.favors > 0 {
                    super::screens::stat(figures, hud, "nili", "1", "Hafenbitte");
                }
            });
            let peak = Service::ALL
                .into_iter()
                .filter(|s| report.tally.missed[*s as usize] > 0)
                .max_by_key(|s| report.tally.missed[*s as usize]);
            if clear {
                page.spawn(row(5.0)).with_children(|stars| {
                    for _ in 0..report.stars {
                        icon(stars, hud, "star", 35.0);
                    }
                    label(
                        stars,
                        hud,
                        format!("{} von 3 Sternen", report.stars),
                        20.0,
                        Face::Bold,
                        GOLD,
                    );
                });
                label(
                    page,
                    hud,
                    if ending {
                        "Flavia und Gaius stoßen auf dem Forum mit Nili an. In der Kampagnenkarte stehen alle 48 Häfen und deine erreichten Sterne."
                    } else {
                        "Der nächste Hafen ist auf der Kampagnenkarte freigeschaltet. Deine Sterne sind im Logbuch eingetragen."
                    },
                    19.0,
                    Face::Body,
                    SAND,
                );
            } else {
                for goal in chapter.goals {
                    super::screens::goal(page, hud, town, goal);
                }
                if let Some(s) = peak {
                    hint(
                        page,
                        hud,
                        &format!(
                            "{} Gäste suchten vergeblich nach {}. Mehr Vorrat, kürzere Wege oder ein günstigerer Preis helfen.",
                            report.tally.missed[s as usize],
                            s.name()
                        ),
                        service_icon(s),
                    );
                } else {
                    hint(
                        page,
                        hud,
                        "Alle Wünsche erfüllt. Plane morgen weiter, um die Aufträge des Hafens abzuschließen.",
                        "check",
                    );
                }
            }
            page.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(9.0)
            })
            .with_children(|actions| {
                button(
                    actions,
                    hud,
                    Action::Chart,
                    "Kampagnenkarte",
                    "chart",
                    "B",
                    Style::Quiet,
                    false,
                );
                if clear {
                    button(
                        actions,
                        hud,
                        Action::NextDay,
                        "Im Hafen bleiben",
                        "",
                        "",
                        Style::Quiet,
                        false,
                    );
                    button(
                        actions,
                        hud,
                        Action::NextChapter,
                        if ending {
                            "Zur Kampagnenkarte"
                        } else if rome_unlocked {
                            "Nach Rom segeln"
                        } else {
                            "Nächster Hafen"
                        },
                        "ferry",
                        "A",
                        Style::Primary,
                        false,
                    );
                } else {
                    button(
                        actions,
                        hud,
                        Action::NextDay,
                        "Nächster Morgen",
                        "sun",
                        "A",
                        Style::Primary,
                        false,
                    );
                }
            });
        },
    );
}

fn pause(p: &mut ChildSpawnerCommands, hud: &Hud) {
    modal(p, hud, 535.0, "", "Ruhe am Hafen", "nili", |page| {
        button(
            page,
            hud,
            Action::Back,
            "Zurück zum Spiel",
            "ferry",
            "A",
            Style::Primary,
            false,
        );
        button(
            page,
            hud,
            Action::Guide,
            "Spielhilfe",
            "guide",
            "",
            Style::Quiet,
            false,
        );
        button(
            page,
            hud,
            Action::Options,
            "Einstellungen",
            "",
            "",
            Style::Quiet,
            false,
        );
        button(
            page,
            hud,
            Action::Chart,
            "Kampagnenkarte",
            "chart",
            "",
            Style::Quiet,
            false,
        );
        button(
            page,
            hud,
            Action::Title,
            "Hauptmenü",
            "",
            "",
            Style::Quiet,
            false,
        );
    });
}

pub fn options(p: &mut ChildSpawnerCommands, hud: &Hud, save: &Campaign) {
    modal(p, hud, 615.0, "", "Einstellungen", "sun", |page| {
        for (action, name, enabled) in [
            (Action::LargeUi, "Große Oberfläche", save.large_ui),
            (Action::Music, "Musik", save.music),
            (Action::Sounds, "Umgebung und Klänge", save.sounds),
        ] {
            button(
                page,
                hud,
                action,
                &format!("{name}  ·  {}", if enabled { "An" } else { "Aus" }),
                if enabled { "check" } else { "" },
                "",
                Style::Quiet,
                false,
            );
        }
        label(
            page,
            hud,
            "Nililane speichert beim Bauen, nach Entscheidungen und regelmäßig während des Tages.",
            17.0,
            Face::Body,
            MUTED,
        );
        button(
            page,
            hud,
            Action::Back,
            "Zurück",
            "",
            "B",
            Style::Primary,
            false,
        );
    });
}

fn demolish(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town) {
    let Some(building) = town.lots[game.selected] else {
        return;
    };
    modal(
        p,
        hud,
        590.0,
        "",
        &format!("{} zurückbauen?", building.kind.name()),
        "hammer",
        |page| {
            label(
                page,
                hud,
                format!(
                    "Der Platz wird frei. Du erhältst {} {} zurück.",
                    building.refund(),
                    Region::of(town.chapter).currency()
                ),
                20.0,
                Face::Body,
                SAND,
            );
            page.spawn(row(12.0)).with_children(|row| {
                button(
                    row,
                    hud,
                    Action::Back,
                    "Behalten",
                    "",
                    "B",
                    Style::Primary,
                    false,
                );
                button(
                    row,
                    hud,
                    Action::ConfirmDemolish,
                    "Zurückbauen",
                    "",
                    "",
                    Style::Danger,
                    false,
                );
            });
        },
    );
}

pub fn replace(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game) {
    modal(
        p,
        hud,
        650.0,
        "KAMPAGNENKARTE",
        "Einen neuen Hafen beginnen?",
        "ferry",
        |page| {
            label(
                page,
                hud,
                format!(
                    "Du möchtest nach {}. Deine abgeschlossenen Kapitel und Sterne bleiben im Inselbuch. Die laufende Gasse wird durch den neuen Hafen ersetzt.",
                    CHAPTERS[game.chart].district
                ),
                19.0,
                Face::Body,
                SAND,
            );
            page.spawn(row(11.0)).with_children(|actions| {
                button(
                    actions,
                    hud,
                    Action::Back,
                    "Kampagnenkarte",
                    "",
                    "B",
                    Style::Primary,
                    false,
                );
                button(
                    actions,
                    hud,
                    Action::ConfirmSail,
                    "Ablegen",
                    "ferry",
                    "",
                    Style::Danger,
                    false,
                );
            });
        },
    );
}

fn hint(p: &mut ChildSpawnerCommands, hud: &Hud, content: &str, image: &str) {
    card(
        p,
        Node {
            padding: UiRect::axes(Val::Px(11.0), Val::Px(7.0)),
            min_height: Val::Px(50.0),
            ..row(9.0)
        },
        CARD,
    )
    .with_children(|box_| {
        icon(box_, hud, image, 36.0);
        label(box_, hud, content, 17.0, Face::Body, SAND);
    });
}

pub fn guide(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game) {
    const PAGES: [&str; 6] = [
        "Bedienung",
        "Tageslauf",
        "Waren",
        "Hafenbitten",
        "Reisen",
        "Rom",
    ];
    modal(p, hud, 985.0, "", "Nilis Spielhilfe", "guide", |page| {
        page.spawn(row(7.0)).with_children(|tabs| {
            for (i, name) in PAGES.iter().enumerate() {
                button(
                    tabs,
                    hud,
                    Action::GuideTab(i),
                    name,
                    "",
                    "",
                    if i == game.guide {
                        Style::Primary
                    } else {
                        Style::Quiet
                    },
                    false,
                );
            }
        });
        let details = match game.guide {
            0 => [
                (
                    "hammer",
                    "Bauplatz und Fokus",
                    "Steuerkreuz oder linker Stick wählen. A bestätigt. B geht zurück. Gold markiert den Bauplatz und die gewählte Schaltfläche.",
                ),
                (
                    "sun",
                    "Direkt zur Aktion",
                    "Y öffnet die Gasse und wechselt danach das Tempo. X zeigt Aufträge, RB das Hafenbuch. Mit Start öffnest du das Menü.",
                ),
                (
                    "chart",
                    "Die Aussicht",
                    "Rechter Stick schwenkt, LT und RT zoomen, LB setzt die Ansicht zurück. Maus und Touch wählen Bauplätze und Schaltflächen direkt.",
                ),
            ],
            1 => [
                (
                    "hammer",
                    "Morgen planen",
                    "Wähle einen Platz aus und baue. Bei einem fertigen Gebäude kannst du Preise festlegen, ausbauen und zurückbauen.",
                ),
                (
                    "ferry",
                    "Gäste beobachten",
                    "Nachbarn starten an ihren Wohnhöfen. Fährgäste kommen von rechts. Im Hafenbuch siehst du Wünsche und Vorräte. Offene Menüs halten die Uhr an.",
                ),
                (
                    "sun",
                    "Abend auswerten",
                    "Nach Sonnenuntergang kommen Verkäufe und sechs Münzen Hafengabe in die Kasse. Morgen kannst du mit dem Ertrag weiterbauen.",
                ),
            ],
            2 => [
                (
                    "water",
                    "Wasser und Oliven",
                    "Ein Brunnen versorgt Backstuben, Töpfereien und Häuser für Gäste. Nahe Werkstätten werden zuerst bedient. Bei Hitze brauchen sie mehr Wasser.",
                ),
                (
                    "meze",
                    "Eine kleine Lieferkette",
                    "Haine geben Oliven und Schatten. Tavernen brauchen zwei Laibe und zwei Olivenportionen für ihre volle Auslage. Der nächste Ofen liefert zuerst.",
                ),
                (
                    "coin",
                    "Laufwege und Preise",
                    "Gäste vergleichen Entfernung und Angebot. Manche können nur günstige Waren bezahlen. Nachbarn laufen bis vier Plätze, die Reichweite der Fähre hängt von der Insel ab.",
                ),
            ],
            3 => [
                (
                    "nili",
                    "Nili hilft",
                    "Wenn die Hafenglocke ertönt, wähle den Ort aus der Hafenbitte. Drücke A, sobald Nili hilft angezeigt wird. Jede Lieferung bringt sechs Münzen und zwei Ansehen.",
                ),
                (
                    "well",
                    "Eine Bitte pro Tag",
                    "Für Krüge zu einem Brunnen gehen, für Schatten zu Hain oder Stoa, für einen Eilkorb zu einem Laden. Die Wirkung steht bei der Bitte im Hafenbuch.",
                ),
                (
                    "star",
                    "Gute Nachbarschaft",
                    "Hafenbitten füllen Vorräte auf und zählen für Aufträge und die Zusatzsterne. Die nächste Bitte steht schon morgens fest.",
                ),
            ],
            4 => [
                (
                    "chart",
                    "Hafen für Hafen",
                    "Jeder Hafen hat drei Aufträge. Erfülle sie bis zu einer Abendabrechnung, um den nächsten Hafen freizuschalten.",
                ),
                (
                    "star",
                    "Drei Sterne",
                    "Ein Stern gehört dem Abschluss, einer dem Richttag und einer den Hafenbitten. Du kannst nach dem Richttag in Ruhe weiterspielen.",
                ),
                (
                    "book",
                    "Alles im Logbuch",
                    "Ein laufender Hafen wird automatisch gespeichert. Abgeschlossene Häfen und ihre besten Ergebnisse stehen dauerhaft auf der Kampagnenkarte.",
                ),
            ],
            _ => [
                (
                    "aqueduct",
                    "Wasser nach Rom",
                    "Brunnen speisen Aquädukte. Ein Aquädukt nahe der Therme erhöht die Zahl der Badeplätze. Regen füllt die Zisterne.",
                ),
                (
                    "forum",
                    "Wege zum Forum",
                    "Läden nahe am Forum sind für Fährgäste einen Bauplatz weiter erreichbar. Mit Villen wachsen Besucherzahl und Budgets.",
                ),
                (
                    "mood",
                    "Zufriedenheit prüfen",
                    "Das Hafenbuch zeigt den Anteil erfüllter Wünsche. Prüfe dort, ob Preis, Weg, Vorrat oder fehlende Läden die Ursache sind.",
                ),
            ],
        };
        for (image, title, words) in details {
            card(
                page,
                Node {
                    padding: UiRect::all(Val::Px(11.0)),
                    ..row(11.0)
                },
                CARD,
            )
            .with_children(|tile| {
                icon(tile, hud, image, 48.0);
                tile.spawn(column(2.0)).with_children(|text| {
                    label(text, hud, title, 20.0, Face::Bold, GOLD);
                    label(text, hud, words, 17.0, Face::Body, SAND);
                });
            });
        }
        button(
            page,
            hud,
            Action::Back,
            "Zurück",
            "",
            "B",
            Style::Quiet,
            false,
        );
    });
}

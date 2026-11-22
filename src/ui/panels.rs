use super::*;
use crate::{
    campaign::{Chapter, REGION_SIZE, Region},
    coach::{self, Step},
    errands::Job,
    model::{Assortment, Building, HarborPlan, MAX_LEVEL, Price, Report, SERVICES, Town},
};

pub fn street_panel(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    town: &Town,
    tutorial: bool,
) {
    let chapter = &CHAPTERS[town.chapter];
    match game.panel {
        Panel::Brief => brief(p, hud, town, chapter, tutorial),
        Panel::Build => build(p, hud, game, town, chapter),
        Panel::Details => details(p, hud, game, town, chapter),
        Panel::Assortment => assortment(p, hud, game, town, chapter),
        Panel::Goals => goals(p, hud, town, chapter),
        Panel::Forecast => forecast(p, hud, game, town, chapter),
        Panel::Nili => nili(p, hud, town, chapter),
        Panel::Pause => pause(p, hud, tutorial),
        Panel::Demolish => demolish(p, hud, game, town),
        Panel::Report => {
            if let Phase::Report(report) = &town.phase {
                evening(p, hud, town, chapter, report, tutorial);
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

fn brief(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter, tutorial: bool) {
    let region = Region::of(town.chapter);
    let heading = if tutorial {
        format!("TUTORIAL  {} / {}", town.chapter + 1, coach::ISLANDS)
    } else {
        format!(
            "{}  ·  HAFEN {} / {}",
            region.name().to_uppercase(),
            town.chapter - region.start() + 1,
            REGION_SIZE
        )
    };
    modal(
        p,
        hud,
        850.0,
        &heading,
        chapter.name,
        if tutorial { "guide" } else { region.icon() },
        |page| {
            let introduction = match town.chapter {
                0 => Some([
                    ("bakery", "Backstube bauen", "A"),
                    ("coin", "Brotpreis wählen", "A"),
                    ("sun", "Tag starten", "Y"),
                ]),
                1 => Some([
                    ("well", "Brunnen bauen", ""),
                    ("water", "Wasser bringen", ""),
                    ("bread", "Brot verkaufen", ""),
                ]),
                2 => Some([
                    ("potter", "Töpferei bauen", ""),
                    ("storehouse", "Nachschub bringen", ""),
                    ("hammer", "Töpferei ausbauen", ""),
                ]),
                _ => None,
            };
            if let Some(steps) = introduction {
                page.spawn(row(10.0)).with_children(|row| {
                    for (i, (art, word, pad)) in steps.into_iter().enumerate() {
                        if i > 0 {
                            label(row, hud, "›", 25.0, Face::Bold, GOLD);
                        }
                        card(
                            row,
                            Node {
                                flex_grow: 1.0,
                                flex_basis: Val::Px(0.0),
                                min_height: Val::Px(137.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(8.0)),
                                ..column(5.0)
                            },
                            CARD,
                        )
                        .with_children(|tile| {
                            icon(tile, hud, art, 61.0);
                            label(tile, hud, word, 19.0, Face::Bold, SAND);
                            if !pad.is_empty() {
                                key(tile, hud, pad, false);
                            }
                        });
                    }
                });
            } else {
                page.spawn(row(16.0)).with_children(|intro| {
                    chapter_picture(intro, hud, town.chapter, Val::Px(230.0), Val::Px(112.0));
                    intro.spawn(column(7.0)).with_children(|words| {
                        label(words, hud, chapter.greeting, 20.0, Face::Body, SAND);
                        metric(
                            words,
                            hud,
                            "hammer",
                            format!("{} Bauplätze", chapter.layout.count()),
                        );
                    });
                });
            }
            button(
                page,
                hud,
                Action::Back,
                if town.chapter == 0 {
                    "Los geht’s"
                } else {
                    "An Land gehen"
                },
                if town.chapter == 0 { "nili" } else { "ferry" },
                "A",
                Style::Primary,
                false,
            );
        },
    );
}

fn nili(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let used = if let Phase::Open(day) = &town.phase {
        day.tally.routes_today()
    } else {
        0
    };
    let heading = format!("{} / {} TOUREN HEUTE", used, Job::limit(town.chapter));
    modal(p, hud, 905.0, &heading, "Nilis Touren", "nili", |page| {
        let available: Vec<_> = Job::ALL
            .into_iter()
            .filter(|job| job.unlocked(town.chapter))
            .collect();
        for jobs in available.chunks(2) {
            page.spawn(Node {
                align_items: AlignItems::Stretch,
                ..row(8.0)
            })
            .with_children(|choices| {
                for &job in jobs {
                    let preview = super::job_preview(town, chapter, job);
                    button_shell(
                        choices,
                        Action::ChooseJob(job),
                        Style::Quiet,
                        preview.is_err(),
                        Node {
                            flex_grow: 1.0,
                            flex_basis: Val::Px(0.0),
                            min_height: Val::Px(96.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..column(5.0)
                        },
                    )
                    .with_children(|card| {
                        card.spawn(row(7.0)).with_children(|top| {
                            icon(top, hud, job.icon(), 31.0);
                            label(top, hud, job.name(), 21.0, Face::Bold, SAND);
                        });
                        label(card, hud, job.purpose(), 16.0, Face::Body, SAND);
                        if let Err(reason) = preview {
                            metric(card, hud, reason.symbol(), reason.label(job));
                        } else {
                            metric(card, hud, "check", "Ziel wählen");
                        }
                    });
                }
                if jobs.len() == 1 {
                    choices.spawn(Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        ..default()
                    });
                }
            });
        }
        button(
            page,
            hud,
            Action::Back,
            "Zurück zum Hafen",
            "",
            "B",
            Style::Quiet,
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
    let guide = coach::next(town, chapter, game.selected, game.saw_prices);
    let before = town.production(chapter);
    let mut preview = town.clone();
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
                        let suggested = guide == Some(Step::Build(game.selected, kind));
                        let building = Building::new(kind);
                        preview.lots[game.selected] = Some(building);
                        let after = preview.production(chapter);
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
                                icon(head, hud, kind_icon(kind), 51.0);
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
                            let (art, effect) = if let Some(service) = kind.service() {
                                (
                                    service_icon(service),
                                    stock_text(service, after.stock[game.selected]),
                                )
                            } else if kind.water_source() {
                                (
                                    "water",
                                    format!("+{} Krüge", after.water.saturating_sub(before.water)),
                                )
                            } else {
                                match kind {
                                    Kind::Grove => (
                                        "olive",
                                        format!(
                                            "+{} Oliven",
                                            after.olives.saturating_sub(before.olives)
                                        ),
                                    ),
                                    Kind::Storehouse => {
                                        let shops = before
                                            .stock
                                            .iter()
                                            .zip(after.stock)
                                            .filter(|&(old, new)| new > *old)
                                            .count();
                                        (
                                            "bread",
                                            match shops {
                                                0 => "Für nahe Läden".into(),
                                                1 => "+1 für 1 Laden".into(),
                                                _ => format!("+1 je Laden ({shops})"),
                                            },
                                        )
                                    }
                                    _ => ("house", format!("+{} Nachbarn", building.residents())),
                                }
                            };
                            metric(item, hud, art, effect);
                            if !afford {
                                label(
                                    item,
                                    hud,
                                    format!("Noch {} Münzen nötig", kind.cost() - town.coins),
                                    15.0,
                                    Face::Body,
                                    MUTED,
                                );
                            } else if suggested {
                                metric(item, hud, "check", "Nili empfiehlt");
                            } else if let Some((art, subtitle)) = building_link(kind) {
                                metric(item, hud, art, subtitle);
                            }
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
            "DER TAG LÄUFT",
            "Freier Bauplatz",
            "hammer",
            |page| {
                metric(page, hud, "sun", "Morgen kannst du hier bauen.");
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
            let before = town.production(chapter);
            if let Some(service) = b.kind.service() {
                let stock = match &town.phase {
                    Phase::Open(day) => day.stock[lot],
                    _ => before.stock[lot],
                };
                page.spawn(row(20.0)).with_children(|stats| {
                    metric(
                        stats,
                        hud,
                        service_icon(service),
                        stock_text(service, stock),
                    );
                    metric(
                        stats,
                        hud,
                        "coin",
                        format!("{} {currency} pro Verkauf", b.price.coins(service)),
                    );
                });
                if let Some(need) = town.supply_need(chapter, lot, &before) {
                    card(page, frame_node(Val::Percent(100.0), 6.0, 9.0), DEEP).with_children(
                        |supply| {
                            supply.spawn(row(13.0)).with_children(|stats| {
                                metric(
                                    stats,
                                    hud,
                                    "guest_book",
                                    format!("{} Gäste erwartet", need.guests),
                                );
                                let missing = need.shortage();
                                let (art, text) = if missing > 0 {
                                    ("guest_empty", format!("Nachschub nötig · {missing} fehlen"))
                                } else {
                                    ("check", "Vorrat reicht".to_owned())
                                };
                                metric(stats, hud, art, text);
                            });
                            if b.kind.water_user() {
                                let label = if need.short_water {
                                    if town.planning() {
                                        "Wasser fehlt"
                                    } else {
                                        "Am Morgen fehlte Wasser"
                                    }
                                } else {
                                    "Genug Wasser für die Produktion"
                                };
                                metric(supply, hud, "water", label);
                            }
                            if matches!(town.phase, Phase::Open(_)) && need.shortage() > 0 {
                                let recommended = need.suggested_job(town, chapter, lot);
                                supply.spawn(row(8.0)).with_children(|choices| {
                                    for job in [Job::Water, Job::Freight] {
                                        if job.plan(town, chapter, lot).is_ok() {
                                            button(
                                                choices,
                                                hud,
                                                Action::ChooseJob(job),
                                                job.name(),
                                                job.icon(),
                                                "",
                                                if recommended == Some(job) {
                                                    Style::Primary
                                                } else {
                                                    Style::Quiet
                                                },
                                                false,
                                            );
                                        }
                                    }
                                });
                            }
                        },
                    );
                }
                if let Some((favor, at)) = town.incident(chapter)
                    && at == lot
                    && town.favor_plan(chapter, lot).is_some()
                {
                    button(
                        page,
                        hud,
                        Action::Aid,
                        favor.name(),
                        favor.icon(),
                        "A",
                        Style::Primary,
                        false,
                    );
                }
                if town.planning() {
                    price_cards(page, hud, town, chapter, lot, service, b.price);
                    if Assortment::supported(b.kind) {
                        button(
                            page,
                            hud,
                            Action::ShowAssortment,
                            &format!("Angebot · {}", b.assortment.label(b.kind)),
                            service_icon(service),
                            "",
                            Style::Quiet,
                            false,
                        );
                    }
                }
            } else {
                let (art, effect) = match b.kind {
                    Kind::House | Kind::Villa => ("house", format!("{} Nachbarn", b.residents())),
                    Kind::Well | Kind::Aqueduct => {
                        ("water", format!("{} Krüge auf der Insel", before.water))
                    }
                    Kind::Grove => ("olive", format!("{} Oliven auf der Insel", before.olives)),
                    _ => (
                        "bread",
                        format!("+{} Portionen je Laden in der Nähe", b.level),
                    ),
                };
                metric(page, hud, art, effect);
            }
            if let Some((art, text)) = building_link(b.kind) {
                metric(page, hud, art, text);
            }
            if town.planning() {
                if let Some(estimate) = upgrade_estimate(town, chapter, lot) {
                    let appearance = expansion(b.kind, b.level + 1, Region::of(town.chapter));
                    page.spawn(row(18.0)).with_children(|next| {
                        metric(next, hud, "hammer", appearance);
                        let UpgradeEstimate {
                            art,
                            from,
                            to,
                            unit,
                            ..
                        } = estimate;
                        metric(next, hud, art, format!("{from} → {to} {unit}"));
                    });
                    if let Some(kind) = estimate.short_water {
                        let words = if kind == b.kind {
                            "Beim Ausbau fehlt Wasser".to_owned()
                        } else {
                            format!("Wasser wird knapp · {}", kind.name())
                        };
                        metric(page, hud, "water", words);
                    }
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
                        "Abreißen",
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

fn assortment(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    town: &Town,
    chapter: &Chapter,
) {
    let lot = game.selected;
    let Some(building) = town.lots[lot] else {
        return;
    };
    let Some(need) = town.supply_need(chapter, lot, &town.production(chapter)) else {
        return;
    };
    let art = kind_icon(building.kind);
    modal(
        p,
        hud,
        730.0,
        building.kind.name(),
        "Angebot wählen",
        art,
        |page| {
            metric(
                page,
                hud,
                "star",
                "+1 Ansehen pro Gast, dem die Spezialität gefällt",
            );
            for choices in Assortment::ALL.chunks(2) {
                page.spawn(Node {
                    align_items: AlignItems::Stretch,
                    ..row(8.0)
                })
                .with_children(|strip| {
                    for &choice in choices {
                        let selected = building.assortment == choice;
                        button_shell(
                            strip,
                            Action::Assortment(choice),
                            if selected {
                                Style::Selected
                            } else {
                                Style::Quiet
                            },
                            false,
                            Node {
                                flex_grow: 1.0,
                                flex_basis: Val::Px(0.0),
                                min_width: Val::Px(0.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..row(7.0)
                            },
                        )
                        .with_children(|card| {
                            icon(card, hud, if selected { "check" } else { art }, 35.0);
                            card.spawn(column(4.0)).with_children(|copy| {
                                label(
                                    copy,
                                    hud,
                                    choice.label(building.kind),
                                    19.0,
                                    Face::Bold,
                                    SAND,
                                );
                                let expects = choice.flavor().map_or_else(
                                    || "Für alle Gäste".into(),
                                    |age| {
                                        format!(
                                            "{} · {} erwartet",
                                            age.group(),
                                            need.ages[age as usize]
                                        )
                                    },
                                );
                                label(copy, hud, expects, 15.0, Face::Body, MUTED);
                            });
                        });
                    }
                });
            }
            button(
                page,
                hud,
                Action::Back,
                "Zurück zum Gebäude",
                "",
                "B",
                Style::Quiet,
                false,
            );
        },
    );
}

fn building_link(kind: Kind) -> Option<(&'static str, &'static str)> {
    Some(match kind {
        Kind::Bakery | Kind::Potter | Kind::Inn => ("water", "Mehr mit Wasser"),
        Kind::Baths => ("aqueduct", "Mehr am Aquädukt"),
        Kind::Grove => ("stoa", "Schatten am Wohnhof"),
        Kind::Taverna => ("bread", "Brot + Oliven"),
        Kind::Press => ("olive", "Braucht Oliven"),
        Kind::Shrine => ("potter", "Mehr mit Schalen"),
        Kind::Aqueduct => ("well", "Mehr am Brunnen"),
        Kind::Forum => ("ferry", "Kürzere Wege"),
        Kind::Villa => ("budget", "Größere Budgets"),
        _ => return None,
    })
}

pub(super) fn metric(p: &mut ChildSpawnerCommands, hud: &Hud, art: &str, text: impl Into<String>) {
    p.spawn(row(5.0)).with_children(|row| {
        icon(row, hud, art, 24.0);
        label(row, hud, text, 16.0, Face::Body, SAND);
    });
}

fn price_cards(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    lot: usize,
    service: Service,
    current: Price,
) {
    let visits = town.visit_plan(chapter);
    let budgets: Vec<_> = visits
        .iter()
        .filter(|visit| visit.wants == service)
        .map(|visit| visit.budget)
        .collect();
    let suggested = coach::recommended_price(town, lot);
    p.spawn(Node {
        align_items: AlignItems::Stretch,
        ..row(9.0)
    })
    .with_children(|prices| {
        for price in Price::ALL {
            let affordable = budgets
                .iter()
                .filter(|&&budget| price as u8 <= budget as u8)
                .count();
            button_shell(
                prices,
                Action::Price(price),
                if price == current || Some(price) == suggested {
                    Style::Selected
                } else {
                    Style::Quiet
                },
                false,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    min_width: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(9.0)),
                    ..column(5.0)
                },
            )
            .with_children(|choice| {
                choice
                    .spawn(Node {
                        justify_content: JustifyContent::SpaceBetween,
                        ..row(4.0)
                    })
                    .with_children(|head| {
                        label(head, hud, price.label(), 19.0, Face::Bold, SAND);
                        if current == price {
                            icon(head, hud, "check", 24.0);
                        } else if Some(price) == suggested {
                            icon(head, hud, "nili", 27.0);
                        }
                    });
                metric(choice, hud, "coin", price.coins(service).to_string());
                metric(
                    choice,
                    hud,
                    "budget",
                    if budgets.is_empty() {
                        "Heute kein Bedarf".to_owned()
                    } else {
                        format!("Für {affordable} von {} bezahlbar", budgets.len())
                    },
                );
                meter(
                    choice,
                    affordable as f32 / budgets.len().max(1) as f32,
                    GOLD,
                );
            });
        }
    });
}

fn meter(p: &mut ChildSpawnerCommands, fraction: f32, fill: Color) {
    p.spawn((
        Node {
            height: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(JADE),
        BorderRadius::all(Val::Px(3.0)),
    ))
    .with_children(|track| {
        track.spawn((
            Node {
                width: Val::Percent(100.0 * fraction.clamp(0.0, 1.0)),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(fill),
            BorderRadius::all(Val::Px(3.0)),
        ));
    });
}

fn expansion(kind: Kind, tier: u8, region: Region) -> &'static str {
    let names = match kind {
        Kind::House if region == Region::Greece => ["Balkon", "Dachpergola", "Schattenterrasse"],
        Kind::House => ["Loggia", "Seitenflügel", "Arkaden"],
        Kind::Well if region == Region::Greece => {
            ["Ziehbrunnen mit Dach", "Tränke", "Hochbehälter"]
        }
        Kind::Well => ["Brunnenarkade", "Doppelter Auslauf", "Wasserturm"],
        Kind::Bakery => ["Außenofen", "Brotregal", "Ofenturm"],
        Kind::Grove => ["Dritter Olivenbaum", "Überdachte Rastbank", "Weinlaube"],
        Kind::Potter => ["Brennofen", "Auslage", "Dachwerkstatt"],
        Kind::Stoa => ["Sitzbänke", "Säulenvorbau", "Verziertes Dach"],
        Kind::Taverna => ["Terrasse", "Überdachte Tische", "Obergeschoss"],
        Kind::Inn => ["Breiter Balkon", "Balkondach", "Dachkammer"],
        Kind::Storehouse => ["Ladeplatz", "Lastenkran", "Speicherboden"],
        Kind::Press => ["Pressdach", "Krugablage", "Ölbehälter"],
        Kind::Shrine => ["Altarstufen", "Stirngiebel", "Feuerschalen"],
        Kind::Aqueduct => ["Zweite Arkadenreihe", "Wasserbecken", "Kopftürme"],
        Kind::Baths => ["Zwei Seitenbecken", "Badegalerie", "Kuppelflügel"],
        Kind::Forum => ["Marktlaube", "Obere Arkaden", "Stadtgiebel"],
        Kind::Villa => ["Mosaikhof", "Säulenloggia", "Turmzimmer"],
    };
    names[usize::from(tier - 2)]
}

struct UpgradeEstimate {
    art: &'static str,
    from: u16,
    to: u16,
    unit: &'static str,
    short_water: Option<Kind>,
}

fn upgrade_estimate(town: &Town, chapter: &Chapter, lot: usize) -> Option<UpgradeEstimate> {
    let building = town.lots[lot]?;
    let (next, after) = town.projected_upgrade(chapter, lot)?;
    let before = town.production(chapter);
    let (art, from, to, unit) = if building.residents() > 0 {
        ("house", building.residents(), next.residents(), "Nachbarn")
    } else if building.kind.water_source() {
        ("water", before.water, after.water, "Krüge")
    } else if building.kind == Kind::Grove {
        ("olive", before.olives, after.olives, "Oliven")
    } else if building.kind == Kind::Storehouse {
        (
            "bread",
            u16::from(building.level),
            u16::from(next.level),
            "mehr je Laden",
        )
    } else {
        let service = building.kind.service()?;
        (
            service_icon(service),
            before.stock[lot],
            after.stock[lot],
            stock_unit(service, 2),
        )
    };
    let short_water = if building.kind.water_user() && to <= from {
        Some(building.kind)
    } else if building.kind.water_user() {
        town.lots.iter().enumerate().find_map(|(i, &b)| {
            b.filter(|b| b.kind.water_user() && after.stock[i] < before.stock[i])
                .map(|b| b.kind)
        })
    } else {
        None
    };
    Some(UpgradeEstimate {
        art,
        from,
        to,
        unit,
        short_water,
    })
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
            for task in chapter.goals {
                super::screens::goal(page, hud, town, task);
            }
            hint(page, hud, chapter.hint, "nili");
            if chapter
                .goals
                .iter()
                .any(|g| matches!(g, crate::campaign::Goal::GoodDay(_)))
            {
                label(
                    page,
                    hud,
                    "Die Zufriedenheit zählt ab 12 Rückmeldungen.",
                    16.0,
                    Face::Body,
                    MUTED,
                );
            }
            page.spawn(row(16.0)).with_children(|stars| {
                metric(
                    stars,
                    hud,
                    "star",
                    format!("Abschluss bis Tag {}", chapter.par),
                );
                metric(
                    stars,
                    hud,
                    "nili",
                    format!("{} Bitten vom Kai", chapter.favor_goal),
                );
            });
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
    let books = [
        ("Vorräte", "storehouse"),
        ("Gästebuch", "guest_book"),
        ("Wetter", "cloud"),
    ];
    modal(
        p,
        hud,
        900.0,
        if town.planning() && game.ledger_page == 1 && town.feedback().is_some() {
            "STIMMEN VON GESTERN"
        } else {
            chapter.district
        },
        "Hafenplan",
        if game.ledger_page == 1 {
            "guest_book"
        } else {
            "ferry"
        },
        |page| {
            page.spawn(row(8.0)).with_children(|tabs| {
                for (i, (name, pic)) in books.into_iter().enumerate() {
                    button(
                        tabs,
                        hud,
                        Action::LedgerPage(i),
                        name,
                        pic,
                        "",
                        if game.ledger_page == i {
                            Style::Primary
                        } else {
                            Style::Quiet
                        },
                        i != 1 && matches!(town.phase, Phase::Report(_)),
                    );
                }
            });
            match game.ledger_page {
                0 => supplies(page, hud, town, chapter),
                1 => guests::book(page, hud, game, town, chapter),
                _ => day_plan(page, hud, town, chapter),
            }
            button(
                page,
                hud,
                Action::Back,
                if matches!(town.phase, Phase::Report(_)) {
                    "Zur Abrechnung"
                } else {
                    "Zur Gasse"
                },
                "",
                "B",
                Style::Quiet,
                false,
            );
        },
    );
}

fn supplies(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let live = matches!(town.phase, Phase::Open(_));
    let (home, ferry, predicted) = if let Phase::Open(day) = &town.phase {
        let mut wishes = [0; SERVICES];
        let (mut home, mut ferry) = (0, 0);
        for visit in day.visits.iter().filter(|visit| !visit.resolved) {
            wishes[visit.wants as usize] += 1;
            if visit.origin == crate::model::Origin::Ferry {
                ferry += 1;
            } else {
                home += 1;
            }
        }
        (home, ferry, wishes)
    } else {
        let (home, ferry) = town.visitors(chapter);
        (home, ferry, town.forecast(chapter))
    };
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
        super::screens::stat(
            figures,
            hud,
            "house",
            &home.to_string(),
            if live {
                "Erwartete Nachbarn"
            } else {
                "Nachbarn"
            },
        );
        super::screens::stat(
            figures,
            hud,
            "ferry",
            &ferry.to_string(),
            if live {
                "Erwartete Fährgäste"
            } else {
                "Fährgäste"
            },
        );
        super::screens::stat(
            figures,
            hud,
            "water",
            &format!("{}/{}", production.used_water, production.water),
            "Wasser verbraucht",
        );
        if chapter.allowed().contains(&Kind::Grove) {
            super::screens::stat(
                figures,
                hud,
                "olive",
                &format!("{}/{}", production.used_olives, production.olives),
                "Oliven verbraucht",
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
                    let short = supply[i] < predicted[i];
                    tile.spawn(Node {
                        justify_content: JustifyContent::SpaceBetween,
                        ..row(4.0)
                    })
                    .with_children(|top| {
                        icon(top, hud, service_icon(service), 33.0);
                        if short {
                            icon(top, hud, "guest_empty", 23.0);
                        } else if predicted[i] > 0 {
                            icon(top, hud, "check", 22.0);
                        }
                    });
                    label(tile, hud, service.name(), 17.0, Face::Bold, SAND);
                    label(
                        tile,
                        hud,
                        format!(
                            "{} {}",
                            supply[i],
                            if live { "auf Lager" } else { "bereit" }
                        ),
                        16.0,
                        Face::Bold,
                        if short { GOLD } else { SAND },
                    );
                    label(
                        tile,
                        hud,
                        format!(
                            "{} {}",
                            predicted[i],
                            if live { "noch erwartet" } else { "erwartet" }
                        ),
                        15.0,
                        Face::Body,
                        MUTED,
                    );
                    if predicted[i] > 0 {
                        meter(
                            tile,
                            f32::from(supply[i]) / f32::from(predicted[i]),
                            if short { GOLD } else { MUTED },
                        );
                    }
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

fn day_plan(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    page.spawn(Node {
        align_items: AlignItems::Stretch,
        ..row(9.0)
    })
    .with_children(|days| {
        for (index, name) in ["Heute", "Morgen", "Übermorgen"].into_iter().enumerate() {
            let weather = town.weather_ahead(chapter, index);
            card(
                days,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(9.0)),
                    ..column(4.0)
                },
                if index == 0 { JADE } else { CARD },
            )
            .with_children(|day| {
                day.spawn(row(7.0)).with_children(|top| {
                    icon(top, hud, weather_icon(weather), 44.0);
                    top.spawn(column(0.0)).with_children(|caption| {
                        label(caption, hud, name, 15.0, Face::Body, MUTED);
                        label(caption, hud, weather.name(), 20.0, Face::Bold, SAND);
                    });
                });
                if weather.ferry_change() != 0 {
                    metric(
                        day,
                        hud,
                        "ferry",
                        format!("{:+} Fährgäste", weather.ferry_change()),
                    );
                }
                if weather.rainfall() > 0 {
                    metric(day, hud, "water", format!("+{} Krüge", weather.rainfall()));
                }
                if weather == Weather::Heat {
                    metric(day, hud, "water", "+1 Krug Bedarf je Laden");
                }
                if weather == Weather::Feast {
                    metric(day, hud, "budget", "Größere Budgets");
                }
                if matches!(weather, Weather::Clear | Weather::Cloud) {
                    metric(day, hud, "ferry", "Fähre nach Plan");
                }
            });
        }
    });
    let favor = town.next_favor(chapter);
    let done = matches!(&town.phase, Phase::Open(day) if day.favor_at.is_some());
    if done {
        metric(page, hud, "check", "Bitte vom Kai erfüllt");
    } else {
        page.spawn(row(16.0)).with_children(|r| {
            metric(r, hud, "nili", favor.name());
            metric(r, hud, "bread", favor.note());
        });
    }
    if chapter.harbor_plans() {
        label(page, hud, "Heutiger Hafenplan", 18.0, Face::Bold, GOLD);
        page.spawn(row(8.0)).with_children(|choices| {
            for choice in HarborPlan::ALL {
                let art = match choice {
                    HarborPlan::Regular => "sun",
                    HarborPlan::Ferry => "ferry",
                    HarborPlan::Water => "water",
                    HarborPlan::Neighbors => "house",
                };
                button(
                    choices,
                    hud,
                    Action::HarborPlan(choice),
                    choice.name(),
                    art,
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
        page.spawn(row(14.0))
            .with_children(|effects| match town.harbor {
                HarborPlan::Regular => {
                    metric(effects, hud, "check", "Fähre und Wasser wie geplant")
                }
                HarborPlan::Ferry => {
                    metric(effects, hud, "ferry", "+3 Fährgäste");
                    metric(effects, hud, "water", "−2 Krüge");
                }
                HarborPlan::Water => {
                    metric(effects, hud, "water", "+4 Krüge");
                    metric(effects, hud, "ferry", "−2 Fährgäste");
                }
                HarborPlan::Neighbors => {
                    metric(effects, hud, "house", "+1 Besuch je Wohnhof oder Villa");
                    metric(effects, hud, "ferry", "−2 Fährgäste");
                }
            });
    }
}

fn evening(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    report: &Report,
    tutorial: bool,
) {
    let clear = report.stars > 0;
    let ending = clear && town.chapter + 1 == CHAPTERS.len();
    let rome_unlocked = clear && town.chapter + 1 == REGION_SIZE;
    let tutorial_complete = clear && tutorial && town.chapter + 1 == coach::ISLANDS;
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
                    guests::total_symbol(&report.tally),
                    &report
                        .tally
                        .satisfaction()
                        .map_or_else(|| "—".into(), |n| format!("{n} %")),
                    "Zufriedenheit",
                );
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
                if report.tally.routes_today() > 0 {
                    super::screens::stat(
                        figures,
                        hud,
                        "nili",
                        &report.tally.routes_today().to_string(),
                        "Touren",
                    );
                } else if report.tally.favors > 0 {
                    super::screens::stat(figures, hud, "nili", "1", "Bitte vom Kai");
                }
            });
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
                if report.stars < 3 {
                    page.spawn(row(13.0)).with_children(|remaining| {
                        if town.totals.favors < chapter.favor_goal {
                            metric(
                                remaining,
                                hud,
                                "nili",
                                format!(
                                    "{} / {} Bitten vom Kai",
                                    town.totals.favors, chapter.favor_goal
                                ),
                            );
                        }
                        if town.day > chapter.par {
                            metric(
                                remaining,
                                hud,
                                "sunset",
                                format!("Zeitbonus bis Tag {}", chapter.par),
                            );
                        }
                    });
                }
                label(
                    page,
                    hud,
                    if ending {
                        "Nili, Flavia und Gaius feiern mit dir auf dem Forum."
                    } else {
                        "Der nächste Hafen ist frei."
                    },
                    19.0,
                    Face::Body,
                    SAND,
                );
            } else {
                for goal in chapter.goals {
                    super::screens::goal(page, hud, town, goal);
                }
            }
            let (advice, image) = guests::evening_hint(town, &report.tally);
            hint(page, hud, &advice, image);
            page.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(9.0)
            })
            .with_children(|actions| {
                button(
                    actions,
                    hud,
                    Action::Chart,
                    if tutorial {
                        "Übung beenden"
                    } else {
                        "Kampagnenkarte"
                    },
                    "chart",
                    "B",
                    Style::Quiet,
                    false,
                );
                button(
                    actions,
                    hud,
                    Action::Feedback,
                    "Gästebuch",
                    "guest_book",
                    "View",
                    Style::Quiet,
                    false,
                );
                if clear {
                    if !tutorial {
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
                    }
                    button(
                        actions,
                        hud,
                        Action::NextChapter,
                        if tutorial_complete {
                            "Tutorial abschließen"
                        } else if tutorial {
                            "Nächste Übung"
                        } else if ending {
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

fn pause(p: &mut ChildSpawnerCommands, hud: &Hud, tutorial: bool) {
    let heading = if tutorial { "TUTORIAL" } else { "" };
    modal(p, hud, 535.0, heading, "Ruhe am Hafen", "nili", |page| {
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
            Action::Wardrobe,
            "Nilis Garderobe",
            "nili",
            "",
            Style::Quiet,
            false,
        );
        button(
            page,
            hud,
            Action::Chart,
            if tutorial {
                "Übung beenden"
            } else {
                "Kampagnenkarte"
            },
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
            "Dein Fortschritt wird automatisch gespeichert.",
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
                    "Wenn du nach {} reist, endet der laufende Hafen. Deine Sterne bleiben gespeichert.",
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
    const PAGES: [&str; 6] = ["Steuerung", "Dein Tag", "Waren", "Nili", "Reisen", "Rom"];
    modal(p, hud, 985.0, "", "Spielhilfe", "guide", |page| {
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
        if game.guide == 0 {
            controls_guide(page, hud);
        } else {
            let cards: &[(&str, &str, &str, &str)] = match game.guide {
                1 => &[
                    (
                        "hammer",
                        "coin",
                        "Morgens bauen und Preise wählen",
                        "Wähle einen Bauplatz oder ein fertiges Gebäude.",
                    ),
                    (
                        "ferry",
                        "guest_happy",
                        "Tagsüber Gäste beobachten",
                        "Tippe auf eine Sprechblase oder öffne das Gästebuch.",
                    ),
                    (
                        "sunset",
                        "coin",
                        "Abends kommt Geld in die Kasse",
                        "Du bekommst die Einnahmen und 6 Münzen vom Hafen.",
                    ),
                ],
                2 => &[
                    (
                        "well",
                        "bakery",
                        "Mit Wasser gibt es mehr Vorrat",
                        "Läden in der Nähe bekommen zuerst Wasser. Bei Hitze brauchen sie mehr.",
                    ),
                    (
                        "olive",
                        "meze",
                        "2 Laibe und 2 Portionen Oliven für die Taverne",
                        "Mit einem Ofen nebenan gibt es eine Portion Meze mehr.",
                    ),
                    (
                        "storehouse",
                        "bread",
                        "Speicher helfen Läden in der Nähe",
                        "Der Speicher versorgt Läden bis zu zwei Bauplätze entfernt.",
                    ),
                    (
                        "guest_book",
                        "water",
                        "Gebäude antippen",
                        "Die Details zeigen erwartete Gäste, den Vorrat und wo Wasser fehlt.",
                    ),
                ],
                3 => &[
                    (
                        "nili",
                        "pad_dpad",
                        "Nili direkt steuern",
                        "Bewege den linken Stick oder ziehe auf dem runden Steuerfeld.",
                    ),
                    (
                        "water",
                        "check",
                        "Abholen und abgeben",
                        "Bestätige mit A oder per Tipp, sobald Nili bei der Markierung steht.",
                    ),
                    (
                        "fire",
                        "storehouse",
                        "Bei Zwischenfällen helfen",
                        "Lösche einen Ofenbrand oder sichere Kisten, die beim Unwetter verrutscht sind.",
                    ),
                    (
                        "wind",
                        "ferry",
                        "Bei Wind, Regen und Sturm den Kai räumen",
                        "Fährgäste laufen dann zwei Plätze weiter.",
                    ),
                ],
                4 => &[
                    (
                        "book",
                        "chart",
                        "Alle Aufträge erfüllen",
                        "Am Abend schaltest du den nächsten Hafen frei.",
                    ),
                    (
                        "sunset",
                        "star",
                        "Bis zu drei Sterne je Hafen",
                        "Sterne gibt es für die Aufträge, den Abschluss in der Zeit und die Bitten vom Kai.",
                    ),
                    (
                        "nili",
                        "star",
                        "Garderobe auf der Kampagnenkarte",
                        "Tausche freie Sterne gegen Gewänder und Kopfschmuck ein.",
                    ),
                ],
                _ => &[
                    (
                        "well",
                        "aqueduct",
                        "Brunnen speisen das Aquädukt",
                        "Eine Therme in der Nähe bekommt mehr Badeplätze.",
                    ),
                    (
                        "forum",
                        "ferry",
                        "Das Forum hilft Fährgästen",
                        "In der Nähe des Forums laufen Fährgäste einen Platz weiter.",
                    ),
                    (
                        "villa",
                        "budget",
                        "In Villen wohnen mehr Menschen",
                        "Viele von ihnen können mehr ausgeben.",
                    ),
                ],
            };
            for &(from, to, title, words) in cards {
                guide_card(page, hud, from, to, title, words);
            }
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

fn guide_card(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    from: &str,
    to: &str,
    title: &str,
    words: &str,
) {
    card(
        p,
        Node {
            padding: UiRect::axes(Val::Px(11.0), Val::Px(5.0)),
            ..row(12.0)
        },
        CARD,
    )
    .with_children(|tile| {
        tile.spawn(row(3.0)).with_children(|pair| {
            icon(pair, hud, from, 40.0);
            label(pair, hud, "›", 22.0, Face::Bold, MUTED);
            icon(pair, hud, to, 40.0);
        });
        tile.spawn(column(1.0)).with_children(|text| {
            label(text, hud, title, 19.0, Face::Bold, SAND);
            label(text, hud, words, 16.0, Face::Body, MUTED);
        });
    });
}

fn controls_guide(p: &mut ChildSpawnerCommands, hud: &Hud) {
    let keys = [
        ("", "pad_dpad", "Bauplatz oder Feld wählen"),
        ("A", "hammer", "Öffnen und bestätigen"),
        ("B", "", "Zurück"),
        ("Y", "sun", "Tag starten oder Tempo wechseln"),
        ("X", "book", "Aufträge"),
        ("View", "guest_book", "Gästebuch"),
        ("RB", "cloud", "Wetter und Vorräte"),
        (
            "R3",
            "nili",
            "Rechten Stick drücken, um Nilis Touren zu öffnen",
        ),
        ("R-Stick", "chart", "Kamera bewegen"),
        ("LT / RT", "", "Rein- und rauszoomen"),
        ("LB", "", "Kamera zurücksetzen"),
        ("Start", "pause", "Pause"),
    ];
    for pair in keys.as_chunks::<2>().0 {
        p.spawn(row(8.0)).with_children(|r| {
            for &(pad, art, caption) in pair {
                card(
                    r,
                    Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        min_height: Val::Px(42.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(3.0)),
                        ..row(7.0)
                    },
                    CARD,
                )
                .with_children(|tile| {
                    if !pad.is_empty() {
                        key(tile, hud, pad, false);
                    }
                    if !art.is_empty() {
                        icon(tile, hud, art, 26.0);
                    }
                    label(tile, hud, caption, 16.0, Face::Body, SAND);
                });
            }
        });
    }
}

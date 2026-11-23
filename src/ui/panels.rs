use super::*;
use crate::{
    campaign::{
        CORE_HARBORS, Chapter, REGION_SIZE, Region, harbor_stage, main_route_end, next_harbor,
    },
    caravan::Contract,
    coach::{self, Step},
    errands::Job,
    model::{Assortment, Building, HarborPlan, MAX_LEVEL, Price, Report, SERVICES, Town},
    trade::SailingRoute,
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
        Panel::Goals => goals(p, hud, town, chapter),
        Panel::Forecast => forecast(p, hud, game, town, chapter),
        Panel::Nili if hud.phone => phone::tours(p, hud, game, town, chapter),
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

pub(super) fn modal(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    width: f32,
    heading: &str,
    title: &str,
    image: &str,
    content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    overlay(p).with_children(|overlay| {
        let mut node = frame_node(
            Val::Percent(93.0),
            if hud.phone { 8.0 } else { 14.0 },
            if hud.phone { 16.0 } else { 24.0 },
        );
        node.max_width = Val::Px(width);
        if hud.phone {
            node.max_height = Val::Percent(95.0);
        }
        card(overlay, node, INK).with_children(|page| {
            page.spawn(Node {
                flex_shrink: if hud.phone { 0.0 } else { 1.0 },
                ..row(12.0)
            })
            .with_children(|head| {
                icon(head, hud, image, if hud.phone { 44.0 } else { 54.0 });
                head.spawn(if hud.phone {
                    Node {
                        flex_grow: 1.0,
                        min_width: Val::Px(0.0),
                        ..column(1.0)
                    }
                } else {
                    column(1.0)
                })
                .with_children(|words| {
                    if !heading.is_empty() {
                        label(words, hud, heading, 15.0, Face::Bold, GOLD);
                    }
                    label(
                        words,
                        hud,
                        title,
                        if hud.phone { 33.0 } else { 40.0 },
                        Face::Serif,
                        SAND,
                    );
                });
                if hud.phone
                    && let Some(action) = hud.close_modal
                {
                    button(
                        head,
                        hud,
                        action,
                        "",
                        "phone_close",
                        "",
                        Style::Quiet,
                        false,
                    );
                }
            });
            line(page, LINE);
            if hud.phone {
                page.spawn((
                    touch::ScrollPage,
                    ScrollPosition {
                        offset_y: hud.scroll_y,
                        ..default()
                    },
                    Node {
                        min_height: Val::Px(0.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                ))
                .with_children(|viewport| {
                    viewport
                        .spawn(Node {
                            flex_shrink: 0.0,
                            width: Val::Percent(100.0),
                            ..column(9.0)
                        })
                        .with_children(content);
                });
            } else {
                content(page);
            }
        });
    });
}

fn brief(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter, tutorial: bool) {
    let region = Region::of(town.chapter);
    let heading = if tutorial {
        format!("TUTORIAL  {} / {}", town.chapter + 1, coach::ISLANDS)
    } else {
        format!(
            "{}  ·  {}  ·  {} / {}",
            region.name().to_uppercase(),
            if harbor_stage(town.chapter) >= CORE_HARBORS {
                "BONUSROUTE"
            } else {
                "HAUPTREISE"
            },
            harbor_stage(town.chapter) % CORE_HARBORS + 1,
            CORE_HARBORS
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
                        if chapter.shift.night() {
                            metric(
                                words,
                                hud,
                                "moon",
                                format!("Nachtschicht · {} Uhr", chapter.shift.hours()),
                            );
                        }
                        metric(
                            words,
                            hud,
                            "hammer",
                            format!("{} Bauplätze", chapter.layout.count()),
                        );
                    });
                });
            }
            new_in_harbor(page, hud, town.chapter, chapter);
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

fn new_in_harbor(page: &mut ChildSpawnerCommands, hud: &Hud, island: usize, chapter: &Chapter) {
    use crate::model::Age;
    let stage = harbor_stage(island);
    let mut fresh = match stage {
        0 => vec![(
            "age_adult",
            "Erwachsene Gäste",
            "Preis und Vorrat im Blick behalten.",
        )],
        crate::street::LITTER_STAGE => {
            vec![("leaves", "Aufräumen", "Mit Nili Laub und Unrat einsammeln.")]
        }
        x if x == Age::Child.stage() => vec![
            (
                "age_child",
                "Kinder kommen an",
                "Bunte und süße Spezialitäten werden interessant.",
            ),
            (
                "book",
                "Tagesaufträge",
                "Zusätzliche Münzen für kleine Aufgaben.",
            ),
        ],
        crate::street::PET_STAGE => vec![
            (
                "cat",
                "Katzen und Hunde",
                "Streichle sie, solange sie in der Gasse sind.",
            ),
            (
                "fire",
                "Zwischenfälle",
                "Achte auf die Glocke und die Markierung.",
            ),
        ],
        x if x == Age::Senior.stage() => vec![(
            "age_senior",
            "Ältere Gäste",
            "Auch klassische Angebote finden jetzt Abnehmer.",
        )],
        5 => vec![(
            "ferry",
            "Neue Zwischenfälle",
            "Begleite Reisende oder befestige lose Segel.",
        )],
        _ => Vec::new(),
    };
    if chapter.region == Region::Japan {
        let feature = match stage {
            0 => Some((
                "rice_paddy",
                "Vom Feld in die Küche",
                "Brunnen am Feld und Felder neben der Reisküche bringen mehr Reis auf den Tisch.",
            )),
            1 => Some((
                "temple_bell",
                "Pilger rufen",
                "Geh tagsüber zum Tempel und läute die Glocke. Zwei Pilger kommen zum Gebet.",
            )),
            2 => Some((
                "tea_garden",
                "Tee aus der Nähe",
                "Ein Teegarten versorgt benachbarte Teehäuser mit mehr Tee.",
            )),
            4 => Some((
                "lampion",
                "Licht für Pilger",
                "Neben einer Lampionwerkstatt ruft die Glocke drei Gäste.",
            )),
            _ => None,
        };
        fresh.extend(feature);
    }
    if chapter.region == Region::Oasis {
        let feature = match stage {
            0 => Some((
                "cistern",
                "Datteln und Wasser",
                "Baue einen Dattelhain neben der Zisterne.",
            )),
            4 => Some((
                "caravan",
                "Die erste Karawane",
                "Geh mit Nili zur Karawanserei. Halte dort Datteln und drei freie Krüge bereit.",
            )),
            5 => Some((
                "qanat",
                "Wasser unter dem Sand",
                "Ein Qanat bringt den Nachbarhöfen und Karawanen zusätzliches Wasser.",
            )),
            12 => Some((
                "caravan",
                "Die Gewürzstraße",
                "Ein Qanat ab Stufe 2, fünf freie Krüge und zwei Waren öffnen die Fernroute.",
            )),
            14 => Some((
                "moon_garden",
                "Rast nach Sonnenuntergang",
                "Mondgärten bieten nachts vier zusätzliche Plätze.",
            )),
            _ => None,
        };
        fresh.extend(feature);
    }
    if chapter.region == Region::Nordic {
        let feature = match stage {
            0 => Some((
                "fishery",
                "Vom Netz in die Räucherei",
                "Setz die Räucherei neben die Fischerhütte. So gibt es mehr Portionen.",
            )),
            1 => Some((
                "woodlot",
                "Holz zum Räuchern",
                "Ein Holzplatz versorgt die Räucherei. Direkt nebeneinander reicht es für mehr Fisch.",
            )),
            2 => Some((
                "sea_trade",
                "Das erste Handelsschiff",
                "Geh mit Nili zum Bootshaus und lade Räucherfisch für die Fahrt.",
            )),
            4 => Some((
                "feast_hall",
                "Ein Platz am Herd",
                "Eine Räucherei neben der Festhalle bringt Fisch für fünf zusätzliche Plätze.",
            )),
            5 => Some((
                "sea_trade",
                "Fahrt übers offene Meer",
                "Fernhandel braucht ein Bootshaus auf Stufe 2 und sechs Portionen Räucherfisch.",
            )),
            _ => None,
        };
        fresh.extend(feature);
    }
    if stage > 0 {
        for job in Job::ALL
            .into_iter()
            .filter(|job| job.unlocked(island) && !job.unlocked(island - 1))
        {
            fresh.push((job.icon(), job.name(), job.purpose()));
        }
        for &building in chapter
            .allowed()
            .iter()
            .filter(|kind| !CHAPTERS[island - 1].allowed().contains(kind))
        {
            if (stage <= 2 && island < REGION_SIZE)
                || (chapter.region == Region::Japan && stage <= 4)
                || (chapter.region == Region::Nordic && stage <= 2)
            {
                continue;
            }
            let description =
                building_link(building).map_or("Hier kannst du es bauen.", |(_, copy)| copy);
            fresh.push((kind_icon(building), building.name(), description));
        }
    }
    if fresh.is_empty() {
        return;
    }
    label(page, hud, "Neu in diesem Hafen", 17.0, Face::Bold, GOLD);
    for entries in fresh.chunks(3) {
        page.spawn(Node {
            align_items: AlignItems::Stretch,
            ..row(7.0)
        })
        .with_children(|tiles| {
            for &(art, title, explanation) in entries {
                card(
                    tiles,
                    Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        min_width: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        ..row(6.0)
                    },
                    CARD,
                )
                .with_children(|feature| {
                    icon(feature, hud, art, 31.0);
                    feature.spawn(column(2.0)).with_children(|copy| {
                        label(copy, hud, title, 17.0, Face::Bold, SAND);
                        label(copy, hud, explanation, 14.5, Face::Body, MUTED);
                    });
                });
            }
        });
    }
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
                                    Kind::RicePaddy => (
                                        "rice_paddy",
                                        format!(
                                            "+{} Körbe Reis",
                                            after.rice.saturating_sub(before.rice)
                                        ),
                                    ),
                                    Kind::TeaGarden => {
                                        ("tea_garden", "+2 Schalen je Teehaus in der Nähe".into())
                                    }
                                    Kind::HerbGarden => (
                                        "herb_garden",
                                        format!(
                                            "{} Küchen nebenan",
                                            town.kitchens_in_reach(game.selected, 1)
                                        ),
                                    ),
                                    Kind::Fishery => (
                                        "fishery",
                                        format!(
                                            "+{} Fische",
                                            after.fish.saturating_sub(before.fish)
                                        ),
                                    ),
                                    Kind::Woodlot => (
                                        "woodlot",
                                        format!(
                                            "+{} Scheite",
                                            after.wood.saturating_sub(before.wood)
                                        ),
                                    ),
                                    Kind::Boatyard => {
                                        ("sea_trade", "Schickt Handelsschiffe los".into())
                                    }
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
        956.0,
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
                    if service.priced() {
                        metric(
                            stats,
                            hud,
                            "coin",
                            format!("{} {currency} pro Verkauf", b.price.coins(service)),
                        );
                    } else {
                        metric(stats, hud, "temple", "Gebet · kostenlos");
                    }
                });
                if let Some(need) = town.supply_need(chapter, lot, &before) {
                    let expected = if need.guests == 1 {
                        "1 Gast erwartet".to_owned()
                    } else {
                        format!("{} Gäste erwartet", need.guests)
                    };
                    if town.planning() {
                        page.spawn(row(15.0)).with_children(|needs| {
                            metric(needs, hud, "guest_book", expected);
                            let missing = need.shortage();
                            if missing > 0 {
                                metric(
                                    needs,
                                    hud,
                                    "guest_empty",
                                    format!("Es fehlen {missing} {}", stock_unit(service, missing)),
                                );
                            } else {
                                metric(
                                    needs,
                                    hud,
                                    "check",
                                    if has_places(service) {
                                        "Genug Plätze"
                                    } else {
                                        "Vorrat reicht"
                                    },
                                );
                            }
                            if need.short_water {
                                metric(needs, hud, "water", "Wasser fehlt");
                            }
                        });
                    } else {
                        card(page, frame_node(Val::Percent(100.0), 6.0, 9.0), DEEP).with_children(
                            |supply| {
                                supply.spawn(row(13.0)).with_children(|stats| {
                                    metric(stats, hud, "guest_book", expected);
                                    let missing = need.shortage();
                                    let (art, text) = if missing > 0 && service == Service::Prayer {
                                        ("guest_empty", format!("Noch {missing} Plätze nötig"))
                                    } else if missing > 0 {
                                        (
                                            "guest_empty",
                                            format!("Nachschub nötig · {missing} fehlen"),
                                        )
                                    } else {
                                        (
                                            "check",
                                            if has_places(service) {
                                                "Genug Plätze"
                                            } else {
                                                "Vorrat reicht"
                                            }
                                            .to_owned(),
                                        )
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
                if service == Service::Prayer {
                    if town.bell_ready(chapter, lot) {
                        button(
                            page,
                            hud,
                            Action::RingBell(lot),
                            "Glocke läuten",
                            "temple_bell",
                            "A",
                            Style::Primary,
                            false,
                        );
                    }
                    let guests = town.bell_guests(lot);
                    let status = if let Phase::Open(day) = &town.phase
                        && day.rung_bells & (1 << lot) != 0
                    {
                        "Heute schon geläutet".to_owned()
                    } else {
                        format!("Einmal pro Tag läuten · +{guests} Pilger")
                    };
                    metric(page, hud, "temple_bell", status);
                } else if town.planning() {
                    price_cards(page, hud, town, chapter, lot, service, b.price);
                    if let Some(forecast) = town.offer_forecast(chapter, lot, &before) {
                        offer_cards(page, hud, b, &forecast, town, chapter);
                    }
                }
            } else {
                let (art, effect) = match b.kind {
                    Kind::House
                    | Kind::Villa
                    | Kind::Machiya
                    | Kind::Longhouse
                    | Kind::OasisHome => (kind_icon(b.kind), format!("{} Nachbarn", b.residents())),
                    Kind::Well | Kind::Aqueduct | Kind::Cistern | Kind::Qanat => {
                        ("water", format!("{} Krüge auf der Insel", before.water))
                    }
                    Kind::HerbGarden => (
                        "herb_garden",
                        format!(
                            "+{} Portionen je Küche · {} Küchen in Reichweite",
                            b.level + 1,
                            town.kitchens_in_reach(lot, b.level)
                        ),
                    ),
                    Kind::Grove => ("olive", format!("{} Oliven auf der Insel", before.olives)),
                    Kind::RicePaddy => (
                        "rice_paddy",
                        format!(
                            "{} Körbe Reis von diesem Feld",
                            town.paddy_harvest(lot, b.level, town.weather(chapter))
                        ),
                    ),
                    Kind::TeaGarden => (
                        "tea_garden",
                        format!("+{} Schalen je Teehaus in der Nähe", 2 * b.level),
                    ),
                    Kind::Fishery => (
                        "fishery",
                        format!(
                            "{} Fische am Tag",
                            crate::model::fish_catch(b.level, town.weather(chapter))
                        ),
                    ),
                    Kind::Woodlot => ("woodlot", format!("{} Scheite am Tag", 3 + 2 * b.level)),
                    Kind::Boatyard => (
                        "sea_trade",
                        if town.voyage.is_some() {
                            "Ein Schiff ist auf See"
                        } else {
                            "Schiff im Hafen"
                        }
                        .into(),
                    ),
                    _ => (
                        "bread",
                        format!("+{} Portionen je Laden in der Nähe", b.level),
                    ),
                };
                metric(page, hud, art, effect);
                if b.residents() > 0 {
                    age_row(
                        page,
                        hud,
                        "Hier wohnen",
                        town.household(lot),
                        true,
                        town.chapter,
                    );
                }
            }
            if b.kind == Kind::Boatyard {
                trade_routes(page, hud, town, chapter, lot, b.level);
            }
            if b.kind == Kind::Caravanserai && Contract::Oasis.available(town.chapter) {
                caravan_routes(page, hud, town, chapter, lot, b.level);
            }
            if matches!(b.kind, Kind::Bakery | Kind::Taverna) && town.herb_support(lot) > 0 {
                metric(
                    page,
                    hud,
                    "herb_garden",
                    format!(
                        "Kräuter aus der Nähe · +{} Portionen",
                        town.herb_support(lot)
                    ),
                );
            }
            if !(town.planning() && Assortment::supported(b.kind) && b.kind.water_user())
                && let Some((art, text)) = building_link(b.kind)
            {
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
                    if matches!(b.kind, Kind::Boatyard | Kind::Temple) {
                        Style::Quiet
                    } else {
                        Style::Primary
                    },
                    false,
                );
            }
        },
    );
}

fn offer_cards(
    page: &mut ChildSpawnerCommands,
    hud: &Hud,
    building: Building,
    forecast: &crate::model::OfferForecast,
    town: &Town,
    chapter: &Chapter,
) {
    use crate::campaign::Goal;
    let island = town.chapter;
    let requested = crate::campaign_orders::trade_order(chapter, island)
        .filter(|order| {
            order.shop == building.kind
                && town.goal(Goal::Sale(*order, true)).0 < order.special_sales
        })
        .map(|order| {
            if town.goal(Goal::Sale(order, false)).0 < order.basic {
                Assortment::Standard
            } else {
                order.special
            }
        });
    page.spawn(Node {
        flex_wrap: FlexWrap::Wrap,
        row_gap: Val::Px(5.0),
        ..row(11.0)
    })
    .with_children(|header| {
        label(header, hud, "Angebot", 18.0, Face::Bold, GOLD);
        metric(header, hud, "coin", "Möglicher Mehrerlös");
        if let Some(choice) = requested.filter(|choice| !choice.unlocked(building.level)) {
            metric(
                header,
                hud,
                "star",
                format!(
                    "Auftrag ab Stufe {} · {}",
                    choice.tier(),
                    choice.label(building.kind)
                ),
            );
        } else if let Some(next) = Assortment::ALL.into_iter().find(|choice| {
            !choice.unlocked(building.level)
                && choice.flavor().is_some_and(|age| age.available(island))
        }) {
            label(
                header,
                hud,
                format!("Ab Stufe {} · {}", next.tier(), next.label(building.kind)),
                15.0,
                Face::Body,
                MUTED,
            );
        }
    });
    let art = kind_icon(building.kind);
    let options: Vec<_> = Assortment::ALL
        .into_iter()
        .filter(|choice| {
            choice.unlocked(building.level)
                && (*choice == building.assortment
                    || choice.flavor().is_none_or(|age| age.available(island)))
        })
        .collect();
    let columns = options.len().clamp(2, 5);
    for choices in options.chunks(columns) {
        page.spawn(Node {
            align_items: AlignItems::Stretch,
            ..row(7.0)
        })
        .with_children(|strip| {
            for &choice in choices {
                let active = building.assortment == choice;
                button_shell(
                    strip,
                    Action::Assortment(choice),
                    if active {
                        Style::Selected
                    } else {
                        Style::Quiet
                    },
                    false,
                    Node {
                        flex_grow: 1.0,
                        flex_basis: Val::Px(0.0),
                        min_width: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(7.0)),
                        ..column(3.0)
                    },
                )
                .with_children(|tile| {
                    let age = choice.flavor();
                    tile.spawn(row(4.0)).with_children(|id| {
                        let art = if active {
                            "check"
                        } else {
                            age.map_or(art, crate::model::Age::icon)
                        };
                        icon(id, hud, art, 22.0);
                        label(id, hud, choice.label(building.kind), 15.5, Face::Bold, SAND);
                    });
                    if requested == Some(choice) {
                        tile.spawn(row(3.0)).with_children(|hint| {
                            icon(hint, hud, "star", 16.0);
                            label(hint, hud, "Auftrag", 14.0, Face::Bold, GOLD);
                        });
                    }
                    if let Some(age) = age {
                        let people = match age {
                            crate::model::Age::Child => "Kinder",
                            crate::model::Age::Adult => "Erwachsene",
                            crate::model::Age::Senior => "Ältere",
                        };
                        let purchases = forecast.matches[choice as usize];
                        let coins = forecast.extra_coins[choice as usize];
                        label(
                            tile,
                            hud,
                            format!("{purchases} · {people}"),
                            14.5,
                            Face::Body,
                            MUTED,
                        );
                        label(
                            tile,
                            hud,
                            format!("+{coins} Münzen"),
                            15.0,
                            Face::Bold,
                            GOLD,
                        );
                    } else {
                        label(tile, hud, "Grundangebot", 14.5, Face::Body, MUTED);
                    }
                });
            }
            for _ in choices.len()..columns {
                strip.spawn(Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    ..default()
                });
            }
        });
    }
}

fn building_link(kind: Kind) -> Option<(&'static str, &'static str)> {
    Some(match kind {
        Kind::Bakery | Kind::Potter | Kind::Inn => ("water", "Mehr mit Wasser"),
        Kind::Baths => ("aqueduct", "Mehr am Aquädukt"),
        Kind::HerbGarden => ("bread", "Kräuter für Backstuben und Tavernen in der Nähe"),
        Kind::Storehouse => ("bread", "Vorräte für Läden im Umkreis von zwei Bauplätzen"),
        Kind::Grove => ("stoa", "Schatten am Wohnhof"),
        Kind::Taverna => ("bread", "Brot + Oliven"),
        Kind::Press => ("olive", "Braucht Oliven"),
        Kind::Shrine => ("potter", "Mehr mit Schalen"),
        Kind::Aqueduct => ("well", "Mehr am Brunnen"),
        Kind::Forum => ("ferry", "Kürzere Wege"),
        Kind::Villa => ("budget", "Größere Budgets"),
        Kind::RicePaddy => ("water", "Ein Brunnen nebenan erhöht die Reisernte"),
        Kind::RiceKitchen => (
            "rice_paddy",
            "Reis von der Insel liefert mehr Portionen · extra Bonus direkt am Feld",
        ),
        Kind::TeaGarden => (
            "tea_cup",
            "Mehr Tee im Teehaus · ab Stufe 3 bis zu zwei Bauplätze weit",
        ),
        Kind::TeaHouse => ("tea_garden", "Mehr Tee mit einem Teegarten in Reichweite"),
        Kind::PaperWorkshop => (
            "temple_bell",
            "Neben einem Tempel kommt beim Läuten ein Pilger mehr",
        ),
        Kind::Temple => (
            "tea_garden",
            "Ein Teegarten in Reichweite bringt zwei Plätze für Pilger",
        ),
        Kind::Fishery => (
            "smokehouse",
            "Kurze Wege zur Räucherei bringen mehr Portionen",
        ),
        Kind::Smokehouse => (
            "fishery",
            "Fang von der Fischerhütte · mehr mit Holz vom Holzplatz",
        ),
        Kind::Woodlot => ("smoked_fish", "Trockenes Holz für die Räucherei"),
        Kind::Boatyard => (
            "smoked_fish",
            "Proviant aus einer Räucherei im Umkreis von drei Bauplätzen",
        ),
        Kind::Apiary => (
            "herb_garden",
            "Kräutergarten bis zu zwei Plätze weit liefert mehr Honig",
        ),
        Kind::Dyer => ("water", "Krüge Wasser füllen die Farbbäder"),
        Kind::FerryOffice => (
            "ferry",
            "Mehr Fährgäste bis zu zwei Bauplätze vom Anleger entfernt",
        ),
        Kind::Mill => ("bread", "Mehl für Backstuben bis zu zwei Plätze weit"),
        Kind::Scribe => ("forum", "Mehr Wachstafeln mit einem Forum in der Nähe"),
        Kind::GardenCourt => (
            "aqueduct",
            "Ein Aquädukt in der Nähe schafft zusätzliche Rastplätze",
        ),
        Kind::MarketGarden => (
            "tea_cup",
            "Versorgt Reisküche, Teehaus und Seidenladen im Umkreis von zwei Bauplätzen",
        ),
        Kind::CeramicStudio => ("water", "Krüge Wasser füllen die Glasurbecken"),
        Kind::SilkShop => (
            "market_garden",
            "Färbepflanzen aus einem Gemüsegarten bringen mehr Stoffe",
        ),
        Kind::Sheepfold => (
            "weaver",
            "Wolle für Webereien im Umkreis von zwei Bauplätzen",
        ),
        Kind::Cooperage => (
            "boatyard",
            "Spart am benachbarten Bootshaus eine Portion Proviant je Fahrt",
        ),
        Kind::OasisHome => ("house", "Mit weiteren Stufen finden mehr Menschen Platz."),
        Kind::DateGarden => (
            "qanat",
            "Ein Qanat in der Nähe bringt drei zusätzliche Datteln.",
        ),
        Kind::SpiceBazaar => (
            "caravan",
            "Proviant für die Gewürzstraße und die Teppichhalle.",
        ),
        Kind::Caravanserai => (
            "caravan",
            "Ein Schlafplatz für Reisende und Treffpunkt der Karawanen.",
        ),
        Kind::Qanat => (
            "cistern",
            "Eine Zisterne in Reichweite bringt drei zusätzliche Krüge.",
        ),
        Kind::Glassworks => ("water", "Krüge Wasser kühlen das Glas ab."),
        Kind::Perfumer => ("water", "Wasser für die Destillation von Duftölen."),
        Kind::MoonGarden => ("moon", "Vier zusätzliche Rastplätze in einer Nachtschicht."),
        Kind::CarpetHall => (
            "spice_bazaar",
            "Mit einem Gewürzbasar in der Nähe steigt der Vorrat.",
        ),
        Kind::SignalTower => (
            "sea_trade",
            "Erhöht den Handelserlös bis zu zwei Plätze vom Bootshaus entfernt",
        ),
        Kind::FeastHall => (
            "smoked_fish",
            "Zwei Portionen Räucherfisch von nebenan ergeben fünf Plätze",
        ),
        _ => return None,
    })
}

pub(super) fn metric(p: &mut ChildSpawnerCommands, hud: &Hud, art: &str, text: impl Into<String>) {
    p.spawn(row(5.0)).with_children(|row| {
        icon(row, hud, art, 24.0);
        label(row, hud, text, 16.0, Face::Body, SAND);
    });
}

fn trade_routes(
    page: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    lot: usize,
    level: u8,
) {
    if let Some(voyage) = town.voyage {
        card(page, frame_node(Val::Percent(100.0), 6.0, 11.0), CARD).with_children(|status| {
            metric(
                status,
                hud,
                "sea_trade",
                format!(
                    "{} unterwegs · zurück ab Tag {}",
                    voyage.route.name(),
                    voyage.arrival
                ),
            );
            metric(
                status,
                hud,
                "coin",
                format!("Bei Ankunft +{} Silber", voyage.silver),
            );
        });
        return;
    }

    page.spawn(row(9.0)).with_children(|options| {
        for route in SailingRoute::ALL {
            let source = town.trade_source(chapter, lot, route);
            let ready = source.is_ok();
            card(
                options,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    min_width: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..column(6.0)
                },
                CARD,
            )
            .with_children(|details| {
                metric(details, hud, route.icon(), route.name());
                metric(
                    details,
                    hud,
                    "smoked_fish",
                    format!(
                        "{} Portionen Räucherfisch",
                        town.voyage_provisions(lot, route)
                    ),
                );
                metric(
                    details,
                    hud,
                    "coin",
                    format!(
                        "+{} Silber ab Tag {}",
                        town.voyage_silver(lot, route, level),
                        town.day + route.days()
                    ),
                );
                let weather = town.weather_ahead(chapter, route.days() as usize);
                if weather == Weather::Storm {
                    metric(details, hud, "storm", "Sturm verzögert die Rückkehr");
                }
                if let Err(reason) = source {
                    label(details, hud, reason.hint(), 15.0, Face::Body, MUTED);
                }
                if !town.planning() {
                    button(
                        details,
                        hud,
                        Action::SendVoyage(lot, route),
                        "Beladen & ablegen",
                        "ferry",
                        "",
                        Style::Primary,
                        !ready,
                    );
                }
            });
        }
    });
}

fn caravan_routes(
    page: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    lot: usize,
    level: u8,
) {
    let water = match &town.phase {
        Phase::Open(day) => day.caravan_water,
        _ => {
            let plan = town.production(chapter);
            plan.water.saturating_sub(plan.used_water)
        }
    };
    metric(page, hud, "water", format!("{water} Krüge in der Reserve"));
    for contract in Contract::ALL
        .into_iter()
        .filter(|c| c.available(town.chapter))
    {
        let (dates, spices, water) = contract.supplies();
        let needs = if spices > 0 {
            format!("{dates} Datteln · {spices} Gewürze · {water} Krüge")
        } else {
            format!("{dates} Datteln · {water} Krüge")
        };
        page.spawn(row(13.0)).with_children(|line| {
            icon(line, hud, "caravan", 30.0);
            line.spawn(Node {
                flex_grow: 1.0,
                ..column(3.0)
            })
            .with_children(|text| {
                label(text, hud, contract.name(), 18.0, Face::Bold, SAND);
                label(text, hud, needs, 15.0, Face::Body, MUTED);
            });
            let pay = contract.pay(chapter, level);
            let ready = town.caravan_supply(chapter, lot, contract);
            if town.planning() {
                metric(line, hud, "coin", format!("+{pay} Dirham"));
            } else {
                button(
                    line,
                    hud,
                    Action::SendCaravan(lot, contract),
                    &format!("Losschicken · +{pay}"),
                    "coin",
                    "",
                    Style::Primary,
                    ready.is_err(),
                );
            }
        });
        if !town.planning()
            && let Err(reason) = town.caravan_supply(chapter, lot, contract)
        {
            label(page, hud, reason.label(), 15.0, Face::Body, MUTED);
        }
    }
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
                    padding: UiRect::all(Val::Px(7.0)),
                    ..column(3.0)
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
                metric(
                    choice,
                    hud,
                    "coin",
                    format!(
                        "{} · {affordable}/{} können zahlen",
                        price.coins(service),
                        budgets.len()
                    ),
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
    use Kind::*;
    let stages = match (region, kind) {
        (Region::Greece, House) => [
            "Pflanzen am Eingang",
            "Seitliches Wohnhaus",
            "Schattenplatz",
            "Mosaikterrasse",
            "Höheres Seitenhaus",
        ],
        (Region::Rome, House) => [
            "Breite Loggia",
            "Gestützter Balkon",
            "Arkaden",
            "Überdachter Balkon",
            "Oberes Geschoss",
        ],
        (Region::Greece, Well) => [
            "Ziegeldach",
            "Tränke",
            "Zwei Wassergefäße",
            "Zwei Ausläufe",
            "Brunnenhalle",
        ],
        (Region::Rome, Well) => [
            "Brunnenarkade",
            "Doppelter Auslauf",
            "Rückwand",
            "Tieferes Becken",
            "Brunnenhalle",
        ],
        (Region::Japan, Well) => [
            "Wassertröge",
            "Auslauf",
            "Papierlaternen",
            "Breites Dach",
            "Steinlaternen",
        ],
        (Region::Nordic, Well) => [
            "Schöpfplatz",
            "Wassertröge",
            "Breites Dach",
            "Zweiter Trog",
            "Verzierte Pfosten",
        ],
        (_, Bakery) => [
            "Außenofen",
            "Brotregal",
            "Höherer Rauchfang",
            "Überdachte Auslage",
            "Backstubenboden",
        ],
        (_, HerbGarden) => [
            "Dichtere Kräuterbeete",
            "Rankhilfe · größere Reichweite",
            "Gießbecken",
            "Hochbeete",
            "Ranklaube",
        ],
        (Region::Greece, Grove) => [
            "Dritter Olivenbaum",
            "Überdachte Rastbank",
            "Weinranken",
            "Breite Pergola",
            "Blühende Hochbeete",
        ],
        (Region::Rome, Grove) => [
            "Mosaikpfad",
            "Rastarkade",
            "Weinranken",
            "Höhere Arkade",
            "Blumentröge",
        ],
        (Region::Greece, Potter) => [
            "Tonwarenregal",
            "Überdachte Auslage",
            "Trockengestell",
            "Werkstattboden",
            "Höherer Brennofen",
        ],
        (Region::Rome, Potter) => [
            "Außenofen",
            "Tonwarenregal",
            "Ziegeldach",
            "Werkstattboden",
            "Goldenes Ladenschild",
        ],
        (Region::Greece, Stoa) => [
            "Breite Stufen",
            "Tiefere Säulenhalle",
            "Geschmückter Giebel",
            "Höhere Säulen",
            "Giebelschmuck",
        ],
        (Region::Rome, Stoa) => [
            "Mosaikboden",
            "Tiefere Arkade",
            "Ziegelgiebel",
            "Höhere Arkaden",
            "Goldenes Relief",
        ],
        (_, Taverna) => [
            "Mosaikterrasse",
            "Überdachte Tische",
            "Leuchten am Ausschank",
            "Kammer hinter der Tafel",
            "Höhere Gaststube",
        ],
        (Region::Greece, Inn) => [
            "Breiter Balkon",
            "Gestützte Loggia",
            "Treppenaufgang",
            "Beschildertes Obergeschoss",
            "Balkondach",
        ],
        (Region::Rome, Inn) => [
            "Balkon",
            "Mosaik am Eingang",
            "Säulenloggia",
            "Drittes Fenster",
            "Herbergszeichen",
        ],
        (Region::Greece | Region::Rome, Storehouse) => [
            "Ladeplatz",
            "Lastenkran",
            "Überdachte Ablage",
            "Fenster im Speicherboden",
            "Großer Warenboden",
        ],
        (_, Press) => [
            "Verstärktes Pressdach",
            "Ölwanne",
            "Überdachte Presshalle",
            "Vorratsgefäße",
            "Höhere Mühle",
        ],
        (_, Shrine) => [
            "Altarstufen",
            "Mosaik vor dem Altar",
            "Giebel und Feuerschalen",
            "Höhere Säulen",
            "Goldener Dachschmuck",
        ],
        (_, Aqueduct) => [
            "Zweite Arkadenreihe",
            "Breites Auffangbecken",
            "Befestigte Wasserläufe",
            "Mosaik und Brunnenpfeiler",
            "Oberes Sammelbecken",
        ],
        (_, Baths) => [
            "Breiteres Badebecken",
            "Badegalerie",
            "Zwei Seitenkuppeln",
            "Badestufen",
            "Höherer Kuppelsaal",
        ],
        (_, Forum) => [
            "Marktkisten",
            "Obere Arkaden",
            "Stadtgiebel",
            "Überdachte Marktstände",
            "Breiter Treppenaufgang",
        ],
        (_, Villa) => [
            "Mosaikhof",
            "Tiefere Säulenloge",
            "Anbau mit Stufen",
            "Breiteres Wasserbecken",
            "Zypressen am Eingang",
        ],
        (_, Apiary) => [
            "Dritter Bienenkorb",
            "Überdachter Wabenplatz",
            "Vierter Bienenkorb",
            "Blühender Vorgarten",
            "Honigabfüllung",
        ],
        (_, Dyer) => [
            "Erste Trockenstoffe",
            "Gestell für Farbstoffe",
            "Drittes Farbbad",
            "Höhere Färbehalle",
            "Zweites Trockengestell",
        ],
        (_, FerryOffice) => [
            "Mehr Fährwimpel",
            "Überdachter Warteplatz",
            "Höherer Signalmast",
            "Oberer Ausguck",
            "Treppen und Leuchtfeuer",
        ],
        (_, Mill) => [
            "Mehlsäcke",
            "Vorratskiste",
            "Größere Mahlsteine",
            "Überdachte Mühle",
            "Mosaik und Speicherfenster",
        ],
        (_, Scribe) => [
            "Mosaik am Schreibpult",
            "Überdachte Ablage",
            "Travertinbögen",
            "Schild der Schreibstube",
            "Oberes Schreibzimmer",
        ],
        (_, GardenCourt) => [
            "Wasserspiel",
            "Laubengang",
            "Zwei Bänke",
            "Höhere Arkaden",
            "Begrünter Laubengang",
        ],
        (Region::Japan, Machiya | Inn) => [
            "Veranda mit Laternen",
            "Breiter Erdgeschossvorbau",
            "Schattige Veranda",
            "Oberer Wohnraum",
            "Ausgearbeitetes Obergeschoss",
        ],
        (_, RicePaddy) => [
            "Junge Reishalme",
            "Dichtere Saat",
            "Bewässerungsrinne",
            "Reife Reisbüschel",
            "Gestell für die Ernte",
        ],
        (_, TeaGarden) => [
            "Bambus",
            "Steinlaterne · größere Reichweite",
            "Bambushain",
            "Zweite Steinlaterne",
            "Ahorn am Garten",
        ],
        (_, RiceKitchen) => [
            "Weitere Reisschalen",
            "Überdachte Auslage",
            "Dampfkessel",
            "Obere Reiskammer",
            "Geschmückte Küche",
        ],
        (_, TeaHouse) => [
            "Teegeschirr",
            "Schattige Veranda",
            "Geschmückter Teetisch",
            "Höherer Teeraum",
            "Offener Teesalon",
        ],
        (_, PaperWorkshop) => [
            "Papiergestell",
            "Vordach für Lampions",
            "Größere Lampionauslage",
            "Höhere Werkstatt",
            "Schrein der Lampions",
        ],
        (_, Temple) => [
            "Steinlaternen",
            "Geschmückte Vorhalle",
            "Dritte Dachstufe",
            "Lampions am Aufgang",
            "Goldene Dachspitze",
        ],
        (Region::Japan, Storehouse) => [
            "Ladekisten",
            "Vordach",
            "Papierlaternen",
            "Oberer Lagerraum",
            "Steinlaternen",
        ],
        (_, MarketGarden) => [
            "Viertes Gemüsebeet",
            "Bewässerungsbecken",
            "Hochbeete",
            "Bepflanzter Laubengang",
            "Bambus und Steinlaterne",
        ],
        (_, CeramicStudio) => [
            "Glasierte Schalen",
            "Überdachte Auslage",
            "Werkstattlaterne",
            "Oberes Werkstattfenster",
            "Hoher Brennofen",
        ],
        (_, SilkShop) => [
            "Mehr Stoffbahnen",
            "Überdachter Stoffstand",
            "Breite Veranda",
            "Oberer Webraum",
            "Seidengalerie",
        ],
        (_, Longhouse) => [
            "Holzbank",
            "Vordach",
            "Seitenkammer",
            "Größerer Wohnraum",
            "Geschmückter Eingang",
        ],
        (_, FeastHall) => [
            "Gästebänke",
            "Vordach",
            "Seitenkammer",
            "Höhere Festhalle",
            "Geschmückter Eingang",
        ],
        (_, Fishery) => [
            "Zweites Trockengestell",
            "Beiboot",
            "Überdachter Fangplatz",
            "Breiter Steg",
            "Netzaufzug",
        ],
        (_, Smokehouse) => [
            "Zweites Räuchergestell",
            "Räucherkammer",
            "Höherer Rauchfang",
            "Vordach",
            "Überdachte Kühlung",
        ],
        (_, Woodlot) => [
            "Holzstapel",
            "Holzschuppen",
            "Mehr Holzstapel",
            "Sortierter Stapelplatz",
            "Überdachter Ladeplatz",
        ],
        (_, Boatyard) => [
            "Seilwinde · Fernhandel",
            "Seilschuppen",
            "Hafenfahne",
            "Verbreiterter Steg",
            "Höheres Werkdach",
        ],
        (_, Weaver) => [
            "Weiterer Webstuhl",
            "Veranda",
            "Färbekammer",
            "Höhere Webstube",
            "Gewebte Fahnen",
        ],
        (Region::Nordic, Storehouse) => [
            "Ladebank",
            "Kleiner Schuppen",
            "Vordach",
            "Höherer Speicher",
            "Markierte Pfosten",
        ],
        (_, Sheepfold) => [
            "Drittes Schaf",
            "Viehtränke",
            "Höherer Weidezaun",
            "Stall mit viertem Schaf",
            "Heuballen und Weidetor",
        ],
        (_, Cooperage) => [
            "Drittes Fass",
            "Geschützte Ablage",
            "Großes Vorratsfass",
            "Höhere Werkstatt",
            "Fassbinderzunft",
        ],
        (_, SignalTower) => [
            "Höherer Leuchtkorb",
            "Brennholzschuppen",
            "Signalflagge",
            "Wappenschild",
            "Gesicherter Feuerplatz",
        ],
        (_, OasisHome) => [
            "Palisanderfenster",
            "Breitere Hofstufen",
            "Seitengalerie",
            "Oberer Wohnraum",
            "Gestufter Dachhof",
        ],
        (_, Cistern) => [
            "Zweiter Krug",
            "Schattendach",
            "Tieferes Becken",
            "Zweiter Auslauf",
            "Kuppel über der Zisterne",
        ],
        (_, DateGarden) => [
            "Dritte Palme",
            "Bewässerungsbecken",
            "Terrasse",
            "Erntekorb",
            "Überdachter Ernteplatz",
        ],
        (_, SpiceBazaar) => [
            "Weitere Schalen",
            "Längere Markise",
            "Zweiter Marktstand",
            "Bogenhalle",
            "Oberes Warenlager",
        ],
        (_, Caravanserai) => [
            "Breitere Treppe",
            "Seitengalerie",
            "Obere Arkaden",
            "Geschmückte Hoffassade",
            "Hof mit zwei Kuppeln",
        ],
        (_, Qanat) => [
            "Höherer Schacht",
            "Doppelter Wasserlauf",
            "Begehbare Arkade",
            "Zweites Sammelbecken",
            "Geschützter Wasserturm",
        ],
        (_, Glassworks) => [
            "Kühlbecken",
            "Gläserregal",
            "Breiter Ofen",
            "Höhere Werkstatt",
            "Große Glashalle",
        ],
        (_, Perfumer) => [
            "Kupferkolben",
            "Kräutertisch",
            "Überdachter Eingang",
            "Oberes Destillierzimmer",
            "Galerie für Duftwaren",
        ],
        (_, MoonGarden) => [
            "Blühende Büsche",
            "Überdachte Rastbank",
            "Beleuchteter Gang",
            "Weiteres Wasserbecken",
            "Kleine Gartenkuppel",
        ],
        (_, CarpetHall) => [
            "Neue Muster",
            "Überdachte Auslage",
            "Größere Webstühle",
            "Obere Webhalle",
            "Große Tuchgalerie",
        ],
        _ => unreachable!("building unavailable in this region"),
    };
    stages[usize::from(tier - 2)]
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
    } else if building.kind == Kind::HerbGarden {
        (
            "herb_garden",
            u16::from(building.level) + 1,
            u16::from(next.level) + 1,
            "Portionen je Küche",
        )
    } else if building.kind == Kind::RicePaddy {
        ("rice_paddy", before.rice, after.rice, "Körbe Reis")
    } else if building.kind == Kind::TeaGarden {
        (
            "tea_cup",
            u16::from(building.level) * 2,
            u16::from(next.level) * 2,
            "Schalen je Teehaus",
        )
    } else if building.kind == Kind::Fishery {
        ("fishery", before.fish, after.fish, "Fische")
    } else if building.kind == Kind::Woodlot {
        ("woodlot", before.wood, after.wood, "Scheite")
    } else if building.kind == Kind::Boatyard {
        let route = SailingRoute::Coastal;
        (
            "sea_trade",
            town.voyage_silver(lot, route, building.level) as u16,
            town.voyage_silver(lot, route, next.level) as u16,
            "Silber im Inselhandel",
        )
    } else if building.kind == Kind::SignalTower {
        (
            "sea_trade",
            u16::from(building.level) * 3,
            u16::from(next.level) * 3,
            "Silber pro Fahrt",
        )
    } else if building.kind == Kind::FerryOffice {
        (
            "ferry",
            u16::from(building.level).div_ceil(2),
            u16::from(next.level).div_ceil(2),
            "zusätzliche Ankünfte",
        )
    } else if matches!(
        building.kind,
        Kind::Storehouse | Kind::Mill | Kind::MarketGarden | Kind::Sheepfold
    ) {
        let bonus = if building.kind == Kind::Storehouse {
            0
        } else {
            2
        };
        (
            "bread",
            u16::from(building.level) + bonus,
            u16::from(next.level) + bonus,
            "Portionen für Betriebe in Reichweite",
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
            objective_list(page, hud, town, chapter);
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
            contract_card(page, hud, town, chapter);
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

fn objective_list(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    use crate::campaign::Goal;
    let extra = crate::campaign_orders::extra_goals(chapter, town.chapter);
    if extra.iter().all(Option::is_none) {
        for task in chapter.goals {
            super::screens::goal(p, hud, town, task);
        }
        return;
    }
    let last = extra.into_iter().flatten();
    let order = crate::campaign_orders::trade_order(chapter, town.chapter);
    let basics = chapter.goals.into_iter().chain(last.clone().filter(|task| {
        order.is_some() && !matches!(task, Goal::Sale(..) | Goal::TradeUpgrade(..))
    }));
    let followups = last
        .filter(|task| order.is_none() || matches!(task, Goal::Sale(..) | Goal::TradeUpgrade(..)));
    let main = |p: &mut ChildSpawnerCommands| {
        for task in basics {
            campaign_goal(p, hud, town, task);
        }
    };
    let extra = |p: &mut ChildSpawnerCommands| {
        if let Some(order) = order {
            metric(
                p,
                hud,
                kind_icon(order.shop),
                format!("Handelsauftrag · {}", order.shop.name()),
            );
        }
        for task in followups {
            campaign_goal(p, hud, town, task);
        }
    };
    if hud.phone {
        main(p);
        line(p, LINE);
        extra(p);
    } else {
        p.spawn(Node {
            align_items: AlignItems::Start,
            ..row(23.0)
        })
        .with_children(|columns| {
            let group = Node {
                flex_basis: Val::Px(0.0),
                flex_grow: 1.0,
                min_width: Val::Px(0.0),
                ..column(10.0)
            };
            columns.spawn(group.clone()).with_children(main);
            columns.spawn(group).with_children(extra);
        });
    }
}

fn campaign_goal(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    task: crate::campaign::Goal,
) {
    use crate::campaign::Goal;
    super::screens::goal(p, hud, town, task);
    match task {
        Goal::GuestCare(_, minimum, _) => {
            label(
                p,
                hud,
                format!("Ab {minimum} zufriedenen Gästen dieser Gruppe an einem Tag."),
                14.0,
                Face::Body,
                MUTED,
            );
        }
        Goal::FavoriteMenu(_, _) => {
            label(
                p,
                hud,
                "Am selben Tag, aus verschiedenen Warengruppen.",
                14.0,
                Face::Body,
                MUTED,
            );
        }
        Goal::Rebuild(order) if town.planning() && town.goal(task).0 < task.target() => {
            button(
                p,
                hud,
                Action::LookAtLot(order.lot),
                "Bauplatz öffnen",
                "hammer",
                "",
                Style::Quiet,
                false,
            );
        }
        Goal::TradeUpgrade(order)
            if super::shown_goal(town, Goal::Sale(order, false)).0 < order.basic =>
        {
            label(
                p,
                hud,
                "Nach dem Verkauf des Grundangebots",
                14.0,
                Face::Body,
                MUTED,
            );
        }
        Goal::Sale(order, true)
            if super::shown_goal(town, Goal::TradeUpgrade(order)).0
                >= u32::from(order.special.tier())
                && town.goal(task).0 < task.target() =>
        {
            let help = order
                .special
                .flavor()
                .filter(|_| order.favorite_only)
                .map_or_else(
                    || "Wähle dieses Angebot morgens im Gebäude.".to_owned(),
                    |age| format!("Morgens im Laden wählen · {}", age.group()),
                );
            label(p, hud, help, 14.0, Face::Body, MUTED);
        }
        Goal::Dogs(_) if town.planning() && town.goal(task).0 < task.target() => {
            label(
                p,
                hud,
                "In der ersten Schichthälfte kommt ein Hund in die Gasse.",
                14.0,
                Face::Body,
                MUTED,
            );
        }
        _ => {}
    }
}

fn contract_card(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    let Some(goal) = town.daily_contract(chapter) else {
        return;
    };
    let (text, icon_name) = goal.label();
    let (have, need) = if let Phase::Open(day) = &town.phase {
        goal.progress(&day.tally)
    } else {
        goal.progress(&Default::default())
    };
    card(p, frame_node(Val::Percent(100.0), 3.0, 9.0), JADE).with_children(|order| {
        order
            .spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(9.0)
            })
            .with_children(|header| {
                label(header, hud, "Extra für heute", 16.0, Face::Bold, GOLD);
                metric(header, hud, "coin", format!("+{} Münzen", goal.reward()));
            });
        metric(order, hud, icon_name, format!("{text} · {have}/{need}"));
    });
}

fn forecast(p: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, town: &Town, chapter: &Chapter) {
    let books = [
        ("Heute", "sun"),
        ("Gästebuch", "guest_book"),
        ("Wetter", "cloud"),
        ("Notizen", "book"),
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
        "Tagesplan",
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
                        matches!(i, 0 | 2) && matches!(town.phase, Phase::Report(_)),
                    );
                }
            });
            match game.ledger_page {
                0 => supplies(page, hud, town, chapter),
                1 => guests::book(page, hud, game, town, chapter),
                2 => day_plan(page, hud, town, chapter),
                _ => morning_notes(page, hud, town),
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

fn age_row(
    page: &mut ChildSpawnerCommands,
    hud: &Hud,
    title: &str,
    ages: [u16; 3],
    residents: bool,
    island: usize,
) {
    use crate::model::Age;
    card(page, frame_node(Val::Percent(100.0), 6.0, 10.0), DEEP).with_children(|group| {
        label(group, hud, title, 16.0, Face::Bold, GOLD);
        group
            .spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(12.0)
            })
            .with_children(|counts| {
                for (age, count) in Age::ALL.into_iter().zip(ages) {
                    if !age.available(island) {
                        continue;
                    }
                    let word = if residents && age == Age::Senior {
                        "Ältere"
                    } else {
                        age.group()
                    };
                    super::screens::stat(counts, hud, age.icon(), &count.to_string(), word);
                }
            });
    });
}

fn morning_notes(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town) {
    use crate::model::{Age, Miss};
    let visits = &town.last_visits;
    let title = match town.phase {
        Phase::Report(_) => "Notizen für morgen",
        _ => "Notizen vom Vortag",
    };
    label(page, hud, title, 23.0, Face::Serif, SAND);
    if visits.is_empty() {
        metric(
            page,
            hud,
            "guest_wait",
            "Hier stehen nach dem ersten Markttag die Rückmeldungen.",
        );
        return;
    }
    let mut wishes = [[0u32; 3]; SERVICES];
    let mut missed = [[0u32; 4]; SERVICES];
    let mut matched = [0u32; SERVICES];
    for (i, visit) in visits.iter().enumerate() {
        let service = visit.wants as usize;
        wishes[service][visit.age(i) as usize] += 1;
        if let Some(reason) = visit.miss {
            missed[service][reason as usize] += 1;
        }
        if visit.specialty_bonus > 0 {
            matched[service] += 1;
        }
    }
    let mut services: Vec<_> = Service::ALL
        .into_iter()
        .filter(|s| wishes[*s as usize].iter().sum::<u32>() > 0)
        .collect();
    services.sort_by_key(|&s| {
        let i = s as usize;
        std::cmp::Reverse((missed[i].iter().sum::<u32>(), wishes[i].iter().sum::<u32>()))
    });
    for service in services.into_iter().take(4) {
        let i = service as usize;
        card(page, frame_node(Val::Percent(100.0), 4.0, 8.0), CARD).with_children(|note| {
            note.spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(8.0)
            })
            .with_children(|header| {
                metric(header, hud, service_icon(service), service.name());
                if matched[i] > 0 {
                    metric(
                        header,
                        hud,
                        "star",
                        format!("{} Lieblingsangebote", matched[i]),
                    );
                }
                header.spawn(row(9.0)).with_children(|people| {
                    for age in Age::ALL
                        .into_iter()
                        .filter(|age| age.available(town.chapter))
                    {
                        metric(people, hud, age.icon(), wishes[i][age as usize].to_string());
                    }
                });
            });
            let reasons = [
                (Miss::NoShop, "Kein Laden"),
                (Miss::Walk, "Weg zu weit"),
                (Miss::Price, "Zu teuer"),
                (Miss::Empty, "Kein Vorrat / Platz"),
            ];
            note.spawn(row(14.0)).with_children(|result| {
                for (reason, word) in reasons {
                    let count = missed[i][reason as usize];
                    if count > 0 {
                        let symbol = match reason {
                            Miss::NoShop => "hammer",
                            Miss::Walk => "guest_steps",
                            Miss::Price => "budget",
                            Miss::Empty => "guest_empty",
                        };
                        metric(result, hud, symbol, format!("{count} · {word}"));
                    }
                }
                if missed[i].iter().all(|&n| n == 0) {
                    metric(
                        result,
                        hud,
                        "guest_happy",
                        "Alle Interessierten wurden bedient.",
                    );
                }
            });
        });
    }
}

fn supplies(page: &mut ChildSpawnerCommands, hud: &Hud, town: &Town, chapter: &Chapter) {
    age_row(
        page,
        hud,
        &format!("Tag {} · Heute erwartet", town.day),
        town.age_forecast(chapter),
        false,
        town.chapter,
    );
    if town.planning() && !town.last_visits.is_empty() {
        button(
            page,
            hud,
            Action::LedgerPage(3),
            "Notizen vom Vortag",
            "book",
            "",
            Style::Quiet,
            false,
        );
    }
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
                "Noch aus der Gasse"
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
                "Noch von der Fähre"
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
        if production.olives > 0 || production.used_olives > 0 {
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
    let schedule = town.weather_schedule(chapter);
    let current = town.sky_phase(chapter);
    page.spawn(Node {
        align_items: AlignItems::Stretch,
        ..row(9.0)
    })
    .with_children(|parts| {
        for (phase, (time, sky)) in crate::model::WEATHER_PHASES
            .into_iter()
            .zip(schedule)
            .enumerate()
        {
            card(
                parts,
                Node {
                    flex_grow: 1.0,
                    flex_basis: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..row(7.0)
                },
                if phase == current { JADE } else { CARD },
            )
            .with_children(|slot| {
                icon(slot, hud, weather_icon(sky), 34.0);
                slot.spawn(column(0.0)).with_children(|copy| {
                    label(
                        copy,
                        hud,
                        chapter.shift.clock(time),
                        14.0,
                        Face::Body,
                        MUTED,
                    );
                    label(copy, hud, sky.name(), 20.0, Face::Bold, SAND);
                });
            });
        }
    });
    label(
        page,
        hud,
        if chapter.region == Region::Nordic {
            "Fähre, Wasser und Fang"
        } else {
            "Fähre und Wasser"
        },
        16.0,
        Face::Bold,
        GOLD,
    );
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
                if chapter.region == Region::Nordic
                    && matches!(weather, Weather::Wind | Weather::Storm)
                {
                    metric(
                        day,
                        hud,
                        "fishery",
                        if weather == Weather::Storm {
                            "−6 Fische pro Hütte"
                        } else {
                            "−2 Fische pro Hütte"
                        },
                    );
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
        metric(page, hud, "check", favor.resolved_name());
    } else if !favor.incident() || town.incident(chapter).is_some() {
        page.spawn(row(12.0)).with_children(|request| {
            metric(request, hud, favor.icon(), favor.name());
            label(request, hud, favor.note(), 15.0, Face::Body, MUTED);
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
    let onward = next_harbor(town.chapter);
    let milestone = clear && !tutorial && main_route_end(town.chapter);
    let bonus_complete = clear && !milestone && onward.is_none();
    let next_region = milestone.then(|| onward.map(Region::of)).flatten();
    let next_label = match next_region {
        Some(Region::Rome) => "Nach Rom segeln",
        Some(Region::Japan) => "Nach Japan segeln",
        Some(Region::Nordic) => "In den Norden segeln",
        Some(Region::Oasis) => "Zu den Oasen reisen",
        _ if onward.is_none() => "Zur Kampagnenkarte",
        _ => "Nächster Hafen",
    };
    let tutorial_complete = clear && tutorial && town.chapter + 1 == coach::ISLANDS;
    let title = if milestone {
        "Zwölf Häfen geschafft"
    } else if bonus_complete {
        "Bonusroute abgeschlossen"
    } else if clear {
        "Der Hafen blüht auf"
    } else if chapter.shift.night() {
        "Morgengrauen am Kai"
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
            if report.tally.street_bonus > 0
                || report.tally.specialty_income > 0
                || report.contract_bonus > 0
            {
                page.spawn(row(14.0)).with_children(|bonuses| {
                    if report.tally.street_bonus > 0 {
                        metric(
                            bonuses,
                            hud,
                            "nili",
                            format!("Gasse · +{} Ansehen", report.tally.street_bonus),
                        );
                    }
                    if report.tally.specialty_income > 0 {
                        metric(
                            bonuses,
                            hud,
                            "bread",
                            format!("Angebote · +{} Münzen", report.tally.specialty_income),
                        );
                    }
                    if report.contract_bonus > 0 {
                        metric(
                            bonuses,
                            hud,
                            "check",
                            format!("Tagesextra · +{} Münzen", report.contract_bonus),
                        );
                    }
                });
            }
            if clear {
                let next = onward.map(|i| (i, &CHAPTERS[i]));
                card(page, frame_node(Val::Percent(100.0), 5.0, 11.0), JADE).with_children(
                    |victory| {
                        victory
                            .spawn(Node {
                                justify_content: JustifyContent::SpaceBetween,
                                ..row(10.0)
                            })
                            .with_children(|band| {
                                band.spawn(row(5.0)).with_children(|stars| {
                                    for i in 0..3 {
                                        icon(
                                            stars,
                                            hud,
                                            if i < report.stars { "star" } else { "lock" },
                                            35.0,
                                        );
                                    }
                                    label(
                                        stars,
                                        hud,
                                        format!("{} von 3 Sternen", report.stars),
                                        19.0,
                                        Face::Bold,
                                        SAND,
                                    );
                                });
                                if let Some((index, next)) = next.filter(|_| !tutorial) {
                                    icon(band, hud, &format!("island_{index}"), 58.0);
                                    band.spawn(column(0.0)).with_children(|reward| {
                                        label(reward, hud, next.name, 21.0, Face::Serif, SAND);
                                        let bonus = crate::save::travel_bonus(
                                            index,
                                            town.totals.income + town.totals.caravan_income,
                                        );
                                        label(
                                            reward,
                                            hud,
                                            format!("+{bonus} Startmünzen"),
                                            17.0,
                                            Face::Bold,
                                            GOLD,
                                        );
                                    });
                                }
                            });
                        if !tutorial {
                            metric(
                                victory,
                                hud,
                                "coin",
                                format!(
                                    "{} Münzen Handelsumsatz in diesem Hafen",
                                    town.totals.income
                                ),
                            );
                        }
                    },
                );
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
                    if milestone && next_region.is_some() {
                        "Das nächste Gebiet und die Bonusroute sind jetzt frei."
                    } else if milestone {
                        "Du hast alle fünf Hauptreisen abgeschlossen. Die Bonusroute der Oasen ist frei."
                    } else if bonus_complete {
                        "Du hast den letzten Hafen dieser Bonusroute erreicht."
                    } else {
                        "Der nächste Hafen ist frei."
                    },
                    19.0,
                    Face::Body,
                    SAND,
                );
            } else {
                objective_list(page, hud, town, chapter);
                if !tutorial && let Some(next) = onward {
                    let bonus = crate::save::travel_bonus(
                        next,
                        town.totals.income + town.totals.caravan_income,
                    );
                    metric(
                        page,
                        hud,
                        "coin",
                        format!("Reisekasse · +{bonus} Startmünzen beim Hafenabschluss"),
                    );
                }
            }
            if town.last_visits.is_empty() {
                let (advice, image) = guests::evening_hint(town, &report.tally);
                hint(page, hud, &advice, image);
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
                    Action::TomorrowNotes,
                    "Notizen",
                    "book",
                    "",
                    Style::Quiet,
                    false,
                );
                if clear {
                    if milestone {
                        button(
                            actions,
                            hud,
                            Action::BonusHarbor,
                            "Bonusroute",
                            "star",
                            "",
                            Style::Quiet,
                            false,
                        );
                    } else if !tutorial {
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
                        } else {
                            next_label
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
    if hud.phone {
        return modal(p, hud, 690.0, "", "Spielhilfe", "guide", |page| {
            phone::help(page, hud, Action::Back);
        });
    }
    const PAGES: [&str; 8] = [
        "Steuerung",
        "Dein Tag",
        "Waren",
        "Nili",
        "Reisen",
        "Rom",
        "Japan",
        "Nordisch",
    ];
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
                5 => &[
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
                6 => &[
                    (
                        "rice_paddy",
                        "rice_bowl",
                        "Vom Feld in die Küche",
                        "Ein Brunnen neben dem Feld bringt mehr Reis. Neben einem Feld kocht die Reisküche mehr Portionen.",
                    ),
                    (
                        "tea_garden",
                        "tea_cup",
                        "Tee aus dem eigenen Garten",
                        "Mehr Schalen für nahe Teehäuser. Ab Stufe 3 reicht der Garten weiter.",
                    ),
                    (
                        "temple_bell",
                        "temple",
                        "Die Pilgerglocke",
                        "Geh tagsüber mit Nili zum Tempel. Läute mit A oder tippe auf die Glocke. Die Gäste beten kostenlos.",
                    ),
                    (
                        "lampion",
                        "temple_bell",
                        "Lampions am Weg",
                        "Mit einer Lampionwerkstatt direkt neben dem Tempel kommt beim Läuten ein Pilger mehr.",
                    ),
                ],
                _ => &[
                    (
                        "fishery",
                        "smokehouse",
                        "Fisch für die Räucherei",
                        "Fischerhütten liefern Fang. Holzplätze erhöhen den Ertrag. Bei Wind und Sturm fällt der Fang kleiner aus.",
                    ),
                    (
                        "smoked_fish",
                        "feast_hall",
                        "Die Festhalle versorgen",
                        "Eine benachbarte Räucherei gibt zwei Portionen an die Festhalle ab und bewirtet so fünf zusätzliche Gäste.",
                    ),
                    (
                        "boatyard",
                        "sea_trade",
                        "Für den Seehandel beladen",
                        "Geh tagsüber zum Bootshaus und wähle eine Fahrt. Das Schiff lädt Fisch aus einer Räucherei in der Nähe.",
                    ),
                    (
                        "storm",
                        "coin",
                        "Wetter und Rückkehr",
                        "Inselhandel dauert einen Tag. Fernhandel ab Stufe 2 dauert zwei Tage. Bei Sturm kehrt das Schiff später zurück.",
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

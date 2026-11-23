use super::*;
use crate::{
    adventure::{
        Adventure, Build, BuildingSite, Command, Good, HALL_STORAGE, Island, Level, MAX_BUILD_CREW,
        NILI_CAPACITY, Quest, Screen, Target, Task, Terrain,
    },
    campaign::{CORE_HARBORS, Region},
    world::StreetCamera,
};

const GREEN: Color = Color::srgb(0.63, 0.87, 0.67);
const WARN: Color = Color::srgb(1.0, 0.69, 0.56);

pub(super) fn draw(root: &mut ChildSpawnerCommands, hud: &Hud, game: &Game, a: &Adventure) {
    if a.screen == Screen::Atlas {
        atlas(root, hud, a);
        return;
    }
    let Some((terrain, island)) = a.terrain.as_ref().zip(a.active()) else {
        return;
    };
    let level = terrain.level;
    match a.screen {
        Screen::Atlas => {}
        Screen::Brief => {
            panels::modal(
                root,
                hud,
                680.0,
                level.region.name(),
                level.name,
                level.region.icon(),
                |p| {
                    label(p, hud, level.theme, 21.0, Face::Body, SAND);
                    let (art, hint) = match level.stage {
                        1 => (
                            "storehouse",
                            "Pack im Rathaus die erste Lieferung und bring sie zur Außenstation.",
                        ),
                        2 => (
                            "adventure_bricks",
                            "Aus Lehm, Holz und Wasser brennt die Ziegelei Ziegel.",
                        ),
                        3 => (
                            "adventure_forge",
                            "Werkzeug kommt aus der Schmiede. Dafür brauchst du Erz.",
                        ),
                        6 => (
                            "adventure_blueprint",
                            "Das Heiligtum kennt den Bauplan. Markiere den Weg im Auftragsbuch.",
                        ),
                        7 => (
                            "adventure_craft",
                            "Für den Bauplan lieferst du Waren am Heiligtum ab.",
                        ),
                        9 => (
                            "hammer",
                            "Du kannst schon bauen. Für die höheren Stufen brauchst du den Bauplan.",
                        ),
                        11 => (
                            "adventure_hill",
                            "Diese Insel braucht sowohl ein Fundstück als auch eine Warenlieferung.",
                        ),
                        _ => (
                            "book",
                            "Die Aufträge zeigen dir, welche Gebäude und Vorräte diese Insel braucht.",
                        ),
                    };
                    p.spawn(row(10.0)).with_children(|r| {
                        icon(r, hud, art, 42.0);
                        label(r, hud, hint, 19.0, Face::Bold, GOLD);
                    });
                    action(
                        p,
                        hud,
                        Command::Close,
                        "An Land gehen",
                        "ferry",
                        "A",
                        Style::Primary,
                        false,
                    );
                },
            );
        }

        Screen::Walk => walk(root, hud, game, a, island, level),
        Screen::Lot(lot) => {
            if let Some(site) = island.site(lot) {
                construction_panel(root, hud, a, island, level, lot, site);
            } else if let Some(property) = island.lots[lot] {
                property_panel(root, hud, island, terrain, lot, property);
            } else {
                build_menu(root, hud, a, island, level);
            }
        }
        Screen::Demolish(lot) => {
            let Some(property) = island.lots[lot] else {
                return;
            };
            panels::modal(root, hud, 520.0, "", "Gebäude abreißen?", "hammer", |p| {
                label(
                    p,
                    hud,
                    property.kind.name(level.region),
                    22.0,
                    Face::Bold,
                    SAND,
                );
                label(
                    p,
                    hud,
                    "Der Bauplatz wird frei. Du erhältst die Hälfte der ursprünglichen Baukosten zurück.",
                    20.0,
                    Face::Body,
                    MUTED,
                );
                p.spawn(row(9.0)).with_children(|r| {
                    action(
                        r,
                        hud,
                        Command::Close,
                        "Behalten",
                        "",
                        "B",
                        Style::Primary,
                        false,
                    );
                    action(
                        r,
                        hud,
                        Command::Remove,
                        "Abreißen",
                        "hammer",
                        "",
                        Style::Danger,
                        false,
                    );
                });
            });
        }
        Screen::Resource(id) => {
            let good = terrain.deposits[id].kind;
            let remaining = island.remaining(terrain, id);
            panels::modal(
                root,
                hud,
                580.0,
                good.district_name(),
                good.name(level.region),
                good.icon(level.region),
                |p| {
                    p.spawn(row(14.0)).with_children(|r| {
                        stock(r, hud, good, island.amount(good), level.region);
                        label(r, hud, "im Lager", 17.0, Face::Body, MUTED);
                        stock(r, hud, good, remaining, level.region);
                        label(r, hud, "an dieser Stelle", 17.0, Face::Body, MUTED);
                    });
                    team(p, hud, island, Task::Gather(id));
                    p.spawn(row(10.0)).with_children(|r| {
                        action(
                            r,
                            hud,
                            Command::Use(Target::Deposit(id)),
                            good.gather_action(),
                            good.icon(level.region),
                            "",
                            Style::Primary,
                            remaining == 0 || island.room_for(good) == 0,
                        );
                        back(r, hud, "Weitergehen");
                    });
                },
            );
        }
        Screen::Labor => labor(root, hud, a, terrain, island),
        Screen::Mission => mission(root, hud, a, island, level),
        Screen::Map => map(root, hud, a, terrain, island),
        Screen::Storage => storage(root, hud, a, island, level),
        Screen::Pause => {
            panels::modal(root, hud, 608.0, level.name, "Pause", "pause", |p| {
                action(
                    p,
                    hud,
                    Command::Close,
                    "Weiterspielen",
                    "",
                    "A",
                    Style::Primary,
                    false,
                );
                if island.completed {
                    action(
                        p,
                        hud,
                        Command::Summary,
                        "Siedlungsbilanz",
                        "check",
                        "",
                        Style::Quiet,
                        false,
                    );
                }
                button(
                    p,
                    hud,
                    Action::Wardrobe,
                    "Nilis Garderobe",
                    "nili",
                    "",
                    Style::Quiet,
                    false,
                );
                action(
                    p,
                    hud,
                    Command::Help,
                    "Steuerung und Hilfe",
                    "guide",
                    "",
                    Style::Quiet,
                    false,
                );
                action(
                    p,
                    hud,
                    Command::Atlas,
                    "Inselkarte",
                    "chart",
                    "",
                    Style::Quiet,
                    false,
                );
                action(
                    p,
                    hud,
                    Command::Home,
                    "Hauptmenü",
                    "",
                    "",
                    Style::Quiet,
                    false,
                );
            });
        }
        Screen::Help if hud.phone => panels::modal(
            root,
            hud,
            660.0,
            level.name,
            "Spielhilfe",
            "guide",
            |page| phone::help(page, hud, Action::Adventure(Command::Close)),
        ),
        Screen::Help => {
            panels::modal(
                root,
                hud,
                680.0,
                level.name,
                "Steuerung und Hilfe",
                "guide",
                |p| {
                    for (art, title, input) in [
                        (
                            "nili",
                            "Nili bewegen",
                            "Linker Stick, WASD oder Touch-Stick",
                        ),
                        (
                            "adventure_wood",
                            "Sammeln oder öffnen",
                            "A, Eingabe oder auf den Ort tippen",
                        ),
                        (
                            "adventure_labor",
                            "Details und Bürger",
                            "Rechter Stick-Klick vor einem Rohstoff oder Y für alle",
                        ),
                        (
                            "chart",
                            "Wege markieren",
                            "X und ein Ziel auf der Karte wählen",
                        ),
                        ("book", "Inselziele ansehen", "RB oder auf Aufträge tippen"),
                    ] {
                        p.spawn(row(10.0)).with_children(|r| {
                            icon(r, hud, art, 38.0);
                            r.spawn(column(1.0)).with_children(|c| {
                                label(c, hud, title, 19.0, Face::Bold, SAND);
                                label(c, hud, input, 16.0, Face::Body, MUTED);
                            });
                        });
                    }
                    back(p, hud, "Zur Pause");
                },
            );
        }
        Screen::Finish => finish(root, hud, a, island, level),
    }
}

fn atlas(root: &mut ChildSpawnerCommands, hud: &Hud, a: &Adventure) {
    let level = Level::at(a.selected);
    let region = level.region;
    let start = region as usize * CORE_HARBORS;
    let complete = (start..start + CORE_HARBORS)
        .filter(|&index| a.completed(index))
        .count();
    let cover = format!("adventure_cover_{}", region as usize);
    let mut node = frame_node(Val::Percent(94.0), 12.0, 20.0);
    node.max_width = Val::Px(1110.0);
    overlay(root).with_children(|overlay| {
        card(overlay, node, INK).with_children(|p| {
            p.spawn(row(12.0)).with_children(|r| {
                icon(r, hud, "adventure", 58.0);
                r.spawn(column(0.0)).with_children(|c| {
                    label(c, hud, "Abenteuerkampagne", 44.0, Face::Serif, SAND);
                    label(
                        c,
                        hud,
                        "Sammeln, bauen und eine Insel zum Leben erwecken.",
                        18.0,
                        Face::Body,
                        MUTED,
                    );
                });
            });
            p.spawn(row(8.0)).with_children(|r| {
                for area in Region::ALL {
                    action(
                        r,
                        hud,
                        Command::Area(area),
                        area.name(),
                        area.icon(),
                        "",
                        if area == region {
                            Style::Selected
                        } else {
                            Style::Quiet
                        },
                        false,
                    );
                }
            });
            p.spawn(Node {
                align_items: AlignItems::Stretch,
                ..row(12.0)
            })
            .with_children(|r| {
                r.spawn(Node {
                    flex_grow: 1.0,
                    ..column(7.0)
                })
                .with_children(|list| {
                    for pair in (start..start + CORE_HARBORS).collect::<Vec<_>>().chunks(3) {
                        list.spawn(row(6.0)).with_children(|line| {
                            for &index in pair {
                                let l = Level::at(index);
                                button_shell(
                                    line,
                                    Action::Adventure(Command::Choose(index)),
                                    if index == a.selected {
                                        Style::Selected
                                    } else {
                                        Style::Quiet
                                    },
                                    false,
                                    Node {
                                        width: Val::Percent(32.5),
                                        min_height: Val::Px(49.0),
                                        padding: UiRect::all(Val::Px(5.0)),
                                        ..row(4.0)
                                    },
                                )
                                .with_children(|tile| {
                                    let state = if a.completed(index) {
                                        "check"
                                    } else if a.unlocked(index) {
                                        "adventure_hill"
                                    } else {
                                        "lock"
                                    };
                                    icon(tile, hud, state, 23.0);
                                    label(
                                        tile,
                                        hud,
                                        l.name,
                                        16.0,
                                        Face::Bold,
                                        if a.unlocked(index) { SAND } else { MUTED },
                                    );
                                });
                            }
                        });
                    }
                });
                let mut detail = frame_node(Val::Px(292.0), 4.0, 8.0);
                detail.flex_shrink = 0.0;
                card(r, detail, DEEP).with_children(|c| {
                    picture(c, hud, &cover, Val::Percent(100.0), Val::Px(103.0));
                    label(c, hud, level.name, 25.0, Face::Serif, SAND);
                    label(c, hud, level.theme, 16.0, Face::Body, MUTED);
                    c.spawn(row(13.0)).with_children(|m| {
                        panels::metric(m, hud, "hammer", format!("{} Bauplätze", level.plots()));
                        panels::metric(m, hud, "check", format!("{complete} / {CORE_HARBORS}"));
                    });
                    if !a.unlocked(level.index) {
                        label(
                            c,
                            hud,
                            format!("Nach „{}“ verfügbar", Level::at(level.index - 1).name),
                            14.0,
                            Face::Bold,
                            GOLD,
                        );
                    }
                });
            });
            p.spawn(row(9.0)).with_children(|r| {
                let launch = if a.completed(a.selected) {
                    "Siedlung besuchen"
                } else if a.continuing(a.selected) {
                    "Fortsetzen"
                } else {
                    "Insel beginnen"
                };
                action(
                    r,
                    hud,
                    Command::Start,
                    launch,
                    "ferry",
                    "A",
                    Style::Primary,
                    !a.unlocked(a.selected),
                );
                action(
                    r,
                    hud,
                    Command::Tutorial,
                    if a.completed(0) {
                        "Einführung wiederholen"
                    } else {
                        "Einführung spielen"
                    },
                    "guide",
                    "",
                    Style::Quiet,
                    false,
                );
                button(
                    r,
                    hud,
                    Action::Wardrobe,
                    "Garderobe",
                    "nili",
                    "",
                    Style::Quiet,
                    false,
                );
                action(
                    r,
                    hud,
                    Command::Home,
                    "Hauptmenü",
                    "",
                    "B",
                    Style::Quiet,
                    false,
                );
            });
        });
    });
}

fn walk(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    if hud.phone {
        phone::expedition(root, hud, game, a, island, level);
    } else {
        let mut status = frame_node(Val::Auto, 5.0, 10.0);
        status.position_type = PositionType::Absolute;
        status.left = Val::Px(16.0);
        status.top = Val::Px(14.0);
        card(root, status, INK).with_children(|p| {
            p.spawn(row(10.0)).with_children(|r| {
                icon(r, hud, level.region.icon(), 30.0);
                label(r, hud, level.name, 25.0, Face::Serif, SAND);
                let light = if island.phase < 0.18 || island.phase > 0.82 {
                    "moon"
                } else {
                    "sun"
                };
                icon(r, hud, light, 24.0);
                panels::metric(r, hud, "adventure_labor", free_staff(island));
            });
            p.spawn(row(11.0)).with_children(|r| {
                for good in [Good::Wood, Good::Stone, Good::Food, Good::Boards] {
                    stock(r, hud, good, island.amount(good), level.region);
                }
                if level.stage >= 1 {
                    for good in [Good::Clay, Good::Water, Good::Bricks] {
                        stock(r, hud, good, island.amount(good), level.region);
                    }
                }
            });
        });
        let mut tasks = frame_node(Val::Px(296.0), 6.0, 10.0);
        tasks.position_type = PositionType::Absolute;
        tasks.right = Val::Px(16.0);
        tasks.top = Val::Px(14.0);
        card(root, tasks, INK).with_children(|c| {
            let (command, label, art) = if island.completed {
                (Command::Summary, "Siedlungsbilanz", "check")
            } else {
                (Command::Inspect(Target::Hall), "Aufträge", "book")
            };
            action(c, hud, command, label, art, "RB", Style::Quiet, false);
            if !island.completed {
                objectives(c, hud, level, island);
            }
        });
    }
    if let Some((art, hint, target)) = a.tutorial() {
        let mut tip = frame_node(Val::Px(if hud.phone { 352.0 } else { 404.0 }), 5.0, 10.0);
        tip.position_type = PositionType::Absolute;
        tip.left = Val::Px(16.0);
        tip.top = Val::Px(if hud.phone { 73.0 } else { 116.0 });
        card(root, tip, INK).with_children(|c| {
            c.spawn(row(9.0)).with_children(|r| {
                icon(r, hud, art, 38.0);
                r.spawn(column(1.0)).with_children(|c| {
                    label(c, hud, hint, 18.0, Face::Bold, SAND);
                    if !hud.phone
                        && island.lots.iter().all(Option::is_none)
                        && island.amount(Good::Wood) == 0
                    {
                        label(
                            c,
                            hud,
                            "Bewegen mit Stick oder Steuerkreuz",
                            15.0,
                            Face::Body,
                            MUTED,
                        );
                    }
                });
            });
            if let Some(waypoint) = a.waypoint {
                route_status(c, hud, a, waypoint);
            } else if !a.close_enough(target) {
                action(
                    c,
                    hud,
                    Command::SetRoute(target),
                    "Weg anzeigen",
                    "adventure_hill",
                    "View",
                    Style::Quiet,
                    false,
                );
            }
        });
    } else if let Some(target) = a.waypoint {
        let mut marker = frame_node(Val::Auto, 0.0, 8.0);
        marker.position_type = PositionType::Absolute;
        marker.left = Val::Px(16.0);
        marker.top = Val::Px(if hud.phone { 73.0 } else { 116.0 });
        card(root, marker, INK).with_children(|c| {
            route_status(c, hud, a, target);
        });
    }
    if hud.phone {
        phone::pad(root, hud, !game.phone_walked);
    } else {
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                bottom: Val::Px(14.0),
                ..default()
            },
            FocusPolicy::Pass,
        ))
        .with_children(|foot| steering::pad(foot, hud));
    }
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(14.0),
            align_items: AlignItems::FlexEnd,
            ..column(8.0)
        },
        FocusPolicy::Pass,
    ))
    .with_children(|corner| {
        if let Some(load) = island.cargo {
            let node = frame_node(Val::Auto, 0.0, 10.0);
            card(corner, node, INK).with_children(|chip| {
                chip.spawn(row(8.0)).with_children(|line| {
                    icon(line, hud, load.kind.icon(level.region), 27.0);
                    label(
                        line,
                        hud,
                        format!(
                            "{} {}/{}",
                            load.kind.name(level.region),
                            load.units,
                            NILI_CAPACITY
                        ),
                        18.0,
                        Face::Bold,
                        SAND,
                    );
                    let at_site = matches!(a.waypoint, Some(Target::Plot(lot)) if island.site(lot).is_some())
                        || island.building_load.is_some();
                    label(line, hud, if at_site { "→ Baustelle" } else { "→ Rathaus" }, 17.0, Face::Bold, GOLD);
                });
            });
        } else if island.shipment.is_some() {
            let node = frame_node(Val::Auto, 0.0, 10.0);
            card(corner, node, INK).with_children(|chip| {
                panels::metric(
                    chip,
                    hud,
                    "storehouse",
                    format!("Fracht → {}", level.outpost()),
                );
            });
        }
        if let Some(harvest) = a.harvesting {
            let kind = a.terrain.as_ref().unwrap().deposits[harvest.site].kind;
            let node = frame_node(Val::Px(258.0), 6.0, 10.0);
            card(corner, node, INK).with_children(|chip| {
                panels::metric(chip, hud, kind.icon(level.region), "Nili arbeitet …");
                chip.spawn((
                    Node {
                        height: Val::Px(6.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(CARD),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        HarvestProgress,
                        Node {
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(GOLD),
                    ));
                });
            });
        } else if let Some(target) = a.target {
            corner.spawn(row(8.0)).with_children(|r| {
                if matches!(target, Target::Deposit(_)) {
                    action(
                        r,
                        hud,
                        Command::Inspect(target),
                        "Details",
                        "adventure_labor",
                        "",
                        Style::Quiet,
                        false,
                    );
                } else if target == Target::Hall {
                    action(
                        r,
                        hud,
                        Command::Storage,
                        "Lager",
                        "storehouse",
                        "",
                        Style::Quiet,
                        false,
                    );
                    if level.order(island.orders_delivered).is_some() {
                        action(r, hud, Command::Deliveries, "Lieferungen", "ferry", "", Style::Quiet, false);
                    }
                    if a.delivery_action() == Some(Command::PackOrder) && let Some(lot) = a.supply_lot() {
                        action(r, hud, Command::TakeMaterial(lot), "Baumaterial", "hammer", "", Style::Quiet, false);
                    }
                } else if let Target::Plot(lot) = target && island.site(lot).is_some() {
                    action(r, hud, Command::Inspect(target), "Baustelle", "hammer", "R3", Style::Quiet, false);
                }
                let mut disabled = false;
                let (label, art) = match target {
                    Target::Deposit(id) => {
                        let terrain = a.terrain.as_ref().unwrap();
                        let kind = terrain.deposits[id].kind;
                        disabled = island.remaining(terrain, id) == 0 || island.room_for(kind) == 0;
                        (kind.gather_action(), kind.icon(level.region))
                    }
                    Target::Plot(lot) if island.site(lot).is_some() => {
                        let site = island.site(lot).unwrap();
                        if island.cargo.is_some_and(|load| site.missing(load.kind) > 0) {
                            ("Material ablegen", "storehouse")
                        } else if a.building == Some(lot) {
                            ("Baupause", "pause")
                        } else if site.ready() {
                            ("Mitbauen", "hammer")
                        } else {
                            ("Baustelle ansehen", "hammer")
                        }
                    }
                    Target::Plot(lot) if island.lots[lot].is_none() => ("Bauen", "hammer"),
                    Target::Plot(_) => ("Gebäude ansehen", "house"),
                    Target::Hall if island.cargo.is_some() => ("Fracht abladen", "storehouse"),
                    Target::Hall if a.delivery_action() == Some(Command::PackOrder) => {
                        ("Fracht packen", "ferry")
                    }
                    Target::Hall if a.supply_lot().is_some() => ("Baumaterial holen", "hammer"),
                    Target::Hall => ("Rathaus ansehen", "book"),
                    Target::Outpost if island.shipment.is_some() => {
                        ("Lieferung abgeben", "storehouse")
                    }
                    Target::Outpost => ("Lieferaufträge", "book"),
                    Target::Shrine if island.quest == Quest::Unheard => {
                        ("Auftrag annehmen", "adventure_blueprint")
                    }
                    Target::Shrine if island.quest == Quest::Carrying => {
                        ("Fundstück abgeben", "adventure_blueprint")
                    }
                    Target::Shrine if island.quest == Quest::Offering => {
                        ("Lieferung ansehen", "adventure_craft")
                    }
                    Target::Relic => ("Fundstück nehmen", "adventure_blueprint"),
                    Target::Shrine => ("Auftrag ansehen", "book"),
                };
                action(
                    r,
                    hud,
                    Command::Use(target),
                    label,
                    art,
                    "A",
                    Style::Primary,
                    disabled,
                );
            });
        }
        if !hud.phone {
            corner.spawn(row(7.0)).with_children(|r| {
                action(
                    r,
                    hud,
                    Command::Map,
                    "Karte",
                    "chart",
                    "X",
                    Style::Quiet,
                    false,
                );
                action(
                    r,
                    hud,
                    Command::Labor,
                    "Bürger",
                    "adventure_labor",
                    "Y",
                    Style::Quiet,
                    false,
                );
                action(
                    r,
                    hud,
                    Command::Pause,
                    "Pause",
                    "pause",
                    "",
                    Style::Quiet,
                    false,
                );
            });
        }
    });
    if let Some(target) = a.target {
        let mut node = frame_node(Val::Auto, 0.0, 8.0);
        node.position_type = PositionType::Absolute;
        card(root, node, INK)
            .insert(WorldTip(target))
            .with_children(|c| {
                c.spawn(row(8.0)).with_children(|r| {
                    icon(r, hud, target_icon(target, a), 24.0);
                    label(r, hud, target_name(target, a), 17.0, Face::Bold, SAND);
                    if let Target::Deposit(id) = target {
                        let remaining = island.remaining(a.terrain.as_ref().unwrap(), id);
                        label(r, hud, remaining.to_string(), 17.0, Face::Bold, GOLD);
                    } else if target == Target::Hall {
                        label(
                            r,
                            hud,
                            format!("{}/{}", island.raw_stock(), island.storage_capacity()),
                            17.0,
                            Face::Bold,
                            if island.free_storage() == 0 {
                                WARN
                            } else {
                                GOLD
                            },
                        );
                    } else if let Target::Plot(lot) = target
                        && let Some(site) = island.site(lot)
                    {
                        let received = site.materials.iter().sum::<u16>();
                        let needed = site.needed.iter().sum::<u16>();
                        if site.ready() {
                            construction_bar(r, lot, site.progress(), 84.0);
                        } else {
                            label(
                                r,
                                hud,
                                format!("{received}/{needed}"),
                                17.0,
                                Face::Bold,
                                GOLD,
                            );
                        }
                    }
                    if let Target::Plot(lot) = target
                        && let Some(property) = island.lots[lot]
                        && let Some(recipe) = property.kind.recipe(property.tier, level.region)
                        && let Some((good, need)) = recipe
                            .need
                            .into_iter()
                            .find(|&(good, need)| island.amount(good) < need)
                    {
                        icon(r, hud, good.icon(level.region), 21.0);
                        label(
                            r,
                            hud,
                            format!("{}/{}", island.amount(good), need),
                            16.0,
                            Face::Bold,
                            WARN,
                        );
                    }
                });
            });
    }
    root.spawn((
        PlayerPin,
        Visibility::Hidden,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(GOLD),
        BorderRadius::MAX,
        FocusPolicy::Pass,
    ))
    .with_children(|pin| icon(pin, hud, "nili", 26.0));
    if a.waypoint
        .or_else(|| a.tutorial().map(|item| item.2))
        .is_some()
    {
        let mut node = frame_node(Val::Auto, 0.0, 7.0);
        node.position_type = PositionType::Absolute;
        card(root, node, INK)
            .insert((RoutePointer, Visibility::Hidden))
            .with_children(|c| {
                c.spawn(row(5.0)).with_children(|r| {
                    icon(r, hud, "adventure_hill", 24.0);
                    r.spawn((
                        RouteArrow,
                        Text::new("↑"),
                        TextFont {
                            font: hud.fonts[1].clone(),
                            font_size: 23.0,
                            ..default()
                        },
                        TextColor(GOLD),
                    ));
                });
            });
    }
}

fn route_status(p: &mut ChildSpawnerCommands, hud: &Hud, a: &Adventure, target: Target) {
    p.spawn(row(8.0)).with_children(|r| {
        icon(r, hud, target_icon(target, a), 24.0);
        r.spawn(Node {
            flex_grow: 1.0,
            min_width: Val::Px(0.0),
            ..column(0.0)
        })
        .with_children(|copy| {
            label(copy, hud, target_name(target, a), 18.0, Face::Bold, SAND);
            copy.spawn((
                RouteDistance,
                Text::new(route_distance(a, target)),
                TextFont {
                    font: hud.fonts[1].clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(144, 255, 212)),
            ));
        });
        action(
            r,
            hud,
            Command::ClearRoute,
            "Ausblenden",
            "",
            "View",
            Style::Quiet,
            false,
        );
    });
}

fn route_distance(a: &Adventure, target: Target) -> String {
    if a.close_enough(target) {
        return "In Reichweite".to_owned();
    }
    let Some((at, island)) = a.target_position(target).zip(a.active()) else {
        return String::new();
    };
    let meters = at.distance(Vec2::from_array(island.position)).round() as u32;
    format!("{meters} m bis zum Ziel")
}

fn construction_panel(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
    lot: usize,
    site: &BuildingSite,
) {
    let ready = site.ready();
    let held = island.cargo.is_some_and(|load| site.missing(load.kind) > 0);
    panels::modal(
        root,
        hud,
        720.0,
        &format!("Baustelle · Stufe {}", site.property.tier),
        site.property.kind.name(level.region),
        build_icon(site.property.kind),
        |p| {
            p.spawn(Node {
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(7.0),
                ..row(8.0)
            })
            .with_children(|r| {
                for good in Good::ALL
                    .into_iter()
                    .filter(|&good| site.needed[good as usize] > 0)
                {
                    let i = good as usize;
                    card(r, frame_node(Val::Auto, 3.0, 8.0), CARD).with_children(|card| {
                        card.spawn(row(7.0)).with_children(|chip| {
                            icon(chip, hud, good.icon(level.region), 26.0);
                            label(
                                chip,
                                hud,
                                format!("{} / {}", site.materials[i], site.needed[i]),
                                21.0,
                                Face::Bold,
                                if site.missing(good) == 0 { GREEN } else { GOLD },
                            );
                        });
                        label(card, hud, good.name(level.region), 16.0, Face::Body, SAND);
                    });
                }
            });
            if ready {
                panels::metric(p, hud, "check", "Baumaterial vollständig");
                construction_bar(p, lot, site.progress(), 320.0);
            } else {
                label(
                    p,
                    hud,
                    "Freie Bürger holen Baumaterial am Rathaus.",
                    18.0,
                    Face::Body,
                    MUTED,
                );
            }
            p.spawn(row(8.0)).with_children(|r| {
                action(
                    r,
                    hud,
                    Command::BuildCrew(lot, -1),
                    "−",
                    "",
                    "",
                    Style::Quiet,
                    site.crew == 0,
                );
                panels::metric(
                    r,
                    hud,
                    "adventure_labor",
                    format!("Bautrupp {}/{}", island.site_workers(lot), site.crew),
                );
                action(
                    r,
                    hud,
                    Command::BuildCrew(lot, 1),
                    "+",
                    "",
                    "",
                    Style::Quiet,
                    site.crew >= MAX_BUILD_CREW,
                );
            });
            p.spawn(Node {
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(7.0),
                ..row(8.0)
            })
            .with_children(|r| {
                if held || ready {
                    action(
                        r,
                        hud,
                        Command::Use(Target::Plot(lot)),
                        if held { "Material ablegen" } else { "Mitbauen" },
                        "hammer",
                        "A",
                        Style::Primary,
                        !a.close_enough(Target::Plot(lot)),
                    );
                } else if island.cargo.is_none()
                    && island.shipment.is_none()
                    && island.site_supply(lot).is_some()
                {
                    action(
                        r,
                        hud,
                        Command::TakeMaterial(lot),
                        "Baumaterial holen",
                        "storehouse",
                        "",
                        Style::Primary,
                        false,
                    );
                }
                back(r, hud, "Zur Insel");
                action(
                    r,
                    hud,
                    Command::CancelBuild(lot),
                    "Plan aufheben",
                    "",
                    "",
                    Style::Quiet,
                    false,
                );
            });
        },
    );
}

fn construction_bar(p: &mut ChildSpawnerCommands, lot: usize, progress: f32, width: f32) {
    p.spawn((
        Node {
            width: Val::Px(width),
            height: Val::Px(7.0),
            ..default()
        },
        BackgroundColor(JADE),
        BorderRadius::MAX,
    ))
    .with_children(|track| {
        track.spawn((
            WorkProgress::Construction(lot),
            Node {
                width: Val::Percent(progress * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(GOLD),
            BorderRadius::MAX,
        ));
    });
}

fn build_menu(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    let Screen::Lot(lot) = a.screen else {
        return;
    };
    panels::modal(
        root,
        hud,
        924.0,
        level.name,
        "Was möchtest du bauen?",
        "hammer",
        |p| {
            p.spawn(row(8.0)).with_children(|r| {
                for (id, name, art) in [
                    (0, "Grundversorgung", "house"),
                    (1, "Werkstätten und Läden", "adventure_tools"),
                    (2, "Gemeinschaft", "shrine"),
                ] {
                    action(
                        r,
                        hud,
                        Command::Group(id),
                        name,
                        art,
                        "",
                        if a.group == id {
                            Style::Selected
                        } else {
                            Style::Quiet
                        },
                        false,
                    );
                }
            });
            let plans: Vec<_> = Build::all(level)
                .into_iter()
                .filter(|plan| plan.page() == a.group)
                .collect();
            for chunk in plans
                .iter()
                .skip(usize::from(a.page) * 6)
                .take(6)
                .copied()
                .collect::<Vec<_>>()
                .chunks(3)
            {
                p.spawn(Node {
                    align_items: AlignItems::Stretch,
                    ..row(7.0)
                })
                .with_children(|r| {
                    for &kind in chunk {
                        let costs = kind.cost(1);
                        let locked = island.locked(level, kind, 1);
                        let fits = kind.fits(level, lot);
                        let required = level.goals().any(|goal| goal.kind == kind);
                        button_shell(
                            r,
                            Action::Adventure(Command::Build(kind)),
                            if required {
                                Style::Selected
                            } else {
                                Style::Quiet
                            },
                            locked || !fits,
                            Node {
                                width: Val::Percent(32.7),
                                min_height: Val::Px(97.0),
                                padding: UiRect::all(Val::Px(9.0)),
                                ..column(5.0)
                            },
                        )
                        .with_children(|tile| {
                            tile.spawn(row(7.0)).with_children(|head| {
                                icon(head, hud, build_icon(kind), 35.0);
                                label(head, hud, kind.name(level.region), 19.0, Face::Bold, SAND);
                                if required {
                                    icon(head, hud, "star", 19.0);
                                }
                            });
                            if !fits {
                                label(
                                    tile,
                                    hud,
                                    "Braucht einen Küstenbauplatz",
                                    15.0,
                                    Face::Bold,
                                    MUTED,
                                );
                            } else if locked {
                                let message = if level.plan() == Some(kind) {
                                    "Bauplan aus dem Inselauftrag"
                                } else {
                                    "Auf einer späteren Insel verfügbar"
                                };
                                label(tile, hud, message, 15.0, Face::Bold, MUTED);
                            } else {
                                costs_row(tile, hud, level.region, island, &costs);
                                if kind == Build::Harbor(Kind::Storehouse) {
                                    label(
                                        tile,
                                        hud,
                                        format!(
                                            "+{} Rohstoffplätze",
                                            Island::warehouse_capacity(1)
                                        ),
                                        15.0,
                                        Face::Bold,
                                        GOLD,
                                    );
                                }
                            }
                        });
                    }
                });
            }
            p.spawn(row(8.0)).with_children(|r| {
                back(r, hud, "Zur Insel");
                paging(r, hud, a.page, plans.len(), 6);
                label(
                    r,
                    hud,
                    "Goldener Stern = Inselziel",
                    16.0,
                    Face::Body,
                    MUTED,
                );
            });
        },
    );
}

fn property_panel(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    island: &Island,
    terrain: &Terrain,
    lot: usize,
    property: crate::adventure::Property,
) {
    let level = terrain.level;
    panels::modal(
        root,
        hud,
        730.0,
        &format!("STUFE {} / {}", property.tier, MAX_LEVEL),
        property.kind.name(level.region),
        build_icon(property.kind),
        |p| {
            if let Some(recipe) = property.kind.recipe(property.tier, level.region) {
                let fishing = recipe.output.0 == Good::Fish;
                let staffed = island.assignment(Task::Produce(lot));
                p.spawn(row(12.0)).with_children(|r| {
                    if recipe.need.is_empty() {
                        label(r, hud, "Ohne Verbrauch", 17.0, Face::Body, MUTED);
                    } else {
                        costs_row(r, hud, level.region, island, &recipe.need);
                    }
                    label(r, hud, "→", 23.0, Face::Bold, GOLD);
                    stock(r, hud, recipe.output.0, recipe.output.1, level.region);
                    label(
                        r,
                        hud,
                        recipe.output.0.name(level.region),
                        17.0,
                        Face::Bold,
                        SAND,
                    );
                });
                if let Some(batch) = island.batch(lot) {
                    panels::metric(
                        p,
                        hud,
                        if fishing { "fishery" } else { "guest_wait" },
                        if batch.left == 0.0 {
                            "Fertig · wartet auf Lagerplatz"
                        } else if fishing {
                            "Das Fischerboot ist unterwegs."
                        } else {
                            "Die Produktion läuft."
                        },
                    );
                    p.spawn((
                        Node {
                            height: Val::Px(7.0),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(CARD),
                    ))
                    .with_children(|bar| {
                        bar.spawn((
                            WorkProgress::Production(lot),
                            Node {
                                width: Val::Percent(batch.progress() * 100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(GOLD),
                        ));
                    });
                } else if fishing && staffed == 0 {
                    panels::metric(
                        p,
                        hud,
                        "adventure_labor",
                        "Weise der Fischerhütte einen Bürger zu.",
                    );
                } else if fishing && !island.fish_available(terrain, lot) {
                    panels::metric(p, hud, "fishery", "Die Fischgründe sind erschöpft.");
                } else if !island.ready(&recipe.need) {
                    label(
                        p,
                        hud,
                        "Es fehlen Vorräte. Rot markierte Zutaten zuerst beschaffen.",
                        17.0,
                        Face::Bold,
                        WARN,
                    );
                } else if !island.room_for_recipe(&recipe) {
                    label(
                        p,
                        hud,
                        "Das Rohstofflager ist voll.",
                        17.0,
                        Face::Bold,
                        WARN,
                    );
                } else {
                    let seconds = recipe.seconds / (1.0 + 0.48 * staffed.saturating_sub(1) as f32);
                    panels::metric(
                        p,
                        hud,
                        "guest_wait",
                        if fishing {
                            "Fahrzeit hängt vom Fanggebiet ab.".to_owned()
                        } else {
                            format!("Rund {:.0} s je Arbeitsgang", seconds)
                        },
                    );
                }
                team(p, hud, island, Task::Produce(lot));
                if !fishing {
                    action(
                        p,
                        hud,
                        Command::Work(lot),
                        "Arbeitsgang starten",
                        "adventure_tools",
                        "",
                        Style::Quiet,
                        island.batch(lot).is_some()
                            || staffed > 0
                            || !island.ready(&recipe.need)
                            || !island.room_for_recipe(&recipe),
                    );
                }
            } else if property.kind == Build::Harbor(Kind::Storehouse) {
                panels::metric(
                    p,
                    hud,
                    "storehouse",
                    format!(
                        "{} Rohstoffplätze in diesem Speicher",
                        Island::warehouse_capacity(property.tier)
                    ),
                );
                if property.tier < MAX_LEVEL {
                    label(
                        p,
                        hud,
                        format!(
                            "Nach dem Ausbau {} Plätze",
                            Island::warehouse_capacity(property.tier + 1)
                        ),
                        17.0,
                        Face::Body,
                        MUTED,
                    );
                }
                action(
                    p,
                    hud,
                    Command::Storage,
                    "Rohstofflager öffnen",
                    "storehouse",
                    "",
                    Style::Quiet,
                    false,
                );
            } else if let Build::Harbor(kind) = property.kind {
                let residents = crate::model::Building::new(kind).residents();
                if residents > 0 {
                    panels::metric(
                        p,
                        hud,
                        "adventure_labor",
                        format!(
                            "{} Bürger können hier arbeiten",
                            residents - 1 + u16::from(property.tier - 1)
                        ),
                    );
                } else {
                    label(
                        p,
                        hud,
                        "Dieses Gebäude zählt zu deiner Siedlung.",
                        17.0,
                        Face::Body,
                        MUTED,
                    );
                }
            }
            if property.tier < MAX_LEVEL {
                line(p, LINE);
                let target = property.tier + 1;
                let costs = property.kind.cost(target);
                let locked = island.locked(level, property.kind, target);
                p.spawn(row(11.0)).with_children(|r| {
                    action(
                        r,
                        hud,
                        Command::Build(property.kind),
                        &format!("Stufe {target} planen"),
                        "hammer",
                        "",
                        Style::Primary,
                        locked,
                    );
                    if locked {
                        label(r, hud, "Bauplan fehlt", 18.0, Face::Bold, WARN);
                    } else {
                        costs_row(r, hud, level.region, island, &costs);
                    }
                });
            }
            p.spawn(row(8.0)).with_children(|r| {
                back(r, hud, "Zur Insel");
                action(
                    r,
                    hud,
                    Command::Demolish,
                    "Abreißen …",
                    "",
                    "",
                    Style::Quiet,
                    false,
                );
            });
        },
    );
}

fn labor(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    terrain: &Terrain,
    island: &Island,
) {
    let level = terrain.level;
    let collection = a.group == 0;
    panels::modal(
        root,
        hud,
        785.0,
        &free_staff(island),
        "Bürgeraufträge",
        "adventure_labor",
        |p| {
            if island.staff() == 0 {
                label(
                    p,
                    hud,
                    "Baue ein Wohnhaus, damit Bürger einziehen und mitarbeiten können.",
                    19.0,
                    Face::Bold,
                    GOLD,
                );
            }
            p.spawn(row(8.0)).with_children(|r| {
                action(
                    r,
                    hud,
                    Command::Group(0),
                    "Rohstoffe sammeln",
                    "adventure_wood",
                    "",
                    if collection {
                        Style::Selected
                    } else {
                        Style::Quiet
                    },
                    false,
                );
                action(
                    r,
                    hud,
                    Command::Group(1),
                    "Gebäude betreiben",
                    "hammer",
                    "",
                    if collection {
                        Style::Quiet
                    } else {
                        Style::Selected
                    },
                    false,
                );
                if level.order(island.orders_delivered).is_some() || island.courier.is_some() {
                    action(
                        r,
                        hud,
                        Command::Deliveries,
                        "Lieferungen",
                        "ferry",
                        "",
                        Style::Quiet,
                        false,
                    );
                }
            });
            if let Some(courier) = &island.courier {
                panels::metric(
                    p,
                    hud,
                    "ferry",
                    if courier.order.is_some() {
                        "Ein Bürger bringt die Fracht zum Außenposten."
                    } else {
                        "Der Bürger kehrt zum Rathaus zurück."
                    },
                );
            }
            let jobs: Vec<_> = if collection {
                Good::RAW
                    .into_iter()
                    .flat_map(|good| {
                        let mut places = terrain
                            .deposits
                            .iter()
                            .enumerate()
                            .filter(move |(_, deposit)| deposit.kind == good);
                        places
                            .next()
                            .into_iter()
                            .chain(
                                places.filter(|(id, _)| island.assignment(Task::Gather(*id)) > 0),
                            )
                            .map(move |(id, _)| {
                                (
                                    Task::Gather(id),
                                    good.name(level.region),
                                    good.icon(level.region),
                                )
                            })
                    })
                    .collect()
            } else {
                island
                    .lots
                    .iter()
                    .enumerate()
                    .filter_map(|(lot, property)| {
                        let property = (*property)?;
                        property.kind.recipe(property.tier, level.region).map(|_| {
                            (
                                Task::Produce(lot),
                                property.kind.name(level.region),
                                build_icon(property.kind),
                            )
                        })
                    })
                    .collect()
            };
            if jobs.is_empty() {
                label(
                    p,
                    hud,
                    "Noch keine Werkstatt gebaut.",
                    18.0,
                    Face::Body,
                    MUTED,
                );
            }
            for &(task, name, art) in jobs.iter().skip(usize::from(a.page) * 6).take(6) {
                p.spawn(row(9.0)).with_children(|r| {
                    r.spawn(Node {
                        flex_grow: 1.0,
                        ..row(7.0)
                    })
                    .with_children(|entry| {
                        icon(entry, hud, art, 27.0);
                        label(entry, hud, name, 18.0, Face::Bold, SAND);
                        if let Task::Gather(id) = task {
                            let returning = island
                                .workers
                                .iter()
                                .zip(&island.hauls)
                                .filter(|(job, haul)| {
                                    **job == Task::Gather(id) && haul.cargo.is_some()
                                })
                                .count();
                            if returning > 0 && island.free_storage() == 0 {
                                label(entry, hud, "Lager voll", 15.0, Face::Bold, WARN);
                            } else if returning > 0 {
                                label(
                                    entry,
                                    hud,
                                    format!("{returning} zum Rathaus"),
                                    15.0,
                                    Face::Body,
                                    GOLD,
                                );
                            } else if island.assignment(task) > 0 {
                                label(entry, hud, "Sammeln und liefern", 15.0, Face::Body, MUTED);
                            }
                        }
                        if let Task::Produce(lot) = task
                            && let Some(recipe) = island.lots[lot].and_then(|property| {
                                property.kind.recipe(property.tier, level.region)
                            })
                            && !island.ready(&recipe.need)
                        {
                            label(entry, hud, "Vorräte fehlen", 16.0, Face::Body, WARN);
                        }
                    });
                    let target = match task {
                        Task::Gather(id) => island
                            .available_near(
                                terrain,
                                terrain.deposits[id].kind,
                                terrain.deposits[id].at,
                            )
                            .map(Target::Deposit),
                        Task::Produce(lot) => Some(Target::Plot(lot)),
                    };
                    action(
                        r,
                        hud,
                        Command::SetRoute(target.unwrap_or(Target::Hall)),
                        "Ort",
                        "chart",
                        "",
                        Style::Quiet,
                        target.is_none(),
                    );
                    assignment(r, hud, island, task);
                });
            }
            p.spawn(row(8.0)).with_children(|r| {
                back(r, hud, "Zur Insel");
                paging(r, hud, a.page, jobs.len(), 6);
            });
        },
    );
}

fn map(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    terrain: &Terrain,
    island: &Island,
) {
    let level = terrain.level;
    panels::modal(
        root,
        hud,
        846.0,
        level.region.name(),
        level.name,
        "chart",
        |p| {
            p.spawn(Node {
                align_items: AlignItems::Stretch,
                ..row(16.0)
            })
            .with_children(|r| {
                r.spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(400.0),
                        flex_shrink: 0.0,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.083, 0.335, 0.38)),
                    BorderRadius::all(Val::Px(16.0)),
                ))
                .with_children(|map| {
                    if let Some((_, image)) = &hud.adventure_map {
                        map.spawn((
                            ImageNode::new(image.clone()),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            FocusPolicy::Pass,
                            BorderRadius::all(Val::Px(16.0)),
                        ));
                    }
                    for at in terrain.plots.iter().copied() {
                        let mut node = pin_node(at, terrain, 5.0);
                        node.height = Val::Px(5.0);
                        map.spawn((
                            node,
                            BackgroundColor(PAPER),
                            BorderRadius::all(Val::Px(2.0)),
                        ));
                    }
                    let mut pins = vec![(Target::Hall, Vec2::ZERO)];
                    pins.extend(
                        terrain
                            .fishing
                            .iter()
                            .map(|place| (Target::Plot(place.lot), terrain.plots[place.lot])),
                    );
                    for (lot, site) in island.construction.iter().enumerate() {
                        if site.is_some() && !level.coastal(lot) {
                            pins.push((Target::Plot(lot), terrain.plots[lot]));
                        }
                    }
                    if let Some(Target::Plot(lot)) = a.target
                        && !pins.iter().any(|(target, _)| *target == Target::Plot(lot))
                    {
                        pins.push((Target::Plot(lot), terrain.plots[lot]));
                    }
                    for good in Good::RAW {
                        if let Some(id) =
                            island.available_near(terrain, good, Vec2::from_array(island.position))
                        {
                            pins.push((Target::Deposit(id), terrain.deposits[id].at));
                        }
                    }
                    if level.order_count() > 0 {
                        pins.push((Target::Outpost, terrain.outpost));
                    }
                    if level.plan().is_some() && island.quest != Quest::Finished {
                        pins.push((Target::Shrine, terrain.shrine));
                    }
                    if island.quest == Quest::Searching {
                        pins.push((Target::Relic, terrain.relic));
                    }
                    for (target, at) in pins {
                        button_shell(
                            map,
                            Action::Adventure(Command::SetRoute(target)),
                            if a.waypoint == Some(target) {
                                Style::Primary
                            } else {
                                Style::Quiet
                            },
                            false,
                            pin_node(at, terrain, 30.0),
                        )
                        .with_children(|pin| {
                            icon(pin, hud, target_icon(target, a), 22.0);
                        });
                    }
                    let mut me = pin_node(Vec2::from_array(island.position), terrain, 23.0);
                    me.margin = UiRect::top(Val::Px(-16.0));
                    map.spawn((
                        me,
                        FocusPolicy::Pass,
                        BackgroundColor(GOLD),
                        BorderRadius::all(Val::Px(11.0)),
                    ))
                    .with_children(|pin| icon(pin, hud, "nili", 22.0));
                });
                r.spawn(Node {
                    flex_grow: 1.0,
                    ..column(7.0)
                })
                .with_children(|c| {
                    label(c, hud, "Lager", 25.0, Face::Serif, SAND);
                    storage_meter(c, hud, island);
                    let goods: Vec<_> = Good::ALL
                        .into_iter()
                        .filter(|&good| good.present_in(level.region))
                        .collect();
                    for group in goods.chunks(2) {
                        c.spawn(Node {
                            justify_content: JustifyContent::SpaceBetween,
                            ..row(8.0)
                        })
                        .with_children(|r| {
                            for &good in group {
                                r.spawn(row(3.0)).with_children(|entry| {
                                    stock(entry, hud, good, island.amount(good), level.region);
                                    label(
                                        entry,
                                        hud,
                                        good.name(level.region),
                                        14.0,
                                        Face::Body,
                                        MUTED,
                                    );
                                });
                            }
                        });
                    }
                    line(c, LINE);
                    label(
                        c,
                        hud,
                        "Tippe auf ein Rohstoffsymbol, um den Weg ins Abbaugebiet zu markieren.",
                        18.0,
                        Face::Body,
                        MUTED,
                    );
                    action(
                        c,
                        hud,
                        Command::Storage,
                        "Rohstoffe verwalten",
                        "storehouse",
                        "",
                        Style::Quiet,
                        false,
                    );
                    if a.waypoint.is_some() {
                        action(
                            c,
                            hud,
                            Command::ClearRoute,
                            "Markierung löschen",
                            "",
                            "",
                            Style::Quiet,
                            false,
                        );
                    }
                    back(c, hud, "Zur Insel");
                });
            });
        },
    );
}

fn storage_meter(p: &mut ChildSpawnerCommands, hud: &Hud, island: &Island) {
    let full = island.free_storage() == 0;
    p.spawn(row(7.0)).with_children(|r| {
        icon(r, hud, "storehouse", 24.0);
        label(
            r,
            hud,
            format!(
                "{} / {} Rohstoffplätze",
                island.raw_stock(),
                island.storage_capacity()
            ),
            18.0,
            Face::Bold,
            if full { WARN } else { SAND },
        );
    });
    let fraction = (f32::from(island.raw_stock()) / f32::from(island.storage_capacity())).min(1.0);
    p.spawn((
        Node {
            height: Val::Px(6.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(CARD),
    ))
    .with_children(|bar| {
        bar.spawn((
            Node {
                width: Val::Percent(fraction * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(if full { WARN } else { GOLD }),
        ));
    });
}

fn storage(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    panels::modal(
        root,
        hud,
        684.0,
        level.hall(),
        "Rohstofflager",
        "storehouse",
        |p| {
            storage_meter(p, hud, island);
            if let Some(lot) = a.supply_lot() {
                action(
                    p,
                    hud,
                    Command::TakeMaterial(lot),
                    "Baumaterial holen",
                    "hammer",
                    "",
                    Style::Primary,
                    false,
                );
            }
            if level.order(island.orders_delivered).is_some() {
                p.spawn(row(8.0)).with_children(|r| {
                    action(
                        r,
                        hud,
                        Command::Deliveries,
                        "Lieferaufträge",
                        "ferry",
                        "",
                        Style::Quiet,
                        false,
                    );
                    if a.delivery_action() == Some(Command::PackOrder) {
                        action(
                            r,
                            hud,
                            Command::PackOrder,
                            "Fracht packen",
                            "storehouse",
                            "",
                            Style::Primary,
                            false,
                        );
                    }
                });
            }
            label(
                p,
                hud,
                format!(
                    "{HALL_STORAGE} Plätze im Rathaus · {} in Speichern",
                    island.storage_capacity() - HALL_STORAGE
                ),
                17.0,
                Face::Body,
                MUTED,
            );
            if island.free_storage() == 0 {
                label(
                    p,
                    hud,
                    "Baue oder erweitere einen Speicher, oder sortiere Vorräte aus.",
                    18.0,
                    Face::Bold,
                    WARN,
                );
            }
            for kind in Good::ALL.into_iter().filter(|&kind| {
                kind.uses_storage() && (kind != Good::Fish || level.region == Region::Nordic)
            }) {
                let count = island.amount(kind);
                p.spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row(8.0)
                })
                .with_children(|r| {
                    r.spawn(row(7.0)).with_children(|line| {
                        stock(line, hud, kind, count, level.region);
                        label(line, hud, kind.name(level.region), 18.0, Face::Bold, SAND);
                    });
                    if count > 0 {
                        action(
                            r,
                            hud,
                            Command::ClearStock(kind),
                            &format!("{} aussortieren", count.min(5)),
                            "",
                            "",
                            Style::Quiet,
                            false,
                        );
                    }
                });
            }
            back(p, hud, "Zur Insel");
        },
    );
}

pub(super) fn map_image(terrain: &Terrain) -> Image {
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    const SIZE: u32 = 286;
    let mut pixels = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    let region = terrain.level.region;
    let (grass, rock, sand) = [
        ([111, 148, 102], [186, 169, 128], [220, 186, 126]),
        ([133, 154, 107], [188, 160, 119], [217, 177, 124]),
        ([105, 156, 114], [173, 160, 136], [220, 185, 140]),
        ([103, 142, 124], [167, 184, 179], [194, 181, 143]),
        ([154, 143, 96], [198, 150, 107], [226, 181, 123]),
    ][region as usize];
    let span = terrain.radius / 0.82;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let p = (Vec2::new(x as f32, y as f32) / (SIZE - 1) as f32 * 2.0 - Vec2::ONE) * span;
            let edge = terrain.shore(p);
            let h = terrain.height(p);
            let tint = if edge < -4.0 {
                [34, 99, 115]
            } else if edge < -1.6 {
                [59, 126, 129]
            } else if edge < 0.0 {
                [82, 151, 140]
            } else if edge < 2.8 {
                sand
            } else if let Some(district) = terrain.district(p) {
                district.district_color()
            } else if h > 2.4 {
                rock
            } else {
                grass
            };
            let stroke = if edge > 0.0 && h.rem_euclid(1.3) < 0.063 {
                -15.0
            } else {
                0.0
            };
            let dapple = (p.x * 0.19).sin() * (p.y * 0.15).cos() * 5.0;
            let slope = (terrain.height(p + Vec2::new(0.9, 0.8)) - h) * 26.0;
            pixels.extend(
                tint.map(|color| (color as f32 + stroke + dapple + slope).clamp(0.0, 255.0) as u8),
            );
            pixels.push(255);
        }
    }
    Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn pin_node(at: Vec2, terrain: &Terrain, size: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: Val::Percent(50.0 + at.x / terrain.radius * 41.0 - size * 0.125),
        top: Val::Percent(50.0 + at.y / terrain.radius * 41.0 - size * 0.125),
        width: Val::Px(size),
        height: Val::Px(size),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn mission(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    panels::modal(root, hud, 692.0, level.hall(), "Aufträge", "book", |p| {
        if level.order_count() > 0 || level.plan().is_some() {
            p.spawn(row(7.0)).with_children(|tabs| {
                for (group, name, art, visible) in [
                    (0, "Bauziele", "hammer", true),
                    (1, "Lieferungen", "storehouse", level.order_count() > 0),
                    (2, "Bauplan", "adventure_blueprint", level.plan().is_some()),
                ] {
                    if visible {
                        action(
                            tabs,
                            hud,
                            Command::Group(group),
                            name,
                            art,
                            "",
                            if a.group == group {
                                Style::Selected
                            } else {
                                Style::Quiet
                            },
                            false,
                        );
                    }
                }
            });
        }
        if a.group == 1 && level.order_count() > 0 {
            shipment_panel(p, hud, a, island, level);
        } else if a.group == 2
            && let Some(plan) = level.plan()
        {
            label(
                p,
                hud,
                format!("Bauplan für {}", plan.name(level.region)),
                23.0,
                Face::Serif,
                SAND,
            );
            let (art, instruction, target, interaction) = match island.quest {
                Quest::Unheard => (
                    "shrine",
                    format!("Hol dir den Auftrag bei „{}“.", level.landmark),
                    Target::Shrine,
                    "Auftrag annehmen",
                ),
                Quest::Searching => (
                    "adventure_hill",
                    format!("Suche am Berg nach „{}“.", level.relic),
                    Target::Relic,
                    "Fundstück nehmen",
                ),
                Quest::Carrying => (
                    "adventure_blueprint",
                    format!("Bring das Fundstück zu „{}“.", level.landmark),
                    Target::Shrine,
                    "Fundstück abgeben",
                ),
                Quest::Offering => {
                    let (good, count) = level.offering().unwrap();
                    (
                        good.icon(level.region),
                        format!(
                            "Liefere {count} {} bei „{}“ ab.",
                            good.name(level.region),
                            level.landmark
                        ),
                        Target::Shrine,
                        "Waren abgeben",
                    )
                }
                Quest::Finished => (
                    "check",
                    "Der Bauplan liegt in deinem Baumenü.".to_owned(),
                    Target::Shrine,
                    "",
                ),
            };
            p.spawn(row(9.0)).with_children(|r| {
                icon(r, hud, art, 37.0);
                label(r, hud, instruction, 20.0, Face::Body, MUTED);
            });
            if island.quest != Quest::Finished {
                let offering = level.offering().filter(|_| island.quest == Quest::Offering);
                if let Some(goods) = offering {
                    costs_row(p, hud, level.region, island, &[goods]);
                }
                if a.close_enough(target) {
                    action(
                        p,
                        hud,
                        Command::Use(target),
                        interaction,
                        "adventure_blueprint",
                        "A",
                        Style::Primary,
                        offering.is_some_and(|goods| !island.ready(&[goods])),
                    );
                } else {
                    action(
                        p,
                        hud,
                        Command::SetRoute(target),
                        "Weg markieren",
                        "chart",
                        "",
                        Style::Primary,
                        false,
                    );
                }
            }
        } else {
            objectives(p, hud, level, island);
            if let Some(load) = island.cargo {
                action(
                    p,
                    hud,
                    if a.close_enough(Target::Hall) {
                        Command::Use(Target::Hall)
                    } else {
                        Command::SetRoute(Target::Hall)
                    },
                    if a.close_enough(Target::Hall) {
                        "Fracht abladen"
                    } else {
                        "Weg zum Rathaus"
                    },
                    load.kind.icon(level.region),
                    "",
                    Style::Primary,
                    false,
                );
            }
        }
        back(p, hud, "Zur Insel");
    });
}

fn shipment_panel(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    let Some(order) = level.order(island.orders_delivered) else {
        panels::metric(p, hud, "check", "Alle Lieferungen sind angekommen.");
        return;
    };
    label(p, hud, order.title, 25.0, Face::Serif, SAND);
    p.spawn(row(7.0)).with_children(|r| {
        panels::metric(
            r,
            hud,
            "storehouse",
            format!("Rathaus → {}", level.outpost()),
        );
        label(
            r,
            hud,
            format!(
                "{} / {} geliefert",
                island.orders_delivered,
                level.order_count()
            ),
            16.0,
            Face::Bold,
            GOLD,
        );
    });
    if island.shipment.is_some() {
        panels::metric(p, hud, "check", "Fracht ist gepackt.");
        let at_post = a.close_enough(Target::Outpost);
        action(
            p,
            hud,
            if at_post {
                Command::Use(Target::Outpost)
            } else {
                Command::SetRoute(Target::Outpost)
            },
            if at_post {
                "Lieferung abgeben"
            } else {
                "Weg zum Posten"
            },
            "ferry",
            "",
            Style::Primary,
            false,
        );
        if a.close_enough(Target::Hall) {
            action(
                p,
                hud,
                Command::ReturnOrder,
                "Fracht wieder einlagern",
                "storehouse",
                "",
                Style::Quiet,
                false,
            );
        }
    } else if island
        .courier
        .as_ref()
        .is_some_and(|trip| trip.order.is_some())
    {
        panels::metric(
            p,
            hud,
            "adventure_labor",
            "Ein Bürger trägt die Fracht zum Außenposten.",
        );
    } else {
        costs_row(p, hud, level.region, island, &order.goods);
        let command = a.delivery_action();
        let caption = match command {
            Some(Command::Use(Target::Hall)) => "Rohstoffe abladen",
            Some(Command::SetRoute(Target::Hall)) => "Weg zum Rathaus",
            _ => "Fracht packen",
        };
        let ready = island.order_ready(level);
        p.spawn(Node {
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(7.0),
            ..row(8.0)
        })
        .with_children(|r| {
            action(
                r,
                hud,
                command.unwrap_or(Command::PackOrder),
                caption,
                "storehouse",
                "",
                Style::Primary,
                command.is_none(),
            );
            action(
                r,
                hud,
                Command::SendCourier,
                "Bürger schicken",
                "adventure_labor",
                "",
                Style::Quiet,
                !ready || island.available_staff() == 0 || island.courier.is_some(),
            );
        });
        if island.courier.is_some() {
            label(
                p,
                hud,
                "Der Bürger kehrt gerade zurück.",
                17.0,
                Face::Body,
                MUTED,
            );
        } else if island.available_staff() == 0 && ready {
            label(
                p,
                hud,
                "Für den Transport brauchst du einen freien Bürger.",
                17.0,
                Face::Body,
                MUTED,
            );
        }
    }
}

fn finish(
    root: &mut ChildSpawnerCommands,
    hud: &Hud,
    a: &Adventure,
    island: &Island,
    level: Level,
) {
    let cover = format!("adventure_cover_{}", level.region as usize);
    let next = a.next_island();
    panels::modal(
        root,
        hud,
        775.0,
        level.name,
        if a.practice() {
            "Einführung geschafft"
        } else {
            "Deine Siedlung steht!"
        },
        "check",
        |p| {
            picture(p, hud, &cover, Val::Percent(100.0), Val::Px(138.0));
            p.spawn(row(9.0)).with_children(|r| {
                label(r, hud, "Siedlungsbilanz", 23.0, Face::Bold, SAND);
                if !a.practice() {
                    panels::metric(r, hud, "star", "2 Sterne verdient");
                }
            });
            let buildings = island.lots.iter().flatten().count();
            let extra = if level.order_count() > 0 {
                (
                    "ferry",
                    "Lieferungen",
                    format!("{} / {}", island.orders_delivered, level.order_count()),
                )
            } else {
                (
                    "hammer",
                    "Bürger im Einsatz",
                    island.assigned_staff().to_string(),
                )
            };
            for metrics in [
                [
                    (
                        "house",
                        "Bebaute Bauplätze",
                        format!("{buildings} / {}", level.plots()),
                    ),
                    ("age_adult", "Einwohner", island.staff().to_string()),
                ],
                [
                    (
                        "storehouse",
                        "Rohstoffe im Lager",
                        format!("{} / {}", island.raw_stock(), island.storage_capacity()),
                    ),
                    extra,
                ],
            ] {
                p.spawn(row(8.0)).with_children(|r| {
                    for (art, title, value) in metrics {
                        let mut tile = frame_node(Val::Percent(50.0), 4.0, 9.0);
                        tile.flex_direction = FlexDirection::Row;
                        tile.align_items = AlignItems::Center;
                        tile.column_gap = Val::Px(9.0);
                        card(r, tile, CARD).with_children(|cell| {
                            icon(cell, hud, art, 34.0);
                            cell.spawn(column(1.0)).with_children(|words| {
                                label(words, hud, value, 23.0, Face::Bold, GOLD);
                                label(words, hud, title, 17.0, Face::Body, SAND);
                            });
                        });
                    }
                });
            }
            if let Some(next) = next {
                let destination = if next.region != level.region {
                    format!("{} · {}", next.region.name(), next.name)
                } else {
                    next.name.to_owned()
                };
                action(
                    p,
                    hud,
                    Command::Next,
                    &format!("Nächste Insel · {destination}"),
                    "ferry",
                    "A",
                    Style::Primary,
                    false,
                );
            }
            p.spawn(Node {
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(7.0),
                ..row(8.0)
            })
            .with_children(|r| {
                action(
                    r,
                    hud,
                    Command::Close,
                    if a.practice() {
                        "Weiter üben"
                    } else {
                        "Bei Siedlung bleiben"
                    },
                    "house",
                    if next.is_some() { "B" } else { "A" },
                    if next.is_some() {
                        Style::Quiet
                    } else {
                        Style::Primary
                    },
                    false,
                );
                action(
                    r,
                    hud,
                    Command::Atlas,
                    "Inselkarte",
                    "chart",
                    "",
                    Style::Quiet,
                    false,
                );
                button(
                    r,
                    hud,
                    Action::Wardrobe,
                    "Garderobe",
                    "nili",
                    "",
                    Style::Quiet,
                    false,
                );
            });
        },
    );
}

fn objectives(p: &mut ChildSpawnerCommands, hud: &Hud, level: Level, island: &Island) {
    for req in level.goals() {
        let done = island.progress(req).min(req.count);
        let step = if req.tier > 1 {
            format!("{} · Stufe {}", req.kind.name(level.region), req.tier)
        } else {
            req.kind.name(level.region).to_owned()
        };
        goal(
            p,
            hud,
            build_icon(req.kind),
            &step,
            done >= req.count,
            format!("{done}/{}", req.count),
        );
    }
    let (good, need) = level.reserve();
    let units = island.amount(good);
    goal(
        p,
        hud,
        good.icon(level.region),
        good.name(level.region),
        units >= need,
        format!("{}/{need}", units.min(need)),
    );
    if level.plan().is_some() {
        let finished = island.quest == Quest::Finished;
        goal(
            p,
            hud,
            "adventure_blueprint",
            "Bauplan finden",
            finished,
            String::new(),
        );
    }
    if level.order_count() > 0 {
        goal(
            p,
            hud,
            "storehouse",
            "Lieferungen zum Außenposten",
            island.orders_delivered >= level.order_count(),
            format!("{}/{}", island.orders_delivered, level.order_count()),
        );
    }
}

fn goal(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    art: &str,
    title: &str,
    done: bool,
    count: String,
) {
    p.spawn(row(7.0)).with_children(|r| {
        icon(r, hud, if done { "check" } else { art }, 25.0);
        r.spawn(Node {
            flex_grow: 1.0,
            ..default()
        })
        .with_children(|c| {
            label(
                c,
                hud,
                title,
                16.0,
                Face::Bold,
                if done { GREEN } else { SAND },
            )
        });
        if !count.is_empty() {
            label(
                r,
                hud,
                count,
                16.0,
                Face::Bold,
                if done { GREEN } else { GOLD },
            );
        }
    });
}

fn stock(p: &mut ChildSpawnerCommands, hud: &Hud, good: Good, n: u16, region: Region) {
    p.spawn(row(3.0)).with_children(|r| {
        icon(r, hud, good.icon(region), 25.0);
        label(r, hud, n.to_string(), 19.0, Face::Bold, SAND);
    });
}

fn costs_row(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    region: Region,
    island: &Island,
    costs: &[(Good, u16)],
) {
    p.spawn(Node {
        flex_wrap: FlexWrap::Wrap,
        ..row(8.0)
    })
    .with_children(|row| {
        for &(good, needed) in costs {
            row.spawn(self::row(2.0)).with_children(|cell| {
                let enough = island.amount(good) >= needed;
                icon(cell, hud, good.icon(region), 23.0);
                label(
                    cell,
                    hud,
                    if enough {
                        needed.to_string()
                    } else {
                        format!("{}/{needed}", island.amount(good))
                    },
                    17.0,
                    Face::Bold,
                    if enough { SAND } else { WARN },
                );
            });
        }
    });
}

fn free_staff(island: &Island) -> String {
    format!("{} / {} frei", island.available_staff(), island.staff())
}

fn team(p: &mut ChildSpawnerCommands, hud: &Hud, island: &Island, task: Task) {
    p.spawn(row(10.0)).with_children(|r| {
        panels::metric(r, hud, "adventure_labor", free_staff(island));
        assignment(r, hud, island, task);
    });
}

fn assignment(p: &mut ChildSpawnerCommands, hud: &Hud, island: &Island, task: Task) {
    let assigned = island.assignment(task);
    action(
        p,
        hud,
        Command::Assign(task, false),
        "−",
        "",
        "",
        Style::Quiet,
        assigned == 0,
    );
    label(p, hud, assigned.to_string(), 20.0, Face::Bold, SAND);
    action(
        p,
        hud,
        Command::Assign(task, true),
        "+",
        "",
        "",
        Style::Quiet,
        assigned >= 3 || island.available_staff() == 0,
    );
}

fn paging(p: &mut ChildSpawnerCommands, hud: &Hud, page: u8, total: usize, page_size: usize) {
    if total <= page_size {
        return;
    }
    action(
        p,
        hud,
        Command::Page(page.saturating_sub(1)),
        "‹",
        "",
        "",
        Style::Quiet,
        page == 0,
    );
    label(
        p,
        hud,
        format!("{} / {}", page + 1, total.div_ceil(page_size)),
        17.0,
        Face::Bold,
        MUTED,
    );
    action(
        p,
        hud,
        Command::Page(page + 1),
        "›",
        "",
        "",
        Style::Quiet,
        (usize::from(page) + 1) * page_size >= total,
    );
}

fn build_icon(kind: Build) -> &'static str {
    match kind {
        Build::Harbor(kind) => kind_icon(kind),
        Build::Sawmill => "adventure_sawmill",
        Build::Brickyard => "adventure_brickyard",
        Build::Forge => "adventure_forge",
    }
}

fn target_icon(target: Target, a: &Adventure) -> &'static str {
    let terrain = a.terrain.as_ref().unwrap();
    match target {
        Target::Deposit(id) => terrain.deposits[id].kind.icon(terrain.level.region),
        Target::Plot(id) if a.active().unwrap().site(id).is_some() => "hammer",
        Target::Plot(id) if terrain.level.coastal(id) => "fishery",
        Target::Plot(id) => {
            a.active().unwrap().lots[id].map_or("hammer", |property| build_icon(property.kind))
        }
        Target::Hall => "book",
        Target::Shrine => "shrine",
        Target::Relic => "adventure_blueprint",
        Target::Outpost => "storehouse",
    }
}

fn target_name(target: Target, a: &Adventure) -> &'static str {
    let terrain = a.terrain.as_ref().unwrap();
    match target {
        Target::Deposit(id) => terrain.deposits[id].kind.name(terrain.level.region),
        Target::Plot(id) if a.active().unwrap().site(id).is_some() => a
            .active()
            .unwrap()
            .site(id)
            .unwrap()
            .property
            .kind
            .name(terrain.level.region),
        Target::Plot(id) => a.active().unwrap().lots[id].map_or(
            if terrain.level.coastal(id) {
                "Küstenbauplatz"
            } else {
                "Freier Bauplatz"
            },
            |property| property.kind.name(terrain.level.region),
        ),
        Target::Hall => terrain.level.hall(),
        Target::Shrine => terrain.level.landmark,
        Target::Relic => terrain.level.relic,
        Target::Outpost => terrain.level.outpost(),
    }
}

#[allow(clippy::too_many_arguments)]
fn action(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    cmd: Command,
    title: &str,
    art: &str,
    hotkey: &str,
    style: Style,
    disabled: bool,
) {
    button(
        p,
        hud,
        Action::Adventure(cmd),
        title,
        art,
        hotkey,
        style,
        disabled,
    );
}

fn back(p: &mut ChildSpawnerCommands, hud: &Hud, title: &str) {
    action(p, hud, Command::Close, title, "", "B", Style::Quiet, false);
}

#[derive(Component)]
pub(crate) struct WorldTip(Target);
#[derive(Component)]
pub(crate) struct RoutePointer;
#[derive(Component)]
pub(crate) struct PlayerPin;
#[derive(Component)]
pub(crate) struct RouteArrow;
#[derive(Component)]
pub(crate) struct RouteDistance;

#[derive(Component)]
pub(crate) struct HarvestProgress;

#[derive(Component)]
pub(crate) enum WorkProgress {
    Construction(usize),
    Production(usize),
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn project(
    a: Res<Adventure>,
    game: Res<Game>,
    ui_scale: Res<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<StreetCamera>>,
    mut tips: Query<(&WorldTip, &ComputedNode, &mut Node, &mut Visibility)>,
    mut pointers: Query<(&mut Node, &mut Visibility), (With<RoutePointer>, Without<WorldTip>)>,
    mut player: Query<
        (&mut Node, &mut Visibility),
        (With<PlayerPin>, Without<WorldTip>, Without<RoutePointer>),
    >,
    mut arrow: Query<&mut Text, With<RouteArrow>>,
    mut distance: Query<&mut Text, (With<RouteDistance>, Without<RouteArrow>)>,
    mut progress: Query<
        &mut Node,
        (
            With<HarvestProgress>,
            Without<WorldTip>,
            Without<RoutePointer>,
            Without<PlayerPin>,
        ),
    >,
    mut batches: Query<
        (&WorkProgress, &mut Node),
        (
            Without<HarvestProgress>,
            Without<WorldTip>,
            Without<RoutePointer>,
            Without<PlayerPin>,
        ),
    >,
) {
    if let Some(harvest) = a.harvesting
        && let Ok(mut bar) = progress.single_mut()
    {
        bar.width = Val::Percent((1.0 - harvest.left / harvest.duration) * 100.0);
    }
    if let Some(island) = a.active() {
        for (source, mut bar) in &mut batches {
            let progress = match *source {
                WorkProgress::Construction(lot) => island.site(lot).map(BuildingSite::progress),
                WorkProgress::Production(lot) => island.batch(lot).map(|batch| batch.progress()),
            };
            if let Some(progress) = progress {
                bar.width = Val::Percent(progress * 100.0);
            }
        }
    }
    if tips.is_empty() && pointers.is_empty() && player.is_empty() {
        return;
    }
    let Some((terrain, island)) = a.terrain.as_ref().zip(a.active()) else {
        return;
    };
    let (camera, transform) = *camera;
    let scale = ui_scale.0;
    let safe = if game.phone {
        insets().map(|n| f32::from(n) / (scale * window.scale_factor()))
    } else {
        [0.0; 4]
    };
    let origin = Vec2::new(safe[0], safe[1]);
    let size =
        Vec2::new(window.width(), window.height()) / scale - origin - Vec2::new(safe[2], safe[3]);
    let project = |target, height| {
        let position = crate::world::adventure_position(terrain, a.target_position(target)?);
        camera
            .world_to_viewport(transform, position + Vec3::Y * height)
            .ok()
            .map(|point| point / scale - origin)
    };
    if let Ok((mut node, mut visible)) = player.single_mut() {
        let here = Vec2::from_array(island.position);
        let toward_camera = Vec2::new(0.18, 1.32).normalize();
        let obscured = terrain
            .plots
            .iter()
            .zip(&island.lots)
            .filter(|(_, property)| property.is_some())
            .map(|(&plot, _)| plot - here)
            .chain(std::iter::once(-here))
            .any(|delta| {
                (0.75..3.0).contains(&delta.dot(toward_camera))
                    && delta.perp_dot(toward_camera).abs() < 1.26
            });
        let point = crate::world::adventure_position(terrain, here) + Vec3::Y * 2.17;
        if obscured && let Ok(pixel) = camera.world_to_viewport(transform, point) {
            let pos = pixel / scale - origin;
            node.left = Val::Px(pos.x - 15.0);
            node.top = Val::Px(pos.y - 22.0);
            *visible = Visibility::Inherited;
        } else {
            *visible = Visibility::Hidden;
        }
    }
    for (tip, computed, mut node, mut vis) in &mut tips {
        let height = match tip.0 {
            Target::Plot(lot) if island.lots[lot].is_some() => 2.6,
            Target::Plot(_) => 1.1,
            _ => 2.1,
        };
        if let Some(point) = project(tip.0, height).filter(|point| {
            point.x > 0.0 && point.x < size.x && point.y > 74.0 && point.y < size.y - 75.0
        }) {
            node.left = Val::Px(
                (point.x - computed.size().x * computed.inverse_scale_factor() * 0.5).max(10.0),
            );
            node.top = Val::Px((point.y - 20.0).max(12.0));
            *vis = Visibility::Inherited;
        } else {
            *vis = Visibility::Hidden;
        }
    }
    if let Some(target) = a.waypoint.or_else(|| a.tutorial().map(|t| t.2))
        && !a.close_enough(target)
        && let Some(point) = project(target, 0.8)
    {
        let min = Vec2::new(55.0, if game.phone { 184.0 } else { 284.0 });
        let max = Vec2::new(
            (size.x
                - if game.phone && !game.quick_menu {
                    84.0
                } else {
                    321.0
                })
            .max(min.x + 5.0),
            (size.y - 136.0).max(min.y + 5.0),
        );
        let edge = point.clamp(min, max);
        let offscreen = point.distance(edge) > 18.0;
        let badge = if offscreen {
            edge
        } else {
            project(target, 3.3).unwrap_or(point).clamp(min, max)
        };
        for (mut node, mut visible) in &mut pointers {
            node.left = Val::Px(badge.x - 36.0);
            node.top = Val::Px(badge.y - 22.0);
            *visible = if offscreen || a.waypoint.is_some() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        if let Ok(mut text) = arrow.single_mut() {
            let delta = point - edge;
            let direction = ((delta.y.atan2(delta.x) * 4.0 / std::f32::consts::PI).round() as i32)
                .rem_euclid(8) as usize;
            let glyph = if offscreen {
                ["→", "↘", "↓", "↙", "←", "↖", "↑", "↗"][direction]
            } else {
                "↓"
            };
            if text.0 != glyph {
                text.0 = glyph.to_owned();
            }
        }
    } else {
        for (_, mut visible) in &mut pointers {
            *visible = Visibility::Hidden;
        }
    }
    if let Some(target) = a.waypoint
        && let Ok(mut text) = distance.single_mut()
    {
        let message = route_distance(&a, target);
        if text.0 != message {
            text.0 = message;
        }
    }
}

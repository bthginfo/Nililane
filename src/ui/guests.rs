use crate::{
    campaign::{Chapter, Region},
    model::{
        Age, GuestTip, Miss, Origin, SERVICES, Service, Tally, Town, Verdict, Visit,
        satisfaction_percent,
    },
};

use super::*;

const SATISFIED: Color = Color::srgb(0.60, 0.83, 0.68);
const UNSERVED: Color = Color::srgb(0.98, 0.61, 0.53);

#[derive(Clone, Copy, Default)]
struct Responses {
    served: u32,
    total: u32,
}

impl Responses {
    fn from_tally(tally: &Tally) -> Self {
        Self {
            served: tally.served_total(),
            total: tally.responses(),
        }
    }

    fn percent(self) -> Option<u32> {
        satisfaction_percent(self.served, self.total)
    }

    fn fill(self) -> Val {
        Val::Percent(if self.total == 0 {
            0.0
        } else {
            self.served as f32 / self.total as f32 * 100.0
        })
    }

    fn symbol(self) -> &'static str {
        match self.percent() {
            Some(85..=100) => "guest_happy",
            Some(60..=84) => "guest_mixed",
            Some(_) => "guest_down",
            None => "guest_wait",
        }
    }

    fn color(self) -> Color {
        match self.percent() {
            Some(85..=100) => SATISFIED,
            Some(60..=84) => GOLD,
            Some(_) => UNSERVED,
            None => MUTED,
        }
    }

    fn counts(self) -> String {
        format!("{} von {} erfüllt", self.served, self.total)
    }
}

pub(super) fn face(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pleased => "guest_happy",
        Verdict::Missed(_) => "guest_down",
    }
}

pub(super) fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pleased => "Zufrieden",
        Verdict::Missed(reason) => reason.label(),
    }
}

pub(super) fn short_verdict(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pleased => "Zufrieden",
        Verdict::Missed(Miss::NoShop) => "Fehlt",
        Verdict::Missed(Miss::Walk) => "Zu weit",
        Verdict::Missed(Miss::Price) => "Zu teuer",
        Verdict::Missed(Miss::Empty) => "Ausverkauft",
    }
}

fn reason_icon(reason: Miss) -> &'static str {
    match reason {
        Miss::NoShop => "hammer",
        Miss::Walk => "guest_steps",
        Miss::Price => "coin",
        Miss::Empty => "guest_empty",
    }
}

fn latest(town: &Town) -> Option<(&Visit, Verdict)> {
    town.guest_history()
        .iter()
        .filter_map(|visit| Some((visit, visit.verdict()?)))
        .max_by(|(a, _), (b, _)| a.at.total_cmp(&b.at))
}

#[derive(Component)]
pub(crate) enum SummaryIcon {
    Mood,
    Latest,
}

#[derive(Component)]
pub(crate) enum SummaryText {
    Score,
    Counts,
    Latest,
}

#[derive(Component)]
pub(crate) struct SummaryFill;

#[derive(Component)]
pub(crate) struct SummaryTrack;

pub(super) fn summary(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town) {
    button_shell(
        p,
        Action::Feedback,
        Style::Quiet,
        false,
        Node {
            padding: UiRect::all(Val::Px(9.0)),
            ..column(6.0)
        },
    )
    .with_children(|summary| {
        summary.spawn(row(6.0)).with_children(|head| {
            head.spawn((
                SummaryIcon::Mood,
                ImageNode::new(hud.pictures["guest_wait"].clone()),
                Node {
                    width: Val::Px(39.0),
                    height: Val::Px(39.0),
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
            head.spawn(Node {
                flex_grow: 1.0,
                ..column(1.0)
            })
            .with_children(|heading| {
                label(heading, hud, "Zufriedenheit", 18.0, Face::Bold, SAND);
                heading.spawn((
                    SummaryText::Counts,
                    Text::new(""),
                    TextFont {
                        font: hud.fonts[0].clone(),
                        font_size: 13.5,
                        ..default()
                    },
                    TextColor(MUTED),
                ));
            });
            head.spawn((
                SummaryText::Score,
                Text::new("—"),
                TextLayout::new_with_no_wrap(),
                TextFont {
                    font: hud.fonts[1].clone(),
                    font_size: 27.0,
                    ..default()
                },
                TextColor(GOLD),
            ));
        });
        summary
            .spawn((
                SummaryTrack,
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
                    SummaryFill,
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(SATISFIED),
                    BorderRadius::all(Val::Px(3.0)),
                ));
            });
        summary
            .spawn(Node {
                justify_content: JustifyContent::SpaceBetween,
                ..row(5.0)
            })
            .with_children(|footer| {
                footer.spawn(row(4.0)).with_children(|seen| {
                    seen.spawn((
                        SummaryIcon::Latest,
                        ImageNode::new(hud.pictures["guest_book"].clone()),
                        Node {
                            width: Val::Px(24.0),
                            height: Val::Px(24.0),
                            flex_shrink: 0.0,
                            ..default()
                        },
                    ));
                    seen.spawn((
                        SummaryText::Latest,
                        Text::new(if town.planning() {
                            "Gästebuch"
                        } else {
                            "Gästestimmen"
                        }),
                        TextFont {
                            font: hud.fonts[0].clone(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(SAND),
                    ));
                });
                key(footer, hud, "View", false);
            });
    });
}

#[allow(clippy::type_complexity)]
pub fn refresh(
    save: Res<Campaign>,
    hud: Res<Hud>,
    mut texts: Query<(&SummaryText, &mut Text, &mut TextColor)>,
    mut icons: Query<(&SummaryIcon, &mut ImageNode)>,
    mut fill: Query<&mut Node, With<SummaryFill>>,
    mut track: Query<&mut BackgroundColor, With<SummaryTrack>>,
) {
    if texts.is_empty() {
        return;
    }
    let Some(town) = save.active_town() else {
        return;
    };
    let responses = town
        .feedback()
        .map_or_else(Responses::default, Responses::from_tally);
    let reaction = latest(town);
    for (slot, mut text, mut color) in &mut texts {
        let (content, tint) = match slot {
            SummaryText::Score => (
                responses
                    .percent()
                    .map_or_else(|| "—".into(), |n| format!("{n} %")),
                responses.color(),
            ),
            SummaryText::Counts => (
                if responses.total > 0 {
                    let when = if town.planning() { "Gestern" } else { "Heute" };
                    format!("{when} · {}", responses.counts())
                } else {
                    "Noch keine Stimmen".into()
                },
                MUTED,
            ),
            SummaryText::Latest => (
                reaction.map_or_else(
                    || "Gästebuch öffnen".into(),
                    |(visit, verdict)| {
                        format!("{} · {}", visit.wants.name(), short_verdict(verdict))
                    },
                ),
                if let Some((_, Verdict::Missed(_))) = reaction {
                    UNSERVED
                } else {
                    SAND
                },
            ),
        };
        if text.0 != content {
            text.0 = content;
        }
        if color.0 != tint {
            color.0 = tint;
        }
    }
    for (slot, mut image) in &mut icons {
        let icon = match slot {
            SummaryIcon::Mood => responses.symbol(),
            SummaryIcon::Latest => {
                reaction.map_or("guest_book", |(visit, _)| service_icon(visit.wants))
            }
        };
        let new = &hud.pictures[icon];
        if image.image != *new {
            image.image = new.clone();
        }
    }
    if let Ok(mut node) = fill.single_mut() {
        let target = responses.fill();
        if node.width != target {
            node.width = target;
        }
    }
    if let Ok(mut background) = track.single_mut() {
        let target = if responses.total > 0 { UNSERVED } else { LINE };
        if background.0 != target {
            background.0 = target;
        }
    }
}

pub(super) fn book(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    game: &Game,
    town: &Town,
    chapter: &Chapter,
) {
    let Some(tally) = town.feedback().filter(|tally| tally.responses() > 0) else {
        waiting(p, hud, town);
        return;
    };
    let responses = Responses::from_tally(tally);
    let mut groups = [Responses::default(); 2];
    let mut ages = [Responses::default(); 3];
    let mut reviewed = Vec::new();
    for (index, visit) in town.guest_history().iter().enumerate() {
        let Some(verdict) = visit.verdict() else {
            continue;
        };
        let count = &mut groups[usize::from(visit.origin == Origin::Ferry)];
        count.total += 1;
        count.served += u32::from(verdict == Verdict::Pleased);
        let age = &mut ages[visit.age(index) as usize];
        age.total += 1;
        age.served += u32::from(verdict == Verdict::Pleased);
        reviewed.push((index, visit, verdict));
    }
    p.spawn(Node {
        align_items: AlignItems::Stretch,
        ..row(8.0)
    })
    .with_children(|r| {
        metric(r, hud, "Insgesamt", responses.symbol(), responses);
        if !reviewed.is_empty() {
            metric(r, hud, "Nachbarn", "house", groups[0]);
            metric(r, hud, "Von der Fähre", "ferry", groups[1]);
        }
    });
    if Age::Child.available(town.chapter) && !reviewed.is_empty() {
        p.spawn(row(8.0)).with_children(|r| {
            for age in Age::ALL
                .into_iter()
                .filter(|age| age.available(town.chapter))
            {
                metric(r, hud, age.group(), age.icon(), ages[age as usize]);
            }
        });
    }

    if tally.gaps.iter().any(|&count| count != 0) {
        p.spawn(row(8.0)).with_children(|reasons| {
            for reason in Miss::ALL {
                blame(
                    reasons,
                    hud,
                    reason,
                    tally.gaps[reason as usize],
                    game.review_reason,
                );
            }
        });
    }

    if let Some(reason) = game.review_reason {
        reviewed.retain(|(_, _, verdict)| *verdict == Verdict::Missed(reason));
    }
    reviewed.sort_by(|a, b| b.1.at.total_cmp(&a.1.at));
    let pointed = game
        .review_guest
        .and_then(|guest| reviewed.iter().find(|(i, _, _)| *i == guest));
    let next_step = pointed.or_else(|| {
        let mut by_issue = [[0_u32; SERVICES]; 4];
        for (_, visit, verdict) in &reviewed {
            if let Verdict::Missed(reason) = verdict {
                by_issue[*reason as usize][visit.wants as usize] += 1;
            }
        }
        reviewed
            .iter()
            .filter_map(|card @ (_, visit, verdict)| {
                let Verdict::Missed(reason) = verdict else {
                    return None;
                };
                Some((by_issue[*reason as usize][visit.wants as usize], card))
            })
            .max_by(|(left, a), (right, b)| left.cmp(right).then_with(|| a.1.at.total_cmp(&b.1.at)))
            .map(|(_, card)| card)
    });
    let chosen = next_step.or_else(|| reviewed.first());
    let companion = reviewed
        .iter()
        .find(|candidate| chosen.is_some_and(|card| candidate.0 != card.0 && candidate.2 != card.2))
        .or_else(|| {
            reviewed
                .iter()
                .find(|candidate| chosen.is_some_and(|card| candidate.0 != card.0))
        });

    if let Some(&(i, visit, verdict)) = chosen {
        p.spawn(Node {
            align_items: AlignItems::Stretch,
            ..row(8.0)
        })
        .with_children(|cards| {
            guest_card(cards, hud, town, chapter, i, visit, verdict, true);
            if let Some(&(i, visit, verdict)) = companion {
                guest_card(cards, hud, town, chapter, i, visit, verdict, false);
            }
        });
    } else {
        let hint = game.review_reason.or_else(|| {
            Miss::ALL
                .into_iter()
                .filter(|reason| tally.gaps[*reason as usize] > 0)
                .max_by_key(|reason| tally.gaps[*reason as usize])
        });
        card(
            p,
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                ..row(8.0)
            },
            CARD,
        )
        .with_children(|r| {
            icon(r, hud, hint.map_or("guest_book", reason_icon), 36.0);
            label(
                r,
                hud,
                hint.map_or("Neue Stimmen gibt’s am nächsten Spieltag.", Miss::hint),
                17.0,
                Face::Body,
                SAND,
            );
        });
    }
}

fn waiting(p: &mut ChildSpawnerCommands, hud: &Hud, town: &Town) {
    card(
        p,
        Node {
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(12.0)),
            ..row(16.0)
        },
        CARD,
    )
    .with_children(|welcome| {
        chapter_picture(welcome, hud, town.chapter, Val::Px(180.0), Val::Px(94.0));
        welcome.spawn(column(5.0)).with_children(|words| {
            label(
                words,
                hud,
                "Die ersten Gäste kommen gleich",
                24.0,
                Face::Serif,
                SAND,
            );
            label(
                words,
                hud,
                "Tippe eine Sprechblase an, um die Rückmeldung zu sehen.",
                16.0,
                Face::Body,
                MUTED,
            );
            words.spawn(row(12.0)).with_children(|legend| {
                for (symbol, caption) in [
                    ("guest_happy", "Wunsch erfüllt"),
                    ("guest_down", "Wunsch offen"),
                ] {
                    legend.spawn(row(4.0)).with_children(|mark| {
                        icon(mark, hud, symbol, 27.0);
                        label(mark, hud, caption, 16.0, Face::Bold, SAND);
                    });
                }
            });
        });
    });
}

fn metric(p: &mut ChildSpawnerCommands, hud: &Hud, heading: &str, symbol: &str, value: Responses) {
    card(
        p,
        Node {
            flex_grow: 1.0,
            flex_basis: Val::Px(0.0),
            padding: UiRect::all(Val::Px(9.0)),
            ..column(5.0)
        },
        CARD,
    )
    .with_children(|metric| {
        metric.spawn(row(6.0)).with_children(|r| {
            icon(r, hud, symbol, 36.0);
            r.spawn(column(0.0)).with_children(|c| {
                label(c, hud, heading, 16.0, Face::Bold, SAND);
                label(
                    c,
                    hud,
                    if value.total == 0 {
                        "Noch keine Stimmen".into()
                    } else {
                        value.counts()
                    },
                    14.5,
                    Face::Body,
                    MUTED,
                );
            });
            r.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            label(
                r,
                hud,
                value
                    .percent()
                    .map_or_else(|| "—".into(), |n| format!("{n} %")),
                25.0,
                Face::Bold,
                value.color(),
            );
        });
        metric
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(6.0),
                    ..default()
                },
                BackgroundColor(if value.total > 0 { UNSERVED } else { LINE }),
                BorderRadius::all(Val::Px(3.0)),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: value.fill(),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(SATISFIED),
                    BorderRadius::all(Val::Px(3.0)),
                ));
            });
    });
}

fn blame(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    reason: Miss,
    count: u32,
    selected: Option<Miss>,
) {
    button_shell(
        p,
        Action::ReviewReason(reason),
        if selected == Some(reason) {
            Style::Selected
        } else {
            Style::Quiet
        },
        count == 0,
        Node {
            flex_grow: 1.0,
            flex_basis: Val::Px(0.0),
            min_height: Val::Px(45.0),
            padding: UiRect::all(Val::Px(6.0)),
            ..row(5.0)
        },
    )
    .with_children(|r| {
        icon(r, hud, reason_icon(reason), 27.0);
        label(
            r,
            hud,
            reason.label(),
            15.0,
            Face::Body,
            if count == 0 { MUTED } else { SAND },
        );
        r.spawn(Node {
            flex_grow: 1.0,
            ..default()
        });
        label(
            r,
            hud,
            count.to_string(),
            18.0,
            Face::Bold,
            if count == 0 { MUTED } else { UNSERVED },
        );
    });
}

#[allow(clippy::too_many_arguments)]
fn guest_card(
    p: &mut ChildSpawnerCommands,
    hud: &Hud,
    town: &Town,
    chapter: &Chapter,
    index: usize,
    visit: &Visit,
    verdict: Verdict,
    featured: bool,
) {
    card(
        p,
        Node {
            flex_grow: if featured { 1.16 } else { 1.0 },
            flex_basis: Val::Px(0.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..column(7.0)
        },
        if featured { JADE } else { CARD },
    )
    .with_children(|c| {
        c.spawn(row(7.0)).with_children(|identity| {
            icon(identity, hud, face(verdict), 40.0);
            identity.spawn(column(1.0)).with_children(|words| {
                let guest = match visit.origin {
                    Origin::Home(_) => "Aus der Gasse",
                    Origin::Ferry if visit.wants == Service::Prayer => "Auf Pilgerreise",
                    Origin::Ferry if visit.guided_to.is_some() => "Mit Nili unterwegs",
                    Origin::Ferry => "Von der Fähre",
                };
                let names = match Region::of(town.chapter) {
                    Region::Greece => [
                        "Kleio", "Doros", "Eleni", "Mara", "Ion", "Thaleia", "Nikos", "Ione",
                    ],
                    Region::Rome => [
                        "Flavia", "Gaius", "Livia", "Felix", "Tertia", "Lucius", "Junia", "Marius",
                    ],
                    Region::Japan => ["Aki", "Hana", "Sora", "Haru", "Yui", "Ren", "Mio", "Nao"],
                    Region::Oasis => [
                        "Nura", "Bahir", "Soraya", "Rima", "Dalia", "Samir", "Zahra", "Ilyas",
                    ],
                    Region::Nordic => [
                        "Liv", "Leif", "Eira", "Tora", "Signe", "Astrid", "Rune", "Inga",
                    ],
                };
                let day = town.day.saturating_sub(u32::from(town.planning())) as usize;
                let name = names[(index + day) % names.len()];
                label(
                    words,
                    hud,
                    format!("{name} · {guest}"),
                    14.0,
                    Face::Bold,
                    MUTED,
                );
                label(
                    words,
                    hud,
                    format!("{} · {}", visit.age(index).label(), verdict_name(verdict)),
                    18.0,
                    Face::Bold,
                    SAND,
                );
            });
            identity.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            icon(identity, hud, service_icon(visit.wants), 38.0);
        });
        let quote = match verdict {
            Verdict::Pleased if visit.specialty_bonus > 0 => "Mein Lieblingsangebot!",
            Verdict::Pleased if visit.wants == Service::Prayer => "Am Tempel war es ganz still.",
            Verdict::Pleased => "Ich habe bekommen, was ich gesucht habe!",
            Verdict::Missed(Miss::NoShop) => "Das habe ich hier vermisst.",
            Verdict::Missed(Miss::Walk) => "Der Weg zum Laden war mir zu weit.",
            Verdict::Missed(Miss::Price) => "Das war mir zu teuer.",
            Verdict::Missed(Miss::Empty) => {
                if has_places(visit.wants) {
                    "Es war kein Platz mehr frei."
                } else {
                    "Das war schon ausverkauft."
                }
            }
        };
        label(
            c,
            hud,
            format!("{} · „{quote}“", visit.wants.name()),
            16.5,
            Face::Body,
            SAND,
        );
        if visit.specialty_bonus > 0 {
            super::panels::metric(
                c,
                hud,
                visit.age(index).icon(),
                format!("Spezialität · +{} Münzen", visit.specialty_bonus),
            );
        } else if verdict == Verdict::Pleased
            && let Some(b) = visit.shop.and_then(|lot| town.lots[usize::from(lot)])
            && crate::model::Assortment::supported(b.kind)
        {
            use crate::model::Age;
            let age = visit.age(index);
            let observation = match (b.kind, age) {
                (Kind::Potter, Age::Child) => "Ein bisschen Farbe wäre schön.",
                (Kind::Potter, Age::Adult) => "Ich suche etwas für den Alltag.",
                (Kind::Potter, Age::Senior) => "Mir gefallen traditionelle Muster.",
                (Kind::PaperWorkshop, Age::Child) => "Ein Lampion in Tierform wäre schön.",
                (Kind::PaperWorkshop, Age::Adult) => "Ich suche einen Lampion für den Weg.",
                (Kind::PaperWorkshop, Age::Senior) => "Ich mag die traditionellen Lampions.",
                (Kind::TeaHouse, Age::Child) => "Ich mag milden Tee.",
                (Kind::TeaHouse, Age::Adult) => "Gerösteter Tee duftet so gut.",
                (Kind::TeaHouse, Age::Senior) => "Ich trinke gern frisch aufgebrühten Grüntee.",
                (Kind::Weaver, Age::Child) => "Ein buntes Band für mein Haar!",
                (Kind::Weaver, Age::Adult) => "Ich brauche etwas Warmes für die Fahrt.",
                (Kind::Weaver, Age::Senior) => "Schönes Leinen fühlt sich gut an.",
                (_, Age::Child) => "Ich mag es lieber süß.",
                (_, Age::Adult) => "Ich hätte Lust auf etwas Herzhaftes.",
                (_, Age::Senior) => "Am liebsten etwas Vertrautes.",
            };
            super::panels::metric(c, hud, age.icon(), observation);
        }
        if verdict == Verdict::Missed(Miss::Price) {
            let budget = visit.budget.coins(visit.wants);
            let price = visit
                .declined_at
                .and_then(|lot| town.lots[usize::from(lot)])
                .filter(|b| b.kind.service() == Some(visit.wants))
                .map(|b| b.price.coins(visit.wants));
            let budget = format!("Bis zu {budget} {}", Region::of(town.chapter).currency());
            let price = price.map_or_else(
                || budget.clone(),
                |price| format!("Preis {price}  ·  {budget}"),
            );
            label(c, hud, price, 15.5, Face::Bold, GOLD);
        }
        if let Some(tip) = town.guest_tip(chapter, index) {
            let (name, pic) = match tip {
                GuestTip::Shop(lot) => {
                    let shop = town.lots[lot].expect("a shop tip points to an existing building");
                    (
                        format!("{} ansehen", shop.kind.name()),
                        kind_icon(shop.kind),
                    )
                }
                GuestTip::Build(_, kind) => (format!("{} bauen", kind.name()), "hammer"),
            };
            button(
                c,
                hud,
                Action::GuestAdvice(index),
                &name,
                pic,
                "",
                Style::Quiet,
                false,
            );
        } else if let Verdict::Missed(reason) = verdict {
            label(c, hud, reason.hint(), 15.0, Face::Body, MUTED);
        }
    });
}

pub(super) fn total_symbol(tally: &Tally) -> &'static str {
    Responses::from_tally(tally).symbol()
}

pub(super) fn evening_hint(town: &Town, tally: &Tally) -> (String, &'static str) {
    let mut causes = [[0_u32; SERVICES]; 4];
    for visit in town.guest_history() {
        if let Some(Verdict::Missed(reason)) = visit.verdict() {
            causes[reason as usize][visit.wants as usize] += 1;
        }
    }
    let most = Miss::ALL
        .into_iter()
        .flat_map(|reason| {
            Service::ALL
                .into_iter()
                .map(move |service| (causes[reason as usize][service as usize], service, reason))
        })
        .filter(|(count, _, _)| *count > 0)
        .max_by_key(|&(count, _, _)| count);
    if let Some((count, service, reason)) = most {
        (
            format!(
                "{} · {} · {count} {}. {}",
                service.name(),
                reason.label(),
                if count == 1 { "Gast" } else { "Gäste" },
                reason.hint()
            ),
            reason_icon(reason),
        )
    } else if tally.responses() == tally.served_total() && tally.responses() > 0 {
        ("Alle Wünsche erfüllt.".into(), "guest_happy")
    } else {
        (
            "Die Rückmeldungen stehen im Gästebuch.".into(),
            "guest_book",
        )
    }
}

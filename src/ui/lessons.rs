use crate::{
    coach::{Lesson, Prompt, Step},
    errands::{Job, Stop},
    model::{Favor, Town},
    world::{StreetCamera, lot_position, route_position},
};

use super::{floating::place_box, *};

#[derive(Component)]
pub(crate) enum Piece {
    Shade(usize),
    Ring,
    Note,
}

struct Wording {
    icon: &'static str,
    title: &'static str,
    hint: &'static str,
    button: &'static str,
    dismiss: bool,
}

impl Wording {
    fn for_prompt(prompt: Prompt, chapter: usize) -> Self {
        let (icon, title, hint, button, dismiss) = match prompt {
            Prompt::Action(Step::Build(_, Kind::Bakery)) => (
                "bakery",
                "Hier entsteht die Backstube",
                "Die Nachbarn kommen zu Fuß zum Bäcker.",
                "Zum Bauplatz",
                true,
            ),
            Prompt::Action(Step::Build(_, Kind::Potter)) => (
                "potter",
                "Eine Töpferei am Kai",
                "Vom Kai aus sind es nur ein paar Schritte.",
                "Zum Bauplatz",
                true,
            ),
            Prompt::Action(Step::Build(_, Kind::Well)) if chapter == 2 => (
                "well",
                "Ein Brunnen für Delos",
                "Mit genug Wasser kannst du mehr Schalen brennen.",
                "Zum Bauplatz",
                true,
            ),
            Prompt::Action(Step::Build(..)) => (
                "well",
                "Wasser für die Backstube",
                "Mit mehr Wasser gibt es mehr Brot.",
                "Zum Bauplatz",
                true,
            ),
            Prompt::Action(Step::Price(_)) if chapter == 2 => (
                "budget",
                "Was darf eine Schale kosten?",
                "Die Balken zeigen, wie viele Gäste den Preis zahlen können.",
                "Preise ansehen",
                true,
            ),
            Prompt::Action(Step::Price(_)) => (
                "budget",
                "Was darf das Brot kosten?",
                "Die Balken zeigen, wie viele Gäste den Preis zahlen können.",
                "Preise ansehen",
                true,
            ),
            Prompt::Action(Step::Upgrade(_)) => (
                "hammer",
                "Mehr Schalen für Delos",
                "Baue die Töpferei auf Stufe 2 aus.",
                "Zum Ausbau",
                true,
            ),
            Prompt::Action(Step::StartDay) => (
                "sun",
                "Bereit für Gäste?",
                "Sobald du den Tag startest, kommen die Gäste.",
                "Tag starten",
                true,
            ),
            Prompt::Action(Step::Arrivals) => (
                "ferry",
                "Die Fähre ist da!",
                "In den Sprechblasen siehst du, was die Gäste suchen.",
                "Weiter",
                false,
            ),
            Prompt::Action(Step::Aid(_, Favor::Cargo)) => (
                "nili",
                "Nili kann aushelfen",
                "Steuere Nili zum Kai. Dort nimmst du die Kiste auf.",
                "Nili steuern",
                true,
            ),
            Prompt::Action(Step::Aid(..)) => (
                "nili",
                "Nili kann aushelfen",
                "Geh mit Nili zur Markierung und bestätige dort mit A oder per Tipp.",
                "Nili steuern",
                true,
            ),
            Prompt::Action(Step::Tour(_, Job::Water)) => (
                "water",
                "Wasser für die Backstube",
                "Wähle „Wasser bringen“ und dann die Backstube.",
                "Nilis Touren",
                true,
            ),
            Prompt::Action(Step::Tour(..)) => (
                "storehouse",
                "Nachschub für Delos",
                "Hol die Kiste mit Nili am Kai und bring sie zur Töpferei.",
                "Nilis Touren",
                true,
            ),
            Prompt::Feedback(_) => (
                "guest_happy",
                "Wie zufrieden sind die Gäste?",
                "Im Gästebuch siehst du, was gut lief.",
                "Gästebuch öffnen",
                true,
            ),
            Prompt::Delivered(job, _) => (
                "check",
                "Nili ist angekommen!",
                if job == Job::Freight {
                    "Die Töpferei hat wieder Schalen auf Lager."
                } else {
                    job.purpose()
                },
                "Weiter",
                false,
            ),
            Prompt::Action(Step::Evening) => (
                "sunset",
                "Zeit für den Feierabend",
                "Jetzt bekommst du die Einnahmen des Tages.",
                "Weiter",
                false,
            ),
        };
        Self {
            icon,
            title,
            hint,
            button,
            dismiss,
        }
    }
}

pub(super) fn spawn(p: &mut ChildSpawnerCommands, hud: &Hud, lesson: Lesson, town: &Town) {
    let copy = Wording::for_prompt(lesson.prompt, town.chapter);
    p.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ZIndex(30),
        FocusPolicy::Block,
    ))
    .with_children(|overlay| {
        for edge in 0..4 {
            overlay.spawn((
                Piece::Shade(edge),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.023, 0.095, 0.11, 0.62)),
                FocusPolicy::Pass,
            ));
        }
        overlay.spawn((
            Piece::Ring,
            Button,
            ButtonAction {
                action: Action::Lesson(true),
                style: Style::Spotlight,
                disabled: false,
            },
            feel::Ease::default(),
            BackgroundColor(Color::NONE),
            Node {
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(19.0)),
            BorderColor(GOLD),
            Outline::new(
                Val::Px(5.0),
                Val::Px(2.0),
                Color::srgba(0.98, 0.80, 0.48, 0.29),
            ),
            FocusPolicy::Block,
        ));
        let mut body = frame_node(Val::Px(364.0), 8.0, 13.0);
        body.position_type = PositionType::Absolute;
        body.border = UiRect::all(Val::Px(2.0));
        card(overlay, body, INK)
            .insert((Piece::Note, BorderColor(GOLD), ZIndex(1)))
            .with_children(|note| {
                note.spawn(row(8.0)).with_children(|caption| {
                    icon(
                        caption,
                        hud,
                        if town.planning() { "nili" } else { "pause" },
                        27.0,
                    );
                    label(
                        caption,
                        hud,
                        if town.planning() {
                            "NILIS TIPP"
                        } else {
                            "TAG PAUSIERT"
                        },
                        14.0,
                        Face::Bold,
                        GOLD,
                    );
                });
                note.spawn(row(9.0)).with_children(|title| {
                    icon(title, hud, copy.icon, 44.0);
                    title.spawn(column(2.0)).with_children(|words| {
                        label(words, hud, copy.title, 25.0, Face::Serif, SAND);
                        let hint = if hud.phone
                            && matches!(lesson.prompt, Prompt::Action(Step::Aid(..)))
                        {
                            "Zieh den Stick unten links. Am Ziel tippst du auf die goldene Aktion."
                        } else {
                            copy.hint
                        };
                        label(
                            words,
                            hud,
                            hint,
                            if hud.phone { 18.0 } else { 16.0 },
                            Face::Body,
                            MUTED,
                        );
                    });
                });
                note.spawn(row(7.0)).with_children(|actions| {
                    button(
                        actions,
                        hud,
                        Action::Lesson(true),
                        copy.button,
                        "",
                        "A",
                        Style::Primary,
                        false,
                    );
                    if copy.dismiss {
                        button(
                            actions,
                            hud,
                            Action::Lesson(false),
                            "Später",
                            "",
                            "B",
                            Style::Quiet,
                            false,
                        );
                    }
                });
            });
    });
}

fn note_at(spot: Rect, note: Vec2, screen: Vec2) -> Vec2 {
    let edge = Vec2::splat(12.0);
    let max = (screen - note - edge).max(edge);
    let centered = spot.center() - note * 0.5;
    let gap = 15.0;
    let right = Vec2::new(spot.max.x + gap, centered.y.clamp(edge.y, max.y));
    let left = Vec2::new(spot.min.x - note.x - gap, right.y);
    let below = Vec2::new(centered.x.clamp(edge.x, max.x), spot.max.y + gap);
    let above = Vec2::new(below.x, spot.min.y - note.y - gap);
    [right, left, below, above]
        .into_iter()
        .find(|p| p.cmpge(edge).all() && p.cmple(max).all())
        .unwrap_or_else(|| right.clamp(edge, max))
}

fn shade_rects(viewport: Rect, spot: Rect) -> [Rect; 4] {
    [
        Rect::from_corners(viewport.min, Vec2::new(viewport.max.x, spot.min.y)),
        Rect::from_corners(
            Vec2::new(viewport.min.x, spot.min.y),
            Vec2::new(spot.min.x, spot.max.y),
        ),
        Rect::from_corners(
            Vec2::new(spot.max.x, spot.min.y),
            Vec2::new(viewport.max.x, spot.max.y),
        ),
        Rect::from_corners(Vec2::new(viewport.min.x, spot.max.y), viewport.max),
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn update(
    game: Res<Game>,
    save: Res<Campaign>,
    scale: Res<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<StreetCamera>>,
    buttons: FocusButtons,
    hud: Res<Hud>,
    mut pieces: Query<(&Piece, &mut Node, &ComputedNode, Option<&mut Outline>)>,
) {
    let (Some(lesson), Some(town)) = (game.lesson, save.active_town()) else {
        return;
    };
    let Ok((camera, transform)) = camera.single() else {
        return;
    };
    let chapter = &CHAPTERS[town.chapter];
    let pixels = scale.0 * window.scale_factor();
    let [left, top, right, bottom] = insets().map(|v| f32::from(v) / pixels);
    let origin = Vec2::new(left, top);
    let screen =
        Vec2::new(window.width(), window.height()) / scale.0 - origin - Vec2::new(right, bottom);
    let world = |p: Vec3, size: Vec2| {
        camera
            .world_to_viewport(transform, p)
            .ok()
            .map(|point| Rect::from_center_size(point / scale.0 - origin, size))
    };
    let lot = |i, empty| {
        let center =
            lot_position(i, chapter.layout.width()) + Vec3::Y * if empty { 0.15 } else { 1.05 };
        world(
            center,
            if empty {
                Vec2::new(156.0, 100.0)
            } else {
                Vec2::new(173.0, 144.0)
            },
        )
    };
    let stop = |point| match point {
        Stop::Parcel(i) => lot(usize::from(i), false),
        _ => world(
            route_position(point, chapter.layout) + Vec3::Y * 0.32,
            Vec2::new(149.0, 121.0),
        ),
    };
    let target = match lesson.prompt {
        Prompt::Action(Step::StartDay) => buttons
            .iter()
            .find(|(_, _, action, _)| action.action == Action::Open)
            .map(|(node, transform, _, _)| {
                Rect::from_center_size(
                    transform.translation().truncate() / pixels - origin,
                    node.size() / pixels + Vec2::splat(12.0),
                )
            }),
        Prompt::Action(Step::Build(i, _)) => lot(i, true),
        Prompt::Action(Step::Tour(i, job)) => job
            .plan(town, chapter, i)
            .ok()
            .and_then(|route| stop(route.pickup)),
        Prompt::Action(step) => step
            .lot()
            .map_or_else(|| stop(Stop::Pier), |i| lot(i, false)),
        Prompt::Feedback(i) => i.map_or_else(|| stop(Stop::Quay), |i| lot(i, false)),
        Prompt::Delivered(_, to) => stop(to),
    };
    let frame = target
        .filter(|r| r.size().cmpgt(Vec2::splat(4.0)).all())
        .unwrap_or_else(|| {
            Rect::from_center_size(screen * Vec2::new(0.48, 0.51), Vec2::new(140.0, 112.0))
        });
    let margin = frame.size() * 0.5 + Vec2::splat(8.0);
    let center = frame.center().clamp(margin, (screen - margin).max(margin));
    let spot = Rect::from_center_size(center, frame.size());
    let viewport = Rect::from_corners(-origin, screen + Vec2::new(right, bottom));
    let shades = shade_rects(viewport, spot);
    for (piece, mut node, computed, outline) in &mut pieces {
        match piece {
            Piece::Shade(i) => place_box(&mut node, shades[*i]),
            Piece::Ring => {
                place_box(&mut node, spot);
                if let Some(mut outline) = outline {
                    outline.color =
                        Color::srgba(0.99, 0.80, 0.48, 0.26 + 0.10 * (game.clock * 4.0).sin());
                }
            }
            Piece::Note => {
                let size = (computed.size() / pixels).max(Vec2::new(364.0, 150.0));
                // On tablets and desktops the note keeps clear of the status panel on the
                // right (317 px wide, 22 px from the edge, see `screens.rs`).
                let room = if hud.phone {
                    screen
                } else {
                    Vec2::new((screen.x - 349.0).max(size.x + 24.0), screen.y)
                };
                let point = note_at(spot, size, room);
                let left = Val::Px(point.x);
                let top = Val::Px(point.y);
                if (node.left, node.top) != (left, top) {
                    node.left = left;
                    node.top = top;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spotlight_dims_the_screen_edges_past_the_safe_area() {
        let viewport = Rect::from_corners(Vec2::new(-43.0, -12.0), Vec2::new(800.0, 480.0));
        let spot = Rect::from_corners(Vec2::new(176.0, 148.0), Vec2::new(364.0, 288.0));
        let shades = shade_rects(viewport, spot);
        for point in [
            viewport.min + Vec2::splat(0.5),
            viewport.max - Vec2::splat(0.5),
            Vec2::new(viewport.min.x + 0.5, spot.center().y),
            Vec2::new(viewport.max.x - 0.5, spot.center().y),
        ] {
            assert_eq!(
                shades.iter().filter(|shade| shade.contains(point)).count(),
                1
            );
        }
        assert!(shades.iter().all(|shade| !shade.contains(spot.center())));
    }

    #[test]
    fn notes_fit_next_to_spots_near_each_viewport_edge() {
        let note = Vec2::new(364.0, 175.0);
        for screen in [Vec2::new(1158.0, 650.0), Vec2::new(1280.0, 960.0)] {
            for center in [
                Vec2::new(95.0, 81.0),
                screen - Vec2::new(95.0, 81.0),
                screen / 2.0,
            ] {
                let spot = Rect::from_center_size(center, Vec2::new(162.0, 126.0));
                let at = note_at(spot, note, screen);
                assert!(at.cmpge(Vec2::ZERO).all());
                assert!((at + note).cmple(screen).all());
                assert!(
                    at.x >= spot.max.x
                        || at.x + note.x <= spot.min.x
                        || at.y >= spot.max.y
                        || at.y + note.y <= spot.min.y
                );
            }
        }
    }
}

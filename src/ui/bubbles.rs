use crate::{
    model::Verdict,
    world::{Reactions, StreetCamera},
};

use super::{
    floating::{self, CaptionMeasure, SafeViewport},
    *,
};

const BUBBLES: usize = 4;
const FACE_ICON: f32 = 35.0;
const SERVICE_ICON: f32 = 27.0;
const NAME_FONT: f32 = 13.0;
const VERDICT_FONT: f32 = 16.0;
const BUBBLE_GAP: f32 = 5.0;
const BUBBLE_PAD: Vec2 = Vec2::new(6.0, 3.0);
const BUBBLE_TAIL: f32 = 7.0;

pub(super) fn bubble_size(name: Vec2, verdict: Vec2) -> Vec2 {
    Vec2::new(
        FACE_ICON + SERVICE_ICON + 2.0 * BUBBLE_GAP + name.x.max(verdict.x),
        FACE_ICON.max(name.y + verdict.y),
    ) + 2.0 * BUBBLE_PAD
        + Vec2::splat(4.0)
}

fn verdict_text(verdict: Verdict, favorite: bool) -> &'static str {
    if favorite {
        "Lieblingsangebot"
    } else {
        guests::short_verdict(verdict)
    }
}

#[derive(Component)]
pub struct Bubble {
    slot: usize,
    guest: Option<usize>,
}

#[derive(Component)]
pub struct BubbleIcon {
    slot: usize,
    face: bool,
}

#[derive(Component)]
pub struct BubbleText {
    slot: usize,
    result: bool,
}

pub(super) fn spawn(p: &mut ChildSpawnerCommands, hud: &Hud) {
    for slot in 0..BUBBLES {
        button_shell(
            p,
            Action::GuestReview(0),
            Style::Speech,
            true,
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::axes(Val::Px(BUBBLE_PAD.x), Val::Px(BUBBLE_PAD.y)),
                ..row(BUBBLE_GAP)
            },
        )
        .insert((Bubble { slot, guest: None }, Visibility::Hidden, ZIndex(-1)))
        .with_children(|bubble| {
            bubble.spawn((
                BubbleIcon { slot, face: true },
                ImageNode::new(hud.pictures["guest_happy"].clone()),
                Node {
                    width: Val::Px(FACE_ICON),
                    height: Val::Px(FACE_ICON),
                    flex_shrink: 0.0,
                    ..default()
                },
                FocusPolicy::Pass,
            ));
            bubble
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        flex_shrink: 0.0,
                        ..column(0.0)
                    },
                    FocusPolicy::Pass,
                ))
                .with_children(|message| {
                    for (result, size, font) in [(false, NAME_FONT, 0), (true, VERDICT_FONT, 1)] {
                        message.spawn((
                            BubbleText { slot, result },
                            Text::new(""),
                            TextColor(INK),
                            TextLayout::new_with_no_wrap(),
                            TextFont {
                                font: hud.fonts[font].clone(),
                                font_size: size,
                                ..default()
                            },
                            Node {
                                flex_shrink: 0.0,
                                ..default()
                            },
                            FocusPolicy::Pass,
                        ));
                    }
                });
            bubble.spawn((
                BubbleIcon { slot, face: false },
                ImageNode::new(hud.pictures["bread"].clone()),
                Node {
                    width: Val::Px(SERVICE_ICON),
                    height: Val::Px(SERVICE_ICON),
                    flex_shrink: 0.0,
                    ..default()
                },
                FocusPolicy::Pass,
            ));
            bubble.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(48.0),
                    bottom: Val::Px(-BUBBLE_TAIL),
                    width: Val::Px(7.0),
                    height: Val::Px(10.0),
                    ..default()
                },
                BackgroundColor(SAND),
                BorderRadius::bottom(Val::Px(4.0)),
                FocusPolicy::Pass,
            ));
        });
    }
}

#[derive(Clone, Copy)]
struct Placed {
    guest: usize,
    bounds: Rect,
    verdict: Verdict,
    age: f32,
}

type Bubbles<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Bubble,
        &'static mut Node,
        &'static mut Visibility,
        &'static mut ButtonAction,
    ),
>;

#[allow(clippy::too_many_arguments)]
pub fn update(
    game: Res<Game>,
    save: Res<Campaign>,
    world: Res<Reactions>,
    hud: Res<Hud>,
    scale: Res<UiScale>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<StreetCamera>>,
    mut badges: Bubbles,
    mut icons: Query<(&BubbleIcon, &mut ImageNode)>,
    mut texts: Query<(&BubbleText, &mut Text)>,
    markers: Query<&prompts::Marker>,
    mut measure: CaptionMeasure,
) {
    if badges.is_empty() {
        return;
    }
    let viewport = SafeViewport::new(&window, scale.0);
    let mut candidates = Vec::new();
    if game.view == View::Street
        && game.panel == Panel::None
        && game.job.is_none()
        && let Ok((camera, transform)) = camera.single()
        && let Some(town) = save.active_town()
    {
        let name_font = TextFont {
            font: hud.fonts[0].clone(),
            font_size: NAME_FONT,
            ..default()
        };
        let verdict_font = TextFont {
            font: hud.fonts[1].clone(),
            font_size: VERDICT_FONT,
            ..default()
        };
        let limits = Rect {
            min: Vec2::new(6.0, 114.0),
            max: viewport.size
                - Vec2::new(
                    if !hud.phone || game.quick_menu {
                        347.0
                    } else {
                        10.0
                    },
                    142.0,
                ),
        };
        for sign in &world.0 {
            let Ok(head) = camera.world_to_viewport(transform, sign.head) else {
                continue;
            };
            let anchor = viewport.point(head);
            if !anchor.cmpge(Vec2::ZERO).all() || !anchor.cmple(viewport.size).all() {
                continue;
            }
            let Some(guest) = town.guest_history().get(sign.guest) else {
                continue;
            };
            let Some(name) = measure.size(guest.wants.name(), &name_font, viewport.pixels_per_unit)
            else {
                continue;
            };
            let Some(verdict) = measure.size(
                verdict_text(sign.verdict, guest.specialty_bonus > 0),
                &verdict_font,
                viewport.pixels_per_unit,
            ) else {
                continue;
            };
            let Some(bounds) = floating::above(
                limits,
                anchor,
                bubble_size(name, verdict),
                BUBBLE_TAIL + 3.0,
            ) else {
                continue;
            };
            if markers
                .iter()
                .filter_map(|marker| marker.bounds)
                .any(|rect| floating::overlap(rect, bounds, Vec2::new(6.0, 10.0)))
            {
                continue;
            }
            candidates.push(Placed {
                guest: sign.guest,
                bounds,
                verdict: sign.verdict,
                age: sign.seconds,
            });
        }
    }
    candidates.sort_by(|a, b| {
        (a.verdict == Verdict::Pleased)
            .cmp(&(b.verdict == Verdict::Pleased))
            .then_with(|| a.age.total_cmp(&b.age))
    });

    let mut placed = [None; BUBBLES];
    for (badge, ..) in &badges {
        if let Some(pending) = candidates
            .iter()
            .copied()
            .find(|p| Some(p.guest) == badge.guest)
            && room(&placed, pending)
        {
            placed[badge.slot] = Some(pending);
        }
    }
    for pending in candidates {
        if room(&placed, pending) {
            let Some(free) = placed.iter_mut().find(|slot| slot.is_none()) else {
                break;
            };
            *free = Some(pending);
        }
    }
    for (mut badge, mut node, mut visible, mut shortcut) in &mut badges {
        if let Some(p) = placed[badge.slot] {
            if *visible != Visibility::Inherited {
                *visible = Visibility::Inherited;
            }
            floating::place_box(&mut node, p.bounds);
            if badge.guest != Some(p.guest) {
                badge.guest = Some(p.guest);
                shortcut.action = Action::GuestReview(p.guest);
                shortcut.disabled = false;
            }
        } else if *visible != Visibility::Hidden {
            *visible = Visibility::Hidden;
            badge.guest = None;
            shortcut.disabled = true;
        }
    }
    let Some(town) = save.active_town() else {
        return;
    };
    for (badge, mut icon) in &mut icons {
        if let Some(p) = placed[badge.slot]
            && let Some(guest) = town.guest_history().get(p.guest)
        {
            let name = if badge.face {
                if guest.specialty_bonus > 0 {
                    "star"
                } else {
                    guests::face(p.verdict)
                }
            } else {
                service_icon(guest.wants)
            };
            let sprite = &hud.pictures[name];
            if icon.image != *sprite {
                icon.image = sprite.clone();
            }
        }
    }
    for (badge, mut text) in &mut texts {
        if let Some(p) = placed[badge.slot]
            && let Some(guest) = town.guest_history().get(p.guest)
        {
            let words = if badge.result {
                verdict_text(p.verdict, guest.specialty_bonus > 0)
            } else {
                guest.wants.name()
            };
            if text.0 != words {
                text.0 = words.to_string();
            }
        }
    }
}

fn room(placed: &[Option<Placed>; BUBBLES], candidate: Placed) -> bool {
    placed.iter().flatten().all(|other| {
        candidate.guest != other.guest
            && !floating::overlap(other.bounds, candidate.bounds, Vec2::new(6.0, 13.0))
    })
}

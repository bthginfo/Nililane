use crate::{
    model::Verdict,
    world::{Reactions, StreetCamera},
};

use super::*;

const BUBBLES: usize = 4;
const WIDTH: f32 = 183.0;
const HEIGHT: f32 = 47.0;

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
                width: Val::Px(WIDTH),
                height: Val::Px(HEIGHT),
                padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                ..row(5.0)
            },
        )
        .insert((Bubble { slot, guest: None }, Visibility::Hidden, ZIndex(-1)))
        .with_children(|bubble| {
            bubble.spawn((
                BubbleIcon { slot, face: true },
                ImageNode::new(hud.pictures["guest_happy"].clone()),
                Node {
                    width: Val::Px(35.0),
                    height: Val::Px(35.0),
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
            bubble
                .spawn(Node {
                    flex_grow: 1.0,
                    ..column(0.0)
                })
                .with_children(|message| {
                    for (result, size, font) in [(false, 13.0, 0), (true, 16.0, 1)] {
                        message.spawn((
                            BubbleText { slot, result },
                            Text::new(""),
                            TextColor(INK),
                            TextFont {
                                font: hud.fonts[font].clone(),
                                font_size: size,
                                ..default()
                            },
                        ));
                    }
                });
            bubble.spawn((
                BubbleIcon { slot, face: false },
                ImageNode::new(hud.pictures["bread"].clone()),
                Node {
                    width: Val::Px(27.0),
                    height: Val::Px(27.0),
                    flex_shrink: 0.0,
                    ..default()
                },
            ));
            bubble.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(48.0),
                    bottom: Val::Px(-7.0),
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
    position: Vec2,
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
) {
    if badges.is_empty() {
        return;
    }
    let size = Vec2::new(window.width(), window.height()) / scale.0;
    let safe = insets().map(|n| f32::from(n) / (scale.0 * window.scale_factor()));
    let mut candidates = Vec::new();
    if game.view == View::Street
        && game.panel == Panel::None
        && game.job.is_none()
        && let Ok((camera, transform)) = camera.single()
    {
        for sign in &world.0 {
            let Ok(head) = camera.world_to_viewport(transform, sign.head) else {
                continue;
            };
            let pos = head / scale.0 - Vec2::new(WIDTH / 2.0, HEIGHT + 10.0);
            // Keep the badges clear of the day panel, the right rail and the footer.
            if pos.x < safe[0] + 6.0
                || pos.x + WIDTH > size.x - safe[2] - 347.0
                || pos.y < safe[1] + 114.0
                || pos.y + HEIGHT > size.y - safe[3] - 142.0
            {
                continue;
            }
            candidates.push(Placed {
                guest: sign.guest,
                position: pos,
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
            let left = Val::Px(p.position.x - safe[0]);
            let top = Val::Px(p.position.y - safe[1]);
            if (node.left, node.top) != (left, top) {
                node.left = left;
                node.top = top;
            }
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
                if guest.specialty_bonus > 0 {
                    "Lieblingsangebot"
                } else {
                    guests::short_verdict(p.verdict)
                }
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
        let apart = (other.position - candidate.position).abs();
        candidate.guest != other.guest && (apart.x > WIDTH + 6.0 || apart.y > HEIGHT + 13.0)
    })
}

// Phone dialogs keep their close button fixed while their content can be swiped.
use bevy::{
    ecs::system::SystemParam,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    ui::CalculatedClip,
};

use super::ButtonAction;
use crate::game::{Action, Game};

#[derive(Component)]
pub(super) struct ScrollPage;

#[derive(Clone, Copy)]
struct Swipe {
    id: u64,
    start: Vec2,
    last: Vec2,
    button: Option<Action>,
    moved: bool,
}

#[derive(SystemParam)]
#[allow(clippy::type_complexity)]
pub(crate) struct PanelTouch<'w, 's> {
    swipe: Local<'s, Option<Swipe>>,
    pages: Query<
        'w,
        's,
        (
            &'static ComputedNode,
            &'static GlobalTransform,
            &'static mut ScrollPosition,
        ),
        With<ScrollPage>,
    >,
    targets: Query<
        'w,
        's,
        (
            &'static ComputedNode,
            &'static GlobalTransform,
            &'static ButtonAction,
            &'static InheritedVisibility,
            Option<&'static CalculatedClip>,
        ),
    >,
    wheel: EventReader<'w, 's, MouseWheel>,
}

impl PanelTouch<'_, '_> {
    pub fn second_thumb(&self, window: &Window, touches: &Touches) -> Option<Action> {
        touches.iter_just_pressed().find_map(|touch| {
            let point = touch.start_position() * window.scale_factor();
            self.targets
                .iter()
                .filter(|(node, transform, button, visible, clip)| {
                    !node.is_empty()
                        && !button.disabled
                        && visible.get()
                        && clip.is_none_or(|clip| clip.clip.contains(point))
                        && (point - transform.translation().truncate())
                            .abs()
                            .cmple(node.size() * 0.5)
                            .all()
                })
                .max_by_key(|(node, ..)| node.stack_index())
                .map(|(_, _, button, _, _)| button.action)
        })
    }

    pub fn update(
        &mut self,
        game: &mut Game,
        window: &Window,
        touches: &Touches,
        pressed: Option<Action>,
    ) -> Option<Action> {
        let Ok((node, transform, mut scroll)) = self.pages.single_mut() else {
            self.swipe.take();
            self.wheel.clear();
            return pressed;
        };
        if !game.phone || !window.focused {
            self.swipe.take();
            self.wheel.clear();
            return pressed;
        }
        let to_physical = window.scale_factor();
        let to_ui = to_physical * node.inverse_scale_factor();
        let limit = (node.content_size().y - node.size().y).max(0.0) * node.inverse_scale_factor();
        let inside = |at: Vec2| {
            let point = at * to_physical - transform.translation().truncate();
            point.abs().cmple(node.size() * 0.5).all()
        };
        let wheel = self
            .wheel
            .read()
            .map(|event| {
                -event.y
                    * match event.unit {
                        MouseScrollUnit::Line => 46.0,
                        MouseScrollUnit::Pixel => to_ui,
                    }
            })
            .sum::<f32>();
        if wheel.abs() > 0.0 {
            scroll.offset_y = (scroll.offset_y + wheel).clamp(0.0, limit);
        }
        if self.swipe.is_none()
            && let Some(touch) = touches
                .iter_just_pressed()
                .find(|touch| inside(touch.start_position()))
        {
            let start = touch.start_position();
            *self.swipe = Some(Swipe {
                id: touch.id(),
                start,
                last: start,
                button: pressed,
                moved: false,
            });
        }
        let result = if let Some(mut swipe) = *self.swipe {
            if let Some(touch) = touches.get_pressed(swipe.id) {
                let at = touch.position();
                swipe.moved |= at.distance(swipe.start) * to_ui > 10.0;
                if swipe.moved {
                    scroll.offset_y =
                        (scroll.offset_y + (swipe.last.y - at.y) * to_ui).clamp(0.0, limit);
                    swipe.last = at;
                }
                *self.swipe = Some(swipe);
                None
            } else {
                self.swipe.take();
                touches.get_released(swipe.id).and_then(|touch| {
                    (!swipe.moved
                        && touch.position().distance(swipe.start) * to_ui <= 10.0
                        && inside(touch.position()))
                    .then_some(swipe.button)
                    .flatten()
                })
            }
        } else {
            pressed
        };
        game.scroll_y = scroll.offset_y;
        result
    }
}

#[cfg(test)]
mod tests {
    use bevy::input::touch::{TouchInput, TouchPhase, touch_screen_input_system};

    use super::*;
    use crate::ui::Style;

    #[derive(Resource, Default)]
    struct InputProbe {
        pressed: Option<Action>,
        accepted: Option<Action>,
        secondary: Option<Action>,
    }

    fn app() -> App {
        let mut app = App::new();
        let mut game = Game::default();
        game.phone = true;
        app.insert_resource(game)
            .init_resource::<InputProbe>()
            .init_resource::<Touches>()
            .add_event::<MouseWheel>()
            .add_event::<TouchInput>()
            .add_systems(PreUpdate, touch_screen_input_system)
            .add_systems(
                Update,
                |mut touch: PanelTouch,
                 mut game: ResMut<Game>,
                 mut probe: ResMut<InputProbe>,
                 fingers: Res<Touches>| {
                    let window = Window::default();
                    probe.accepted =
                        touch.update(&mut game, &window, &fingers, probe.pressed.take());
                    probe.secondary = touch.second_thumb(&window, &fingers);
                },
            );
        app
    }

    fn finger(app: &mut App, id: u64, phase: TouchPhase, x: f32, y: f32) {
        app.world_mut().send_event(TouchInput {
            id,
            phase,
            position: Vec2::new(x, y),
            force: None,
            window: Entity::PLACEHOLDER,
        });
        app.update();
    }

    #[test]
    fn swiping_over_a_button_scrolls_without_activating_it() {
        let mut app = app();
        app.world_mut().spawn((
            ScrollPage,
            ScrollPosition::default(),
            ComputedNode {
                size: Vec2::new(400.0, 300.0),
                content_size: Vec2::new(400.0, 600.0),
                inverse_scale_factor: 1.0,
                ..default()
            },
            GlobalTransform::from_xyz(250.0, 250.0, 0.0),
        ));
        app.world_mut().resource_mut::<InputProbe>().pressed = Some(Action::Plot);
        finger(&mut app, 0, TouchPhase::Started, 250.0, 360.0);
        finger(&mut app, 0, TouchPhase::Moved, 250.0, 200.0);
        finger(&mut app, 0, TouchPhase::Ended, 250.0, 200.0);
        assert_eq!(app.world().resource::<Game>().scroll_y, 160.0);
        assert_eq!(app.world().resource::<InputProbe>().accepted, None);

        app.world_mut().resource_mut::<InputProbe>().pressed = Some(Action::Plot);
        finger(&mut app, 0, TouchPhase::Started, 250.0, 200.0);
        finger(&mut app, 0, TouchPhase::Ended, 251.0, 202.0);
        assert_eq!(
            app.world().resource::<InputProbe>().accepted,
            Some(Action::Plot)
        );
    }

    #[test]
    fn second_thumb_selects_the_visible_action_while_the_first_stays_down() {
        let mut app = app();
        for (stack_index, clip, action) in [
            (1, None, Action::Plot),
            (
                2,
                Some(CalculatedClip {
                    clip: Rect::new(500.0, 100.0, 580.0, 160.0),
                }),
                Action::Pause,
            ),
        ] {
            let mut button = app.world_mut().spawn((
                ComputedNode {
                    stack_index,
                    size: Vec2::new(80.0, 60.0),
                    ..default()
                },
                GlobalTransform::from_xyz(550.0, 460.0, 0.0),
                InheritedVisibility::VISIBLE,
                ButtonAction {
                    action,
                    disabled: false,
                    style: Style::Primary,
                },
            ));
            if let Some(clip) = clip {
                button.insert(clip);
            }
        }
        finger(&mut app, 0, TouchPhase::Started, 70.0, 460.0);
        assert_eq!(app.world().resource::<InputProbe>().secondary, None);
        finger(&mut app, 1, TouchPhase::Started, 550.0, 460.0);
        assert!(app.world().resource::<Touches>().get_pressed(0).is_some());
        assert_eq!(
            app.world().resource::<InputProbe>().secondary,
            Some(Action::Plot)
        );
    }
}

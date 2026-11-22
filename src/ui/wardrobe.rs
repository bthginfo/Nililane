use super::*;
use crate::cosmetics::{Category, Cosmetic};

pub(super) fn draw(p: &mut ChildSpawnerCommands, hud: &Hud, save: &Campaign, active: Category) {
    let stars = save.available_stars();
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::left(Val::Px(20.0)),
            align_items: AlignItems::Center,
            ..row(10.0)
        },
        BackgroundColor(Color::srgba(0.024, 0.090, 0.10, 0.25)),
        FocusPolicy::Block,
    ))
    .with_children(|root| {
        let mut node = frame_node(Val::Percent(61.0), 10.0, 18.0);
        node.max_width = Val::Px(755.0);
        card(root, node, INK).with_children(|closet| {
            closet
                .spawn(Node {
                    justify_content: JustifyContent::SpaceBetween,
                    ..row(6.0)
                })
                .with_children(|head| {
                    label(head, hud, "Nilis Garderobe", 35.0, Face::Serif, SAND);
                    panels::metric(head, hud, "star", format!("{stars} Sterne"));
                });
            line(closet, LINE);
            closet.spawn(row(6.0)).with_children(|tabs| {
                for category in Category::ALL {
                    button(
                        tabs,
                        hud,
                        Action::WardrobeTab(category),
                        category.label(),
                        category.icon(),
                        "",
                        if category == active {
                            Style::Primary
                        } else {
                            Style::Quiet
                        },
                        false,
                    );
                }
            });
            let items: Vec<_> = Cosmetic::ALL
                .into_iter()
                .filter(|item| item.category() == active)
                .collect();
            for items in items.chunks(4) {
                closet
                    .spawn(Node {
                        align_items: AlignItems::Stretch,
                        ..row(7.0)
                    })
                    .with_children(|row| {
                        for &item in items {
                            let wearing = save.wardrobe.wearing(item);
                            let owned = save.wardrobe.owns(item);
                            button_shell(
                                row,
                                Action::Wear(item),
                                if wearing {
                                    Style::Selected
                                } else {
                                    Style::Quiet
                                },
                                !owned && item.cost() > stars,
                                Node {
                                    flex_grow: 1.0,
                                    flex_basis: Val::Px(0.0),
                                    min_width: Val::Px(0.0),
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    ..column(4.0)
                                },
                            )
                            .with_children(|tile| {
                                icon(tile, hud, item.icon(), 49.0);
                                label(tile, hud, item.name(), 15.0, Face::Bold, SAND);
                                let (art, cost) = if wearing {
                                    ("check", "Aktiv".to_owned())
                                } else if owned {
                                    ("", "Anziehen".to_owned())
                                } else {
                                    ("star", item.cost().to_string())
                                };
                                panels::metric(tile, hud, art, cost);
                            });
                        }
                    });
            }
            button(
                closet,
                hud,
                Action::Back,
                "Zurück",
                "",
                "B",
                Style::Quiet,
                false,
            );
        });
    });
}

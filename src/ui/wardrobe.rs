use super::*;
use crate::{
    campaign::Region,
    cosmetics::{Category, Cosmetic},
};

pub(super) fn draw(p: &mut ChildSpawnerCommands, hud: &Hud, save: &Campaign, game: &Game) {
    let stars = save.available_stars();
    p.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::left(Val::Px(20.0)),
            align_items: AlignItems::Center,
            ..row(10.0)
        },
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
            closet
                .spawn(Node {
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(6.0),
                    ..row(5.0)
                })
                .with_children(|tabs| {
                    for (region, title, art) in [
                        (None, "Klassiker", ""),
                        (Some(Region::Greece), "Griechenland", "ferry"),
                        (Some(Region::Rome), "Rom", "forum"),
                        (Some(Region::Japan), "Japan", "temple"),
                        (Some(Region::Nordic), "Norden", "boatyard"),
                        (Some(Region::Oasis), "Oasen", "oasis_home"),
                    ] {
                        button_shell(
                            tabs,
                            Action::WardrobeRegion(region),
                            if game.wardrobe_region == region {
                                Style::Selected
                            } else {
                                Style::Quiet
                            },
                            false,
                            Node {
                                min_height: Val::Px(46.0),
                                padding: UiRect::horizontal(Val::Px(8.0)),
                                justify_content: JustifyContent::Center,
                                ..row(5.0)
                            },
                        )
                        .with_children(|tab| {
                            if !art.is_empty() {
                                icon(tab, hud, art, 21.0);
                            }
                            label(tab, hud, title, 16.0, Face::Bold, SAND);
                        });
                    }
                });
            closet.spawn(row(6.0)).with_children(|tabs| {
                key(tabs, hud, "LB", false);
                for category in Category::ALL {
                    button(
                        tabs,
                        hud,
                        Action::WardrobeTab(category),
                        category.label(),
                        category.icon(),
                        "",
                        if category == game.wardrobe_tab {
                            Style::TabOn
                        } else {
                            Style::Tab
                        },
                        false,
                    );
                }
                key(tabs, hud, "RB", false);
            });
            let items: Vec<_> = Cosmetic::ALL
                .into_iter()
                .filter(|item| {
                    item.category() == game.wardrobe_tab && item.region() == game.wardrobe_region
                })
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
                            let locked = !save.cosmetic_available(item);
                            button_shell(
                                row,
                                Action::Wear(item),
                                if wearing {
                                    Style::Selected
                                } else {
                                    Style::Quiet
                                },
                                locked || (!owned && item.cost() > stars),
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
                                if let Some(island) = item.requires_chapter().filter(|_| locked) {
                                    tile.spawn(super::row(3.0)).with_children(|r| {
                                        icon(r, hud, "lock", 15.0);
                                        label(
                                            r,
                                            hud,
                                            CHAPTERS[island].district,
                                            14.0,
                                            Face::Bold,
                                            MUTED,
                                        );
                                    });
                                    if let Some(index) = item.requires_adventure() {
                                        label(
                                            tile,
                                            hud,
                                            format!(
                                                "oder {}",
                                                crate::adventure::Level::at(index).name
                                            ),
                                            13.0,
                                            Face::Body,
                                            MUTED,
                                        );
                                    }
                                }
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

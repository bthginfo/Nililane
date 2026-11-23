use bevy::{
    ecs::system::SystemParam,
    text::{ComputedTextBlock, CosmicFontSystem, TextPipeline},
};

use super::*;

pub(super) struct SafeViewport {
    pub origin: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub pixels_per_unit: f32,
}

impl SafeViewport {
    pub fn new(window: &Window, scale: f32) -> Self {
        let pixels_per_unit = window.scale_factor() * scale;
        let [left, top, right, bottom] =
            insets().map(|pixel| f32::from(pixel) / pixels_per_unit.max(0.2));
        let origin = Vec2::new(left, top);
        let size =
            Vec2::new(window.width(), window.height()) / scale - origin - Vec2::new(right, bottom);
        Self {
            origin,
            size,
            scale,
            pixels_per_unit,
        }
    }

    pub fn point(&self, viewport_point: Vec2) -> Vec2 {
        viewport_point / self.scale - self.origin
    }
}

pub(super) fn above(bounds: Rect, anchor: Vec2, size: Vec2, gap: f32) -> Option<Rect> {
    let top_left = anchor - Vec2::new(size.x * 0.5, size.y + gap);
    fit(bounds, top_left, size)
}

pub(super) fn fit(bounds: Rect, top_left: Vec2, size: Vec2) -> Option<Rect> {
    let max = bounds.max - size;
    if !top_left.is_finite() || !size.cmpgt(Vec2::ZERO).all() || !max.cmpge(bounds.min).all() {
        return None;
    }
    let min = top_left.clamp(bounds.min, max);
    Some(Rect {
        min,
        max: min + size,
    })
}

pub(super) fn overlap(a: Rect, b: Rect, gap: Vec2) -> bool {
    (a.min - gap).cmplt(b.max).all() && (a.max + gap).cmpgt(b.min).all()
}

pub(super) fn place_box(node: &mut Node, rect: Rect) {
    let [left, top, width, height] =
        [rect.min.x, rect.min.y, rect.width(), rect.height()].map(Val::Px);
    if (node.left, node.top, node.width, node.height) != (left, top, width, height) {
        (node.left, node.top, node.width, node.height) = (left, top, width, height);
    }
}

#[derive(Default)]
struct WidthCache {
    scale: f32,
    sizes: HashMap<(AssetId<Font>, u32), HashMap<String, Vec2>>,
    block: ComputedTextBlock,
}

#[derive(SystemParam)]
pub(crate) struct CaptionMeasure<'w, 's> {
    fonts: Res<'w, Assets<Font>>,
    pipeline: ResMut<'w, TextPipeline>,
    font_system: ResMut<'w, CosmicFontSystem>,
    cache: Local<'s, WidthCache>,
}

impl CaptionMeasure<'_, '_> {
    pub fn size(&mut self, text: &str, font: &TextFont, scale: f32) -> Option<Vec2> {
        let cache = &mut *self.cache;
        if scale != cache.scale {
            cache.scale = scale;
            cache.sizes.clear();
        }
        let sizes = cache
            .sizes
            .entry((font.font.id(), font.font_size.to_bits()))
            .or_default();
        if let Some(&size) = sizes.get(text) {
            return Some(size);
        }
        let size = caption_size(
            &self.fonts,
            &mut self.pipeline,
            &mut self.font_system,
            &mut cache.block,
            text,
            font,
            scale,
        )?;
        if sizes.len() == 256 {
            sizes.clear();
        }
        sizes.insert(text.to_owned(), size);
        Some(size)
    }
}

pub(super) fn caption_size(
    fonts: &Assets<Font>,
    pipeline: &mut TextPipeline,
    font_system: &mut CosmicFontSystem,
    block: &mut ComputedTextBlock,
    text: &str,
    font: &TextFont,
    scale: f32,
) -> Option<Vec2> {
    let text = pipeline
        .create_text_measure(
            Entity::PLACEHOLDER,
            fonts,
            std::iter::once((Entity::PLACEHOLDER, 0, text, font, Color::WHITE)),
            f64::from(scale),
            &TextLayout::new_with_no_wrap(),
            block,
            font_system,
        )
        .ok()?;
    // Round outwards in physical pixels so a fractional UI scale cannot trim a glyph.
    Some(((text.max.ceil() + Vec2::splat(2.0)) / scale).ceil())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_and_guest_text_fit_using_the_game_fonts_at_both_ui_scales() {
        let mut fonts = Assets::<Font>::default();
        let load = |bytes: &[u8]| Font::try_from_bytes(bytes.to_vec()).unwrap();
        let medium = fonts.add(load(include_bytes!("../../assets/fonts/DMSans-Medium.ttf")));
        let bold = fonts.add(load(include_bytes!("../../assets/fonts/DMSans-Bold.ttf")));
        let mut pipeline = TextPipeline::default();
        let mut font_system = CosmicFontSystem::default();
        let mut block = ComputedTextBlock::default();

        for (scale, viewport) in [
            (0.625, Vec2::new(1232.0, 945.0)),
            (0.923, Vec2::new(1364.0, 626.0)),
        ] {
            let mut measure = |text: &str, handle, font_size| {
                let font = TextFont {
                    font: handle,
                    font_size,
                    ..default()
                };
                caption_size(
                    &fonts,
                    &mut pipeline,
                    &mut font_system,
                    &mut block,
                    text,
                    &font,
                    scale,
                )
                .unwrap()
            };
            let bounds = Rect {
                min: Vec2::new(8.0, 109.0),
                max: viewport - Vec2::new(349.0, 145.0),
            };
            for service in Service::ALL {
                for (amount, capacity) in [(0, 1), (1, 9), (18, 22), (u16::MAX, u16::MAX)] {
                    let caption = prompts::stock_caption(service, amount, capacity);
                    let text = measure(&caption, bold.clone(), 15.5);
                    let size = prompts::marker_size(text);
                    if service == Service::Meze && amount == 18 {
                        assert_eq!(caption, "18/22 Portionen Meze");
                        assert!(
                            size.x > 145.0,
                            "the previous fixed width truncated this caption"
                        );
                    }
                    let rect = above(bounds, viewport, size, 10.0).unwrap();
                    assert!(rect.max.cmple(bounds.max).all());
                    assert!(rect.width() >= text.x + 49.0);
                }
                let name = measure(service.name(), medium.clone(), 13.0);
                for verdict in [
                    "Zufrieden",
                    "Ausverkauft",
                    "Zu teuer",
                    "Zu weit",
                    "Fehlt",
                    "Lieblingsangebot",
                ] {
                    let verdict = measure(verdict, bold.clone(), 16.0);
                    let size = bubbles::bubble_size(name, verdict);
                    let rect = above(bounds, Vec2::ZERO, size, 10.0).unwrap();
                    assert!(rect.min.cmpge(bounds.min).all());
                    assert!(rect.width() >= name.x.max(verdict.x) + 87.0);
                    assert!(rect.height() >= name.y + verdict.y + 9.0);
                }
            }
        }
    }

    #[test]
    fn wide_badges_keep_their_full_size_next_to_every_edge() {
        for screen in [Vec2::new(1280.0, 960.0), Vec2::new(1388.0, 650.0)] {
            let bounds = Rect {
                min: Vec2::new(21.0, 43.0),
                max: screen - Vec2::new(45.0, 28.0),
            };
            let size = Vec2::new(260.0, 51.0);
            for anchor in [
                Vec2::ZERO,
                screen,
                Vec2::new(0.0, screen.y),
                Vec2::new(screen.x, 0.0),
            ] {
                let rect = above(bounds, anchor, size, 10.0).unwrap();
                assert_eq!(rect.size(), size);
                assert!(rect.min.cmpge(bounds.min).all());
                assert!(rect.max.cmple(bounds.max).all());
            }
        }
    }
}

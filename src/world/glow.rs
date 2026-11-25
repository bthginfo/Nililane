//! Soft light halos for flames and lit doorways at night.
//!
//! The halos are camera-facing quads with a radial gradient that add their light
//! on top of the scene. They need no HDR target or bloom pass, so they look the
//! same on desktop, tablets and phones.

use bevy::{
    asset::RenderAssetUsages,
    pbr::NotShadowCaster,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::StreetCamera;

#[derive(Resource, Clone)]
pub(crate) struct Glow {
    pub(crate) quad: Handle<Mesh>,
    pub(crate) warm: Handle<StandardMaterial>,
}

#[derive(Component)]
pub(crate) struct Halo {
    pub(crate) size: f32,
    pub(crate) phase: f32,
    /// 0 hides the halo, 1 shows it at full size.
    pub(crate) strength: f32,
}

impl Halo {
    pub(crate) fn new(size: f32, phase: f32) -> Self {
        Self {
            size,
            phase,
            strength: 1.0,
        }
    }
}

pub(crate) fn prepare(
    meshes: &mut Assets<Mesh>,
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
) -> Glow {
    const SIZE: u32 = 64;
    let mut pixels = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let d = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) / SIZE as f32 * 2.0 - Vec2::ONE;
            let r = d.length().min(1.0);
            // A bright core that falls off softly towards the rim.
            let core = (1.0 - r).powi(3);
            let falloff = (1.0 - r * r).max(0.0).powi(2) * 0.55 + core * 0.45;
            pixels.extend_from_slice(&[255, 255, 255, (falloff * 255.0) as u8]);
        }
    }
    let image = Image::new(
        Extent3d {
            width: SIZE,
            height: SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    Glow {
        quad: meshes.add(Rectangle::new(1.0, 1.0)),
        warm: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.70, 0.36, 0.62),
            base_color_texture: Some(images.add(image)),
            unlit: true,
            alpha_mode: AlphaMode::Add,
            fog_enabled: false,
            ..default()
        }),
    }
}

pub(crate) fn halo(glow: &Glow, at: Vec3, size: f32, phase: f32) -> impl Bundle {
    (
        Halo::new(size, phase),
        Mesh3d(glow.quad.clone()),
        MeshMaterial3d(glow.warm.clone()),
        Transform::from_translation(at).with_scale(Vec3::splat(size)),
        NotShadowCaster,
    )
}

/// Turns every halo towards the camera and lets the flames breathe a little.
pub(crate) fn animate(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<StreetCamera>>,
    parents: Query<&GlobalTransform, Without<Halo>>,
    mut halos: Query<(&Halo, &mut Transform, Option<&ChildOf>)>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let facing = camera.compute_transform().rotation;
    let t = time.elapsed_secs();
    for (halo, mut transform, parent) in &mut halos {
        let parent_rotation = parent
            .and_then(|parent| parents.get(parent.parent()).ok())
            .map_or(Quat::IDENTITY, |global| global.compute_transform().rotation);
        transform.rotation = parent_rotation.inverse() * facing;
        let flicker =
            1.0 + (t * 3.1 + halo.phase).sin() * 0.045 + (t * 7.3 + halo.phase * 1.7).sin() * 0.025;
        transform.scale = Vec3::splat(halo.size * flicker * halo.strength.clamp(0.0, 1.0));
    }
}

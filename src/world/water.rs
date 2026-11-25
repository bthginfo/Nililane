//! Animated low-poly sea shared by the harbors and the adventure islands.

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

use crate::model::Weather;

pub(crate) const SHADER: &str = "embedded://shaders/water.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub(crate) struct Water {
    #[uniform(0)]
    pub(crate) settings: Settings,
}

pub(crate) use uniform::Settings;

// The `ShaderType` derive emits per-field layout checks that rustc reports as unused.
#[allow(dead_code)]
mod uniform {
    use bevy::{prelude::*, render::render_resource::ShaderType};

    #[derive(Clone, Copy, Debug, ShaderType, PartialEq)]
    pub(crate) struct Settings {
        /// Colour multiplier for the vertex colours; alpha is the overall brightness.
        pub(crate) tint: Vec4,
        /// Colour of sun glints and sparkles; alpha is the glint strength.
        pub(crate) glint: Vec4,
        /// Facet tilt, animation speed, sparkle amount and swell height.
        pub(crate) motion: Vec4,
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tint: Vec4::ONE,
            glint: Vec4::new(1.0, 0.97, 0.88, 1.0),
            motion: Vec4::new(0.20, 1.0, 1.0, 0.0),
        }
    }
}

impl Settings {
    /// Calm daylight sea with the given colour multiplier.
    pub(crate) fn calm(tint: Vec3, swell: f32) -> Self {
        Self {
            tint: tint.extend(1.0),
            motion: Vec4::new(0.20, 1.0, 1.0, swell),
            ..default()
        }
    }

    /// Adjusts the motion and glitter to the weather and the time of day.
    /// `night` is 0 by day and 1 under a full night sky.
    pub(crate) fn weather(mut self, weather: (Weather, Weather, f32), night: f32) -> Self {
        let rough = |weather| match weather {
            Weather::Storm => (0.36, 1.85, 0.25),
            Weather::Rain => (0.27, 1.35, 0.35),
            Weather::Wind => (0.30, 1.55, 0.85),
            Weather::Cloud => (0.20, 1.0, 0.45),
            Weather::Heat => (0.15, 0.75, 1.25),
            _ => (0.20, 1.0, 1.0),
        };
        let (a, b) = (rough(weather.0), rough(weather.1));
        let t = weather.2.clamp(0.0, 1.0);
        let mix = |x: f32, y: f32| x + (y - x) * t;
        self.motion.x = mix(a.0, b.0);
        self.motion.y = mix(a.1, b.1);
        let sparkle = mix(a.2, b.2);
        let night = night.clamp(0.0, 1.0);
        // By night the moon draws fewer, colder glints on the water.
        self.motion.z = sparkle * (1.0 - night * 0.55);
        let warm = Vec3::new(1.0, 0.96, 0.86);
        let moon = Vec3::new(0.70, 0.82, 1.0);
        self.glint = warm.lerp(moon, night).extend(1.0 - night * 0.45);
        self
    }
}

impl Material for Water {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

pub(crate) fn plugin() -> MaterialPlugin<Water> {
    MaterialPlugin::<Water> {
        prepass_enabled: false,
        shadows_enabled: false,
        ..default()
    }
}

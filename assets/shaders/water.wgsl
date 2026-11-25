// Stylized low-poly sea for Nililane.
//
// Every triangle of the sea keeps one animated facet normal (flat interpolation),
// so the faceted water catches the sun like hammered glass while the surface itself
// only swells a few centimetres. Sparkles, a soft depth tint from the vertex colours
// and the scene fog finish the look. Works on desktop and on Android Vulkan 1.1.

#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
    mesh_view_bindings::{view, globals, lights},
}
#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping::tone_mapping
#endif
#ifdef DISTANCE_FOG
#import bevy_pbr::{mesh_view_bindings::fog, pbr_functions::apply_fog}
#endif

struct WaterSettings {
    // rgb: colour multiplier for the vertex colours, a: overall brightness
    tint: vec4<f32>,
    // rgb: colour of sun glints and sparkles, a: glint strength
    glint: vec4<f32>,
    // x: facet tilt, y: animation speed, z: sparkle amount, w: swell height
    motion: vec4<f32>,
};

@group(2) @binding(0) var<uniform> water: WaterSettings;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) facet: vec3<f32>,
};

fn wave(p: vec2<f32>, t: f32, direction: vec2<f32>, number: f32, speed: f32, height: f32) -> vec3<f32> {
    let phase = dot(direction, p) * number + t * speed;
    let slope = cos(phase) * height * number;
    return vec3<f32>(sin(phase) * height, slope * direction.x, slope * direction.y);
}

// Height and slope of three travelling swells.
fn swell(p: vec2<f32>, t: f32) -> vec3<f32> {
    return wave(p, t, vec2<f32>(0.80, 0.60), 0.55, 1.05, 1.00)
        + wave(p, t, vec2<f32>(-0.45, 0.89), 0.93, 1.55, 0.62)
        + wave(p, t, vec2<f32>(0.95, -0.31), 1.73, 2.35, 0.31);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var world = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(vertex.position, 1.0),
    ).xyz;
    let t = globals.time * water.motion.y;
    let s = swell(world.xz, t);
    world.y += s.x * water.motion.w;
    out.world_position = world;
    out.clip_position = position_world_to_clip(world);
    out.color = vertex.color;
    out.facet = normalize(vec3<f32>(-s.y * water.motion.x, 1.0, -s.z * water.motion.x));
    return out;
}

fn hash(p: vec2<f32>) -> f32 {
    let q = vec2<u32>(vec2<i32>(p) + vec2<i32>(4096));
    var h = q.x * 1597334677u ^ q.y * 3812015801u;
    h = h * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    return f32((h >> 22u) ^ h) / 4294967295.0;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = in.facet;
    var to_sun = normalize(vec3<f32>(0.35, 0.80, 0.45));
    var sun_hue = vec3<f32>(1.0);
    if lights.n_directional_lights > 0u {
        let sun = lights.directional_lights[0];
        to_sun = sun.direction_to_light;
        sun_hue = sun.color.rgb / max(max(sun.color.r, sun.color.g), max(sun.color.b, 1e-4));
    }
    // Orthographic camera: the view direction is the same for every fragment.
    let to_eye = normalize(view.world_from_view[2].xyz);

    let light = 0.82 + 0.30 * clamp(dot(n, to_sun), 0.0, 1.0);
    var color = in.color.rgb * water.tint.rgb * light;

    // A sky reflection lifts facets that tilt towards the viewer.
    let fresnel = pow(1.0 - clamp(dot(n, to_eye), 0.0, 1.0), 3.0);
    color += vec3<f32>(0.10, 0.16, 0.20) * fresnel * water.tint.a;

    // Sun glitter on the facets that mirror the light into the camera.
    let half_vector = normalize(to_sun + to_eye);
    let mirror = clamp(dot(n, half_vector), 0.0, 1.0);
    let glitter = pow(mirror, 140.0) * 1.6 + pow(mirror, 28.0) * 0.12;
    color += water.glint.rgb * sun_hue * glitter * water.glint.a;

    // Tiny twinkles that come and go across the surface.
    let q = in.world_position.xz * 1.45;
    let cell = floor(q);
    let seed = hash(cell);
    let offset = vec2<f32>(hash(cell + vec2<f32>(17.0, 3.0)), hash(cell + vec2<f32>(5.0, 29.0)));
    let spot = fract(q) - 0.25 - offset * 0.5;
    let life = fract(globals.time * (0.18 + seed * 0.27) + seed * 11.0);
    let flash = clamp(1.0 - abs(life - 0.5) * 7.0, 0.0, 1.0);
    let dot_size = 1.0 - smoothstep(0.012, 0.055, length(spot * vec2<f32>(1.0, 1.6)));
    let sparkle = step(0.62, seed) * flash * dot_size * water.motion.z;
    color += water.glint.rgb * sparkle;

    color *= water.tint.a;
    var output = vec4<f32>(color, 1.0);
#ifdef DISTANCE_FOG
    output = apply_fog(fog, output, in.world_position, view.world_position.xyz);
#endif
#ifdef TONEMAP_IN_SHADER
    output = tone_mapping(output, view.color_grading);
#endif
    return output;
}

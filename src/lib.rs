mod campaign;
mod controls;
mod errands;
mod game;
mod layout;
mod model;
mod save;
mod ui;
mod world;

use bevy::{
    asset::io::embedded::EmbeddedAssetRegistry,
    prelude::*,
    window::{PresentMode, WindowResolution},
};

use crate::{
    controls::Controls,
    game::{Game, frame},
    save::Campaign,
    ui::Hud,
};

#[cfg(target_os = "android")]
#[bevy::prelude::bevy_main]
fn main() {
    run();
}

pub fn run() {
    let mut app = App::new();
    let plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Nililane".into(),
                resolution: WindowResolution::new(1280.0, 800.0),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_linear());
    #[cfg(target_os = "android")]
    let plugins = plugins
        .set(android_renderer())
        .add_after::<bevy::render::RenderPlugin>(AndroidMeshPreparation)
        .set(bevy::pbr::PbrPlugin {
            use_gpu_instance_buffer_builder: false,
            ..default()
        });
    app.add_plugins(plugins);

    embed_assets(&app);
    app.insert_resource(ClearColor(Color::srgb_u8(119, 176, 178)))
        .insert_resource(Campaign::load())
        .init_resource::<Game>()
        .init_resource::<Controls>()
        .init_resource::<Hud>()
        .init_resource::<world::Reactions>()
        .insert_resource(AmbientLight {
            color: Color::srgb_u8(231, 230, 252),
            brightness: 245.0,
            ..default()
        })
        .add_systems(Startup, (ui::setup, world::setup, world::setup_weather))
        .add_systems(
            Update,
            (
                controls::poll,
                frame,
                ui::draw,
                ui::refresh,
                ui::refresh_guest_summary,
                game::audio,
                world::sync,
                world::animate,
                world::animate_people,
                ui::update_reactions,
                world::animate_construction,
                world::animate_weather,
            )
                .chain(),
        );
    app.run();
}

#[cfg(target_os = "android")]
fn android_renderer() -> bevy::render::RenderPlugin {
    use bevy::render::settings::{WgpuFeatures, WgpuSettings, WgpuSettingsPriority};
    bevy::render::RenderPlugin {
        render_creation: WgpuSettings {
            priority: WgpuSettingsPriority::Compatibility,
            features: WgpuFeatures::empty(),
            ..default()
        }
        .into(),
        synchronous_pipeline_compilation: true,
        ..default()
    }
}

#[cfg(target_os = "android")]
struct AndroidMeshPreparation;

#[cfg(target_os = "android")]
impl Plugin for AndroidMeshPreparation {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        use bevy::render::{
            RenderApp,
            batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport},
        };
        if let Some(renderer) = app.get_sub_app_mut(RenderApp) {
            renderer.insert_resource(GpuPreprocessingSupport {
                max_supported_mode: GpuPreprocessingMode::None,
            });
        }
    }
}

fn embed_assets(app: &App) {
    use std::path::Path;

    let registry = app.world().resource::<EmbeddedAssetRegistry>();
    macro_rules! add {
        ($($asset:literal),* $(,)?) => {
            $(registry.insert_asset(Path::new($asset).into(), Path::new($asset),
                include_bytes!(concat!("../assets/", $asset)).as_slice());)*
        };
    }
    add!(
        "fonts/DMSans-Bold.ttf",
        "fonts/DMSans-Medium.ttf",
        "fonts/DMSerifText-Regular.ttf",
        "models/hippo.glb",
        "models/villages/aegean.glb",
        "models/works/aegean.glb",
        "models/waldwinkel/wood_tree.glb",
        "models/waldwinkel/resource_rock.glb",
        "audio/aegean.ogg",
        "audio/roman_quay.ogg",
        "audio/harbor_rain.ogg",
        "audio/ui_choose.wav",
        "audio/ui_blocked.wav",
        "audio/ui_back.wav",
        "audio/chapter_chime.wav",
        "audio/quest_accepted.wav",
        "audio/build_place.wav",
        "audio/tier_assembly.wav",
        "audio/market_sale.wav",
        "audio/ferry_bell.wav",
        "audio/nili_aid.wav",
        "audio/shore_loop.wav",
        "ui/aqueduct.png",
        "ui/bakery.png",
        "ui/baths.png",
        "ui/bed.png",
        "ui/book.png",
        "ui/bread.png",
        "ui/chart.png",
        "ui/check.png",
        "ui/cloud.png",
        "ui/coin.png",
        "ui/ferry.png",
        "ui/forum.png",
        "ui/grove.png",
        "ui/guide.png",
        "ui/guest_book.png",
        "ui/guest_down.png",
        "ui/guest_empty.png",
        "ui/guest_happy.png",
        "ui/guest_mixed.png",
        "ui/guest_steps.png",
        "ui/guest_wait.png",
        "ui/hammer.png",
        "ui/house.png",
        "ui/inn.png",
        "ui/island_0.png",
        "ui/island_1.png",
        "ui/island_10.png",
        "ui/island_11.png",
        "ui/island_12.png",
        "ui/island_13.png",
        "ui/island_14.png",
        "ui/island_15.png",
        "ui/island_16.png",
        "ui/island_17.png",
        "ui/island_18.png",
        "ui/island_19.png",
        "ui/island_2.png",
        "ui/island_20.png",
        "ui/island_21.png",
        "ui/island_22.png",
        "ui/island_23.png",
        "ui/island_24.png",
        "ui/island_25.png",
        "ui/island_26.png",
        "ui/island_27.png",
        "ui/island_28.png",
        "ui/island_29.png",
        "ui/island_3.png",
        "ui/island_30.png",
        "ui/island_31.png",
        "ui/island_32.png",
        "ui/island_33.png",
        "ui/island_34.png",
        "ui/island_35.png",
        "ui/island_36.png",
        "ui/island_37.png",
        "ui/island_38.png",
        "ui/island_39.png",
        "ui/island_4.png",
        "ui/island_40.png",
        "ui/island_41.png",
        "ui/island_42.png",
        "ui/island_43.png",
        "ui/island_44.png",
        "ui/island_45.png",
        "ui/island_46.png",
        "ui/island_47.png",
        "ui/island_5.png",
        "ui/island_6.png",
        "ui/island_7.png",
        "ui/island_8.png",
        "ui/island_9.png",
        "ui/lock.png",
        "ui/meze.png",
        "ui/mood.png",
        "ui/nili.png",
        "ui/olive.png",
        "ui/potter.png",
        "ui/press.png",
        "ui/rain.png",
        "ui/shrine.png",
        "ui/star.png",
        "ui/stoa.png",
        "ui/storehouse.png",
        "ui/storm.png",
        "ui/sun.png",
        "ui/taverna.png",
        "ui/title_fade.png",
        "ui/villa.png",
        "ui/water.png",
        "ui/well.png",
        "ui/wind.png",
    );
}

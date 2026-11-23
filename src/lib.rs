mod adventure;
mod campaign;
mod campaign_orders;
mod caravan;
mod coach;
mod controls;
mod cosmetics;
mod errands;
mod game;
mod layout;
mod model;
mod save;
mod street;
mod trade;
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
                present_mode: PresentMode::Fifo,
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
    let mut campaign = Campaign::load();
    let adventure = adventure::Adventure::default();
    if adventure.restore_rewards(&mut campaign)
        && let Err(error) = campaign.save()
    {
        warn!("Nililane kann die Abenteuersterne noch nicht speichern: {error}");
    }
    app.insert_resource(ClearColor(Color::srgb_u8(119, 176, 178)))
        .insert_resource(campaign)
        .init_resource::<Game>()
        .insert_resource(adventure)
        .init_resource::<world::AdventureScene>()
        .init_resource::<Controls>()
        .init_resource::<Hud>()
        .init_resource::<world::Reactions>()
        .insert_resource(AmbientLight {
            color: Color::srgb_u8(231, 230, 252),
            brightness: 245.0,
            ..default()
        })
        .add_systems(
            Startup,
            (
                ui::setup,
                world::setup,
                world::setup_weather,
                world::setup_pets,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    controls::poll,
                    frame,
                    ui::draw,
                    ui::refresh,
                    ui::animate_walk_pad,
                    ui::refresh_guest_summary,
                    game::audio,
                )
                    .chain(),
                (
                    world::sync,
                    world::animate,
                    world::dress_nili,
                    world::animate_people,
                    world::animate_work,
                    world::animate_bells,
                    ui::update_prompts,
                    ui::update_reactions,
                    ui::update_lessons,
                    world::animate_construction,
                )
                    .chain(),
                (
                    world::adventure_scene,
                    world::adventure_animate,
                    world::adventure_logistics,
                    world::adventure_fishing,
                    ui::project_adventure,
                    world::animate_weather,
                    world::animate_pets,
                )
                    .chain(),
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
    #[cfg(not(target_os = "android"))]
    macro_rules! harbor_scores {
        ($($id:literal),* $(,)?) => {
            $(
                registry.insert_asset(
                    Path::new(concat!("audio/harbors/", $id, ".ogg")).into(),
                    Path::new(concat!("audio/harbors/", $id, ".ogg")),
                    include_bytes!(concat!("../assets/audio/harbors/", $id, ".ogg")).as_slice(),
                );
                registry.insert_asset(
                    Path::new(concat!("audio/harbors/", $id, "-abend.ogg")).into(),
                    Path::new(concat!("audio/harbors/", $id, "-abend.ogg")),
                    include_bytes!(concat!("../assets/audio/harbors/", $id, "-abend.ogg")).as_slice(),
                );
            )*
        };
    }
    #[cfg(not(target_os = "android"))]
    harbor_scores!(
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14",
        "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44",
        "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74",
        "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
        "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "100", "101", "102", "103",
        "104", "105", "106", "107", "108", "109", "110", "111", "112", "113", "114", "115", "116",
        "117", "118", "119",
    );
    #[cfg(not(target_os = "android"))]
    macro_rules! expedition_scores {
        ($($id:literal),* $(,)?) => {
            $(registry.insert_asset(Path::new(concat!("audio/abenteuer/", $id, ".ogg")).into(),
                Path::new(concat!("audio/abenteuer/", $id, ".ogg")),
                include_bytes!(concat!("../assets/audio/abenteuer/", $id, ".ogg")).as_slice());)*
        };
    }
    #[cfg(not(target_os = "android"))]
    expedition_scores!(
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14",
        "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44",
        "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "nacht-0", "nacht-1", "nacht-2", "nacht-3", "nacht-4",
    );
    add!(
        "fonts/DMSans-Bold.ttf",
        "fonts/DMSans-Medium.ttf",
        "fonts/DMSerifText-Regular.ttf",
        "models/hippo.glb",
        "models/villages/aegean.glb",
        "models/works/aegean.glb",
        "models/waldwinkel/wood_tree.glb",
        "models/waldwinkel/resource_copper.glb",
        "models/waldwinkel/resource_crystal.glb",
        "models/waldwinkel/resource_rock.glb",
        "audio/abenteuer/wood.wav",
        "audio/abenteuer/stone.wav",
        "audio/abenteuer/forage.wav",
        "audio/aegean.ogg",
        "audio/roman_quay.ogg",
        "audio/street_sweep.wav",
        "audio/street_purr.wav",
        "audio/street_woof.wav",
        "audio/street_bell.wav",
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
        "audio/temple_bell.wav",
        "audio/cast_off.wav",
        "audio/nili_aid.wav",
        "audio/shore_loop.wav",
        "ui/wear_aegean_chiton.png",
        "ui/wear_olive_peplos.png",
        "ui/wear_myrtle_crown.png",
        "ui/wear_folded_petasos.png",
        "ui/wear_shell_pin.png",
        "ui/wear_mulberry_toga.png",
        "ui/wear_aventin_tunic.png",
        "ui/wear_red_pileus.png",
        "ui/wear_gold_laurel.png",
        "ui/wear_signet_cord.png",
        "ui/wear_sakura_kosode.png",
        "ui/wear_mino_cape.png",
        "ui/wear_river_kasa.png",
        "ui/wear_crane_pin.png",
        "ui/wear_rice_pouch.png",
        "ui/wear_sea_cloak.png",
        "ui/wear_tablet_dress.png",
        "ui/wear_fur_cap.png",
        "ui/wear_knit_band.png",
        "ui/wear_knot_pendant.png",
        "ui/wear_oasis_caftan.png",
        "ui/wear_moon_silk.png",
        "ui/wear_star_turban.png",
        "ui/wear_caravan_veil.png",
        "ui/wear_rose_flask.png",
        "ui/phone_menu.png",
        "ui/phone_close.png",
        "ui/phone_camera.png",
        "ui/adventure.png",
        "ui/adventure_blueprint.png",
        "ui/adventure_boards.png",
        "ui/adventure_bricks.png",
        "ui/adventure_brickyard.png",
        "ui/adventure_clay.png",
        "ui/adventure_cloth.png",
        "ui/adventure_cover_0.png",
        "ui/adventure_cover_1.png",
        "ui/adventure_cover_2.png",
        "ui/adventure_cover_3.png",
        "ui/adventure_cover_4.png",
        "ui/adventure_craft.png",
        "ui/adventure_food.png",
        "ui/adventure_forge.png",
        "ui/adventure_hill.png",
        "ui/adventure_labor.png",
        "ui/adventure_meals.png",
        "ui/adventure_ore.png",
        "ui/adventure_sawmill.png",
        "ui/adventure_stone.png",
        "ui/adventure_tools.png",
        "ui/adventure_wood.png",
        "ui/machiya.png",
        "ui/temple.png",
        "ui/temple_bell.png",
        "ui/rice_paddy.png",
        "ui/rice_kitchen.png",
        "ui/rice_bowl.png",
        "ui/tea_garden.png",
        "ui/tea_house.png",
        "ui/tea_cup.png",
        "ui/paper_workshop.png",
        "ui/lampion.png",
        "ui/wear_indigo_kosode.png",
        "ui/wear_ahorn.png",
        "ui/wear_haori.png",
        "ui/wear_kasa.png",
        "ui/wear_pilgrim.png",
        "ui/wear_kanzashi.png",
        "ui/wear_obi.png",
        "ui/wear_inro.png",
        "ui/island_48.png",
        "ui/island_49.png",
        "ui/island_50.png",
        "ui/island_51.png",
        "ui/island_52.png",
        "ui/island_53.png",
        "ui/island_54.png",
        "ui/island_55.png",
        "ui/island_56.png",
        "ui/island_57.png",
        "ui/island_58.png",
        "ui/island_59.png",
        "ui/island_60.png",
        "ui/island_61.png",
        "ui/island_62.png",
        "ui/island_63.png",
        "ui/island_64.png",
        "ui/island_65.png",
        "ui/island_66.png",
        "ui/island_67.png",
        "ui/island_68.png",
        "ui/island_69.png",
        "ui/island_70.png",
        "ui/island_71.png",
        "ui/longhouse.png",
        "ui/fishery.png",
        "ui/smokehouse.png",
        "ui/woodlot.png",
        "ui/boatyard.png",
        "ui/sea_trade.png",
        "ui/weaver.png",
        "ui/feast_hall.png",
        "ui/smoked_fish.png",
        "ui/woven_cloth.png",
        "ui/hearth_bowl.png",
        "ui/wear_fjord.png",
        "ui/wear_krapp.png",
        "ui/wear_winter.png",
        "ui/wear_wool.png",
        "ui/wear_sailor.png",
        "ui/wear_braid.png",
        "ui/wear_amber_brooch.png",
        "ui/wear_woven_bag.png",
        "ui/island_72.png",
        "ui/island_73.png",
        "ui/island_74.png",
        "ui/island_75.png",
        "ui/island_76.png",
        "ui/island_77.png",
        "ui/island_78.png",
        "ui/island_79.png",
        "ui/island_80.png",
        "ui/island_81.png",
        "ui/island_82.png",
        "ui/island_83.png",
        "ui/island_84.png",
        "ui/island_85.png",
        "ui/island_86.png",
        "ui/island_87.png",
        "ui/island_88.png",
        "ui/island_89.png",
        "ui/island_90.png",
        "ui/island_91.png",
        "ui/island_92.png",
        "ui/island_93.png",
        "ui/island_94.png",
        "ui/island_95.png",
        "ui/oasis_home.png",
        "ui/cistern.png",
        "ui/date_garden.png",
        "ui/spice_bazaar.png",
        "ui/caravanserai.png",
        "ui/qanat.png",
        "ui/glassworks.png",
        "ui/perfumer.png",
        "ui/moon_garden.png",
        "ui/carpet_hall.png",
        "ui/caravan.png",
        "ui/wear_lapis.png",
        "ui/wear_dune_wrap.png",
        "ui/wear_glass_pendant.png",
        "ui/island_96.png",
        "ui/island_97.png",
        "ui/island_98.png",
        "ui/island_99.png",
        "ui/island_100.png",
        "ui/island_101.png",
        "ui/island_102.png",
        "ui/island_103.png",
        "ui/island_104.png",
        "ui/island_105.png",
        "ui/island_106.png",
        "ui/island_107.png",
        "ui/island_108.png",
        "ui/island_109.png",
        "ui/island_110.png",
        "ui/island_111.png",
        "ui/island_112.png",
        "ui/island_113.png",
        "ui/island_114.png",
        "ui/island_115.png",
        "ui/island_116.png",
        "ui/island_117.png",
        "ui/island_118.png",
        "ui/island_119.png",
        "ui/moon.png",
        "ui/apiary.png",
        "ui/dyer.png",
        "ui/ferry_office.png",
        "ui/mill.png",
        "ui/scribe.png",
        "ui/garden_court.png",
        "ui/market_garden.png",
        "ui/ceramic_studio.png",
        "ui/silk_shop.png",
        "ui/sheepfold.png",
        "ui/cooperage.png",
        "ui/signal_tower.png",
        "ui/age_child.png",
        "ui/age_adult.png",
        "ui/age_senior.png",
        "ui/cat.png",
        "ui/dog.png",
        "ui/leaves.png",
        "ui/scraps.png",
        "ui/herb_garden.png",
        "ui/wear_olive.png",
        "ui/wear_linen.png",
        "ui/wear_coral.png",
        "ui/wear_night.png",
        "ui/wear_mosaic.png",
        "ui/wear_sunbonnet.png",
        "ui/wear_indigo.png",
        "ui/wear_diadem.png",
        "ui/wear_wreath.png",
        "ui/wear_headband.png",
        "ui/wear_satchel.png",
        "ui/wear_shell.png",
        "ui/wear_pearls.png",
        "ui/wear_brooch.png",
        "ui/wear_scarf.png",
        "ui/wear_belt.png",
        "ui/wear_basket.png",
        "ui/wear_amulet.png",
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
        "ui/fire.png",
        "ui/wear_sea.png",
        "ui/wear_saffron.png",
        "ui/wear_toga.png",
        "ui/wear_straw.png",
        "ui/wear_laurel.png",
        "ui/wear_petasos.png",
        "ui/forum.png",
        "ui/grove.png",
        "ui/guide.png",
        "ui/guest_book.png",
        "ui/pad_dpad.png",
        "ui/pad_view.png",
        "ui/pause.png",
        "ui/sunset.png",
        "ui/budget.png",
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

use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{cosmetics::Cosmetic, save::Campaign};

use super::{
    buildings::{CREAM, GOLD},
    mesh::{Batch, Tint, shift, v},
};

#[derive(Component)]
pub struct Costume(Cosmetic);

pub(super) fn spawn(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    for item in Cosmetic::ALL {
        parent.spawn((
            Costume(item),
            Mesh3d(meshes.add(model(item))),
            MeshMaterial3d(material.clone()),
            Transform::default(),
            Visibility::Hidden,
        ));
    }
}

pub fn dress(save: Res<Campaign>, mut clothes: Query<(&Costume, &mut Visibility)>) {
    for (item, mut visible) in &mut clothes {
        let next = if save.wardrobe.wearing(item.0) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if next != *visible {
            *visible = next;
        }
    }
}

fn model(item: Cosmetic) -> Mesh {
    let mut fabric = Batch::default();
    if let Some((base, trim)) = collection_colors(item) {
        expedition(&mut fabric, item, base, trim);
        return fabric.mesh();
    }
    match item {
        Cosmetic::Sea => garment(&mut fabric, [53, 127, 155], [193, 224, 210], false),
        Cosmetic::Saffron => garment(&mut fabric, [213, 146, 66], [241, 201, 117], false),
        Cosmetic::Toga => garment(&mut fabric, [165, 78, 121], [241, 213, 178], true),
        Cosmetic::Olive => garment(&mut fabric, [105, 139, 71], [215, 199, 125], false),
        Cosmetic::Linen => garment(&mut fabric, [235, 220, 177], [139, 63, 84], true),
        Cosmetic::Coral => garment(&mut fabric, [207, 105, 104], [249, 191, 137], true),
        Cosmetic::Night => garment(&mut fabric, [47, 59, 114], [155, 179, 211], false),
        Cosmetic::Mosaic => garment(&mut fabric, [61, 144, 135], [235, 166, 95], true),
        Cosmetic::IndigoKosode => kosode(&mut fabric, [64, 93, 153], [177, 212, 222], false),
        Cosmetic::AutumnKosode => kosode(&mut fabric, [184, 87, 59], [235, 170, 96], false),
        Cosmetic::Haori => kosode(&mut fabric, [104, 67, 134], [249, 182, 112], true),
        Cosmetic::LapisRobe => {
            garment(&mut fabric, [52, 127, 145], [226, 171, 88], true);
            fabric_pin(
                &mut fabric,
                v(0.18, 0.98, 0.445),
                [223, 173, 86],
                [133, 67, 102],
            );
        }
        Cosmetic::DuneWrap => {
            let at = v(0.0, 1.55, -0.099);
            fabric.gem(
                at + Vec3::Y * 0.10,
                0.425,
                0.205,
                0.367,
                0.07,
                [232, 208, 165],
            );
            fabric.frustum(at + Vec3::Y * 0.02, 0.431, 0.416, 0.055, 16, [53, 103, 119]);
            for side in [-1.0, 1.0] {
                fabric.beam(
                    at + v(side * 0.365, 0.042, 0.15),
                    at + v(side * 0.365, -0.24, 0.049),
                    0.100,
                    [216, 184, 138],
                );
                fabric.beam(
                    at + v(side * 0.365, -0.22, 0.049),
                    at + v(side * 0.282, -0.40, 0.017),
                    0.073,
                    [236, 203, 154],
                );
            }
        }
        Cosmetic::FjordTunic => wool_coat(&mut fabric, [70, 118, 142], [195, 177, 127], false),
        Cosmetic::DyedCloak => wool_coat(&mut fabric, [162, 83, 62], [239, 172, 101], true),
        Cosmetic::WinterCoat => wool_coat(&mut fabric, [86, 116, 100], [221, 211, 179], true),
        Cosmetic::WoolHood => {
            let at = v(0.0, 1.535, -0.188);
            fabric.gem(at, 0.414, 0.403, 0.258, 0.45, [111, 99, 135]);
            fabric.gem(
                at + v(0.0, -0.185, -0.037),
                0.334,
                0.268,
                0.260,
                0.24,
                [124, 112, 147],
            );
            for side in [-1.0, 1.0] {
                fabric.beam(
                    at + v(side * 0.23, 0.226, 0.225),
                    at + v(side * 0.383, -0.187, 0.24),
                    0.057,
                    [205, 187, 162],
                );
                fabric.beam(
                    at + v(side * 0.379, -0.177, 0.25),
                    at + v(side * 0.18, -0.451, 0.321),
                    0.046,
                    [205, 187, 162],
                );
            }
        }
        Cosmetic::SailorsCap => {
            let at = v(0.0, 1.569, -0.038);
            fabric.frustum(at, 0.382, 0.394, 0.077, 14, [168, 126, 83]);
            fabric.frustum(
                at + Vec3::Y * 0.077,
                0.394,
                0.264,
                0.137,
                14,
                [78, 113, 143],
            );
            fabric.frustum(
                at + Vec3::Y * 0.213,
                0.264,
                0.079,
                0.072,
                14,
                [111, 146, 170],
            );
            for i in 0..14 {
                let a = i as f32 * TAU / 14.0;
                let r = v(a.cos(), 0.0, a.sin());
                fabric.beam(
                    at + r * 0.385 + Vec3::Y * 0.079,
                    at + r * 0.264 + Vec3::Y * 0.213,
                    0.029,
                    [104, 141, 171],
                );
            }
        }
        Cosmetic::BraidBand => {
            for i in 0..20 {
                let a = i as f32 * TAU / 20.0;
                let p = v(a.cos() * 0.372, 1.548 + a.sin() * 0.035, a.sin() * 0.34);
                fabric.gem(
                    p,
                    0.068,
                    0.041,
                    0.053,
                    -a,
                    [[170, 85, 61], [212, 164, 94], [231, 202, 154]][i % 3],
                );
            }
        }
        Cosmetic::Kasa | Cosmetic::PilgrimHat => {
            let pilgrim = item == Cosmetic::PilgrimHat;
            let origin = v(0.0, 1.61, 0.007);
            let radius = if pilgrim { 0.61 } else { 0.49 };
            let straw = if pilgrim {
                [175, 125, 74]
            } else {
                [221, 180, 113]
            };
            fabric.frustum(origin, radius, radius - 0.018, 0.052, 18, shift(straw, -17));
            fabric.frustum(
                origin + Vec3::Y * 0.052,
                radius - 0.018,
                0.056,
                0.31,
                18,
                straw,
            );
            for i in 0..18 {
                let theta = i as f32 * TAU / 18.0;
                let r = v(theta.cos(), 0.0, theta.sin());
                fabric.beam(
                    origin + r * radius + Vec3::Y * 0.054,
                    origin + r * 0.049 + Vec3::Y * 0.363,
                    0.019,
                    shift(straw, 18),
                );
            }
            if pilgrim {
                fabric.slab(v(0.0, 1.54, -0.40), v(0.47, 0.24, 0.027), [64, 91, 144]);
                for x in [-0.42, 0.42] {
                    fabric.beam(
                        v(x, 1.63, 0.14),
                        v(x * 0.7, 1.37, 0.21),
                        0.026,
                        [215, 202, 152],
                    );
                }
            }
        }
        Cosmetic::Kanzashi => {
            fabric.beam(v(0.16, 1.54, -0.06), v(0.48, 1.78, -0.047), 0.048, GOLD);
            for i in 0..7 {
                let a = i as f32 * TAU / 7.0;
                fabric.gem(
                    v(0.418 + a.cos() * 0.072, 1.739 + a.sin() * 0.074, -0.045),
                    0.043,
                    0.051,
                    0.041,
                    a,
                    if i % 2 == 0 {
                        [228, 116, 141]
                    } else {
                        [252, 175, 175]
                    },
                );
            }
            fabric.gem(v(0.42, 1.74, -0.012), 0.051, 0.049, 0.027, 0.09, GOLD);
            for x in [0.38, 0.45] {
                fabric.beam(v(x, 1.724, -0.048), v(x + 0.055, 1.496, -0.02), 0.017, GOLD);
                fabric.gem(
                    v(x + 0.055, 1.49, -0.02),
                    0.032,
                    0.048,
                    0.026,
                    0.5,
                    [186, 102, 138],
                );
            }
        }
        Cosmetic::Straw => hat(&mut fabric, [203, 155, 85], [55, 119, 121], true),
        Cosmetic::Petasos => hat(&mut fabric, [164, 72, 64], [232, 188, 127], false),
        Cosmetic::Sunbonnet => hat(&mut fabric, [229, 194, 124], [203, 105, 123], true),
        Cosmetic::Indigo => {
            let p = v(0.0, 1.58, -0.055);
            fabric.frustum(p, 0.36, 0.38, 0.065, 12, [55, 68, 131]);
            fabric.frustum(p + Vec3::Y * 0.065, 0.38, 0.19, 0.18, 12, [84, 105, 174]);
            fabric.frustum(p + Vec3::Y * 0.245, 0.19, 0.05, 0.073, 12, [100, 127, 189]);
            fabric.block(
                v(0.0, 1.602, 0.32),
                v(0.47, 0.044, 0.30),
                Quat::from_rotation_x(0.17),
                [50, 62, 119],
            );
        }
        Cosmetic::Diadem | Cosmetic::Headband => {
            let metal = if item == Cosmetic::Diadem {
                GOLD
            } else {
                [206, 106, 103]
            };
            let at = |i: usize| {
                let angle = i as f32 * TAU / 18.0;
                v(
                    angle.cos() * 0.351,
                    1.53 + angle.sin() * 0.038,
                    angle.sin() * 0.317 + 0.024,
                )
            };
            for i in 0..18 {
                fabric.beam(
                    at(i),
                    at(i + 1),
                    0.034,
                    shift(metal, (i % 3) as i16 * 9 - 7),
                );
            }
            if item == Cosmetic::Diadem {
                fabric.gem(v(0.0, 1.603, 0.35), 0.076, 0.082, 0.03, 0.5, GOLD);
                fabric.gem(
                    v(0.0, 1.609, 0.377),
                    0.047,
                    0.058,
                    0.023,
                    0.6,
                    [68, 152, 147],
                );
            }
        }
        Cosmetic::Laurel | Cosmetic::Wreath => {
            for i in 0..14 {
                let angle = i as f32 / 14.0 * TAU;
                let at = v(
                    angle.cos() * 0.337,
                    1.53 + angle.sin() * 0.024,
                    angle.sin() * 0.306 + 0.02,
                );
                let next = angle + TAU / 14.0;
                let reach = v(
                    next.cos() * 0.337,
                    1.53 + next.sin() * 0.024,
                    next.sin() * 0.306 + 0.02,
                );
                fabric.beam(at, reach, 0.025, [86, 139, 74]);
                fabric.block(
                    at + Vec3::Y * 0.045,
                    v(0.064, 0.122, 0.033),
                    Quat::from_rotation_y(-angle) * Quat::from_rotation_z(0.55),
                    shift([101, 171, 92], (i % 4) as i16 * 11 - 12),
                );
                if item == Cosmetic::Wreath && i % 2 == 0 {
                    let petals = [[237, 151, 156], [240, 202, 129], [154, 142, 212]];
                    fabric.gem(
                        at + Vec3::Y * 0.061,
                        0.081,
                        0.049,
                        0.064,
                        angle,
                        petals[i % 3],
                    );
                    fabric.gem(at + Vec3::Y * 0.102, 0.034, 0.021, 0.032, angle, CREAM);
                }
            }
            fabric.gem(v(0.0, 1.555, 0.337), 0.054, 0.042, 0.023, 0.31, GOLD);
        }
        _ => accessory(&mut fabric, item),
    }
    fabric.mesh()
}

fn collection_colors(item: Cosmetic) -> Option<(Tint, Tint)> {
    use Cosmetic::*;
    Some(match item {
        AegeanChiton => ([64, 138, 153], [230, 194, 136]),
        OlivePeplos => ([142, 155, 89], [230, 202, 137]),
        MyrtleCrown => ([96, 148, 119], [240, 216, 163]),
        FoldedPetasos => ([93, 127, 165], [224, 184, 124]),
        ShellPin => ([237, 205, 185], [130, 180, 183]),
        MulberryToga => ([155, 85, 117], [245, 212, 170]),
        MosaicTunic => ([212, 163, 91], [66, 132, 123]),
        RedPileus => ([169, 88, 81], [242, 194, 140]),
        GoldLaurel => ([203, 156, 80], [249, 223, 142]),
        SignetCord => ([206, 150, 87], [174, 103, 92]),
        SakuraKosode => ([180, 105, 135], [248, 201, 173]),
        MinoCape => ([171, 132, 86], [239, 211, 151]),
        RiverKasa => ([128, 148, 117], [190, 216, 174]),
        CranePin => ([241, 219, 176], [182, 123, 98]),
        RicePouch => ([86, 120, 144], [223, 166, 118]),
        SeaCloak => ([81, 117, 134], [214, 190, 154]),
        TabletDress => ([125, 82, 95], [227, 180, 134]),
        FurCap => ([155, 112, 84], [229, 194, 161]),
        KnitBand => ([121, 108, 157], [224, 184, 137]),
        KnotPendant => ([209, 174, 113], [91, 138, 136]),
        OasisCaftan => ([54, 138, 129], [241, 192, 114]),
        MoonSilk => ([120, 99, 143], [233, 204, 152]),
        StarTurban => ([68, 91, 148], [243, 204, 119]),
        CaravanVeil => ([202, 130, 93], [249, 223, 172]),
        RoseFlask => ([180, 117, 145], [232, 189, 121]),
        _ => return None,
    })
}

fn accessory(b: &mut Batch, item: Cosmetic) {
    match item {
        Cosmetic::Satchel | Cosmetic::Basket => {
            let strap = [139, 88, 59];
            b.beam(v(-0.30, 0.915, 0.29), v(0.37, 0.62, 0.347), 0.042, strap);
            b.beam(v(-0.30, 0.915, -0.28), v(0.36, 0.61, -0.338), 0.042, strap);
            if item == Cosmetic::Satchel {
                b.block(
                    v(0.396, 0.60, 0.19),
                    v(0.26, 0.273, 0.26),
                    Quat::from_rotation_z(0.19),
                    strap,
                );
                b.slab(v(0.397, 0.69, 0.254), v(0.27, 0.096, 0.181), [174, 119, 73]);
                b.gem(v(0.401, 0.64, 0.333), 0.044, 0.047, 0.025, 0.5, GOLD);
            } else {
                let at = v(0.404, 0.432, 0.192);
                for i in 0..5 {
                    let side = i as f32 * 0.012;
                    b.frustum(
                        at + Vec3::Y * (i as f32 * 0.040),
                        0.094 + side,
                        0.103 + side,
                        0.041,
                        12,
                        shift([191, 137, 78], i as i16 * 5 - 8),
                    );
                }
                b.gem(
                    at + v(0.02, 0.194, 0.03),
                    0.13,
                    0.069,
                    0.10,
                    0.1,
                    [115, 159, 80],
                );
            }
        }
        Cosmetic::Shell | Cosmetic::Pearls | Cosmetic::Amulet => {
            let bead = if item == Cosmetic::Pearls {
                [251, 230, 198]
            } else {
                [181, 120, 70]
            };
            for i in 0..13 {
                let x = (i as f32 - 6.0) / 6.0;
                b.gem(
                    v(x * 0.329, 0.855 + x * x * 0.161, 0.391 - x * x * 0.131),
                    0.022,
                    0.025,
                    0.023,
                    i as f32,
                    shift(bead, i % 4 * 6),
                );
            }
            let color = if item == Cosmetic::Amulet {
                [222, 144, 66]
            } else {
                [236, 184, 148]
            };
            b.gem(v(0.0, 0.786, 0.408), 0.071, 0.078, 0.032, 0.38, color);
            if item == Cosmetic::Shell {
                for x in [-0.042, 0.0, 0.042] {
                    b.beam(v(0.0, 0.814, 0.438), v(x, 0.736, 0.435), 0.013, CREAM);
                }
            }
        }
        Cosmetic::Brooch => {
            b.gem(v(0.28, 0.92, 0.323), 0.092, 0.086, 0.036, 0.23, GOLD);
            b.gem(
                v(0.28, 0.922, 0.350),
                0.063,
                0.058,
                0.038,
                0.31,
                [80, 168, 133],
            );
        }
        Cosmetic::Scarf => {
            b.beam(
                v(-0.29, 0.961, 0.27),
                v(0.285, 0.925, 0.35),
                0.10,
                [190, 80, 104],
            );
            b.gem(
                v(0.16, 0.925, 0.371),
                0.084,
                0.066,
                0.031,
                0.5,
                [222, 106, 132],
            );
            b.block(
                v(0.22, 0.813, 0.396),
                v(0.11, 0.236, 0.047),
                Quat::from_rotation_z(0.40),
                [190, 80, 104],
            );
        }
        Cosmetic::Belt => {
            let point = |i: usize| {
                let a = i as f32 * TAU / 18.0;
                v(a.cos() * 0.425, 0.59, a.sin() * 0.383 - 0.017)
            };
            for i in 0..18 {
                b.beam(
                    point(i),
                    point(i + 1),
                    0.067,
                    shift([190, 138, 71], i as i16 % 3 * 10),
                );
            }
            b.gem(v(0.04, 0.595, 0.387), 0.079, 0.064, 0.026, 0.4, GOLD);
        }
        Cosmetic::Obi => {
            let at = |i| {
                let angle = i as f32 * TAU / 18.0;
                v(angle.cos() * 0.461, 0.67, angle.sin() * 0.405 - 0.017)
            };
            for i in 0..18 {
                b.beam(
                    at(i) + Vec3::Y * 0.041,
                    at(i + 1) + Vec3::Y * 0.041,
                    0.051,
                    [234, 164, 97],
                );
                b.beam(at(i), at(i + 1), 0.059, [181, 77, 110]);
                b.beam(
                    at(i) - Vec3::Y * 0.037,
                    at(i + 1) - Vec3::Y * 0.037,
                    0.044,
                    [234, 164, 97],
                );
            }
            for x in [-0.13, 0.13] {
                b.block(
                    v(x, 0.69, -0.432),
                    v(0.267, 0.177, 0.115),
                    Quat::from_rotation_z(-x * 2.5),
                    [194, 102, 137],
                );
            }
            b.gem(
                v(0.0, 0.684, -0.487),
                0.101,
                0.087,
                0.048,
                0.4,
                [226, 152, 162],
            );
        }
        Cosmetic::Inro => {
            b.beam(
                v(0.41, 0.83, 0.055),
                v(0.49, 0.59, 0.16),
                0.032,
                [205, 168, 110],
            );
            b.gem(v(0.463, 0.743, 0.105), 0.052, 0.048, 0.046, 0.1, GOLD);
            for n in 0..4 {
                b.slab(
                    v(0.50, 0.44 + n as f32 * 0.054, 0.17),
                    v(0.106, 0.048, 0.143),
                    [91, 59, 60],
                );
                b.slab(
                    v(0.556, 0.449 + n as f32 * 0.054, 0.171),
                    v(0.013, 0.022, 0.103),
                    [209, 149, 71],
                );
            }
            b.gem(
                v(0.551, 0.533, 0.241),
                0.035,
                0.057,
                0.019,
                0.3,
                [234, 173, 84],
            );
        }
        Cosmetic::GlassPendant => {
            let at = v(0.0, 0.94, 0.447);
            for side in [-1.0, 1.0] {
                b.beam(v(side * 0.34, 1.21, 0.26), at, 0.028, [172, 122, 71]);
            }
            b.gem(
                at + v(0.0, -0.081, 0.030),
                0.113,
                0.132,
                0.052,
                0.22,
                [218, 167, 78],
            );
            b.gem(
                at + v(0.0, -0.081, 0.074),
                0.082,
                0.101,
                0.038,
                0.45,
                [74, 164, 164],
            );
        }
        Cosmetic::AmberBrooch => {
            let at = v(-0.35, 0.894, 0.298);
            fabric_pin(b, at, [222, 168, 91], [207, 114, 63]);
            for (i, x) in [-0.052, 0.052].into_iter().enumerate() {
                b.beam(
                    at + v(x, -0.03, -0.003),
                    at + v(x + 0.025, -0.153 - 0.028 * i as f32, -0.013),
                    0.018,
                    GOLD,
                );
            }
        }
        Cosmetic::WovenBag => {
            b.beam(
                v(0.28, 0.94, 0.266),
                v(-0.44, 0.60, 0.255),
                0.047,
                [174, 130, 81],
            );
            b.block(
                v(-0.46, 0.56, 0.198),
                v(0.23, 0.25, 0.126),
                Quat::from_rotation_z(0.20),
                [77, 123, 132],
            );
            for n in 0..6 {
                let y = 0.465 + n as f32 * 0.035;
                b.beam(
                    v(-0.57 - n as f32 * 0.005, y - 0.020, 0.265),
                    v(-0.36 - n as f32 * 0.005, y + 0.019, 0.265),
                    0.019,
                    if n % 3 == 0 {
                        [225, 188, 123]
                    } else {
                        [163, 126, 106]
                    },
                );
            }
            fabric_pin(b, v(-0.463, 0.626, 0.275), GOLD, [154, 82, 79]);
        }
        _ => unreachable!("only accessories use these meshes"),
    }
}

fn fabric_pin(b: &mut Batch, at: Vec3, outer: Tint, inner: Tint) {
    b.gem(at, 0.102, 0.092, 0.031, 0.12, outer);
    b.gem(at + v(0.0, 0.0, 0.029), 0.055, 0.055, 0.021, 0.35, inner);
}

fn wool_coat(b: &mut Batch, color: Tint, trim: Tint, cloak: bool) {
    garment(b, color, trim, cloak);
    b.gem(
        v(0.0, 0.900, -0.031),
        0.42,
        0.107,
        0.349,
        0.39,
        shift(color, 17),
    );
    if cloak {
        let top = [v(-0.32, 0.914, -0.24), v(0.385, 0.944, -0.23)];
        let ends = [v(-0.43, 0.398, -0.432), v(0.45, 0.469, -0.449)];
        b.face(&[top[0], top[1], ends[1], ends[0]], shift(color, -13));
        b.beam(ends[0], ends[1], 0.049, trim);
        for (i, p) in ends.into_iter().enumerate() {
            b.beam(top[i], p, 0.046, shift(trim, -12));
        }
    }
    for side in [-1.0, 1.0] {
        fabric_pin(b, v(side * 0.232, 0.864, 0.313), trim, [179, 120, 69]);
    }
    b.beam(v(-0.23, 0.855, 0.34), v(0.23, 0.86, 0.34), 0.029, trim);
}

fn kosode(b: &mut Batch, cloth: Tint, trim: Tint, formal: bool) {
    let point = |ring: usize, i: usize| {
        let (y, x, z) = [
            (0.43, 0.408, 0.359),
            (0.66, 0.450, 0.396),
            (0.86, 0.47, 0.389),
            (0.965, 0.357, 0.314),
        ][ring];
        let theta = i as f32 * TAU / 18.0;
        v(theta.cos() * x, y, theta.sin() * z - 0.025)
    };
    for ring in 0..3 {
        for i in 0..18 {
            let tint = if ring == 0 && i % 3 == 0 {
                shift(cloth, 21)
            } else {
                cloth
            };
            b.face(
                &[
                    point(ring, i),
                    point(ring + 1, i),
                    point(ring + 1, i + 1),
                    point(ring, i + 1),
                ],
                shift(tint, (i * 9 % 20) as i16 - 9),
            );
        }
    }
    for side in [-1.0, 1.0] {
        let shoulder = v(side * 0.218, 0.945, 0.265);
        let cross = v(-side * 0.128, 0.684, 0.391);
        b.beam(
            shoulder,
            cross,
            0.053,
            if side < 0.0 { trim } else { shift(trim, -22) },
        );
        b.block(
            v(side * 0.431, 0.761, -0.019),
            v(if formal { 0.239 } else { 0.183 }, 0.287, 0.39),
            Quat::from_rotation_z(side * 0.16),
            shift(cloth, if formal { -19 } else { 5 }),
        );
        b.beam(
            v(side * 0.504, 0.663, -0.185),
            v(side * 0.504, 0.663, 0.167),
            0.043,
            trim,
        );
    }
    for (x, y, z) in [
        (-0.24, 0.51, 0.311),
        (0.16, 0.57, 0.348),
        (0.29, 0.70, 0.288),
    ] {
        for i in 0..5 {
            let a = i as f32 * TAU / 5.0;
            b.gem(
                v(x + a.cos() * 0.032, y + a.sin() * 0.032, z),
                0.024,
                0.028,
                0.019,
                a,
                trim,
            );
        }
    }
}

fn garment(m: &mut Batch, color: Tint, edge: Tint, mantle: bool) {
    let rings = [
        (0.44, 0.364, 0.352),
        (0.50, 0.407, 0.379),
        (0.72, 0.454, 0.389),
        (0.84, 0.456, 0.385),
        (0.935, 0.387, 0.329),
    ];
    let point = |ring: usize, i: usize| {
        let (y, rx, rz) = rings[ring];
        let a = i as f32 / 18.0 * TAU;
        let fold = 0.013 * (a * 9.0).cos();
        let diagonal = a.cos() * if mantle { 0.064 } else { 0.021 };
        v(
            a.cos() * (rx + fold),
            y + diagonal * (0.4 + ring as f32 * 0.19),
            a.sin() * (rz + fold) - 0.024,
        )
    };
    for ring in 0..rings.len() - 1 {
        for i in 0..18 {
            let trim = ring == 0 || mantle && matches!(i, 3 | 4 | 13 | 14) && ring >= 2;
            m.face(
                &[
                    point(ring, i),
                    point(ring + 1, i),
                    point(ring + 1, i + 1),
                    point(ring, i + 1),
                ],
                shift(if trim { edge } else { color }, (i * 9 % 17) as i16 - 9),
            );
        }
    }
    m.face(
        &[
            v(-0.31, 0.61, 0.315),
            v(-0.27, 0.51, 0.342),
            v(0.07, 0.66, 0.409),
            v(0.025, 0.73, 0.406),
        ],
        edge,
    );
    m.face(
        &[
            v(0.025, 0.73, 0.406),
            v(0.07, 0.66, 0.409),
            v(0.37, 0.815, 0.279),
            v(0.29, 0.872, 0.268),
        ],
        shift(edge, 6),
    );
    m.gem(v(-0.311, 0.857, 0.258), 0.069, 0.065, 0.032, 0.4, GOLD);
    m.gem(
        v(-0.312, 0.856, 0.287),
        0.035,
        0.035,
        0.013,
        0.4,
        if mantle { edge } else { CREAM },
    );
}

fn hat(m: &mut Batch, color: Tint, band: Tint, straw: bool) {
    let center = v(0.038, 1.59, -0.019);
    m.frustum(
        center,
        0.42,
        if straw { 0.49 } else { 0.44 },
        0.044,
        14,
        shift(color, -14),
    );
    m.frustum(
        center + Vec3::Y * 0.044,
        if straw { 0.49 } else { 0.44 },
        0.298,
        0.071,
        14,
        color,
    );
    m.frustum(center + Vec3::Y * 0.105, 0.298, 0.277, 0.061, 14, band);
    m.frustum(
        center + Vec3::Y * 0.157,
        0.278,
        if straw { 0.185 } else { 0.147 },
        0.145,
        14,
        shift(color, 13),
    );
    m.gem(
        center + v(0.153, 0.139, 0.229),
        0.044,
        0.046,
        0.022,
        0.4,
        GOLD,
    );
}

fn expedition(b: &mut Batch, item: Cosmetic, base: Tint, trim: Tint) {
    use Cosmetic::*;
    let front = v(0.0, 0.72, 0.397);
    let cap = v(0.0, 1.58, -0.06);
    match item {
        AegeanChiton | OlivePeplos | MulberryToga | MosaicTunic | OasisCaftan | MoonSilk => {
            garment(
                b,
                base,
                trim,
                matches!(item, OlivePeplos | MulberryToga | MoonSilk),
            );
            for side in [-1.0, 1.0] {
                b.block(
                    v(side * 0.34, 0.868, -0.018),
                    v(0.22, 0.136, 0.39),
                    Quat::from_rotation_z(side * 0.31),
                    shift(base, 12),
                );
            }
            match item {
                AegeanChiton | MosaicTunic => {
                    for i in 0..8 {
                        let a = i as f32 / 8.0 * std::f32::consts::PI;
                        let p = v(a.cos() * 0.406, 0.524, a.sin() * 0.384);
                        b.gem(p, 0.029, 0.026, 0.018, a, trim);
                    }
                    fabric_pin(b, front + v(0.125, -0.13, -0.005), trim, base);
                }
                OlivePeplos => {
                    b.face(
                        &[
                            v(-0.25, 0.81, 0.344),
                            v(0.24, 0.72, 0.333),
                            v(0.29, 0.55, 0.310),
                            v(-0.18, 0.60, 0.368),
                        ],
                        shift(base, 19),
                    );
                    for i in 0..5 {
                        let x = i as f32 * 0.091 - 0.19;
                        b.gem(
                            v(x, 0.622 - x * 0.13, 0.377),
                            0.032,
                            0.044,
                            0.012,
                            0.2,
                            trim,
                        );
                    }
                }
                MulberryToga => {
                    b.beam(v(-0.32, 0.934, 0.275), v(0.39, 0.77, 0.332), 0.076, trim);
                    b.beam(v(0.39, 0.77, 0.332), v(0.39, 0.51, 0.277), 0.072, trim);
                    fabric_pin(b, front + v(-0.25, 0.17, -0.06), trim, shift(base, -19));
                }
                OasisCaftan => {
                    for side in [-1.0, 1.0] {
                        b.beam(
                            front + v(side * 0.07, 0.14, -0.067),
                            front + v(side * 0.11, -0.25, -0.022),
                            0.032,
                            trim,
                        );
                        for row in 0..3 {
                            b.gem(
                                front + v(side * 0.144, row as f32 * 0.08 - 0.16, -0.02),
                                0.029,
                                0.042,
                                0.018,
                                0.4,
                                trim,
                            );
                        }
                    }
                }
                MoonSilk => {
                    for (x, y) in [(-0.19, 0.65), (0.19, 0.56), (0.29, 0.76)] {
                        b.gem(
                            v(x, y, 0.384 - x.abs() * 0.16),
                            0.040,
                            0.033,
                            0.017,
                            0.3,
                            trim,
                        );
                        b.gem(
                            v(x + 0.016, y + 0.013, 0.393 - x.abs() * 0.16),
                            0.035,
                            0.024,
                            0.014,
                            0.5,
                            base,
                        );
                    }
                }
                _ => {}
            }
        }
        SakuraKosode => {
            kosode(b, base, trim, true);
            for (x, y, z) in [
                (-0.34, 0.81, 0.28),
                (0.19, 0.639, 0.36),
                (0.34, 0.61, 0.256),
            ] {
                for petal in 0..5 {
                    let a = petal as f32 * TAU / 5.0;
                    b.gem(
                        v(x + a.cos() * 0.039, y + a.sin() * 0.039, z),
                        0.029,
                        0.032,
                        0.019,
                        a,
                        trim,
                    );
                }
            }
        }
        MinoCape => {
            kosode(b, shift(base, -21), trim, false);
            for row in 0..3 {
                for i in 0..13 {
                    let a = (i as f32 / 12.0 + 0.5) * std::f32::consts::PI;
                    let y = 0.96 - row as f32 * 0.108;
                    let r = 0.34 + row as f32 * 0.046;
                    let top = v(r * a.sin(), y, r * a.cos() - 0.11);
                    let bottom = top + v(a.sin() * 0.09, -0.156, a.cos() * 0.104);
                    b.beam(top, bottom, 0.04, shift(base, (i % 5) as i16 * 9 - 14));
                    b.beam(top.midpoint(bottom), bottom, 0.025, trim);
                }
            }
        }
        SeaCloak | TabletDress => {
            wool_coat(b, base, trim, item == SeaCloak);
            for i in 0..6 {
                let x = i as f32 * 0.087 - 0.216;
                let z = 0.413 - x.abs() * 0.27;
                if item == SeaCloak {
                    b.gem(
                        v(x, 0.915, z - 0.079),
                        0.071,
                        0.067,
                        0.055,
                        0.36 * i as f32,
                        trim,
                    );
                } else {
                    b.block(
                        v(x, 0.626, z),
                        v(0.05, 0.05, 0.02),
                        Quat::from_rotation_z(0.75),
                        trim,
                    );
                    b.beam(v(x, 0.75, z), v(x, 0.67, z), 0.01, shift(trim, 16));
                }
            }
        }
        MyrtleCrown | GoldLaurel | KnitBand => {
            for i in 0..18 {
                let a = i as f32 * TAU / 18.0;
                let p = v(a.cos() * 0.339, 1.55 + a.sin() * 0.044, a.sin() * 0.304);
                b.block(
                    p,
                    v(0.112, 0.047, 0.036),
                    Quat::from_rotation_y(-a - 1.13) * Quat::from_rotation_z(0.29),
                    shift(base, (i % 3) as i16 * 11),
                );
                if item != KnitBand {
                    let form = if item == MyrtleCrown { 0.024 } else { 0.039 };
                    b.gem(
                        p + v(a.cos() * 0.016, 0.043, a.sin() * 0.022),
                        form,
                        0.040,
                        0.023,
                        a,
                        trim,
                    );
                }
                if item == MyrtleCrown && i % 3 == 0 {
                    b.gem(p + Vec3::Y * 0.071, 0.045, 0.038, 0.043, a, trim);
                }
            }
            if item == KnitBand {
                fabric_pin(b, v(0.048, 1.57, 0.327), trim, base);
            }
        }
        FoldedPetasos | RiverKasa => {
            let wide = item == RiverKasa;
            b.frustum(cap, 0.474, 0.485, 0.039, 12, shift(base, -12));
            b.frustum(
                cap + Vec3::Y * 0.04,
                0.485,
                if wide { 0.115 } else { 0.279 },
                0.142,
                12,
                base,
            );
            b.frustum(
                cap + Vec3::Y * 0.18,
                if wide { 0.115 } else { 0.279 },
                0.089,
                0.031,
                12,
                trim,
            );
            for i in 0..12 {
                let d = Vec2::from_angle(i as f32 * TAU / 12.0);
                b.beam(
                    cap + v(d.x * 0.456, 0.057, d.y * 0.456),
                    cap + v(d.x * 0.18, 0.16, d.y * 0.18),
                    0.012,
                    shift(trim, -15),
                );
            }
            if !wide {
                b.block(
                    cap + v(0.36, 0.096, 0.05),
                    v(0.15, 0.14, 0.38),
                    Quat::from_rotation_z(-0.51),
                    trim,
                );
            }
        }
        RedPileus => {
            b.frustum(cap, 0.364, 0.351, 0.061, 14, trim);
            b.frustum(cap + Vec3::Y * 0.054, 0.351, 0.18, 0.199, 14, base);
            b.gem(
                cap + v(0.037, 0.22, -0.026),
                0.153,
                0.117,
                0.152,
                0.31,
                shift(base, 10),
            );
        }
        CranePin => {
            let tip = cap + v(0.35, 0.13, 0.09);
            b.beam(
                tip - v(0.01, 0.304, 0.01),
                tip + v(-0.09, 0.06, 0.0),
                0.025,
                trim,
            );
            b.gem(tip, 0.102, 0.041, 0.08, 0.36, base);
            b.beam(tip, tip + v(-0.079, 0.081, -0.01), 0.026, base);
            b.beam(
                tip + v(-0.079, 0.081, -0.01),
                tip + v(-0.109, 0.072, 0.032),
                0.015,
                trim,
            );
            for side in [-1.0, 1.0] {
                b.face(
                    &[
                        tip,
                        tip + v(0.129, 0.047, side * 0.094),
                        tip + v(0.084, -0.071, side * 0.12),
                    ],
                    shift(base, 11),
                );
                b.beam(
                    tip + v(0.072, -0.05, side * 0.09),
                    tip + v(0.121, 0.039, side * 0.079),
                    0.012,
                    trim,
                );
            }
        }
        FurCap => {
            b.frustum(cap, 0.401, 0.399, 0.087, 13, trim);
            b.gem(cap + Vec3::Y * 0.131, 0.359, 0.163, 0.339, 0.60, base);
            for side in [-1.0, 1.0] {
                b.gem(
                    cap + v(side * 0.36, -0.065, 0.034),
                    0.077,
                    0.152,
                    0.104,
                    0.2,
                    trim,
                );
                b.beam(
                    cap + v(side * 0.363, -0.141, 0.072),
                    cap + v(side * 0.353, -0.245, 0.089),
                    0.026,
                    base,
                );
            }
        }
        StarTurban | CaravanVeil => {
            b.gem(cap + Vec3::Y * 0.095, 0.405, 0.192, 0.366, 0.25, base);
            for i in 0..16 {
                let angle = i as f32 * TAU / 16.0;
                let p = cap
                    + v(
                        angle.cos() * 0.361,
                        0.004 + angle.sin() * 0.04,
                        angle.sin() * 0.343,
                    );
                b.block(
                    p,
                    v(0.136, 0.048, 0.053),
                    Quat::from_rotation_y(-angle) * Quat::from_rotation_z(0.52),
                    shift(base, (i % 4) as i16 * 10),
                );
            }
            if item == StarTurban {
                fabric_pin(b, cap + v(0.016, 0.126, 0.371), trim, [109, 165, 171]);
                b.beam(
                    cap + v(0.014, 0.15, 0.343),
                    cap + v(0.12, 0.33, 0.246),
                    0.049,
                    trim,
                );
            } else {
                for side in [-1.0, 1.0] {
                    let edge = cap + v(side * 0.35, -0.035, 0.14);
                    b.face(
                        &[
                            edge,
                            cap + v(side * 0.28, 0.058, -0.21),
                            cap + v(side * 0.39, -0.377, -0.175),
                            cap + v(side * 0.34, -0.249, 0.225),
                        ],
                        trim,
                    );
                    b.beam(edge, cap + v(side * 0.34, -0.249, 0.225), 0.029, base);
                }
            }
        }
        ShellPin | SignetCord | RicePouch | KnotPendant | RoseFlask => {
            collection_accessory(b, item, base, trim);
        }
        _ => {}
    }
}

fn collection_accessory(b: &mut Batch, item: Cosmetic, base: Tint, trim: Tint) {
    use Cosmetic::*;
    let center = v(0.042, 0.737, 0.415);
    if matches!(item, RicePouch | RoseFlask) {
        let at = v(0.392, 0.60, 0.235);
        b.beam(
            v(-0.292, 0.934, 0.267),
            at + v(-0.032, 0.10, 0.074),
            0.027,
            trim,
        );
        b.beam(v(-0.287, 0.94, -0.236), v(0.36, 0.608, -0.302), 0.027, base);
        if item == RicePouch {
            b.gem(at, 0.136, 0.133, 0.124, 0.29, base);
            b.beam(
                at + v(-0.087, 0.087, 0.071),
                at + v(0.084, 0.069, 0.078),
                0.039,
                trim,
            );
            for y in [0.55, 0.58, 0.61] {
                b.beam(v(0.372, y, 0.344), v(0.413, y - 0.009, 0.345), 0.007, trim);
            }
        } else {
            b.gem(at, 0.098, 0.145, 0.089, 0.31, base);
            b.frustum(at + Vec3::Y * 0.095, 0.042, 0.042, 0.047, 10, trim);
            b.gem(at + v(0.0, 0.0, 0.089), 0.038, 0.052, 0.017, 0.32, trim);
        }
    } else if item == ShellPin {
        let at = v(-0.259, 0.847, 0.32);
        b.gem(at, 0.089, 0.074, 0.038, 0.31, trim);
        for i in 0..5 {
            let a = i as f32 / 4.0 * std::f32::consts::PI;
            b.beam(
                at + v(0.0, -0.016, 0.039),
                at + v(a.cos() * 0.081, a.sin() * 0.064, 0.031),
                0.019,
                base,
            );
        }
    } else {
        b.beam(v(-0.21, 0.937, 0.29), center, 0.018, trim);
        b.beam(v(0.25, 0.937, 0.288), center, 0.018, trim);
        if item == SignetCord {
            fabric_pin(b, center, base, trim);
            b.block(
                center + Vec3::Z * 0.027,
                v(0.035, 0.035, 0.025),
                Quat::from_rotation_z(0.65),
                shift(base, 18),
            );
        } else {
            for i in 0..3 {
                let a = i as f32 * TAU / 3.0;
                let at = center + v(a.cos() * 0.035, a.sin() * 0.041 - 0.01, 0.011);
                b.gem(at, 0.044, 0.046, 0.022, a, base);
                b.gem(at + Vec3::Z * 0.019, 0.019, 0.028, 0.012, a, trim);
            }
        }
    }
}

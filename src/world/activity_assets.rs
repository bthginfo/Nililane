use bevy::prelude::*;

use crate::{adventure::Build, campaign::Region, model::Kind};

use super::{
    buildings::amphora,
    mesh::{Batch, Tint, shift, v},
    upgrades::crate_box,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(super) enum Craft {
    Home,
    Draw,
    Bake,
    Garden,
    Clay,
    Table,
    Store,
    Press,
    Offering,
    Bath,
    Market,
    Write,
    Lantern,
    Nets,
    Saw,
    Weave,
    Hive,
    Dye,
    Mill,
    Barrel,
    Signal,
    Glass,
    Scent,
    Brick,
    Forge,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(super) struct Profile {
    pub craft: Craft,
    pub variation: u8,
}

impl Profile {
    pub fn of(build: Build) -> Self {
        use Craft::*;
        use Kind::*;
        let (craft, variation) = match build {
            Build::Sawmill => (Saw, 1),
            Build::Brickyard => (Brick, 0),
            Build::Forge => (Forge, 0),
            Build::Monument => (Offering, 1),
            Build::Harbor(kind) => match kind {
                House => (Home, 0),
                Villa => (Home, 1),
                Machiya => (Home, 2),
                Longhouse => (Home, 3),
                OasisHome => (Home, 4),
                Well => (Draw, 0),
                Aqueduct => (Draw, 1),
                Cistern => (Draw, 2),
                Qanat => (Draw, 3),
                Bakery => (Bake, 0),
                RiceKitchen => (Bake, 1),
                Smokehouse => (Bake, 2),
                Grove => (Garden, 0),
                HerbGarden => (Garden, 1),
                RicePaddy => (Garden, 2),
                TeaGarden => (Garden, 3),
                Woodlot => (Saw, 0),
                MarketGarden => (Garden, 4),
                Sheepfold => (Garden, 5),
                DateGarden => (Garden, 6),
                Potter => (Clay, 0),
                CeramicStudio => (Clay, 1),
                Stoa => (Table, 0),
                Taverna => (Table, 1),
                Inn => (Table, 2),
                TeaHouse => (Table, 3),
                FeastHall => (Table, 4),
                GardenCourt => (Table, 5),
                MoonGarden => (Table, 6),
                Storehouse => (Store, 0),
                FerryOffice => (Store, 1),
                Caravanserai => (Store, 2),
                Boatyard => (Saw, 2),
                Kind::Press => (Craft::Press, 0),
                Shrine => (Offering, 0),
                Temple => (Offering, 1),
                Baths => (Bath, 0),
                Forum => (Market, 0),
                SpiceBazaar => (Market, 1),
                Scribe => (Write, 0),
                PaperWorkshop => (Lantern, 0),
                Fishery => (Nets, 0),
                Weaver => (Weave, 0),
                SilkShop => (Weave, 1),
                CarpetHall => (Weave, 2),
                Apiary => (Hive, 0),
                Dyer => (Dye, 0),
                Kind::Mill => (Craft::Mill, 0),
                Cooperage => (Barrel, 0),
                SignalTower => (Signal, 0),
                Glassworks => (Glass, 0),
                Perfumer => (Scent, 0),
            },
        };
        Self { craft, variation }
    }

    pub fn seating(self) -> bool {
        self.craft == Craft::Table
    }
}

#[derive(Clone, Copy)]
pub(super) enum Movement {
    Still,
    Spin(f32),
    Swing(Vec3, f32, f32),
    Slide(Vec3, f32),
    Smoke(u8),
    Fleck(u8, f32),
}

pub(super) struct Part {
    pub mesh: Mesh,
    pub at: Vec3,
    pub movement: Movement,
}

pub(super) struct Station {
    pub parts: Vec<Part>,
    pub person: Vec3,
    pub visitor: Option<Vec3>,
}

struct Palette {
    wood: Tint,
    woven: Tint,
    accent: Tint,
    ceramic: Tint,
    stone: Tint,
}

fn palette(region: Region) -> Palette {
    let (wood, woven, accent, ceramic, stone) = match region {
        Region::Greece => (
            [164, 106, 60],
            [240, 217, 166],
            [46, 116, 146],
            [172, 83, 56],
            [208, 197, 162],
        ),
        Region::Rome => (
            [139, 86, 56],
            [234, 209, 162],
            [154, 62, 63],
            [199, 105, 63],
            [216, 188, 147],
        ),
        Region::Japan => (
            [111, 78, 62],
            [233, 222, 173],
            [72, 100, 136],
            [85, 146, 129],
            [156, 167, 153],
        ),
        Region::Nordic => (
            [124, 88, 63],
            [207, 188, 147],
            [60, 125, 125],
            [174, 99, 75],
            [152, 173, 173],
        ),
        Region::Oasis => (
            [173, 114, 62],
            [239, 203, 134],
            [62, 148, 135],
            [205, 124, 64],
            [228, 180, 123],
        ),
    };
    Palette {
        wood,
        woven,
        accent,
        ceramic,
        stone,
    }
}

fn table(b: &mut Batch, height: f32, depth: f32, colors: &Palette) {
    let center = v(0.0, height, 0.0);
    b.slab(center, v(0.91, 0.083, depth), colors.wood);
    for direction in [-1.0, 1.0] {
        b.slab(
            v(direction * 0.37, height * 0.49, 0.0),
            v(0.073, height, depth * 0.83),
            shift(colors.wood, -15),
        );
    }
    b.slab(
        center + v(0.0, 0.049, 0.0),
        v(0.27, 0.017, depth * 1.03),
        colors.woven,
    );
    for z in [-0.36, 0.36] {
        b.slab(
            center + v(0.0, 0.052, z * depth),
            v(0.27, 0.023, 0.026),
            colors.accent,
        );
    }
}

fn bowl(b: &mut Batch, p: Vec3, r: f32, color: Tint) {
    b.frustum(p, r * 0.65, r, r * 0.46, 9, color);
    b.frustum(
        p + Vec3::Y * (r * 0.44),
        r,
        r * 0.98,
        0.014,
        9,
        shift(color, 25),
    );
}

fn brazier(b: &mut Batch, p: Vec3, colors: &Palette) {
    b.frustum(p, 0.195, 0.15, 0.13, 8, colors.stone);
    b.frustum(
        p + Vec3::Y * 0.12,
        0.15,
        0.25,
        0.17,
        8,
        shift(colors.stone, -39),
    );
    b.frustum(p + Vec3::Y * 0.292, 0.22, 0.22, 0.017, 9, [91, 57, 42]);
    for i in 0..5 {
        let a = i as f32 * 2.4;
        b.gem(
            p + v(a.cos() * 0.103, 0.313, a.sin() * 0.10),
            0.055,
            0.038,
            0.047,
            a,
            [226, 115 + i * 12, 54],
        );
    }
}

fn trestle(b: &mut Batch, colors: &Palette) {
    for x in [-0.29, 0.29] {
        for z in [-1.0, 1.0] {
            b.beam(
                v(x, 0.036, z * 0.21),
                v(x, 0.34, -z * 0.107),
                0.063,
                colors.wood,
            );
        }
        b.beam(
            v(x, 0.33, -0.19),
            v(x, 0.33, 0.19),
            0.065,
            shift(colors.wood, 10),
        );
    }
}

fn log(b: &mut Batch, x: f32, z: f32, y: f32, color: Tint) {
    let cut = v(x + 0.43, y, z);
    b.beam(v(x - 0.43, y, z), cut, 0.156, color);
    b.block(
        cut + Vec3::X * 0.009,
        v(0.018, 0.137, 0.137),
        Quat::IDENTITY,
        [221, 160, 100],
    );
    b.block(
        cut + Vec3::X * 0.02,
        v(0.016, 0.051, 0.055),
        Quat::IDENTITY,
        [160, 102, 66],
    );
}

fn part(parts: &mut Vec<Part>, b: Batch, at: Vec3, movement: Movement) {
    parts.push(Part {
        mesh: b.mesh(),
        at,
        movement,
    });
}

fn smoke(parts: &mut Vec<Part>, origin: Vec3, color: Tint) {
    for i in 0..3 {
        let mut p = Batch::default();
        p.gem(
            v(0.0, 0.0, 0.0),
            0.109,
            0.128,
            0.097,
            i as f32 * 0.6,
            shift(color, 8 * i as i16),
        );
        p.gem(
            v(0.078, -0.027, -0.01),
            0.079,
            0.09,
            0.082,
            0.3,
            shift(color, -14),
        );
        part(parts, p, origin, Movement::Smoke(i));
    }
}

fn flecks(parts: &mut Vec<Part>, at: Vec3, tint: Tint, rate: f32) {
    for i in 0..3 {
        let mut p = Batch::default();
        p.gem(Vec3::ZERO, 0.033, 0.046, 0.028, i as f32 * 0.7, tint);
        part(parts, p, at, Movement::Fleck(i, rate));
    }
}

pub(super) fn station(profile: Profile, region: Region) -> Station {
    use Craft::*;
    let colors = palette(region);
    let variety = profile.variation;
    let mut base = Batch::default();
    let mut parts = Vec::with_capacity(5);
    let mut person = v(-0.23, 0.0, -0.50);
    let mut visitor = None;
    match profile.craft {
        Home => {
            table(&mut base, 0.39, 0.41, &colors);
            amphora(&mut base, v(0.32, 0.01, 0.41), 0.62, colors.ceramic);
            let mut fabric = Batch::default();
            for i in 0..4 {
                fabric.slab(
                    v(0.0, 0.023 * i as f32, 0.0),
                    v(0.24 - 0.02 * i as f32, 0.034, 0.22),
                    if (i + variety).is_multiple_of(2) {
                        colors.woven
                    } else {
                        colors.accent
                    },
                );
            }
            part(
                &mut parts,
                fabric,
                v(-0.06, 0.46, -0.055),
                Movement::Slide(v(0.12, 0.052, 0.0), 2.2),
            );
        }
        Draw | Bath => {
            let bath = profile.craft == Bath;
            let radius = if bath {
                0.36
            } else {
                0.24 + 0.023 * f32::from(variety)
            };
            base.frustum(Vec3::ZERO, radius * 0.86, radius, 0.24, 10, colors.stone);
            base.frustum(
                v(0.0, 0.22, 0.0),
                radius * 0.87,
                radius * 0.87,
                0.014,
                10,
                [70, 158, 177],
            );
            let mut bucket = Batch::default();
            bucket.frustum(Vec3::ZERO, 0.074, 0.10, 0.14, 8, colors.wood);
            bucket.beam(v(-0.07, 0.15, 0.0), v(0.07, 0.15, 0.0), 0.026, colors.woven);
            part(
                &mut parts,
                bucket,
                v(0.05, 0.26, 0.015),
                Movement::Slide(v(0.09, 0.19, 0.075), 2.7),
            );
            flecks(&mut parts, v(0.04, 0.29, 0.05), [115, 193, 203], 0.95);
        }
        Table => {
            let meals = matches!(variety, 1 | 3 | 4);
            table(&mut base, 0.40, 0.43, &colors);
            for (x, z) in [(-0.27, -0.09), (0.21, 0.11)] {
                bowl(&mut base, v(x, 0.451, z), 0.107, colors.ceramic);
                if meals {
                    for i in 0..3 {
                        base.gem(
                            v(x + i as f32 * 0.035 - 0.048, 0.514, z),
                            0.044,
                            0.025,
                            0.05,
                            i as f32,
                            [[208, 136, 69], [123, 153, 78], [231, 191, 119]][i],
                        );
                    }
                }
            }
            base.slab(v(0.17, 0.212, 0.462), v(0.51, 0.079, 0.21), colors.wood);
            base.slab(
                v(0.17, 0.099, 0.462),
                v(0.41, 0.17, 0.14),
                shift(colors.wood, -10),
            );
            visitor = Some(v(0.18, 0.225, 0.48));
            if meals {
                smoke(&mut parts, v(0.215, 0.54, 0.11), [203, 197, 174]);
            } else {
                let mut cup = Batch::default();
                cup.frustum(Vec3::ZERO, 0.038, 0.049, 0.081, 8, colors.accent);
                part(
                    &mut parts,
                    cup,
                    v(-0.02, 0.454, 0.087),
                    Movement::Slide(v(0.062, 0.021, 0.014), 2.1),
                );
            }
        }
        Garden | Hive => {
            let hue = [
                [93, 141, 76],
                [143, 164, 87],
                [113, 169, 110],
                [72, 131, 92],
                [143, 157, 70],
                [224, 193, 133],
                [124, 153, 82],
            ][variety as usize];
            base.slab(
                v(0.0, 0.035, 0.0),
                v(0.89, 0.10, 0.50),
                shift(colors.wood, -12),
            );
            for x in [-0.27, 0.0, 0.27] {
                if profile.craft == Hive {
                    let p = v(x, 0.15, 0.0);
                    base.frustum(p, 0.118, 0.075, 0.20, 8, [197, 143, 71]);
                    base.frustum(
                        p + v(0.0, 0.19, 0.0),
                        0.123,
                        0.062,
                        0.07,
                        8,
                        [221, 179, 105],
                    );
                    base.slab(
                        p + v(0.0, 0.073, 0.107),
                        v(0.080, 0.025, 0.013),
                        [90, 60, 43],
                    );
                    continue;
                }
                let mut stalks = Batch::default();
                for i in 0..4 {
                    let x = (i as f32 - 1.5) * 0.074;
                    let h = 0.17 + (i % 3) as f32 * 0.048;
                    stalks.beam(
                        v(x, 0.0, 0.0),
                        v(x + 0.052, h, 0.015),
                        0.023,
                        shift(hue, -18),
                    );
                    stalks.gem(v(x + 0.04, h, 0.026), 0.075, 0.065, 0.048, i as f32, hue);
                }
                part(
                    &mut parts,
                    stalks,
                    v(x, 0.098, 0.0),
                    Movement::Swing(Vec3::Z, 0.13, 2.0 + x),
                );
            }
            if profile.craft == Hive {
                flecks(&mut parts, v(0.0, 0.33, -0.07), [223, 166, 67], 0.62);
            } else {
                bowl(&mut base, v(0.43, 0.05, 0.30), 0.145, colors.woven);
            }
        }
        Saw | Barrel => {
            trestle(&mut base, &colors);
            log(&mut base, 0.0, -0.044, 0.411, colors.wood);
            for z in [0.26, 0.40] {
                log(&mut base, -0.06, z, 0.092, shift(colors.wood, -18));
            }
            let mut saw = Batch::default();
            saw.slab(v(0.0, 0.0, 0.0), v(0.69, 0.076, 0.020), [179, 191, 182]);
            for i in 0..16 {
                let x = i as f32 * 0.035 - 0.26;
                saw.face(
                    &[
                        v(x, -0.033, 0.013),
                        v(x + 0.033, -0.033, 0.013),
                        v(x + 0.019, -0.065, 0.013),
                    ],
                    [208, 217, 212],
                );
            }
            for x in [-0.36, 0.36] {
                saw.block(
                    v(x, 0.04, 0.0),
                    v(0.11, 0.135, 0.066),
                    Quat::from_rotation_z(x * 0.68),
                    colors.wood,
                );
            }
            part(
                &mut parts,
                saw,
                v(0.0, 0.497, 0.09),
                Movement::Slide(v(0.26, 0.0, 0.0), 7.1),
            );
            flecks(&mut parts, v(0.03, 0.43, 0.09), [226, 170, 104], 1.65);
            if profile.craft == Barrel {
                for y in [0.06, 0.22] {
                    base.frustum(
                        v(0.53, y, 0.19),
                        0.145,
                        0.157,
                        0.112,
                        10,
                        shift(colors.wood, 6),
                    );
                    base.frustum(
                        v(0.53, y + 0.012, 0.19),
                        0.154,
                        0.156,
                        0.027,
                        10,
                        [107, 109, 98],
                    );
                }
            }
            person = v(-0.18, 0.0, -0.51);
        }
        Clay | Mill | Press => {
            base.frustum(Vec3::ZERO, 0.265, 0.19, 0.32, 9, colors.stone);
            let mut wheel = Batch::default();
            wheel.frustum(Vec3::ZERO, 0.265, 0.265, 0.07, 11, shift(colors.stone, -29));
            for x in [-0.155, 0.155] {
                wheel.slab(v(x, 0.071, 0.0), v(0.047, 0.014, 0.155), colors.woven);
            }
            if profile.craft == Clay {
                amphora(&mut wheel, v(0.0, 0.077, 0.0), 0.69, colors.ceramic);
            } else {
                wheel.beam(v(0.0, 0.093, 0.0), v(0.37, 0.12, 0.0), 0.046, colors.wood);
            }
            part(
                &mut parts,
                wheel,
                v(0.0, 0.323, 0.0),
                Movement::Spin(if profile.craft == Clay { 3.9 } else { 2.3 }),
            );
            bowl(&mut base, v(0.38, 0.034, 0.24), 0.142, colors.ceramic);
        }
        Weave | Lantern | Dye => {
            for x in [-0.39, 0.39] {
                base.beam(v(x, 0.035, 0.035), v(x, 0.68, 0.015), 0.065, colors.wood);
                base.beam(v(x, 0.11, -0.24), v(x, 0.29, 0.042), 0.054, colors.wood);
            }
            base.beam(
                v(-0.42, 0.657, 0.015),
                v(0.42, 0.657, 0.015),
                0.069,
                colors.wood,
            );
            let mut cloth = Batch::default();
            for i in 0..7 {
                let x = i as f32 * 0.083 - 0.255;
                cloth.slab(
                    v(x, -0.19 - 0.014 * (i % 3) as f32, 0.025),
                    v(0.088, 0.39, 0.039),
                    if i % 3 == usize::from(variety) % 3 {
                        colors.woven
                    } else {
                        colors.accent
                    },
                );
            }
            if profile.craft == Lantern {
                let mut lamp = Batch::default();
                lamp.frustum(v(0.0, -0.25, 0.0), 0.16, 0.185, 0.23, 8, colors.woven);
                lamp.frustum(v(0.0, -0.253, 0.0), 0.166, 0.166, 0.03, 8, colors.accent);
                lamp.frustum(v(0.0, -0.05, 0.0), 0.18, 0.16, 0.027, 8, colors.accent);
                part(
                    &mut parts,
                    lamp,
                    v(0.0, 0.65, 0.041),
                    Movement::Swing(Vec3::Z, 0.20, 2.0),
                );
            } else {
                part(
                    &mut parts,
                    cloth,
                    v(0.0, 0.595, 0.015),
                    Movement::Swing(Vec3::X, 0.19, 3.7),
                );
                if profile.craft == Dye {
                    bowl(&mut base, v(0.13, 0.08, 0.28), 0.21, colors.accent);
                }
            }
        }
        Bake | Glass | Forge | Brick | Signal => {
            brazier(&mut base, v(0.26, 0.0, 0.0), &colors);
            trestle(&mut base, &colors);
            base.slab(v(-0.18, 0.35, 0.13), v(0.54, 0.081, 0.42), colors.wood);
            let hot = match profile.craft {
                Glass => [98, 190, 173],
                Forge => [223, 122, 62],
                Brick => colors.ceramic,
                Signal => [249, 165, 63],
                _ => [230, 169, 96],
            };
            base.gem(v(-0.20, 0.397, 0.15), 0.16, 0.088, 0.116, 0.46, hot);
            let mut handle = Batch::default();
            handle.beam(v(-0.27, 0.0, 0.0), v(0.25, 0.013, 0.0), 0.044, colors.wood);
            handle.slab(v(0.23, 0.015, 0.0), v(0.108, 0.074, 0.089), shift(hot, -20));
            part(
                &mut parts,
                handle,
                v(-0.03, 0.56, 0.07),
                Movement::Swing(Vec3::Z, 0.52, 5.3),
            );
            smoke(
                &mut parts,
                v(0.26, 0.36, 0.0),
                if variety == 2 {
                    [147, 139, 127]
                } else {
                    [183, 163, 136]
                },
            );
            if matches!(profile.craft, Forge | Signal) {
                flecks(&mut parts, v(0.21, 0.38, 0.06), [250, 175, 76], 1.5);
            }
        }
        Offering | Write | Scent | Market | Store | Nets => {
            table(&mut base, 0.36, 0.45, &colors);
            match profile.craft {
                Store | Market => {
                    for (x, z, height) in [(-0.31, 0.27, 0.11), (0.32, 0.02, 0.56)] {
                        crate_box(&mut base, v(x, height, z), v(0.25, 0.22, 0.26), colors.wood);
                        base.slab(v(x, height + 0.13, z), v(0.19, 0.042, 0.20), colors.accent);
                    }
                }
                Nets => {
                    for i in 0..5 {
                        let x = i as f32 * 0.11 - 0.22;
                        base.beam(
                            v(x, 0.44, -0.22),
                            v(x + 0.20, 0.44, 0.20),
                            0.024,
                            colors.woven,
                        );
                        base.beam(
                            v(x, 0.45, 0.21),
                            v(x + 0.20, 0.45, -0.20),
                            0.021,
                            colors.accent,
                        );
                    }
                }
                Write => {
                    base.slab(v(0.06, 0.421, -0.045), v(0.41, 0.054, 0.26), colors.woven);
                    for n in 0..4 {
                        base.slab(
                            v(0.054, 0.451, n as f32 * 0.036 - 0.09),
                            v(0.22, 0.009, 0.01),
                            colors.wood,
                        );
                    }
                }
                _ => {
                    amphora(&mut base, v(0.28, 0.43, 0.045), 0.41, colors.ceramic);
                    bowl(&mut base, v(-0.22, 0.42, -0.052), 0.15, colors.accent);
                }
            }
            let mut tool = Batch::default();
            tool.beam(v(-0.09, 0.02, 0.0), v(0.12, 0.08, 0.0), 0.027, colors.wood);
            tool.gem(
                v(-0.11, 0.009, 0.0),
                0.036,
                0.024,
                0.031,
                0.6,
                colors.accent,
            );
            part(
                &mut parts,
                tool,
                v(-0.10, 0.494, -0.092),
                Movement::Slide(v(0.20, 0.06, 0.048), 3.1),
            );
            if profile.craft == Scent {
                smoke(&mut parts, v(-0.22, 0.51, -0.052), [177, 152, 168]);
            }
        }
    }
    part(&mut parts, base, Vec3::ZERO, Movement::Still);
    Station {
        parts,
        person,
        visitor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::mesh::VertexAttributeValues;

    #[test]
    fn every_building_has_valid_workshop_geometry_in_all_regions() {
        for region in Region::ALL {
            for kind in Kind::ALL.into_iter().map(Build::Harbor).chain([
                Build::Sawmill,
                Build::Brickyard,
                Build::Forge,
                Build::Monument,
            ]) {
                let prop = station(Profile::of(kind), region);
                assert!((2..=9).contains(&prop.parts.len()));
                for part in prop.parts {
                    let Some(VertexAttributeValues::Float32x3(points)) =
                        part.mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                    else {
                        panic!("{region:?} {kind:?} is missing its workshop mesh");
                    };
                    assert!(points.len() >= 3);
                    for &point in points {
                        let position = Vec3::from_array(point) + part.at;
                        assert!(
                            position.is_finite()
                                && position.x.abs() < 1.0
                                && (-0.12..1.20).contains(&position.y)
                                && position.z.abs() < 0.81,
                            "{kind:?} {region:?} {position:?}"
                        );
                    }
                }
            }
        }
    }
}

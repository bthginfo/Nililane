use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::mesh::{
        VertexAttributeValues,
        skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    },
};

use crate::{
    campaign::{CHAPTERS, Region},
    game::{Game, View},
    layout::Layout,
    model::{Age, Miss, Origin, Phase, Service, Verdict, Visit},
    save::Campaign,
};

use super::{
    buildings::{CREAM, GOLD, MARBLE, OAK},
    mesh::{Batch, Tint, shift, v},
    motion::{self, Motion, Path},
};

const STYLES: usize = 24;
const RESIDENTS: usize = 72;
const BONES: usize = 9;
const HIP: f32 = 0.405;
const THIGH: f32 = 0.207;
const SHIN: f32 = 0.182;
const ANKLE: f32 = 0.046;

#[derive(Component)]
pub struct Joint;

#[derive(Component)]
pub struct Resident(usize);

pub const REACTION_TIME: f32 = 3.8;

pub struct Reaction {
    pub guest: usize,
    pub head: Vec3,
    pub verdict: Verdict,
    pub seconds: f32,
}

#[derive(Resource, Default)]
pub struct Reactions(pub Vec<Reaction>);

struct Outfit {
    mesh: Handle<Mesh>,
    bind: Handle<SkinnedMeshInverseBindposes>,
    rest: [Vec3; BONES],
    age: Age,
}

#[derive(Resource)]
pub struct Cast {
    clothes: [[Outfit; STYLES]; Region::ALL.len()],
}

pub fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    bind_poses: &mut Assets<SkinnedMeshInverseBindposes>,
    materials: &mut Assets<StandardMaterial>,
    matte: &Handle<StandardMaterial>,
) {
    let clothes = Region::ALL.map(|region| {
        std::array::from_fn(|index| {
            let (mesh, rest) = outfit(index, region);
            Outfit {
                mesh: meshes.add(mesh),
                bind: bind_poses.add(SkinnedMeshInverseBindposes::from(
                    rest.iter()
                        .map(|p| Mat4::from_translation(-*p))
                        .collect::<Vec<_>>(),
                )),
                rest,
                age: Age::ALL[index / 8],
            }
        })
    });
    let shadow = meshes.add(Ellipse::new(0.21, 0.15));
    let shade = materials.add(StandardMaterial {
        base_color: Color::srgba(0.19, 0.12, 0.09, 0.30),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    for i in 0..RESIDENTS {
        let look = &clothes[0][i % STYLES];
        let actor = commands
            .spawn((
                Resident(i),
                Mesh3d(look.mesh.clone()),
                MeshMaterial3d(matte.clone()),
                Transform::default(),
                Visibility::Hidden,
                NotShadowCaster,
            ))
            .id();
        let joints = look
            .rest
            .iter()
            .map(|&at| {
                commands
                    .spawn((Joint, ChildOf(actor), Transform::from_translation(at)))
                    .id()
            })
            .collect();
        commands.entity(actor).insert(SkinnedMesh {
            inverse_bindposes: look.bind.clone(),
            joints,
        });
        commands.spawn((
            ChildOf(actor),
            Mesh3d(shadow.clone()),
            MeshMaterial3d(shade.clone()),
            Transform::from_xyz(0.0, -0.016, 0.014)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            NotShadowCaster,
        ));
    }
    commands.insert_resource(Cast { clothes });
}

#[derive(Component)]
pub(crate) struct WorkerJoint;

pub(super) fn worker_rig(parent: &mut EntityCommands, cast: &Cast, index: usize, region: Region) {
    let look = &cast.clothes[region as usize][8 + index % 8];
    let mut joints = Vec::with_capacity(BONES);
    parent.with_children(|root| {
        joints.extend(look.rest.iter().map(|&at| {
            root.spawn((WorkerJoint, Transform::from_translation(at)))
                .id()
        }));
    });
    parent.insert((
        Mesh3d(look.mesh.clone()),
        SkinnedMesh {
            inverse_bindposes: look.bind.clone(),
            joints,
        },
        NotShadowCaster,
    ));
}

pub(super) fn worker_pose(
    cast: &Cast,
    index: usize,
    region: Region,
    stride: f32,
    moving: f32,
    seconds: f32,
    busy: bool,
) -> [Transform; BONES] {
    let look = &cast.clothes[region as usize][8 + index % 8];
    let mut joints = pose(look, stride, moving, seconds.sin(), None);
    if busy {
        let swing = (seconds * 4.6).sin() * 0.42;
        joints[5].rotation *= Quat::from_rotation_x(-0.88 + swing);
        joints[1].rotation *= Quat::from_rotation_x(-0.56 - swing * 0.35);
        joints[0].rotation *= Quat::from_rotation_x(0.14 + swing * 0.09);
    }
    joints
}

fn outfit(index: usize, region: Region) -> (Mesh, [Vec3; BONES]) {
    let style = index % 8;
    let age = Age::ALL[index / 8];
    let roman = region == Region::Rome;
    let japan = region == Region::Japan;
    let nordic = region == Region::Nordic;
    let oasis = region == Region::Oasis;
    let cloth = if oasis {
        [
            [55, 116, 125],
            [183, 113, 64],
            [139, 75, 106],
            [188, 146, 100],
            [81, 105, 133],
            [95, 127, 95],
            [186, 95, 73],
            [104, 80, 133],
        ][style]
    } else if nordic {
        [
            [132, 77, 63],
            [67, 110, 139],
            [137, 131, 87],
            [76, 103, 95],
            [154, 103, 68],
            [163, 123, 112],
            [96, 95, 132],
            [151, 82, 74],
        ][style]
    } else if japan {
        [
            [66, 93, 146],
            [166, 79, 86],
            [212, 169, 96],
            [79, 131, 114],
            [157, 106, 152],
            [93, 105, 141],
            [174, 114, 75],
            [98, 142, 94],
        ][style]
    } else if roman {
        [
            [198, 162, 124],
            [167, 57, 65],
            [67, 112, 110],
            [116, 137, 88],
            [139, 88, 125],
            [62, 100, 138],
            [191, 133, 65],
            [141, 80, 54],
        ][style]
    } else {
        [
            [47, 122, 137],
            [191, 104, 86],
            [192, 143, 74],
            [78, 107, 151],
            [151, 96, 144],
            [100, 129, 71],
            [214, 147, 78],
            [56, 119, 110],
        ][style]
    };
    let skin = [
        [233, 179, 137],
        [183, 119, 84],
        [240, 197, 155],
        [158, 106, 79],
        [204, 145, 102],
        [122, 83, 65],
    ][(index * 7 + index / 3) % 6];
    let hair = if age == Age::Senior {
        [[186, 175, 158], [157, 151, 141], [211, 202, 178]][style % 3]
    } else if japan || oasis {
        [[48, 40, 37], [66, 49, 42], [42, 37, 35], [78, 56, 43]][style % 4]
    } else {
        [
            [73, 48, 36],
            [123, 75, 46],
            [46, 35, 29],
            [83, 54, 41],
            [168, 109, 61],
            [164, 151, 132],
        ][(index * 3 + index / 2) % 6]
    };
    let width = 0.148 + (index % 5) as f32 * 0.013 - 0.020 * f32::from(age == Age::Child);
    let delicate = matches!(style, 1 | 4 | 7);
    let hem = if oasis {
        0.185
    } else if delicate {
        0.162
    } else {
        0.259
    } + (index % 3) as f32 * 0.012;
    let belt = if oasis {
        [
            [227, 196, 147],
            [182, 108, 66],
            [212, 175, 99],
            [121, 62, 83],
        ][style % 4]
    } else if japan {
        [
            [195, 147, 79],
            [88, 119, 123],
            [205, 186, 150],
            [146, 78, 90],
        ][style % 4]
    } else if roman {
        [157, 62, 48]
    } else {
        shift(cloth, -29)
    };
    let mantle = if roman && matches!(style, 1 | 3 | 5 | 7) {
        [236, 216, 182]
    } else {
        shift(cloth, 36)
    };
    let mut b = Batch::default();
    let mut end = [0; BONES];
    let mut joint = [Vec3::ZERO; BONES];
    joint[0] = v(0.0, HIP, 0.0);
    let pleats = [
        (hem, width * 1.15, 0.132),
        (hem + 0.038, width * 1.16, 0.135),
        (0.46, width * 0.84, 0.105),
        (0.58, width * 1.11, 0.122),
        (0.655, width * 0.69, 0.099),
    ];
    for (i, pair) in pleats.windows(2).enumerate() {
        let [(y0, x0, z0), (y1, x1, z1)] = [pair[0], pair[1]];
        wrap(
            &mut b,
            (y0, x0, z0),
            (y1, x1, z1),
            if i == 0 { mantle } else { cloth },
            12,
            true,
        );
    }
    wrap(
        &mut b,
        (0.458, width * 0.88, 0.113),
        (0.489, width * 0.87, 0.120),
        belt,
        12,
        false,
    );
    b.gem(v(0.044, 0.475, 0.127), 0.022, 0.020, 0.010, 0.18, GOLD);

    if nordic {
        let wool = shift(cloth, -12);
        b.gem(v(0.0, 0.60, -0.05), width * 1.19, 0.113, 0.138, 0.49, wool);
        for sign in [-1.0, 1.0] {
            b.beam(
                v(sign * width * 0.77, 0.636, 0.083),
                v(sign * width * 0.94, 0.445, 0.113),
                0.037,
                mantle,
            );
            b.gem(
                v(sign * width * 0.67, 0.566, 0.133),
                0.034,
                0.040,
                0.016,
                sign,
                GOLD,
            );
        }
        if matches!(style, 0 | 2 | 3 | 7) {
            let mut cloak = [
                v(-width * 1.17, 0.57, -0.08),
                v(-width * 1.04, 0.214, -0.165),
                v(width * 1.10, 0.274, -0.171),
                v(width * 1.25, 0.61, -0.089),
            ];
            cloak.reverse();
            b.face(&cloak, wool);
            b.beam(
                v(-width * 1.04, 0.216, -0.165),
                v(width * 1.10, 0.274, -0.171),
                0.030,
                mantle,
            );
        }
        for x in [-0.082, -0.028, 0.026, 0.080] {
            b.gem(
                v(x, hem + 0.043, 0.133),
                0.025,
                0.019,
                0.011,
                x * 12.0,
                GOLD,
            );
        }
    } else if japan {
        // Kosode lapels cross at the upper edge of the obi.
        b.beam(
            v(-width * 0.57, 0.647, 0.092),
            v(width * 0.42, 0.486, 0.137),
            0.036,
            mantle,
        );
        b.beam(
            v(width * 0.56, 0.646, 0.094),
            v(-width * 0.57, 0.428, 0.135),
            0.041,
            CREAM,
        );
        wrap(
            &mut b,
            (0.427, width * 0.87, 0.116),
            (0.482, width * 0.89, 0.126),
            belt,
            12,
            false,
        );
        b.slab(
            v(0.0, 0.459, -0.132),
            v(0.155, 0.072, 0.068),
            shift(belt, 8),
        );
        if matches!(style, 0 | 3 | 5) {
            for i in -2..=2 {
                let x = i as f32 * width * 0.35;
                b.beam(
                    v(x * 0.78, 0.425, 0.117),
                    v(x, hem + 0.023, 0.133),
                    0.015,
                    shift(cloth, 19),
                );
            }
        } else {
            for (x, y) in [(-0.068, 0.30), (0.066, 0.25), (0.093, 0.36)] {
                b.gem(v(x, y, 0.139), 0.025, 0.022, 0.006, x * 18.0, mantle);
            }
        }
    } else if roman || matches!(style, 0 | 1 | 4 | 7) {
        let a = v(-width * 0.82, 0.652, 0.078);
        let center = v(-width * 0.26, 0.479, 0.139);
        let lower = v(width * 0.94, 0.303, 0.127);
        let back = v(-width * 0.68, 0.563, -0.129);
        b.face(
            &[
                a,
                center,
                v(-width * 0.54, 0.443, 0.115),
                v(-width * 1.13, 0.54, 0.050),
            ],
            mantle,
        );
        b.face(
            &[
                v(-width * 0.54, 0.443, 0.115),
                center,
                v(width * 0.98, 0.416, 0.124),
                lower,
            ],
            shift(mantle, -8),
        );
        b.face(
            &[
                a,
                v(-width * 1.13, 0.54, 0.050),
                back,
                v(-width * 0.49, 0.65, -0.071),
            ],
            shift(mantle, -15),
        );
        b.beam(
            center + v(0.0, 0.013, 0.003),
            lower + v(0.0, 0.055, 0.002),
            0.018,
            shift(mantle, 17),
        );
        b.gem(a + v(0.016, -0.026, 0.023), 0.027, 0.029, 0.013, 0.2, GOLD);
    }
    b.frustum(v(0.0, 0.61, 0.014), 0.058, 0.062, 0.103, 8, skin);
    head(&mut b, index, region, skin, hair, cloth);
    if (japan && matches!(style, 2 | 6))
        || (!japan && (age == Age::Child || matches!(style, 0 | 2 | 6)))
    {
        b.beam(
            v(width * 0.66, 0.594, 0.127),
            v(-width * 0.63, 0.414, 0.127),
            0.021,
            OAK,
        );
        b.block(
            v(-width * 0.82, 0.379, 0.142),
            v(0.117, 0.125, 0.075),
            Quat::from_rotation_z(-0.16),
            if style == 2 { CREAM } else { OAK },
        );
        b.gem(
            v(-width * 0.82, 0.396, 0.184),
            0.021,
            0.015,
            0.009,
            0.1,
            GOLD,
        );
    }
    end[0] = b.vertices();

    for (side_index, side) in [-1.0, 1.0].into_iter().enumerate() {
        let base = 1 + side_index * 4;
        let shoulder = v(side * width * 1.06, 0.590, 0.013);
        b.gem(
            shoulder + v(0.0, -0.039, 0.0),
            0.068,
            0.086,
            0.074,
            0.17,
            if side < 0.0 && roman { mantle } else { cloth },
        );
        if nordic {
            b.beam(
                shoulder + v(0.0, -0.035, 0.005),
                shoulder + v(side * 0.023, -0.184, 0.033),
                0.095,
                shift(cloth, 7),
            );
            b.slab(
                shoulder + v(side * 0.028, -0.176, 0.033),
                v(0.105, 0.031, 0.104),
                mantle,
            );
        } else if japan {
            b.block(
                shoulder + v(side * 0.028, -0.085, 0.002),
                v(0.128, 0.171, 0.145),
                Quat::from_rotation_z(side * 0.11),
                cloth,
            );
            b.slab(
                shoulder + v(side * 0.030, -0.165, 0.008),
                v(0.119, 0.028, 0.132),
                mantle,
            );
        }
        b.beam(
            shoulder + v(0.0, -0.079, 0.0),
            shoulder + v(side * 0.021, -0.172, 0.033),
            0.074,
            skin,
        );
        b.gem(
            shoulder + v(side * 0.022, -0.180, 0.032),
            0.040,
            0.052,
            0.043,
            0.35,
            skin,
        );
        b.gem(
            shoulder + v(side * 0.023, -0.225, 0.048),
            0.037,
            0.055,
            0.036,
            0.10,
            skin,
        );
        b.gem(
            shoulder + v(side * 0.022, -0.272, 0.066),
            0.038,
            0.042,
            0.032,
            0.25,
            skin,
        );
        if delicate {
            b.gem(
                shoulder + v(side * 0.022, -0.230, 0.049),
                0.041,
                0.014,
                0.043,
                0.30,
                GOLD,
            );
        }
        joint[base] = shoulder;
        end[base] = b.vertices();

        let x = side * width * 0.51;
        let hip = v(x, HIP, 0.0);
        let knee = hip - Vec3::Y * THIGH;
        let shin = knee - Vec3::Y * SHIN;
        b.frustum(
            knee - v(0.0, 0.012, 0.0),
            0.050,
            0.068,
            THIGH + 0.018,
            8,
            skin,
        );
        joint[base + 1] = hip;
        end[base + 1] = b.vertices();

        b.frustum(shin, 0.035, 0.054, SHIN + 0.020, 8, skin);
        b.gem(
            knee - v(0.0, 0.047, -0.005),
            0.052,
            0.109,
            0.051,
            0.22,
            skin,
        );
        joint[base + 2] = knee;
        end[base + 2] = b.vertices();

        b.gem(shin + v(0.0, -0.021, 0.044), 0.056, 0.027, 0.096, 0.2, OAK);
        b.gem(
            shin + v(0.0, -0.003, 0.038),
            0.047,
            0.035,
            0.076,
            0.22,
            skin,
        );
        for z in [0.021, 0.064] {
            b.beam(
                shin + v(-0.037, 0.014, z),
                shin + v(0.036, 0.016, z + 0.024),
                0.020,
                shift(OAK, 26),
            );
        }
        joint[base + 3] = shin;
        end[base + 3] = b.vertices();
    }

    let mut indices = vec![[0_u16; 4]; b.vertices()];
    let mut begin = 0;
    for (bone, end) in end.into_iter().enumerate() {
        indices[begin..end].fill([bone as u16, 0, 0, 0]);
        begin = end;
    }
    let weights = vec![[1.0_f32, 0.0, 0.0, 0.0]; indices.len()];
    let mesh = b
        .mesh()
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_JOINT_INDEX,
            VertexAttributeValues::Uint16x4(indices),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT, weights);
    (mesh, joint)
}

fn wrap(
    b: &mut Batch,
    base: (f32, f32, f32),
    top: (f32, f32, f32),
    tint: Tint,
    sides: usize,
    folds: bool,
) {
    let ring = |i: usize, (y, rx, rz): (f32, f32, f32)| {
        let angle = (i as f32 + 0.5) / sides as f32 * std::f32::consts::TAU;
        let pleat = if folds && i % 2 == 1 { 0.97 } else { 1.016 };
        v(angle.cos() * rx * pleat, y, angle.sin() * rz * pleat)
    };
    for i in 0..sides {
        b.face(
            &[
                ring(i, base),
                ring(i, top),
                ring(i + 1, top),
                ring(i + 1, base),
            ],
            if folds {
                shift(tint, (i * 13 % 21) as i16 - 8)
            } else {
                tint
            },
        );
    }
}

fn head(b: &mut Batch, index: usize, region: Region, skin: Tint, hair: Tint, cloth: Tint) {
    let age = Age::ALL[index / 8];
    let w = 0.130 + (index % 4) as f32 * 0.009 + 0.023 * f32::from(age == Age::Child);
    let style = index % 8;
    let rings = [
        (0.685, w * 0.50, 0.072),
        (0.732, w * 0.82, 0.108),
        (0.799, w, 0.123),
        (0.863, w * 0.96, 0.117),
        (0.922, w * 0.68, 0.087),
    ];
    for pair in rings.windows(2) {
        wrap(b, pair[0], pair[1], skin, 10, false);
    }
    b.gem(v(0.0, 0.898, -0.029), w + 0.019, 0.111, 0.133, 0.31, hair);
    b.gem(
        v(0.0, 0.827, -0.086),
        w + 0.008,
        0.118,
        0.076,
        0.45,
        shift(hair, -2),
    );
    match style {
        0 | 2 | 5 if region == Region::Oasis => {
            let headscarf = if style == 2 {
                [231, 210, 169]
            } else {
                shift(cloth, 24)
            };
            b.gem(
                v(0.0, 0.91, -0.039),
                w + 0.024,
                0.116,
                0.142,
                0.31,
                headscarf,
            );
            for (side, length) in [(-1.0, 0.18), (1.0, 0.103)] {
                let start = v((w + 0.012) * side, 0.908, 0.048);
                b.beam(
                    start,
                    start + v(-side * 0.013, -length, -0.044),
                    0.046,
                    headscarf,
                );
            }
            b.frustum(
                v(0.0, 0.918, -0.029),
                w + 0.029,
                w + 0.005,
                0.030,
                11,
                shift(cloth, -30),
            );
        }
        3 | 6 if region == Region::Oasis => {
            b.frustum(
                v(0.0, 0.926, -0.032),
                w + 0.024,
                w + 0.014,
                0.063,
                12,
                shift(cloth, 18),
            );
            b.frustum(v(0.0, 0.988, -0.032), w + 0.014, w * 0.62, 0.039, 12, cloth);
            b.gem(v(0.0, 0.954, 0.092), 0.035, 0.039, 0.027, 0.20, GOLD);
        }
        0 | 3 | 7 if region == Region::Nordic => {
            let wool = shift(cloth, -8);
            b.gem(v(0.0, 0.91, -0.067), w + 0.043, 0.161, 0.144, 0.31, wool);
            for side in [-1.0, 1.0] {
                b.beam(
                    v(side * (w + 0.027), 0.885, 0.018),
                    v(side * (w - 0.029), 0.713, 0.016),
                    0.047,
                    wool,
                );
                b.beam(
                    v(side * 0.091, 0.969, 0.08),
                    v(side * 0.120, 0.873, 0.085),
                    0.025,
                    shift(wool, 41),
                );
            }
        }
        2 | 4 if region == Region::Nordic => {
            b.frustum(
                v(0.0, 0.905, -0.017),
                w + 0.019,
                w + 0.020,
                0.045,
                12,
                shift(cloth, 32),
            );
            b.frustum(v(0.0, 0.95, -0.017), w + 0.021, w * 0.58, 0.083, 12, cloth);
            b.frustum(
                v(0.0, 1.033, -0.017),
                w * 0.58,
                0.025,
                0.039,
                12,
                shift(cloth, 17),
            );
        }
        2 | 7 if region == Region::Japan => {
            b.frustum(v(0.0, 0.92, 0.0), 0.245, 0.227, 0.030, 14, shift(GOLD, -15));
            b.frustum(v(0.0, 0.95, 0.0), 0.227, 0.033, 0.15, 14, shift(CREAM, -9));
            b.beam(
                v(-0.121, 0.928, 0.07),
                v(-0.068, 0.719, 0.079),
                0.015,
                cloth,
            );
        }
        4 if region == Region::Japan => {
            b.gem(v(0.0, 0.989, -0.063), 0.072, 0.052, 0.068, 0.4, hair);
            b.beam(
                v(-0.085, 0.976, -0.073),
                v(0.111, 1.032, -0.073),
                0.022,
                GOLD,
            );
            b.gem(v(0.10, 1.023, -0.069), 0.037, 0.048, 0.026, 0.0, cloth);
        }
        1 | 6 => {
            b.gem(v(0.0, 0.944, -0.142), 0.079, 0.075, 0.070, 0.45, hair);
            b.gem(v(0.0, 0.929, -0.108), 0.078, 0.032, 0.069, 0.29, cloth);
        }
        2 if region == Region::Greece => {
            b.frustum(v(0.0, 0.90, 0.0), 0.207, 0.135, 0.062, 12, CREAM);
            b.frustum(v(0.0, 0.957, 0.0), 0.137, 0.091, 0.069, 10, shift(GOLD, 11));
        }
        3 | 5 if age != Age::Child => {
            b.gem(v(0.0, 0.716, 0.068), w * 0.77, 0.062, 0.058, 0.47, hair);
            b.gem(v(0.0, 0.751, 0.112), 0.072, 0.019, 0.019, 0.21, hair);
        }
        _ => {
            for n in 0..7 {
                let angle = 0.08 + n as f32 * 0.44;
                b.gem(
                    v(
                        angle.cos() * w * 0.91,
                        0.882 + (n % 2) as f32 * 0.017,
                        angle.sin() * 0.101,
                    ),
                    0.043,
                    0.050,
                    0.046,
                    angle,
                    shift(hair, (n % 4) as i16 * 4),
                );
            }
        }
    }
    if region == Region::Rome && matches!(style, 0 | 4) {
        for side in [-1.0, 1.0] {
            for i in 0..4 {
                let x = side * (w - i as f32 * 0.019);
                let z = 0.026 + i as f32 * 0.024;
                b.gem(
                    v(x, 0.895 + i as f32 * 0.005, z),
                    0.027,
                    0.015,
                    0.022,
                    i as f32,
                    [112, 142, 76],
                );
            }
        }
    }
    b.gem(
        v(0.0, 0.782, 0.126),
        0.031,
        0.039,
        0.036,
        0.38,
        shift(skin, 9),
    );
    b.gem(
        v(0.0, 0.736, 0.106),
        0.038,
        0.011,
        0.012,
        0.3,
        shift(skin, -37),
    );
    for side in [-1.0, 1.0] {
        let eye = v(side * w * 0.48, 0.807, 0.112);
        b.gem(eye, 0.030, 0.023, 0.017, 0.12, [70, 47, 38]);
        b.gem(
            eye + v(-0.006, 0.008, 0.014),
            0.008,
            0.008,
            0.004,
            0.2,
            MARBLE,
        );
        b.beam(
            eye + v(-0.022, 0.031, 0.0),
            eye + v(0.019, 0.033 + side * 0.006, -0.003),
            0.013,
            hair,
        );
        b.gem(
            v(side * w * 0.76, 0.768, 0.092),
            0.031,
            0.020,
            0.018,
            0.27,
            shift(skin, -8),
        );
        b.gem(v(side * w, 0.781, 0.0), 0.033, 0.043, 0.033, 0.11, skin);
        if matches!(style, 1 | 4 | 7) {
            b.gem(
                v(side * (w + 0.008), 0.747, 0.027),
                0.018,
                0.030,
                0.015,
                0.2,
                GOLD,
            );
        }
    }
}

fn pose(
    outfit: &Outfit,
    distance: f32,
    movement: f32,
    idle: f32,
    reaction: Option<(Verdict, f32, bool)>,
) -> [Transform; BONES] {
    let mut joints = outfit.rest.map(Transform::from_translation);
    let stride = match outfit.age {
        Age::Child => 0.46,
        Age::Adult => 0.56,
        Age::Senior => 0.49,
    };
    let cycle = distance / stride;
    let beat = cycle * std::f32::consts::TAU;
    let spring = if outfit.age == Age::Child {
        0.029
    } else {
        0.018
    };
    let bob = (spring * (beat * 2.0).cos().abs() - 0.007) * movement;
    joints[0].translation.y += bob + 0.004 * idle;
    let gesture = reaction.map_or(0.0, |(verdict, age, _)| {
        let strength = if verdict == Verdict::Pleased {
            1.0
        } else {
            -1.0
        };
        let progress = (age / 1.28).clamp(0.0, 1.0);
        (progress * std::f32::consts::PI).sin() * strength
    });
    let prayer = reaction.map_or(0.0, |(_, seconds, prays)| {
        let bow = (seconds / 1.76).clamp(0.0, 1.0) * std::f32::consts::PI;
        f32::from(prays) * bow.sin().max(0.0)
    });
    joints[0].rotation = Quat::from_rotation_z(beat.sin() * 0.037 * movement)
        * Quat::from_rotation_x(
            gesture * 0.11 + prayer * 0.46 + 0.10 * f32::from(outfit.age == Age::Senior),
        );
    for (side, offset) in [0.0, 0.5].into_iter().enumerate() {
        let phase = (cycle + offset).rem_euclid(1.0);
        let base = 1 + side * 4;
        let shoulder = outfit.rest[base];
        joints[base].translation =
            joints[0].rotation * (shoulder - outfit.rest[0]) + joints[0].translation;
        let swing = -0.33 * (phase * std::f32::consts::TAU).cos() * movement - 0.06;
        let hand = if prayer > 0.003 {
            -0.98 * prayer
        } else if side == 1 {
            -0.72 * gesture.max(0.0)
        } else {
            0.0
        };
        let shrug = (gesture.min(0.0) * 0.22 + prayer * 0.57) * (1.0 - 2.0 * side as f32);
        joints[base].rotation = Quat::from_rotation_z(shrug) * Quat::from_rotation_x(swing + hand);

        let hip = outfit.rest[base + 1] + v(0.0, bob, 0.0);
        let (z, y, pivot) = if phase < 0.60 {
            ((0.30 - phase) * stride, 0.0, 0.0)
        } else {
            let swing = (phase - 0.60) / 0.40;
            let lift = (swing * std::f32::consts::PI).sin();
            (
                -0.168 + 0.336 * (swing * swing * (3.0 - 2.0 * swing)),
                lift * 0.093,
                0.22 * lift,
            )
        };
        let desired = v(hip.x, ANKLE + y * movement, z * movement);
        let delta = (desired - hip).clamp_length_max(THIGH + SHIN - 0.001);
        let reach = delta.length().max(0.01);
        let axis = delta / reach;
        let along = (THIGH * THIGH - SHIN * SHIN + reach * reach) / (2.0 * reach);
        let knee = hip
            + axis * along
            + v(0.0, axis.z, -axis.y) * (THIGH * THIGH - along * along).max(0.0).sqrt();
        let foot = hip + delta;
        joints[base + 1] = Transform::from_translation(hip).with_rotation(Quat::from_rotation_arc(
            Vec3::NEG_Y,
            (knee - hip).normalize(),
        ));
        joints[base + 2] = Transform::from_translation(knee).with_rotation(
            Quat::from_rotation_arc(Vec3::NEG_Y, (foot - knee).normalize()),
        );
        joints[base + 3] = Transform::from_translation(foot)
            .with_rotation(Quat::from_rotation_x(pivot * movement));
    }
    joints
}

struct Walk {
    motion: Motion,
    reacting_for: Option<f32>,
    effort: f32,
}

fn visit_walk(visit: &Visit, index: usize, seconds: f32, layout: Layout) -> Option<Walk> {
    let age = seconds - visit.at;
    if !(-5.9..15.0).contains(&age) {
        return None;
    }
    let lane = [-0.70, -0.31, 0.12, 0.58][index % 4];
    let companion = visit.guided_to;
    let mut origin = match visit.origin {
        Origin::Home(lot) => motion::entrance(usize::from(lot), layout),
        Origin::Ferry => motion::stop(crate::errands::Stop::Pier, layout),
    } + Vec3::X * ((index % 3) as f32 - 1.0) * 0.16;
    if let Some(lot) = companion {
        let target = motion::entrance(usize::from(lot), layout);
        if age < 0.0 {
            return (age >= -1.85).then(|| Walk {
                motion: Motion::still(
                    target + v((index % 2) as f32 * 0.33 - 0.21, 0.0, 0.16),
                    0.23,
                    0.0,
                ),
                reacting_for: None,
                effort: 0.0,
            });
        }
        origin = motion::stop(crate::errands::Stop::Pier, layout);
    }
    let entry = v(origin.x, 0.071, lane);
    let approach = Path::direct(origin, entry);
    let pre = (approach.length() / 1.23).clamp(0.55, 5.7);
    if age < 0.0 {
        return (age >= -pre).then(|| Walk {
            motion: approach.sample(age + pre, pre, index as f32 * 0.127),
            reacting_for: None,
            effort: ((age + pre) * 5.0).clamp(0.0, 1.0),
        });
    }

    let declined_at = match visit.miss {
        Some(Miss::Price | Miss::Empty) => visit.declined_at,
        _ => None,
    };
    let goal = if let Some(lot) = visit.shop.or(companion).or(declined_at) {
        motion::entrance(usize::from(lot), layout)
    } else {
        let x = origin.x - origin.x.signum() * (1.0 + (index % 4) as f32 * 0.37);
        v(x, 0.072, lane + if lane >= 0.0 { 0.17 } else { -0.13 })
    };
    let towards = if companion.is_some() {
        Path::direct(goal, goal)
    } else {
        Path::road(entry, goal, lane)
    };
    let length = towards.length();
    let start = approach.length() + index as f32 * 0.127;
    let travel = if companion.is_some() {
        0.0
    } else {
        (length / 1.71).max(0.32)
    };
    let entering_yaw = approach.sample(pre, pre, 0.0).yaw;
    if age < travel {
        let mut step = towards.sample(age, travel, start);
        step.yaw = motion::turn(entering_yaw, step.yaw, age * 4.0);
        return Some(Walk {
            motion: step,
            reacting_for: None,
            effort: 1.0,
        });
    }
    let pause = 1.62;
    let exit = Path::road(goal, origin, lane);
    let wait = age - travel;
    let stop = towards.sample(travel, travel, start);
    let reacting_for = (wait < REACTION_TIME).then_some(wait);
    if wait < pause {
        let turn = (wait - 0.43).max(0.0) / (pause - 0.43);
        let yaw = motion::turn(stop.yaw, exit.sample(0.0, 1.0, 0.0).yaw, turn);
        return Some(Walk {
            motion: Motion::still(goal, yaw, stop.distance),
            reacting_for,
            effort: (1.0 - wait * 4.0).max(0.0),
        });
    }
    let going = wait - pause;
    let duration = (exit.length() / 1.85).max(0.55);
    (going < duration).then(|| Walk {
        motion: exit.sample(going, duration, stop.distance),
        reacting_for,
        effort: (going * 4.0).min((duration - going) * 5.0).clamp(0.0, 1.0),
    })
}

type Residents<'w, 's> = Query<
    'w,
    's,
    (
        &'static Resident,
        &'static mut Transform,
        &'static mut Visibility,
        &'static mut Mesh3d,
        &'static mut SkinnedMesh,
    ),
    Without<Joint>,
>;

#[allow(clippy::type_complexity)]
pub fn animate(
    game: Res<Game>,
    save: Res<Campaign>,
    cast: Res<Cast>,
    mut residents: Residents,
    mut joints: Query<&mut Transform, With<Joint>>,
    mut reactions: ResMut<Reactions>,
) {
    let town = game.visible_town(&save);
    let region = Region::of(town.chapter);
    let layout = CHAPTERS[town.chapter].layout;
    let mut active = Vec::with_capacity(RESIDENTS);
    reactions.0.clear();
    if let Phase::Open(day) = &town.phase {
        if let Some((route, _)) = day.route_pose().filter(|(route, (stage, phase))| {
            use crate::errands::RouteStage::*;
            route.job == crate::errands::Job::Welcome
                && (matches!(stage, Carry | Deliver) || *stage == Collect && *phase > 0.17)
        }) {
            let path = motion::delivery_path(route, layout);
            for n in 0..2 {
                let offset = 0.40 + n as f32 * 0.30;
                let motion = if let Some((_, nili)) = day.manual_route() {
                    let yaw = nili.yaw;
                    let rotation = Quat::from_rotation_y(yaw);
                    let at = motion::controlled_position(nili)
                        + rotation * v(-0.32 + n as f32 * 0.53, 0.0, -0.59 - n as f32 * 0.29);
                    let at = motion::ground(at, layout);
                    let follower = crate::errands::NiliControl {
                        position: at.to_array(),
                        ..nili
                    };
                    Motion {
                        at: motion::controlled_position(follower),
                        yaw,
                        distance: game.walk_distance + offset,
                        moving: game.walking,
                    }
                } else {
                    let mut motion = path.sample(
                        day.seconds - route.depart_at - offset,
                        route.arrival - route.depart_at,
                        0.0,
                    );
                    motion.at += v(-0.24 + n as f32 * 0.37, 0.0, 0.09);
                    motion
                };
                let effort = f32::from(motion.moving);
                active.push((
                    day.visits.len() + n,
                    Walk {
                        motion,
                        reacting_for: None,
                        effort,
                    },
                ));
            }
        }
        for (i, visit) in day.visits.iter().enumerate() {
            if active.len() == RESIDENTS {
                break;
            }
            if let Some(walk) = visit_walk(visit, i, day.seconds, layout) {
                if let (Some(seconds), Some(verdict)) = (walk.reacting_for, visit.verdict()) {
                    let height = 1.09 * person_scale(visit.age(i), i);
                    reactions.0.push(Reaction {
                        guest: i,
                        head: walk.motion.at + Vec3::Y * height,
                        verdict,
                        seconds,
                    });
                }
                active.push((i, walk));
            }
        }
    } else {
        let playing = game.view == View::Street;
        let town_width = (layout.width() as f32 - 1.0) * super::LOT_STEP * 0.5;
        for i in 0..if playing { 5 } else { 10 } {
            let phase = game.clock * 0.22 + i as f32 * 1.71;
            let speed = phase.cos();
            let x = phase.sin() * town_width;
            let at = v(x, 0.074, [-0.66, -0.28, 0.19, 0.61][i % 4]);
            let walk = Motion {
                at,
                yaw: speed.atan2((phase * 0.9).sin() * 0.42),
                distance: x + town_width,
                moving: true,
            };
            active.push((
                i,
                Walk {
                    motion: walk,
                    reacting_for: None,
                    effort: speed.abs().min(1.0),
                },
            ));
        }
    }
    for (actor, mut transform, mut visible, mut mesh, mut skin) in &mut residents {
        let Some((i, walk)) = active.get(actor.0) else {
            if *visible != Visibility::Hidden {
                *visible = Visibility::Hidden;
            }
            continue;
        };
        *visible = Visibility::Inherited;
        let age = town.guest_history().get(*i).map_or_else(
            || Age::guest(*i).on_island(town.chapter),
            |visit| visit.age(*i),
        );
        let style = age as usize * 8 + (i * 7 + town.chapter / 3) % 8;
        let look = &cast.clothes[region as usize][style];
        if mesh.0 != look.mesh {
            mesh.0 = look.mesh.clone();
            skin.inverse_bindposes = look.bind.clone();
        }
        transform.translation = walk.motion.at;
        transform.rotation = Quat::from_rotation_y(walk.motion.yaw);
        transform.scale = Vec3::splat(person_scale(age, *i));
        let reaction = walk.reacting_for.and_then(|age| {
            let visit = town.guest_history().get(*i)?;
            visit.verdict().map(|verdict| {
                (
                    verdict,
                    age,
                    verdict == Verdict::Pleased && visit.wants == Service::Prayer,
                )
            })
        });
        let pose = pose(
            look,
            walk.motion.distance,
            walk.effort,
            (game.clock * 2.3 + *i as f32).sin(),
            reaction,
        );
        for (&joint, placed) in skin.joints.iter().zip(pose) {
            if let Ok(mut transform) = joints.get_mut(joint) {
                *transform = placed;
            }
        }
    }
}

fn person_scale(age: Age, index: usize) -> f32 {
    let base = match age {
        Age::Child => 0.73,
        Age::Adult => 1.01,
        Age::Senior => 0.94,
    };
    base + (index % 4) as f32 * 0.034
}

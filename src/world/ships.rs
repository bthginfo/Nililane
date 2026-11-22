use bevy::prelude::*;

use crate::campaign::Region;

use super::{
    buildings::{BLUE, CREAM, GOLD, MARBLE, OAK, TEAL, amphora},
    mesh::{Batch, Tint, shift, v},
};

const OCHRE: Tint = [170, 111, 65];
const PITCH: Tint = [74, 48, 39];
const WINE: Tint = [169, 69, 52];
const LENGTHS: [f32; 3] = [2.25, 1.19, 1.52];
const WIDTHS: [f32; 3] = [0.63, 0.44, 0.54];
pub(super) const FERRY_SPAR: f32 = 2.11;

pub(super) struct FerryPose {
    pub hull: Transform,
    pub wake: Transform,
    pub gangway: Option<Transform>,
}

pub(super) fn berth(quay: f32) -> Vec3 {
    v(quay + 0.86, -0.346, super::landscape::LANDING_EDGE + 1.54)
}

pub(super) fn ferry(quay: f32, day: f32, seconds: f32, swell: f32) -> FerryPose {
    let travel = (day - 0.42).max(0.0) * (15.0 + swell * 3.3);
    let turning = travel / (travel + 1.0);
    let wave = seconds * (1.1 + swell);
    let yaw = -0.038 - 0.11 * turning + 0.017 * wave.cos();
    let angle = Quat::from_rotation_y(yaw);
    let dip = wave.sin() * (0.025 + swell * 0.049 + 0.055 * turning);
    let hull = Transform::from_translation(
        berth(quay) + v(travel + 0.045 * (wave * 0.67).sin(), dip, travel * 0.25),
    )
    .with_rotation(
        angle
            * Quat::from_rotation_z(wave.cos() * (0.020 + swell * 0.064 + 0.038 * turning))
            * Quat::from_rotation_x(wave.sin() * swell * (0.061 + 0.034 * turning)),
    );
    let gangway = (travel < 0.11).then(|| {
        let start = v(quay + 0.29, 0.154, super::landscape::LANDING_EDGE - 0.11);
        let end = hull.transform_point(v(-0.59, 0.427, -0.49));
        let span = end - start;
        Transform::from_translation((start + end) * 0.5)
            .with_rotation(Quat::from_rotation_arc(Vec3::Z, span.normalize()))
            .with_scale(v(1.0, 1.0, span.length()))
    });
    let wake = Transform::from_xyz(
        hull.translation.x,
        super::landscape::WAKE_LEVEL,
        hull.translation.z,
    )
    .with_rotation(angle)
    .with_scale(v(
        0.86 + wave.sin() * 0.10 + swell * 0.32,
        1.0,
        0.79 + swell * 0.44,
    ));
    FerryPose {
        hull,
        wake,
        gangway,
    }
}

pub(super) fn gangway() -> Mesh {
    let mut b = Batch::default();
    for i in 0..9 {
        b.slab(
            v(0.0, 0.0, -0.448 + i as f32 * 0.112),
            v(0.43, 0.045, 0.106),
            shift(OAK, 14 + (i % 3) as i16 * 6),
        );
    }
    for side in [-1.0, 1.0] {
        b.slab(v(side * 0.19, -0.038, 0.0), v(0.062, 0.051, 1.0), PITCH);
    }
    b.mesh()
}

fn paint(region: Region, class: usize) -> Tint {
    match region {
        Region::Greece => [TEAL, BLUE, GOLD][class],
        Region::Rome => [WINE, [63, 112, 99], [172, 127, 68]][class],
        Region::Japan => [[76, 103, 130], [180, 88, 66], [110, 143, 99]][class],
        Region::Nordic => [[155, 73, 53], [69, 112, 131], [156, 118, 67]][class],
    }
}

pub fn hull(region: Region, class: usize) -> Mesh {
    let mut b = Batch::default();
    let large = class == 0;
    let roman = region == Region::Rome;
    let japan = region == Region::Japan;
    let nordic = region == Region::Nordic;
    let length = LENGTHS[class];
    let width = WIDTHS[class];
    let tint = paint(region, class);
    let stations = [
        -1.0, -0.91, -0.73, -0.49, -0.23, 0.03, 0.30, 0.55, 0.76, 0.91, 1.0,
    ];
    let planks = match region {
        Region::Japan => [126, 85, 56],
        Region::Nordic => [105, 76, 55],
        _ => OCHRE,
    };
    let sections = [
        (-0.19, 0.38, PITCH),
        (-0.04, 0.73, shift(OAK, 12)),
        (0.10, 0.90, planks),
        (0.23, 0.98, shift(planks, 10)),
        (0.31, 1.0, tint),
        (0.395, 0.98, shift(OAK, 20)),
    ];
    let point = |station: usize, strake: usize, side: f32| {
        let t: f32 = stations[station];
        let fullness = (1.0f32 - t.abs().powf(if roman { 1.95 } else { 1.62 })).powf(0.60);
        let (level, breadth, _) = sections[strake];
        let lift = if nordic {
            0.40
        } else if roman || japan {
            0.29
        } else {
            0.22
        };
        let ends = t.abs().powi(5) * lift;
        v(
            t * length * (0.88 + 0.025 * strake as f32),
            level + ends,
            side * width * fullness * breadth,
        )
    };
    for station in 0..stations.len() - 1 {
        for (band, section) in sections.iter().enumerate().take(sections.len() - 1) {
            for side in [-1.0, 1.0] {
                let mut planks = [
                    point(station, band, side),
                    point(station + 1, band, side),
                    point(station + 1, band + 1, side),
                    point(station, band + 1, side),
                ];
                if side < 0.0 {
                    planks.reverse();
                }
                let color = shift(section.2, (station * 19 % 22) as i16 - 9);
                b.face(&planks, color);
            }
        }
        let edge = sections.len() - 1;
        let at = [
            point(station, edge, -1.0),
            point(station, edge, 1.0),
            point(station + 1, edge, 1.0),
            point(station + 1, edge, -1.0),
        ];
        b.face(
            &at.map(|p| p - Vec3::Y * 0.041),
            shift(OCHRE, (station * 7 % 19) as i16),
        );
        for side in [-1.0, 1.0] {
            b.beam(
                point(station, edge, side),
                point(station + 1, edge, side),
                0.049,
                PITCH,
            );
            let a = point(station, edge - 1, side);
            let c = point(station + 1, edge - 1, side);
            b.beam(
                a + Vec3::Z * side * 0.005,
                c + Vec3::Z * side * 0.005,
                0.021,
                GOLD,
            );
            if nordic && station > 0 && station < stations.len() - 2 {
                for band in 1..4 {
                    b.beam(
                        point(station, band, side),
                        point(station + 1, band, side),
                        0.028,
                        PITCH,
                    );
                }
            }
        }
    }
    b.beam(
        v(-length * 0.85, -0.18, 0.0),
        v(length * 0.85, -0.18, 0.0),
        0.11,
        PITCH,
    );
    b.frustum(
        v(-0.20, 0.36, 0.0),
        0.078,
        0.058,
        if large { 2.08 } else { 1.60 },
        10,
        OAK,
    );

    let mast_top = if large { 2.30 } else { 1.89 };
    for side in [-1.0, 1.0] {
        b.beam(
            v(-0.2, mast_top, 0.0),
            v(-0.96, 0.46, side * width * 0.80),
            0.022,
            CREAM,
        );
        b.beam(
            v(-0.2, mast_top - 0.07, 0.0),
            v(0.76, 0.43, side * width * 0.78),
            0.017,
            CREAM,
        );
        for i in 0..if large { 4 } else { 2 } {
            let x = -0.77 + i as f32 * 0.41;
            let at = v(x, 0.40, side * width * 0.90);
            let tip = v(
                x - 0.30,
                -0.08 + (i % 2) as f32 * 0.042,
                side * (width + 0.34),
            );
            b.beam(at, tip, 0.036, shift(OAK, 37));
            b.block(
                tip,
                v(0.11, 0.046, 0.29),
                Quat::from_rotation_y(-side * 0.63),
                OCHRE,
            );
        }
    }
    let stern = -length * 0.93;
    let prow = length * 0.95;
    if nordic {
        for direction in [-1.0, 1.0] {
            let end = direction * length * 0.91;
            b.beam(
                v(end - direction * 0.25, 0.64, 0.0),
                v(end + direction * 0.08, 1.00, 0.0),
                0.13,
                PITCH,
            );
            b.beam(
                v(end + direction * 0.08, 1.00, 0.0),
                v(end + direction * 0.04, 1.27, 0.0),
                0.094,
                planks,
            );
            b.gem(
                v(end + direction * 0.035, 1.26, 0.0),
                0.127,
                0.086,
                0.069,
                0.22,
                tint,
            );
            b.gem(
                v(end + direction * 0.104, 1.247, 0.0),
                0.046,
                0.046,
                0.061,
                0.0,
                GOLD,
            );
        }
        for station in [2, 4, 6, 8] {
            for side in [-1.0, 1.0] {
                let at = point(station, 5, side) + v(0.0, 0.034, -side * 0.022);
                let color = if station % 4 == 0 {
                    tint
                } else {
                    shift(GOLD, -21)
                };
                for i in 0..10 {
                    let rim = |i| {
                        let angle = i as f32 * std::f32::consts::TAU / 10.0;
                        at + v(angle.cos() * 0.161, angle.sin() * 0.161, 0.0)
                    };
                    let mut face = [at + v(0.0, 0.0, side * 0.029), rim(i), rim(i + 1)];
                    if side < 0.0 {
                        face.reverse();
                    }
                    b.face(&face, shift(color, (i * 13 % 28) as i16 - 9));
                }
                b.gem(
                    at + v(0.0, 0.0, side * 0.032),
                    0.046,
                    0.046,
                    0.021,
                    0.2,
                    PITCH,
                );
            }
        }
    } else if japan {
        b.beam(
            v(prow - 0.23, 0.53, 0.0),
            v(prow + 0.12, 0.88, 0.0),
            0.127,
            PITCH,
        );
        b.beam(
            v(prow - 0.28, 0.55, 0.0),
            v(prow + 0.115, 0.88, 0.0),
            0.052,
            OCHRE,
        );
        b.beam(
            v(stern + 0.05, 0.22, 0.0),
            v(stern - 0.08, 0.85, 0.0),
            0.073,
            PITCH,
        );
        let cabin = -length * 0.52;
        let cabin_width = if large { 0.39 } else { 0.27 };
        for side in [-1.0, 1.0] {
            for offset in [-0.26, 0.26] {
                b.slab(
                    v(cabin + offset, 0.63, side * cabin_width),
                    v(0.064, 0.46, 0.067),
                    OAK,
                );
            }
            for n in 0..6 {
                let x = cabin - 0.27 + n as f32 * 0.106;
                b.slab(
                    v(x, 0.60, side * (cabin_width - 0.017)),
                    v(0.035, 0.30, 0.037),
                    shift(OCHRE, n as i16 * 4),
                );
            }
        }
        super::japan::roof(
            &mut b,
            v(cabin, 0.86, 0.0),
            0.39,
            cabin_width + 0.075,
            0.18,
            tint,
        );
        b.gem(
            v(cabin - 0.31, 0.70, 0.07),
            0.076,
            0.099,
            0.076,
            0.3,
            [234, 173, 104],
        );
    } else if roman {
        b.beam(
            v(stern + 0.32, 0.48, 0.0),
            v(stern + 0.015, 0.81, 0.0),
            0.105,
            PITCH,
        );
        b.beam(
            v(stern + 0.015, 0.81, 0.0),
            v(stern + 0.095, 1.16, 0.0),
            0.076,
            OCHRE,
        );
        b.gem(v(stern + 0.09, 1.18, 0.0), 0.077, 0.091, 0.071, 0.25, GOLD);
        if large {
            b.slab(
                v(-length * 0.52, 0.64, 0.0),
                v(0.63, 0.51, 0.78),
                TRAVELLER_CABIN,
            );
            b.slab(v(-length * 0.52, 0.92, 0.0), v(0.75, 0.11, 0.89), WINE);
            b.slab(v(-length * 0.52, 0.68, 0.397), v(0.28, 0.21, 0.05), PITCH);
        }
    } else {
        b.beam(
            v(prow - 0.31, 0.43, 0.0),
            v(prow + 0.12, 0.61, 0.0),
            0.111,
            PITCH,
        );
        b.beam(
            v(prow + 0.12, 0.61, 0.0),
            v(prow + 0.053, 0.94, 0.0),
            0.081,
            OCHRE,
        );
        b.beam(
            v(prow + 0.053, 0.94, 0.0),
            v(prow - 0.10, 1.07, 0.0),
            0.059,
            GOLD,
        );
        for side in [-1.0, 1.0] {
            b.gem(
                v(prow - 0.25, 0.49, side * 0.22),
                0.12,
                0.054,
                0.035,
                0.0,
                MARBLE,
            );
            b.gem(
                v(prow - 0.23, 0.49, side * 0.242),
                0.041,
                0.038,
                0.024,
                0.0,
                PITCH,
            );
        }
    }
    for (x, z) in [(-0.54, 0.22), (0.38, 0.19), (0.72, -0.15)] {
        b.slab(v(x, 0.43, z), v(0.37, 0.17, 0.29), OAK);
        if large {
            amphora(&mut b, v(x, 0.50, z), 0.65, if roman { WINE } else { tint });
        }
    }
    b.beam(
        v(stern + 0.20, 0.50, 0.20),
        v(stern - 0.26, -0.04, 0.62),
        0.060,
        OAK,
    );
    b.mesh()
}

const TRAVELLER_CABIN: Tint = [177, 130, 83];

pub fn sail(region: Region, class: usize) -> Mesh {
    let mut b = Batch::default();
    let large = class == 0;
    let width = if large { 1.84 } else { 1.34 };
    let y = if large { FERRY_SPAR } else { 1.73 };
    let height = if large { 1.10 } else { 0.87 };
    let tint = paint(region, class);
    let japan = region == Region::Japan;
    let nordic = region == Region::Nordic;
    let pinned = |u: f32, v_: f32| {
        let billow = (std::f32::consts::PI * u).sin() * (std::f32::consts::PI * v_ * 0.87).sin();
        let taper = if japan {
            -0.085
        } else if nordic {
            0.022
        } else {
            0.17
        };
        let x = (u - 0.5) * width * (1.0 - taper * v_) - 0.21 + (v_ * v_) * 0.18;
        v(
            x,
            y - v_ * height - (1.0 - billow) * 0.042,
            0.031 + billow * if large { 0.31 } else { 0.21 },
        )
    };
    for row in 0..7 {
        for col in 0..12 {
            let u = col as f32 / 12.0;
            let vs = row as f32 / 7.0;
            let striped = if nordic {
                (col / 2) % 2 == 0 || row == 6
            } else if japan {
                matches!(col, 1 | 10) && row > 0
            } else {
                row == 5 || (region == Region::Rome && col == 5)
            };
            let color = if striped {
                tint
            } else {
                let linen = if japan {
                    [236, 220, 182]
                } else if nordic {
                    [225, 209, 171]
                } else {
                    MARBLE
                };
                shift(linen, (col * 8 % 17) as i16 - 9)
            };
            let corners = [
                pinned(u, vs),
                pinned(u, vs + 1.0 / 7.0),
                pinned(u + 1.0 / 12.0, vs + 1.0 / 7.0),
                pinned(u + 1.0 / 12.0, vs),
            ];
            b.face(&corners, color);
            b.face(
                &corners
                    .map(|point| point - Vec3::Z * 0.015)
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>(),
                shift(color, -8),
            );
        }
    }
    b.beam(
        pinned(0.0, 0.0) + v(-0.088, 0.06, -0.06),
        pinned(1.0, 0.0) + v(0.088, 0.06, -0.06),
        0.061,
        OAK,
    );
    if japan || nordic {
        for n in 0..8 {
            let u = n as f32 / 7.0;
            let at = pinned(u, 0.0);
            b.beam(
                at - v(0.0, 0.045, 0.0),
                at + v(0.0, 0.084, 0.0),
                0.024,
                CREAM,
            );
        }
    }
    if !large {
        b.beam(pinned(0.0, 1.0), v(-0.79, 0.46, 0.07), 0.018, CREAM);
        b.beam(pinned(1.0, 1.0), v(1.18, 0.41, -0.07), 0.018, CREAM);
    }
    b.face(
        &[
            v(-0.2, y + 0.18, 0.003),
            v(-0.2, y + 0.34, 0.012),
            v(0.41, y + 0.23, 0.025),
        ],
        tint,
    );
    b.mesh()
}

pub fn wake(class: usize) -> Mesh {
    let mut b = Batch::default();
    let length = LENGTHS[class];
    let width = WIDTHS[class];
    for side in [-1.0, 1.0] {
        for i in 0..9 {
            let x = length * 0.82 - i as f32 * length * 0.25;
            let z = side * (width + 0.13 + i as f32 * 0.091);
            let len = 0.33 + (i % 4) as f32 * 0.18;
            b.face(
                &[
                    v(x - len, 0.0, z - 0.051),
                    v(x - 0.21, 0.0, z + 0.035),
                    v(x + len * 0.32, 0.0, z + 0.005),
                    v(x - 0.16, 0.0, z - 0.076),
                ],
                if i % 3 == 0 {
                    [196, 224, 212]
                } else {
                    [137, 189, 179]
                },
            );
        }
    }
    b.mesh()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::mesh::VertexAttributeValues;

    #[test]
    fn the_ferry_and_its_oars_clear_the_pier_in_every_region() {
        for region in Region::ALL {
            let mesh = hull(region, 0);
            let Some(VertexAttributeValues::Float32x3(points)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            else {
                panic!("the ferry has no vertices");
            };
            for swell in [0.18, 0.58, 0.78, 1.0] {
                for day in [0.0, 0.42, 0.43, 0.52, 1.0] {
                    for step in 0..48 {
                        let pose = ferry(5.0, day, step as f32 * 0.23, swell);
                        assert!(points.iter().all(|p| {
                            pose.hull.transform_point(Vec3::from_array(*p)).z
                                > super::super::landscape::LANDING_EDGE + 0.14
                        }));
                        assert_eq!(pose.gangway.is_some(), day <= 0.42);
                    }
                }
                let pose = ferry(5.0, 0.0, 1.0, swell);
                assert!(
                    pose.hull.transform_point(v(0.0, -0.19, 0.0)).y
                        < super::super::landscape::WATER_LEVEL
                );
            }
        }
    }
}

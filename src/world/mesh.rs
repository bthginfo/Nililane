use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub type Tint = [u8; 3];

#[derive(Default)]
pub struct Batch {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl Batch {
    pub fn face(&mut self, vertices: &[Vec3], tint: Tint) {
        let [r, g, b] = tint;
        let color = Color::srgb_u8(r, g, b).to_linear().to_f32_array();
        let offset = self.positions.len() as u32;
        let norm = (vertices[1] - vertices[0])
            .cross(vertices[2] - vertices[0])
            .normalize_or_zero();
        for vertex in vertices {
            self.positions.push(vertex.to_array());
            self.normals.push(norm.to_array());
            self.colors.push(color);
        }
        for i in 1..vertices.len() as u32 - 1 {
            self.indices.extend([offset, offset + i, offset + i + 1]);
        }
    }

    pub fn mesh(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
        .with_inserted_indices(Indices::U32(self.indices))
    }

    pub fn slab(&mut self, origin: Vec3, size: Vec3, tint: Tint) {
        self.block(origin, size, Quat::IDENTITY, tint);
    }

    pub fn block(&mut self, origin: Vec3, size: Vec3, tilt: Quat, tint: Tint) {
        let [x, y, z] = (size * 0.5).to_array();
        let at = |x, y, z| origin + tilt * Vec3::new(x, y, z);
        let corners = [
            at(-x, -y, -z),
            at(x, -y, -z),
            at(x, y, -z),
            at(-x, y, -z),
            at(-x, -y, z),
            at(x, -y, z),
            at(x, y, z),
            at(-x, y, z),
        ];
        for face in [
            [1, 0, 3, 2],
            [4, 5, 6, 7],
            [0, 4, 7, 3],
            [5, 1, 2, 6],
            [7, 6, 2, 3],
            [0, 1, 5, 4],
        ] {
            self.face(&face.map(|i| corners[i]), tint);
        }
    }

    pub fn beam(&mut self, a: Vec3, b: Vec3, width: f32, tint: Tint) {
        let d = b - a;
        self.block(
            (a + b) * 0.5,
            Vec3::new(width, d.length(), width),
            Quat::from_rotation_arc(Vec3::Y, d.normalize_or_zero()),
            tint,
        );
    }

    pub fn frustum(&mut self, p: Vec3, base: f32, ring: f32, h: f32, sides: usize, tint: Tint) {
        let point = |i: usize, radius, y| {
            let a = (i as f32 / sides as f32) * std::f32::consts::TAU;
            p + Vec3::new(a.cos() * radius, y, a.sin() * radius)
        };
        for i in 0..sides {
            self.face(
                &[
                    point(i, base, 0.0),
                    point(i, ring, h),
                    point(i + 1, ring, h),
                    point(i + 1, base, 0.0),
                ],
                tint,
            );
        }
        if ring > 0.001 {
            let top: Vec<_> = (0..sides).rev().map(|i| point(i, ring, h)).collect();
            self.face(&top, tint);
        }
    }

    pub fn gem(&mut self, p: Vec3, width: f32, height: f32, depth: f32, spin: f32, tint: Tint) {
        let point = |n: usize, r: f32, y: f32, off: f32| {
            let a = (n as f32 / 7.0 + off) * std::f32::consts::TAU + spin;
            p + Vec3::new(a.cos() * width * r, y * height, a.sin() * depth * r)
        };
        let top = p + Vec3::Y * height;
        let bottom = p - Vec3::Y * height;
        for i in 0..7 {
            let a = point(i, 0.91, -0.38, 0.0);
            let b = point(i + 1, 0.91, -0.38, 0.0);
            let c = point(i, 0.85, 0.47, 1.0 / 14.0);
            let d = point(i + 1, 0.85, 0.47, 1.0 / 14.0);
            self.face(&[bottom, a, b], shift(tint, -9));
            self.face(&[a, c, b], tint);
            self.face(&[b, c, d], shift(tint, 6));
            self.face(&[c, top, d], shift(tint, 12));
        }
    }
}

pub fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn shift(tint: Tint, brightness: i16) -> Tint {
    tint.map(|c| (i16::from(c) + brightness).clamp(0, 255) as u8)
}

pub struct Grain(u32);

impl Grain {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }
    pub fn get(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 >> 8) as f32 / 16_777_216.0
    }
    pub fn span(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.get()
    }
}

use bevy::{
    asset::RenderAssetUsages,
    prelude::Mesh,
    render::mesh::{Indices, PrimitiveTopology},
};
use map_gen::{Chunk, CHUNK_BOUNDARY};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub voxel: [usize; 3],
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct QuadGroups {
    pub groups: [Vec<Quad>; 6],
    pub offset: (usize, usize, usize),
}

impl QuadGroups {
    const LEFT: usize = 0;
    const RIGHT: usize = 1;
    // const BOTTOM: usize = 2;
    const TOP: usize = 3;
    const FRONT: usize = 4;
    const BACK: usize = 5;

    pub fn with_offset(self, offset: (usize, usize, usize)) -> Self {
        let mut new = self;
        new.offset = offset;
        new
    }
}

impl From<QuadGroups> for Mesh {
    fn from(value: QuadGroups) -> Self {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        // Top
        value.groups[QuadGroups::TOP].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32 + 1.0, z as f32],
                [x as f32 + width, y as f32 + 1.0, z as f32],
                [x as f32 + width, y as f32 + 1.0, z as f32 + height],
                [x as f32, y as f32 + 1.0, z as f32 + height],
            ]);
            indices.extend_from_slice(&[start, start + 2, start + 1, start, start + 3, start + 2]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 1.0, 0.0]; 4]);
        });

        // LEFT
        value.groups[QuadGroups::LEFT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32],
                [x as f32, y as f32 + height, z as f32],
                [x as f32, y as f32 + height, z as f32 + width],
                [x as f32, y as f32, z as f32 + width],
            ]);
            indices.extend_from_slice(&[start, start + 2, start + 1, start, start + 3, start + 2]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[-1.0, 0.0, 0.0]; 4]);
        });

        // RIGHT
        value.groups[QuadGroups::RIGHT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32 + 1.0, y as f32, z as f32],
                [x as f32 + 1.0, y as f32, z as f32 + width],
                [x as f32 + 1.0, y as f32 + height, z as f32 + width],
                [x as f32 + 1.0, y as f32 + height, z as f32],
            ]);
            indices.extend_from_slice(&[start, start + 2, start + 1, start, start + 3, start + 2]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[1.0, 0.0, 0.0]; 4]);
        });

        // FRONT
        value.groups[QuadGroups::FRONT].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32],
                [x as f32 + width, y as f32, z as f32],
                [x as f32 + width, y as f32 + height, z as f32],
                [x as f32, y as f32 + height, z as f32],
            ]);
            indices.extend_from_slice(&[start, start + 2, start + 1, start, start + 3, start + 2]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 0.0, -1.0]; 4]);
        });

        // BACK
        value.groups[QuadGroups::BACK].iter().for_each(|quad| {
            let [x, y, z] = quad.voxel;
            let width = quad.width as f32;
            let height = quad.height as f32;

            let start = vertices.len() as u32;
            vertices.extend_from_slice(&vec![
                [x as f32, y as f32, z as f32 + 1.0],
                [x as f32, y as f32 + height, z as f32 + 1.0],
                [x as f32 + width, y as f32 + height, z as f32 + 1.0],
                [x as f32 + width, y as f32, z as f32 + 1.0],
            ]);
            indices.extend_from_slice(&[start, start + 2, start + 1, start, start + 3, start + 2]);
            uvs.extend_from_slice(&vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
            normals.extend_from_slice(&vec![[0.0, 0.0, 1.0]; 4]);
        });

        for pos in vertices.iter_mut() {
            pos[0] += (value.offset.0) as f32;
            pos[1] += (value.offset.1) as f32;
            pos[2] += (value.offset.2) as f32;
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh
    }
}

pub fn generate_quads(chunk: &Chunk) -> QuadGroups {
    let mut buffer = QuadGroups::default();

    for x in 1..CHUNK_BOUNDARY - 1 {
        for y in 1..CHUNK_BOUNDARY - 1 {
            for z in 1..CHUNK_BOUNDARY - 1 {
                let voxel = chunk.get(x, y, z);
                if *voxel == map_gen::BlockType::Air {
                    continue;
                }

                let neighbors = [
                    chunk.get(x - 1, y, z),
                    chunk.get(x + 1, y, z),
                    chunk.get(x, y - 1, z),
                    chunk.get(x, y + 1, z),
                    chunk.get(x, y, z - 1),
                    chunk.get(x, y, z + 1),
                ];

                for (i, neighbor) in neighbors.iter().enumerate() {
                    if *neighbor != voxel {
                        buffer.groups[i].push(Quad {
                            voxel: [x, y, z],
                            width: 1,
                            height: 1,
                        });
                    }
                }
            }
        }
    }

    buffer
}

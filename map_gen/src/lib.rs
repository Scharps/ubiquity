use std::collections::HashMap;

pub mod biome;

pub struct ChunkMap {
    chunks: HashMap<(isize, isize, isize), Chunk>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    #[default]
    Air,
    Stone,
}

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct Chunk {
    voxels: Vec<BlockType>,
    offset: (isize, isize, isize),
}

impl Chunk {
    pub fn new(offset: (isize, isize, isize)) -> Self {
        Chunk {
            voxels: vec![BlockType::Air; Self::size()],
            offset,
        }
    }

    pub fn length() -> usize {
        CHUNK_SIZE
    }

    pub fn size() -> usize {
        CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn index(x: usize, y: usize, z: usize) -> usize {
        x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_SIZE)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &BlockType {
        &self.voxels[Self::index(x, y, z)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut BlockType {
        &mut self.voxels[Self::index(x, y, z)]
    }

    pub fn offset(&self) -> (isize, isize, isize) {
        self.offset
    }
}

pub trait ChunkTransformer {
    fn transform(&self, chunk: &mut Chunk);
}

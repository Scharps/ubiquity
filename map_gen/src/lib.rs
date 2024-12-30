use std::collections::HashMap;

pub mod biome;

pub struct ChunkMap {
    chunks: HashMap<(usize, usize, usize), Chunk>,
}

impl ChunkMap {
    pub fn new() -> Self {
        ChunkMap {
            chunks: HashMap::new(),
        }
    }

    pub fn chunks(&self) -> Vec<&Chunk> {
        self.chunks.values().collect::<Vec<_>>()
    }

    pub fn get_chunk(&self, offset: (usize, usize, usize)) -> Option<&Chunk> {
        self.chunks.get(&offset)
    }

    pub fn get_chunk_mut(&mut self, offset: (usize, usize, usize)) -> Option<&mut Chunk> {
        self.chunks.get_mut(&offset)
    }

    pub fn insert_chunk(&mut self, offset: (usize, usize, usize), chunk: Chunk) {
        self.chunks.insert(offset, chunk);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    #[default]
    Air,
    Stone,
}

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_BOUNDARY: usize = CHUNK_SIZE + 2;

#[derive(Debug, Clone)]
pub struct Chunk {
    voxels: Vec<BlockType>,
    offset: (usize, usize, usize),
}

impl Chunk {
    pub fn new(offset: (usize, usize, usize)) -> Self {
        Chunk {
            voxels: vec![BlockType::Air; CHUNK_BOUNDARY * CHUNK_BOUNDARY * CHUNK_BOUNDARY],
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
        x + (y * CHUNK_BOUNDARY) + (z * CHUNK_BOUNDARY * CHUNK_BOUNDARY)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &BlockType {
        &self.voxels[Self::index(x, y, z)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut BlockType {
        &mut self.voxels[Self::index(x, y, z)]
    }

    pub fn offset(&self) -> (usize, usize, usize) {
        self.offset
    }
}

pub trait ChunkTransformer {
    fn transform(&self, chunk: &mut Chunk);
}

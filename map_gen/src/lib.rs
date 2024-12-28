pub mod biome;

pub struct Map<Chunk: Chunky<BlockType>> {
    pub chunks: Vec<Chunk>,
}

pub trait Chunky<TBlock> {
    const X: usize;
    const Y: usize;
    const Z: usize;

    fn size() -> usize {
        Self::X * Self::Y * Self::Z
    }

    fn linearize(x: usize, y: usize, z: usize) -> usize {
        x + (y * Self::X) + (z * Self::X * Self::Y)
    }

    fn delinearize(mut index: usize) -> (usize, usize, usize) {
        let z = index / (Self::X * Self::Y);
        index -= z * (Self::X * Self::Y);
    
        let y = index / Self::X;
        index -= y * Self::X;
    
        let x = index;
    
        (x, y, z)
      }

      fn get(&self, x: usize, y: usize, z: usize) -> TBlock;

      fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut TBlock;

      fn offset(&self) -> (usize, usize, usize);
}

pub trait ChunkPopulate<TBlock> {
    fn populate<T: Chunky<TBlock>>(&self, chunk: &mut T);
}

pub struct TestChunk {
    voxels: Vec<BlockType>,
    pub offset: (usize, usize, usize),
}

impl TestChunk {
    pub fn new() -> Self {
        Self { voxels: vec![BlockType::Air; 16*16*16], offset: (0, 0, 0) }
    }
    
    pub fn voxels(&self) -> &[BlockType] {
        &self.voxels
    }
}

impl Chunky<BlockType> for TestChunk {
    const X: usize = 16;

    const Y: usize = 16;

    const Z: usize = 16;

    fn get(&self, x: usize, y: usize, z: usize) -> BlockType {
        self.voxels[Self::linearize(x, y, z)]
    }

    fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut BlockType {
        &mut self.voxels[Self::linearize(x, y, z)]
    }

    fn offset(&self) -> (usize, usize, usize) {
        self.offset
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    #[default]
    Air,
    Stone,
}

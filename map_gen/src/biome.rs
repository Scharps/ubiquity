use crate::{BlockType, Chunk, ChunkTransformer};
use libnoise::{Fbm, Generator, Perlin, Source};

pub struct FbmDescriptor {
    pub octaves: u32,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

pub struct LandGenerator {
    pub seed: u64,
    pub noise: Fbm<2, Perlin<2>>,
}

impl LandGenerator {
    pub fn new(seed: u64, fbm: FbmDescriptor) -> Self {
        let noise = Source::<2>::perlin(seed).fbm(
            fbm.octaves,
            fbm.frequency,
            fbm.lacunarity,
            fbm.persistence,
        );
        LandGenerator { seed, noise }
    }
}

impl ChunkTransformer for LandGenerator {
    fn transform(&self, chunk: &mut crate::Chunk) {
        let (offset_x, offset_z, _) = chunk.offset();

        let step_x = 1.0 / Chunk::length() as f64;
        let step_z = 1.0 / Chunk::length() as f64;

        for x in 0..Chunk::length() {
            for z in 0..Chunk::length() {
                let value = self.noise.sample([
                    (x as isize + offset_x) as f64 * step_x,
                    (z as isize + offset_z) as f64 * step_z,
                ]);
                let height = (value * Chunk::length() as f64) as usize;
                for y in 0..height {
                    *chunk.get_mut(x, y, z) = BlockType::Stone;
                }
            }
        }
    }
}

use crate::{BlockType, Chunk, ChunkTransformer, CHUNK_BOUNDARY};
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
        let (offset_x, _, offset_z) = chunk.offset();

        let step_x = 1.0 / Chunk::length() as f64;
        let step_z = 1.0 / Chunk::length() as f64;

        for x in 0..CHUNK_BOUNDARY {
            for z in 0..CHUNK_BOUNDARY {
                let value = self.noise.sample([
                    (x + offset_x) as f64 * step_x,
                    (z + offset_z) as f64 * step_z,
                ]);
                let height = (value * Chunk::length() as f64) as usize + 2;
                for y in 0..height {
                    *chunk.get_mut(x, y, z) = BlockType::Stone;
                }
            }
        }
    }
}

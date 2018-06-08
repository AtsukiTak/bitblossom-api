use std::{mem, collections::HashMap, hash::{BuildHasher, Hasher}, sync::{Arc, Mutex}};
use rand::{FromEntropy, RngCore, prng::XorShiftRng};

use images::size::Size;
use mosaic::MosaicArt;

pub struct ArtContainer<S> {
    container: HashMap<u64, Arc<Mutex<MosaicArt<S>>>, NothingU64HasherBuilder>,
    id_gen: IdGenerator,
}

impl<S: Size> ArtContainer<S> {
    pub fn new() -> ArtContainer<S> {
        ArtContainer {
            container: HashMap::with_hasher(NothingU64HasherBuilder),
            id_gen: IdGenerator::new(),
        }
    }

    pub fn add(&mut self, art: Arc<Mutex<MosaicArt<S>>>) -> u64 {
        let id = self.id_gen.next_id();
        self.container.insert(id, art);
        id
    }

    pub fn get(&self, id: u64) -> Option<&Arc<Mutex<MosaicArt<S>>>> {
        self.container.get(&id)
    }
}

struct IdGenerator {
    rng: XorShiftRng,
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator {
            rng: XorShiftRng::from_entropy(),
        }
    }

    fn next_id(&mut self) -> u64 {
        self.rng.next_u64()
    }
}

struct NothingU64HasherBuilder;

impl BuildHasher for NothingU64HasherBuilder {
    type Hasher = NothingU64Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        NothingU64Hasher { n: 0 }
    }
}

struct NothingU64Hasher {
    n: u64,
}

impl Hasher for NothingU64Hasher {
    fn finish(&self) -> u64 {
        self.n
    }

    fn write(&mut self, bytes: &[u8]) {
        self.n = match bytes.len() {
            0 => 0,
            1 => bytes[0] as u64,
            2 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 2]);
                mem::transmute::<[u8; 2], u16>(b) as u64
            },
            4 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 4]);
                mem::transmute::<[u8; 4], u32>(b) as u64
            },
            8 => unsafe {
                let b = *(bytes as *const _ as *const [u8; 8]);
                mem::transmute::<[u8; 8], u64>(b) as u64
            },
            i => panic!("Unexpected hasher input : {}", i),
        };
    }

    fn write_u64(&mut self, i: u64) {
        self.n = i;
    }
}

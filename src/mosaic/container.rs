use std::{mem, collections::HashMap, hash::{BuildHasher, Hasher}};
use rand::{FromEntropy, RngCore, prng::XorShiftRng};

use images::size::{MultipleOf, Size};
use mosaic::SharedMosaicArt;

#[derive(Debug, PartialEq, Eq)]
pub struct MosaicArtId(pub u64);

impl ::std::fmt::Display for MosaicArtId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

pub struct MosaicArtContainer<S, SS> {
    container: HashMap<u64, SharedMosaicArt<S, SS>, NothingU64HasherBuilder>,
    id_gen: IdGenerator,
}

impl<S, SS> MosaicArtContainer<S, SS>
where
    S: Size + MultipleOf<SS>,
    SS: Size,
{
    pub fn new() -> MosaicArtContainer<S, SS> {
        MosaicArtContainer {
            container: HashMap::with_hasher(NothingU64HasherBuilder),
            id_gen: IdGenerator::new(),
        }
    }

    pub fn add(&mut self, art: SharedMosaicArt<S, SS>) -> MosaicArtId {
        let id = self.id_gen.next_id();
        self.container.insert(id, art);
        MosaicArtId(id)
    }

    pub fn get(&self, id: MosaicArtId) -> Option<&SharedMosaicArt<S, SS>> {
        self.container.get(&id.0)
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

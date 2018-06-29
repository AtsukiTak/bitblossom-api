use std::{mem, collections::HashMap, hash::{BuildHasher, Hasher}};
use rand::{FromEntropy, RngCore, prng::XorShiftRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn into_raw(&self) -> u64 {
        self.0
    }

    pub fn from_raw(id: u64) -> Id {
        Id(id)
    }
}

pub struct IdGenerator {
    rng: XorShiftRng,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator {
            rng: XorShiftRng::from_entropy(),
        }
    }

    pub fn next_id(&mut self) -> Id {
        Id(self.rng.next_u64())
    }
}

/// HashMap whose key is u64 and value is V.
/// This HashMap uses NothingU64HasherBuilder as HasherBuilder
/// to reduce hash cost.
pub struct IdHashMap<V>(HashMap<Id, V, NothingU64HasherBuilder>);

impl<V> IdHashMap<V> {
    pub fn new() -> IdHashMap<V> {
        IdHashMap(HashMap::with_hasher(NothingU64HasherBuilder))
    }

    pub fn insert(&mut self, id: Id, val: V) -> Option<V> {
        self.0.insert(id, val)
    }

    pub fn get<'a, 'b>(&'a self, id: &'b Id) -> Option<&'a V> {
        self.0.get(id)
    }

    pub fn remove(&mut self, id: &Id) -> Option<V> {
        self.0.remove(id)
    }
}

pub struct NothingU64HasherBuilder;

impl BuildHasher for NothingU64HasherBuilder {
    type Hasher = NothingU64Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        NothingU64Hasher { n: 0 }
    }
}

pub struct NothingU64Hasher {
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

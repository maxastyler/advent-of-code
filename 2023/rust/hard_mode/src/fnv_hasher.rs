use core::hash::{BuildHasher, Hasher};

pub struct FNVHasher {
    val: u64,
}

impl FNVHasher {
    pub fn new() -> Self {
        Self {
            val: 14695981039346656037,
        }
    }
}

impl Hasher for FNVHasher {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|b| {
            self.val = self.val.wrapping_mul(1099511628211);
            self.val ^= *b as u64;
        })
    }
}

pub struct BuildFNVHasher;

impl BuildHasher for BuildFNVHasher {
    type Hasher = FNVHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new()
    }
}

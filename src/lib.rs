//! A very CRappy hASHer.


use std::{
	hash::{Hasher, BuildHasher},
	collections::{HashMap, HashSet},
};


pub type CrashMap<K, V> = HashMap<K, V, BuildCrasher>;
pub type CrashSet<K> = HashSet<K, BuildCrasher>;


/// This is equal to `2^64 / golden_ratio`.
/// 
/// Used by [`Crasher`] as the multiplication value, and by [`BuildCrasher`] as the hash seed.
pub const SEED: u64 = 0x9E37_79B9_7F4A_7C15;


#[derive(Debug, Clone, Copy)]
pub struct Crasher {
	hash: u64,
}

impl Hasher for Crasher {
	fn finish(&self) -> u64 {self.hash}
	
	fn write(&mut self, bytes: &[u8]) {
		for &b in bytes {
			self.write_u8(b);
		}
	}
	
	fn write_u8(&mut self, value: u8) {
		self.hash = self.hash
			.wrapping_add(u64::from(value))
			.wrapping_mul(SEED);
	}
}


#[derive(Debug, Clone, Copy)]
pub struct BuildCrasher {
	seed: u64,
}

impl BuildCrasher {
	/// Creates a `BuildCrasher` with a custom `seed`.
	pub fn new(seed: u64) -> Self {
		Self {seed}
	}
}

/// By default, `BuildCrasher` uses [`SEED`] as the hash seed.
impl Default for BuildCrasher {
	fn default() -> Self {
		Self::new(SEED)
	}
}

impl BuildHasher for BuildCrasher {
	type Hasher = Crasher;
	
	fn build_hasher(&self) -> Self::Hasher {
		Crasher {hash: self.seed}
	}
}

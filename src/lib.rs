//! A very CRappy hASHer.


use std::{
	hash::{Hasher, BuildHasherDefault},
	collections::{HashMap, HashSet},
};


pub type BuildCrasher<const SEED: u64 = DEFAULT_SEED> = BuildHasherDefault<Crasher<SEED>>;

pub type CrashMap<K, V> = HashMap<K, V, BuildCrasher>;
pub type CrashSet<K> = HashSet<K, BuildCrasher>;


/// This is equal to `2^64 / golden_ratio`.
pub const DEFAULT_SEED: u64 = 0x9E37_79B9_7F4A_7C15;


#[derive(Debug, Clone, Copy)]
pub struct Crasher<const SEED: u64> {
	hash: u64,
}

impl<const SEED: u64> Hasher for Crasher<SEED> {
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

impl<const SEED: u64> Default for Crasher<SEED> {
	fn default() -> Self {
		// Lazily just using the multiplication seed as the initial hash
		Self {hash: SEED}
	}
}

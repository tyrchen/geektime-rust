use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
    mem,
};

fn main() {
    let state = RandomState::new();
    let mut hasher = state.build_hasher();

    'a'.hash(&mut hasher);
    let hash = hasher.finish();
    println!(
        "hash: 0x{:x}, pos: {}, ctrl: 0x{:x}",
        hash,
        find_pos(hash, 0x3),
        h2(hash)
    );
}

fn find_pos(hash: u64, mask: usize) -> usize {
    let h = hash as usize;
    h & mask
}

fn h2(hash: u64) -> u8 {
    // Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
    // value, some hash functions (such as FxHash) produce a usize result
    // instead, which means that the top 32 bits are 0 on 32-bit platforms.
    let hash_len = usize::min(mem::size_of::<usize>(), mem::size_of::<u64>());
    let top7 = hash >> (hash_len * 8 - 7);
    (top7 & 0x7f) as u8 // truncation
}

#[derive(Debug)]
struct Pos {
    pos: usize,
    stride: usize,
    mask: usize,
}

// quadratic probing in hashbrown
#[allow(dead_code)]
fn probe_next(pos: &mut Pos) {
    pos.stride += 16;
    pos.pos += pos.stride;
    pos.pos &= pos.mask;

    println!("pos: {:?}", pos);
}

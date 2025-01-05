use risc0_zkvm::{guest::env, sha, sha::Sha256};

fn main() {
    let (num_iter, data): (u32, Vec<u8>) = env::read();

    let mut hash = sha::Impl::hash_bytes(&data);
    for _ in 1..num_iter {
        hash = sha::Impl::hash_bytes(hash.as_bytes());
    }

    env::commit(&hash)
}
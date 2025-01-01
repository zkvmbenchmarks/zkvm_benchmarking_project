use crate::env_adapters::NotImplementedEnv as env;

#[precompile]
use sha2::{Digest as _, Sha256};

use hex;

fn main() {
    let data: String = env::read(); // #input()

    let mut hasher = Sha256::new();

    hasher.update(data.as_bytes());

    let result = hasher.finalize();
    let hash_hex = hex::encode(result);

    env::commit(hash_hex);
}

#[host]
fn input() ->  Vec<u8> {
    b"Hello, world!".to_vec()
}

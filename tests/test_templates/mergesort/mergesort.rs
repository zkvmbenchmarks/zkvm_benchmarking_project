use crate::env_adapters::NotImplementedEnv as env;

#[host]
use rand::{Rng, SeedableRng};
#[host]
use rand::rngs::StdRng;

fn main() {
    let mut data: Vec<u8> = env::read(); // #input()

    merge_sort(&mut data);

    env::commit(&data);
}


fn merge_sort(arr: &mut [u8]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr, &left, &right);
}


fn merge(arr: &mut [u8], left: &[u8], right: &[u8]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[host]
fn input() -> Vec<u8> {
    // Generate a vector of size 1e5 with seeded random values
    let mut data = Vec::new();
    let seed = [0; 32]; // 32-byte array for seeding
    let mut rng = StdRng::from_seed(seed);

    for _ in 0..1e3 as usize {
        data.push(rng.gen());
    }
    data
}
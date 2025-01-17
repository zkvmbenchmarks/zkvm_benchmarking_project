use crate::env_adapters::NotImplementedEnv as env;

fn main() {
    let n: u64 = env::read(); // #input()
    let answer = is_prime(n);
    let answer_u32 = if answer { 1 } else { 0 };
    env::commit(&answer_u32);
}

#[host]
fn input() -> u64 {
    230932049823041
}
// Implementation from https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}



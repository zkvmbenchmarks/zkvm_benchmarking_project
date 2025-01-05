use risc0_zkvm::guest::env;

fn main() {
    let size: usize = env::read();
    let mut array = initialize_large_array(size);
    merge_sort(&mut array);
    let answer = array[(size/2) as usize]; //burda ne dondurmeliyiz emin degilim?
    env::commit(&answer);
}

fn initialize_large_array(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(result: &mut [i32], left: &[i32], right: &[i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}

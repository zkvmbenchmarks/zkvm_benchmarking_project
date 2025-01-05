pub fn test_func() -> i32 {
    let input = initialize_large_array(100);
    let sum: i32 = input.iter().sum();
    sum
}

fn initialize_large_array(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}
fn sliding_window_max_sum(arr: &[i32], window_size: usize) -> i32 {
    if arr.len() < window_size {
        return arr.iter().sum();
    }

    let mut window_sum: i32 = arr[..window_size].iter().sum();
    let mut max_sum = window_sum;
    
    for i in 0..arr.len() - window_size{
        window_sum = window_sum - arr[i] + arr[i + window_size];
        max_sum = max_sum.max(window_sum);
    }
    
    max_sum
}

pub fn test_sliding_window_max_sum() {
    let arr = [ 2, 1, 5, 1, 3, 2, 8, 4, 1, 9];
    let window_size = 3;
    let result = sliding_window_max_sum(&arr, window_size);
    println!("Max sum of sliding window of size {}: {}", window_size, result);
}
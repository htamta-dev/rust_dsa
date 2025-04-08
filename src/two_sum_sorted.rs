fn two_sum_sorted(arr: &[i32], target: i32) -> Vec<usize> {
    let mut indexed_nums: Vec<(&i32, usize)> =
        arr.iter().enumerate().map(|(i, val)| (val, i)).collect();

    indexed_nums.sort_by_key(|&(val, _)| val);

    let mut left = 0;
    let mut right = indexed_nums.len() - 1;

    while left < right {
        let curr_sum = indexed_nums[left].0 + indexed_nums[right].0;
        if curr_sum == target {
            return vec![indexed_nums[left].1, indexed_nums[right].1];
        } else if curr_sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    vec![]
}

pub fn test_two_sum_sorted() {
    let arr = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum_sorted(&arr, target);
    println!("Two sum result : {:?}", result);
}

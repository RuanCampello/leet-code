pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len() {
        let start = usize::max(0, i.saturating_sub(nums[i] as usize));
        for num in &nums[start..=i] {
            sum += num
        }
    }

    sum
}
fn main() {
    assert_eq!(subarray_sum(vec![2, 3, 1]), 11);
    assert_eq!(subarray_sum(vec![3, 1, 1, 2]), 13);
}

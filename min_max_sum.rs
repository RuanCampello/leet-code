fn max_and_min(numbers: &[i32]) -> (i32, i32) {
    let d = &i32::default();
    let mut large = numbers.get(0).unwrap_or(d);
    let mut min = large;

    for number in numbers {
        match large < number {
            true => large = number,
            false => min = number,
        }
    }

    (*large, *min)
}

pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let len = nums.len() * k as usize;

    for i in 0..len {
        let wrapped_index = i % nums.len();
        let start = wrapped_index;
        let end = (wrapped_index + 1).min(nums.len());

        let (max, min) = max_and_min(&nums[start..end]);
        sum += max + min;
    }

    sum
}

fn main() {
    assert_eq!(min_max_sums(vec![1, 2, 3], 2), 24);
    assert_eq!(min_max_sums(vec![5, 0, 6], 1), 22);
    assert_eq!(min_max_sums(vec![1, 1, 1], 2), 12);
    assert_eq!(min_max_sums(vec![262, 988], 2), 3750);
}

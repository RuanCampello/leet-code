pub fn kth_factor(n: i32, k: i32) -> i32 {
    (1..=n)
        .filter(|&num| n % num == 0)
        .nth((k - 1) as _)
        .unwrap_or(-1)
}

fn main() {
    assert_eq!(kth_factor(12, 3), 3);
    assert_eq!(kth_factor(7, 2), 7);
    assert_eq!(kth_factor(4, 4), -1);
}

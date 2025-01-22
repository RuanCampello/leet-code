pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let greatest = *candies.iter().max().unwrap();
    candies.into_iter().map(|candy| candy + extra_candies >= greatest).collect()
}

fn main() {
    assert_eq!(kids_with_candies(vec![2, 3, 5, 1, 3], 3), vec![true, true, true, false, true])
}
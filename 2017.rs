pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid[0].len();
    let mut prefix_one: Vec<i64> = grid[0].iter().map(|&x| x as i64).collect();
    let mut prefix_two: Vec<i64> = grid[1].iter().map(|&x| x as i64).collect();

    for i in 1..n {
        prefix_one[i] += prefix_one[i - 1];
        prefix_two[i] += prefix_two[i - 1];
    }

    let mut result = i64::MAX;
    for i in 0..n {
        let top = prefix_one[n - 1] - prefix_one[i];
        let bottom = if i > 0 { prefix_two[i - 1] } else { 0 };
        let second_robot_gain = i64::max(bottom, top);
        result = i64::min(result, second_robot_gain);
    }

    result
}


fn main() {
    assert_eq!(grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
    assert_eq!(grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
    assert_eq!(grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]), 7);
}
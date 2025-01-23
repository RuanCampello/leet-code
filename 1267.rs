pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let (row, col) = (grid.len(), grid[0].len());
    let (mut row_count, mut col_count) = (vec![0; row], vec![0; col]);

    // precompute the server cells
    for r in 0..row {
        for c in 0..col {
            if grid[r][c] == 1 {
                row_count[r] += 1;
                col_count[c] += 1;
            }
        }
    }

    for r in 0..row {
        for c in 0..col {
            if grid[r][c] == 1 && (row_count[r] > 1 || col_count[c] > 1) {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    assert_eq!(count_servers(vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]]), 4);
}
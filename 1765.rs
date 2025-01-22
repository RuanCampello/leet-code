use std::collections::{HashSet, VecDeque};

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut is_water = is_water;
    let (row, col) = (is_water.len(), is_water[0].len());

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    (0..row)
        .flat_map(|r| (0..col).map(move |c| (r, c)))
        .for_each(|(r, c)| {
            if is_water[r][c] == 1 {
                queue.push_back((r, c));
                visited.insert((r, c));
                is_water[r][c] = 0;
            }
        });

    // bfs
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        let height = is_water[r][c];
        let neighbours = [
            (r as isize + 1, c as isize), // down
            (r as isize, c as isize + 1), // right
            (r as isize - 1, c as isize), // up
            (r as isize, c as isize - 1), // left
        ];

        for (nr, nc) in neighbours {
            if nr >= 0 && nc >= 0 && (nr as usize) < row && (nc as usize) < col {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited.contains(&(nr, nc)) {
                    queue.push_back((nr, nc));
                    visited.insert((nr, nc));
                    is_water[nr][nc] = height + 1;
                }
            }
        }
    }

    is_water
}


fn main() {
    assert_eq!(highest_peak(vec![vec![0, 1], vec![0, 0]]), vec![vec![1, 0], vec![2, 1]])
}
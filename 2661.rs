use std::collections::HashMap;
struct Painted {
    rows: Vec<usize>,
    cols: Vec<usize>,
    completed_rows: Vec<bool>,
    completed_cols: Vec<bool>,
    height: usize,
    width: usize,
}

impl Painted {
    fn new(height: usize, width: usize) -> Self {
        Self {
            rows: vec![0; height],
            cols: vec![0; width],
            completed_rows: vec![false; height],
            completed_cols: vec![false; width],
            height,
            width,
        }
    }

    fn paint(&mut self, r: usize, c: usize) {
        if self.completed_rows[r] || self.completed_cols[c] {
            return;
        }
        self.rows[r] += 1;
        self.cols[c] += 1;

        if self.rows[r] == self.width {
            self.completed_rows[r] = true;
        }

        if self.cols[c] == self.height {
            self.completed_cols[c] = true;
        }
    }

    fn has_completed_something(&self) -> bool {
        self.completed_rows.iter().any(|&completed| completed)
            || self.completed_cols.iter().any(|&completed| completed)
    }
}

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let height = mat.len();
    let width = mat[0].len();
    let mut positions: HashMap<i32, (usize, usize)> = HashMap::with_capacity(height * width);

    for i in 0..height {
        for j in 0..width {
            positions.insert(mat[i][j], (i, j));
        }
    }

    let mut painted = Painted::new(height, width);
    for k in 0..arr.len() {
        let target = arr[k];
        if let Some(&(i, j)) = positions.get(&target) {
            painted.paint(i, j);
            if painted.has_completed_something() {
                return k as i32;
            }
        }
    }

    -1
}

fn main() {
    assert_eq!(
        first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
        2
    );
    assert_eq!(
        first_complete_index(
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
        ),
        3
    );
    assert_eq!(
        first_complete_index(vec![1, 4, 5, 2, 6, 3], vec![vec![4, 3, 5], vec![1, 2, 6]]),
        3
    );
}

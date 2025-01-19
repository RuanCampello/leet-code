use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let height = height_map.len();
    let width = height_map[0].len();

    let mut visited = vec![vec![false; width]; height];
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut max_height = 0;
    let mut volume = 0;

    let is_on_edge = |height_idx, width_idx| {
        height_idx == 0 || height_idx == height - 1 || width_idx == 0 || width_idx == width - 1
    };

    (0..height)
        .flat_map(|height_idx| {
            (0..width).map(move |width_idx| (height_idx, width_idx))
        })
        .filter(|&(height_idx, width_idx)| is_on_edge(height_idx, width_idx))
        .for_each(|(height_idx, width_idx)| {
            heap.push(Reverse((height_map[height_idx][width_idx], height_idx, width_idx)));
            visited[height_idx][width_idx] = true;
        });

    while let Some(Reverse((current_height, row, col))) = heap.pop() {
        max_height = max_height.max(current_height);

        for (direction_row, direction_col) in directions.iter() {
            let new_x = row as isize + *direction_row;
            let new_y = col as isize + *direction_col;

            if new_x >= 0 && new_y >= 0 && new_x < height as isize && new_y < width as isize {
                let (new_x, new_y) = (new_x as usize, new_y as usize);
                if !visited[new_x][new_y] {
                    visited[new_x][new_y] = true;
                    if height_map[new_x][new_y] < max_height {
                        volume += max_height - height_map[new_x][new_y];
                    }

                    heap.push(Reverse((height_map[new_x][new_y], new_x, new_y)));
                }
            }
        }
    }

    volume
}

fn main() {
    assert_eq!(
        trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1]
        ]),
        4
    );
    assert_eq!(
        trap_rain_water(vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3]
        ]),
        10
    );
}

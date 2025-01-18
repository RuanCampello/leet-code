struct Controller {
    direction: Direction,
    should_skip: bool,
}

impl Controller {
    fn new() -> Self {
        Self {
            should_skip: false,
            direction: Direction::Left,
        }
    }

    fn next(&mut self) {
        self.should_skip = match self.should_skip {
            true => false,
            false => true,
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn invert(&mut self) {
        *self = match *self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn range(&self, row: Vec<i32>) -> impl Iterator<Item = i32> {
        let range = match self {
            Self::Left => row,
            Self::Right => row.into_iter().rev().collect::<Vec<i32>>(),
        };

        range.into_iter()
    }
}

pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut controller = Controller::new();
    let mut output: Vec<i32> = Vec::new();

    for row in grid.into_iter() {
        let range = controller.direction.range(row);
        for cell in range {
            if !controller.should_skip {
                output.push(cell);
            }
            controller.next();
        }

        controller.direction.invert();
    }

    output
}

fn main() {
    let grid = vec![vec![2, 1], vec![2, 1], vec![2, 1]];
    zigzag_traversal(grid);
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    zigzag_traversal(grid);
}

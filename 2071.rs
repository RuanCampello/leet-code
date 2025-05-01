pub fn max_task_assign(
    mut tasks: Vec<i32>,
    mut workers: Vec<i32>,
    pills: i32,
    strength: i32,
) -> i32 {
    tasks.sort_unstable();
    workers.sort_unstable();

    let mut left = 0;
    let mut right = tasks.len().min(workers.len()) as i32;
    let mut answer = 0;

    while left <= right {
        let mid = (left + right) / 2;
        let mut j = workers.len() as i32 - 1;
        let mut p = pills;
        let mut possible = true;
        let mut queue = std::collections::VecDeque::new();

        for i in (0..mid).rev() {
            let task = tasks[i as usize];
            while j >= 0 && workers[j as usize] + strength >= task {
                queue.push_back(workers[j as usize]);
                j -= 1;
            }

            if queue.is_empty() {
                possible = false;
                break;
            }

            if queue.front().unwrap() >= &task {
                queue.pop_front();
            } else if p > 0 {
                queue.pop_back();
                p -= 1;
            } else {
                possible = false;
                break;
            }
        }

        match possible {
            true => {
                answer = mid;
                left = mid + 1;
            }
            false => right = mid - 1,
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let tasks = vec![3, 2, 1];
        let workers = vec![0, 3, 3];
        let pills = 1;
        let strength = 1;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 3);
    }

    #[test]
    fn test_example2() {
        let tasks = vec![5, 4];
        let workers = vec![0, 0, 0];
        let pills = 1;
        let strength = 5;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 1);
    }

    #[test]
    fn test_example3() {
        let tasks = vec![10, 15, 30];
        let workers = vec![0, 10, 10, 10, 10];
        let pills = 3;
        let strength = 10;
        assert_eq!(max_task_assign(tasks, workers, pills, strength), 2);
    }
}

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    candidates.sort_unstable();

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }

        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue; // duplication
            }
            if candidates[i] > target {
                break;
            }
            current.push(candidates[i]);
            backtrack(&candidates, target - candidates[i], i + 1, current, result);
            current.pop();
        }
    }

    backtrack(&candidates, target, 0, &mut current, &mut result);
    result
}

fn main() {
    let result = combination_sum2(vec![2, 5, 2, 1, 2], 5);
    print!("{:?}", result)
}

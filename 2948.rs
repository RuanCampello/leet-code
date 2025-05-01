use std::collections::{HashMap, VecDeque};

fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let mut groups: Vec<VecDeque<i32>> = Vec::new();
    let mut num_to_group: HashMap<i32, usize> = HashMap::new();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();

    for &num in &sorted_nums {
        if groups.is_empty() || (num - groups.last().unwrap().back().unwrap()).abs() > limit {
            groups.push(VecDeque::new());
        }
        groups.last_mut().unwrap().push_back(num);
        num_to_group.insert(num, groups.len() - 1);
    }

    let mut result = Vec::with_capacity(nums.len());
    for num in nums {
        let idx = num_to_group[&num];
        result.push(groups[idx].pop_front().unwrap());
    }

    result
}

fn main() {
    assert_eq!(lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2), vec![1, 3, 5, 8, 9]);
}
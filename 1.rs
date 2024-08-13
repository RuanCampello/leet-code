pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = std::collections::HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        // if indices.get returns some value, it will be stored in index
        if let Some(&index) = indices.get(&complement) {
            return vec![index as i32, i as i32];
        }
        indices.insert(num, i);
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let result = two_sum(nums, 9);

    println!("{:?}", result)
}

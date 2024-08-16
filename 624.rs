pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    if arrays.is_empty() {
        return 0;
    }
    //min is always the first element, as the array is ascending order
    let mut min = arrays[0][0];
    //max is always the last for the same reason
    let mut max = *arrays[0].last().unwrap();
    let mut max_distance = 0;

    for array in arrays.iter().skip(1) {
        let array_min = array[0];
        let array_max = *array.last().unwrap();

        // update max distance if (array_max - min) or (max - array_min) are greater
        max_distance = max_distance.max((array_max - min).abs().max((max - array_min).abs()));

        min = min.min(array_min);
        max = max.max(array_max);
    }

    max_distance
}

fn main() {
    // let first_array = vec![1, 2, 3];
    // let sec_array = vec![4, 5];
    // let third_array = vec![1, 2, 3];
    // let first_array = vec![1];
    // let sec_array = vec![1];
    // let first_array = vec![1];
    // let sec_array = vec![2];

    // let res = max_distance(vec![first_array, sec_array]);
    // let res = max_distance(vec![vec![1, 4], vec![0, 5]]);
    let res = max_distance(vec![vec![1, 5], vec![3, 4]]);

    // let res = max_distance(vec![first_array, sec_array, third_array]);

    println!("{}", res)
}

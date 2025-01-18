pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let n = derived.len();
    for start in [0, 1] {
        let mut original = vec![start];
        for i in 0..n - 1 {
            original.push(derived[i] ^ original[i]);
        }

        if derived[n - 1] == (original[n - 1] ^ original[0]) {
            return true;
        }
    }

    false
}


fn main() {
    let derived = vec![1, 1, 0];
    println!("1 {}", does_valid_array_exist(derived));
    let derived = vec![1, 1];
    println!("2 {}", does_valid_array_exist(derived));
    let derived = vec![1, 0];
    println!("3 {}", does_valid_array_exist(derived));
    let derived = vec![0, 1, 1];
    println!("4 {}", does_valid_array_exist(derived));
}
pub fn find_complement(num: i32) -> i32 {
    let mut mask = !0;
    // mask starts as 1, this loop trigger while the mask is not the complement of num
    // cause if it's, num & mask will be equals zero
    // while this condition is not true, shift mask
    while num & mask != 0 {
        mask <<= 1;
    }

    !num & !mask
}

fn main() {
    let result = find_complement(5);
    println!("{}", result);
}

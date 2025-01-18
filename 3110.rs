pub fn score_of_string(s: String) -> i32 {
    s.chars()
        .zip(s.chars().skip(1))
        .map(|(c1, c2)| (c1 as i32).abs_diff(c2 as i32) as i32)
        .sum()
}

fn main() {
    let string = "hello";
    println!("{}", score_of_string(string.into()));
    let string = "zaz";
    println!("{}", score_of_string(string.into()));
}

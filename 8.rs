pub fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    let (s, sign) = match s.strip_prefix("-") {
        Some(s) => (s, -1),
        None => (s.strip_prefix("+").unwrap_or(s), 1),
    };

    s.chars()
        .map_while(|c| c.to_digit(10))
        .fold(0, |count, digit| {
            count.saturating_mul(10).saturating_add(sign * digit as i32)
        })
}

fn main() {
    let s = "42";
    println!("{}", my_atoi(s.to_string()));
}

pub fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();
    x.chars().rev().collect::<String>() == x
}

fn main() {
    let result = is_palindrome(-121);
    print!("{}", result)
}

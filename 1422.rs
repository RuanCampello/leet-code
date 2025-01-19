pub fn max_score(s: String) -> i32 {
    let mut ones = s.matches('1').count() as i32;
    let mut score = 0;
    let mut zeros = 0;

    for (idx, char) in s.chars().enumerate() {
        match char {
            '1' => ones -= 1,
            '0' => zeros += 1,
            _ => {}
        }

        if idx < s.len() - 1 {
            score = i32::max(score, zeros + ones)
        }
    }

    score
}

fn main() {
    assert_eq!(max_score("011101".into()), 5);
    assert_eq!(max_score("00111".into()), 5);
    assert_eq!(max_score("1111".into()), 3);
    assert_eq!(max_score("00".into()), 1);
}

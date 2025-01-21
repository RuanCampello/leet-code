use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut combinations: Vec<String> = vec![String::new()];

    if digits.is_empty() {
        return Vec::new();
    }

    let keyboard: HashMap<char, &str> = HashMap::from([
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]);

    for digit in digits.chars() {
        if let Some(&letters) = keyboard.get(&digit) {
            let mut new_combinations = Vec::new();
            for combo in combinations {
                for letter in letters.chars() {
                    new_combinations.push(format!("{combo}{letter}"));
                }
            }

            combinations = new_combinations;
        }
    }

    combinations
}

fn main() {
    assert_eq!(
        letter_combinations("23".into()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(letter_combinations("2".into()), vec!["a", "b", "c"]);
    assert_eq!(
        letter_combinations("22".into()),
        vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
    );
}

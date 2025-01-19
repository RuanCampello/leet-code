use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let roman_symbols = HashMap::from([
        (1000, "M"),
        (500, "D"),
        (100, "C"),
        (50, "L"),
        (10, "X"),
        (5, "V"),
        (1, "I"),
    ]);

    let values: Vec<i32> = roman_symbols.keys().cloned().collect();

    let mut result = String::new();
    let mut remaining = num;

    for &value in &values {
        while remaining >= value {
            result.push_str(roman_symbols[&value]);
            remaining -= value;
        }
    }

    result
}

fn main() {
    let num = 1994;
    println!("{}", int_to_roman(num)); // "MCMXCIV"
}

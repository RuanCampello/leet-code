use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut prev_value: i32 = 0;

    let romans_symbols = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    for c in s.chars() {
        if let Some(&value) = romans_symbols.get(&c) {
            // if the current value is greater than the previous
            // it means that is a roman substraction
            // so, in that case, we remove the wrong value already in the result
            //
            // example: XL (40)
            // - add X (10) to result
            // result and prev_value = 10
            // procede to L (50)
            // wait, 50 is greater than 10, so we need to correct the count
            // result = 10 + 50 - 2 * 10 = 40

            if value > prev_value {
                result += value - 2 * prev_value;
            } else {
                result += value;
            }
            prev_value = value;
        }
    }

    result
}

fn main() {
    let result = roman_to_int("MCMXCIV".to_string());
    print!("{}", result)
}

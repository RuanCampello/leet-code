use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let romans_symbols = HashMap::from([
        (1, 'I'),
        (5, 'V'),
        (10, 'X'),
        (50, 'L'),
        (100, 'C'),
        (500, 'D'),
        (1000, 'M'),
    ]);

    let mut result = String::new();
    let mut remaining = num;

    for (&v, &s) in &romans_symbols {
        while remaining >= v {
            result.push(s);
            remaining -= v;
        }
    }

    result
}

fn main() {
    let num = 1994;
    println!("{}", int_to_roman(num))
}

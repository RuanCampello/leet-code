pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut output = String::new();

    for idx in 0..strs.len() {
        let str = &strs[idx].chars().nth(0).unwrap_or_default();
        let next = idx + 1;
        println!("{next} {idx}");

        while idx < strs.len() - next {
            let next_char = strs[idx + next].chars().nth(idx);

            match next_char {
                Some(char) => {
                    if char == *str {
                        output.push(*str);
                    }
                }
                None => match strs[idx + next].chars().nth(next) {
                    Some(char) => {
                        if char == *str {
                            output.push(*str);
                        }
                    }
                    None => {}
                },
            }

            break;
        }

        if idx == strs.len() - 1 {
            if *str == strs[idx].chars().nth(idx).unwrap_or_default() {
                output.push(*str);
            }
        }
    }

    output
}

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "fly".to_string()];
    println!("{:#?}", longest_common_prefix(strs))
}

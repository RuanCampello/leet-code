pub fn partition_string(s: String) -> i32 {
    let mut partitions = 1;
    let mut seen_chars = std::collections::HashSet::new();

    for char in s.chars() {
        if seen_chars.contains(&char) {
            partitions += 1;
            seen_chars.clear();
        }
        
        seen_chars.insert(char);
    }

    partitions
}

fn main() {
    assert_eq!(partition_string("abacaba".into()), 4);
    assert_eq!(partition_string("ssssss".into()), 6);
    assert_eq!(partition_string("cuieokbs".into()), 1);
}

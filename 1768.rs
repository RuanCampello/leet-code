pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut merged = String::new();
    let mut first = word1.chars().peekable();
    let mut last = word2.chars().peekable();

    while first.peek().is_some() || last.peek().is_some() {
        if let Some(first) = first.next() {
            merged.push(first);
        }
        if let Some(last) = last.next() {
            merged.push(last);
        }
    }

    merged
}

fn main() {
    assert_eq!(merge_alternately("abc".into(), "pqr".into()), "apbqcr");
    assert_eq!(merge_alternately("abcd".into(), "pq".into()), "apbqcd");
    assert_eq!(merge_alternately("ab".into(), "pqrs".into()), "apbqrs");
}
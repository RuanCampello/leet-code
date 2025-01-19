pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.match_indices(&needle).next() {
        Some((idx, _)) => idx as _,
        None => -1,
    }
}

fn main() {
    assert_eq!(str_str("sadbutsad".into(), "sad".into()), 0);
    assert_eq!(str_str("hello".into(), "ll".into()), 2)
}

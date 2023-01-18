pub struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut result = 0;
        for word in words {
            if word.starts_with(&pref[..]) {
                result += 1;
            }
        }
        result
    }
}

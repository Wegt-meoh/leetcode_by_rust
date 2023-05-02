pub struct Solution {}

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut prefix_end = s.len() - 1;
        let mut suffix_begin = 1;

        while prefix_end > 0 {
            let prefix = &s[0..prefix_end];
            let suffix = &s[suffix_begin..s.len()];
            if prefix == suffix {
                return String::from(prefix);
            }
            prefix_end -= 1;
            suffix_begin += 1;
        }

        String::new()
    }
}

#[test]
fn test() {
    let input = String::from("level");
    let ans = String::from("l");
    let result = Solution::longest_prefix(input);
    assert!(ans == result);

    let input = String::from("ababab");
    let ans = String::from("abab");
    let result = Solution::longest_prefix(input);
    assert!(ans == result);
}

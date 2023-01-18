pub struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let str = &s[..];
        let mut left = 0;
        let mut right = str.len() - 1;
        let mut curr_word = "_";
        while (left < right) {
            if str[left..left + 1] == str[right..right + 1] {
                curr_word = &str[left..left + 1];
                while left < right && (&str[left..left + 1] == curr_word) {
                    left += 1;
                }

                if left==right{
                    return 0;
                }

                while &str[right..right + 1] == curr_word {
                    right -= 1;
                }
            } else {
                break;
            }
        }

        (right - left + 1) as i32
    }
}

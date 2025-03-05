pub struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            return "".to_string();
        };

        let mut s: Vec<char> = palindrome.chars().collect();
        let end = s.len() / 2 - 1;
        for i in 0..=end {
            if i == end && s[i] == 'a' {
                s[palindrome.len() - 1] = 'b';
            } else if s[i] != 'a' {
                s[i] = 'a';
                break;
            }
        }
        // println!("{s:?}");

        return s.iter().fold("".to_string(), |mut acc, &x| {
            acc.push(x);
            return acc;
        });
    }
}

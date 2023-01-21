pub struct Solution {}

const LOWER_A_U8: u8 = b'a';

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let window_len = p.len();
        if window_len > s.len() {
            return vec![];
        }
        let mut map = [0; 26];
        let mut result = vec![];
        let s = s.into_bytes();
        let p = p.into_bytes();

        for item in p {
            map[(item - LOWER_A_U8) as usize] += 1;
        }

        for index in 0..window_len {
            map[(s[index] - LOWER_A_U8) as usize] -= 1;
        }

        if Self::check(&map) {
            result.push(0);
        }

        for index in 0..s.len() {
            map[(s[index] - LOWER_A_U8) as usize] += 1;
            if index + window_len >= s.len() {
                break;
            }

            map[(s[index + window_len] - LOWER_A_U8) as usize] -= 1;

            if Self::check(&map) {
                result.push((index + 1) as i32);
            }
        }

        result
    }

    fn check(map: &[i32; 26]) -> bool {
        for i in map {
            if *i != 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let res1 = Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc"));
    let res2 = Solution::find_anagrams(String::from("abab"), String::from("ab"));
    let res3 = Solution::find_anagrams(String::from("aa"), String::from("bb"));
    // println!("{:#?}", res1);
    assert!(res1 == vec![0, 6]);
    assert!(res2 == vec![0, 1, 2]);
    assert!(res3 == vec![]);
}

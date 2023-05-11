use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut char_vec = vec![];
        for mut x in 1..=n {
            while x > 0 {
                if x & 1 == 1 {
                    char_vec.push("1");
                } else {
                    char_vec.push("0");
                }
                x >>= 1;
            }
            char_vec.reverse();
            let patten = char_vec.join("");
            if s.contains(&patten) {
                char_vec.clear();
            } else {
                return false;
            }
        }
        true
    }

    pub fn query_string_better(s: String, n: i32) -> bool {
        if n == 1 {
            return s.contains("1");
        }

        let k = {
            let mut res = 1;
            while (1 << res) <= n {
                res += 1;
            }
            res - 1
        };

        let len1 = (1 << (k - 1)) + k - 1;
        let len2 = n as usize - (1 << k) + k + 1;

        if s.len() < len1 || s.len() < len2 {
            return false;
        }

        let mut hash_set = HashSet::new();
        let mut left_begin = 0;
        loop {
            // 这个end取不到
            let right_end = left_begin + k;
            if right_end > s.len() {
                break;
            }
            let s_str = &s[left_begin..right_end];
            if &s_str[0..1] == "1" {
                hash_set.insert(s_str);
            }

            let right_end = right_end + 1;
            if right_end > s.len() {
                break;
            }
            let s_str = &s[left_begin..right_end];
            if &s_str[0..1] == "1" {
                hash_set.insert(s_str);
            }

            left_begin += 1;
        }

        // println!("hash_set = {:?}", hash_set);

        for i in 1 << (k - 1)..=n {
            let mut t = i;
            let mut patten = String::new();
            while t > 0 {
                if t & 1 == 1 {
                    patten = "1".to_string() + &patten;
                } else {
                    patten = "0".to_string() + &patten;
                }
                t >>= 1;
            }

            if !hash_set.contains(&patten[..]) {
                return false;
            }
        }

        true
    }
}

// 给定一个二进制字符串 s 和一个正整数 n，如果对于 [1, n] 范围内的每个整数，其二进制表示都是 s 的 子字符串 ，就返回 true，否则返回 false 。

// 子字符串 是字符串中连续的字符序列。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/binary-string-with-substrings-representing-1-to-n
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let s = "0110".to_string();
    let n = 3;
    let ans = true;
    let result = Solution::query_string(s, n);
    assert!(ans == result);

    let s = "0110".to_string();
    let n = 4;
    let ans = false;
    let result = Solution::query_string(s, n);
    assert!(ans == result);

    let s = "10010111100001110010".to_string();
    let n = 10;
    let ans = false;
    let result = Solution::query_string(s, n);
    assert!(ans == result)
}

#[test]
fn test1() {
    let s = "0110".to_string();
    let n = 3;
    let ans = true;
    let result = Solution::query_string_better(s, n);
    assert!(ans == result, "ans={},result={}", ans, result);

    let s = "0110".to_string();
    let n = 4;
    let ans = false;
    let result = Solution::query_string_better(s, n);
    assert!(ans == result, "ans={},result={}", ans, result);

    let s = "10010111100001110010".to_string();
    let n = 10;
    let ans = false;
    let result = Solution::query_string_better(s, n);
    assert!(ans == result, "ans={},result={}", ans, result);
}

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let text: Vec<char> = text.chars().collect();
        let mut result = 0;
        let mut hash_map = HashMap::new();
        for i in 0..text.len() {
            hash_map.entry(text[i]).and_modify(|x| *x += 1).or_insert(1);
        }

        while right < text.len() {
            if right == text.len() - 1 || text[right + 1] != text[right] {
                let total = *hash_map.get(&text[left]).unwrap();
                if total > (right - left + 1) as i32 {
                    if left == 0 {
                        result = result.max((right - left + 1) as i32);
                    } else if left == 1 {
                        result = result.max((right - left + 1 + 1) as i32);
                    } else {
                        let mut i = left - 2;
                        let mut count = 0;
                        loop {
                            if text[i] == text[left] {
                                count += 1;
                                if i == 0 {
                                    break;
                                }
                                i -= 1;
                            } else {
                                break;
                            }
                        }

                        result = result.max(total.min((right - left + 1 + 1 + count) as i32));
                    }

                    if right < text.len() - 1 {
                        let mut i = right + 2;
                        let mut count = 0;
                        while i < text.len() {
                            if text[i] == text[right] {
                                count += 1;
                                i += 1;
                            } else {
                                break;
                            }
                        }
                        result = result.max(total.min((right - left + 1 + 1 + count) as i32));
                    }
                } else {
                    result = result.max((right - left + 1) as i32);
                }

                left = right + 1;
                right += 1;
            } else {
                right += 1;
            }
        }
        result
    }
}

// 如果字符串中的所有字符都相同，那么这个字符串是单字符重复的字符串。

// 给你一个字符串 text，你只能交换其中两个字符一次或者什么都不做，然后得到一些单字符重复的子串。返回其中最长的子串的长度。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/swap-for-longest-repeated-character-substring
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}

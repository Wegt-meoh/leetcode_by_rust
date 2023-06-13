use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];

        let f_words = {
            let mut a = words.iter().map(|s| Self::f(s)).collect::<Vec<_>>();
            a.sort();
            a
        };
        let f_que = queries.iter().map(|s| Self::f(s)).collect::<Vec<_>>();

        for (index, q) in f_que.iter().enumerate() {
            if *q < f_words[0] {
                result[index] = f_words.len() as i32;
                continue;
            }

            if q >= f_words.last().unwrap() {
                result[index] = 0;
                continue;
            }

            let mut left = 0;
            let mut right = f_words.len();
            while left < right {
                let mid = (left + right) / 2;
                if f_words[mid] > *q {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            if f_words[left] > *q {
                result[index] = (f_words.len() - left) as i32;
            } else {
                result[index] = (f_words.len() - left - 1) as i32;
            }
        }

        println!("f_que={:?}", f_que);
        println!("f_words={:?}", f_words);

        result
    }

    fn f(s: &str) -> i32 {
        let mut word_count = HashMap::new();
        for i in s.chars() {
            word_count.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut min_char = None;
        let mut min_char_count = 0;

        for (word, count) in word_count.iter() {
            if min_char == None || min_char.unwrap() > word {
                min_char = Some(word);
                min_char_count = *count;
            }
        }

        min_char_count
    }
}

// 定义一个函数 f(s)，统计 s  中（按字典序比较）最小字母的出现频次 ，其中 s 是一个非空字符串。

// 例如，若 s = "dcce"，那么 f(s) = 2，因为字典序最小字母是 "c"，它出现了 2 次。

// 现在，给你两个字符串数组待查表 queries 和词汇表 words 。对于每次查询 queries[i] ，需统计 words 中满足 f(queries[i]) < f(W) 的 词的数目 ，W 表示词汇表 words 中的每个词。

// 请你返回一个整数数组 answer 作为答案，其中每个 answer[i] 是第 i 次查询的结果。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}

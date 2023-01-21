use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let window_len = word_len * words.len();
        if window_len > s.len() {
            return vec![];
        }
        let mut word_map = HashMap::new();
        let mut result: Vec<i32> = vec![];

        'count: for begin in 0..word_len {
            word_map.clear();
            for i in words.iter() {
                word_map.entry(&i[..]).and_modify(|x| *x += 1).or_insert(1);
            }
            for index in (begin..begin + window_len).step_by(word_len) {
                if index >= s.len() || index + word_len > s.len() {
                    break 'count;
                }

                word_map
                    .entry(&s[index..index + word_len])
                    .and_modify(|x| *x -= 1)
                    .or_insert(-1);
            }

            if Self::check(&word_map) {
                result.push(begin as i32);
            }

            if (s.len() as isize - window_len as isize - word_len as isize) < 0 {
                continue;
            }

            for index in (begin..=s.len() - window_len - word_len).step_by(word_len) {
                word_map
                    .entry(&s[index..index + word_len])
                    .and_modify(|x| *x += 1)
                    .or_insert(1);

                if index + window_len >= s.len() || index + window_len + word_len > s.len() {
                    continue 'count;
                }
                word_map
                    .entry(&s[index + window_len..index + window_len + word_len])
                    .and_modify(|x| *x -= 1)
                    .or_insert(-1);

                if Self::check(&word_map) {
                    result.push((index + word_len) as i32);
                }
            }
        }

        result
    }

    fn check(map: &HashMap<&str, i32>) -> bool {
        for (_, value) in map {
            if *value != 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let res1 = Solution::find_substring(
        String::from("barfoothefoobarman"),
        vec![String::from("foo"), String::from("bar")],
    );
    let res2 = Solution::find_substring(
        String::from("wordgoodgoodgoodbestword"),
        vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("word"),
        ],
    );
    let res3 = Solution::find_substring(
        String::from("barfoofoobarthefoobarman"),
        vec![
            String::from("bar"),
            String::from("foo"),
            String::from("the"),
        ],
    );
    let res4 = Solution::find_substring(
        String::from("lingmindraboofooowingdingbarrwingmonkeypoundcake"),
        vec![
            String::from("fooo"),
            String::from("barr"),
            String::from("wing"),
            String::from("ding"),
            String::from("wing"),
        ],
    );
    let res5 = Solution::find_substring(String::from("a"), vec![String::from("a")]);
    let res6 = Solution::find_substring(
        String::from("a"),
        vec![String::from("a"), String::from("a")],
    );
    let res7 = Solution::find_substring(
        String::from("ababababab"),
        vec![String::from("ababa"), String::from("babab")],
    );

    assert!(res1 == vec![0, 9]);
    assert!(res2 == vec![]);
    assert!(res3 == vec![6, 9, 12]);
    assert!(res4 == vec![13]);
    assert!(res5 == vec![0]);
    assert!(res6 == vec![]);
    assert!(res7 == vec![0]);
}

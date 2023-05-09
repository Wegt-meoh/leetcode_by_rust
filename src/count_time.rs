use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let hour_part = &time[0..2];
        let min_part = &time[3..];
        Self::get_hour_part_times(hour_part) * Self::get_min_part_times(min_part)
    }

    fn get_hour_part_times(part: &str) -> i32 {
        let mut result = 0;
        let mut char_dequeue = VecDeque::with_capacity(2);
        for c in part.chars() {
            if c == '?' {
                char_dequeue.push_back(VecDeque::from([
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                ]));
            } else {
                char_dequeue.push_back(VecDeque::from([c]));
            }
        }

        let head = char_dequeue.pop_front().unwrap();
        let second = char_dequeue.pop_front().unwrap();
        for c in head {
            for c2 in second.iter() {
                let s_bytes = &[c as u8, *c2 as u8];
                let s = std::str::from_utf8(s_bytes).unwrap();
                if s >= "00" && s <= "23" {
                    result += 1;
                }
            }
        }

        result
    }

    fn get_min_part_times(part: &str) -> i32 {
        let mut result = 0;
        let mut char_dequeue = VecDeque::with_capacity(2);
        for c in part.chars() {
            if c == '?' {
                char_dequeue.push_back(VecDeque::from([
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                ]));
            } else {
                char_dequeue.push_back(VecDeque::from([c]));
            }
        }

        let head = char_dequeue.pop_front().unwrap();
        let second = char_dequeue.pop_front().unwrap();
        for c in head {
            for c2 in second.iter() {
                let s_bytes = &[c as u8, *c2 as u8];
                let s = std::str::from_utf8(s_bytes).unwrap();
                if s >= "00" && s <= "59" {
                    result += 1;
                }
            }
        }

        result
    }
}

// 给你一个长度为 5 的字符串 time ，表示一个电子时钟当前的时间，格式为 "hh:mm" 。最早 可能的时间是 "00:00" ，最晚 可能的时间是 "23:59" 。

// 在字符串 time 中，被字符 ? 替换掉的数位是 未知的 ，被替换的数字可能是 0 到 9 中的任何一个。

// 请你返回一个整数 answer ，将每一个 ? 都用 0 到 9 中一个数字替换后，可以得到的有效时间的数目。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/number-of-valid-clock-times
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let time = "?5:00".to_string();
    let ans = 2;
    let result = Solution::count_time(time);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let time = "0?:0?".to_string();
    let ans = 100;
    let result = Solution::count_time(time);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let time = "??:??".to_string();
    let ans = 1440;
    let result = Solution::count_time(time);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

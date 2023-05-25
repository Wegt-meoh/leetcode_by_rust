pub struct Solution {}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let n = words[0].len();
        let mut difference = vec![Vec::with_capacity(n - 1); words.len()];
        for i in 0..words.len() {
            let chars: Vec<char> = words[i].chars().collect();
            for j in 0..chars.len() - 1 {
                difference[i].push(chars[j + 1] as i32 - chars[j] as i32);
            }
        }

        let mut prev1: (Option<&Vec<i32>>, usize, usize) = (None, 0, 0);
        let mut prev2: (Option<&Vec<i32>>, usize, usize) = (None, 0, 0);
        for (index, i) in difference.iter().enumerate() {
            if prev1.0 == None {
                prev1.0 = Some(i);
                prev1.2 = index;
            } else if prev1.0.unwrap() == i {
                prev1.1 += 1;
            } else if prev2.0 == None {
                prev2.0 = Some(i);
                prev2.2 = index;
            } else {
                prev2.1 += 1;
            }
        }

        if prev1.1 < prev2.1 {
            return words[prev1.2].clone();
        } else {
            return words[prev2.2].clone();
        }
    }
}

// 给你一个字符串数组 words ，每一个字符串长度都相同，令所有字符串的长度都为 n 。

// 每个字符串 words[i] 可以被转化为一个长度为 n - 1 的 差值整数数组 difference[i] ，其中对于 0 <= j <= n - 2 有 difference[i][j] = words[i][j+1] - words[i][j] 。注意两个字母的差值定义为它们在字母表中 位置 之差，也就是说 'a' 的位置是 0 ，'b' 的位置是 1 ，'z' 的位置是 25 。

// 比方说，字符串 "acb" 的差值整数数组是 [2 - 0, 1 - 2] = [2, -1] 。
// words 中所有字符串 除了一个字符串以外 ，其他字符串的差值整数数组都相同。你需要找到那个不同的字符串。

// 请你返回 words中 差值整数数组 不同的字符串。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/odd-string-difference
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}

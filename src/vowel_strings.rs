pub struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len()];
        const YU: [&str; 5] = ["a", "e", "i", "o", "u"];

        let mut prev = 0;

        for i in 0..words.len() {
            if YU.contains(&&words[i][words[i].len() - 1..words[i].len()])
                && YU.contains(&&words[i][0..1])
            {
                prev += 1;
            }
            prefix_sum[i] = prev;
        }

        let mut result = vec![0; queries.len()];
        for i in 0..queries.len() {
            let right = queries[i][1] as usize;
            let left = queries[i][0] as usize;

            if left == 0 {
                result[i] = prefix_sum[right];
            } else {
                result[i] =
                    prefix_sum[right] - prefix_sum[left] + prefix_sum[left] - prefix_sum[left - 1];
            }
        }
        result
    }
}

// 给你一个下标从 0 开始的字符串数组 words 以及一个二维整数数组 queries 。

// 每个查询 queries[i] = [li, ri] 会要求我们统计在 words 中下标在 li 到 ri 范围内（包含 这两个值）并且以元音开头和结尾的字符串的数目。

// 返回一个整数数组，其中数组的第 i 个元素对应第 i 个查询的答案。

// 注意：元音字母是 'a'、'e'、'i'、'o' 和 'u' 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/count-vowel-strings-in-ranges
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}

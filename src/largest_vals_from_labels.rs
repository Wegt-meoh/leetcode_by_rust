use std::collections::HashMap;

pub struct Solution {}

#[derive(Eq, PartialEq, Debug, Ord)]
struct Pair {
    value: i32,
    label: i32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.value.cmp(&self.value))
    }
}

impl Pair {
    fn new(value: i32, label: i32) -> Self {
        Self { value, label }
    }
}

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut pair_vec = {
            let mut result = Vec::with_capacity(values.len());

            for i in 0..values.len() {
                result.push(Pair::new(values[i], labels[i]))
            }

            result.sort();
            result
        };

        let mut result = 0;
        let mut result_count = 0;
        let mut hash_map = HashMap::new();

        for i in pair_vec.iter() {
            if result_count == num_wanted {
                break;
            }
            let value = i.value;
            let label = i.label;

            let times = {
                let time = hash_map.get(&label);
                if time == None {
                    1
                } else {
                    time.unwrap() + 1
                }
            };

            if times <= use_limit {
                hash_map.entry(label).and_modify(|x| *x += 1).or_insert(1);
                result += value;
                result_count += 1;
            }
        }

        result
    }
}

// 我们有一个 n 项的集合。给出两个整数数组 values 和 labels ，第 i 个元素的值和标签分别是 values[i] 和 labels[i]。还会给出两个整数 numWanted 和 useLimit 。

// 从 n 个元素中选择一个子集 s :

// 子集 s 的大小 小于或等于 numWanted 。
// s 中 最多 有相同标签的 useLimit 项。
// 一个子集的 分数 是该子集的值之和。

// 返回子集 s 的最大 分数 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/largest-values-from-labels
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let values = vec![5, 4, 3, 2, 1];
    let labels = vec![1, 1, 2, 2, 3];
    let num_wanted = 3;
    let use_limit = 1;
    let ans = 9;
    let result = Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit);
    assert!(ans == result);
}

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let n = heights.len();
        let mut height_index: Vec<(i32, usize)> = (0..n).map(|x| (heights[x], x)).collect();

        height_index.sort_by(compare);

        height_index.iter().map(|x| names[x.1].clone()).collect()
    }
}

fn compare(this: &(i32, usize), other: &(i32, usize)) -> Ordering {
    other.0.cmp(&this.0)
}

#[test]
fn test() {
    // 给你一个字符串数组 names ，和一个由 互不相同 的正整数组成的数组 heights 。两个数组的长度均为 n 。

    // 对于每个下标 i，names[i] 和 heights[i] 表示第 i 个人的名字和身高。

    // 请按身高 降序 顺序返回对应的名字数组 names 。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/sort-the-people
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let names = vec!["Mary", "John", "Emma"];
    let names: Vec<String> = names.iter().map(|x| return String::from(*x)).collect();

    let heights = vec![180, 165, 170];

    let result = Solution::sort_people(names, heights);
    let ans = vec!["Mary", "Emma", "John"];
    let ans: Vec<String> = ans.iter().map(|x| return String::from(*x)).collect();

    assert!(result == ans);

    let names = vec!["Alice", "Bob", "Bob"];
    let names: Vec<String> = names.iter().map(|x| return String::from(*x)).collect();

    let heights = vec![155, 185, 150];

    let result = Solution::sort_people(names, heights);
    let ans = vec!["Bob", "Alice", "Bob"];
    let ans: Vec<String> = ans.iter().map(|x| return String::from(*x)).collect();

    assert!(result == ans);
}

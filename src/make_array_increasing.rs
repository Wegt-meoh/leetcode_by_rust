use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut hash_set = HashSet::new();
        arr2.iter().for_each(|x| {
            hash_set.insert(*x);
        });
        let arr2: Vec<i32> = hash_set.iter().map(|x| *x).collect();
        let max_times = arr1.len().min(arr2.len());

        todo!()
    }
}

#[test]
fn test() {
    // 给你两个整数数组 arr1 和 arr2，返回使 arr1 严格递增所需要的最小「操作」数（可能为 0）。

    // 每一步「操作」中，你可以分别从 arr1 和 arr2 中各选出一个索引，分别为 i 和 j，0 <= i < arr1.length 和 0 <= j < arr2.length，然后进行赋值运算 arr1[i] = arr2[j]。

    // 如果无法让 arr1 严格递增，请返回 -1。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/make-array-strictly-increasing
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![1, 3, 2, 4];
    let ans = 1;
    let result = Solution::make_array_increasing(arr1, arr2);
    assert!(result == ans);

    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![4, 3, 1];
    let ans = 2;
    let result = Solution::make_array_increasing(arr1, arr2);
    assert!(result == ans);

    let arr1 = vec![1, 5, 3, 6, 7];
    let arr2 = vec![1, 6, 3, 3];
    let ans = -1;
    let result = Solution::make_array_increasing(arr1, arr2);
    assert!(result == ans);
}

use std::collections::BinaryHeap;

pub struct Solution {}

#[derive(PartialEq, Eq)]
struct Pair {
    x: usize,
    y: usize,
    x_value: i32,
    y_value: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return (other.x_value + other.y_value).cmp(&(self.x_value + self.y_value));
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some((other.x_value + other.y_value).cmp(&(self.x_value + self.y_value)));
    }
}

impl Pair {
    fn new(x: usize, y: usize, x_value: i32, y_value: i32) -> Self {
        Self {
            x,
            y,
            x_value,
            y_value,
        }
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        heap.push(Pair::new(0, 0, nums1[0], nums2[0]));

        for i in 1..k as usize {
            if i < nums1.len() {
                heap.push(Pair::new(i, 0, nums1[i], nums2[0]))
            }
        }

        let mut result = vec![];

        for _ in 0..k {
            let item = heap.pop();
            if let Some(item) = item {
                result.push(vec![item.x_value, item.y_value]);
                if item.y + 1 < nums2.len() {
                    heap.push(Pair::new(
                        item.x,
                        item.y + 1,
                        nums1[item.x],
                        nums2[item.y + 1],
                    ));
                }
            } else {
                break;
            }
        }

        result
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let k = 3;
    let result = Solution::k_smallest_pairs(nums1, nums2, k);
    let ans = vec![vec![1, 2], vec![1, 4], vec![1, 6]];
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);
}

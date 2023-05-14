use std::collections::{BinaryHeap, HashMap};

pub struct Solution {}

#[derive(PartialEq, Eq, Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(barcodes.len());
        let mut hash_map = barcodes.iter().fold(HashMap::new(), |mut a, b| {
            a.entry(*b).and_modify(|x| *x += 1).or_insert(1);
            a
        });

        let mut max_time = 0;
        let mut max_item_key = 0;
        let mut prev_key = -1;

        for i in 0..barcodes.len() {
            for (key, value) in hash_map.iter() {
                if *value > 0 && *value > max_time && *key != prev_key {
                    max_time = *value;
                    max_item_key = *key;
                }
            }

            result.push(max_item_key);
            max_time = -1;
            hash_map.entry(max_item_key).and_modify(|x| *x -= 1);
            prev_key = max_item_key;
        }

        result
    }

    pub fn use_binaryheap(barcodes: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(barcodes.len());
        let mut hash_map = barcodes.iter().fold(HashMap::new(), |mut a, b| {
            a.entry(*b).and_modify(|x| *x += 1).or_insert(1);
            a
        });
        let mut vec_pair = hash_map
            .iter()
            .map(|(x, y)| Pair { x: *x, y: *y })
            .collect::<Vec<_>>();
        let mut big_binary_heap = BinaryHeap::from(vec_pair);
        let mut prev_numb = 0;

        println!("big heap={:?}", big_binary_heap);

        for _ in 0..barcodes.len() {
            let mut bigest_pair = big_binary_heap.pop().unwrap();
            if bigest_pair.x == prev_numb {
                let mut second_pair = big_binary_heap.pop().unwrap();
                prev_numb = second_pair.x;
                result.push(prev_numb);
                second_pair.y -= 1;
                big_binary_heap.push(bigest_pair);
                if second_pair.y > 0 {
                    big_binary_heap.push(second_pair);
                }
            } else {
                prev_numb = bigest_pair.x;
                result.push(prev_numb);
                bigest_pair.y -= 1;
                if bigest_pair.y > 0 {
                    big_binary_heap.push(bigest_pair);
                }
            }
        }

        result
    }

    pub fn better_method(b: Vec<i32>) -> Vec<i32> {
        let mut odd_index = 1;
        let mut even_index = 0;
        let mut hash_map: HashMap<i32, usize> = b.iter().fold(HashMap::new(), |mut a, b| {
            a.entry(*b).and_modify(|x| *x += 1).or_insert(1);
            a
        });
        let mut result = vec![0; b.len()];
        let half_time = b.len() / 2;

        for (numb, mut times) in hash_map {
            if times > half_time {
                for _ in 0..times {
                    result[even_index] = numb;
                    even_index += 2;
                }
            } else {
                for _ in 0..times {
                    if odd_index < result.len() {
                        result[odd_index] = numb;
                        odd_index += 2;
                    } else {
                        result[even_index] = numb;
                        even_index += 2;
                    }
                }
            }
        }
        result
    }
}

// 在一个仓库里，有一排条形码，其中第 i 个条形码为 barcodes[i]。

// 请你重新排列这些条形码，使其中任意两个相邻的条形码不能相等。 你可以返回任何满足该要求的答案，此题保证存在答案。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/distant-barcodes
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn check(a: &Vec<i32>) -> bool {
    for i in 0..a.len() - 2 {
        if a[i] == a[i + 1] {
            return false;
        }
    }

    true
}

#[test]
fn test() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let result = Solution::rearrange_barcodes(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::rearrange_barcodes(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::rearrange_barcodes(barcodes);
    assert!(check(&result), "result = {:?}", result);
}

#[test]
fn test1() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let result = Solution::better_method(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::better_method(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::better_method(barcodes);
    assert!(check(&result), "result = {:?}", result);
}

#[test]
fn test2() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let result = Solution::use_binaryheap(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::use_binaryheap(barcodes);
    assert!(check(&result), "result = {:?}", result);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let result = Solution::use_binaryheap(barcodes);
    assert!(check(&result), "result = {:?}", result);
}

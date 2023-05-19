use std::{collections::HashMap, ops::Add};

pub struct Solution {}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut result = 0;
        let mut hash_map = tiles.chars().fold(HashMap::new(), |mut a, b| {
            a.entry(b).and_modify(|x| *x += 1).or_insert(1);
            a
        });
        let keys = hash_map.keys().map(|a| *a).collect::<Vec<_>>();

        // return the answer
        fn dfs(hash_map: &mut HashMap<char, i32>, keys: &Vec<char>, index: usize) -> i32 {
            if index == 0 {
                return 1;
            }

            // why sum init value is 1
            let mut sum = 1;

            for i in keys {
                match hash_map.get(i) {
                    Some(value) => {
                        if *value > 0 {
                            hash_map.entry(*i).and_modify(|x| *x -= 1);
                            sum += dfs(hash_map, keys, index - 1);
                            hash_map.entry(*i).and_modify(|x| *x += 1);
                        }
                    }
                    None => (),
                }
            }

            return sum;
        }

        // why result should sub 1
        dfs(&mut hash_map, &keys, tiles.len()) - 1
    }

    fn not_use_map(tiles: String) -> i32 {
        let mut byte_count: [i32; 26] = [0; 26];
        for b in tiles.bytes() {
            byte_count[b as usize - 'A' as usize] += 1;
        }

        fn dfs(byte_count: &mut [i32; 26], curr: &mut Vec<u8>) -> i32 {
            let mut res = 0;

            println!("curr={}", String::from_utf8(curr.clone()).unwrap());

            for i in 0..26 {
                if byte_count[i] > 0 {
                    byte_count[i] -= 1;
                    curr.push(b'A' + i as u8);
                    res += dfs(byte_count, curr) + 1;
                    curr.pop();
                    byte_count[i] += 1;
                }
            }

            res
        }

        dfs(&mut byte_count, &mut vec![])
    }
}

// 你有一套活字字模 tiles，其中每个字模上都刻有一个字母 tiles[i]。返回你可以印出的非空字母序列的数目。

// 注意：本题中，每个活字字模只能使用一次。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/letter-tile-possibilities
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let tiles = "AAB".to_string();
    let ans = 8;
    let result = Solution::num_tile_possibilities(tiles);
    assert!(ans == result, "ans={ans}, result={result}");

    let tiles = "AAABBC".to_string();
    let ans = 188;
    let result = Solution::num_tile_possibilities(tiles);
    assert!(ans == result, "ans={ans}, result={result}");

    let tiles = "A".to_string();
    let ans = 1;
    let result = Solution::num_tile_possibilities(tiles);
    assert!(ans == result, "ans={ans}, result={result}");
}

#[test]
fn test1() {
    let tiles = "AAB".to_string();
    let ans = 8;
    let result = Solution::not_use_map(tiles);
    assert!(ans == result, "ans={ans}, result={result}");

    let tiles = "AAABBC".to_string();
    let ans = 188;
    let result = Solution::not_use_map(tiles);
    assert!(ans == result, "ans={ans}, result={result}");

    let tiles = "A".to_string();
    let ans = 1;
    let result = Solution::not_use_map(tiles);
    assert!(ans == result, "ans={ans}, result={result}");
}

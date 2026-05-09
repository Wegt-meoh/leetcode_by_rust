use std::collections::{HashMap, VecDeque};

pub struct Solution;

const MX: usize = 1_000_001;

lazy_static::lazy_static! {
    static ref FACTORS: Vec<Vec<i32>> = {
        let mut f = vec![vec![]; MX];
        for i in 2..MX {
            if f[i].is_empty() {
                for j in (i..MX).step_by(i) {
                    f[j].push(i as i32);
                }
            }
        }
        f
    };
}

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return n as i32 - 1;
        }

        let mut flag_graph = vec![false; n];
        let mut prime_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            for prime in FACTORS[*num as usize].iter() {
                prime_map.entry(*prime).or_default().push(index);
            }
        }

        let mut step = 0;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        flag_graph[0] = true;

        while !queue.is_empty() {
            let times = queue.len();
            for _ in 0..times {
                let item = queue.pop_front().unwrap();
                if item == n - 1 {
                    return step;
                }

                if item > 0 && !flag_graph[item - 1] {
                    queue.push_back(item - 1);
                    flag_graph[item - 1] = true;
                }
                if item + 1 < n && !flag_graph[item + 1] {
                    queue.push_back(item + 1);
                    flag_graph[item + 1] = true;
                }

                if FACTORS[nums[item] as usize].len() == 1 {
                    if let Some(next_vec) = prime_map.get(&nums[item]) {
                        for next in next_vec {
                            if !flag_graph[*next] {
                                queue.push_back(*next);
                                flag_graph[*next] = true;
                            }
                        }
                        prime_map.remove(&nums[item]);
                    }
                }
            }
            step += 1;
        }
        -1
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }

        // Check small primes first
        let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for &p in &small_primes {
            if n == p {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }

        let limit = (n as f32).sqrt() as i32;
        let mut i = 31; // Start from 31
        while i <= limit {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

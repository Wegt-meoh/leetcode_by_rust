use std::collections::VecDeque;

pub mod collections {
    use super::*;
    pub struct OrderedQueue<T> {
        queue: VecDeque<T>,
    }

    impl<T> OrderedQueue<T>
    where
        T: Ord + PartialOrd + Copy,
    {
        pub fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }

        pub fn push(&mut self, item: T) {
            let idx = self.queue.partition_point(|&x| x < item);
            self.queue.insert(idx, item);
        }

        pub fn front(&self) -> Option<&T> {
            self.queue.front()
        }

        pub fn back(&self) -> Option<&T> {
            self.queue.back()
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.queue.pop_front()
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.queue.pop_back()
        }

        pub fn len(&self) -> usize {
            self.queue.len()
        }

        pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
            self.queue.iter()
        }

        pub fn remove(&mut self, item: &T) -> Option<T> {
            match self.queue.binary_search(item) {
                Ok(idx) => self.queue.remove(idx),
                Err(_) => None,
            }
        }
    }
}

pub mod transform {
    pub fn tuple_to_vec() {}
}

pub mod utils {
    use std::collections::{HashMap, HashSet};

    pub fn kmp(s: &str, patten: &str) -> Option<usize> {
        fn init_state_dp(patten: &str) -> (Vec<Vec<usize>>, HashMap<char, usize>) {
            let parren_char = patten.chars().collect::<Vec<_>>();

            // 映射patten中的char到一个数字，这个数字作为state_dp[i][j]中的j下标
            let char_index_map: HashMap<char, usize> = parren_char
                .iter()
                .fold(HashSet::new(), |mut a, b| {
                    a.insert(*b);
                    a
                })
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut a, (index, b)| {
                    a.insert(*b, index);
                    a
                });

            let mut state_dp = vec![Vec::with_capacity(char_index_map.len()); patten.len()];
            for j in 0..char_index_map.len() {
                state_dp[0][j] = 0;
            }
            let char_index = char_index_map
                .get(&parren_char[0])
                .expect("patten char must exist in char_index_map");
            state_dp[0][*char_index] = 1;

            for i in 1..patten.len() {
                let char_index = char_index_map
                    .get(&parren_char[i])
                    .expect("patten char must exist in char_index_map");
                for j in 0..char_index_map.len() {
                    if char_index == &j {
                        state_dp[i][j] = i + 1;
                    } else {
                    }
                }
            }

            (state_dp, char_index_map)
        }

        fn search(
            s: &str,
            state_dp: Vec<Vec<usize>>,
            char_index_map: HashMap<char, usize>,
        ) -> Option<usize> {
            let mut i = 0;
            let mut j = 0;
            let s_chars = s.chars().collect::<Vec<_>>();

            while i < s.len() {
                if j == state_dp.len() {
                    return Some(i);
                }
                match char_index_map.get(&s_chars[i]) {
                    Some(index) => {
                        j = state_dp[j][*index];
                    }
                    None => {
                        return None;
                    }
                }
                i += 1;
            }

            None
        }

        let (state_dp, char_index_map) = init_state_dp(patten);
        search(s, state_dp, char_index_map)
    }
}

#[test]
fn test_kmp() {
    use self::utils::kmp;
    let s = "sjalkdfja jfald jf dklajska slkdjf k";
    let patten = "fald";
    let ans = s.find(patten);
    let result = kmp(s, patten);
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);

    let s = "sjalkdfja jfald jf dklajska slkdjf k";
    let patten = "aaaa";
    let ans = s.find(patten);
    let result = kmp(s, patten);
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);
}

use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

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
    use super::*;
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode<T: Copy> {
        pub val: T,
        pub left: Option<Rc<RefCell<TreeNode<T>>>>,
        pub right: Option<Rc<RefCell<TreeNode<T>>>>,
    }

    impl<T: Copy> TreeNode<T> {
        fn new(val: T) -> Self {
            TreeNode {
                val: val,
                left: None,
                right: None,
            }
        }
    }

    pub fn build_tree_from_vec<T: Copy>(v: Vec<Option<T>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if v.len() == 0 {
            return None;
        }
        let mut queue = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(v[0].unwrap())));
        queue.push_back(Rc::clone(&root));
        for i in (1..v.len()).step_by(2) {
            let a = v[i];
            let b = v[i + 1];
            let node = queue.pop_front().unwrap();
            if let Some(a) = a {
                let left_node = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push_back(left_node);
            }

            if let Some(b) = b {
                let right_node = Rc::new(RefCell::new(TreeNode::new(b)));
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                queue.push_back(right_node);
            }
        }
        Some(root)
    }
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

#[test]
fn test_build_tree() {
    use self::transform::{build_tree_from_vec, TreeNode};
    let root = build_tree_from_vec(vec![
        Some(1),
        Some(4),
        Some(3),
        Some(2),
        Some(4),
        Some(2),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(4),
        Some(6),
    ]);

    let ans = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));

    assert!(ans == root, "ans={:#?}, root={:#?}", ans, root);
}

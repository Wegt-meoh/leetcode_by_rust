use std::borrow::BorrowMut;

pub struct Solution {}

type Link<T> = Option<Box<T>>;

struct WordTree {
    is_end: bool,
    //0
    left_node: Link<Self>,
    //1
    right_node: Link<Self>,
}

impl WordTree {
    fn new(is_end: bool, left_node: Link<Self>, right_node: Link<Self>) -> Box<Self> {
        Box::new(Self {
            is_end,
            left_node,
            right_node,
        })
    }
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        todo!()
    }

    fn f(x: i32, nums: &Vec<i32>) -> i32 {
        todo!()
    }

    fn generate_tree(nums: &Vec<String>) -> Box<WordTree> {
        let root = WordTree::new(false, None, None);

        for this_string in nums.iter() {
            let mut fake_root = &mut root;
            for (index, this_char) in this_string.char_indices() {
                if this_char == '0' {
                    match &fake_root.left_node {
                        Some(left_node) => {                            
                            // fake_root=;
                        },
                        None => {
                            fake_root.left_node=Some(WordTree::new(false, None, None));
                        },
                    }                    
                } else {
                }
            }
        }
        root
    }

    pub fn num_to_2(mut x: i32) -> String {
        let mut t = String::new();
        let mut result = String::new();

        while x > 0 {
            if x % 2 == 0 {
                t += "0"
            } else {
                t += "1";
            }
            x /= 2;
        }

        while t.len() < 15 {
            t += "0";
        }

        for i in (0..t.len()).rev() {
            result += &t[i..i + 1];
        }

        result
    }
}

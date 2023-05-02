// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut node = vec![];
        let mut queue = VecDeque::new();

        while head.is_some() {
            node.push(head.as_ref().unwrap().val);
            head = head.unwrap().next;
        }

        let mut result = vec![0; node.len()];

        for i in 0..node.len() {
            while !queue.is_empty() && node[queue[queue.len() - 1] as usize] < node[i] {
                result[queue.pop_back().unwrap() as usize] = node[i];
            }
            queue.push_back(i as i32);
        }
        todo!()
    }
}

fn getHead(input: Vec<i32>) -> Option<Box<ListNode>> {
    if input.len() > 0 {
        let mut head = Some(Box::new(ListNode::new(input[0])));
        let node = &mut head;
        for i in 1..input.len() {
            node.as_mut().unwrap().next = Some(Box::new(ListNode::new(input[i])));
        }
        return head;
    } else {
        return None;
    }
}

#[test]
fn test() {
    let input = vec![2, 1, 5];
    let ans = vec![5, 5, 0];
    let result = Solution::next_larger_nodes(getHead(input));

    let input = vec![2, 7, 4, 3, 5];
    let ans = vec![7, 0, 5, 5, 0];
    let result = Solution::next_larger_nodes(getHead(input));
}

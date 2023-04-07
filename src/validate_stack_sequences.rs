pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut push_index = 0;
        let mut pop_index = 0;
        let n = pushed.len();
        let mut stack = Vec::with_capacity(n);

        if n == 0 {
            return false;
        }
        while pop_index < n {
            let pop_item = popped[pop_index];
            pop_index += 1;

            if stack.is_empty() == false && stack[stack.len() - 1] == pop_item {
                stack.pop();
                continue;
            }

            for i in stack.iter() {
                if pop_item == *i {
                    return false;
                }
            }

            while push_index < n && pushed[push_index] != pop_item {
                stack.push(pushed[push_index]);
                push_index += 1;
            }

            push_index += 1;
        }

        if pop_index == n && push_index == n && stack.len() == 0 {
            return true;
        }

        false
    }
}

#[test]
fn test() {
    let input = vec![1, 2, 3, 4, 5];
    let input2 = vec![4, 5, 3, 2, 1];
    let ans = true;
    let res = Solution::validate_stack_sequences(input, input2);

    println!("ans ={},res={}", ans, res);
    assert!(ans == res);

    let input = vec![1, 2, 3, 4, 5];
    let input2 = vec![4, 3, 5, 1, 2];
    let ans = false;
    let res = Solution::validate_stack_sequences(input, input2);

    println!("ans ={},res={}", ans, res);
    assert!(ans == res);
}

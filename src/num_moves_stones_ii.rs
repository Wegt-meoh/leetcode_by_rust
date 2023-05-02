pub struct Solution {}

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort();

        let n = stones.len() as i32;

        let mut max_ans = 0;
        let mut min_ans = 0;

        let empty_item = stones.last().unwrap() - stones[0] + 1 - n;

        return vec![min_ans, max_ans];
    }
}

#[test]
fn test() {
    let input = vec![7, 4, 9];
    let ans = vec![1, 2];
    let res = Solution::num_moves_stones_ii(input);

    assert!(res == ans);

    let input = vec![6, 5, 4, 3, 10];
    let ans = vec![2, 3];
    let res = Solution::num_moves_stones_ii(input);

    assert!(res == ans);

    let input = vec![100, 101, 104, 102, 103];
    let ans = vec![0, 0];
    let res = Solution::num_moves_stones_ii(input);

    assert!(res == ans);
}

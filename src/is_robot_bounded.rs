pub struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let direction = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut current_direction = 0;
        let mut current_position = (0, 0);
        for i in 0..instructions.len() {
            let c = &instructions[i..i + 1];
            if c == "G" {
                current_position.0 += direction[current_direction].0;
                current_position.1 += direction[current_direction].1;
            } else if c == "L" {
                current_direction = (current_direction + 1) % direction.len();
            } else if current_direction == 0 {
                current_direction = 3;
            } else {
                current_direction -= 1;
            }
        }

        if current_position == (0, 0) {
            true
        } else {
            current_direction != 0
        }
    }
}

#[test]
fn test() {
    let input = String::from("GGLLGG");
    let ans = true;
    let res = Solution::is_robot_bounded(input);
    println!("ans={},res={}", ans, res);
    assert!(ans == res);

    let input = String::from("GG");
    let ans = false;
    let res = Solution::is_robot_bounded(input);
    println!("ans={},res={}", ans, res);
    assert!(ans == res);

    let input = String::from("GL");
    let ans = true;
    let res = Solution::is_robot_bounded(input);
    println!("ans={},res={}", ans, res);
    assert!(ans == res);
}

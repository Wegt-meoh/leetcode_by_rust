pub struct Solution {}

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let z_num = (k - n) / 25;
        if z_num == n {
            return "z".repeat(n);
        }
        let p_value = ((k - n) % 25) as u8;
        let a_num = n - z_num - 1;
        let mut result = String::with_capacity(n);
        for _ in 0..a_num {
            result.push('a');
        }
        result.push((b'a' + p_value) as char);
        for _ in 0..z_num {
            result.push('z');
        }
        result
    }
}

#[test]
fn test() {
    let res = Solution::get_smallest_string(3, 27);
    assert!(res == String::from("aay"));
    let res1 = Solution::get_smallest_string(5, 73);
    println!("{res1}");
    assert!(res1 == String::from("aaszz"));
}

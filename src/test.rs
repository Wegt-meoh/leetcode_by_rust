pub struct Solution;

impl Solution {
    fn test() {
        let a = String::from("sdfas");
        for c in a.chars() {
            let b = c as i32 - 'a' as i32;
            println!("{b}");
        }
    }
}

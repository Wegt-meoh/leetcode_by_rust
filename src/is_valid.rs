pub struct Solution {}

impl Solution {
    pub fn is_valid2(mut s: String) -> bool {
        if s.len() < 3 {
            return false;
        }

        let pattern = "abc";

        while s.len() > 0 {
            match s.find(pattern) {
                Some(index) => {
                    let left = &s[0..index];
                    let right = {
                        if index + 3 >= s.len() {
                            ""
                        } else {
                            &s[index + 3..]
                        }
                    };
                    s = String::from(left) + right;
                }
                None => {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for i in 0..s.len() {
            let item = &s[i..i + 1];
            if item == "c" {
                let prev1=stack.pop();
                let prev2=stack.pop();
                if prev1!=Some("b")||prev2!=Some("a"){
                    return false;
                }
            } else {
                stack.push(item)
            }
        }

        if stack.is_empty() {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    // 给你一个字符串 s ，请你判断它是否 有效 。
    // 字符串 s 有效 需要满足：假设开始有一个空字符串 t = "" ，你可以执行 任意次 下述操作将 t 转换为 s ：

    // 将字符串 "abc" 插入到 t 中的任意位置。形式上，t 变为 tleft + "abc" + tright，其中 t == tleft + tright 。注意，tleft 和 tright 可能为 空 。
    // 如果字符串 s 有效，则返回 true；否则，返回 false。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/check-if-word-is-valid-after-substitutions
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let s = String::from("aabcbc");
    let ans = true;
    let result = Solution::is_valid(s);
    assert!(ans == result);

    let s = String::from("abcabcababcc");
    let ans = true;
    let result = Solution::is_valid(s);
    assert!(ans == result);

    let s = String::from("abccba");
    let ans = false;
    let result = Solution::is_valid(s);
    assert!(ans == result);
}

#[test]
fn test1() {
    let s = String::from("dsafadfs");
    println!("{}", &s[200..]);
}

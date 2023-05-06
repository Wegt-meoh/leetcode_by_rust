use std::io::Read;

pub struct Solution {}

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut max_num = 0;
        let mut curr_num = 0;
        let mut count_vec: [i32; 5] = [0; 5];

        for c in croak_of_frogs.chars() {
            let index = Self::get_index(&c);
            if c == 'c' {
                count_vec[index] += 1;
                curr_num += 1;
                max_num = max_num.max(curr_num);
            } else {
                if count_vec[index - 1] == 0 {
                    return -1;
                }

                count_vec[index - 1] -= 1;
                count_vec[index] += 1;

                if c == 'k' {
                    count_vec[index] -= 1;
                    curr_num -= 1;
                }
            }
        }

        for i in count_vec {
            if i != 0 {
                return -1;
            }
        }

        max_num
    }

    fn get_index(c: &char) -> usize {
        const CHECK: [char; 5] = ['c', 'r', 'o', 'a', 'k'];
        for i in 0..CHECK.len() {
            if c == &CHECK[i] {
                return i;
            }
        }

        0
    }
}

#[test]
fn test() {
    let croak_of_frogs = "croakcroak".to_string();
    let ans = 1;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    assert!(ans == result);

    let croak_of_frogs = "crcoakroak".to_string();
    let ans = 2;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    assert!(ans == result);

    let croak_of_frogs = "crroakcoak".to_string();
    let ans = -1;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    assert!(ans == result);

    let croak_of_frogs = "croakcrook".to_string();
    let ans = -1;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    assert!(ans == result);
}

#[test]
fn test2() {
    let croak_of_frogs = "croakcroa".to_string();
    let ans = -1;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test1() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./src/input_data/min_number_of_frogs.input.data")?;
    let mut croak_of_frogs = String::new();
    file.read_to_string(&mut croak_of_frogs)?;
    let ans = 16404;
    let result = Solution::min_number_of_frogs(croak_of_frogs);
    assert!(ans == result);
    Ok(())
}

// 给你一个字符串 croakOfFrogs，它表示不同青蛙发出的蛙鸣声（字符串 "croak" ）的组合。由于同一时间可以有多只青蛙呱呱作响，所以 croakOfFrogs 中会混合多个 “croak” 。

// 请你返回模拟字符串中所有蛙鸣所需不同青蛙的最少数目。

// 要想发出蛙鸣 "croak"，青蛙必须 依序 输出 ‘c’, ’r’, ’o’, ’a’, ’k’ 这 5 个字母。如果没有输出全部五个字母，那么它就不会发出声音。如果字符串 croakOfFrogs 不是由若干有效的 "croak" 字符混合而成，请返回 -1 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/minimum-number-of-frogs-croaking
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

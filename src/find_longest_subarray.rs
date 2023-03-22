use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_longest_subarray(mut array: Vec<String>) -> Vec<String> {
        let mut prefix_sum = HashMap::new();
        prefix_sum.insert(0, vec![-1]);
        let mut curr_sum = 0;
        for (index, item) in array.iter().enumerate() {
            if string_is_number(item) {
                curr_sum += 1;
            } else {
                curr_sum -= 1;
            }
            prefix_sum
                .entry(curr_sum)
                .and_modify(|x| x.push(index as i32))
                .or_insert(vec![index as i32]);
        }

        let mut ans_start = -1;
        let mut max_length = 0;

        println!("prefix_sum_hash_table={:?}", prefix_sum);

        for (k, v) in prefix_sum {
            let curr_min = get_vec_min_item(&v);
            let curr_max = get_vec_max_item(&v);
            let curr_length = curr_max - curr_min - 1;
            if curr_length > max_length || curr_length == max_length && curr_min < ans_start {
                println!(
                    "found another ans curr_min={},max_length={}",
                    curr_min, curr_length
                );

                ans_start = curr_min + 1;
                max_length = curr_length
            }
        }

        println!("ans_start={} max_length={}", ans_start, max_length);

        if ans_start == -1 {
            vec![]
        } else {
            array
                .splice(
                    ans_start as usize..(ans_start as usize + max_length as usize + 1),
                    vec![],
                )
                .collect()
        }
    }
}

fn get_vec_max_item(v: &Vec<i32>) -> i32 {
    let mut max_ans = -1;
    for item in v {
        if max_ans < *item {
            max_ans = *item
        }
    }
    max_ans
}

fn get_vec_min_item(v: &Vec<i32>) -> i32 {
    let mut min_ans = std::i32::MAX;
    for item in v {
        if min_ans > *item {
            min_ans = *item
        }
    }
    min_ans
}

fn string_is_number(s: &String) -> bool {
    let c: Vec<char> = s.chars().collect();
    if char::is_ascii_alphabetic(&c[0]) {
        return false;
    }
    true
}

#[test]
fn test() {
    let input1 = convert_str_vec_to_string(vec![
        "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I", "J", "K",
        "L", "M",
    ]);
    let ans1 = convert_str_vec_to_string(vec![
        "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7",
    ]);

    let res1 = Solution::find_longest_subarray(input1);

    println!("res1={:?} ans1={:?}", res1, ans1);
    assert!(res1 == ans1);

    let input2 = convert_str_vec_to_string(vec!["A", "A"]);
    let ans2: Vec<String> = vec![];
    let res2 = Solution::find_longest_subarray(input2);
    assert!(res2 == ans2);
}

fn convert_str_vec_to_string(str_vec: Vec<&str>) -> Vec<String> {
    let mut res = vec![];

    for item in str_vec {
        res.push(String::from(item))
    }

    res
}

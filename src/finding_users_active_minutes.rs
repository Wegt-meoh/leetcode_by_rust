use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut user_count: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut result = vec![0; k as usize];

        for log in logs {
            let id = log[0];
            let time = log[1];

            user_count
                .entry(id)
                .and_modify(|x| {
                    x.insert(time);
                })
                .or_insert(HashSet::from([time]));
        }

        for (_, set) in user_count {
            result[set.len() - 1] += 1;
        }

        result
    }
}

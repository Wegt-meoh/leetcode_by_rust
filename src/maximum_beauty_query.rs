pub struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut sorted_items: Vec<(i32, i32)> =
            items.iter().map(|item| (item[0], item[1])).collect();
        sorted_items.sort_by(|a, b| a.0.cmp(&b.0));

        let mut index_q: Vec<_> = queries.iter().enumerate().map(|(i, q)| (i, *q)).collect();
        index_q.sort_by(|a, b| a.1.cmp(&b.1));

        let mut ptr = 0;
        let mut result = vec![0; queries.len()];
        let mut prev = 0;

        for &(index, price) in index_q.iter() {
            while ptr < sorted_items.len() && price >= sorted_items[ptr].0 {
                prev = prev.max(sorted_items[ptr].1);
                ptr += 1;
            }
            result[index] = prev;
        }

        result
    }
}

pub struct Solution;
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let mut f = vec![vec![false; s.len()]; s.len()];
        let s_char: Vec<char> = s.chars().collect();

        for span in 1..=s.len() {
            for index in 0..=s.len() - span {
                if span == 1 {
                    f[index][index] = true;
                } else if span == 2 {
                    f[index][index + 1] = s_char[index] == s_char[index + 1];
                } else {
                    let end = index + span - 1;
                    f[index][end] = f[index + 1][end - 1] && s_char[index] == s_char[end];
                }
            }
        }

        for left in 0..s.len() - 2 {
            if f[0][left] == false {
                continue;
            };
            for right in left + 1..s.len() - 1 {
                if f[left + 1][right] && f[right + 1][s.len() - 1] {
                    return true;
                }
            }
        }

        return false;
    }
}

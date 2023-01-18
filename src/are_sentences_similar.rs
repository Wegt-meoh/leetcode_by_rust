pub struct Solution {}

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let str1_vec: Vec<&str> = sentence1.split_ascii_whitespace().collect();
        let str2_vec: Vec<&str> = sentence2.split_ascii_whitespace().collect();
        let min_len = str1_vec.len().min(str2_vec.len());

        let mut str1_index = 0;

        while str1_index < min_len && str1_vec[str1_index] == str2_vec[str1_index] {
            str1_index += 1;
        }

        let mut str2_index = 0;

        while str2_index < min_len
            && str2_vec[str2_vec.len() - str2_index - 1]
                == str1_vec[str1_vec.len() - str2_index - 1]
            && str2_index < min_len - str1_index
        {
            str2_index += 1;
        }

        (str1_index + str2_index) == min_len
    }
}

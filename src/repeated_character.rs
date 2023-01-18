use std::collections::HashSet;

pub struct Solution{}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut char_set=HashSet::new();
        let mut result='a';
        for c in s.chars(){
            if char_set.contains(&c){
                result=c;
                break;
            }
            char_set.insert(c);
        }

        result
    }
}
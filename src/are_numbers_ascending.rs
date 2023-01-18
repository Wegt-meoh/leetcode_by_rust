use std::cmp::Ordering;

pub struct Solution{}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let strings=s.split_ascii_whitespace();
        let mut prev_str="";
        for i in strings{            
            if i.starts_with(char::is_numeric){
                if prev_str!=""&&judge_ord(prev_str, i)>Ordering::Less{
                    return false;
                }else{
                    prev_str=i;
                }
            }
        }
        true
    }
}

fn judge_ord(s1:&str,s2:&str)->Ordering{
    if s1.len()!=s2.len(){
        return s1.len().cmp(&s2.len())
    }
    s1.cmp(&s2)
}
pub struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let chars = password.chars();
        let mut lower_word = false;
        let mut upper_word = false;
        let mut number_contains = false;
        let mut special_word = false;
        let mut no_consist_word = true;
        let special_words = "!@#$%^&*()-+";
        let mut prev_word = 'c';
        let mut first_in = true;

        for c in chars {
            if c.is_ascii_digit() {
                number_contains = true;
            } else if c.is_ascii_lowercase() {
                lower_word = true;
            } else if c.is_ascii_uppercase() {
                upper_word = true;
            } else {
                for s_w in special_words.chars() {
                    if c == s_w {
                        special_word = true;
                        break;
                    }
                }
            }

            if no_consist_word {
                if first_in {
                    prev_word = c;
                    first_in = false;
                } else {
                    if prev_word == c {
                        no_consist_word = false;
                    } else {
                        prev_word = c;
                    }
                }
            }
        }

        // println!("lower_word={lower_word} upper_word={upper_word} number_contains{number_contains} special_word{special_word} no_consist_word{no_consist_word}");

        if lower_word && upper_word && number_contains && special_word && no_consist_word {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::strong_password_checker_ii(String::from("IloveLe3tcode!")) == true);
    assert!(Solution::strong_password_checker_ii(String::from("Me+You--IsMyDream")) == false);
    assert!(Solution::strong_password_checker_ii(String::from("1aB!")) == false);
}

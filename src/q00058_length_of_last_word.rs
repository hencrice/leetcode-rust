pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let iter = s.split_ascii_whitespace();
        match iter.last() {
            Some(word) => word.len() as i32,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_last_word_5() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
    }

    #[test]
    fn test_empty_str() {
        assert_eq!(Solution::length_of_last_word(String::from("")), 0);
    }

    #[test]
    fn test_one_word() {
        assert_eq!(Solution::length_of_last_word(String::from("world")), 5);
    }
}
